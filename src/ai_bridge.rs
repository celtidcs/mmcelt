//! # Módulo de Puente Bidireccional IA (`ai_bridge.rs`)
//!
//! Facilita la comunicación desde cualquier modelo de Inteligencia Artificial (ChatGPT, Claude, Gemini)
//! hacia MMCelt:
//! - **Prompt Maestro (`prompt_maestro_para_ia`)**: Especificación formal de esquema JSON para alimentar a los LLMs.
//! - **Parser Tolerante a Fallos (`importar_de_texto_de_ia`)**: Extrae bloques JSON o esquemas Markdown de respuestas
//!   conversacionales complejas y reconstruye el árbol del mapa mental con auto-disposición automática.

use crate::error::{AppError, AppResult, FalloEsquemaMarkdown, FalloJsonIa};
use crate::layout::aplicar_disposicion_automatica;
use crate::model::{
    EstadoNodo, EstadoRevision, PrioridadNodo, Proyecto, RolNodo, TipoRelacion, PROFUNDIDAD_MAXIMA,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Devuelve la instrucción / prompt maestro formateado para copiar y pegar en ChatGPT, Claude o Gemini.
pub fn prompt_maestro_para_ia() -> &'static str {
    r#"### 🤖 INSTRUCCIÓN / PROMPT MAESTRO PARA CONVERTIR CUALQUIER PROYECTO EN UN MAPA MENTAL MMCELT

Copia y pega este mensaje en cualquier modelo de Inteligencia Artificial (ChatGPT, Claude, Gemini, DeepSeek, etc.) junto con la información de tu proyecto o conversación:

---
"Actúa como un Arquitecto de Información y Estratega de Proyectos. Necesito que analices el proyecto/documento/conversación que te proporcionaré a continuación y lo conviertas en un Mapa Mental estructurado y compatible con la aplicación MMCelt.

Cuando el proyecto tenga código, añade a cada nodo el campo `file_path` con la ruta del archivo o la carpeta que le corresponde. Es lo que permite luego seguir el mapa hasta el código real.

Genera ÚNICAMENTE un bloque de código JSON con la siguiente estructura exacta:

```json
{
  "title": "Nombre del Proyecto",
  "creator_vision": "Explicación detallada de lo que se busca plasmar y la visión estratégica",
  "project_goals": "Objetivos concretos, hitos y resultados esperados",
  "target_audience_or_context": "Público objetivo, usuarios o entorno de aplicación",
  "root_node": {
    "title": "Idea Central",
    "notes": "Descripción conceptual central",
    "children": [
      {
        "title": "Pilar 1: Arquitectura Técnica",
        "notes": "Detalles del pilar técnico...",
        "file_path": "src/backend/",
        "role": "PilarEstrategico",
        "status": "EnProgreso",
        "priority": "Alta",
        "tags": ["backend", "rust"],
        "children": [
          {
            "title": "¿Qué base de datos usar?",
            "notes": "Evaluar SQLite vs PostgreSQL según carga esperada",
            "role": "HipotesisDuda",
            "status": "DudaBloqueo",
            "priority": "Critica",
            "tags": ["database", "duda"]
          }
        ]
      },
      {
        "title": "Pilar 2: Modelo de Negocio",
        "notes": "Estrategia de monetización y distribución",
        "role": "PilarEstrategico",
        "status": "Idea",
        "priority": "Media",
        "tags": ["negocio"]
      }
    ]
  },
  "cross_connections": [
    {
      "from_title": "¿Qué base de datos usar?",
      "to_title": "Pilar 2: Modelo de Negocio",
      "label": "Costes de hosting afectan al precio",
      "relation_type": "Dependencia"
    }
  ]
}
```

Valores válidos para los campos:
- `role`: "IdeaCentral", "PilarEstrategico", "Subtema", "HipotesisDuda", "AccionTarea", "RecursoHerramienta"
- `status`: "Idea", "Investigando", "EnProgreso", "DudaBloqueo", "Completado", "Descartado"
- `priority`: "Baja", "Media", "Alta", "Critica"
- `relation_type`: "Dependencia", "InspiradoPor", "Bloquea", "AlternativaA", "Sinergia"

Aquí tienes la información de mi proyecto para convertir:
[PEGA AQUÍ TU TEXTO, CÓDIGO O DESCRIPCIÓN DEL PROYECTO]"
---"#
}

/// Proyecto jerárquico tal como lo entrega una IA antes de convertirlo al dominio de MMCelt.
///
/// Todos los campos salvo el nodo raíz son opcionales para poder importar respuestas parciales sin
/// inventar información. La conversión posterior aplica valores seguros y valida la estructura.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AiJsonProject {
    /// Título general propuesto para el proyecto.
    pub title: Option<String>,
    /// Visión o motivación de la persona creadora.
    pub creator_vision: Option<String>,
    /// Objetivos que debe satisfacer el proyecto.
    pub project_goals: Option<String>,
    /// Público objetivo o contexto de aplicación.
    ///
    /// Admite también `target_audience`, que es como lo llama el esquema del servidor MCP. El
    /// mismo dato tenía dos nombres según la puerta por la que entrara, y cruzarlos lo perdía
    /// **en silencio**: serde no se queja de un campo que no conoce, así que un JSON escrito
    /// siguiendo el esquema del servidor y pegado en el modal de importación llegaba sin
    /// público objetivo y sin ningún aviso.
    #[serde(alias = "target_audience")]
    pub target_audience_or_context: Option<String>,
    /// Autor declarado en la respuesta, si lo aporta el modelo.
    pub author: Option<String>,
    /// El nodo raíz del mapa.
    ///
    /// La clave del JSON sigue siendo `root_node` y no se traduce: es el contrato que
    /// publican la habilidad para agentes, el esquema del servidor MCP y las plantillas de
    /// integración. El nombre del campo en Rust va en español como el resto del código; el
    /// nombre en el archivo es otra cosa, y cambiarlo dejaría fuera a todo mapa generado
    /// por un modelo que siga la documentación publicada.
    #[serde(rename = "root_node")]
    pub nodo_raiz: Option<AiJsonNode>,
    /// Relaciones adicionales entre ramas distintas del árbol.
    pub cross_connections: Option<Vec<AiJsonConnection>>,
}

/// Nodo recursivo del contrato de importación desde IA.
///
/// Conserva texto y metadatos como valores de entrada hasta que la conversión los reconozca como
/// tipos del dominio. `children` expresa únicamente jerarquía; las relaciones laterales viven en
/// [`AiJsonProject::cross_connections`].
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AiJsonNode {
    /// Texto visible del nodo; es el único campo obligatorio.
    pub title: String,
    /// Explicación extensa o contexto asociado.
    pub notes: Option<String>,
    /// Nombre canónico del rol solicitado.
    pub role: Option<String>,
    /// Nombre canónico del estado de trabajo.
    pub status: Option<String>,
    /// Nombre canónico de la prioridad.
    pub priority: Option<String>,
    /// Etiquetas libres propuestas para clasificar el nodo.
    pub tags: Option<Vec<String>>,

    /// Ruta del archivo de código que representa el nodo.
    ///
    /// Faltaba, y era una pérdida silenciosa: la exportación escribe este campo, la
    /// habilidad para agentes se lo pide al modelo y el escáner de repositorios lo rellena,
    /// pero al reimportar el mapa se descartaba. Quien mandaba un mapa a la IA, se lo
    /// devolvía ampliado y lo volvía a abrir, perdía todas las rutas de código: justamente
    /// lo que distingue a MMCelt de un programa de mapas mentales corriente.
    pub file_path: Option<String>,

    /// Estado de revisión humana del nodo.
    ///
    /// Permite que un agente devuelva el mapa marcando lo que ha generado él como
    /// `GeneradoPorIA`, en lugar de que todo llegue indistinguible de lo que escribió la
    /// persona.
    pub review_status: Option<String>,

    /// Corrección exigida por el usuario sobre ese nodo.
    ///
    /// Viaja en los dos sentidos: la persona la escribe y la IA debe acatarla, pero
    /// también permite que el agente devuelva el mapa conservando las correcciones que ya
    /// tenía en lugar de borrarlas al reescribirlo.
    pub correction_feedback: Option<String>,

    /// Descendencia jerárquica, en el orden propuesto por el agente.
    pub children: Option<Vec<AiJsonNode>>,
}

/// Relación lateral recibida por títulos en el contrato de importación.
///
/// Se resuelve contra el mapa después de crear la jerarquía. Los títulos ambiguos o inexistentes
/// se omiten de forma controlada y nunca crean extremos inventados.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AiJsonConnection {
    /// Título del nodo de origen.
    pub from_title: String,
    /// Título del nodo de destino.
    pub to_title: String,
    /// Explicación legible de por qué están relacionados.
    pub label: Option<String>,
    /// Nombre canónico del tipo de relación.
    pub relation_type: Option<String>,
}

/// Construye un mapa mental a partir del texto que el usuario pega desde un modelo de IA.
///
/// Es la frontera de entrada de **datos no confiables**: el texto procede de un tercero
/// (ChatGPT, Claude, Gemini…) y puede contener cualquier cosa, incluidas respuestas
/// truncadas, alucinadas o deliberadamente malformadas.
///
/// La interpretación se intenta en dos pasos:
/// 1. Como bloque JSON con el esquema del prompt maestro, que es el formato pedido.
/// 2. Si eso falla, como esquema Markdown de títulos y viñetas, para aprovechar
///    respuestas conversacionales que no siguieron el formato.
///
/// El proyecto resultante se valida antes de devolverse, igual que si viniera de un
/// archivo.
///
/// Parsea una respuesta generada por una Inteligencia Artificial y la convierte en un [`Proyecto`].
///
/// Soporta dos vías de entrada ordenadas por especificidad:
/// 1. **Bloque JSON nativo**: Extraído de bloques de código markdown ```json ... ``` o JSON en texto plano.
/// 2. **Esquema Markdown estructurado**: Detección jerárquica de encabezados (`#`, `##`) y listas (`-`, `*`)
///    como mecanismo de respaldo si el modelo no generó JSON válido.
///
/// # Errores
/// - [`AppError::ImportacionIa`] si ninguna de las dos interpretaciones da resultado.
///   Conserva por separado el error del análisis JSON y el del esquema Markdown, porque saber cuál falló y
///   por qué es lo que permite al usuario corregir su entrada.
/// - [`AppError::EstructuraInvalida`] si el mapa reconstruido no supera la validación.
pub fn importar_de_texto_de_ia(input: &str) -> AppResult<Proyecto> {
    let clean_input = input.trim();
    if clean_input.is_empty() {
        return Err(AppError::ImportacionIa {
            error_json: FalloJsonIa::TextoVacio,
            error_esquema: FalloEsquemaMarkdown::TextoVacio,
        });
    }

    // Primero como JSON: admite tanto JSON suelto como bloques ```json ... ```.
    let json_str = extraer_bloque_json(clean_input);

    let proyecto = match serde_json::from_str::<AiJsonProject>(&json_str) {
        Ok(ai_proj) => construir_proyecto_desde_json(ai_proj)?,
        Err(json_err) => {
            // Respaldo: esquema Markdown de títulos y viñetas.
            interpretar_esquema_markdown(clean_input).map_err(|outline_err| {
                AppError::ImportacionIa {
                    error_json: FalloJsonIa::Sintaxis(json_err.to_string()),
                    error_esquema: outline_err,
                }
            })?
        }
    };

    // Misma frontera de confianza que en la carga de archivos: lo que devolvamos aquí
    // pasa directamente al lienzo y a los recorridos de `layout`.
    proyecto.validar_estructura()?;

    // El tope de longitud se aplica aquí, en lo que llega de una IA, y no al abrir un
    // archivo del usuario. Ver `Proyecto::validar_longitudes_de_texto`.
    proyecto.validar_longitudes_de_texto()?;

    Ok(proyecto)
}

/// Extrae el bloque JSON de una respuesta conversacional de IA.
///
/// Los modelos rara vez devuelven JSON limpio: suelen envolverlo en un bloque de
/// código y rodearlo de texto explicativo. Esta funcion prueba tres estrategias, de
/// la mas fiable a la mas tolerante:
/// 1. Un bloque delimitado por ```` ```json ````.
/// 2. Un bloque delimitado por ``` ``` ``` cuyo contenido empiece por `{`.
/// 3. El fragmento comprendido entre la primera `{` y la ultima `}` del texto.
///
/// # Parámetros
/// - `text`: respuesta completa del modelo.
///
/// # Devuelve
/// El fragmento con mas probabilidades de ser JSON, o el texto original si ninguna
/// estrategia encuentra nada. No garantiza que el resultado sea JSON válido: de eso
/// se encarga quien lo analiza después.
fn extraer_bloque_json(text: &str) -> String {
    if let Some(start) = text.find("```json") {
        if let Some(end) = text[start + 7..].find("```") {
            return text[start + 7..start + 7 + end].trim().to_string();
        }
    } else if let Some(start) = text.find("```") {
        if let Some(end) = text[start + 3..].find("```") {
            let interior = text[start + 3..start + 3 + end].trim();
            if interior.starts_with('{') {
                return interior.to_string();
            }
        }
    }

    if let (Some(first_brace), Some(last_brace)) = (text.find('{'), text.rfind('}')) {
        if first_brace < last_brace {
            return text[first_brace..=last_brace].trim().to_string();
        }
    }

    text.to_string()
}

// Las tablas de equivalencias de `role`, `status`, `priority` y `relation_type` vivían
// aquí duplicadas. Ahora residen en el propio módulo `model`, junto a las
// enumeraciones que interpretan, y las comparten la deserialización de archivos, este
// importador y —por contrato documentado— el servidor MCP. Estas envolturas se limitan
// a adaptar el `Option<&str>` que produce el JSON de la IA.

/// Interpreta el campo `role` de un nodo escrito por una IA.
fn interpretar_rol(role_str: Option<&str>) -> RolNodo {
    RolNodo::desde_texto(role_str.unwrap_or(""))
}

/// Interpreta el campo `status` de un nodo escrito por una IA.
fn interpretar_estado(status_str: Option<&str>) -> EstadoNodo {
    EstadoNodo::desde_texto(status_str.unwrap_or(""))
}

/// Interpreta el campo `priority` de un nodo escrito por una IA.
fn interpretar_prioridad(prio_str: Option<&str>) -> PrioridadNodo {
    PrioridadNodo::desde_texto(prio_str.unwrap_or(""))
}

/// Interpreta el campo `relation_type` de una conexión escrita por una IA.
fn interpretar_tipo_de_relacion(rel_str: Option<&str>) -> TipoRelacion {
    TipoRelacion::desde_texto(rel_str.unwrap_or(""))
}

/// Reconstruye el proyecto a partir del JSON ya deserializado de la IA.
///
/// # Errores
/// Devuelve [`AppError::ImportacionIa`] si el JSON no contiene un `root_node`, sin el
/// cual no hay mapa que construir.
pub(crate) fn construir_proyecto_desde_json(ai_proj: AiJsonProject) -> AppResult<Proyecto> {
    let title = ai_proj.title.unwrap_or_else(|| "Proyecto IA".to_string());
    let mut proyecto = Proyecto::nuevo_vacio(title.clone());
    proyecto.title = title;
    proyecto.creator_vision = ai_proj.creator_vision.unwrap_or_default();
    proyecto.project_goals = ai_proj.project_goals.unwrap_or_default();
    proyecto.target_audience_or_context = ai_proj.target_audience_or_context.unwrap_or_default();
    proyecto.author = ai_proj.author.unwrap_or_else(|| "IA Copilot".to_string());

    let mut title_to_uuid: HashMap<String, Uuid> = HashMap::new();

    // Sin nodo raíz no hay mapa. Antes se devolvía en silencio un proyecto vacío que
    // el usuario recibía como una importación «correcta».
    if ai_proj.nodo_raiz.is_none() {
        return Err(AppError::ImportacionIa {
            error_json: FalloJsonIa::FaltaNodoRaiz,
            error_esquema: FalloEsquemaMarkdown::NoIntentadoPorJsonIncompleto,
        });
    }

    if let Some(ai_root) = ai_proj.nodo_raiz {
        let root_id = proyecto.root_id;
        if let Some(nodo_raiz) = proyecto.nodes.get_mut(&root_id) {
            nodo_raiz.title = ai_root.title.clone();
            nodo_raiz.notes = ai_root.notes.unwrap_or_default();
            nodo_raiz.role = RolNodo::IdeaCentral;
            nodo_raiz.status = interpretar_estado(ai_root.status.as_deref());
            nodo_raiz.priority = interpretar_prioridad(ai_root.priority.as_deref());
            if let Some(tags) = ai_root.tags {
                nodo_raiz.tags = tags;
            }

            // La raíz recibe los mismos tres campos que los hijos. Al ampliar la
            // importación se aplicaron solo en `insertar_subnodos_de_ia`, de modo que un mapa
            // cuya raíz traía ruta de código o una corrección exigida los perdía.
            if let Some(ruta) = ai_root.file_path.filter(|r| !r.trim().is_empty()) {
                nodo_raiz.file_path = Some(ruta.trim().to_string());
            }
            if let Some(revision) = ai_root.review_status {
                nodo_raiz.review_status = revision_sin_firmar(&revision);
            }
            if let Some(correccion) = ai_root.correction_feedback.filter(|c| !c.trim().is_empty()) {
                nodo_raiz.correction_feedback = correccion.trim().to_string();
            }
        }
        title_to_uuid.insert(ai_root.title.trim().to_lowercase(), root_id);

        if let Some(children) = ai_root.children {
            for child_ai in children {
                insertar_subnodos_de_ia(&mut proyecto, root_id, child_ai, 1, &mut title_to_uuid);
            }
        }
    }

    // Las conexiones cruzadas, que no son jerarquía y viajan aparte.
    if let Some(conns) = ai_proj.cross_connections {
        for conn in conns {
            let from_key = conn.from_title.trim().to_lowercase();
            let to_key = conn.to_title.trim().to_lowercase();

            if let (Some(&from_id), Some(&to_id)) =
                (title_to_uuid.get(&from_key), title_to_uuid.get(&to_key))
            {
                proyecto.anadir_conexion_cruzada(
                    from_id,
                    to_id,
                    conn.label.unwrap_or_default(),
                    interpretar_tipo_de_relacion(conn.relation_type.as_deref()),
                );
            }
        }
    }

    aplicar_disposicion_automatica(&mut proyecto);
    Ok(proyecto)
}

/// Marca el mapa entero como trabajo de la IA, sin rastro de supervisión humana.
///
/// Lo usa el servidor MCP al crear un mapa: quien lo construye es un agente, así que **todo**
/// lo que hay dentro lo ha escrito la IA. Ni un solo nodo puede venir revisado, y ninguno
/// puede traer una corrección exigida por el usuario, porque el usuario no ha visto todavía
/// ese mapa.
///
/// Antes esta función respetaba cualquier estado distinto de «pendiente», con la idea de no
/// pisar mapas ya revisados. Era un agujero: bastaba con que el agente escribiera
/// `"review_status": "approved"` para que sus nodos salieran firmados por el usuario y
/// `mmcelt_get_human_feedback` los devolviera como aprobados.
///
/// El caso que aquella condición pretendía cubrir —un mapa que ya tenía revisiones— no pasa
/// por aquí: actualizar un mapa existente es cosa de `mmcelt_sync_ai_progress`, que conserva
/// los estados y hace caducar la aprobación solo cuando la IA cambia el contenido.
///
/// # Parámetros
/// - `proyecto`: el mapa recién construido, cuyos nodos se marcan en el sitio.
pub(crate) fn marcar_todo_como_trabajo_de_la_ia(proyecto: &mut Proyecto) {
    for nodo in proyecto.nodes.values_mut() {
        nodo.review_status = crate::model::EstadoRevision::GeneradoPorIA;
        nodo.correction_feedback.clear();
    }
}

/// Interpreta el estado de revisión que llega en un texto de la IA, sin permitir que firme.
///
/// La aprobación humana **solo puede darla una persona**, en el inspector. Es la única
/// afirmación del mapa que dice «esto lo he leído yo y lo doy por bueno», y por eso ningún
/// texto de entrada puede establecerla: ni el JSON que pega el usuario, ni el que escribe un
/// agente por el servidor MCP.
///
/// Sin este filtro, bastaba con que un modelo devolviera `"review_status": "approved"` para
/// que sus propios nodos aparecieran en el inspector con el visto bueno del usuario, y para
/// que `mmcelt_get_human_feedback` se los devolviera como «aprobados por el usuario». El
/// programa existe para lo contrario.
///
/// # Parámetros
/// - `texto`: el estado tal como lo escribió el modelo.
///
/// # Por qué la corrección sí se acepta y la aprobación no
///
/// Los dos campos hablan del usuario, pero no afirman lo mismo. La aprobación dice «esto lo
/// he leído y lo doy por bueno»: es autoridad, y si la escribe el modelo, se la está
/// concediendo a sí mismo. La corrección dice «no hagas esto»: es una restricción, y un
/// modelo que se la inventa se está atando las manos, no soltándoselas.
///
/// Además, descartarla tendría un coste real: el usuario escribe la corrección, la exporta,
/// el modelo la acata y devuelve el mapa; si al reimportar se perdiera, desaparecería la
/// orden que permite comprobar si el modelo hizo lo que se le pidió.
///
/// En el servidor MCP el criterio es otro y más estricto, porque allí no hay nadie mirando:
/// un mapa que crea un agente nace sin correcciones, ya que el usuario no lo ha visto
/// todavía. Ver `marcar_todo_como_trabajo_de_la_ia`.
///
/// # Devuelve
/// El estado interpretado, con la aprobación degradada a pendiente de revisión.
fn revision_sin_firmar(texto: &str) -> EstadoRevision {
    match EstadoRevision::desde_texto(texto) {
        // Que un modelo diga que algo está aprobado significa, como mucho, que cree que
        // debería estarlo. Queda pendiente de que lo mire una persona.
        EstadoRevision::AprobadoPorHumano => EstadoRevision::PendienteRevision,
        otro => otro,
    }
}

/// Inserta recursivamente un nodo de la IA y toda su descendencia en el proyecto.
///
/// # Límite de profundidad
///
/// El descenso se detiene al alcanzar [`PROFUNDIDAD_MAXIMA`]. Hoy `serde_json` ya
/// rechaza antes los documentos con más de 128 niveles de anidamiento, así que este
/// límite no llega a activarse por esa vía; existe porque **no debe depender de un
/// detalle de implementación de una biblioteca externa** que puede cambiar de versión
/// o de valor por defecto. Sin él, un JSON suficientemente anidado agotaría la pila.
///
/// # Parámetros
/// - `proyecto`: proyecto en construcción.
/// - `parent_id`: nodo bajo el que se cuelga el que se va a crear.
/// - `ai_node`: nodo tal como lo describió la IA.
/// - `profundidad`: profundidad actual, usada para asignar el rol por defecto y para el
///   control del límite.
/// - `mapa_de_titulos`: índice de título a identificador, necesario después para resolver
///   las conexiones cruzadas, que la IA expresa por título y no por identificador.
fn insertar_subnodos_de_ia(
    proyecto: &mut Proyecto,
    parent_id: Uuid,
    ai_node: AiJsonNode,
    profundidad: usize,
    mapa_de_titulos: &mut HashMap<String, Uuid>,
) {
    if profundidad > PROFUNDIDAD_MAXIMA {
        return;
    }

    let title = ai_node.title.trim().to_string();
    let id_del_hijo = proyecto.anadir_hijo(parent_id, title.clone());
    mapa_de_titulos.insert(title.to_lowercase(), id_del_hijo);

    if let Some(node) = proyecto.nodes.get_mut(&id_del_hijo) {
        node.notes = ai_node.notes.unwrap_or_default();
        node.role = if let Some(r) = ai_node.role {
            interpretar_rol(Some(&r))
        } else if profundidad == 1 {
            RolNodo::PilarEstrategico
        } else {
            RolNodo::Subtema
        };
        node.status = interpretar_estado(ai_node.status.as_deref());
        node.priority = interpretar_prioridad(ai_node.priority.as_deref());
        if let Some(tags) = ai_node.tags {
            node.tags = tags;
        }

        // Los tres campos siguientes se aceptan solo si vienen con contenido: un valor
        // vacío significa que el modelo no dijo nada al respecto, y sobrescribir con vacío
        // sería peor que dejar el valor por omisión.
        if let Some(ruta) = ai_node.file_path.filter(|r| !r.trim().is_empty()) {
            node.file_path = Some(ruta.trim().to_string());
        }
        if let Some(revision) = ai_node.review_status {
            node.review_status = revision_sin_firmar(&revision);
        }
        if let Some(correccion) = ai_node.correction_feedback.filter(|c| !c.trim().is_empty()) {
            node.correction_feedback = correccion.trim().to_string();
        }
    }

    if let Some(children) = ai_node.children {
        for subchild in children {
            insertar_subnodos_de_ia(
                proyecto,
                id_del_hijo,
                subchild,
                profundidad + 1,
                mapa_de_titulos,
            );
        }
    }
}

/// Cuántos espacios de sangría equivalen a un nivel de anidamiento en una lista Markdown.
///
/// Dos es la convención habitual y la que usan los modelos al responder. Una sangría de
/// cuatro cuenta como dos niveles, que es lo que su autor quiso decir.
const ESPACIOS_POR_NIVEL: usize = 2;

/// Deja la pila lista para colgar de ella algo del nivel indicado.
///
/// Quita del final todo lo que esté a ese nivel o por debajo, de forma que la cima pase a ser
/// el antepasado que corresponde. La raíz nunca se quita: siempre tiene que quedar alguien de
/// quien colgar.
///
/// # Parámetros
/// - `pila`: pares de nivel e identificador, del más superficial al más profundo.
/// - `nivel`: el nivel de lo que se va a añadir.
fn desapilar_hasta(pila: &mut Vec<(usize, Uuid)>, nivel: usize) {
    while pila.len() > 1 {
        let Some((nivel_en_la_cima, _)) = pila.last() else {
            return;
        };
        if *nivel_en_la_cima < nivel {
            return;
        }
        pila.pop();
    }
}

/// Interpreta el texto como un esquema Markdown de títulos (`#`) y viñetas (`-`, `*`).
///
/// Es el respaldo que se intenta cuando el análisis JSON fracasa, para aprovechar las
/// respuestas conversacionales que no siguieron el formato del prompt maestro.
///
/// # Devuelve
/// El proyecto reconstruido a partir del esquema.
///
/// # Errores
/// Devuelve error si no consigue extraer **ningún nodo** además de la raíz.
///
/// Esta comprobación es deliberada. Antes, la función solo fallaba con el texto vacío
/// y devolvía «éxito» ante cualquier otra entrada, normalmente un mapa con un único
/// nodo. El usuario veía un mensaje de importación correcta y un lienzo vacío, sin
/// saber que su texto no se había entendido: un fallo silencioso, peor que un error.
fn interpretar_esquema_markdown(text: &str) -> Result<Proyecto, FalloEsquemaMarkdown> {
    let lineas_del_texto: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    if lineas_del_texto.is_empty() {
        return Err(FalloEsquemaMarkdown::SinLineasDeContenido);
    }

    let mut proyecto = Proyecto::nuevo_vacio("Mapa Mental Importado");

    // Títulos y viñetas se miden con la **misma** escala, y por eso pueden convivir en una
    // sola pila. Antes no era así: los títulos entraban todos con nivel 1 y las viñetas con
    // su sangría en espacios, de modo que una viñeta sin sangrar —lo normal en lo que
    // escribe un modelo— tenía nivel 0 y desalojaba al título antes de colgar de él. El
    // resultado era un mapa completamente plano, con todo colgando de la raíz. Y como los
    // títulos entraban todos al mismo nivel, `##` tampoco quedaba nunca bajo `#`.
    //
    // Esta es justo la vía de respaldo para aprovechar respuestas conversacionales que no
    // siguieron el formato JSON, así que fallaba precisamente en el caso para el que existe.
    let mut pila: Vec<(usize, Uuid)> = vec![(0, proyecto.root_id)];

    // Nivel del último título visto. Las viñetas cuelgan por debajo de él.
    let mut nivel_del_titulo = 0usize;
    let mut first_line = true;

    for line in lineas_del_texto {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            let nivel = trimmed.chars().take_while(|c| *c == '#').count();
            let title = trimmed.trim_start_matches('#').trim();

            if first_line {
                proyecto.title = title.to_string();
                if let Some(root) = proyecto.nodes.get_mut(&proyecto.root_id) {
                    root.title = title.to_string();
                }
                nivel_del_titulo = nivel;
                first_line = false;
                continue;
            }

            desapilar_hasta(&mut pila, nivel);
            let padre = pila.last().map(|(_, id)| *id).unwrap_or(proyecto.root_id);
            let id = proyecto.anadir_hijo(padre, title);
            pila.push((nivel, id));
            nivel_del_titulo = nivel;
            continue;
        }

        if trimmed.starts_with('-') || trimmed.starts_with('*') {
            let sangria = line.chars().take_while(|c| c.is_whitespace()).count();
            let title = trimmed
                .trim_start_matches('-')
                .trim_start_matches('*')
                .trim();

            // Una viñeta cuelga siempre por debajo del título vigente; su sangría solo
            // decide cuánto más abajo.
            let nivel = nivel_del_titulo + 1 + sangria / ESPACIOS_POR_NIVEL;

            desapilar_hasta(&mut pila, nivel);
            let padre = pila.last().map(|(_, id)| *id).unwrap_or(proyecto.root_id);
            let id_del_hijo = proyecto.anadir_hijo(padre, title);
            pila.push((nivel, id_del_hijo));
            first_line = false;
        }
    }

    // Solo la raíz significa que no se reconoció ni un título ni una viñeta: el texto
    // no era un esquema. Se informa en lugar de devolver un mapa vacío como si nada.
    if proyecto.nodes.len() <= 1 {
        return Err(FalloEsquemaMarkdown::SinTitulosNiVinetas);
    }

    aplicar_disposicion_automatica(&mut proyecto);
    Ok(proyecto)
}
