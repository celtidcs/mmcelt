//! # Clasificación de un nodo (`model/clasificacion.rs`)
//!
//! Estado de madurez, prioridad, control humano y rol de un nodo, cambiados por operaciones con
//! nombre en vez de escribiendo el campo desde la interfaz.
//!
//! Las tres comparten una regla: **solo un cambio real marca el mapa como modificado**. Elegir
//! en un menú el valor que ya tenía no es un cambio, y marcarlo como tal pediría guardar un mapa
//! idéntico y metería un paso vacío en el historial de deshacer.

use super::tipos::{EstadoNodo, EstadoRevision, Nodo, PrioridadNodo, Proyecto, RolNodo};
use chrono::Utc;
use uuid::Uuid;

impl Proyecto {
    /// Cambia el estado de madurez de un nodo.
    ///
    /// # Parámetros
    /// - `id`: el nodo.
    /// - `valor`: el estado nuevo.
    ///
    /// # Devuelve
    /// `true` si el nodo existe y su estado ha cambiado; solo entonces se marca el mapa.
    pub fn fijar_estado(&mut self, id: Uuid, valor: EstadoNodo) -> bool {
        self.cambiar_si_difiere(id, |nodo| &mut nodo.status, valor)
    }

    /// Cambia la prioridad de un nodo.
    ///
    /// # Devuelve
    /// `true` si el nodo existe y su prioridad ha cambiado.
    pub fn fijar_prioridad(&mut self, id: Uuid, valor: PrioridadNodo) -> bool {
        self.cambiar_si_difiere(id, |nodo| &mut nodo.priority, valor)
    }

    /// Cambia el estado de control humano (revisión) de un nodo.
    ///
    /// # Devuelve
    /// `true` si el nodo existe y su estado de revisión ha cambiado.
    pub fn fijar_revision(&mut self, id: Uuid, valor: EstadoRevision) -> bool {
        self.cambiar_si_difiere(id, |nodo| &mut nodo.review_status, valor)
    }

    /// Cambia el rol de un nodo.
    ///
    /// # Devuelve
    /// `true` si el nodo existe y su rol ha cambiado.
    pub fn fijar_rol(&mut self, id: Uuid, valor: RolNodo) -> bool {
        self.cambiar_si_difiere(id, |nodo| &mut nodo.role, valor)
    }

    /// Escribe `valor` en el campo que elige `campo` si es distinto del que hay.
    ///
    /// Es la única pieza que decide «¿ha cambiado?» para las tres operaciones: así la regla
    /// vive en un solo sitio.
    fn cambiar_si_difiere<T: PartialEq>(
        &mut self,
        id: Uuid,
        campo: fn(&mut Nodo) -> &mut T,
        valor: T,
    ) -> bool {
        let Some(nodo) = self.nodes.get_mut(&id) else {
            return false;
        };
        let destino = campo(nodo);
        if *destino == valor {
            return false;
        }
        *destino = valor;
        self.updated_at = Utc::now();
        self.marcar_modificado();
        true
    }
}
