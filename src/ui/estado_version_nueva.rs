//! # Estado de la comprobación de versión nueva (`ui/estado_version_nueva.rs`)
//!
//! Guarda, durante la sesión, si la comprobación de versión nueva de este arranque sigue en
//! curso y el aviso que haya dejado. La consulta, el hilo de trabajo y el registro de los fallos
//! viven en [`crate::aviso_de_version`], el único módulo que habla con la biblioteca
//! `aviso-version-github`; aquí solo se arranca una vez y se recoge el resultado sin esperar.

use crate::aviso_de_version::{self, ResultadoDeVersion};
use crate::preferencias::Preferencias;
use aviso_version_github::AvisoDeVersionNueva;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

/// Cada cuánto se vuelve a mirar el canal mientras la comprobación sigue en curso.
///
/// La interfaz de `egui` solo se repinta cuando pasa algo; sin este repintado programado, el
/// aviso no aparecería hasta que el usuario moviera el ratón. Un cuarto de segundo basta para
/// que el aviso aparezca sin retraso apreciable, y la espera dura como mucho lo que la
/// consulta (diez segundos por defecto).
const INTERVALO_DE_REVISION: Duration = Duration::from_millis(250);

/// Comprobación de versión nueva de este arranque.
///
/// Empieza en curso (o inactiva, si no procede) y, en cuanto el hilo de trabajo responde,
/// queda terminada, con aviso o sin él. No se repite en la misma sesión.
pub struct EstadoVersionNueva {
    /// Canal del hilo de trabajo mientras la comprobación sigue en curso.
    pendiente: Option<Receiver<ResultadoDeVersion>>,
    /// El aviso que mostrar, si lo hay.
    aviso: Option<AvisoDeVersionNueva>,
}

impl EstadoVersionNueva {
    /// Un estado que no comprueba nada ni avisa de nada.
    pub fn inactivo() -> Self {
        Self::desde_canal(None)
    }

    /// Un estado que espera el resultado por el canal dado, o inactivo si no hay canal.
    ///
    /// # Parámetros
    /// - `canal`: lo que devolvió [`aviso_de_version::arrancar`].
    pub fn desde_canal(canal: Option<Receiver<ResultadoDeVersion>>) -> Self {
        Self {
            pendiente: canal,
            aviso: None,
        }
    }

    /// Un estado ya terminado con el aviso dado.
    ///
    /// # Parámetros
    /// - `aviso`: lo que hay que mostrar.
    #[cfg(test)]
    pub fn con_aviso(aviso: AvisoDeVersionNueva) -> Self {
        Self {
            pendiente: None,
            aviso: Some(aviso),
        }
    }

    /// Arranca la comprobación real por HTTPS, si el usuario no la ha desactivado.
    ///
    /// # Parámetros
    /// - `preferencias`: de aquí salen la opción y los ajustes.
    pub fn arrancar_en_este_equipo(preferencias: &Preferencias) -> Self {
        Self::desde_canal(aviso_de_version::arrancar(
            preferencias,
            crate::version::VERSION,
            aviso_de_version::consulta_real,
            aviso_de_version::registro_de_mmcelt(),
        ))
    }

    /// Si la comprobación sigue esperando respuesta.
    ///
    /// La interfaz no lo necesita —le basta con [`Self::aviso`]—; lo usan las pruebas para
    /// esperar a que termine el hilo de trabajo.
    #[cfg(test)]
    pub fn en_curso(&self) -> bool {
        self.pendiente.is_some()
    }

    /// Recoge el resultado si ya ha llegado. No espera nunca.
    ///
    /// Mientras siga en curso, pide a `egui` otro repintado dentro de
    /// [`INTERVALO_DE_REVISION`] para volver a mirar.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` al que pedir el repintado.
    pub fn atender(&mut self, ctx: &egui::Context) {
        let Some(receptor) = &self.pendiente else {
            return;
        };
        match receptor.try_recv() {
            Ok(aviso) => {
                self.aviso = aviso;
                self.pendiente = None;
            }
            Err(TryRecvError::Empty) => ctx.request_repaint_after(INTERVALO_DE_REVISION),
            // El hilo terminó sin enviar nada: no hay aviso que dar.
            Err(TryRecvError::Disconnected) => self.pendiente = None,
        }
    }

    /// El aviso de versión nueva, si lo hay.
    pub fn aviso(&self) -> Option<&AvisoDeVersionNueva> {
        self.aviso.as_ref()
    }
}

#[cfg(test)]
mod pruebas {
    use super::EstadoVersionNueva;
    use std::sync::{mpsc, Arc, Mutex, PoisonError};
    use std::time::Duration;

    /// Espera máxima entre que llega el aviso y que la interfaz lo dibuja: medio segundo, por
    /// debajo de lo que se percibe como retraso al abrir el programa. El intervalo de revisión
    /// (250 ms) cabe con holgura; uno de una hora dejaría el aviso sin dibujar hasta mover el
    /// ratón.
    const ESPERA_MAXIMA_DEL_REPINTADO: Duration = Duration::from_millis(500);

    /// Mientras la comprobación sigue en curso, el estado pide a egui que repinte pronto,
    /// para que el aviso aparezca sin que el usuario mueva el ratón.
    ///
    /// Sin cadenas: la barrera contra rótulos no admite textos escritos a mano en los archivos de la
    /// interfaz, tampoco en sus pruebas. Lo que falle se ve en el valor que compara `assert_eq!`.
    #[test]
    fn mientras_se_espera_la_version_se_pide_repintar_pronto() {
        let (_enviar, recibir) = mpsc::channel();
        let mut estado = EstadoVersionNueva::desde_canal(Some(recibir));
        let contexto = egui::Context::default();
        let esperas = Arc::new(Mutex::new(Vec::new()));
        let destino = Arc::clone(&esperas);
        contexto.set_repaint_observer(move |peticion| {
            destino
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(peticion.delay);
        });

        estado.atender(&contexto);

        let esperas = esperas
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        let tardias: Vec<Duration> = esperas
            .iter()
            .copied()
            .filter(|espera| *espera > ESPERA_MAXIMA_DEL_REPINTADO)
            .collect();
        assert_eq!((esperas.is_empty(), tardias), (false, Vec::new()));
    }
}
