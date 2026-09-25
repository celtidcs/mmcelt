//! # MMCelt - Aplicación Portable de Mapas Mentales para Inteligencia Artificial
//!
//! MMCelt es una aplicación de escritorio nativa escrita en Rust utilizando `egui` y `eframe`.
//! Actúa como consola visual de alineación, plano de ingeniería de software y control bidireccional
//! para modelos de Inteligencia Artificial (Claude, ChatGPT, Gemini, etc.).
//!
//! ## Licencia
//!
//! Copyright (C) 2026 Celtilander
//!
//! Este programa es software libre: puedes redistribuirlo y modificarlo bajo los términos de la
//! Licencia Pública General de GNU publicada por la Free Software Foundation, en su versión 3 o
//! cualquier versión posterior.
//!
//! Se distribuye con la esperanza de que resulte útil, pero **sin ninguna garantía**; ni siquiera
//! la garantía implícita de comerciabilidad o idoneidad para un propósito particular. Consulta la
//! Licencia Pública General de GNU para más detalles.
//!
//! El texto íntegro está en el archivo `LICENSE`, en la raíz del repositorio, y en
//! <https://www.gnu.org/licenses/>.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
#![deny(missing_docs)]
// `rustc --test` inserta permisos propios para elementos del arnés y rechaza que el crate los
// contradiga con `forbid`. Producción sí usa `forbid`, donde ningún `#[allow]` local puede ocultar
// código muerto; las pruebas conservan `deny` para detectar el mismo defecto sin impedir el arnés.
#![cfg_attr(
    not(test),
    forbid(dead_code, unused_imports, unused_mut, unused_variables)
)]
#![cfg_attr(test, deny(dead_code, unused_imports, unused_mut, unused_variables))]

mod ai_bridge;
mod ai_export;
mod aplicacion;
mod autoguardado;
mod busqueda;
mod carga_en_segundo_plano;
mod conectores;
mod configuracion_agente_proyecto;
mod consola_windows;
mod devolucion_agentes;
mod envio_agente;
mod envio_agente_comando;
mod error;
mod escaner_repositorio;
mod exportacion_freemind;
mod exportacion_freemind_comando;
mod exportacion_opml;
mod exportacion_opml_comando;
mod guardado_como;
mod historial;
mod lanzador_agentes;
mod layout;
mod mcp_server;
mod model;
mod plantillas;
mod preferencias;
mod proyecto_trabajo;
mod proyectos;
mod respaldo;
mod sesiones_agentes;
mod storage;
mod textos;
mod theme;
mod traslado;
mod ui;
mod version;
mod vigilancia_mapa;
mod vigilante;

/// Arnés para ejecutar la interfaz dentro de una prueba sin abrir ventana (solo con `cargo test`).
#[cfg(test)]
mod arnes_interfaz;

use aplicacion::AplicacionMapaMental;

/// Argumento que arranca el ejecutable como servidor MCP en lugar de como aplicación.
const ARGUMENTO_SERVIDOR_MCP: &str = "--mcp-server";

/// Píxeles del icono de la ventana, sin comprimir.
///
/// Se guarda en crudo, y no como PNG, para no arrastrar un descodificador de imágenes
/// solo por dibujar un icono: son 16 KB que el compilador incrusta tal cual.
///
/// El archivo se genera a partir de `assets/icono/mmcelt.svg`, que es la fuente editable.
const ICONO_RGBA: &[u8] = include_bytes!("../assets/icono/mmcelt-64.rgba");

/// Lado del icono de la ventana, en píxeles.
const ICONO_LADO: u32 = 64;

/// Construye el icono que Windows muestra en la barra de título y en la barra de tareas.
///
/// El icono del archivo `.exe` es otra cosa distinta: ese se incrusta como recurso al
/// compilar, en `build.rs`. Este es el de la ventana en marcha, y se pone aquí porque no
/// depende del sistema operativo.
///
/// # Devuelve
/// El icono listo para el constructor de la ventana.
fn icono_de_la_ventana() -> egui::IconData {
    egui::IconData {
        rgba: ICONO_RGBA.to_vec(),
        width: ICONO_LADO,
        height: ICONO_LADO,
    }
}

/// Punto de entrada principal.
///
/// El mismo ejecutable sirve para dos cosas:
///
/// - **Sin argumentos**: abre la aplicación de escritorio.
/// - **Con `--mcp-server`**: se comporta como servidor del Model Context Protocol,
///   hablando JSON-RPC por la entrada y la salida estándar.
///
/// Ese segundo modo es lo que permite conectar MMCelt con un agente de IA **sin instalar
/// nada más**: lo que se registra en el agente es este mismo binario. Un servidor
/// separado en otro lenguaje obligaría a que el equipo de destino tuviera ese lenguaje
/// instalado, lo que estropea el caso de pasarle la aplicación a otra persona.
fn main() -> eframe::Result<()> {
    let argumentos: Vec<String> = std::env::args().collect();

    // `--version` responde por consola y termina. Sirve para comprobar qué es un
    // ejecutable sin abrirlo, que es lo práctico cuando hay varias copias por el disco
    // y se duda de cuál es la actual.
    if argumentos.iter().any(|a| a == "--version" || a == "-V") {
        consola_windows::preparar_salida_de_version();
        version::imprimir_version();
        return Ok(());
    }

    if argumentos.iter().any(|a| a == ARGUMENTO_SERVIDOR_MCP) {
        // En este modo no se abre ninguna ventana: el proceso vive de la tubería que le
        // tiende el agente y termina cuando esta se cierra.
        std::process::exit(mcp_server::ejecutar());
    }

    // La versión va en el título de la ventana: identifica la compilación incluso con
    // varias instancias abiertas a la vez, sin tener que entrar en ningún menú.
    let titulo = format!("MMCelt v{} - Mapas Mentales para IA", version::VERSION);

    let opciones_de_ventana = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(&titulo)
            .with_icon(icono_de_la_ventana())
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 500.0])
            .with_active(true),
        ..Default::default()
    };

    eframe::run_native(
        "MMCelt MindMap",
        opciones_de_ventana,
        Box::new(|cc| Ok(Box::new(AplicacionMapaMental::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::{EstadoNodo, PrioridadNodo, Proyecto, RolNodo, TipoRelacion};

    #[test]
    /// El icono incrustado mide exactamente lo que dice medir.
    ///
    /// Se guarda en crudo, sin cabecera que lo describa, así que nada impide que el archivo y
    /// la constante de tamaño se desincronicen. Si eso pasa, `egui` lee fuera del búfer o
    /// dibuja basura.
    fn el_icono_de_la_ventana_tiene_el_tamano_declarado() {
        // El icono se incrusta como pixeles en crudo, sin cabecera que diga cuanto mide:
        // si alguien regenera el archivo con otro tamano y no toca `ICONO_LADO`, la
        // ventana arrancaria con un icono deformado o directamente vacio. Aqui se ata.
        let esperados = (ICONO_LADO * ICONO_LADO * 4) as usize;
        assert_eq!(
            ICONO_RGBA.len(),
            esperados,
            "el icono deberia ocupar {esperados} bytes para ser de {ICONO_LADO}x{ICONO_LADO}"
        );

        let icono = icono_de_la_ventana();
        assert_eq!(icono.width, ICONO_LADO);
        assert_eq!(icono.height, ICONO_LADO);
        assert!(
            icono.rgba.iter().skip(3).step_by(4).any(|&alfa| alfa > 0),
            "el icono es completamente transparente"
        );
    }

    #[test]
    /// Borrar un nodo se lleva a toda su descendencia, y no deja referencias colgando.
    ///
    /// Un hijo que sobrevive a su padre queda huérfano, y la validación estructural rechaza
    /// el mapa entero la próxima vez que se abra.
    fn la_jerarquia_y_el_borrado_en_cascada_funcionan() {
        let mut proyecto = Proyecto::nuevo_vacio("Proyecto de Prueba");
        let root_id = proyecto.root_id;

        let primer_hijo = proyecto.anadir_hijo(root_id, "Pilar 1");
        let segundo_hijo = proyecto.anadir_hijo(root_id, "Pilar 2");
        let nieto = proyecto.anadir_hijo(primer_hijo, "Subtema 1.1");

        assert_eq!(proyecto.nodes.len(), 4);
        assert_eq!(proyecto.nodes[&root_id].children.len(), 2);
        assert_eq!(proyecto.nodes[&primer_hijo].children.len(), 1);

        // Al borrar el primer hijo debe irse con él toda su descendencia.
        proyecto.eliminar_nodo(primer_hijo);
        assert_eq!(proyecto.nodes.len(), 2);
        assert!(!proyecto.nodes.contains_key(&primer_hijo));
        assert!(!proyecto.nodes.contains_key(&nieto));
        assert!(proyecto.nodes.contains_key(&segundo_hijo));
        assert_eq!(proyecto.nodes[&root_id].children.len(), 1);
    }

    #[test]
    /// El documento que se le entrega a la IA contiene lo que dice contener.
    ///
    /// Es la salida principal del programa: si se deja fuera la visión, las metas o el estado
    /// de un nodo, el modelo trabaja con menos información de la que el usuario cree haberle
    /// dado.
    fn el_markdown_exportado_lleva_lo_que_promete() {
        let mut proyecto = Proyecto::nuevo_vacio("Software Modular Rust");
        proyecto.creator_vision = "Crear un motor de análisis estático ultra rápido".to_string();
        proyecto.project_goals =
            "Lanzar MVP en 3 semanas con 0 dependencias externas pesadas".to_string();
        proyecto.target_audience_or_context = "Desarrolladores de sistemas críticos".to_string();

        let root_id = proyecto.root_id;
        let p1 = proyecto.anadir_hijo(root_id, "Núcleo del Parser");
        if let Some(n) = proyecto.nodes.get_mut(&p1) {
            n.notes = "Implementar AST recursivo descendente".to_string();
            n.tags = vec!["core".into(), "parser".into()];
            n.status = EstadoNodo::EnProgreso;
            n.priority = PrioridadNodo::Critica;
        }

        let p2 = proyecto.anadir_hijo(root_id, "¿Qué formato de salida usar?");
        if let Some(n) = proyecto.nodes.get_mut(&p2) {
            n.notes = "Evaluar JSON vs SARIF vs Markdown".to_string();
            n.role = RolNodo::HipotesisDuda;
            n.status = EstadoNodo::DudaBloqueo;
        }

        proyecto.anadir_conexion_cruzada(
            p2,
            p1,
            "El parser debe emitir los spans para el formato",
            TipoRelacion::Dependencia,
        );

        let md = ai_export::exportar_markdown_para_ia(&proyecto, crate::textos::Idioma::Espanol);

        assert!(md.contains("MAPA MENTAL DEL PROYECTO: Software Modular Rust"));
        assert!(md.contains("Crear un motor de análisis estático ultra rápido"));
        assert!(md.contains("Lanzar MVP en 3 semanas"));
        assert!(md.contains("Núcleo del Parser"));
        assert!(md.contains("PUNTOS DE DECISIÓN, DUDAS Y BLOQUEOS A RESOLVER"));
        assert!(md.contains("¿Qué formato de salida usar?"));
        assert!(md.contains("mermaid"));
        assert!(md.contains("PROMPTS DE ACCIÓN SUGERIDOS PARA LA IA"));
    }

    #[test]
    /// Lo que escribe una IA vuelve a ser un mapa completo al importarlo.
    ///
    /// Cierra el círculo con la exportación: lo que sale tiene que poder volver a entrar sin
    /// perder nodos, conexiones ni campos por el camino.
    fn la_importacion_de_json_reconstruye_el_mapa() {
        let respuesta_de_ejemplo = r#"
        Aquí tienes el mapa mental estructurado según tu solicitud:

        ```json
        {
          "title": "Ecosistema Drone Autónomo",
          "creator_vision": "Sistema de navegación visual para rescates",
          "project_goals": "Prototipo funcional con visión artificial",
          "root_node": {
            "title": "Drone Autónomo",
            "children": [
              {
                "title": "Sensores y Hardware",
                "notes": "Cámaras estéreo y LIDAR",
                "tags": ["hardware"],
                "children": [
                  {
                    "title": "Elección de MCU",
                    "role": "HipotesisDuda",
                    "status": "DudaBloqueo"
                  }
                ]
              },
              {
                "title": "Algoritmos SLAM",
                "tags": ["software"]
              }
            ]
          },
          "cross_connections": [
            {
              "from_title": "Elección de MCU",
              "to_title": "Algoritmos SLAM",
              "label": "Rendimiento requerido",
              "relation_type": "Dependencia"
            }
          ]
        }
        ```
        Espero que te sea de utilidad.
        "#;

        let importado = ai_bridge::importar_de_texto_de_ia(respuesta_de_ejemplo);
        assert!(
            importado.is_ok(),
            "la importación debería funcionar: {:?}",
            importado.err()
        );
        let proyecto = importado.unwrap();

        assert_eq!(proyecto.title, "Ecosistema Drone Autónomo");
        assert_eq!(
            proyecto.creator_vision,
            "Sistema de navegación visual para rescates"
        );
        assert_eq!(proyecto.nodes.len(), 4);
        assert_eq!(proyecto.connections.len(), 1);
    }
}
