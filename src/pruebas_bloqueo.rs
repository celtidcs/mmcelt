//! El mapa no se edita mientras el agente trabaja (0.16.0).
//!
//! - Con el mapa bloqueado, un cambio del contenido se deshace y el mapa vuelve a la
//!   revisión que tenía; confirmar un envío bloquea.
//! - Una entrega parcial del agente no desbloquea; la marcada como terminada, sí. Una
//!   entrega anterior al bloqueo no cuenta.
//! - «Recuperar el control» desbloquea.
//! - Si el envío falla, no queda bloqueo.

use super::carpeta_temporal_contexto;
use super::pruebas_vigilancia::{
    encolar_envio_de_un_mapa_sin_archivo, escribir_como_el_agente,
    esperar_el_envio_y_su_vigilancia, TITULO_DEL_AGENTE,
};
use crate::bloqueo_por_agente::{es_la_entrega_final, INTERVALO_DE_LECTURA_DEL_RECIBO};
use crate::devolucion_agentes::{consultar_recibo, OperacionDevolucion, ReciboDevolucion};
use crate::envio_agente::EstadoEnvio;
use crate::proyecto_trabajo::{ContextoProyecto, IdProyecto};
use std::path::Path;
use std::time::{Duration, Instant};

/// Título con el que la persona intenta cambiar la raíz con el mapa bloqueado.
/// Huella del archivo cuando llega el recibo final, en las pruebas que no leen disco.
const HUELLA_FINAL: u64 = 2;

/// Huella de una lectura anterior al recibo que se aplica después de él.
const HUELLA_PARCIAL: u64 = 1;

const TITULO_INTENTADO: &str = "Cambio con el mapa bloqueado";

/// Tiempo máximo de espera del envío en segundo plano.
const ESPERA_MAXIMA: Duration = Duration::from_secs(5);

/// Escribe en `raiz` el recibo de una entrega del agente sobre `mapa` y devuelve su
/// identificador.
fn escribir_recibo(raiz: &Path, mapa: &str, terminado: bool) -> uuid::Uuid {
    let contexto =
        ContextoProyecto::nuevo(IdProyecto::nuevo(), "prueba", raiz).expect("contexto de prueba");
    let mut recibo = ReciboDevolucion::nuevo(
        &contexto,
        mapa,
        "Agente de prueba",
        OperacionDevolucion::Modificado,
    )
    .expect("recibo de prueba");
    recibo.trabajo_terminado = terminado;
    std::fs::create_dir_all(raiz.join(".mmcelt")).expect("carpeta del recibo");
    std::fs::write(
        raiz.join(crate::devolucion_agentes::RUTA_DEL_RECIBO),
        serde_json::to_vec(&recibo).expect("serializar el recibo"),
    )
    .expect("escribir el recibo");
    recibo.id_entrega
}

/// Un cambio hecho con el mapa bloqueado se deshace al final del fotograma.
#[test]
fn con_el_mapa_bloqueado_un_cambio_se_deshace() {
    let (mut app, _ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("bl1-deshacer");
    let antes = app.mapa().proyecto().revision();
    let id_raiz = app.mapa().proyecto().root_id;
    let titulo = app.mapa().proyecto().nodes[&id_raiz].title.clone();
    app.agentes.bloqueo.bloquear(&app.mapa.proyecto, &raiz);

    let proyecto = app.mapa_mut().proyecto_para_editar();
    proyecto.nodes.get_mut(&id_raiz).expect("raíz").title = TITULO_INTENTADO.to_owned();
    proyecto.marcar_modificado();
    crate::aplicacion::proteger_el_mapa_bloqueado(&mut app);

    assert_eq!(app.mapa().proyecto().revision(), antes);
    assert_eq!(app.mapa().proyecto().nodes[&id_raiz].title, titulo);
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// Sin bloqueo, el mismo cambio se queda.
#[test]
fn sin_bloqueo_el_cambio_se_queda() {
    let (mut app, _ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let id_raiz = app.mapa().proyecto().root_id;
    let proyecto = app.mapa_mut().proyecto_para_editar();
    proyecto.nodes.get_mut(&id_raiz).expect("raíz").title = TITULO_INTENTADO.to_owned();
    proyecto.marcar_modificado();
    crate::aplicacion::proteger_el_mapa_bloqueado(&mut app);
    assert_eq!(
        app.mapa().proyecto().nodes[&id_raiz].title,
        TITULO_INTENTADO
    );
}

/// Confirmar el envío deja el mapa bloqueado.
#[test]
fn confirmar_un_envio_bloquea_el_mapa() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("bl1-confirmar");
    crate::proyecto_trabajo::RepositorioProyecto::abrir_o_inicializar(&raiz)
        .expect("identificar proyecto");
    let ruta = raiz.join("mapa.mmcelt");
    app.agentes.espacio_de_trabajo = raiz.clone();
    let mut agente = super::agente_cli_para_vista_previa(raiz.join("no-debe-usarse.exe"));
    agente.id = crate::conectores::IdAgente::Agy;
    agente.cli = None;
    agente.conectado = true;
    agente.espacio_registrado = Some(raiz.display().to_string().replace('\\', "/"));
    super::preparar_envio_para_prueba(&mut app, &agente).expect("preparar el envío");

    let idioma = app.idioma();
    let iniciado = crate::envio_agente_comando::iniciar_envio_confirmado(
        &mut app.agentes,
        &app.mapa.proyecto,
        Some(&ruta),
        None,
        idioma,
        &ctx,
    )
    .expect("iniciar el envío");
    assert!(iniciado);
    assert!(app.agentes.bloqueo.esta_bloqueado());

    let limite = Instant::now() + ESPERA_MAXIMA;
    while app.agentes.envio == EstadoEnvio::Preparando {
        app.procesar_eventos_de_envio(&ctx);
        assert!(Instant::now() < limite, "el envío no terminó");
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(
        app.agentes.bloqueo.esta_bloqueado(),
        "sigue bloqueado tras preparar"
    );
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// Solo la entrega nueva, del mismo mapa y marcada como terminada, desbloquea.
#[test]
fn solo_la_entrega_final_del_mismo_mapa_cuenta() {
    let raiz = carpeta_temporal_contexto("bl2-recibos");
    let ruta = raiz.join("mapa.mmcelt");
    std::fs::write(&ruta, "mapa").expect("mapa");
    let contexto = ContextoProyecto::nuevo(IdProyecto::nuevo(), "prueba", &raiz).expect("contexto");
    let recibo = |mapa: &str, terminado: bool| {
        let mut recibo =
            ReciboDevolucion::nuevo(&contexto, mapa, "Agente", OperacionDevolucion::Modificado)
                .expect("recibo");
        recibo.trabajo_terminado = terminado;
        recibo
    };
    let parcial = recibo("mapa.mmcelt", false);
    let final_ = recibo("mapa.mmcelt", true);
    let de_otro_mapa = recibo("otro.mmcelt", true);

    assert!(!es_la_entrega_final(&parcial, &raiz, &ruta, None));
    assert!(es_la_entrega_final(&final_, &raiz, &ruta, None));
    assert!(!es_la_entrega_final(&de_otro_mapa, &raiz, &ruta, None));
    assert!(
        !es_la_entrega_final(&final_, &raiz, &ruta, Some(final_.id_entrega)),
        "la entrega que ya estaba al bloquear no cuenta"
    );
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// El bloqueo lee el recibo del disco; una entrega parcial no desbloquea y la final sí.
#[test]
fn el_recibo_final_en_disco_desbloquea() {
    let (app, _ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("bl2-disco");
    let ruta = raiz.join("mapa.mmcelt");
    std::fs::write(&ruta, "mapa").expect("mapa");
    escribir_recibo(&raiz, "mapa.mmcelt", true);
    let mut bloqueo = crate::bloqueo_por_agente::BloqueoPorAgente::default();
    bloqueo.bloquear(app.mapa().proyecto(), &raiz);

    let inicio = Instant::now();
    assert!(
        !bloqueo.comprobar_si_termino(&raiz, &ruta, inicio, || Some(HUELLA_FINAL)),
        "el recibo de antes del bloqueo no cuenta"
    );
    escribir_recibo(&raiz, "mapa.mmcelt", false);
    let despues = inicio + INTERVALO_DE_LECTURA_DEL_RECIBO;
    assert!(!bloqueo.comprobar_si_termino(&raiz, &ruta, despues, || Some(HUELLA_FINAL)));
    assert!(bloqueo.esta_bloqueado());
    escribir_recibo(&raiz, "mapa.mmcelt", true);
    assert!(bloqueo.comprobar_si_termino(
        &raiz,
        &ruta,
        despues + INTERVALO_DE_LECTURA_DEL_RECIBO,
        || Some(HUELLA_FINAL)
    ));
    assert!(
        bloqueo.esta_bloqueado(),
        "el recibo final solo no desbloquea: falta cargar el mapa"
    );
    assert!(!bloqueo.aceptar_entrega(app.mapa().proyecto(), Some(HUELLA_PARCIAL)));
    assert!(
        bloqueo.esta_bloqueado(),
        "una lectura parcial aplicada después del recibo no desbloquea"
    );
    assert!(bloqueo.aceptar_entrega(app.mapa().proyecto(), Some(HUELLA_FINAL)));
    assert!(!bloqueo.esta_bloqueado());
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// «Recuperar el control» quita el bloqueo y vuelve a dejar editar.
#[test]
fn recuperar_el_control_desbloquea() {
    let (mut app, _ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("bl3-recuperar");
    app.agentes.bloqueo.bloquear(&app.mapa.proyecto, &raiz);
    app.agentes.bloqueo.desbloquear();
    let id_raiz = app.mapa().proyecto().root_id;
    let proyecto = app.mapa_mut().proyecto_para_editar();
    proyecto.nodes.get_mut(&id_raiz).expect("raíz").title = TITULO_INTENTADO.to_owned();
    proyecto.marcar_modificado();
    crate::aplicacion::proteger_el_mapa_bloqueado(&mut app);
    assert_eq!(
        app.mapa().proyecto().nodes[&id_raiz].title,
        TITULO_INTENTADO
    );
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// Un envío que falla no deja el mapa bloqueado.
#[test]
fn un_envio_que_falla_no_deja_bloqueo() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("bl4-fallo");
    app.agentes.trabajador =
        crate::envio_agente::EnvioEnSegundoPlano::con_funcion(ctx.clone(), |trabajo| {
            crate::envio_agente::ResultadoTrabajoEnvio {
                instantanea: crate::envio_agente::instantanea_guardada(&trabajo.proyecto),
                ruta_mapa: trabajo.ruta_mapa.clone(),
                mapa_guardado: false,
                huella_guardada: None,
                resultado: Err(crate::error::AppError::RutaFueraDelProyectoConDetalle {
                    ruta: trabajo.ruta_mapa,
                    raiz: std::path::PathBuf::new(),
                }),
            }
        });
    let _ruta = encolar_envio_de_un_mapa_sin_archivo(&mut app, &raiz);
    app.agentes.bloqueo.bloquear(&app.mapa.proyecto, &raiz);

    let limite = Instant::now() + ESPERA_MAXIMA;
    while app.agentes.envio == EstadoEnvio::Preparando {
        app.procesar_eventos_de_envio(&ctx);
        assert!(Instant::now() < limite, "el envío no terminó");
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(app.agentes.envio, EstadoEnvio::Fallido);
    assert!(!app.agentes.bloqueo.esta_bloqueado());
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// Caso hallado en una revisión: con la entrega final del agente en disco, el mapa sigue bloqueado hasta
/// que en pantalla está ese mapa, y solo entonces se desbloquea.
#[test]
fn la_entrega_final_desbloquea_al_cargar_su_mapa() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("bl2-orden");
    let ruta = encolar_envio_de_un_mapa_sin_archivo(&mut app, &raiz);
    esperar_el_envio_y_su_vigilancia(&mut app, &ctx, &ruta);
    app.agentes.bloqueo.bloquear(&app.mapa.proyecto, &raiz);

    escribir_como_el_agente(&ruta);
    let nombre = ruta
        .file_name()
        .and_then(|n| n.to_str())
        .expect("nombre del mapa");
    escribir_recibo(&raiz, nombre, true);

    let limite = Instant::now() + ESPERA_MAXIMA;
    loop {
        app.procesar_eventos_de_vigilancia();
        let cargado =
            app.mapa.proyecto.nodes[&app.mapa.proyecto.root_id].title == TITULO_DEL_AGENTE;
        assert!(
            cargado || app.agentes.bloqueo.esta_bloqueado(),
            "se desbloqueó antes de cargar la entrega final"
        );
        if cargado && !app.agentes.bloqueo.esta_bloqueado() {
            break;
        }
        assert!(Instant::now() < limite, "la entrega final no desbloqueó");
        std::thread::sleep(Duration::from_millis(20));
    }
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// Sin recibo no hay nada que leer ni que registrar; un recibo que no es JSON es un error de
/// formato, que se registra, y uno válido se lee entero.
#[test]
fn un_recibo_ilegible_es_un_error_y_su_falta_no() {
    let raiz = carpeta_temporal_contexto("recibo-ilegible");
    assert!(matches!(consultar_recibo(&raiz), Ok(None)));

    let id = escribir_recibo(&raiz, "mapa.mmcelt", true);
    assert!(matches!(consultar_recibo(&raiz), Ok(Some(recibo)) if recibo.id_entrega == id));

    std::fs::write(
        raiz.join(crate::devolucion_agentes::RUTA_DEL_RECIBO),
        "{ no es json",
    )
    .expect("estropear el recibo");
    assert!(matches!(
        consultar_recibo(&raiz),
        Err(crate::error::AppError::Formato { .. })
    ));
}
