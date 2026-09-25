//! Catálogo público de herramientas MCP.
//!
//! Contiene exclusivamente los nombres, descripciones y esquemas que recibe el agente al
//! llamar a `tools/list`. Mantenerlos juntos permite comparar el contrato anunciado con los
//! DTO que valida `argumentos`.

use serde_json::{json, Value};

/// Lo que se le dice al agente sobre el campo `file_path` de cada herramienta.
///
/// Es una constante y no un texto repetido en cada una porque lo usan seis, y una
/// explicación de dónde puede escribir un agente no puede decir cosas distintas según la
/// herramienta por la que se pregunte.
const AYUDA_DEL_CAMPO_RUTA: &str =
    "Ruta del archivo, relativa al espacio de trabajo de MMCelt. No se admiten rutas que salgan de él.";

/// Devuelve la descripción de las herramientas para `tools/list`.
///
/// Los nombres y esquemas se mantienen estables: un agente ya configurado no debe
/// necesitar cambios si el servidor evoluciona.
pub(super) fn descripcion_herramientas() -> Value {
    json!([
        herramienta_informacion_del_espacio(),
        herramienta_crear_mapa(),
        herramienta_leer_mapa(),
        herramienta_leer_correcciones_humanas(),
        herramienta_sincronizar_progreso(),
        herramienta_exportar_markdown(),
    ])
}

/// La herramienta que dice dónde puede trabajar el agente y qué mapas hay allí.
///
/// Va la primera del listado a propósito: es la que responde a la pregunta con la que empieza
/// cualquier agente —«¿dónde puedo trabajar y qué hay ahí?»— y la que evita que las demás se
/// llamen con rutas inventadas.
///
/// # Devuelve
/// Su descripción en el formato que espera el protocolo MCP.
fn herramienta_informacion_del_espacio() -> Value {
    json!({
        // Va la primera a propósito: es la que responde a la pregunta con la que empieza
        // cualquier agente —«¿dónde puedo trabajar y qué hay ahí?»— y la que evita que
        // las demás se llamen con rutas inventadas.
        "name": "mmcelt_workspace_info",
        "description": "Devuelve la carpeta de trabajo autorizada y la lista de mapas mentales (.mmcelt) que contiene. Llámala antes que ninguna otra: las rutas que devuelve son las que hay que reenviar tal cual en «file_path». No modifica nada.",
        "inputSchema": {
            "type": "object",
            "properties": {},
            "required": []
        }
    })
}

/// La herramienta que crea un mapa nuevo y su Markdown acompañante.
///
/// # Devuelve
/// Su descripción en el formato que espera el protocolo MCP.
fn herramienta_crear_mapa() -> Value {
    json!({
        "name": "mmcelt_create_mindmap",
        "description": "Crea y guarda un nuevo mapa mental MMCelt (.mmcelt) y su archivo Markdown (.md) optimizado para IA.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "file_path": { "type": "string", "description": AYUDA_DEL_CAMPO_RUTA },
                "agent_name": { "type": "string", "description": "Nombre del agente que devolverá el mapa, por ejemplo Claude Code, ChatGPT Codex o Gemini Antigravity." },
                "title": { "type": "string", "description": "Título del proyecto" },
                "creator_vision": { "type": "string", "description": "Qué busca plasmar el usuario y la visión del proyecto" },
                "project_goals": { "type": "string", "description": "Objetivos y entregables deseados" },
                "target_audience": { "type": "string", "description": "Público objetivo o contexto de aplicación" },
                "mindmap_data": {
                    "type": "object",
                    "description": "El mapa. Lleva «root_node» con la jerarquía y, si procede, «cross_connections».",
                    "properties": {
                        "root_node": {
                            "type": "object",
                            "description": "Nodo raíz. Cada nodo admite: title, notes, file_path (ruta del archivo de código), role, status, priority, tags y children. Los campos «review_status» y «correction_feedback» se descartan aquí: un mapa que crea un agente nace entero como trabajo de la IA, y las correcciones las escribe la persona en el programa.",
                        },
                        "cross_connections": {
                            "type": "array",
                            "description": "Relaciones entre ramas distintas, expresadas por título: from_title, to_title, label y relation_type (Dependencia, InspiradoPor, Bloquea, AlternativaA o Sinergia).",
                        }
                    }
                }
            },
            "required": ["file_path", "title", "mindmap_data"]
        }
    })
}

/// La herramienta que devuelve el contenido de un mapa que ya existe.
///
/// # Devuelve
/// Su descripción en el formato que espera el protocolo MCP.
fn herramienta_leer_mapa() -> Value {
    json!({
        "name": "mmcelt_read_mindmap",
        "description": "Lee un mapa mental de MMCelt (.mmcelt o .md) y devuelve su jerarquía, notas, dudas abiertas y conexiones. Cada nodo trae su «id»: es el UUID con el que «mmcelt_sync_ai_progress» señala un nodo sin ambigüedad cuando su título se repite. Cada conexión trae los dos idiomas a la vez —«from» y «to» con los UUID, y «from_title» y «to_title» con los títulos—, así que no hace falta cruzar tablas a mano para escribir después.",
        "inputSchema": {
            "type": "object",
            "properties": { "file_path": { "type": "string", "description": AYUDA_DEL_CAMPO_RUTA } },
            "required": ["file_path"]
        }
    })
}

/// La herramienta con la que el agente se entera de lo que le ha corregido una persona.
///
/// Es la mitad del control humano: sin ella, el agente no tendría forma de saber qué nodos se
/// han marcado como equivocados ni qué camino se ha descartado.
///
/// # Devuelve
/// Su descripción en el formato que espera el protocolo MCP.
fn herramienta_leer_correcciones_humanas() -> Value {
    json!({
        "name": "mmcelt_get_human_feedback",
        "description": "Obtiene las correcciones, directivas y decisiones tomadas por el usuario humano en el mapa mental para ajustar el rumbo del trabajo.",
        "inputSchema": {
            "type": "object",
            "properties": { "file_path": { "type": "string", "description": AYUDA_DEL_CAMPO_RUTA } },
            "required": ["file_path"]
        }
    })
}

/// La herramienta con la que el agente informa de por dónde va.
///
/// Es la que más cuidado exige de las seis: escribe sobre el mapa del usuario, así que su
/// descripción tiene que dejar claro qué respeta y qué no.
///
/// # Devuelve
/// Su descripción en el formato que espera el protocolo MCP.
fn herramienta_sincronizar_progreso() -> Value {
    json!({
        "name": "mmcelt_sync_ai_progress",
        "description": "Desarrolla un mapa mental que ya existe: actualiza nodos y añade los tuyos, con su rol, sus etiquetas y sus conexiones cruzadas. Es la herramienta para enriquecer el mapa de una persona sin sustituirlo; «mmcelt_create_mindmap» es solo para mapas nuevos. Los nodos se señalan por su UUID o por su título indistintamente: usa el UUID que devuelve «mmcelt_read_mindmap» siempre que puedas, porque los títulos se repiten y un título repetido rechaza la llamada en vez de elegir por su cuenta. Lo que añadas nace marcado como generado por la IA, no se borra ni se mueve ningún nodo, y no se reabre ningún camino que el usuario haya descartado.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "file_path": { "type": "string", "description": AYUDA_DEL_CAMPO_RUTA },
                "agent_name": { "type": "string", "description": "Nombre del agente que devuelve la modificación." },
                "node_updates": {
                    "type": "array",
                    "description": "Nodos a actualizar. Cada entrada señala su nodo con «id», con «title» o con los dos; hay que indicar al menos uno. Si el título no existe en el mapa, el nodo se crea colgando de «parent_id» o de «parent_title», o de la raíz si no se indica ninguno. Un título que corresponde a varios nodos rechaza la llamada entera sin escribir nada, y la respuesta enumera los candidatos con su UUID: repite entonces la llamada con «id». Para añadir solo conexiones, manda una lista vacía.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string", "description": "UUID del nodo, tal como lo devuelve «mmcelt_read_mindmap». Es la forma inequívoca de señalarlo, y la única que sirve cuando hay títulos repetidos. Si no existe en el mapa, la llamada se rechaza: no se crea nada." },
                            "title": { "type": "string", "description": "Título del nodo. Basta por sí solo mientras sea único en el mapa. Si acompaña a «id», tiene que ser el título de ese nodo: si no coincide, la llamada se rechaza. Si no hay ningún nodo con ese título, se crea uno nuevo." },
                            "parent_id": { "type": "string", "description": "Solo si el nodo es nuevo: UUID del nodo del que colgará. Manda sobre «parent_title»." },
                            "parent_title": { "type": "string", "description": "Solo si el nodo es nuevo: título del nodo del que colgará. Si no existe ninguno con ese título, el nodo cuelga de la raíz y se avisa en «parents_not_found»; si lo llevan varios, la llamada se rechaza." },
                            "status": { "type": "string", "description": "Idea, Investigando, EnProgreso, DudaBloqueo, Completado o Descartado." },
                            "notes": { "type": "string" },
                            "priority": { "type": "string", "description": "Baja, Media, Alta o Critica." },
                            "role": { "type": "string", "description": "IdeaCentral, PilarEstrategico, Subtema, HipotesisDuda, AccionTarea o RecursoHerramienta. Un valor que no esté en la lista se rechaza con aviso: no se adivina." },
                            "tags": {
                                "type": "array",
                                "description": "Etiquetas temáticas. Se suman a las que ya tenga el nodo; nunca sustituyen a las que puso el usuario.",
                                "items": { "type": "string" }
                            },
                            "file_path": { "type": "string", "description": "Ruta del archivo de código al que corresponde el nodo, relativa a la raíz del proyecto." }
                        },
                        "anyOf": [
                            { "required": ["id"] },
                            { "required": ["title"] }
                        ]
                    }
                },
                "cross_connections": {
                    "type": "array",
                    "description": "Relaciones entre ramas distintas. Cada extremo se señala con su UUID —«from_id», «to_id»— o con su título —«from_title», «to_title»—, o con los dos, que entonces tienen que concordar. Se resuelven después de «node_updates», así que pueden apuntar a un nodo creado en esta misma llamada. Un extremo ambiguo, un UUID inexistente o un UUID que no case con su título rechazan la llamada entera sin escribir nada. Se descartan una a una, informando en «connections_not_created», las que nombren un título inexistente, unan un nodo consigo mismo, toquen un nodo descartado o repitan una relación que ya está.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "from_id": { "type": "string", "description": "UUID del nodo de origen, tal como lo devuelve «mmcelt_read_mindmap». Manda sobre «from_title»." },
                            "to_id": { "type": "string", "description": "UUID del nodo de destino. Manda sobre «to_title»." },
                            "from_title": { "type": "string", "description": "Título del nodo de origen. Basta por sí solo mientras sea único en el mapa." },
                            "to_title": { "type": "string", "description": "Título del nodo de destino. Basta por sí solo mientras sea único en el mapa." },
                            "label": { "type": "string", "description": "Por qué se relacionan." },
                            "relation_type": { "type": "string", "description": "Dependencia, InspiradoPor, Bloquea, AlternativaA o Sinergia." }
                        },
                        "allOf": [
                            { "anyOf": [
                                { "required": ["from_id"] },
                                { "required": ["from_title"] }
                            ] },
                            { "anyOf": [
                                { "required": ["to_id"] },
                                { "required": ["to_title"] }
                            ] }
                        ]
                    }
                }
            },
            "required": ["file_path", "node_updates"]
        }
    })
}

/// La herramienta que vuelve a generar el Markdown de un mapa para dárselo a una IA.
///
/// # Devuelve
/// Su descripción en el formato que espera el protocolo MCP.
fn herramienta_exportar_markdown() -> Value {
    json!({
        "name": "mmcelt_export_ai_markdown",
        "description": "Convierte un archivo de mapa mental .mmcelt a un documento Markdown enriquecido para IA.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "mmcelt_file_path": { "type": "string", "description": AYUDA_DEL_CAMPO_RUTA },
                "output_md_path": { "type": "string", "description": AYUDA_DEL_CAMPO_RUTA }
            },
            "required": ["mmcelt_file_path", "output_md_path"]
        }
    })
}
