//! Reordenacion de los hijos de un mismo padre.

use super::jerarquia::MovimientoRechazado;
use super::tipos::Proyecto;
use jiff::Timestamp;
use uuid::Uuid;

impl Proyecto {
    /// Devuelve las posiciones de dos hermanos distintos en la lista de su padre.
    pub(crate) fn posiciones_de_hermanos(
        &self,
        nodo: Uuid,
        destino: Uuid,
    ) -> Option<(Uuid, usize, usize)> {
        let padre = self.nodes.get(&nodo)?.parent_id?;
        if nodo == destino || self.nodes.get(&destino)?.parent_id != Some(padre) {
            return None;
        }
        let hijos = &self.nodes.get(&padre)?.children;
        Some((
            padre,
            hijos.iter().position(|&id| id == nodo)?,
            hijos.iter().position(|&id| id == destino)?,
        ))
    }

    /// Coloca un hermano justo detrás del destino dentro del mismo padre.
    /// Devuelve si cambió el orden. Una suelta inválida no modifica el árbol.
    pub(crate) fn colocar_despues(
        &mut self,
        nodo: Uuid,
        destino: Uuid,
    ) -> Result<bool, MovimientoRechazado> {
        let (padre, origen, destino) = self
            .posiciones_de_hermanos(nodo, destino)
            .ok_or(MovimientoRechazado::NoSonHermanos)?;
        if origen == destino + 1 {
            return Ok(false);
        }
        let hijos = &mut self
            .nodes
            .get_mut(&padre)
            .ok_or(MovimientoRechazado::NodoInexistente)?
            .children;
        hijos.remove(origen);
        hijos.insert(
            if origen < destino {
                destino
            } else {
                destino + 1
            },
            nodo,
        );
        self.updated_at = Timestamp::now();
        self.marcar_modificado();
        Ok(true)
    }
}
