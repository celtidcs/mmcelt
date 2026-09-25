//! Contratos de entrada de las herramientas MCP.
//!
//! Este módulo es la frontera entre el JSON que entrega un agente y las operaciones que
//! pueden leer o modificar un mapa. Después de deserializar estos tipos, el resto del servidor
//! trabaja con campos Rust conocidos; no consulta claves sueltas de `serde_json::Value`.

use serde::Deserialize;

/// Herramienta que no recibe opciones.
#[derive(Debug, Deserialize)]
pub(crate) struct ArgumentosSinContenido {}

/// Ruta común a las herramientas que leen un mapa.
#[derive(Debug, Deserialize)]
pub(crate) struct ArgumentosLeerMapa {
    /// Ruta relativa del mapa dentro del espacio autorizado.
    pub(crate) file_path: String,
}

/// Datos necesarios para crear un mapa nuevo.
#[derive(Debug, Deserialize)]
pub(crate) struct ArgumentosCrearMapa {
    /// Ruta relativa donde se creará el archivo `.mmcelt`.
    pub(crate) file_path: String,
    /// Nombre del agente que quedará en el recibo de devolución.
    pub(crate) agent_name: Option<String>,
    /// Título general del proyecto nuevo.
    pub(crate) title: String,
    /// Visión de la persona creadora que el agente debe preservar.
    pub(crate) creator_vision: Option<String>,
    /// Objetivos del proyecto que justifican el mapa.
    pub(crate) project_goals: Option<String>,
    /// Público o contexto de uso al que se dirige el proyecto.
    pub(crate) target_audience: Option<String>,
    /// Jerarquía y conexiones propuestas con el contrato común de importación.
    pub(crate) mindmap_data: crate::ai_bridge::AiJsonProject,
}

/// Una referencia a un nodo, válida tanto para actualizaciones como para extremos de aristas.
#[derive(Debug, Deserialize)]
pub(crate) struct ReferenciaNodo {
    /// UUID del nodo, que tiene prioridad cuando también llega un título.
    pub(crate) id: Option<String>,
    /// Título que basta si es único y sirve para comprobar un UUID recibido.
    pub(crate) title: Option<String>,
}

impl ReferenciaNodo {
    /// Comprueba que la referencia contiene al menos una identidad no vacía.
    pub(crate) fn validar(&self, contexto: &str) -> Result<(), String> {
        let tiene_id = self.id.as_deref().is_some_and(|id| !id.trim().is_empty());
        let tiene_titulo = self
            .title
            .as_deref()
            .is_some_and(|titulo| !titulo.trim().is_empty());
        if tiene_id || tiene_titulo {
            Ok(())
        } else {
            Err(format!(
                "{contexto} debe indicar un «id» o un «title» no vacío"
            ))
        }
    }
}

/// Cambio solicitado sobre un nodo existente o nuevo.
#[derive(Debug, Deserialize)]
pub(crate) struct ActualizacionNodo {
    /// Identidad del nodo a modificar o crear.
    #[serde(flatten)]
    pub(crate) nodo: ReferenciaNodo,
    /// UUID del padre solicitado para un nodo nuevo.
    pub(crate) parent_id: Option<String>,
    /// Título del padre, usado si es único o para contrastar su UUID.
    pub(crate) parent_title: Option<String>,
    /// Nombre canónico del nuevo estado de trabajo.
    pub(crate) status: Option<String>,
    /// Notas que el agente propone añadir al nodo.
    pub(crate) notes: Option<String>,
    /// Nombre canónico de la prioridad solicitada.
    pub(crate) priority: Option<String>,
    /// Nombre canónico del rol arquitectónico solicitado.
    pub(crate) role: Option<String>,
    /// Etiquetas que se sumarán sin borrar las escritas por la persona.
    pub(crate) tags: Option<Vec<String>>,
    /// Ruta de código propuesta para el nodo.
    pub(crate) file_path: Option<String>,
}

impl ActualizacionNodo {
    /// Valida las combinaciones que JSON Schema expresa mediante `anyOf`.
    pub(crate) fn validar(&self, posicion: usize) -> Result<(), String> {
        self.nodo
            .validar(&format!("La actualización número {}", posicion + 1))
    }
}

/// Referencia prestada para que la lógica de identidad no dependa de nombres JSON.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReferenciaNodoPrestada<'a> {
    /// UUID prestado sin copiar el texto recibido.
    pub(crate) id: Option<&'a str>,
    /// Título prestado y ya separado del nombre JSON exterior.
    pub(crate) titulo: Option<&'a str>,
}

impl ReferenciaNodo {
    /// Vista prestada de la referencia recibida.
    pub(crate) fn prestada(&self) -> ReferenciaNodoPrestada<'_> {
        ReferenciaNodoPrestada {
            id: self.id.as_deref(),
            titulo: self.title.as_deref(),
        }
    }
}

/// Conexión cruzada solicitada por el agente.
#[derive(Debug, Deserialize)]
pub(crate) struct ConexionSolicitada {
    /// UUID del extremo de origen, si el agente lo conoce.
    pub(crate) from_id: Option<String>,
    /// UUID del extremo de destino, si el agente lo conoce.
    pub(crate) to_id: Option<String>,
    /// Título del origen como alternativa o comprobación del UUID.
    pub(crate) from_title: Option<String>,
    /// Título del destino como alternativa o comprobación del UUID.
    pub(crate) to_title: Option<String>,
    /// Motivo visible de la relación.
    pub(crate) label: Option<String>,
    /// Nombre canónico del tipo de relación.
    pub(crate) relation_type: Option<String>,
}

impl ConexionSolicitada {
    /// Devuelve la referencia prestada del extremo de origen.
    pub(crate) fn origen(&self) -> ReferenciaNodoPrestada<'_> {
        ReferenciaNodoPrestada {
            id: self.from_id.as_deref(),
            titulo: self.from_title.as_deref(),
        }
    }

    /// Devuelve la referencia prestada del extremo de destino.
    pub(crate) fn destino(&self) -> ReferenciaNodoPrestada<'_> {
        ReferenciaNodoPrestada {
            id: self.to_id.as_deref(),
            titulo: self.to_title.as_deref(),
        }
    }

    /// Comprueba que ambos extremos contienen UUID, título o las dos cosas.
    ///
    /// # Errores
    ///
    /// Devuelve una explicación que identifica la posición y el extremo incompleto.
    pub(crate) fn validar(&self, posicion: usize) -> Result<(), String> {
        validar_referencia_prestada(
            self.origen(),
            &format!("El origen de la conexión número {}", posicion + 1),
        )?;
        validar_referencia_prestada(
            self.destino(),
            &format!("El destino de la conexión número {}", posicion + 1),
        )
    }
}

fn validar_referencia_prestada(
    referencia: ReferenciaNodoPrestada<'_>,
    contexto: &str,
) -> Result<(), String> {
    let tiene_id = referencia.id.is_some_and(|id| !id.trim().is_empty());
    let tiene_titulo = referencia
        .titulo
        .is_some_and(|titulo| !titulo.trim().is_empty());
    if tiene_id || tiene_titulo {
        Ok(())
    } else {
        Err(format!(
            "{contexto} debe indicar un identificador o un título no vacío"
        ))
    }
}

/// Lote de cambios que se aplica sobre un mapa ya existente.
#[derive(Debug, Deserialize)]
pub(crate) struct ArgumentosSincronizar {
    /// Ruta relativa del mapa que recibirá el lote.
    pub(crate) file_path: String,
    /// Nombre que se incluirá en el recibo de devolución.
    pub(crate) agent_name: Option<String>,
    /// Cambios de nodos, aplicados solo después de validar el lote entero.
    pub(crate) node_updates: Vec<ActualizacionNodo>,
    /// Relaciones laterales que se añadirán sin duplicar las existentes.
    #[serde(default)]
    pub(crate) cross_connections: Vec<ConexionSolicitada>,
}

impl ArgumentosSincronizar {
    /// Valida las reglas de identidad que no puede representar un `derive(Deserialize)`.
    pub(crate) fn validar(&self) -> Result<(), String> {
        for (posicion, actualizacion) in self.node_updates.iter().enumerate() {
            actualizacion.validar(posicion)?;
        }
        for (posicion, conexion) in self.cross_connections.iter().enumerate() {
            conexion.validar(posicion)?;
        }
        Ok(())
    }
}

/// Origen y destino de la exportación del documento para IA.
#[derive(Debug, Deserialize)]
pub(crate) struct ArgumentosExportar {
    /// Ruta relativa del mapa que se va a leer.
    pub(crate) mmcelt_file_path: String,
    /// Ruta relativa del documento Markdown que se va a escribir.
    pub(crate) output_md_path: String,
}
