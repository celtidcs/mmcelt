//! # Conexiones cruzadas admitidas por la interfaz (`model/conexiones.rs`)
//!
//! Cuándo tiene sentido que la persona cree una conexión cruzada entre dos nodos. El modelo
//! acepta cualquier pareja de nodos distintos (lo necesitan los importadores); esta pregunta es
//! la que hace la interfaz antes de ofrecer la opción (PH-1007-2).

use super::tipos::Proyecto;
use uuid::Uuid;

impl Proyecto {
    /// Dice si se puede crear una conexión cruzada entre dos nodos.
    ///
    /// No se admite si son el mismo nodo, si alguno no existe, si uno es el padre directo del
    /// otro —la jerarquía ya los une con una línea, y una segunda del mismo color solo confunde
    /// (PH-1007-2)— ni si esa pareja ya está conectada, en cualquier sentido.
    pub fn conexion_cruzada_admitida(&self, desde: Uuid, hasta: Uuid) -> bool {
        let (Some(origen), Some(destino)) = (self.nodes.get(&desde), self.nodes.get(&hasta)) else {
            return false;
        };
        let padre_e_hijo = origen.parent_id == Some(hasta) || destino.parent_id == Some(desde);
        let ya_conectados = self
            .connections
            .iter()
            .any(|c| (c.from == desde && c.to == hasta) || (c.from == hasta && c.to == desde));
        desde != hasta && !padre_e_hijo && !ya_conectados
    }
}
