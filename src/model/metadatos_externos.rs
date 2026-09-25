//! # Metadatos de MMCelt embebidos en formatos externos (`model/metadatos_externos.rs`)
//!
//! OPML y el `.mm` de FreeMind/Freeplane —los dos formatos no propietarios elegidos para la
//! tanda 5— solo saben representar un árbol con título y una nota de texto por nodo. **Todo
//! lo demás de [`crate::model::Nodo`] y [`crate::model::Proyecto`] no tiene sitio nativo ahí.**
//!
//! Lo que no cabe **no se pierde**: se vuelca como JSON estructurado
//! dentro de la propia nota, para que quien reciba el archivo —MMCelt al reimportarlo, o
//! cualquier otra persona con un editor de texto— pueda aprovechar la información en bruto.
//!
//! Este módulo define **qué** se guarda y **cómo se reconoce** dentro de una nota de texto
//! libre; no sabe nada de OPML ni de XML. Eso vive en `crate::exportacion_opml` y
//! `crate::exportacion_freemind`, que son quienes conocen cada formato.

use crate::model::{
    ConexionCruzada, EstadoNodo, EstadoRevision, ModoDisposicion, Nodo, PrioridadNodo, Proyecto,
    RolNodo,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Versión del esquema de metadatos embebidos.
///
/// Deliberadamente independiente de [`crate::model::VERSION_ESQUEMA`] del `.mmcelt`: son dos
/// contratos distintos que evolucionan por separado.
pub const VERSION_METADATOS_EXTERNOS: u32 = 1;

/// Marca de texto, al principio de su propia línea, que precede al JSON de metadatos de un
/// nodo dentro de su nota exportada.
///
/// Es una constante y no un literal repetido porque la usan tanto quien exporta como quien
/// importa, en dos módulos distintos: un desajuste entre los dos silenciaría toda
/// reconstrucción.
pub const MARCA_METADATOS_NODO: &str = "MMCELT_METADATOS_NODO_JSON:";

/// Marca de texto que precede al JSON de metadatos del proyecto completo.
///
/// Solo aparece **una vez por archivo**, anexada a la nota del nodo raíz: el proyecto no tiene
/// un nodo propio en el árbol exportado donde vivir aparte.
pub const MARCA_METADATOS_PROYECTO: &str = "MMCELT_METADATOS_PROYECTO_JSON:";

/// Todo lo de un nodo que OPML y `.mm` no pueden representar de forma nativa.
///
/// Título, notas de prosa y la jerarquía (padre/hijos) sí viajan en los campos propios del
/// formato de destino: por eso no están aquí. Esto es exactamente el resto.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadatosNodoExterno {
    /// Versión del esquema con la que se escribió este bloque.
    pub schema: u32,
    /// Identificador original del nodo en MMCelt, para resolver conexiones al reimportar.
    pub id: Uuid,
    /// Ruta del archivo de código asociado, si la tenía.
    #[serde(default)]
    pub file_path: Option<String>,
    /// Etiquetas temáticas del nodo.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Estado de madurez.
    pub status: EstadoNodo,
    /// Prioridad estratégica.
    pub priority: PrioridadNodo,
    /// Rol arquitectónico.
    pub role: RolNodo,
    /// Estado de supervisión humana.
    #[serde(default)]
    pub review_status: EstadoRevision,
    /// Corrección explícita exigida a la IA.
    #[serde(default)]
    pub correction_feedback: String,
    /// Posición `[X, Y]` en el lienzo infinito de MMCelt.
    pub pos: [f32; 2],
    /// Si sus hijos estaban plegados en MMCelt.
    pub collapsed: bool,
}

impl MetadatosNodoExterno {
    /// Extrae del nodo todo lo que un formato externo no puede representar de forma nativa.
    pub fn desde_nodo(nodo: &Nodo) -> Self {
        Self {
            schema: VERSION_METADATOS_EXTERNOS,
            id: nodo.id,
            file_path: nodo.file_path.clone(),
            tags: nodo.tags.clone(),
            status: nodo.status,
            priority: nodo.priority,
            role: nodo.role,
            review_status: nodo.review_status,
            correction_feedback: nodo.correction_feedback.clone(),
            pos: nodo.pos,
            collapsed: nodo.collapsed,
        }
    }

    /// Aplica estos metadatos sobre un nodo ya creado con su título, notas y jerarquía.
    ///
    /// No toca `id`, `title`, `notes`, `parent_id` ni `children`: esos los decide quien
    /// reconstruye el árbol a partir de la jerarquía nativa del formato importado, no este
    /// bloque.
    pub fn aplicar_a(&self, nodo: &mut Nodo) {
        nodo.file_path = self.file_path.clone();
        nodo.tags = self.tags.clone();
        nodo.status = self.status;
        nodo.priority = self.priority;
        nodo.role = self.role;
        nodo.review_status = self.review_status;
        nodo.correction_feedback = self.correction_feedback.clone();
        nodo.pos = self.pos;
        nodo.collapsed = self.collapsed;
    }
}

/// Todo lo de un proyecto que no cabe en la cabecera nativa de OPML o `.mm`.
///
/// Incluye las **conexiones cruzadas**: la parte que más se pierde al exportar a un formato
/// clásico, porque ninguno de los dos representa relaciones que no sean de padre a hijo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadatosProyectoExterno {
    /// Versión del esquema con la que se escribió este bloque.
    pub schema: u32,
    /// Identificador original del proyecto en MMCelt.
    pub id: Uuid,
    /// Título del proyecto en MMCelt.
    ///
    /// Puede diferir del texto del nodo raíz exportado (son dos campos independientes en
    /// [`Proyecto`]), así que necesita su propio sitio para no perderse en la ida y vuelta.
    pub title: String,
    /// Fecha de creación original del proyecto en MMCelt.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Fecha de última modificación original del proyecto en MMCelt.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Visión del creador.
    #[serde(default)]
    pub creator_vision: String,
    /// Objetivos concretos del proyecto.
    #[serde(default)]
    pub project_goals: String,
    /// Público objetivo o contexto de aplicación.
    #[serde(default)]
    pub target_audience_or_context: String,
    /// Autor del mapa.
    #[serde(default)]
    pub author: String,
    /// Modo de disposición espacial activo.
    pub layout_mode: ModoDisposicion,
    /// Conexiones cruzadas transversales del mapa completo.
    pub connections: Vec<ConexionCruzada>,
}

impl MetadatosProyectoExterno {
    /// Extrae del proyecto todo lo que un formato externo no puede representar de forma nativa.
    pub fn desde_proyecto(proyecto: &Proyecto) -> Self {
        Self {
            schema: VERSION_METADATOS_EXTERNOS,
            id: proyecto.id,
            title: proyecto.title.clone(),
            created_at: proyecto.created_at,
            updated_at: proyecto.updated_at,
            creator_vision: proyecto.creator_vision.clone(),
            project_goals: proyecto.project_goals.clone(),
            target_audience_or_context: proyecto.target_audience_or_context.clone(),
            author: proyecto.author.clone(),
            layout_mode: proyecto.layout_mode,
            connections: proyecto.connections.clone(),
        }
    }
}

/// Construye el texto de nota que se exporta para un nodo cualquiera.
///
/// Las notas de prosa del usuario van primero, **tal cual las escribió**, para que abrir el
/// archivo en otra aplicación siga siendo legible. El bloque de metadatos va después, en su
/// propia línea, precedido de [`MARCA_METADATOS_NODO`].
///
/// # Parámetros
/// - `notas_usuario`: el texto libre que la persona escribió en el nodo.
/// - `metadatos`: lo que ese formato no puede representar de otra forma.
pub fn construir_nota_de_nodo(notas_usuario: &str, metadatos: &MetadatosNodoExterno) -> String {
    let json = serde_json::to_string(metadatos)
        .expect("MetadatosNodoExterno siempre serializa: no tiene claves ni floats no finitos");
    if notas_usuario.trim().is_empty() {
        format!("{MARCA_METADATOS_NODO} {json}")
    } else {
        format!("{notas_usuario}\n\n{MARCA_METADATOS_NODO} {json}")
    }
}

/// Añade el bloque de metadatos del proyecto a una nota de nodo ya construida.
///
/// Se llama exclusivamente sobre la nota del nodo raíz, después de
/// [`construir_nota_de_nodo`], porque el proyecto no tiene un nodo propio donde vivir.
pub fn anadir_metadatos_de_proyecto(
    nota_raiz: &str,
    metadatos: &MetadatosProyectoExterno,
) -> String {
    let json = serde_json::to_string(metadatos)
        .expect("MetadatosProyectoExterno siempre serializa: no tiene floats no finitos");
    format!("{nota_raiz}\n{MARCA_METADATOS_PROYECTO} {json}")
}

/// Lo que resulta de separar una nota importada en su prosa y sus metadatos MMCelt.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotaImportada {
    /// El texto libre que escribió la persona, sin ninguna de las dos marcas.
    pub notas_usuario: String,
    /// Metadatos del nodo, si la nota los llevaba y el JSON era válido.
    pub metadatos_nodo: Option<MetadatosNodoExterno>,
    /// Metadatos del proyecto, si la nota los llevaba (solo la del nodo raíz debería).
    pub metadatos_proyecto: Option<MetadatosProyectoExterno>,
}

/// Separa el texto de una nota importada en prosa y metadatos.
///
/// Un archivo que no viene de MMCelt —exportado desde otra aplicación— no lleva ninguna marca:
/// en ese caso toda la nota se conserva como `notas_usuario` y los dos campos de metadatos
/// quedan en `None`. Es la señal que usa quien importa para saber que hace falta aplicar
/// valores por defecto y avisar a la persona (C21-D).
///
/// Un JSON mal formado tras una marca reconocida **no interrumpe la importación**: se trata
/// igual que si la marca no estuviera, y la línea completa se conserva como prosa. Un archivo
/// tocado a mano no debe tumbar el resto del mapa.
pub fn separar_nota_importada(texto: &str) -> NotaImportada {
    let mut resultado = NotaImportada::default();
    let mut lineas_de_prosa = Vec::new();

    for linea in texto.lines() {
        if let Some(resto) = linea.strip_prefix(MARCA_METADATOS_NODO) {
            match serde_json::from_str::<MetadatosNodoExterno>(resto.trim()) {
                Ok(metadatos) => {
                    resultado.metadatos_nodo = Some(metadatos);
                    continue;
                }
                Err(_) => lineas_de_prosa.push(linea),
            }
        } else if let Some(resto) = linea.strip_prefix(MARCA_METADATOS_PROYECTO) {
            match serde_json::from_str::<MetadatosProyectoExterno>(resto.trim()) {
                Ok(metadatos) => {
                    resultado.metadatos_proyecto = Some(metadatos);
                    continue;
                }
                Err(_) => lineas_de_prosa.push(linea),
            }
        } else {
            lineas_de_prosa.push(linea);
        }
    }

    resultado.notas_usuario = lineas_de_prosa.join("\n").trim_end().to_string();
    resultado
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::model::TipoRelacion;

    fn nodo_de_ejemplo() -> Nodo {
        let mut nodo = Nodo::new("Autenticación JWT", None, [12.5, -8.0]);
        nodo.notes = "Especificación técnica del módulo.".to_string();
        nodo.file_path = Some("src/auth/jwt.rs".to_string());
        nodo.tags = vec!["backend".to_string(), "seguridad".to_string()];
        nodo.status = EstadoNodo::EnProgreso;
        nodo.priority = PrioridadNodo::Critica;
        nodo.role = RolNodo::AccionTarea;
        nodo.review_status = EstadoRevision::RequiereCorreccion;
        nodo.correction_feedback = "Falta invalidar el refresh token al cerrar sesión.".into();
        nodo.collapsed = true;
        nodo
    }

    #[test]
    fn una_nota_sin_metadatos_conserva_su_prosa_intacta() {
        let importada = separar_nota_importada("Solo texto libre,\nen dos líneas.");
        assert_eq!(importada.notas_usuario, "Solo texto libre,\nen dos líneas.");
        assert!(importada.metadatos_nodo.is_none());
        assert!(importada.metadatos_proyecto.is_none());
    }

    #[test]
    fn una_nota_vacia_produce_metadatos_sin_marca_de_prosa_vacia() {
        let nodo = nodo_de_ejemplo();
        let metadatos = MetadatosNodoExterno::desde_nodo(&nodo);
        let nota = construir_nota_de_nodo("", &metadatos);
        assert!(
            !nota.starts_with('\n'),
            "sin prosa, la nota no debe empezar con una línea en blanco"
        );
        let importada = separar_nota_importada(&nota);
        assert_eq!(importada.notas_usuario, "");
        assert_eq!(importada.metadatos_nodo, Some(metadatos));
    }

    #[test]
    fn ida_y_vuelta_de_los_metadatos_de_un_nodo_es_exacta() {
        let original = nodo_de_ejemplo();
        let metadatos = MetadatosNodoExterno::desde_nodo(&original);
        let nota_exportada = construir_nota_de_nodo(&original.notes, &metadatos);

        let importada = separar_nota_importada(&nota_exportada);
        assert_eq!(importada.notas_usuario, original.notes);

        let metadatos_recuperados = importada
            .metadatos_nodo
            .expect("la nota llevaba metadatos y deben reconocerse");
        assert_eq!(metadatos_recuperados, metadatos);

        let mut reconstruido = Nodo::new(&original.title, None, [0.0, 0.0]);
        metadatos_recuperados.aplicar_a(&mut reconstruido);
        reconstruido.notes = importada.notas_usuario;

        assert_eq!(reconstruido.file_path, original.file_path);
        assert_eq!(reconstruido.tags, original.tags);
        assert_eq!(reconstruido.status, original.status);
        assert_eq!(reconstruido.priority, original.priority);
        assert_eq!(reconstruido.role, original.role);
        assert_eq!(reconstruido.review_status, original.review_status);
        assert_eq!(
            reconstruido.correction_feedback,
            original.correction_feedback
        );
        assert_eq!(reconstruido.pos, original.pos);
        assert_eq!(reconstruido.collapsed, original.collapsed);
        assert_eq!(reconstruido.notes, original.notes);
    }

    #[test]
    fn ida_y_vuelta_de_los_metadatos_de_proyecto_incluye_las_conexiones_cruzadas() {
        let mut proyecto = Proyecto::nuevo_vacio("Proyecto de prueba");
        let raiz = proyecto.root_id;
        let hijo = proyecto.anadir_hijo(raiz, "Módulo B".to_string());
        proyecto.creator_vision = "Visión completa del creador.".to_string();
        proyecto.project_goals = "Entregar la fase 1 antes de fin de año.".to_string();
        proyecto.author = "Programador de prueba".to_string();
        proyecto.connections.push(ConexionCruzada {
            id: Uuid::new_v4(),
            from: raiz,
            to: hijo,
            label: "depende de".to_string(),
            relation_type: TipoRelacion::Dependencia,
        });

        let metadatos = MetadatosProyectoExterno::desde_proyecto(&proyecto);
        let nota_raiz = construir_nota_de_nodo(
            "Nota de la raíz.",
            &MetadatosNodoExterno::desde_nodo(proyecto.nodo_raiz().unwrap()),
        );
        let nota_completa = anadir_metadatos_de_proyecto(&nota_raiz, &metadatos);

        let importada = separar_nota_importada(&nota_completa);
        assert_eq!(importada.notas_usuario, "Nota de la raíz.");
        assert!(importada.metadatos_nodo.is_some());
        let metadatos_recuperados = importada
            .metadatos_proyecto
            .expect("la nota de la raíz llevaba metadatos de proyecto");
        assert_eq!(
            metadatos_recuperados.creator_vision,
            proyecto.creator_vision
        );
        assert_eq!(metadatos_recuperados.project_goals, proyecto.project_goals);
        assert_eq!(metadatos_recuperados.author, proyecto.author);
        assert_eq!(metadatos_recuperados.connections.len(), 1);
        assert_eq!(metadatos_recuperados.connections[0].from, raiz);
        assert_eq!(metadatos_recuperados.connections[0].to, hijo);
        assert_eq!(
            metadatos_recuperados.connections[0].relation_type,
            TipoRelacion::Dependencia
        );
    }

    #[test]
    fn un_json_corrupto_tras_la_marca_se_conserva_como_prosa_en_vez_de_perder_el_nodo() {
        let texto = format!("{MARCA_METADATOS_NODO} {{ esto no es json valido");
        let importada = separar_nota_importada(&texto);
        assert!(importada.metadatos_nodo.is_none());
        assert_eq!(importada.notas_usuario, texto);
    }

    #[test]
    fn un_archivo_ajeno_sin_ninguna_marca_no_produce_metadatos_de_ningun_tipo() {
        let importada = separar_nota_importada(
            "Nota exportada desde otra aplicación de mapas mentales, sin nada de MMCelt.",
        );
        assert!(importada.metadatos_nodo.is_none());
        assert!(importada.metadatos_proyecto.is_none());
    }
}
