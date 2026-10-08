//! # Estado de la comprobación de versión nueva (`ui/estado_version_nueva.rs`)
//!
//! Guarda, durante la sesión, si la comprobación de versión nueva de este arranque sigue en
//! curso y el aviso que haya dejado. La consulta y la decisión viven en
//! [`crate::comprobacion_de_version`] y [`crate::version_publicada`]; aquí solo se arranca
//! una vez y se recoge el resultado sin esperar.

use crate::comprobacion_de_version::{
    lanzar_en_segundo_plano, registrar_fallo, ConsultaDeVersion, ConsultaHttps,
};
use crate::preferencias::{AjustesDeVersionNueva, Preferencias};
use crate::version_publicada::{AvisoDeVersionNueva, OrigenDeLasVersiones};
use std::sync::mpsc::{Receiver, TryRecvError};

/// Comprobación de versión nueva de este arranque.
///
/// Empieza en curso (o inactiva, si no procede) y, en cuanto el hilo de trabajo responde,
/// queda terminada, con aviso o sin él. No se repite en la misma sesión.
pub struct EstadoVersionNueva {
    /// Canal del hilo de trabajo mientras la comprobación sigue en curso.
    pendiente: Option<Receiver<Option<AvisoDeVersionNueva>>>,
    /// El aviso que mostrar, si lo hay.
    aviso: Option<AvisoDeVersionNueva>,
}

impl EstadoVersionNueva {
    /// Un estado que no comprueba nada ni avisa de nada.
    pub fn inactivo() -> Self {
        Self {
            pendiente: None,
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
    /// - `ctx`: contexto de `egui`, para pedir un repintado cuando llegue el resultado.
    pub fn arrancar_en_este_equipo(preferencias: &Preferencias, ctx: &egui::Context) -> Self {
        Self::arrancar_si_procede(
            preferencias,
            crate::version::VERSION,
            ctx.clone(),
            |ajustes| Box::new(ConsultaHttps::nueva(ajustes)),
        )
    }

    /// Arranca la comprobación en un hilo de trabajo si el usuario no la ha desactivado.
    ///
    /// Con la opción apagada, o con ajustes que no superan la validación, **no se llama a
    /// `fabricar`**: no llega a existir nada capaz de salir a la red.
    ///
    /// # Parámetros
    /// - `preferencias`: de aquí salen la opción y los ajustes.
    /// - `version_local`: la versión que se está ejecutando.
    /// - `ctx`: contexto de `egui`, para pedir un repintado cuando llegue el resultado.
    /// - `fabricar`: construye la consulta; el programa usa [`ConsultaHttps::nueva`].
    pub fn arrancar_si_procede<F>(
        preferencias: &Preferencias,
        version_local: &'static str,
        ctx: egui::Context,
        fabricar: F,
    ) -> Self
    where
        F: FnOnce(&AjustesDeVersionNueva) -> Box<dyn ConsultaDeVersion>,
    {
        if !preferencias.comprobar_version_nueva {
            return Self::inactivo();
        }
        let origen = match OrigenDeLasVersiones::desde_ajustes(&preferencias.ajustes_de_version) {
            Ok(origen) => origen,
            Err(fallo) => {
                registrar_fallo(&fallo);
                return Self::inactivo();
            }
        };
        let consulta = fabricar(&preferencias.ajustes_de_version);
        Self {
            pendiente: Some(lanzar_en_segundo_plano(
                version_local,
                origen,
                consulta,
                ctx,
            )),
            aviso: None,
        }
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
    pub fn atender(&mut self) {
        let Some(receptor) = &self.pendiente else {
            return;
        };
        match receptor.try_recv() {
            Ok(aviso) => {
                self.aviso = aviso;
                self.pendiente = None;
            }
            Err(TryRecvError::Empty) => {}
            // El hilo terminó sin enviar nada: no hay aviso que dar.
            Err(TryRecvError::Disconnected) => self.pendiente = None,
        }
    }

    /// El aviso de versión nueva, si lo hay.
    pub fn aviso(&self) -> Option<&AvisoDeVersionNueva> {
        self.aviso.as_ref()
    }
}
