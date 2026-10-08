//! # Cambios de jerarquía (`model/jerarquia.rs`)
//!
//! Mover un nodo bajo otro padre sin romper el árbol.
//!
//! ## Por qué está aparte
//!
//! Es la única operación que cambia **de quién cuelga** un nodo ya existente, y es la que
//! puede crear un ciclo: colgar un nodo de uno de sus propios descendientes lo dejaría flotando,
//! sin camino hasta la raíz, y los recorridos de `layout.rs` darían vueltas para siempre. Toda
//! la comprobación vive aquí, junto a la operación, para que nadie pueda cambiar `parent_id` a
//! mano desde la interfaz.

use super::tipos::Proyecto;
use chrono::Utc;
use uuid::Uuid;

/// Por qué no se puede colgar un nodo de otro.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovimientoRechazado {
    /// La raíz no tiene padre ni puede tenerlo.
    EsLaRaiz,
    /// Un nodo no puede ser hijo de sí mismo.
    SobreSiMismo,
    /// El nuevo padre está dentro de la descendencia del nodo: se formaría un ciclo.
    BajoSuDescendencia,
    /// El nodo ya cuelga de ese padre: no hay nada que mover.
    YaEsSuPadre,
    /// Alguno de los dos nodos no existe en el mapa.
    NodoInexistente,
    /// El destino es la raíz: no tiene padre del que colgar a un hermano.
    DestinoSinPadre,
    /// Los dos nodos ya cuelgan del mismo padre: ya son hermanos (PH-1007-7).
    YaSonHermanos,
}

impl Proyecto {
    /// Cuelga `nodo`, con su descendencia, del padre de `hermano`, justo detrás de él.
    ///
    /// # Errores
    /// [`MovimientoRechazado::DestinoSinPadre`] si `hermano` es la raíz;
    /// [`MovimientoRechazado::YaSonHermanos`] si ya cuelgan del mismo padre (no hay
    /// nada que mover, igual que «Hacer hijo» cuando ya lo es); los demás motivos, los mismos que [`Self::comprobar_que_se_puede_colgar`]
    /// aplicado al padre de `hermano`. En cualquier error el mapa no cambia.
    pub fn hacer_hermano_de(
        &mut self,
        nodo: Uuid,
        hermano: Uuid,
    ) -> Result<(), MovimientoRechazado> {
        let padre = self.comprobar_que_puede_ser_hermano(nodo, hermano)?;
        let padre_anterior = self.nodes.get(&nodo).and_then(|n| n.parent_id);
        if let Some(anterior) = padre_anterior.and_then(|id| self.nodes.get_mut(&id)) {
            anterior.children.retain(|&hijo| hijo != nodo);
        }
        if let Some(nuevo) = self.nodes.get_mut(&padre) {
            let detras_del_hermano = nuevo
                .children
                .iter()
                .position(|&hijo| hijo == hermano)
                .map_or(nuevo.children.len(), |posicion| posicion + 1);
            nuevo.children.insert(detras_del_hermano, nodo);
        }
        if let Some(movido) = self.nodes.get_mut(&nodo) {
            movido.parent_id = Some(padre);
        }
        self.updated_at = Utc::now();
        self.marcar_modificado();
        Ok(())
    }

    /// Dice si `nodo` puede pasar a ser hermano de `hermano`, sin cambiar nada.
    ///
    /// # Devuelve
    /// El padre del que colgaría.
    ///
    /// # Errores
    /// El mismo [`MovimientoRechazado`] que devolvería [`Self::hacer_hermano_de`].
    pub fn comprobar_que_puede_ser_hermano(
        &self,
        nodo: Uuid,
        hermano: Uuid,
    ) -> Result<Uuid, MovimientoRechazado> {
        if !self.nodes.contains_key(&nodo) || !self.nodes.contains_key(&hermano) {
            return Err(MovimientoRechazado::NodoInexistente);
        }
        if nodo == self.root_id {
            return Err(MovimientoRechazado::EsLaRaiz);
        }
        if nodo == hermano {
            return Err(MovimientoRechazado::SobreSiMismo);
        }
        let padre = self
            .nodes
            .get(&hermano)
            .and_then(|n| n.parent_id)
            .ok_or(MovimientoRechazado::DestinoSinPadre)?;
        if padre == nodo || self.desciende_de(padre, nodo) {
            return Err(MovimientoRechazado::BajoSuDescendencia);
        }
        if self.nodes.get(&nodo).and_then(|n| n.parent_id) == Some(padre) {
            return Err(MovimientoRechazado::YaSonHermanos);
        }
        Ok(padre)
    }

    /// Cuelga `nodo`, con toda su descendencia, del nodo `nuevo_padre`.
    ///
    /// Lo descuelga de su padre anterior, lo añade al final de los hijos del nuevo y marca el
    /// mapa como modificado. La posición del nodo no se toca: de recolocarlo se encarga la
    /// disposición, si está activa.
    ///
    /// # Parámetros
    /// - `nodo`: el nodo que se mueve.
    /// - `nuevo_padre`: del que pasará a colgar.
    ///
    /// # Errores
    /// [`MovimientoRechazado`] con el motivo, y en ese caso el mapa no cambia en absoluto.
    pub fn hacer_hijo_de(
        &mut self,
        nodo: Uuid,
        nuevo_padre: Uuid,
    ) -> Result<(), MovimientoRechazado> {
        self.comprobar_que_se_puede_colgar(nodo, nuevo_padre)?;

        let padre_anterior = self.nodes.get(&nodo).and_then(|n| n.parent_id);
        if let Some(anterior) = padre_anterior.and_then(|id| self.nodes.get_mut(&id)) {
            anterior.children.retain(|&hijo| hijo != nodo);
        }
        if let Some(padre) = self.nodes.get_mut(&nuevo_padre) {
            padre.children.push(nodo);
        }
        if let Some(movido) = self.nodes.get_mut(&nodo) {
            movido.parent_id = Some(nuevo_padre);
        }
        self.updated_at = Utc::now();
        self.marcar_modificado();
        Ok(())
    }

    /// Dice si `nodo` se puede colgar de `nuevo_padre`, sin cambiar nada.
    ///
    /// La interfaz lo usa para deshabilitar la opción antes de que el usuario la pulse.
    ///
    /// # Errores
    /// El mismo [`MovimientoRechazado`] que devolvería [`Self::hacer_hijo_de`].
    pub fn comprobar_que_se_puede_colgar(
        &self,
        nodo: Uuid,
        nuevo_padre: Uuid,
    ) -> Result<(), MovimientoRechazado> {
        if !self.nodes.contains_key(&nodo) || !self.nodes.contains_key(&nuevo_padre) {
            return Err(MovimientoRechazado::NodoInexistente);
        }
        if nodo == self.root_id {
            return Err(MovimientoRechazado::EsLaRaiz);
        }
        if nodo == nuevo_padre {
            return Err(MovimientoRechazado::SobreSiMismo);
        }
        if self.nodes.get(&nodo).and_then(|n| n.parent_id) == Some(nuevo_padre) {
            return Err(MovimientoRechazado::YaEsSuPadre);
        }
        if self.desciende_de(nuevo_padre, nodo) {
            return Err(MovimientoRechazado::BajoSuDescendencia);
        }
        Ok(())
    }

    /// Dice si `posible_descendiente` cuelga, directa o indirectamente, de `antepasado`.
    ///
    /// Sube por los padres desde `posible_descendiente`. El número de pasos está acotado por
    /// el número de nodos, así que un mapa ya dañado con un ciclo no deja colgado el programa;
    /// si se agotan sin llegar a la raíz, se responde que sí, que es la respuesta que impide
    /// el movimiento.
    fn desciende_de(&self, posible_descendiente: Uuid, antepasado: Uuid) -> bool {
        let mut actual = self
            .nodes
            .get(&posible_descendiente)
            .and_then(|n| n.parent_id);
        for _ in 0..self.nodes.len() {
            match actual {
                Some(id) if id == antepasado => return true,
                Some(id) => actual = self.nodes.get(&id).and_then(|n| n.parent_id),
                None => return false,
            }
        }
        true
    }
}
