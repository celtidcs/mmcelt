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
use std::sync::mpsc::{channel, Receiver};

/// Mantiene activo el observador del sistema de archivos y el canal receptor.
pub struct VigilanteArchivo {
    /// Observador de `notify`. Se mantiene vivo para que el hilo de escucha continúe operando.
    _watcher: RecommendedWatcher,
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
        let ruta_vigilada = ruta.to_path_buf();
        let (tx, receptor) = channel();

        let ruta_clonada = ruta_vigilada.clone();
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
            ruta: ruta_vigilada.clone(),
            origen,
        })?;

        // Vigilamos el directorio padre si existe, para capturar guardados atómicos
        // (donde se escribe un archivo temporal y se renombra sobre el destino).
        let ruta_a_vigilar = ruta_vigilada.parent().unwrap_or(&ruta_vigilada);
        watcher
            .watch(ruta_a_vigilar, RecursiveMode::NonRecursive)
            .map_err(|origen| AppError::Vigilancia {
                ruta: ruta_a_vigilar.to_path_buf(),
                origen,
            })?;

        Ok(Self {
            _watcher: watcher,
            receptor,
            ruta_vigilada,
        })
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

/// Calcula una huella digital rápida (hash de 64 bits) de un bloque de bytes.
///
/// Se utiliza para comparar el contenido del archivo en disco con la última versión
/// escrita por MMCelt e ignorar auto-disparos sin depender de marcas de tiempo del sistema de archivos.
pub fn calcular_huella_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}
