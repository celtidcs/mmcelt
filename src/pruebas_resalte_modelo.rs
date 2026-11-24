//! Resalte de lo que ha hecho la IA, parte del modelo: el estado «Sin marcar» de lo que crea la
//! persona, la marca de nodo nuevo de la IA y cómo se quita al abrir el nodo.

use crate::model::{EstadoRevision, Nodo, PrioridadNodo, Proyecto};

/// Un mapa con un hijo de la raíz marcado como nuevo de la IA, puesto en una aplicación de
/// prueba. Devuelve la aplicación y el id de ese hijo.
fn aplicacion_con_un_nodo_nuevo_de_la_ia() -> (crate::aplicacion::AplicacionMapaMental, uuid::Uuid)
{
    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    let mut proyecto = Proyecto::nuevo_vacio("Mapa");
    let hijo = proyecto.anadir_hijo(proyecto.root_id, "Propuesta de la IA");
    let nodo = proyecto.nodes.get_mut(&hijo).expect("el hijo existe");
    nodo.review_status = EstadoRevision::GeneradoPorIA;
    nodo.nuevo_de_la_ia = true;
    app.mapa.proyecto = proyecto;
    (app, hijo)
}

/// Lo que crea la persona nace sin marca de revisión y sin resalte, tanto con el constructor
/// del nodo como con las operaciones del mapa que usan la interfaz, las plantillas y los
/// ejemplos.
#[test]
fn lo_que_crea_la_persona_nace_sin_marcar() {
    let suelto = Nodo::new("Idea", None, [0.0, 0.0]);
    assert_eq!(suelto.review_status, EstadoRevision::SinMarcar);
    assert!(!suelto.nuevo_de_la_ia);

    let mut proyecto = Proyecto::nuevo_vacio("Mapa");
    let hijo = proyecto.anadir_hijo(proyecto.root_id, "Hijo");
    let hermano = proyecto.anadir_hermano(hijo, "Hermano");
    for id in [Some(proyecto.root_id), Some(hijo), hermano] {
        let nodo = &proyecto.nodes[&id.expect("el nodo existe")];
        assert_eq!(
            nodo.review_status,
            EstadoRevision::SinMarcar,
            "«{}»",
            nodo.title
        );
        assert!(!nodo.nuevo_de_la_ia, "«{}»", nodo.title);
    }
}

/// Los dos mapas de ejemplo salen enteros sin marcar: se pueden usar sin que nada aparezca
/// pendiente de revisar.
#[test]
fn los_mapas_de_ejemplo_salen_sin_marcar() {
    for (_, ejemplo) in super::ejemplos() {
        for nodo in ejemplo.nodes.values() {
            assert_eq!(
                nodo.review_status,
                EstadoRevision::SinMarcar,
                "«{}»",
                nodo.title
            );
            assert!(!nodo.nuevo_de_la_ia, "«{}»", nodo.title);
        }
    }
}

/// Un nodo sin marcar no dibuja ningún indicador de revisión; los demás estados sí.
#[test]
fn sin_marcar_no_dibuja_indicador_de_revision() {
    for revision in EstadoRevision::TODOS {
        let indicadores =
            crate::ui::canvas::calcular_indicadores_del_nodo(PrioridadNodo::Media, revision);
        let dibuja_revision = indicadores.iter().any(|indicador| !indicador.es_prioridad);
        assert_eq!(
            dibuja_revision,
            revision != EstadoRevision::SinMarcar,
            "{revision:?}"
        );
    }
}

/// «Sin marcar» se escribe y se lee con su nombre, pero un estado que no se entiende sigue
/// leyéndose como pendiente de revisión: nunca se toma por trabajo de la persona.
#[test]
fn un_estado_desconocido_no_se_lee_como_trabajo_de_la_persona() {
    let texto: String = EstadoRevision::SinMarcar.into();
    assert_eq!(texto, "SinMarcar");
    assert_eq!(EstadoRevision::from(texto), EstadoRevision::SinMarcar);
    assert_eq!(
        EstadoRevision::desde_texto("estado_inventado"),
        EstadoRevision::PendienteRevision
    );
}

/// La marca de nodo nuevo solo se escribe en el archivo cuando vale `true`; un archivo que no
/// la trae la lee como `false`, y la que vale `true` sobrevive al guardado.
#[test]
fn la_marca_de_nuevo_solo_se_guarda_cuando_esta_puesta() {
    let mut nodo = Nodo::new("Idea", None, [0.0, 0.0]);
    let sin_marca = serde_json::to_string(&nodo).expect("serializa");
    assert!(!sin_marca.contains("nuevo_de_la_ia"), "{sin_marca}");
    let leido: Nodo = serde_json::from_str(&sin_marca).expect("lee un nodo sin la marca");
    assert!(!leido.nuevo_de_la_ia);

    nodo.nuevo_de_la_ia = true;
    let con_marca = serde_json::to_string(&nodo).expect("serializa");
    assert!(con_marca.contains("\"nuevo_de_la_ia\":true"), "{con_marca}");
    let leido: Nodo = serde_json::from_str(&con_marca).expect("lee un nodo con la marca");
    assert!(leido.nuevo_de_la_ia);
}

/// Al seleccionar el nodo, la persona lo ha abierto: deja de resaltarse, sigue siendo de la IA
/// y el mapa no cuenta con un cambio nuevo, para que la IA pueda seguir recargándolo.
#[test]
fn seleccionar_un_nodo_nuevo_le_quita_el_resalte_sin_modificar_el_mapa() {
    let (mut app, hijo) = aplicacion_con_un_nodo_nuevo_de_la_ia();
    let revision = app.mapa.proyecto.revision();

    app.mapa.seleccionar_nodo(Some(hijo));

    let nodo = &app.mapa.proyecto.nodes[&hijo];
    assert!(!nodo.nuevo_de_la_ia);
    assert_eq!(nodo.review_status, EstadoRevision::GeneradoPorIA);
    assert_eq!(
        app.mapa.proyecto.revision(),
        revision,
        "no es un cambio del mapa"
    );
}

/// Abrir el mapa no cuenta como abrir el nodo: la selección automática de la raíz no quita el
/// resalte de una raíz nueva de la IA.
#[test]
fn abrir_un_mapa_de_la_ia_no_quita_el_resalte_de_su_raiz() {
    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    let mut proyecto = Proyecto::nuevo_vacio("Mapa de la IA");
    let raiz = proyecto.root_id;
    proyecto
        .nodes
        .get_mut(&raiz)
        .expect("la raíz existe")
        .nuevo_de_la_ia = true;

    app.sustituir_el_mapa_abierto(proyecto, None, "");

    assert_eq!(app.mapa.nodo_seleccionado(), Some(raiz));
    assert!(app.mapa.proyecto.nodes[&raiz].nuevo_de_la_ia);
}

/// Cuando la IA vuelve a escribir el archivo y se recarga, los nodos que la persona ya abrió no
/// vuelven a resaltarse, y los que no ha abierto siguen resaltados.
#[test]
fn una_recarga_del_archivo_no_devuelve_el_resalte_de_lo_ya_abierto() {
    let (mut app, hijo) = aplicacion_con_un_nodo_nuevo_de_la_ia();
    app.mapa.seleccionar_nodo(Some(hijo));

    let mut recargado = app.mapa.proyecto.clone();
    recargado
        .nodes
        .get_mut(&hijo)
        .expect("el hijo existe")
        .nuevo_de_la_ia = true;
    let otro = recargado.anadir_hijo(recargado.root_id, "Otra propuesta");
    recargado
        .nodes
        .get_mut(&otro)
        .expect("el nuevo existe")
        .nuevo_de_la_ia = true;
    app.presentar_accion_de_vigilancia(crate::vigilancia_mapa::AccionCargaExterna::Sustituir(
        Box::new(recargado),
    ));

    assert!(
        !app.mapa.proyecto.nodes[&hijo].nuevo_de_la_ia,
        "ya lo había abierto"
    );
    assert!(
        app.mapa.proyecto.nodes[&otro].nuevo_de_la_ia,
        "todavía no lo ha abierto"
    );
}

/// Deshacer vuelve a una foto del mapa tomada antes de abrir el nodo, pero no le devuelve el
/// resalte.
#[test]
fn deshacer_no_devuelve_el_resalte_de_lo_ya_abierto() {
    let (mut app, hijo) = aplicacion_con_un_nodo_nuevo_de_la_ia();
    let observar = |app: &mut crate::aplicacion::AplicacionMapaMental| {
        app.mapa.historial.observar_si_cambio(
            &app.mapa.proyecto,
            app.mapa.proyecto.revision(),
            std::time::Instant::now(),
        );
    };
    // La foto de partida del historial es el mapa con el nodo todavía resaltado.
    app.mapa.historial.olvidar(&app.mapa.proyecto);
    app.mapa.seleccionar_nodo(Some(hijo));
    let raiz = app.mapa.proyecto.root_id;
    app.mapa
        .proyecto_para_editar()
        .anadir_hijo(raiz, "Cambio de la persona");
    observar(&mut app);

    app.deshacer();

    assert_eq!(
        app.mapa.proyecto.nodes.len(),
        2,
        "el deshacer se ha aplicado"
    );
    assert!(!app.mapa.proyecto.nodes[&hijo].nuevo_de_la_ia);
}

/// Lo visto en un mapa no se arrastra a otro distinto que se abra después.
#[test]
fn abrir_otro_mapa_olvida_lo_visto_en_el_anterior() {
    let (mut app, hijo) = aplicacion_con_un_nodo_nuevo_de_la_ia();
    app.mapa.seleccionar_nodo(Some(hijo));
    assert!(app.mapa.vistos.contains(&hijo));

    app.sustituir_el_mapa_abierto(Proyecto::nuevo_vacio("Otro"), None, "");

    assert!(app.mapa.vistos.is_empty());
}

/// La tarjeta de un nodo nuevo de la IA lleva fondo y borde propios en los tres temas, también
/// si es la raíz; la selección y el cursor siguen mandando sobre el borde, y un nodo corriente
/// no cambia.
#[test]
fn la_tarjeta_de_un_nodo_nuevo_de_la_ia_va_resaltada() {
    use crate::ui::canvas::{colores_de_la_tarjeta, EstadoVisualDeLaTarjeta};

    let reposo = EstadoVisualDeLaTarjeta {
        es_la_raiz: false,
        esta_seleccionado: false,
        cursor_encima: false,
        nuevo_de_la_ia: false,
    };
    let nuevo = EstadoVisualDeLaTarjeta {
        nuevo_de_la_ia: true,
        ..reposo
    };
    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    for modo in crate::theme::AppThemeMode::TODOS {
        app.presentacion.tema = crate::theme::ThemeConfig::new(modo);
        let tema = app.presentacion.tema.clone();
        let presentacion = app.presentacion();
        assert_eq!(
            colores_de_la_tarjeta(presentacion, reposo),
            (tema.fondo_del_nodo, tema.borde_del_nodo),
            "{modo:?}"
        );
        assert_eq!(
            colores_de_la_tarjeta(presentacion, nuevo),
            (
                tema.fondo_del_nodo_nuevo_de_la_ia,
                tema.borde_del_nodo_nuevo_de_la_ia
            ),
            "{modo:?}"
        );
        let raiz_nueva = EstadoVisualDeLaTarjeta {
            es_la_raiz: true,
            ..nuevo
        };
        assert_eq!(
            colores_de_la_tarjeta(presentacion, raiz_nueva).0,
            tema.fondo_del_nodo_nuevo_de_la_ia,
            "{modo:?}"
        );
        let seleccionado = EstadoVisualDeLaTarjeta {
            esta_seleccionado: true,
            ..nuevo
        };
        assert_eq!(
            colores_de_la_tarjeta(presentacion, seleccionado).1,
            tema.borde_del_nodo_seleccionado,
            "{modo:?}"
        );
    }
}

/// El reloj de «Pendiente de revisión» va en color de aviso para que destaque; el resto de
/// estados conserva su color.
#[test]
fn el_indicador_de_pendiente_va_en_color_de_aviso() {
    use crate::ui::canvas::color_del_indicador_de_revision;

    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    for modo in crate::theme::AppThemeMode::TODOS {
        app.presentacion.tema = crate::theme::ThemeConfig::new(modo);
        let tema = app.presentacion.tema.clone();
        for revision in EstadoRevision::TODOS {
            let esperado = match revision {
                EstadoRevision::PendienteRevision => tema.aviso,
                EstadoRevision::GeneradoPorIA => tema.acento,
                EstadoRevision::AprobadoPorHumano => tema.exito,
                EstadoRevision::RequiereCorreccion => tema.peligro,
                EstadoRevision::SinMarcar => tema.texto_secundario,
            };
            assert_eq!(
                color_del_indicador_de_revision(app.presentacion(), revision),
                esperado,
                "{modo:?} / {revision:?}"
            );
        }
    }
}
