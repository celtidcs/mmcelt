//! Carga los mapas modificados por otro proceso sin detener la interfaz.
//!
//! El vigilante del sistema de archivos solo avisa de que algo ha cambiado. Leer el
//! archivo, analizar su JSON y validar el árbol puede tardar, sobre todo si el mapa está
//! en una unidad de red o sincronizada. Este módulo hace esas tres operaciones en un único
//! trabajador estable y devuelve a la interfaz únicamente resultados ya preparados.

use crate::error::{AppError, AppResult};
use crate::model::Proyecto;
use std::path::PathBuf;
use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender, TryRecvError, TrySendError};
use std::thread::JoinHandle;

/// Resultado completo de intentar cargar una versión externa del mapa.
pub struct ResultadoCargaExterna {
    /// Archivo al que pertenece el resultado. Permite descartar respuestas atrasadas.
    pub ruta: PathBuf,
    /// Huella del contenido leído, o cero cuando ni siquiera fue posible leerlo.
    pub huella: u64,
    /// Mapa ya analizado y validado, o el error tipado que impidió prepararlo.
    pub proyecto: AppResult<Proyecto>,
}

/// Mantiene un único trabajador encargado de las lecturas externas.
///
/// El canal de solicitudes admite una sola espera. Si llegan muchos avisos para el mismo
/// archivo mientras se está leyendo, basta conservar una repetición: esa segunda lectura
/// verá el contenido más reciente y evita crear un hilo nuevo por cada evento de `notify`.
pub struct CargadorExterno {
    solicitudes: Option<SyncSender<PathBuf>>,
    resultados: Receiver<ResultadoCargaExterna>,
    trabajador: Option<JoinHandle<()>>,
}

impl CargadorExterno {
    /// Arranca el trabajador que usa el sistema de archivos real.
    pub fn nuevo(ctx: egui::Context) -> Self {
        Self::con_funcion_interna(ctx, cargar_archivo)
    }

    /// Entrega una ruta al trabajador sin esperar a que la lea.
    ///
    /// Devuelve `true` si la solicitud entró en el canal. Un `false` significa que ya hay
    /// otra lectura de esa misma clase esperando; no es un error y no bloquea la interfaz.
    pub fn solicitar(&self, ruta: PathBuf) -> bool {
        let Some(solicitudes) = &self.solicitudes else {
            return false;
        };
        match solicitudes.try_send(ruta) {
            Ok(()) => true,
            Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => false,
        }
    }

    /// Recoge el último resultado disponible sin detener el fotograma actual.
    pub fn intentar_recibir(&self) -> Option<ResultadoCargaExterna> {
        let mut ultimo = None;
        loop {
            match self.resultados.try_recv() {
                Ok(resultado) => ultimo = Some(resultado),
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => return ultimo,
            }
        }
    }

    fn con_funcion_interna<F>(ctx: egui::Context, cargar: F) -> Self
    where
        F: Fn(PathBuf) -> ResultadoCargaExterna + Send + 'static,
    {
        let (solicitudes_tx, solicitudes_rx) = sync_channel::<PathBuf>(1);
        // Los resultados no bloquean al trabajador durante el cierre. En la práctica hay
        // como máximo dos (uno leído y otro en espera), porque la entrada sí está acotada.
        let (resultados_tx, resultados_rx) = channel::<ResultadoCargaExterna>();
        let trabajador = std::thread::Builder::new()
            .name("mmcelt-carga-externa".to_string())
            .spawn(move || {
                while let Ok(ruta) = solicitudes_rx.recv() {
                    let resultado = cargar(ruta);
                    if resultados_tx.send(resultado).is_err() {
                        break;
                    }
                    ctx.request_repaint();
                }
            })
            .expect("no se pudo crear el trabajador de carga externa");

        Self {
            solicitudes: Some(solicitudes_tx),
            resultados: resultados_rx,
            trabajador: Some(trabajador),
        }
    }

    #[cfg(test)]
    fn con_funcion<F>(ctx: egui::Context, cargar: F) -> Self
    where
        F: Fn(PathBuf) -> ResultadoCargaExterna + Send + 'static,
    {
        Self::con_funcion_interna(ctx, cargar)
    }
}

impl Drop for CargadorExterno {
    fn drop(&mut self) {
        // Cerrar primero el emisor despierta a un trabajador que esté esperando `recv`.
        self.solicitudes.take();
        if let Some(trabajador) = self.trabajador.take() {
            let _ = trabajador.join();
        }
    }
}

/// Lee, analiza y valida una versión del mapa enteramente en el trabajador.
fn cargar_archivo(ruta: PathBuf) -> ResultadoCargaExterna {
    let contenido = match crate::proyectos::leer_bytes(&ruta) {
        Ok(contenido) => contenido,
        Err(error) => {
            return ResultadoCargaExterna {
                ruta,
                huella: 0,
                proyecto: Err(error),
            };
        }
    };
    let huella = crate::vigilante::calcular_huella_bytes(&contenido);
    let proyecto = serde_json::from_slice::<Proyecto>(&contenido)
        .map_err(|origen| AppError::formato("cargar un cambio externo del mapa", origen))
        .and_then(|proyecto| {
            proyecto.validar_estructura()?;
            Ok(proyecto)
        });

    ResultadoCargaExterna {
        ruta,
        huella,
        proyecto,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Proyecto;
    use std::path::PathBuf;
    use std::sync::mpsc::sync_channel;
    use std::time::{Duration, Instant};

    #[test]
    fn solicitar_una_carga_no_espera_a_que_termine_la_lectura() {
        let (lectura_iniciada_tx, lectura_iniciada_rx) = sync_channel(0);
        let (continuar_tx, continuar_rx) = sync_channel(0);
        let ruta = PathBuf::from("mapa-lento.mmcelt");
        let ruta_del_resultado = ruta.clone();
        let cargador = CargadorExterno::con_funcion(egui::Context::default(), move |_ruta| {
            lectura_iniciada_tx.send(()).expect("avisar del inicio");
            continuar_rx.recv().expect("autorizar el final");
            ResultadoCargaExterna {
                ruta: ruta_del_resultado.clone(),
                huella: 7,
                proyecto: Ok(Proyecto::nuevo_vacio("Leido fuera de la interfaz")),
            }
        });

        let comienzo = Instant::now();
        assert!(cargador.solicitar(ruta));
        assert!(
            comienzo.elapsed() < Duration::from_millis(30),
            "solicitar no puede ejecutar la lectura en el hilo que dibuja"
        );
        lectura_iniciada_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("el trabajador debe empezar la lectura");
        assert!(
            cargador.intentar_recibir().is_none(),
            "no debe inventarse un resultado mientras la lectura sigue bloqueada"
        );

        continuar_tx.send(()).expect("terminar la lectura");
        let limite = Instant::now() + Duration::from_secs(1);
        let resultado = loop {
            if let Some(resultado) = cargador.intentar_recibir() {
                break resultado;
            }
            assert!(Instant::now() < limite, "el resultado no llego a tiempo");
            std::thread::yield_now();
        };
        assert_eq!(resultado.huella, 7);
        assert_eq!(
            resultado.proyecto.expect("mapa valido").title,
            "Leido fuera de la interfaz"
        );
    }
}
