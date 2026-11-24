//! Resalte de lo que ha hecho la IA, parte de las entradas de la IA (0.15.0): lo que
//! crea por el servidor MCP o por un texto pegado nace marcado como nuevo, y lo que cambia de
//! un nodo ajeno lo deja pendiente de revisión.

use super::{aislar_espacio_mcp, llamar_herramienta, nodo_titulado};
use crate::model::{EstadoRevision, Proyecto};
use crate::storage::{cargar_proyecto_de_archivo, guardar_proyecto_en_archivo};

/// Todo lo que crea `mmcelt_create_mindmap` nace «Generado por IA» y resaltado.
#[test]
fn el_mapa_que_crea_un_agente_nace_resaltado() {
    let espacio = aislar_espacio_mcp("resalte_crear");
    let resultado = llamar_herramienta(
        "mmcelt_create_mindmap",
        serde_json::json!({
            "file_path": "nuevo.mmcelt",
            "title": "Mapa del agente",
            "mindmap_data": { "root_node": { "title": "Raíz", "children": [
                { "title": "Hijo", "review_status": "approved",
                  "children": [{ "title": "Nieto" }] }
            ]}}
        }),
    );
    assert_eq!(resultado["status"].as_str(), Some("success"), "{resultado}");

    let proyecto =
        cargar_proyecto_de_archivo(&espacio.carpeta.join("nuevo.mmcelt")).expect("el mapa se abre");
    assert_eq!(proyecto.nodes.len(), 3);
    for nodo in proyecto.nodes.values() {
        assert_eq!(
            nodo.review_status,
            EstadoRevision::GeneradoPorIA,
            "«{}»",
            nodo.title
        );
        assert!(nodo.nuevo_de_la_ia, "«{}» no está resaltado", nodo.title);
    }
}

/// Escribe un mapa de la persona con un nodo por cada estado de revisión y devuelve su ruta.
fn mapa_con_todos_los_estados(carpeta: &std::path::Path) -> std::path::PathBuf {
    let mut proyecto = Proyecto::nuevo_vacio("Mapa de la persona");
    let raiz = proyecto.root_id;
    let casos = [
        ("Sin marcar", EstadoRevision::SinMarcar, false),
        ("Aprobado", EstadoRevision::AprobadoPorHumano, false),
        ("Corregir", EstadoRevision::RequiereCorreccion, false),
        ("Pendiente", EstadoRevision::PendienteRevision, false),
        ("De la IA resaltado", EstadoRevision::GeneradoPorIA, true),
        ("De la IA visto", EstadoRevision::GeneradoPorIA, false),
    ];
    for (titulo, estado, resaltado) in casos {
        let id = proyecto.anadir_hijo(raiz, titulo);
        let nodo = proyecto.nodes.get_mut(&id).expect("el hijo existe");
        nodo.review_status = estado;
        nodo.nuevo_de_la_ia = resaltado;
        nodo.correction_feedback = format!("corrección de «{titulo}»");
    }
    let ruta = carpeta.join("persona.mmcelt");
    guardar_proyecto_en_archivo(&proyecto, &ruta).expect("el mapa se escribe");
    ruta
}

/// Pide al agente cambiar las notas de los nodos indicados.
fn cambiar_notas(titulos: &[&str], notas: &str) {
    let cambios: Vec<_> = titulos
        .iter()
        .map(|titulo| serde_json::json!({ "title": titulo, "notes": notas }))
        .collect();
    let resultado = llamar_herramienta(
        "mmcelt_sync_ai_progress",
        serde_json::json!({ "file_path": "persona.mmcelt", "node_updates": cambios }),
    );
    assert_eq!(resultado["status"].as_str(), Some("success"), "{resultado}");
}

/// Un nodo que crea la sincronización nace resaltado, y repetirla no lo duplica ni toca
/// los nodos ajenos.
#[test]
fn el_nodo_que_anade_la_sincronizacion_nace_resaltado() {
    let espacio = aislar_espacio_mcp("resalte_sincronizar");
    let ruta = mapa_con_todos_los_estados(&espacio.carpeta);
    let antes = cargar_proyecto_de_archivo(&ruta).expect("el mapa se abre");

    let peticion = serde_json::json!({
        "file_path": "persona.mmcelt",
        "node_updates": [{ "title": "Paso del agente", "parent_title": "Sin marcar" }]
    });
    for _ in 0..2 {
        let resultado = llamar_herramienta("mmcelt_sync_ai_progress", peticion.clone());
        assert_eq!(resultado["status"].as_str(), Some("success"), "{resultado}");
    }

    let despues = cargar_proyecto_de_archivo(&ruta).expect("el mapa se abre");
    assert_eq!(despues.nodes.len(), antes.nodes.len() + 1, "no se duplica");
    let nuevo = nodo_titulado(&despues, "Paso del agente");
    assert_eq!(nuevo.review_status, EstadoRevision::GeneradoPorIA);
    assert!(nuevo.nuevo_de_la_ia);
    for (id, nodo) in &antes.nodes {
        let ahora = &despues.nodes[id];
        assert_eq!(ahora.review_status, nodo.review_status, "«{}»", nodo.title);
        assert_eq!(
            ahora.nuevo_de_la_ia, nodo.nuevo_de_la_ia,
            "«{}»",
            nodo.title
        );
    }
}

/// Lo que la IA cambia de un nodo de la persona, aprobado o por corregir lo deja pendiente;
/// lo suyo y lo pendiente siguen igual, con su resalte, y la corrección no se pierde.
#[test]
fn lo_que_cambia_la_ia_queda_pendiente_de_revision() {
    let espacio = aislar_espacio_mcp("resalte_caducar");
    let ruta = mapa_con_todos_los_estados(&espacio.carpeta);
    let todos = [
        "Sin marcar",
        "Aprobado",
        "Corregir",
        "Pendiente",
        "De la IA resaltado",
        "De la IA visto",
    ];
    cambiar_notas(&todos, "La IA ha reescrito esto");

    let proyecto = cargar_proyecto_de_archivo(&ruta).expect("el mapa se abre");
    let esperado = [
        ("Sin marcar", EstadoRevision::PendienteRevision, false),
        ("Aprobado", EstadoRevision::PendienteRevision, false),
        ("Corregir", EstadoRevision::PendienteRevision, false),
        ("Pendiente", EstadoRevision::PendienteRevision, false),
        ("De la IA resaltado", EstadoRevision::GeneradoPorIA, true),
        ("De la IA visto", EstadoRevision::GeneradoPorIA, false),
    ];
    for (titulo, estado, resaltado) in esperado {
        let nodo = nodo_titulado(&proyecto, titulo);
        assert_eq!(nodo.review_status, estado, "«{titulo}»");
        assert_eq!(nodo.nuevo_de_la_ia, resaltado, "«{titulo}»");
        assert_eq!(
            nodo.correction_feedback,
            format!("corrección de «{titulo}»"),
            "se conserva lo que exigió la persona"
        );
    }
}

/// Una actualización que no cambia nada no toca el estado.
#[test]
fn una_actualizacion_identica_no_caduca_nada() {
    let espacio = aislar_espacio_mcp("resalte_identica");
    let ruta = mapa_con_todos_los_estados(&espacio.carpeta);
    cambiar_notas(&["Aprobado", "Sin marcar", "Corregir"], "");

    let proyecto = cargar_proyecto_de_archivo(&ruta).expect("el mapa se abre");
    assert_eq!(
        nodo_titulado(&proyecto, "Aprobado").review_status,
        EstadoRevision::AprobadoPorHumano
    );
    assert_eq!(
        nodo_titulado(&proyecto, "Sin marcar").review_status,
        EstadoRevision::SinMarcar
    );
    assert_eq!(
        nodo_titulado(&proyecto, "Corregir").review_status,
        EstadoRevision::RequiereCorreccion
    );
}

/// Lo que se importa de un JSON de la IA nace resaltado; sin estado, «Generado por IA»; con
/// estado, el suyo, salvo la aprobación, que queda pendiente. La corrección pegada se conserva.
#[test]
fn lo_que_se_importa_de_un_json_de_la_ia_nace_resaltado() {
    let texto = r#"{ "title": "Mapa", "root_node": { "title": "Raíz", "children": [
        { "title": "Hijo", "children": [{ "title": "Nieto" }] },
        { "title": "Con estado", "review_status": "needs_correction",
          "correction_feedback": "no uses unwrap" },
        { "title": "Se cree aprobado", "review_status": "approved" },
        { "title": "Se cree de la persona", "review_status": "unmarked" }
    ]}}"#;
    let proyecto = crate::ai_bridge::importar_de_texto_de_ia(texto).expect("se importa");

    let esperado = [
        ("Raíz", EstadoRevision::GeneradoPorIA),
        ("Hijo", EstadoRevision::GeneradoPorIA),
        ("Nieto", EstadoRevision::GeneradoPorIA),
        ("Con estado", EstadoRevision::RequiereCorreccion),
        ("Se cree aprobado", EstadoRevision::PendienteRevision),
        ("Se cree de la persona", EstadoRevision::GeneradoPorIA),
    ];
    for (titulo, estado) in esperado {
        let nodo = nodo_titulado(&proyecto, titulo);
        assert_eq!(nodo.review_status, estado, "«{titulo}»");
        assert!(nodo.nuevo_de_la_ia, "«{titulo}» no está resaltado");
    }
    assert_eq!(
        nodo_titulado(&proyecto, "Con estado").correction_feedback,
        "no uses unwrap"
    );
}

/// Lo que se importa de un esquema Markdown de la IA también nace resaltado.
#[test]
fn lo_que_se_importa_de_un_markdown_de_la_ia_nace_resaltado() {
    let texto = "# Raíz\n- Hijo\n  - Nieto\n- Otro hijo\n";
    let proyecto = crate::ai_bridge::importar_de_texto_de_ia(texto).expect("se importa");
    assert!(proyecto.nodes.len() >= 4, "{}", proyecto.nodes.len());
    for nodo in proyecto.nodes.values() {
        assert_eq!(
            nodo.review_status,
            EstadoRevision::GeneradoPorIA,
            "«{}»",
            nodo.title
        );
        assert!(nodo.nuevo_de_la_ia, "«{}» no está resaltado", nodo.title);
    }
}
