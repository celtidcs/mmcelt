//! # Vigilante reactivo de archivos (`vigilante.rs`)
//!
//! Vigila en segundo plano los cambios producidos en el archivo `.mmcelt` activo
//! (por ejemplo, cuando un agente de IA actualiza el mapa mediante el servidor MCP)
//! y despierta la interfaz de `egui` de forma reactiva, con consumo 0 de CPU en reposo.
//!
//! ## Cómo funciona
//!
//! Utiliza la caja [`notify`] con el backend recomendado del sistema operativo
//! (`ReadDirectoryChangesW` en Windows, `inotify` en Linux, `kqueue`/`FSEvents` en macOS).
//! Cuando el sistema de archivos emite un evento de modificación, el hilo del vigilante
//! envía una señal a través de un canal [`std::sync::mpsc::channel`] e invoca
//! `ctx.request_repaint()` para solicitar un fotograma inmediato a `egui`.
//!
//! ## Comprobación periódica
//!
//! Los avisos del sistema no bastan solos: un agente que escribe una sola vez al terminar
//! (Gemini, por ejemplo) depende de que ese único aviso llegue. Por eso un segundo hilo mira la
//! fecha de modificación y el tamaño del archivo cada [`INTERVALO_DE_COMPROBACION`] y avisa por el
//! mismo canal si cambian. El hilo termina solo cuando el vigilante deja de existir.
//!
//! ## Inmunidad al auto-disparo
//!
//! Cada vez que MMCelt escribe en disco (guardado manual o autoguardado de 120 s),
//! se calcula la huella (hash de bytes) del contenido escrito y se guarda en memoria.
//! Al procesar un evento de vigilancia, si el hash del contenido en disco coincide con
//! la última huella escrita por la aplicación, el evento se descarta silenciosamente.

use crate::error::{AppError, AppResult};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Weak};
use std::time::{Duration, SystemTime};

/// Cada cuánto mira el hilo de comprobación la fecha y el tamaño del archivo vigilado.
///
/// Dos segundos es imperceptible para quien espera a un agente que tarda minutos, y leer los metadatos de un
/// archivo es una operación de microsegundos.
pub const INTERVALO_DE_COMPROBACION: Duration = Duration::from_secs(2);

/// Mantiene activo el observador del sistema de archivos y el canal receptor.
pub struct VigilanteArchivo {
    /// Observador de `notify`. Se mantiene vivo para que el hilo de escucha continúe operando.
    /// Es `None` solo en las pruebas que ejercitan la comprobación periódica sin él.
    _watcher: Option<RecommendedWatcher>,
    /// Mientras exista, el hilo de comprobación periódica sigue mirando el archivo. Al soltar el
    /// vigilante, el hilo ve que ya no existe y termina.
    _vida: Arc<()>,
    /// Canal por el que se reciben las notificaciones de modificación.
    receptor: Receiver<()>,
    /// Ruta del archivo vigilado.
    ruta_vigilada: PathBuf,
}

impl VigilanteArchivo {
    /// Inicia la vigilancia de un archivo `.mmcelt` en segundo plano.
    ///
    /// # Parámetros
    /// - `ruta`: ruta absoluta del archivo a vigilar.
    /// - `ctx`: contexto de `egui` que se repintará al recibir eventos.
    ///
    /// # Devuelve
    /// `Ok(VigilanteArchivo)` si se pudo inicializar el observador del sistema operativo,
    /// o un error descriptivo si la ruta no es válida o no se pudo registrar el watch.
    pub fn iniciar(ruta: &Path, ctx: egui::Context) -> AppResult<Self> {
        instalar(ruta, ctx, INTERVALO_DE_COMPROBACION, true)
    }

    /// Comprueba si se ha recibido algún evento de modificación externa.
    ///
    /// Consume todos los eventos acumulados en el canal y devuelve `true` si hubo al menos uno.
    pub fn hay_cambios(&self) -> bool {
        let mut hubo_cambio = false;
        while self.receptor.try_recv().is_ok() {
            hubo_cambio = true;
        }
        hubo_cambio
    }

    /// Devuelve la ruta del archivo que está siendo vigilado actualmente.
    pub fn ruta(&self) -> &Path {
        &self.ruta_vigilada
    }
}

/// Instala la vigilancia: avisos del sistema, si se piden, y comprobación periódica siempre.
///
/// # Parámetros
/// - `ruta`: archivo que se vigila.
/// - `ctx`: contexto de `egui` que se repinta con cada aviso.
/// - `intervalo`: cada cuánto se miran la fecha y el tamaño del archivo.
/// - `con_avisos_del_sistema`: `false` solo en pruebas, para comprobar que la comprobación
///   periódica basta sola.
pub(crate) fn instalar(
    ruta: &Path,
    ctx: egui::Context,
    intervalo: Duration,
    con_avisos_del_sistema: bool,
) -> AppResult<VigilanteArchivo> {
    let ruta_vigilada = ruta.to_path_buf();
    let (tx, receptor) = channel();
    let vida = Arc::new(());
    lanzar_comprobacion_periodica(
        ruta_vigilada.clone(),
        intervalo,
        tx.clone(),
        ctx.clone(),
        Arc::downgrade(&vida),
    );
    let watcher = if con_avisos_del_sistema {
        Some(observar_con_el_sistema(&ruta_vigilada, tx, ctx)?)
    } else {
        None
    };
    Ok(VigilanteArchivo {
        _watcher: watcher,
        _vida: vida,
        receptor,
        ruta_vigilada,
    })
}

/// Registra el observador del sistema operativo sobre la carpeta del archivo.
fn observar_con_el_sistema(
    ruta_vigilada: &Path,
    tx: Sender<()>,
    ctx: egui::Context,
) -> AppResult<RecommendedWatcher> {
    let ruta_clonada = ruta_vigilada.to_path_buf();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let es_relevante = matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Any
                );
                if es_relevante {
                    let coincide = event.paths.is_empty()
                        || event.paths.iter().any(|p| {
                            p == &ruta_clonada || p.file_name() == ruta_clonada.file_name()
                        });
                    if coincide {
                        let _ = tx.send(());
                        ctx.request_repaint();
                    }
                }
            }
        },
        Config::default(),
    )
    .map_err(|origen| AppError::Vigilancia {
        ruta: ruta_vigilada.to_path_buf(),
        origen,
    })?;

    // Vigilamos el directorio padre si existe, para capturar guardados atómicos
    // (donde se escribe un archivo temporal y se renombra sobre el destino).
    let ruta_a_vigilar = ruta_vigilada.parent().unwrap_or(ruta_vigilada);
    watcher
        .watch(ruta_a_vigilar, RecursiveMode::NonRecursive)
        .map_err(|origen| AppError::Vigilancia {
            ruta: ruta_a_vigilar.to_path_buf(),
            origen,
        })?;
    Ok(watcher)
}

/// Fecha de modificación y tamaño del archivo, o `None` si no se pueden leer.
fn firma_del_archivo(ruta: &Path) -> Option<(SystemTime, u64)> {
    let metadatos = std::fs::metadata(ruta).ok()?;
    Some((metadatos.modified().ok()?, metadatos.len()))
}

/// Lanza el hilo que mira el archivo cada `intervalo` y avisa si su fecha o su tamaño cambian.
///
/// Termina solo cuando `vida` ya no existe (se soltó el vigilante) o cuando el canal se cierra.
fn lanzar_comprobacion_periodica(
    ruta: PathBuf,
    intervalo: Duration,
    tx: Sender<()>,
    ctx: egui::Context,
    vida: Weak<()>,
) {
    std::thread::spawn(move || {
        let mut anterior = firma_del_archivo(&ruta);
        loop {
            std::thread::sleep(intervalo);
            if vida.upgrade().is_none() {
                return;
            }
            let actual = firma_del_archivo(&ruta);
            if actual != anterior {
                anterior = actual;
                if tx.send(()).is_err() {
                    return;
                }
                ctx.request_repaint();
            }
        }
    });
}

/// Calcula una huella digital rápida (hash de 64 bits) de un bloque de bytes.
///
/// Se utiliza para comparar el contenido del archivo en disco con la última versión
/// escrita por MMCelt e ignorar auto-disparos sin depender de marcas de tiempo del sistema de archivos.
pub fn calcular_huella_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}
