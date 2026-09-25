//! # Exportación e importación al `.mm` de FreeMind/Freeplane (`exportacion_freemind.rs`)
//!
//! El `.mm` es un árbol de `<node TEXT="...">` anidados. A diferencia de OPML, la nota de un
//! nodo **no** es un atributo: vive en un elemento hijo, `<richcontent TYPE="NOTE">` con un
//! fragmento XHTML dentro. Eso simplifica lo que costó caro en `exportacion_opml.rs`: un texto
//! dentro de un elemento **no** sufre la normalización de saltos de línea que exige el estándar
//! para los valores de atributo (XML 1.0, §3.3.3) — los saltos de línea de la nota viajan tal
//! cual, sin referencias de carácter.
//!
//! El resto del diseño es el mismo que OPML y comparte el mismo módulo de dominio: título y
//! jerarquía son nativos del formato; todo lo demás de MMCelt viaja como JSON dentro de la nota,
//! según decide `crate::model::metadatos_externos`.

use crate::model::{
    anadir_metadatos_de_proyecto, construir_nota_de_nodo, separar_nota_importada,
    MetadatosNodoExterno, MetadatosProyectoExterno, Nodo, Proyecto, RolNodo,
};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

/// Causa concreta por la que un documento `.mm` no pudo importarse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorFreemind {
    /// El XML no está bien formado; conserva el mensaje original de `quick-xml`.
    XmlMalformado(String),
    /// El documento no tenía ni un solo `<node>` de nivel superior dentro de `<map>`.
    SinContenido,
}

/// Resultado de importar un archivo externo: el proyecto reconstruido y si el archivo traía
/// metadatos propios de MMCelt.
#[derive(Debug)]
pub struct ProyectoImportadoFreemind {
    /// El mapa reconstruido, con valores por defecto donde no había metadatos de MMCelt.
    pub proyecto: Proyecto,
    /// `false` cuando ningún nodo llevaba el bloque JSON de MMCelt: la señal de C21-D para que
    /// la interfaz avise a la persona de que el archivo no venía de MMCelt.
    pub trae_metadatos_mmcelt: bool,
}

/// Mensaje del `.expect()` interno: escribir en un `Vec<u8>` en memoria no puede fallar por E/S,
/// así que un error aquí solo puede venir de un evento XML mal formado por este mismo módulo.
const FALLO_ESCRITURA: &str = "escribir en un buffer en memoria no debería fallar";

/// Exporta un proyecto completo a un documento `.mm` de FreeMind/Freeplane.
pub fn exportar_a_freemind(proyecto: &Proyecto) -> String {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    let mut mapa = BytesStart::new("map");
    mapa.push_attribute(("version", "1.0.1"));
    writer
        .write_event(Event::Start(mapa))
        .expect(FALLO_ESCRITURA);

    if let Some(raiz) = proyecto.nodo_raiz() {
        escribir_nodo(&mut writer, proyecto, raiz);
    }

    writer
        .write_event(Event::End(BytesEnd::new("map")))
        .expect(FALLO_ESCRITURA);

    String::from_utf8(writer.into_inner()).expect("quick-xml solo escribe UTF-8 válido")
}

/// Escribe recursivamente un nodo, su nota y sus descendientes como `<node>`.
fn escribir_nodo(writer: &mut Writer<Vec<u8>>, proyecto: &Proyecto, nodo: &Nodo) {
    let metadatos_nodo = MetadatosNodoExterno::desde_nodo(nodo);
    let mut nota = construir_nota_de_nodo(&nodo.notes, &metadatos_nodo);
    if nodo.id == proyecto.root_id {
        let metadatos_proyecto = MetadatosProyectoExterno::desde_proyecto(proyecto);
        nota = anadir_metadatos_de_proyecto(&nota, &metadatos_proyecto);
    }

    let mut inicio = BytesStart::new("node");
    inicio.push_attribute(("TEXT", nodo.title.as_str()));
    inicio.push_attribute(("ID", format!("mmcelt_{}", nodo.id).as_str()));
    writer
        .write_event(Event::Start(inicio))
        .expect(FALLO_ESCRITURA);

    escribir_nota(writer, &nota);

    for hijo in nodo.children.iter().filter_map(|id| proyecto.nodes.get(id)) {
        escribir_nodo(writer, proyecto, hijo);
    }

    writer
        .write_event(Event::End(BytesEnd::new("node")))
        .expect(FALLO_ESCRITURA);
}

/// Escribe el `<richcontent TYPE="NOTE">` con el fragmento XHTML que envuelve la nota.
///
/// Un único `<p>` con el texto completo, saltos de línea incluidos: un elemento de texto no
/// normaliza los saltos como lo hace un atributo, así que no hace falta ninguna referencia de
/// carácter aquí (a diferencia de `exportacion_opml.rs`).
fn escribir_nota(writer: &mut Writer<Vec<u8>>, nota: &str) {
    let mut richcontent = BytesStart::new("richcontent");
    richcontent.push_attribute(("TYPE", "NOTE"));
    writer
        .write_event(Event::Start(richcontent))
        .expect(FALLO_ESCRITURA);

    writer
        .write_event(Event::Start(BytesStart::new("html")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::Empty(BytesStart::new("head")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::Start(BytesStart::new("body")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::Start(BytesStart::new("p")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::Text(BytesText::new(nota)))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::End(BytesEnd::new("p")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::End(BytesEnd::new("body")))
        .expect(FALLO_ESCRITURA);
    writer
        .write_event(Event::End(BytesEnd::new("html")))
        .expect(FALLO_ESCRITURA);

    writer
        .write_event(Event::End(BytesEnd::new("richcontent")))
        .expect(FALLO_ESCRITURA);
}

/// Un `<node>` sobre la pila mientras se lee: su [`Nodo`] ya existe —con su `id` real, asignado
/// al llegar el `<node TEXT="...">` de apertura, como en `exportacion_opml`—, pero su nota crudo
/// se sigue acumulando hasta que se cierra el `<richcontent>` que la contiene.
struct NodoEnConstruccion {
    nodo: Nodo,
    nota_cruda: String,
}

/// Importa un documento `.mm`, sea o no de MMCelt.
///
/// Reconstruye la jerarquía desde el anidamiento real de los `<node>`. Si el `<richcontent
/// TYPE="NOTE">` de un nodo lleva el bloque JSON de MMCelt, se restablecen su estado, prioridad,
/// rol, revisión, comentario, ruta, etiquetas, posición y plegado exactos (C21-C). Si no lo
/// lleva, el nodo se crea con los valores por defecto de [`Nodo::new`] (C21-D): la persona que
/// llama decide si avisar, mirando `trae_metadatos_mmcelt`.
pub fn importar_desde_freemind(xml: &str) -> Result<ProyectoImportadoFreemind, ErrorFreemind> {
    // Sin `trim_text`: el texto de la nota se corta en varios `Event::Text` en cada frontera de
    // `&entidad;`, y recortar los extremos de cada fragmento por separado se comería los espacios
    // junto a cualquier carácter escapado. La nota es el único texto que se captura (todo lo
    // demás vive en atributos), así que no hace falta recortar nada.
    let mut reader = Reader::from_str(xml);

    let mut pila: Vec<NodoEnConstruccion> = Vec::new();
    let mut raiz: Option<Nodo> = None;
    let mut nodos_planos: Vec<Nodo> = Vec::new();
    let mut trae_metadatos_mmcelt = false;
    let mut metadatos_proyecto: Option<MetadatosProyectoExterno> = None;
    let mut dentro_de_richcontent_de_nota = false;
    let mut dentro_de_nota = false;
    // Id provisional (asignado al abrir el `<node>`) -> id definitivo de sus metadatos.
    let mut remapeo_de_ids: std::collections::HashMap<uuid::Uuid, uuid::Uuid> =
        std::collections::HashMap::new();

    loop {
        let evento = reader
            .read_event()
            .map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
        match evento {
            Event::Eof => break,
            Event::Start(inicio) if inicio.name().as_ref() == b"node" => {
                let titulo = leer_atributo_texto(&inicio, &mut reader, b"TEXT")?;
                let mut nodo = Nodo::new(titulo, None, [0.0, 0.0]);
                // `Nodo::new` decide el rol a partir del `parent_id`, y aquí siempre se le pasa
                // `None` porque el padre real todavía no se conoce (igual que en
                // `exportacion_opml::leer_nodo_desde_atributos`). Sin este reseteo, todo nodo sin
                // metadatos propios —incluidos los hijos reales— heredaría `IdeaCentral`, el rol
                // que `Nodo::new` reserva a la raíz.
                nodo.role = RolNodo::default();
                pila.push(NodoEnConstruccion {
                    nodo,
                    nota_cruda: String::new(),
                });
            }
            Event::Empty(inicio) if inicio.name().as_ref() == b"node" => {
                let titulo = leer_atributo_texto(&inicio, &mut reader, b"TEXT")?;
                let mut nodo = Nodo::new(titulo, None, [0.0, 0.0]);
                nodo.role = RolNodo::default();
                anadir_como_hijo_o_raiz(nodo, &mut pila, &mut raiz, &mut nodos_planos);
            }
            Event::Start(inicio) if inicio.name().as_ref() == b"richcontent" => {
                dentro_de_richcontent_de_nota = es_richcontent_de_nota(&inicio, &mut reader)?;
            }
            Event::End(fin) if fin.name().as_ref() == b"richcontent" => {
                dentro_de_richcontent_de_nota = false;
            }
            // Se captura solo el `<p>`, no todo el `<richcontent>`: entre las etiquetas
            // estructurales (`<html>`, `<head/>`, `<body>`) el escritor con sangrado inserta
            // espacios en blanco como texto, y capturarlos habría antepuesto esa indentación a
            // la nota real.
            Event::Start(inicio)
                if dentro_de_richcontent_de_nota && inicio.name().as_ref() == b"p" =>
            {
                dentro_de_nota = true;
            }
            Event::End(fin) if fin.name().as_ref() == b"p" => {
                dentro_de_nota = false;
            }
            Event::Text(texto) if dentro_de_nota => {
                if let Some(actual) = pila.last_mut() {
                    let decodificado = texto
                        .decode()
                        .map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
                    actual.nota_cruda.push_str(&decodificado);
                }
            }
            // `quick-xml` corta el texto en cada `&entidad;` y entrega la referencia como un
            // evento aparte en vez de dejarla dentro de `Event::Text`: sin este brazo, cada
            // carácter escapado (`&lt;`, `&amp;`, las comillas del JSON…) desaparecía sin más.
            Event::GeneralRef(referencia) if dentro_de_nota => {
                if let Some(actual) = pila.last_mut() {
                    if let Some(caracter) = referencia
                        .resolve_char_ref()
                        .map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?
                    {
                        actual.nota_cruda.push(caracter);
                    } else {
                        let nombre = referencia
                            .decode()
                            .map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
                        let caracter = match nombre.as_ref() {
                            "amp" => '&',
                            "lt" => '<',
                            "gt" => '>',
                            "quot" => '"',
                            "apos" => '\'',
                            otra => {
                                return Err(ErrorFreemind::XmlMalformado(format!(
                                    "entidad no reconocida en la nota: &{otra};"
                                )))
                            }
                        };
                        actual.nota_cruda.push(caracter);
                    }
                }
            }
            Event::End(fin) if fin.name().as_ref() == b"node" => {
                if let Some(NodoEnConstruccion {
                    mut nodo,
                    nota_cruda,
                }) = pila.pop()
                {
                    let separada = separar_nota_importada(&nota_cruda);
                    trae_metadatos_mmcelt |= separada.metadatos_nodo.is_some();
                    // El `id` definitivo de los metadatos **no** se aplica aquí: los hijos de
                    // este nodo ya se cerraron y enlazaron con el `id` provisional que tenía al
                    // entrar en la pila (los metadatos, en un `<richcontent>` hijo, llegan
                    // después del `<node>` de apertura, al revés que en OPML). Cambiarlo ahora
                    // dejaría a esos hijos apuntando a un padre que ya no existe. El remapeo real
                    // ocurre en una pasada aparte, después de cerrar todo el árbol.
                    if let Some(metadatos) = &separada.metadatos_nodo {
                        remapeo_de_ids.insert(nodo.id, metadatos.id);
                    }
                    nodo.notes = separada.notas_usuario;
                    if let Some(metadatos) = separada.metadatos_nodo {
                        metadatos.aplicar_a(&mut nodo);
                    }
                    if separada.metadatos_proyecto.is_some() {
                        metadatos_proyecto = separada.metadatos_proyecto;
                    }
                    anadir_como_hijo_o_raiz(nodo, &mut pila, &mut raiz, &mut nodos_planos);
                }
            }
            _ => {}
        }
    }

    let mut raiz = raiz.ok_or(ErrorFreemind::SinContenido)?;
    if raiz.role == RolNodo::default() {
        // Ningún metadato de MMCelt fijó el rol de la raíz: el papel semántico de la raíz de un
        // mapa mental es `IdeaCentral`, igual que cuando se crea un proyecto nuevo. Si la raíz sí
        // traía metadatos con un rol explícito, esta rama no se ejecuta y no se pisa nada.
        raiz.role = RolNodo::IdeaCentral;
    }
    remapear_identidad(&mut raiz, &remapeo_de_ids);
    for nodo in &mut nodos_planos {
        remapear_identidad(nodo, &remapeo_de_ids);
    }

    let mut proyecto = Proyecto::nuevo_vacio(raiz.title.clone());
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
    }

    Ok(ProyectoImportadoFreemind {
        proyecto,
        trae_metadatos_mmcelt,
    })
}

/// Cuelga un nodo ya cerrado del padre en la cima de la pila (si lo hay) o lo marca como raíz.
///
/// Igual que `exportacion_opml::anadir_como_hijo_o_raiz`: un segundo `<node>` de nivel superior
/// dentro de `<map>` —que FreeMind no produce nunca, pero el formato no lo prohíbe— se descarta
/// en silencio, porque MMCelt no tiene sitio para un segundo árbol raíz.
fn anadir_como_hijo_o_raiz(
    nodo: Nodo,
    pila: &mut [NodoEnConstruccion],
    raiz: &mut Option<Nodo>,
    nodos_planos: &mut Vec<Nodo>,
) {
    if let Some(padre) = pila.last_mut() {
        let mut nodo = nodo;
        nodo.parent_id = Some(padre.nodo.id);
        padre.nodo.children.push(nodo.id);
        nodos_planos.push(nodo);
    } else if raiz.is_none() {
        *raiz = Some(nodo);
    }
}

/// Sustituye el `id` provisional de un nodo, su `parent_id` y cada entrada de `children` por su
/// equivalente definitivo, si lo tenían en `remapeo`. Un nodo sin metadatos no tiene entrada en
/// el mapa y se deja tal cual.
fn remapear_identidad(
    nodo: &mut Nodo,
    remapeo: &std::collections::HashMap<uuid::Uuid, uuid::Uuid>,
) {
    if let Some(&definitivo) = remapeo.get(&nodo.id) {
        nodo.id = definitivo;
    }
    if let Some(padre) = nodo.parent_id {
        nodo.parent_id = Some(*remapeo.get(&padre).unwrap_or(&padre));
    }
    for hijo in &mut nodo.children {
        *hijo = *remapeo.get(hijo).unwrap_or(hijo);
    }
}

/// Lee el atributo `TYPE` de un `<richcontent>` y dice si es la nota (`TYPE="NOTE"`).
fn es_richcontent_de_nota(
    inicio: &BytesStart,
    reader: &mut Reader<&[u8]>,
) -> Result<bool, ErrorFreemind> {
    if inicio.name().as_ref() != b"richcontent" {
        return Ok(false);
    }
    for atributo in inicio.attributes() {
        let atributo = atributo.map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
        if atributo.key.as_ref() == b"TYPE" {
            let valor = atributo
                .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
                .map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
            return Ok(valor.as_ref() == "NOTE");
        }
    }
    Ok(false)
}

/// Lee un atributo de texto de un `<node>`, ya normalizado según el estándar de XML.
fn leer_atributo_texto(
    inicio: &BytesStart,
    reader: &mut Reader<&[u8]>,
    nombre: &[u8],
) -> Result<String, ErrorFreemind> {
    for atributo in inicio.attributes() {
        let atributo = atributo.map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
        if atributo.key.as_ref() == nombre {
            let valor = atributo
                .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
                .map_err(|e| ErrorFreemind::XmlMalformado(e.to_string()))?;
            return Ok(valor.into_owned());
        }
    }
    Ok(String::new())
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
            raiz.notes = "Nota de la raíz,\ncon salto de línea propio.".to_string();
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
    fn exportar_un_mapa_vacio_produce_un_nodo_unico() {
        let proyecto = Proyecto::nuevo_vacio("Proyecto vacío");
        let xml = exportar_a_freemind(&proyecto);
        assert!(xml.contains("<map version=\"1.0.1\">"));
        assert!(xml.contains("<node"));
    }

    #[test]
    fn importar_un_xml_malformado_devuelve_el_error_tipado() {
        let resultado = importar_desde_freemind("<map><node TEXT=\"sin cerrar\">");
        assert!(matches!(
            resultado,
            Err(ErrorFreemind::XmlMalformado(_)) | Err(ErrorFreemind::SinContenido)
        ));
    }

    #[test]
    fn importar_un_documento_sin_ningun_node_devuelve_sin_contenido() {
        let resultado =
            importar_desde_freemind(r#"<?xml version="1.0"?><map version="1.0.1"></map>"#);
        assert_eq!(resultado.unwrap_err(), ErrorFreemind::SinContenido);
    }

    /// **C21-C** — exportar un mapa de MMCelt y reimportarlo reconstruye estado, prioridad,
    /// rol, revisión, comentario, etiquetas, ruta, posición, plegado y conexiones cruzadas
    /// idénticos. Ida y vuelta exacta, campo a campo.
    #[test]
    fn ida_y_vuelta_de_un_mapa_completo_es_exacta() {
        let original = mapa_de_ejemplo();
        let xml = exportar_a_freemind(&original);

        let importado = importar_desde_freemind(&xml).expect("el XML propio siempre es válido");
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

    /// **C21-D** — un `.mm` ajeno, sin ningún bloque de MMCelt, se importa con valores por
    /// defecto y la señal `trae_metadatos_mmcelt` en `false`.
    #[test]
    fn importar_un_mm_ajeno_usa_valores_por_defecto_y_no_senala_metadatos() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<map version="1.0.1">
  <node TEXT="Idea central" ID="ID_1">
    <node TEXT="Rama uno" ID="ID_2"/>
    <node TEXT="Rama dos" ID="ID_3">
      <node TEXT="Nieta" ID="ID_4"/>
    </node>
  </node>
</map>"#;

        let importado = importar_desde_freemind(xml).expect("es un .mm válido, aunque ajeno");
        assert!(!importado.trae_metadatos_mmcelt);

        let proyecto = importado.proyecto;
        assert_eq!(proyecto.title, "Idea central");
        assert_eq!(proyecto.nodes.len(), 4);
        let raiz = proyecto.nodo_raiz().expect("debe reconstruirse una raíz");
        assert_eq!(raiz.title, "Idea central");
        assert_eq!(raiz.children.len(), 2);
        assert_eq!(raiz.status, EstadoNodo::default());
        assert_eq!(raiz.priority, PrioridadNodo::default());
        assert_eq!(raiz.role, RolNodo::IdeaCentral);
        for hijo_id in &raiz.children {
            let hijo = proyecto.nodes.get(hijo_id).unwrap();
            assert_eq!(
                hijo.role,
                RolNodo::default(),
                "un hijo ajeno sin metadatos debe tener el rol por defecto, no IdeaCentral"
            );
        }
    }

    #[test]
    fn los_caracteres_especiales_de_una_nota_sobreviven_la_ida_y_vuelta() {
        let mut proyecto = Proyecto::nuevo_vacio("Con caracteres raros: <>&\"'");
        let raiz_id = proyecto.root_id;
        proyecto.nodes.get_mut(&raiz_id).unwrap().notes =
            "Texto con <etiquetas>, & comerciales y \"comillas\".".to_string();

        let xml = exportar_a_freemind(&proyecto);
        let importado = importar_desde_freemind(&xml).unwrap();
        assert_eq!(
            importado.proyecto.nodo_raiz().unwrap().notes,
            "Texto con <etiquetas>, & comerciales y \"comillas\"."
        );
    }

    /// El motivo de todo el diseño de este módulo, frente al de OPML: aquí no hace falta ninguna
    /// referencia de carácter para que sobrevivan los saltos de línea, porque la nota vive en un
    /// elemento, no en un atributo.
    #[test]
    fn una_nota_con_varios_parrafos_del_usuario_conserva_sus_saltos_de_linea() {
        let mut proyecto = Proyecto::nuevo_vacio("Proyecto de prueba");
        let raiz_id = proyecto.root_id;
        proyecto.nodes.get_mut(&raiz_id).unwrap().notes =
            "Primer párrafo.\n\nSegundo párrafo, tras una línea en blanco.\nTercera línea."
                .to_string();

        let xml = exportar_a_freemind(&proyecto);
        let importado = importar_desde_freemind(&xml).unwrap();

        assert_eq!(
            importado.proyecto.nodo_raiz().unwrap().notes,
            "Primer párrafo.\n\nSegundo párrafo, tras una línea en blanco.\nTercera línea."
        );
    }
}
