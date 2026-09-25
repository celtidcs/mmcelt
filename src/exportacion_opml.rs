//! # Exportación e importación a OPML (`exportacion_opml.rs`)
//!
//! OPML es un árbol de `<outline>` con dos campos nativos: `text` (el título) y, por convención
//! —no forma parte del estándar, pero la usan los principales editores de esquemas— el atributo
//! `_note` para una nota de texto libre. Es exactamente lo que necesita MMCelt: título y notas
//! del nodo viajan ahí; todo lo demás va como JSON dentro de la propia nota, según decide
//! `crate::model::metadatos_externos` (C21-A).
//!
//! Este módulo solo sabe de XML y de recorrer el árbol. La forma del JSON, sus marcas y cómo se
//! separa de la prosa viven en el módulo de dominio; aquí no se duplica esa lógica.

use crate::model::{
    anadir_metadatos_de_proyecto, construir_nota_de_nodo, separar_nota_importada,
    MetadatosNodoExterno, MetadatosProyectoExterno, Nodo, Proyecto,
};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::borrow::Cow;

/// Causa concreta por la que un documento OPML no pudo importarse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorOpml {
    /// El XML no está bien formado; conserva el mensaje original de `quick-xml`.
    XmlMalformado(String),
    /// El documento no tenía ni un solo `<outline>` de nivel superior dentro de `<body>`.
    SinContenido,
}

/// Resultado de importar un archivo externo: el proyecto reconstruido y si el archivo traía
/// metadatos propios de MMCelt.
#[derive(Debug)]
pub struct ProyectoImportado {
    /// El mapa reconstruido, con valores por defecto donde no había metadatos de MMCelt.
    pub proyecto: Proyecto,
    /// `false` cuando ningún nodo llevaba el bloque JSON de MMCelt: la señal de C21-D para que
    /// la interfaz avise a la persona de que el archivo no venía de MMCelt.
    pub trae_metadatos_mmcelt: bool,
}

/// Exporta un proyecto completo a un documento OPML 2.0.
pub fn exportar_a_opml(proyecto: &Proyecto) -> String {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    let mut opml = BytesStart::new("opml");
    opml.push_attribute(("version", "2.0"));
    writer
        .write_event(Event::Start(opml))
        .expect(FALLO_ESCRITURA);

    writer
        .write_event(Event::Start(BytesStart::new("head")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::Start(BytesStart::new("title")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::Text(BytesText::new(&proyecto.title)))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::End(BytesEnd::new("title")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::End(BytesEnd::new("head")))
        .expect(FALLO_ESCRITURA);

    writer
        .write_event(Event::Start(BytesStart::new("body")))
        .expect(FALLO_ESCRITURA);
    if let Some(raiz) = proyecto.nodo_raiz() {
        escribir_outline(&mut writer, proyecto, raiz);
    }
    writer
        .write_event(Event::End(BytesEnd::new("body")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::End(BytesEnd::new("opml")))
        .expect(FALLO_ESCRITURA);

    String::from_utf8(writer.into_inner()).expect("quick-xml solo escribe UTF-8 válido")
}

/// Mensaje del `.expect()` interno: escribir en un `Vec<u8>` en memoria no puede fallar por E/S,
/// así que un error aquí solo puede venir de un evento XML mal formado por este mismo módulo.
const FALLO_ESCRITURA: &str = "escribir en un buffer en memoria no debería fallar";

/// Construye un atributo XML cuyo valor puede contener saltos de línea sin perderlos.
///
/// La conversión automática `(&str, &str) -> Attribute` de `quick-xml` escapa `& < > ' "`, pero
/// dentro de un valor de atributo el estándar de XML exige además **normalizar** cualquier salto
/// de línea literal a un simple espacio al leerlo de vuelta (XML 1.0, §3.3.3): es justo lo que le
/// pasaba a la nota de un nodo con varias líneas antes de esta función, y solo se detectó al
/// subir `quick-xml` a una versión que por fin implementa esa norma correctamente.
///
/// La propia norma exceptúa de la normalización a las **referencias de carácter**: `&#10;` se
/// restituye como el salto de línea real, en vez de convertirse en un espacio. Por eso este
/// atributo se construye a mano —tras el escapado estándar, cada salto de línea u retorno de
/// carro se sustituye por su referencia numérica— en vez de con la conversión automática.
fn atributo_de_texto<'a>(nombre: &'static str, valor: &str) -> Attribute<'a> {
    let escapado = quick_xml::escape::escape(valor);
    let seguro = escapado.replace('\r', "&#13;").replace('\n', "&#10;");
    Attribute {
        key: QName(nombre.as_bytes()),
        value: Cow::Owned(seguro.into_bytes()),
    }
}

/// Escribe recursivamente un nodo y sus descendientes como `<outline>`.
fn escribir_outline(writer: &mut Writer<Vec<u8>>, proyecto: &Proyecto, nodo: &Nodo) {
    let metadatos_nodo = MetadatosNodoExterno::desde_nodo(nodo);
    let mut nota = construir_nota_de_nodo(&nodo.notes, &metadatos_nodo);
    if nodo.id == proyecto.root_id {
        let metadatos_proyecto = MetadatosProyectoExterno::desde_proyecto(proyecto);
        nota = anadir_metadatos_de_proyecto(&nota, &metadatos_proyecto);
    }

    let mut inicio = BytesStart::new("outline");
    inicio.push_attribute(atributo_de_texto("text", &nodo.title));
    inicio.push_attribute(atributo_de_texto("_note", &nota));

    let hijos: Vec<&Nodo> = nodo
        .children
        .iter()
        .filter_map(|id| proyecto.nodes.get(id))
        .collect();

    if hijos.is_empty() {
        writer
            .write_event(Event::Empty(inicio))
            .expect(FALLO_ESCRITURA);
    } else {
        writer
            .write_event(Event::Start(inicio))
            .expect(FALLO_ESCRITURA);
        for hijo in hijos {
            escribir_outline(writer, proyecto, hijo);
        }
        writer
            .write_event(Event::End(BytesEnd::new("outline")))
            .expect(FALLO_ESCRITURA);
    }
}

/// Importa un documento OPML, sea o no de MMCelt.
///
/// Reconstruye la jerarquía del árbol desde el anidamiento real de los `<outline>`. Si el
/// `_note` de un nodo lleva el bloque JSON de MMCelt, se restablecen su estado, prioridad, rol,
/// revisión, comentario, ruta, etiquetas, posición y plegado exactos (C21-C). Si no lo lleva,
/// el nodo se crea con los valores por defecto de [`Nodo::new`] y solo con el texto como notas
/// (C21-D): la persona que llama decide si avisar, mirando `trae_metadatos_mmcelt`.
pub fn importar_desde_opml(xml: &str) -> Result<ProyectoImportado, ErrorOpml> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut titulo_cabecera: Option<String> = None;
    let mut dentro_de_head = false;
    let mut dentro_de_title = false;

    // Pila de outlines abiertos: (nodo en construcción, notas crudas pendientes de separar).
    let mut pila: Vec<Nodo> = Vec::new();
    let mut raiz: Option<Nodo> = None;
    let mut trae_metadatos_mmcelt = false;
    let mut metadatos_proyecto: Option<MetadatosProyectoExterno> = None;
    let mut nodos_planos: Vec<Nodo> = Vec::new();

    loop {
        let evento = reader
            .read_event()
            .map_err(|e| ErrorOpml::XmlMalformado(e.to_string()))?;
        match evento {
            Event::Eof => break,
            Event::Start(inicio) if inicio.name().as_ref() == b"head" => dentro_de_head = true,
            Event::End(fin) if fin.name().as_ref() == b"head" => dentro_de_head = false,
            Event::Start(inicio) if dentro_de_head && inicio.name().as_ref() == b"title" => {
                dentro_de_title = true
            }
            Event::End(fin) if fin.name().as_ref() == b"title" => dentro_de_title = false,
            Event::Text(texto) if dentro_de_title => {
                let decodificado = texto
                    .decode()
                    .map_err(|e| ErrorOpml::XmlMalformado(e.to_string()))?;
                let desescapado = quick_xml::escape::unescape(&decodificado)
                    .map_err(|e| ErrorOpml::XmlMalformado(e.to_string()))?;
                titulo_cabecera = Some(desescapado.into_owned());
            }
            Event::Start(inicio) if inicio.name().as_ref() == b"outline" => {
                let (nodo, tenia_metadatos, meta_proyecto) =
                    leer_nodo_desde_atributos(&inicio, &mut reader)?;
                trae_metadatos_mmcelt |= tenia_metadatos;
                if meta_proyecto.is_some() {
                    metadatos_proyecto = meta_proyecto;
                }
                pila.push(nodo);
            }
            Event::Empty(inicio) if inicio.name().as_ref() == b"outline" => {
                let (nodo, tenia_metadatos, meta_proyecto) =
                    leer_nodo_desde_atributos(&inicio, &mut reader)?;
                trae_metadatos_mmcelt |= tenia_metadatos;
                if meta_proyecto.is_some() {
                    metadatos_proyecto = meta_proyecto;
                }
                anadir_como_hijo_o_raiz(nodo, &mut pila, &mut raiz, &mut nodos_planos);
            }
            Event::End(fin) if fin.name().as_ref() == b"outline" => {
                if let Some(nodo) = pila.pop() {
                    anadir_como_hijo_o_raiz(nodo, &mut pila, &mut raiz, &mut nodos_planos);
                }
            }
            _ => {}
        }
    }

    let mut raiz = raiz.ok_or(ErrorOpml::SinContenido)?;
    if raiz.role == crate::model::RolNodo::default() {
        // Ningún metadato de MMCelt fijó el rol de la raíz (ver el reseteo neutral en
        // `leer_nodo_desde_atributos`): el papel semántico de la raíz de un mapa mental es
        // `IdeaCentral`, igual que cuando se crea un proyecto nuevo. Si la raíz sí traía
        // metadatos con un rol explícito, esta rama no se ejecuta y no se pisa nada.
        raiz.role = crate::model::RolNodo::IdeaCentral;
    }
    let mut proyecto = Proyecto::nuevo_vacio(
        titulo_cabecera
            .clone()
            .unwrap_or_else(|| raiz.title.clone()),
    );
    let root_id_generado = proyecto.root_id;
    proyecto.nodes.remove(&root_id_generado);
    proyecto.root_id = raiz.id;
    proyecto.nodes.insert(raiz.id, raiz);
    for nodo in nodos_planos {
        proyecto.nodes.insert(nodo.id, nodo);
    }

    if let Some(metadatos) = metadatos_proyecto {
        proyecto.title = metadatos.title;
        proyecto.created_at = metadatos.created_at;
        proyecto.updated_at = metadatos.updated_at;
        proyecto.creator_vision = metadatos.creator_vision;
        proyecto.project_goals = metadatos.project_goals;
        proyecto.target_audience_or_context = metadatos.target_audience_or_context;
        proyecto.author = metadatos.author;
        proyecto.layout_mode = metadatos.layout_mode;
        proyecto.connections = metadatos.connections;
        proyecto.id = metadatos.id;
    } else if let Some(titulo) = titulo_cabecera {
        proyecto.title = titulo;
    }

    Ok(ProyectoImportado {
        proyecto,
        trae_metadatos_mmcelt,
    })
}

/// Cuelga un nodo ya cerrado de la cima de la pila (si la hay) o lo marca como raíz.
fn anadir_como_hijo_o_raiz(
    nodo: Nodo,
    pila: &mut [Nodo],
    raiz: &mut Option<Nodo>,
    nodos_planos: &mut Vec<Nodo>,
) {
    if let Some(padre) = pila.last_mut() {
        let mut nodo = nodo;
        nodo.parent_id = Some(padre.id);
        padre.children.push(nodo.id);
        nodos_planos.push(nodo);
    } else if raiz.is_none() {
        *raiz = Some(nodo);
    } else {
        // Un segundo outline de nivel superior en el body: OPML lo permite, MMCelt no tiene
        // sitio para un segundo árbol raíz, así que se descarta en silencio. No hay caso de
        // uso real que produzca esto desde MMCelt, y documentarlo aquí evita reabrir la duda.
    }
}

/// Lee `text` y `_note` de un `<outline>`, separa la nota en prosa y metadatos, y construye el
/// nodo correspondiente con valores por defecto donde no había JSON de MMCelt.
///
/// Devuelve también si la nota llevaba un bloque de metadatos de **nodo** reconocible: es la
/// señal exacta de C21-D, tomada directamente de si `separar_nota_importada` encontró y validó
/// la marca, no de comparar el resultado contra unos valores por defecto a posteriori.
fn leer_nodo_desde_atributos(
    inicio: &BytesStart,
    reader: &mut Reader<&[u8]>,
) -> Result<(Nodo, bool, Option<MetadatosProyectoExterno>), ErrorOpml> {
    let mut texto = String::new();
    let mut nota_cruda = String::new();
    for atributo in inicio.attributes() {
        let atributo = atributo.map_err(|e| ErrorOpml::XmlMalformado(e.to_string()))?;
        let valor = atributo
            .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
            .map_err(|e| ErrorOpml::XmlMalformado(e.to_string()))?;
        match atributo.key.as_ref() {
            b"text" => texto = valor.into_owned(),
            b"_note" => nota_cruda = valor.into_owned(),
            _ => {}
        }
    }

    let separada = separar_nota_importada(&nota_cruda);
    let mut nodo = Nodo::new(texto, None, [0.0, 0.0]);
    // `Nodo::new` decide el rol a partir del `parent_id` que se le pasa, y aquí siempre se le
    // pasa `None` porque el padre real todavía no se conoce (se resuelve al cerrar el árbol).
    // Sin este reseteo, todo nodo sin metadatos propios heredaría `IdeaCentral`, el rol que
    // `Nodo::new` reserva a la raíz, aunque sea una hoja profunda del árbol importado.
    nodo.role = crate::model::RolNodo::default();

    let tenia_metadatos_de_nodo = separada.metadatos_nodo.is_some();
    if let Some(metadatos) = &separada.metadatos_nodo {
        nodo.id = metadatos.id;
    }
    nodo.notes = separada.notas_usuario;
    if let Some(metadatos) = separada.metadatos_nodo {
        metadatos.aplicar_a(&mut nodo);
    }

    Ok((nodo, tenia_metadatos_de_nodo, separada.metadatos_proyecto))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::model::{
        ConexionCruzada, EstadoNodo, EstadoRevision, ModoDisposicion, PrioridadNodo, RolNodo,
        TipoRelacion,
    };
    use uuid::Uuid;

    fn mapa_de_ejemplo() -> Proyecto {
        let mut proyecto = Proyecto::nuevo_vacio("Backend del pedido");
        proyecto.creator_vision = "Sustituir el monolito por microservicios.".to_string();
        proyecto.project_goals = "Cero caídas en el cambio.".to_string();
        proyecto.target_audience_or_context = "Equipo de plataforma.".to_string();
        proyecto.author = "Programador de prueba".to_string();
        proyecto.layout_mode = ModoDisposicion::RadialTree;

        let raiz_id = proyecto.root_id;
        {
            let raiz = proyecto.nodes.get_mut(&raiz_id).unwrap();
            raiz.notes = "Nota de la raíz, con acentos y «comillas».".to_string();
            raiz.priority = PrioridadNodo::Alta;
        }

        let auth_id = proyecto.anadir_hijo(raiz_id, "Autenticación JWT");
        {
            let auth = proyecto.nodes.get_mut(&auth_id).unwrap();
            auth.notes = "Especificación & <detalles> \"técnicos\".".to_string();
            auth.file_path = Some("src/auth/jwt.rs".to_string());
            auth.tags = vec!["backend".to_string(), "seguridad".to_string()];
            auth.status = EstadoNodo::EnProgreso;
            auth.priority = PrioridadNodo::Critica;
            auth.role = RolNodo::AccionTarea;
            auth.review_status = EstadoRevision::RequiereCorreccion;
            auth.correction_feedback = "Falta invalidar el refresh token.".to_string();
            auth.pos = [120.0, -45.5];
            auth.collapsed = true;
        }

        let db_id = proyecto.anadir_hijo(raiz_id, "Base de datos");
        proyecto.connections.push(ConexionCruzada {
            id: Uuid::new_v4(),
            from: auth_id,
            to: db_id,
            label: "depende de".to_string(),
            relation_type: TipoRelacion::Dependencia,
        });

        proyecto
    }

    #[test]
    fn exportar_un_mapa_vacio_produce_un_outline_unico() {
        let proyecto = Proyecto::nuevo_vacio("Proyecto vacío");
        let xml = exportar_a_opml(&proyecto);
        assert!(xml.contains("<opml version=\"2.0\">"));
        assert!(xml.contains("<outline"));
    }

    #[test]
    fn importar_un_xml_malformado_devuelve_el_error_tipado() {
        let resultado = importar_desde_opml("<opml><body><outline text=\"sin cerrar\">");
        assert!(matches!(
            resultado,
            Err(ErrorOpml::XmlMalformado(_)) | Err(ErrorOpml::SinContenido)
        ));
    }

    #[test]
    fn importar_un_documento_sin_ningun_outline_devuelve_sin_contenido() {
        let resultado = importar_desde_opml(
            r#"<?xml version="1.0"?><opml version="2.0"><head><title>Vacío</title></head><body></body></opml>"#,
        );
        assert_eq!(resultado.unwrap_err(), ErrorOpml::SinContenido);
    }

    /// **C21-C** — exportar un mapa de MMCelt y reimportarlo reconstruye estado, prioridad,
    /// rol, revisión, comentario, etiquetas, ruta, posición, plegado y conexiones cruzadas
    /// idénticos. Ida y vuelta exacta, campo a campo.
    #[test]
    fn ida_y_vuelta_de_un_mapa_completo_es_exacta() {
        let original = mapa_de_ejemplo();
        let xml = exportar_a_opml(&original);

        let importado = importar_desde_opml(&xml).expect("el XML propio siempre es válido");
        assert!(
            importado.trae_metadatos_mmcelt,
            "un archivo exportado por MMCelt debe reconocerse como tal"
        );

        let reconstruido = importado.proyecto;
        assert_eq!(reconstruido.title, original.title);
        assert_eq!(reconstruido.creator_vision, original.creator_vision);
        assert_eq!(reconstruido.project_goals, original.project_goals);
        assert_eq!(
            reconstruido.target_audience_or_context,
            original.target_audience_or_context
        );
        assert_eq!(reconstruido.author, original.author);
        assert_eq!(reconstruido.layout_mode, original.layout_mode);
        assert_eq!(reconstruido.connections, original.connections);
        assert_eq!(reconstruido.nodes.len(), original.nodes.len());

        for (id, nodo_original) in &original.nodes {
            let nodo_reconstruido = reconstruido
                .nodes
                .get(id)
                .unwrap_or_else(|| panic!("falta el nodo {id} tras reimportar"));
            assert_eq!(nodo_reconstruido.title, nodo_original.title);
            assert_eq!(nodo_reconstruido.notes, nodo_original.notes);
            assert_eq!(nodo_reconstruido.file_path, nodo_original.file_path);
            assert_eq!(nodo_reconstruido.tags, nodo_original.tags);
            assert_eq!(nodo_reconstruido.status, nodo_original.status);
            assert_eq!(nodo_reconstruido.priority, nodo_original.priority);
            assert_eq!(nodo_reconstruido.role, nodo_original.role);
            assert_eq!(nodo_reconstruido.review_status, nodo_original.review_status);
            assert_eq!(
                nodo_reconstruido.correction_feedback,
                nodo_original.correction_feedback
            );
            assert_eq!(nodo_reconstruido.pos, nodo_original.pos);
            assert_eq!(nodo_reconstruido.collapsed, nodo_original.collapsed);
            assert_eq!(nodo_reconstruido.parent_id, nodo_original.parent_id);
            assert_eq!(
                nodo_reconstruido.children.len(),
                nodo_original.children.len()
            );
        }
    }

    /// **C21-D** — un OPML ajeno, sin ningún bloque de MMCelt, se importa con valores por
    /// defecto y la señal `trae_metadatos_mmcelt` en `false`.
    #[test]
    fn importar_un_opml_ajeno_usa_valores_por_defecto_y_no_senala_metadatos() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head><title>Exportado desde otra aplicación</title></head>
  <body>
    <outline text="Idea central">
      <outline text="Rama uno"/>
      <outline text="Rama dos">
        <outline text="Nieta"/>
      </outline>
    </outline>
  </body>
</opml>"#;

        let importado = importar_desde_opml(xml).expect("es un OPML válido, aunque ajeno");
        assert!(!importado.trae_metadatos_mmcelt);

        let proyecto = importado.proyecto;
        assert_eq!(proyecto.title, "Exportado desde otra aplicación");
        assert_eq!(proyecto.nodes.len(), 4);
        let raiz = proyecto.nodo_raiz().expect("debe reconstruirse una raíz");
        assert_eq!(raiz.title, "Idea central");
        assert_eq!(raiz.children.len(), 2);
        assert_eq!(raiz.status, EstadoNodo::default());
        assert_eq!(raiz.priority, PrioridadNodo::default());
    }

    #[test]
    fn los_caracteres_especiales_de_una_nota_sobreviven_la_ida_y_vuelta() {
        let mut proyecto = Proyecto::nuevo_vacio("Con caracteres raros: <>&\"'");
        let raiz_id = proyecto.root_id;
        proyecto.nodes.get_mut(&raiz_id).unwrap().notes =
            "Texto con <etiquetas>, & comerciales y \"comillas\".".to_string();

        let xml = exportar_a_opml(&proyecto);
        let importado = importar_desde_opml(&xml).unwrap();
        assert_eq!(
            importado.proyecto.nodo_raiz().unwrap().notes,
            "Texto con <etiquetas>, & comerciales y \"comillas\"."
        );
        assert_eq!(importado.proyecto.title, "Con caracteres raros: <>&\"'");
    }

    /// Un valor de atributo XML normaliza cualquier salto de línea literal a un espacio
    /// (XML 1.0, §3.3.3). Como la nota exportada siempre inserta `\n\n` antes de la marca de
    /// metadatos, **todo** nodo con notas de prosa lo sufre, no solo un caso especial: esta
    /// prueba fija el título con saltos propios del usuario para comprobarlo sin depender de
    /// esa marca interna.
    #[test]
    fn una_nota_con_varios_parrafos_del_usuario_conserva_sus_saltos_de_linea() {
        let mut proyecto = Proyecto::nuevo_vacio("Título\ncon salto");
        let raiz_id = proyecto.root_id;
        proyecto.nodes.get_mut(&raiz_id).unwrap().notes =
            "Primer párrafo.\n\nSegundo párrafo, tras una línea en blanco.\nTercera línea."
                .to_string();

        let xml = exportar_a_opml(&proyecto);
        let importado = importar_desde_opml(&xml).unwrap();

        assert_eq!(importado.proyecto.title, "Título\ncon salto");
        assert_eq!(
            importado.proyecto.nodo_raiz().unwrap().notes,
            "Primer párrafo.\n\nSegundo párrafo, tras una línea en blanco.\nTercera línea."
        );
    }
}
