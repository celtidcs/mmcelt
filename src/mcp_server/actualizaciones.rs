//! Conversión semántica de los cambios solicitados por MCP.
//!
//! `argumentos` garantiza la forma del JSON. Este módulo da el paso siguiente: convierte una
//! sola vez los textos de enumeraciones a los tipos del dominio y conserva de forma explícita
//! los valores desconocidos para poder avisar al agente sin degradarlos en silencio.

use super::argumentos::{
    ActualizacionNodo, ArgumentosSincronizar, ConexionSolicitada, ReferenciaNodo,
    ReferenciaNodoPrestada,
};
use crate::model::{EstadoNodo, PrioridadNodo, RolNodo, TipoRelacion};

/// Valor opcional de enumeración recibido de un agente.
#[derive(Debug)]
pub(crate) enum ValorEnumerado<T> {
    /// El agente no pidió cambiar este dato.
    Ausente,
    /// El texto coincide con una variante conocida del dominio.
    Reconocido(T),
    /// El agente envió un nombre que debe rechazarse o notificarse expresamente.
    Desconocido(String),
}

fn reconocer<T>(
    texto: Option<String>,
    conversor: impl FnOnce(&str) -> Option<T>,
) -> ValorEnumerado<T> {
    match texto {
        None => ValorEnumerado::Ausente,
        Some(texto) => match conversor(&texto) {
            Some(valor) => ValorEnumerado::Reconocido(valor),
            None => ValorEnumerado::Desconocido(texto),
        },
    }
}

/// Actualización lista para aplicarse al modelo, sin JSON dinámico.
#[derive(Debug)]
pub(crate) struct ActualizacionTipada {
    /// Identidad ya validada del nodo al que se dirige el cambio.
    pub(crate) nodo: ReferenciaNodo,
    /// UUID opcional del padre de un nodo que todavía no existe.
    pub(crate) parent_id: Option<String>,
    /// Título opcional del padre, alternativo o complementario al UUID.
    pub(crate) parent_title: Option<String>,
    /// Estado reconocido, ausente o desconocido sin pérdida silenciosa.
    pub(crate) estado: ValorEnumerado<EstadoNodo>,
    /// Notas propuestas por el agente.
    pub(crate) notas: Option<String>,
    /// Prioridad reconocida, ausente o desconocida.
    pub(crate) prioridad: ValorEnumerado<PrioridadNodo>,
    /// Rol reconocido, ausente o desconocido.
    pub(crate) rol: ValorEnumerado<RolNodo>,
    /// Etiquetas que se añadirán conservando las existentes.
    pub(crate) etiquetas: Vec<String>,
    /// Ruta de código propuesta, si la actualización la incluye.
    pub(crate) ruta_archivo: Option<String>,
}

impl ActualizacionTipada {
    /// Devuelve la referencia prestada del padre solicitado.
    pub(crate) fn padre(&self) -> ReferenciaNodoPrestada<'_> {
        ReferenciaNodoPrestada {
            id: self.parent_id.as_deref(),
            titulo: self.parent_title.as_deref(),
        }
    }
}

impl From<ActualizacionNodo> for ActualizacionTipada {
    fn from(valor: ActualizacionNodo) -> Self {
        Self {
            nodo: valor.nodo,
            parent_id: valor.parent_id,
            parent_title: valor.parent_title,
            estado: reconocer(valor.status, EstadoNodo::reconocer_texto),
            notas: valor.notes,
            prioridad: reconocer(valor.priority, PrioridadNodo::reconocer_texto),
            rol: reconocer(valor.role, RolNodo::reconocer_texto),
            etiquetas: valor.tags.unwrap_or_default(),
            ruta_archivo: valor.file_path,
        }
    }
}

/// Conexión lista para resolverse contra el mapa.
#[derive(Debug)]
pub(crate) struct ConexionTipada {
    /// UUID opcional del origen.
    pub(crate) from_id: Option<String>,
    /// UUID opcional del destino.
    pub(crate) to_id: Option<String>,
    /// Título opcional del origen.
    pub(crate) from_title: Option<String>,
    /// Título opcional del destino.
    pub(crate) to_title: Option<String>,
    /// Explicación visible de la relación.
    pub(crate) etiqueta: Option<String>,
    /// Tipo de relación reconocido, ausente o desconocido.
    pub(crate) relacion: ValorEnumerado<TipoRelacion>,
}

impl ConexionTipada {
    /// Devuelve la referencia prestada del origen.
    pub(crate) fn origen(&self) -> ReferenciaNodoPrestada<'_> {
        ReferenciaNodoPrestada {
            id: self.from_id.as_deref(),
            titulo: self.from_title.as_deref(),
        }
    }

    /// Devuelve la referencia prestada del destino.
    pub(crate) fn destino(&self) -> ReferenciaNodoPrestada<'_> {
        ReferenciaNodoPrestada {
            id: self.to_id.as_deref(),
            titulo: self.to_title.as_deref(),
        }
    }
}

impl From<ConexionSolicitada> for ConexionTipada {
    fn from(valor: ConexionSolicitada) -> Self {
        Self {
            from_id: valor.from_id,
            to_id: valor.to_id,
            from_title: valor.from_title,
            to_title: valor.to_title,
            etiqueta: valor.label,
            relacion: reconocer(valor.relation_type, TipoRelacion::reconocer_texto),
        }
    }
}

/// Lote completo una vez superadas la deserialización y la validación de identidad.
#[derive(Debug)]
pub(crate) struct SincronizacionTipada {
    /// Ruta relativa del mapa que recibirá los cambios.
    pub(crate) file_path: String,
    /// Nombre opcional del agente para el recibo de devolución.
    pub(crate) agent_name: Option<String>,
    /// Actualizaciones convertidas una sola vez a tipos del dominio.
    pub(crate) node_updates: Vec<ActualizacionTipada>,
    /// Conexiones convertidas una sola vez a tipos del dominio.
    pub(crate) cross_connections: Vec<ConexionTipada>,
}

impl From<ArgumentosSincronizar> for SincronizacionTipada {
    fn from(valor: ArgumentosSincronizar) -> Self {
        Self {
            file_path: valor.file_path,
            agent_name: valor.agent_name,
            node_updates: valor.node_updates.into_iter().map(Into::into).collect(),
            cross_connections: valor
                .cross_connections
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
