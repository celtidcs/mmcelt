//! Pregunta al cerrar con cambios sin guardar (0.15.0): cuándo se pregunta, qué hace cada
//! botón y cómo se anula y se confirma el cierre de la ventana.

use crate::cierre_con_cambios::{
    hay_que_preguntar_al_cerrar, resolver_respuesta_al_cerrar, CambiosPendientes,
    DesenlaceDelCierre, RespuestaAlCerrar,
};
use crate::model::{EstadoRevision, Proyecto};
use crate::ui::dialogo_de_cierre::{
    atender_peticion_de_cierre, cambios_pendientes_al_cerrar, responder_al_cierre,
};

/// Ejecuta un fotograma en el que el sistema pide cerrar la ventana, o no, y devuelve las
/// órdenes que la aplicación ha mandado a la ventana principal.
fn fotograma_de_cierre(
    app: &mut crate::aplicacion::AplicacionMapaMental,
    ctx: &egui::Context,
    piden_cerrar: bool,
) -> Vec<egui::ViewportCommand> {
    let mut entrada = egui::RawInput::default();
    if piden_cerrar {
        let mut info = egui::ViewportInfo::default();
        info.events.push(egui::ViewportEvent::Close);
        entrada.viewports.insert(egui::ViewportId::ROOT, info);
    }
    let mut salida = ctx.run_ui(entrada, |ui| atender_peticion_de_cierre(app, ui.ctx()));
    salida.textures_delta.clear();
    salida
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .map(|vista| vista.commands.clone())
        .unwrap_or_default()
}

/// Una aplicación con un mapa ya guardado en el que un hijo es nuevo de la IA.
fn aplicacion_guardada_con_un_nodo_nuevo_de_la_ia() -> (
    crate::aplicacion::AplicacionMapaMental,
    egui::Context,
    uuid::Uuid,
) {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let mut proyecto = Proyecto::nuevo_vacio("Mapa");
    let hijo = proyecto.anadir_hijo(proyecto.root_id, "Propuesta de la IA");
    let nodo = proyecto.nodes.get_mut(&hijo).expect("el hijo existe");
    nodo.review_status = EstadoRevision::GeneradoPorIA;
    nodo.nuevo_de_la_ia = true;
    app.mapa.proyecto = proyecto;
    app.anotar_guardado(std::path::Path::new("mapa-que-no-se-escribe.mmcelt"));
    (app, ctx, hijo)
}

/// C1: basta cualquiera de los tres cambios pendientes para preguntar, y sin ninguno no se
/// pregunta.
#[test]
fn se_pregunta_al_cerrar_si_queda_cualquier_cambio_sin_guardar() {
    assert!(!hay_que_preguntar_al_cerrar(CambiosPendientes::default()));
    let solo_uno = [
        CambiosPendientes {
            hay_edicion_activa: true,
            ..Default::default()
        },
        CambiosPendientes {
            mapa_cambiado: true,
            ..Default::default()
        },
        CambiosPendientes {
            vistos_sin_guardar: true,
            ..Default::default()
        },
    ];
    for cambios in solo_uno {
        assert!(hay_que_preguntar_al_cerrar(cambios), "{cambios:?}");
    }
}

/// C4: Cancelar y No guardar no guardan; Guardar cierra solo si el guardado sale bien.
#[test]
fn cada_boton_del_cierre_decide_si_la_aplicacion_se_cierra() {
    let mut llamadas = 0;
    let mut guardar_que_no_debe_llamarse = || {
        llamadas += 1;
        true
    };
    assert_eq!(
        resolver_respuesta_al_cerrar(
            RespuestaAlCerrar::Cancelar,
            &mut guardar_que_no_debe_llamarse
        ),
        DesenlaceDelCierre::SeguirAbierta
    );
    assert_eq!(
        resolver_respuesta_al_cerrar(
            RespuestaAlCerrar::NoGuardar,
            &mut guardar_que_no_debe_llamarse
        ),
        DesenlaceDelCierre::Cerrar
    );
    assert_eq!(llamadas, 0, "Cancelar y No guardar no guardan");

    assert_eq!(
        resolver_respuesta_al_cerrar(RespuestaAlCerrar::Guardar, || true),
        DesenlaceDelCierre::Cerrar
    );
    assert_eq!(
        resolver_respuesta_al_cerrar(RespuestaAlCerrar::Guardar, || false),
        DesenlaceDelCierre::SeguirAbierta,
        "si se cancela «Guardar como» o el guardado falla, la aplicación sigue abierta"
    );
}

/// C2: el ejemplo inicial, sin tocar, se cierra sin preguntar; en cuanto la persona lo cambia,
/// sí se pregunta.
#[test]
fn el_ejemplo_inicial_sin_tocar_se_cierra_sin_preguntar() {
    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    assert!(!hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(
        &app
    )));

    let raiz = app.mapa.proyecto.root_id;
    app.mapa
        .proyecto_para_editar()
        .anadir_hijo(raiz, "Idea nueva");

    assert!(cambios_pendientes_al_cerrar(&app).mapa_cambiado);
    assert!(hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(
        &app
    )));
}

/// C3: abrir un nodo nuevo de la IA deja un cambio para el cierre sin tocar el historial; guardar
/// o abrir otro mapa lo olvida.
#[test]
fn abrir_un_nodo_nuevo_de_la_ia_cuenta_solo_para_el_cierre() {
    let (mut app, _ctx, hijo) = aplicacion_guardada_con_un_nodo_nuevo_de_la_ia();
    app.mapa.historial.olvidar(&app.mapa.proyecto);
    assert!(!hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(
        &app
    )));

    app.mapa.seleccionar_nodo(Some(hijo));

    let cambios = cambios_pendientes_al_cerrar(&app);
    assert!(cambios.vistos_sin_guardar);
    assert!(!cambios.mapa_cambiado, "no es un cambio del mapa");
    assert!(!app.mapa.puede_deshacer(), "no es un paso de deshacer");
    assert!(
        !app.hay_cambios_locales_sin_guardar(),
        "no retiene las recargas del archivo"
    );

    // Volver a seleccionarlo no vuelve a contar: ya estaba visto.
    app.anotar_guardado(std::path::Path::new("mapa-que-no-se-escribe.mmcelt"));
    app.mapa.seleccionar_nodo(Some(hijo));
    assert!(!cambios_pendientes_al_cerrar(&app).vistos_sin_guardar);

    let (mut otra, _ctx, hijo) = aplicacion_guardada_con_un_nodo_nuevo_de_la_ia();
    otra.mapa.seleccionar_nodo(Some(hijo));
    otra.sustituir_el_mapa_abierto(Proyecto::nuevo_vacio("Otro"), None, "");
    assert!(!cambios_pendientes_al_cerrar(&otra).vistos_sin_guardar);
}

/// C3: si una recarga devuelve el resalte a un nodo ya visto y se le vuelve a quitar, eso
/// tampoco está en disco.
#[test]
fn quitar_otra_vez_el_resalte_tras_una_recarga_cuenta_para_el_cierre() {
    let (mut app, _ctx, hijo) = aplicacion_guardada_con_un_nodo_nuevo_de_la_ia();
    app.mapa.seleccionar_nodo(Some(hijo));
    app.anotar_guardado(std::path::Path::new("mapa-que-no-se-escribe.mmcelt"));
    assert!(!cambios_pendientes_al_cerrar(&app).vistos_sin_guardar);

    app.mapa
        .proyecto_para_editar()
        .nodes
        .get_mut(&hijo)
        .expect("el hijo existe")
        .nuevo_de_la_ia = true;
    app.mapa.reaplicar_los_vistos();

    assert!(!app.mapa.proyecto.nodes[&hijo].nuevo_de_la_ia);
    assert!(cambios_pendientes_al_cerrar(&app).vistos_sin_guardar);
}

/// C5: con cambios sin guardar, la petición de cierre se anula y se abre la pregunta; tras «No
/// guardar» se cierra y la siguiente petición ya no pregunta.
#[test]
fn cerrar_con_cambios_anula_el_cierre_hasta_que_se_responde() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = app.mapa.proyecto.root_id;
    app.mapa
        .proyecto_para_editar()
        .anadir_hijo(raiz, "Idea nueva");

    let sin_peticion = fotograma_de_cierre(&mut app, &ctx, false);
    assert!(!sin_peticion.contains(&egui::ViewportCommand::CancelClose));
    assert!(!app.presentacion.pregunta_al_cerrar);

    let ordenes = fotograma_de_cierre(&mut app, &ctx, true);
    assert!(
        ordenes.contains(&egui::ViewportCommand::CancelClose),
        "{ordenes:?}"
    );
    assert!(app.presentacion.pregunta_al_cerrar);

    responder_al_cierre(&mut app, &ctx, RespuestaAlCerrar::Cancelar);
    assert!(!app.presentacion.pregunta_al_cerrar);
    assert!(!app.presentacion.cierre_confirmado, "Cancelar no cierra");

    responder_al_cierre(&mut app, &ctx, RespuestaAlCerrar::NoGuardar);
    assert!(app.presentacion.cierre_confirmado);
    let ordenes = fotograma_de_cierre(&mut app, &ctx, true);
    assert!(
        !ordenes.contains(&egui::ViewportCommand::CancelClose),
        "tras responder, el cierre sigue adelante: {ordenes:?}"
    );
}

/// C5: «No guardar» manda cerrar la ventana.
#[test]
fn no_guardar_manda_cerrar_la_ventana() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let mut salida = ctx.run_ui(egui::RawInput::default(), |ui| {
        responder_al_cierre(&mut app, ui.ctx(), RespuestaAlCerrar::NoGuardar);
    });
    salida.textures_delta.clear();
    let ordenes = &salida.viewport_output[&egui::ViewportId::ROOT].commands;
    assert!(
        ordenes.contains(&egui::ViewportCommand::Close),
        "{ordenes:?}"
    );
}

/// C5: sin cambios, el cierre sigue adelante sin preguntar.
#[test]
fn cerrar_sin_cambios_no_pregunta() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let ordenes = fotograma_de_cierre(&mut app, &ctx, true);
    assert!(!ordenes.contains(&egui::ViewportCommand::CancelClose));
    assert!(!app.presentacion.pregunta_al_cerrar);
}

/// C6: la ventana pinta el título, la explicación y los tres botones en los seis idiomas, y
/// ningún texto se repite entre sí.
#[test]
fn la_ventana_del_cierre_esta_en_los_seis_idiomas() {
    use crate::textos::{Idioma, Texto};
    let claves = [
        Texto::CierreConCambiosTitulo,
        Texto::CierreConCambiosExplicacion,
        Texto::CierreBotonGuardar,
        Texto::CierreBotonNoGuardar,
        Texto::CierreBotonCancelar,
    ];
    for idioma in Idioma::TODOS {
        let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
        app.presentacion.preferencias.idioma = idioma;
        app.presentacion.pregunta_al_cerrar = true;
        let textos = crate::arnes_interfaz::textos_de_ventana_en_pantalla(
            &mut app,
            egui::vec2(1280.0, 800.0),
            crate::ui::dialogo_de_cierre::dibujar_dialogo_de_cierre,
        );
        let mut vistos = std::collections::HashSet::new();
        for clave in claves {
            let texto = clave.en(idioma);
            assert!(!texto.trim().is_empty(), "{idioma:?} {clave:?}");
            assert!(vistos.insert(texto), "{idioma:?}: «{texto}» repetido");
            assert!(
                textos.iter().any(|pintado| pintado.contains(texto)),
                "{idioma:?}: no se pinta «{texto}» en {textos:?}"
            );
        }
    }
}

/// C4: «Guardar» guarda también el título que estaba a medio escribir al pedir el cierre.
/// Se encontró al verificar la 0.15.0.
#[test]
fn guardar_al_cerrar_incluye_el_titulo_en_edicion() {
    let carpeta = std::env::temp_dir().join(format!("mmcelt_cierre_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&carpeta).expect("crear carpeta de prueba");
    let ruta = carpeta.join("mapa.mmcelt");
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let id = app.mapa.proyecto.root_id;
    app.anotar_guardado(&ruta);
    assert!(app.solicitar_guardado(false), "guardado del mapa inicial");
    let nuevo = "Título escrito justo antes de cerrar";
    app.abrir_edicion_del_nodo(id, nuevo);

    let ordenes = fotograma_de_cierre(&mut app, &ctx, true);
    assert!(ordenes.contains(&egui::ViewportCommand::CancelClose));
    responder_al_cierre(&mut app, &ctx, RespuestaAlCerrar::Guardar);

    let guardado = super::cargar_proyecto_de_archivo(&ruta).expect("mapa legible");
    let _ = std::fs::remove_dir_all(&carpeta);
    assert!(app.presentacion.cierre_confirmado);
    assert_eq!(
        guardado.nodes[&id].title, nuevo,
        "título que quedó en disco"
    );
}

/// C7: sin archivo se pregunta aunque el mapa no haya cambiado, salvo que sea desechable.
#[test]
fn un_mapa_sin_archivo_pregunta_salvo_que_sea_desechable() {
    let sin_archivo = CambiosPendientes {
        sin_archivo: true,
        ..Default::default()
    };
    assert!(hay_que_preguntar_al_cerrar(sin_archivo));
    assert!(!hay_que_preguntar_al_cerrar(CambiosPendientes {
        mapa_desechable: true,
        ..sin_archivo
    }));
}

/// C7 y C8: un escaneo, una importación o una plantilla sin archivo pregunta aunque no se
/// toque; un mapa nuevo vacío de «Nuevo mapa» y el ejemplo del arranque, no; con archivo y sin
/// cambios, tampoco.
#[test]
fn al_cerrar_pregunta_todo_mapa_sin_archivo_que_tenga_contenido() {
    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    assert!(app.mapa.es_un_mapa_de_partida());
    assert!(!hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(
        &app
    )));

    let mut escaneado = Proyecto::nuevo_vacio("Escaneo");
    let raiz = escaneado.root_id;
    escaneado.anadir_hijo(raiz, "src");
    app.sustituir_el_mapa_abierto(escaneado.clone(), None, "");
    assert!(!app.mapa.es_un_mapa_de_partida());
    assert!(
        hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(&app)),
        "un mapa sin archivo con contenido se perdería al cerrar"
    );

    crate::aplicacion::abrir_un_mapa_nuevo(&mut app);
    assert!(app.mapa.es_un_mapa_de_partida());
    assert!(!hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(
        &app
    )));

    app.sustituir_el_mapa_abierto(
        escaneado,
        Some(std::path::PathBuf::from("mapa-que-no-se-escribe.mmcelt")),
        "",
    );
    assert!(!hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(
        &app
    )));
}

/// Caso hallado en una revisión: una importación que solo trae la raíz, con notas, también pregunta
/// al cerrar sin archivo. Solo «Nuevo mapa» da un mapa que se puede cerrar sin preguntar.
#[test]
fn una_importacion_con_solo_la_raiz_pregunta_al_cerrar() {
    // Lo mismo que deja el importador con ese JSON (el caso hallado): solo la raíz, con notas.
    let mut proyecto = Proyecto::nuevo_vacio("Trabajo importado");
    let raiz = proyecto.root_id;
    proyecto.nodes.get_mut(&raiz).expect("raíz").notes = "Notas que quiero conservar".to_owned();
    assert_eq!(proyecto.nodes.len(), 1, "solo la raíz");

    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    app.sustituir_el_mapa_abierto(proyecto, None, "");
    assert!(!app.mapa.es_un_mapa_de_partida());
    assert!(
        hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(&app)),
        "las notas importadas se perderían al cerrar"
    );

    let mut solo_raiz_sin_notas = Proyecto::nuevo_vacio("Plantilla de una idea");
    solo_raiz_sin_notas
        .nodes
        .get_mut(&solo_raiz_sin_notas.root_id)
        .expect("raíz")
        .notes = String::new();
    app.sustituir_el_mapa_abierto(solo_raiz_sin_notas, None, "");
    assert!(
        hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(&app)),
        "lo que no viene de «Nuevo mapa» pregunta aunque solo tenga la raíz"
    );
}
