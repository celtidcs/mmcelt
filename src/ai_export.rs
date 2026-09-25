//! # Módulo de Exportación a Markdown para IA (`ai_export.rs`)
//!
//! Genera un documento Markdown altamente optimizado con técnicas de **Meta-Prompting**
//! e **Ingeniería de Contexto** para que los modelos LLM (Claude, ChatGPT, Gemini, etc.)
//! comprendan a fondo la visión del creador, analicen dependencias, resuelvan incógnitas
//! y acaten directivas de corrección humana (*Human-in-the-loop*).
//!
//! ## Una sola implementación
//!
//! Este módulo es el **único** sitio donde se genera el formato. Lo usan tanto la
//! aplicación como el servidor MCP integrado ([`crate::mcp_server`]).
//!
//! No siempre fue así: hubo un servidor MCP escrito aparte que reimplementaba este mismo
//! documento en otro lenguaje, con la garantía de que antes o después divergirían. Al
//! integrar el servidor en el ejecutable, esa copia desapareció.
//!
//! Las secciones siguen una numeración natural del 1 al 9 cuando el mapa activa todos los
//! bloques opcionales. Las plantillas que remiten a una sección se generan con la misma
//! traducción que su encabezado para que el agente no reciba referencias contradictorias.

use crate::model::{EstadoNodo, Nodo, Proyecto, RolNodo, TipoRelacion};
use crate::textos::{Idioma, Texto};
use std::collections::HashSet;
use uuid::Uuid;

/// Ayudas de encargo que la persona puede copiar de una en una a una sesión.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlantillaEncargo {
    /// Solicita convertir la visión y los pilares en un plan de ejecución.
    DesarrolloEstrategico,
    /// Solicita aplicar exclusivamente las correcciones humanas registradas.
    AplicarCorrecciones,
    /// Solicita estudiar los puntos de decisión, dudas y riesgos del mapa.
    ResolverDudas,
}

impl PlantillaEncargo {
    /// Catálogo completo y estable que deben recorrer la interfaz y las pruebas.
    pub const TODAS: [Self; 3] = [
        Self::DesarrolloEstrategico,
        Self::AplicarCorrecciones,
        Self::ResolverDudas,
    ];

    /// Devuelve el título limpio en prosa para la interfaz de usuario en el idioma elegido.
    pub fn titulo_interfaz(self, idioma: Idioma) -> &'static str {
        match self {
            Self::DesarrolloEstrategico => Texto::PlantillaEncargoOpcionA.en(idioma),
            Self::AplicarCorrecciones => Texto::PlantillaEncargoOpcionB.en(idioma),
            Self::ResolverDudas => Texto::PlantillaEncargoOpcionC.en(idioma),
        }
    }

    /// Devuelve el encabezado Markdown (con almohadillas de nivel 3 y salto de línea)
    /// destinado exclusivamente al documento de instrucciones para el agente (`_AI.md`).
    pub fn encabezado_markdown(self, idioma: Idioma) -> &'static str {
        match self {
            Self::DesarrolloEstrategico => Texto::MdOpcionADesarrolloEstrategico.en(idioma),
            Self::AplicarCorrecciones => Texto::MdOpcionBAplicacionDe.en(idioma),
            Self::ResolverDudas => Texto::MdOpcionCResolucionDe.en(idioma),
        }
    }

    /// Redacta un único encargo usando el mapa y el idioma seleccionados por la persona.
    pub fn contenido(self, proyecto: &Proyecto, idioma: Idioma) -> String {
        self.contenido_para_titulo(&proyecto.title, idioma)
    }

    /// Redacta la ayuda cuando el llamador conserva únicamente el título del proyecto.
    pub fn contenido_para_titulo(self, titulo_proyecto: &str, idioma: Idioma) -> String {
        match self {
            Self::DesarrolloEstrategico => {
                let mut contenido =
                    rellenar(Texto::MdActuaComoUnConsultor.en(idioma), &[titulo_proyecto]);
                contenido.push_str(Texto::MdBasandoteEnMiVision.en(idioma));
                contenido
            }
            Self::AplicarCorrecciones => {
                rellenar(Texto::MdHeRevisadoElMapa2.en(idioma), &[titulo_proyecto])
            }
            Self::ResolverDudas => Texto::MdRevisaEspecificamenteLaSeccion
                .en(idioma)
                .to_string(),
        }
    }
}

/// Rellena por posición los huecos `{}` de una plantilla traducida.
///
/// # Por qué no se usa `format!`
///
/// Porque no se puede: `format!` exige que la plantilla sea un literal escrito en el fuente, y
/// aquí la plantilla llega en tiempo de ejecución, elegida según el idioma del usuario. Las 28
/// cadenas de este documento que llevan huecos se rellenan por tanto a mano, y **por posición**:
/// los huecos son todos anónimos y el orden es el que fija el castellano. Un `{}` movido de
/// sitio pone el estado donde va la prioridad y el documento sale mintiendo sin que nada falle,
/// así que el generador de las tablas comprueba que los huecos coinciden en cantidad y orden en
/// los seis idiomas.
///
/// # Por qué se parte la plantilla en vez de sustituir en bucle
///
/// Un `replacen("{}", valor, 1)` repetido vuelve a mirar el texto ya insertado. Los valores que
/// entran aquí son títulos y notas del mapa, que escriben la persona **y un agente de IA**: un
/// título que contenga `{}` haría que el siguiente valor se metiera dentro del anterior. Al
/// partir la plantilla y coser los trozos, lo insertado no se vuelve a examinar nunca.
///
/// # Parámetros
/// - `plantilla`: el texto traducido, con sus huecos `{}`.
/// - `valores`: lo que va en cada hueco, en el orden en que aparecen.
///
/// # Devuelve
/// La plantilla con los huecos rellenos. Si faltan valores, el hueco sobrante se deja visible
/// en lugar de cortar el texto: un documento con un `{}` a la vista se detecta de un vistazo,
/// uno truncado en silencio no.
fn rellenar(plantilla: &str, valores: &[&str]) -> String {
    let trozos: Vec<&str> = plantilla.split("{}").collect();
    let mut resultado = String::with_capacity(plantilla.len());
    for (posicion, trozo) in trozos.iter().enumerate() {
        resultado.push_str(trozo);
        if posicion + 1 < trozos.len() {
            match valores.get(posicion) {
                Some(valor) => resultado.push_str(valor),
                None => resultado.push_str("{}"),
            }
        }
    }
    resultado
}

/// Elige nodos del mapa y los devuelve **siempre en el mismo orden**.
///
/// Los nodos vivían en una tabla dispersa, y recorrerla daba un orden distinto en cada
/// ejecución. Eso no se nota al leer el documento, pero sí en el archivo: el servidor MCP
/// reescribe el `_AI.md` en cada informe de progreso, así que dos sincronizaciones seguidas
/// sobre un mapa que no ha cambiado producían archivos distintos, con los bloques barajados.
/// Quien lo tenga en un repositorio ve un diff enorme por nada.
///
/// El orden es por identificador: arbitrario, pero **estable**. Es el mismo criterio que ya
/// se aplicó al índice del servidor por este mismo motivo.
///
/// # Por qué se sigue ordenando
///
/// `Proyecto::nodes` es hoy un mapa ordenado, así que recorrerlo ya sale por identificador y
/// la ordenación **parece** sobrar. No sobra: ordena por `nodo.id`, que es un campo del
/// nodo, mientras que el recorrido sale por la **clave** del mapa. En un archivo bien
/// formado coinciden, pero nada del tipo lo obliga, y este documento es lo que lee el agente.
/// Donde el criterio de orden es la propia clave —el índice del servidor, la huella del
/// autoguardado— la ordenación sí se quitó.
///
/// # Parámetros
/// - `proyecto`: el mapa del que se eligen los nodos.
/// - `interesa`: qué nodos entran en la lista.
///
/// # Devuelve
/// Los nodos elegidos, ordenados por identificador.
fn en_orden_estable(proyecto: &Proyecto, interesa: impl Fn(&Nodo) -> bool) -> Vec<&Nodo> {
    let mut elegidos: Vec<&Nodo> = proyecto.nodes.values().filter(|n| interesa(n)).collect();
    elegidos.sort_by_key(|nodo| nodo.id);
    elegidos
}

/// Genera el documento Markdown semántico completo a partir del proyecto de mapa mental.
///
/// Incluye:
/// 1. Preámbulo y meta-instrucciones para el LLM.
/// 2. Visión del creador y objetivos del proyecto en el mundo real.
/// 3. Resumen ejecutivo y métricas de madurez.
/// 4. Estructura jerárquica con notas, rutas de código y tags.
/// 5. Directivas de corrección y validación del usuario (*Human-in-the-loop*).
/// 6. Matriz de dependencias y relaciones cruzadas.
/// 7. Sección prioritaria de puntos de decisión y dudas a resolver.
/// 8. Diagrama visual Mermaid renderizable.
/// 9. Prompts de acción sugeridos.
/// 10. Contrato MCP para devolver el trabajo sin editar el mapa directamente.
///
/// # Parámetros
/// - `proyecto`: el mapa completo que se va a describir. Se lee entero, sin modificarlo.
/// - `idioma`: la lengua en que se escribe el documento, que es la que el usuario tenga
///   elegida en la interfaz. Quien pone la interfaz en chino habla con su modelo en chino,
///   y entregarle este documento en castellano no le sirve de nada.
///
/// # Devuelve
/// El documento Markdown listo para pegar en la conversación con el modelo.
pub fn exportar_markdown_para_ia(proyecto: &Proyecto, idioma: Idioma) -> String {
    let mut md = exportar_contexto_mapa_para_agente(proyecto, idioma);
    escribir_prompts_sugeridos(&mut md, proyecto, idioma);
    md.push_str(&contrato_mmcelt_para_agente(idioma));
    sin_caracteres_de_control(&md)
}

/// Genera solo el contexto del mapa, sin repetir el contrato MMCelt/MCP de la sesión.
///
/// Comparte todos los escritores con [`exportar_markdown_para_ia`]. La separación existe para que
/// el modal pueda presentar el contrato como bloque protegido y el mapa como bloque independiente,
/// sin enviar dos veces las mismas instrucciones de devolución.
pub(crate) fn exportar_contexto_mapa_para_agente(proyecto: &Proyecto, idioma: Idioma) -> String {
    let mut md = String::new();

    escribir_preambulo(&mut md, proyecto, idioma);
    escribir_vision_del_creador(&mut md, proyecto, idioma);
    escribir_resumen_ejecutivo(&mut md, proyecto, idioma);
    escribir_estructura_jerarquica(&mut md, proyecto, idioma);
    escribir_correcciones_humanas(&mut md, proyecto, idioma);
    escribir_matriz_de_conexiones(&mut md, proyecto, idioma);
    escribir_dudas_y_decisiones(&mut md, proyecto, idioma);
    escribir_diagrama_mermaid(&mut md, proyecto, idioma);

    sin_caracteres_de_control(&md)
}

/// Genera el contrato que explica a cualquier agente cómo devolver su trabajo mediante MMCelt.
///
/// La exportación del mapa y el prompt inicial llaman a esta misma función para que los nombres
/// de herramientas, las reglas de supervisión y los valores canónicos no puedan divergir.
pub(crate) fn contrato_mmcelt_para_agente(idioma: Idioma) -> String {
    let mut contrato = String::new();
    escribir_como_devolver_el_trabajo(&mut contrato, idioma);
    contrato
}

/// Explica al agente cómo devolver cambios al mapa mediante el servidor MCP.
///
/// Esta sección cierra el recorrido de ida y vuelta: el documento deja de ser una descripción
/// sin puerta de regreso. Los nombres de herramientas y los valores canónicos no se traducen,
/// porque son parte del contrato JSON que el agente debe enviar literalmente.
fn escribir_como_devolver_el_trabajo(md: &mut String, idioma: Idioma) {
    let introduccion = match idioma {
        Idioma::Espanol => "## 🔄 9. CÓMO DEVOLVER TU TRABAJO A ESTE MAPA\n\nMMCelt expone un servidor MCP. No edites el archivo `.mmcelt` directamente: usa sus herramientas para conservar la supervisión humana y las copias de seguridad.\n\n",
        Idioma::Ingles => "## 🔄 9. HOW TO RETURN YOUR WORK TO THIS MAP\n\nMMCelt exposes an MCP server. Do not edit the `.mmcelt` file directly: use its tools to preserve human supervision and backups.\n\n",
        Idioma::Frances => "## 🔄 9. COMMENT RENVOYER VOTRE TRAVAIL À CETTE CARTE\n\nMMCelt expose un serveur MCP. Ne modifiez pas directement le fichier `.mmcelt` : utilisez ses outils afin de préserver la supervision humaine et les sauvegardes.\n\n",
        Idioma::Aleman => "## 🔄 9. SO GIBST DU DEINE ARBEIT AN DIESE KARTE ZURÜCK\n\nMMCelt stellt einen MCP-Server bereit. Bearbeite die `.mmcelt`-Datei nicht direkt: Verwende seine Werkzeuge, damit menschliche Kontrolle und Sicherungen erhalten bleiben.\n\n",
        Idioma::Ruso => "## 🔄 9. КАК ВЕРНУТЬ РАБОТУ В ЭТУ КАРТУ\n\nMMCelt предоставляет сервер MCP. Не редактируйте файл `.mmcelt` напрямую: используйте его инструменты, чтобы сохранить контроль человека и резервные копии.\n\n",
        Idioma::ChinoSimplificado => "## 🔄 9. 如何将工作返回此思维导图\n\nMMCelt 提供 MCP 服务器。请勿直接编辑 `.mmcelt` 文件；请使用其工具，以保留人工监督和备份。\n\n",
    };
    md.push_str(introduccion);
    let flujo = match idioma {
        Idioma::Espanol => "1. `mmcelt_workspace_info`: descubre la carpeta autorizada y los mapas disponibles.\n2. `mmcelt_read_mindmap`: lee nodos, UUID, títulos y conexiones.\n3. `mmcelt_get_human_feedback`: lee correcciones y decisiones humanas.\n4. `mmcelt_sync_ai_progress`: devuelve avances al mapa existente sin sustituirlo.\n5. `mmcelt_create_mindmap`: crea un mapa nuevo; no sustituye un mapa humano.\n6. `mmcelt_export_ai_markdown`: regenera este documento cuando sea necesario.\n\nIdentidad: usa el UUID siempre que haya títulos repetidos; si envías UUID y título, ambos deben concordar. Nunca declares una aprobación humana ni cambies `review_status`: lo añadido por la IA nace como `GeneradoPorIA`.\n\n",
        Idioma::Ingles => "1. `mmcelt_workspace_info`: discover the authorized folder and available maps.\n2. `mmcelt_read_mindmap`: read nodes, UUIDs, titles and connections.\n3. `mmcelt_get_human_feedback`: read human corrections and decisions.\n4. `mmcelt_sync_ai_progress`: return progress to the existing map without replacing it.\n5. `mmcelt_create_mindmap`: create a new map; never replace a human map.\n6. `mmcelt_export_ai_markdown`: regenerate this document when needed.\n\nIdentity: use the UUID whenever titles repeat; if UUID and title are both sent, they must match. Never claim human approval or change `review_status`: AI additions are born as `GeneradoPorIA`.\n\n",
        Idioma::Frances => "1. `mmcelt_workspace_info` : découvrir le dossier autorisé et les cartes disponibles.\n2. `mmcelt_read_mindmap` : lire les nœuds, UUID, titres et connexions.\n3. `mmcelt_get_human_feedback` : lire les corrections et décisions humaines.\n4. `mmcelt_sync_ai_progress` : renvoyer les progrès sans remplacer la carte.\n5. `mmcelt_create_mindmap` : créer une nouvelle carte, jamais remplacer une carte humaine.\n6. `mmcelt_export_ai_markdown` : régénérer ce document.\n\nIdentité : utilisez l’UUID lorsque les titres se répètent ; UUID et titre doivent correspondre. Ne revendiquez jamais une approbation humaine et ne modifiez pas `review_status` : les ajouts de l’IA naissent `GeneradoPorIA`.\n\n",
        Idioma::Aleman => "1. `mmcelt_workspace_info`: autorisierten Ordner und Karten ermitteln.\n2. `mmcelt_read_mindmap`: Knoten, UUIDs, Titel und Verbindungen lesen.\n3. `mmcelt_get_human_feedback`: menschliche Korrekturen und Entscheidungen lesen.\n4. `mmcelt_sync_ai_progress`: Fortschritt zurückgeben, ohne die Karte zu ersetzen.\n5. `mmcelt_create_mindmap`: eine neue Karte erstellen, keine menschliche Karte ersetzen.\n6. `mmcelt_export_ai_markdown`: dieses Dokument neu erzeugen.\n\nIdentität: Bei doppelten Titeln die UUID verwenden; UUID und Titel müssen übereinstimmen. Niemals menschliche Freigabe beanspruchen oder `review_status` ändern: KI-Ergänzungen entstehen als `GeneradoPorIA`.\n\n",
        Idioma::Ruso => "1. `mmcelt_workspace_info`: узнать разрешённую папку и доступные карты.\n2. `mmcelt_read_mindmap`: прочитать узлы, UUID, заголовки и связи.\n3. `mmcelt_get_human_feedback`: прочитать исправления и решения человека.\n4. `mmcelt_sync_ai_progress`: вернуть результат, не заменяя карту.\n5. `mmcelt_create_mindmap`: создать новую карту, не заменять карту человека.\n6. `mmcelt_export_ai_markdown`: заново создать этот документ.\n\nИдентификация: при повторяющихся заголовках используйте UUID; UUID и заголовок должны совпадать. Не присваивайте одобрение человека и не меняйте `review_status`: добавления ИИ создаются как `GeneradoPorIA`.\n\n",
        Idioma::ChinoSimplificado => "1. `mmcelt_workspace_info`：查找授权文件夹和可用导图。\n2. `mmcelt_read_mindmap`：读取节点、UUID、标题和连接。\n3. `mmcelt_get_human_feedback`：读取人工修正和决定。\n4. `mmcelt_sync_ai_progress`：将进度返回现有导图，不替换导图。\n5. `mmcelt_create_mindmap`：创建新导图，不替换人工导图。\n6. `mmcelt_export_ai_markdown`：在需要时重新生成本文档。\n\n标识：标题重复时使用 UUID；同时发送 UUID 和标题时两者必须一致。不得声称人工批准或更改 `review_status`；AI 新增内容标记为 `GeneradoPorIA`。\n\n",
    };
    md.push_str(flujo);
    md.push_str(
        "Valores canónicos que deben enviarse literalmente / Canonical values to send literally:\n\
- `status`: Idea, Investigando, EnProgreso, DudaBloqueo, Completado, Descartado\n\
- `role`: IdeaCentral, PilarEstrategico, Subtema, HipotesisDuda, AccionTarea, RecursoHerramienta\n\
- `priority`: Baja, Media, Alta, Critica\n\
- `relation_type`: Dependencia, InspiradoPor, Bloquea, AlternativaA, Sinergia\n",
    );
}

/// Quita del documento los caracteres de control, menos los saltos de línea y tabuladores.
///
/// El documento lo compone el mapa, y en el mapa escribe también un agente de IA: los títulos
/// y las notas llegan tal como se recibieron. Un título con `\u{1b}[2J`, `\u{7}` o un byte nulo
/// se aceptaba y se copiaba crudo aquí dentro, y este documento está hecho para que **otro
/// agente lo lea**. Si lo vuelca por una terminal, esas secuencias no se leen: se ejecutan —
/// borran la pantalla, mueven el cursor, hacen sonar el aparato—.
///
/// Se aplica en un solo sitio, a la salida, en vez de en cada punto donde se escribe un texto
/// del mapa: son muchos, y bastaría olvidar uno. Es la misma lección de las vallas que se
/// rodean por la puerta de al lado.
///
/// # Parámetros
/// - `documento`: el Markdown ya compuesto.
///
/// # Devuelve
/// El mismo documento sin caracteres de control. Los saltos de línea y los tabuladores se
/// conservan: son la estructura del Markdown y del contenido que escribe el usuario.
fn sin_caracteres_de_control(documento: &str) -> String {
    documento
        .chars()
        .filter(|caracter| !caracter.is_control() || *caracter == '\n' || *caracter == '\t')
        .collect()
}

/// Genera un prompt centrado exclusivamente en las correcciones exigidas por el usuario.
///
/// A diferencia de [`exportar_markdown_para_ia`], que describe el mapa entero, este
/// documento contiene solo las directivas de rumbo: los nodos marcados como
/// `RequiereCorreccion`, los que llevan comentarios de correccion y los descartados.
/// Esta pensado para reconducir a una IA que ya conoce el proyecto.
///
/// # Parámetros
/// - `proyecto`: proyecto del que se extraen las correcciones.
/// - `idioma`: la lengua en que se escribe el prompt, la que el usuario tenga elegida.
///
/// # Devuelve
/// El texto del prompt, listo para copiar al portapapeles.
pub fn exportar_correcciones_del_usuario(proyecto: &Proyecto, idioma: Idioma) -> String {
    let mut prompt = String::new();
    prompt.push_str(Texto::MdInstruccionDeCorreccionY.en(idioma));
    prompt.push_str(&rellenar(
        Texto::MdHeRevisadoElMapa.en(idioma),
        &[&proyecto.title],
    ));

    let correcciones = en_orden_estable(proyecto, |n| n.exige_correccion_del_usuario());

    if correcciones.is_empty() {
        prompt.push_str(Texto::MdTodoElMapaMental.en(idioma));
    } else {
        prompt.push_str(Texto::MdPorFavorAcataEstrictamente.en(idioma));
        for corr in correcciones {
            prompt.push_str(&rellenar(Texto::MdSobreN.en(idioma), &[&corr.title]));
            if !corr.correction_feedback.trim().is_empty() {
                prompt.push_str(&rellenar(
                    Texto::MdMiCorreccionN.en(idioma),
                    &[corr.correction_feedback.trim()],
                ));
            }
            if corr.status == EstadoNodo::Descartado {
                prompt.push_str(Texto::MdEstaOpcionEstaTotalmente.en(idioma));
            }
        }
    }

    prompt.push_str(Texto::MdNadjuntoElMapaMental.en(idioma));
    prompt.push_str(&exportar_markdown_para_ia(proyecto, idioma));
    prompt
}

/// Vuelca un nodo y toda su descendencia en el documento Markdown.
///
/// Recorre el árbol en profundidad con una pila explícita en el montículo para evitar
/// desbordar la pila de llamadas del hilo en mapas con ramas profundas.
///
/// Lleva un conjunto de nodos visitados para que un mapa con un ciclo no provoque un bucle
/// infinito. La profundidad determina el nivel de encabezado o de sangría con que se
/// presenta cada nodo.
///
/// # Parámetros
/// - `md`: documento Markdown en construcción donde se añade la estructura.
/// - `proyecto`: proyecto del que se obtienen los datos.
/// - `raiz_id`: identificador de la raíz desde donde comenzar el recorrido.
/// - `idioma`: idioma en el que formatear estados y prioridades.
fn dibujar_estructura_jerarquica_iterativa(
    md: &mut String,
    proyecto: &Proyecto,
    raiz_id: Uuid,
    idioma: Idioma,
) {
    let mut visitados: HashSet<Uuid> = HashSet::with_capacity(proyecto.nodes.len());
    let mut pila: Vec<(Uuid, usize)> = vec![(raiz_id, 0)];

    while let Some((id_del_nodo, profundidad)) = pila.pop() {
        if !visitados.insert(id_del_nodo) {
            continue;
        }

        let Some(node) = proyecto.nodes.get(&id_del_nodo) else {
            continue;
        };

        let sangria = "  ".repeat(profundidad);

        let etiquetas_en_texto = if node.tags.is_empty() {
            String::new()
        } else {
            let tags = node
                .tags
                .iter()
                .map(|t| format!("`#{}`", t))
                .collect::<Vec<_>>()
                .join(" ");
            format!(" [{}]", tags)
        };

        let ruta_en_texto = if let Some(ref path) = node.file_path {
            format!(" `[📁 {}]`", path)
        } else {
            String::new()
        };

        // El orden de los huecos lo fija el castellano y es el mismo en los seis idiomas: se
        // rellenan por posición, así que mover uno aquí pondría el estado donde va la prioridad.
        if profundidad == 0 {
            md.push_str(&rellenar(
                Texto::MdNodoCentralNN.en(idioma),
                &[&node.title, &ruta_en_texto],
            ));
        } else if profundidad == 1 {
            md.push_str(&rellenar(
                Texto::MdPilarPrioridadNN.en(idioma),
                &[
                    &node.title,
                    node.status.emoji(),
                    node.status.nombre_para_interfaz(idioma),
                    node.priority.nombre_para_interfaz(idioma),
                    &ruta_en_texto,
                    &etiquetas_en_texto,
                ],
            ));
        } else {
            md.push_str(&rellenar(
                Texto::MdElementoPrioridadN.en(idioma),
                &[
                    &sangria,
                    &node.title,
                    node.status.emoji(),
                    node.status.nombre_para_interfaz(idioma),
                    node.priority.nombre_para_interfaz(idioma),
                    &ruta_en_texto,
                    &etiquetas_en_texto,
                ],
            ));
        }

        if !node.notes.trim().is_empty() {
            let notas_con_sangria = node.notes.trim().replace('\n', &format!("\n{}> ", sangria));
            md.push_str(&rellenar(
                Texto::MdNotasNN.en(idioma),
                &[&sangria, &notas_con_sangria],
            ));
        } else if profundidad <= 1 {
            md.push('\n');
        }

        // Se apilan los hijos en orden inverso para que el primero declarado sea el primero
        // en extraerse de la pila LIFO, conservando el orden de recorrido original.
        for &id_del_hijo in node.children.iter().rev() {
            pila.push((id_del_hijo, profundidad + 1));
        }
    }
}

/// Convierte un UUID en un identificador válido para Mermaid.
///
/// Mermaid no admite guiones en los identificadores de nodo, asi que se eliminan y se
/// antepone un prefijo alfabético para que el identificador no empiece por un dígito.
///
/// # Parámetros
/// - `id`: identificador del nodo.
///
/// # Devuelve
/// El identificador saneado.
fn sanear_identificador_mermaid(id: &Uuid) -> String {
    format!("N_{}", id.simple())
}

/// Escapa el texto de una etiqueta para que no rompa la sintaxis de Mermaid.
///
/// Sustituye las comillas dobles, que delimitan las etiquetas; colapsa los saltos de línea,
/// que cortarían la declaración del nodo; cambia los corchetes, que delimitan la forma de la
/// caja; y sustituye las comillas invertidas.
///
/// Estas últimas son las menos evidentes y las que más daño hacen. El diagrama se escribe
/// dentro de un bloque de código que abre y cierra con tres comillas invertidas. Un título
/// que lleve alguna —algo normal en un mapa de código, y más si lo llenó el escáner de
/// repositorios con nombres de archivo— cerraba ese bloque antes de tiempo: el resto del
/// diagrama se derramaba como texto suelto y el documento que recibe el modelo quedaba mal
/// formado de ahí en adelante.
///
/// # Parámetros
/// - `text`: texto original de la etiqueta.
///
/// # Devuelve
/// El texto seguro para insertar en el diagrama.
fn sanear_texto_mermaid(text: &str) -> String {
    text.replace('"', "'")
        .replace('\n', " ")
        .replace('[', "(")
        .replace(']', ")")
        .replace('`', "'")
}

/// Añade al documento el diagrama Mermaid que representa el grafo completo.
///
/// Dibuja tanto la jerarquía padre-hijo como las conexiones cruzadas, con un estilo
/// distinto para la raíz, los pilares y el resto de nodos.
///
/// # Parámetros
/// - `md`: documento en construcción, al que se anexa el diagrama.
/// - `proyecto`: proyecto que se representa.
fn dibujar_diagrama_mermaid(md: &mut String, proyecto: &Proyecto) {
    // Declaración de los nodos.
    for (id, node) in &proyecto.nodes {
        let node_id_str = sanear_identificador_mermaid(id);
        let safe_title = sanear_texto_mermaid(&node.title);
        let emoji = node.status.emoji();
        let display = format!("{} {}", emoji, safe_title);

        if id == &proyecto.root_id {
            md.push_str(&format!("  {}[(\"{}\")]:::root\n", node_id_str, display));
        } else if node.parent_id == Some(proyecto.root_id) {
            md.push_str(&format!("  {}[\"{}\"]:::pillar\n", node_id_str, display));
        } else {
            md.push_str(&format!("  {}(\"{}\"):::sub\n", node_id_str, display));
        }
    }

    // Aristas de la jerarquía.
    for (id, node) in &proyecto.nodes {
        let parent_str = sanear_identificador_mermaid(id);
        for id_del_hijo in &node.children {
            let hijo_en_texto = sanear_identificador_mermaid(id_del_hijo);
            md.push_str(&format!("  {} --> {}\n", parent_str, hijo_en_texto));
        }
    }

    // Aristas de las conexiones cruzadas.
    for conn in &proyecto.connections {
        let from_str = sanear_identificador_mermaid(&conn.from);
        let to_str = sanear_identificador_mermaid(&conn.to);
        let label = sanear_texto_mermaid(&conn.label);
        let label_part = if label.is_empty() {
            String::new()
        } else {
            format!("|\"{}\"|", label)
        };

        match conn.relation_type {
            TipoRelacion::Dependencia => {
                md.push_str(&format!("  {} ==>{}=> {}\n", from_str, label_part, to_str))
            }
            TipoRelacion::InspiradoPor => {
                md.push_str(&format!("  {} -.->{} {}\n", from_str, label_part, to_str))
            }
            TipoRelacion::Bloquea => {
                md.push_str(&format!("  {} x--x{} {}\n", from_str, label_part, to_str))
            }
            TipoRelacion::AlternativaA => {
                md.push_str(&format!("  {} <--{}--> {}\n", from_str, label_part, to_str))
            }
            TipoRelacion::Sinergia => {
                md.push_str(&format!("  {} ==={} {}\n", from_str, label_part, to_str))
            }
        }
    }

    // Estilos de clase de Mermaid.
    md.push_str("  classDef root fill:#2563eb,stroke:#1d4ed8,stroke-width:3px,color:#fff;\n");
    md.push_str("  classDef pillar fill:#059669,stroke:#047857,stroke-width:2px,color:#fff;\n");
    md.push_str("  classDef sub fill:#334155,stroke:#475569,stroke-width:1px,color:#f8fafc;\n");
}

/// Escribe la sección «Preámbulo» del documento.
///
/// El comentario HTML de cabecera y el título. No se ve al leer el documento renderizado,
/// pero el modelo sí lo lee, y ahí se le dice qué tiene entre manos.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_preambulo(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    md.push_str(
        "<!-- ==========================================================================\n",
    );
    md.push_str(Texto::MdDocumentoDeMapaMental.en(idioma));
    md.push_str(Texto::MdGeneradoPorMmceltMindmap.en(idioma));
    md.push_str(Texto::MdObjetivoServirComoEspecificacion.en(idioma));
    md.push_str(Texto::MdComprendaAFondoLa.en(idioma));
    md.push_str(
        "========================================================================== -->\n\n",
    );

    md.push_str(&rellenar(
        Texto::MdMapaMentalDelProyecto.en(idioma),
        &[proyecto.title.trim()],
    ));
}

/// Escribe la sección «Visión del creador» del documento.
///
/// Lo que la persona buscaba al montar el mapa. Va antes que los datos a propósito: sin
/// esto, el modelo interpreta la estructura sin saber para qué existe.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_vision_del_creador(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    md.push_str(Texto::MdVisionDelCreadorY.en(idioma));
    md.push_str(Texto::MdInstruccionParaLaIa.en(idioma));

    md.push_str(Texto::MdLoQueElCreador.en(idioma));
    if !proyecto.creator_vision.trim().is_empty() {
        md.push_str(&format!("{}\n\n", proyecto.creator_vision.trim()));
    } else {
        md.push_str(Texto::MdNoEspecificadoExplicitamenteEn.en(idioma));
    }

    if !proyecto.project_goals.trim().is_empty() {
        md.push_str(Texto::MdObjetivosYResultadosDeseados.en(idioma));
        md.push_str(&format!("{}\n\n", proyecto.project_goals.trim()));
    }

    if !proyecto.target_audience_or_context.trim().is_empty() {
        md.push_str(Texto::MdContextoDeAplicacionPublico.en(idioma));
        md.push_str(&format!(
            "{}\n\n",
            proyecto.target_audience_or_context.trim()
        ));
    }
}

/// Escribe la sección «Resumen ejecutivo» del documento.
///
/// Recuento de nodos, pilares y reparto de estados. Da al modelo el tamaño y la madurez
/// del proyecto antes de entrar en el detalle.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_resumen_ejecutivo(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    md.push_str(Texto::MdResumenEjecutivoYMetricas.en(idioma));
    let total_nodes = proyecto.nodes.len();
    // La raíz puede faltar en un proyecto dañado; en ese caso se exporta un documento
    // degradado en lugar de interrumpir la exportación.
    let root = proyecto.nodo_raiz();
    let root_title = root
        .map(|r| r.title.as_str())
        .unwrap_or_else(|| Texto::MdSinNodoRaiz.en(idioma));
    let strategic_pillars_count = root.map(|r| r.children.len()).unwrap_or(0);
    let total_connections = proyecto.connections.len();
    let todas_las_etiquetas_del_mapa = proyecto.todas_las_etiquetas();

    let ideas = proyecto.nodos_con_estado(EstadoNodo::Idea).len();
    let investigando = proyecto.nodos_con_estado(EstadoNodo::Investigando).len();
    let en_progreso = proyecto.nodos_con_estado(EstadoNodo::EnProgreso).len();
    let con_dudas = proyecto.nodos_con_estado(EstadoNodo::DudaBloqueo).len();
    let completados = proyecto.nodos_con_estado(EstadoNodo::Completado).len();

    md.push_str(&rellenar(Texto::MdIdeaNodoRaizN.en(idioma), &[root_title]));
    md.push_str(&rellenar(
        Texto::MdTotalDeNodosRamas.en(idioma),
        &[&total_nodes.to_string()],
    ));
    md.push_str(&rellenar(
        Texto::MdPilaresEstrategicosPrincipalesN.en(idioma),
        &[&strategic_pillars_count.to_string()],
    ));
    md.push_str(&rellenar(
        Texto::MdRelacionesCruzadasTransversalesN.en(idioma),
        &[&total_connections.to_string()],
    ));
    // El patrón de fecha no se traduce: si se traduce, deja de formatear.
    md.push_str(&rellenar(
        Texto::MdUltimaActualizacionN.en(idioma),
        &[&proyecto
            .updated_at
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()],
    ));

    if !todas_las_etiquetas_del_mapa.is_empty() {
        let tag_list = todas_las_etiquetas_del_mapa
            .iter()
            .map(|t| format!("`#{}`", t))
            .collect::<Vec<_>>()
            .join(" ");
        md.push_str(&rellenar(
            Texto::MdEtiquetasGlobalesN.en(idioma),
            &[&tag_list],
        ));
    }

    md.push_str(Texto::MdNDesglosePorEstado.en(idioma));
    md.push_str(&rellenar(
        Texto::MdIdeasPorMadurarN.en(idioma),
        &[&ideas.to_string()],
    ));
    md.push_str(&rellenar(
        Texto::MdEnInvestigacionN.en(idioma),
        &[&investigando.to_string()],
    ));
    md.push_str(&rellenar(
        Texto::MdEnProgresoDesarrolloN.en(idioma),
        &[&en_progreso.to_string()],
    ));
    md.push_str(&rellenar(
        Texto::MdDudasOBloqueosN.en(idioma),
        &[&con_dudas.to_string()],
    ));
    md.push_str(&rellenar(
        Texto::MdCompletadosValidadosNN.en(idioma),
        &[&completados.to_string()],
    ));
}

/// Escribe la sección «Estructura jerárquica» del documento.
///
/// El mapa entero, con sus notas, rutas de código y etiquetas.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_estructura_jerarquica(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    md.push_str(Texto::MdEstructuraJerarquicaDetalladaN.en(idioma));
    md.push_str(Texto::MdLaSiguienteJerarquiaRefleja.en(idioma));

    dibujar_estructura_jerarquica_iterativa(md, proyecto, proyecto.root_id, idioma);
}

/// Escribe la sección «Correcciones humanas» del documento.
///
/// Lo que la persona ha vetado o mandado corregir. Es la sección que convierte el documento
/// en una orden y no en una sugerencia.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_correcciones_humanas(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    // Correcciones exigidas por la persona, que el modelo debe acatar.
    let correcciones = en_orden_estable(proyecto, |n| n.exige_correccion_del_usuario());

    // Aprobados **sin reparos**: la corrección manda sobre la aprobación.
    //
    // Los dos filtros eran independientes, y las dos condiciones son ortogonales: un nodo
    // aprobado que conserve escrita una corrección —el final normal de «pedí un cambio, la IA
    // lo hizo, lo aprobé»— salía en las dos listas del mismo documento. El modelo leía
    // «⛔ DESCARTADO por el usuario. No insistas» y «Validado y autorizado para proceder»
    // sobre el mismo nodo, sin forma de saber a cuál hacer caso.
    //
    // La herramienta que responde a los agentes por MCP ya clasificaba de forma excluyente;
    // esta no. Es la asimetría de siempre entre los dos caminos, que es justo lo que este
    // programa existe para evitar.
    let aprobados = en_orden_estable(proyecto, |n| {
        n.review_status == crate::model::EstadoRevision::AprobadoPorHumano
            && !n.exige_correccion_del_usuario()
    });

    if !correcciones.is_empty() || !aprobados.is_empty() {
        // El encabezado de esta sección y la cita que le hace el prompt sugerido de la opción
        // B tienen que decir lo mismo en cada idioma: el prompt manda al modelo a «la sección
        // 4 titulada X», y si la sección se titula Y, un modelo que la busque por el título
        // no la encuentra. Lo vigila la prueba `la_seccion_4_se_llama_igual_...`.
        md.push_str(Texto::MdNControlYDirectivas.en(idioma));
        md.push_str(Texto::MdObligatorioParaLaIa.en(idioma));

        if !correcciones.is_empty() {
            md.push_str(Texto::MdCorreccionesYCambiosExigidos.en(idioma));
            for corr in correcciones {
                md.push_str(&rellenar(Texto::MdNodoAfectadoN.en(idioma), &[&corr.title]));
                md.push_str(&rellenar(
                    Texto::MdEstadoDeRevisionN.en(idioma),
                    &[corr.review_status.nombre_para_interfaz(idioma)],
                ));
                if !corr.correction_feedback.trim().is_empty() {
                    md.push_str(&rellenar(
                        Texto::MdInstruccionDeCorreccionDel.en(idioma),
                        &[&corr.correction_feedback.replace('\n', "\n    > ")],
                    ));
                }
                if corr.status == EstadoNodo::Descartado {
                    md.push_str(Texto::MdAtencionEsteEnfoqueHa.en(idioma));
                }
            }
            md.push('\n');
        }

        if !aprobados.is_empty() {
            md.push_str(Texto::MdRamasYDecisionesAprobadas.en(idioma));
            for app_node in aprobados {
                md.push_str(&rellenar(
                    Texto::MdValidadoYAutorizadoPara.en(idioma),
                    &[&app_node.title],
                ));
            }
            md.push('\n');
        }
    }
}

/// Escribe la sección «Matriz de conexiones» del documento.
///
/// Las relaciones entre ramas distintas, que la jerarquía no puede expresar.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_matriz_de_conexiones(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    if !proyecto.connections.is_empty() {
        md.push_str(Texto::MdNMatrizDeRelaciones.en(idioma));
        md.push_str(Texto::MdNodoOrigenRelacionNodo.en(idioma));
        // La fila de alineación es sintaxis de tabla Markdown, no texto: no se traduce.
        md.push_str("| :--- | :---: | :--- | :--- |\n");

        for conn in &proyecto.connections {
            let from_title = proyecto
                .nodes
                .get(&conn.from)
                .map(|n| n.title.as_str())
                .unwrap_or_else(|| Texto::MdDesconocido.en(idioma));
            let to_title = proyecto
                .nodes
                .get(&conn.to)
                .map(|n| n.title.as_str())
                .unwrap_or_else(|| Texto::MdDesconocido.en(idioma));
            let rel_str = match conn.relation_type {
                TipoRelacion::Dependencia => Texto::MdDependeDe.en(idioma),
                TipoRelacion::InspiradoPor => Texto::MdInspiradoPor.en(idioma),
                TipoRelacion::Bloquea => Texto::MdBloqueaA.en(idioma),
                TipoRelacion::AlternativaA => Texto::MdAlternativaA.en(idioma),
                TipoRelacion::Sinergia => Texto::MdSinergiaCon.en(idioma),
            };
            let label_str = if conn.label.trim().is_empty() {
                "—"
            } else {
                conn.label.trim()
            };
            md.push_str(&format!(
                "| `{}` | {} | `{}` | {} |\n",
                from_title, rel_str, to_title, label_str
            ));
        }
        md.push('\n');
    }
}

/// Escribe la sección «Dudas y decisiones» del documento.
///
/// Los puntos abiertos, agrupados y aparte, para que el modelo sepa dónde se le pide que
/// aporte algo en lugar de limitarse a ejecutar.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el documento.
fn escribir_dudas_y_decisiones(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    let dudas = en_orden_estable(proyecto, |n| {
        n.status == EstadoNodo::DudaBloqueo
            || n.role == RolNodo::HipotesisDuda
            || n.tags.iter().any(|t| {
                t.to_lowercase().contains("duda")
                    || t.to_lowercase().contains("bloqueo")
                    || t.to_lowercase().contains("todo")
            })
    });

    if !dudas.is_empty() {
        md.push_str(Texto::MdPuntosDeDecisionDudas.en(idioma));
        md.push_str(Texto::MdAreaDeAtencionPrioritaria.en(idioma));

        for doubt in dudas {
            // Esta línea queda fuera del módulo de textos a propósito: no tiene ni una
            // palabra que traducir, solo sintaxis Markdown y dos huecos.
            md.push_str(&format!(
                "### 📌 `{}` ({})\n",
                doubt.title,
                doubt.status.nombre_para_interfaz(idioma)
            ));
            md.push_str(&rellenar(
                Texto::MdPrioridadN.en(idioma),
                &[doubt.priority.nombre_para_interfaz(idioma)],
            ));
            if let Some(parent_id) = doubt.parent_id {
                if let Some(parent) = proyecto.nodes.get(&parent_id) {
                    md.push_str(&rellenar(
                        Texto::MdPertenecienteAlPilarN.en(idioma),
                        &[&parent.title],
                    ));
                }
            }
            if !doubt.notes.trim().is_empty() {
                md.push_str(&rellenar(
                    Texto::MdDetallesPreguntaFormuladaN.en(idioma),
                    &[&doubt.notes.replace('\n', "\n  > ")],
                ));
            } else {
                md.push_str(Texto::MdDetallesSinNotasAdicionales.en(idioma));
            }
            md.push('\n');
        }
    }
}

/// Escribe la sección «Diagrama Mermaid» del documento.
///
/// El grafo, que se ve renderizado en GitHub y en la mayoría de interfaces de chat.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escribe el encabezado de la sección. El diagrama en sí no
///   lleva idioma: `graph LR` y los estilos son sintaxis de Mermaid, y traducirlos lo rompe.
fn escribir_diagrama_mermaid(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    md.push_str(Texto::MdDiagramaVisualMermaidGrafo.en(idioma));
    md.push_str("```mermaid\ngraph LR\n");
    dibujar_diagrama_mermaid(md, proyecto);
    md.push_str("```\n\n");
}

/// Escribe la sección «Prompts sugeridos» del documento.
///
/// Peticiones ya redactadas para que la persona no tenga que pensar cómo pedir un plan o
/// una auditoría.
///
/// # Parámetros
/// - `md`: el documento en construcción, al que se añade esta sección.
/// - `proyecto`: el mapa del que se toman los datos.
/// - `idioma`: la lengua en que se escriben los prompts. Es la que más importa de todo el
///   documento: esto es lo que la persona copia y pega en su conversación con el modelo.
fn escribir_prompts_sugeridos(md: &mut String, proyecto: &Proyecto, idioma: Idioma) {
    // Peticiones ya redactadas para la persona. Las aperturas de bloque de código no se
    // traducen: ```text es sintaxis de Markdown.
    md.push_str(Texto::MdPromptsDeAccionSugeridos.en(idioma));
    md.push_str(Texto::MdCopiaYPegaCualquiera.en(idioma));

    for (posicion, plantilla) in PlantillaEncargo::TODAS.into_iter().enumerate() {
        md.push_str(plantilla.encabezado_markdown(idioma));
        md.push_str("```text\n");
        md.push_str(&plantilla.contenido(proyecto, idioma));
        if posicion + 1 < PlantillaEncargo::TODAS.len() {
            md.push_str("```\n\n");
        } else {
            md.push_str("```\n");
        }
    }
}
