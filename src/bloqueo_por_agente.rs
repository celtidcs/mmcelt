//! Bloqueo del mapa mientras un agente trabaja en él (0.16.0).
//!
//! Al confirmar un envío, el mapa deja de poder editarse: si la persona y el agente cambian el
//! mismo archivo a la vez, uno de los dos pierde su trabajo. El bloqueo se levanta cuando el
//! agente declara en su recibo de entrega (`.mmcelt/devolucion.json`) que ha terminado, o
//! cuando la persona pulsa «Recuperar el control», por si el agente falla y no avisa nunca.
//!
//! Mientras dura, se puede ver, recorrer, seleccionar, buscar y exportar. Cualquier cambio del
//! contenido que llegue a producirse, por el camino que sea, se deshace al final del fotograma
//! volviendo a la copia del mapa: así ninguna vía de edición queda sin cubrir.

use crate::devolucion_agentes::{leer_recibo, ReciboDevolucion};
use crate::model::Proyecto;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Cada cuánto se lee el recibo de entrega mientras el mapa está bloqueado. Es el mismo ritmo
/// que la comprobación periódica del archivo del mapa.
pub(crate) const INTERVALO_DE_LECTURA_DEL_RECIBO: Duration =
    crate::vigilante::INTERVALO_DE_COMPROBACION;

/// Estado del bloqueo de un mapa enviado a un agente.
#[derive(Default)]
pub(crate) struct BloqueoPorAgente {
    activo: Option<Bloqueo>,
}

/// Datos del bloqueo en curso.
struct Bloqueo {
    /// El mapa tal como debe seguir: el enviado o la última entrega del agente.
    copia: Proyecto,
    /// Entrega que ya estaba en el recibo al bloquear: no cuenta como la del agente actual.
    entrega_previa: Option<Uuid>,
    /// Última vez que se leyó el recibo.
    ultima_lectura: Option<Instant>,
    /// Huella del archivo del mapa cuando llegó el recibo de la entrega final. El bloqueo se
    /// quita al cargar exactamente ese contenido, no una lectura anterior que llegue después.
    huella_final: Option<u64>,
}

impl BloqueoPorAgente {
    /// Bloquea el mapa tal como está ahora. `raiz` es la carpeta del proyecto, donde el agente
    /// deja su recibo.
    pub(crate) fn bloquear(&mut self, proyecto: &Proyecto, raiz: &Path) {
        self.activo = Some(Bloqueo {
            copia: proyecto.clone(),
            entrega_previa: leer_recibo(raiz).map(|recibo| recibo.id_entrega),
            ultima_lectura: None,
            huella_final: None,
        });
    }

    /// Quita el bloqueo: el agente ha terminado, el envío falló o la persona recupera el control.
    pub(crate) fn desbloquear(&mut self) {
        self.activo = None;
    }

    /// Indica si el mapa está bloqueado.
    pub(crate) fn esta_bloqueado(&self) -> bool {
        self.activo.is_some()
    }

    /// Una entrega del agente ha sustituido el mapa en pantalla; `huella_cargada` es la huella
    /// del archivo leído. Si es la entrega final (la huella anotada al llegar el recibo), quita
    /// el bloqueo y devuelve `true`; si no, pasa a ser lo que hay que conservar.
    pub(crate) fn aceptar_entrega(
        &mut self,
        proyecto: &Proyecto,
        huella_cargada: Option<u64>,
    ) -> bool {
        let es_la_final = self.activo.as_ref().is_some_and(|bloqueo| {
            bloqueo.huella_final.is_some() && bloqueo.huella_final == huella_cargada
        });
        if es_la_final {
            self.desbloquear();
        } else if let Some(bloqueo) = &mut self.activo {
            bloqueo.copia = proyecto.clone();
        }
        es_la_final
    }

    /// Deshace cualquier cambio del contenido hecho con el mapa bloqueado. Devuelve `true` si
    /// había alguno.
    pub(crate) fn deshacer_cambios(&self, proyecto: &mut Proyecto) -> bool {
        let Some(bloqueo) = &self.activo else {
            return false;
        };
        if proyecto.revision() == bloqueo.copia.revision() {
            return false;
        }
        *proyecto = bloqueo.copia.clone();
        true
    }

    /// Mira, como mucho cada [`INTERVALO_DE_LECTURA_DEL_RECIBO`], si el agente ha declarado
    /// terminado su trabajo sobre `ruta_mapa`. Si es así, anota la huella del mapa en disco
    /// (`huella_en_disco`) y devuelve `true` una sola vez: el bloqueo sigue hasta que se carga
    /// ese contenido ([`Self::aceptar_entrega`]). El agente escribe el mapa antes que el recibo,
    /// así que esa huella es la de su entrega final.
    pub(crate) fn comprobar_si_termino(
        &mut self,
        raiz: &Path,
        ruta_mapa: &Path,
        ahora: Instant,
        huella_en_disco: impl FnOnce() -> Option<u64>,
    ) -> bool {
        let Some(bloqueo) = &mut self.activo else {
            return false;
        };
        if bloqueo.huella_final.is_some() {
            return false;
        }
        if bloqueo
            .ultima_lectura
            .is_some_and(|antes| ahora.duration_since(antes) < INTERVALO_DE_LECTURA_DEL_RECIBO)
        {
            return false;
        }
        bloqueo.ultima_lectura = Some(ahora);
        let terminado = leer_recibo(raiz).is_some_and(|recibo| {
            es_la_entrega_final(&recibo, raiz, ruta_mapa, bloqueo.entrega_previa)
        });
        if !terminado {
            return false;
        }
        bloqueo.huella_final = huella_en_disco();
        bloqueo.huella_final.is_some()
    }
}

/// Decide si `recibo` es la entrega final del agente para el mapa bloqueado: una entrega nueva
/// (no la que ya había al bloquear), de ese mismo mapa y marcada como trabajo terminado.
pub(crate) fn es_la_entrega_final(
    recibo: &ReciboDevolucion,
    raiz: &Path,
    ruta_mapa: &Path,
    entrega_previa: Option<Uuid>,
) -> bool {
    recibo.trabajo_terminado
        && Some(recibo.id_entrega) != entrega_previa
        && misma_ruta(&raiz.join(&recibo.map_path), ruta_mapa)
}

/// Compara dos rutas por su forma canónica cuando existen, o tal cual si no.
fn misma_ruta(una: &Path, otra: &Path) -> bool {
    let canonica = |ruta: &Path| -> PathBuf { ruta.canonicalize().unwrap_or(ruta.to_path_buf()) };
    canonica(una) == canonica(otra)
}
