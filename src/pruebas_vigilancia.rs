//! La aplicación ve lo que escribe el agente (0.15.0).
//!
//! - Al enviar a un agente un mapa sin archivo, el envío lo guarda con un nombre nuevo y la
//!   vigilancia queda sobre ese archivo, no sobre el anterior ni sobre nada.
//! - Lo que escribe el agente se recarga tanto si llega después de instalar la vigilancia
//!   como si llega antes, entre el guardado y la instalación, cuando nadie vigilaba aún.
//! - La comprobación periódica detecta un cambio aunque no llegue ningún aviso del sistema.

use super::{agente_cli_para_vista_previa, carpeta_temporal_contexto, preparar_envio_para_prueba};
use crate::envio_agente::EstadoEnvio;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Intervalo corto para que las pruebas con espera no tarden.
const INTERVALO_DE_PRUEBA: Duration = Duration::from_millis(50);

/// Tiempo máximo de espera de cualquier paso asíncrono de estas pruebas.
const ESPERA_MAXIMA: Duration = Duration::from_secs(5);

/// Título con el que «el agente» cambia la raíz del mapa desde fuera de la aplicación.
pub(super) const TITULO_DEL_AGENTE: &str = "Nodo modificado desde fuera de MMCelt";

/// Prepara el envío a un agente de un mapa sin archivo y lo pone en cola, como hace «Enviar a…»
/// cuando la persona elige el nombre en «Guardar como». Devuelve la ruta que se guardará.
pub(super) fn encolar_envio_de_un_mapa_sin_archivo(
    app: &mut crate::AplicacionMapaMental,
    raiz: &Path,
) -> PathBuf {
    crate::proyecto_trabajo::RepositorioProyecto::abrir_o_inicializar(raiz)
        .expect("identificar proyecto");
    app.agentes.espacio_de_trabajo = raiz.to_path_buf();
    app.persistencia.ruta_actual = None;
    let mut agente = agente_cli_para_vista_previa(raiz.join("no-debe-usarse.exe"));
    agente.id = crate::conectores::IdAgente::Agy;
    agente.cli = None;
    agente.conectado = true;
    agente.espacio_registrado = Some(raiz.display().to_string().replace('\\', "/"));
    preparar_envio_para_prueba(app, &agente).expect("preparar el envío");

    let ruta_nueva = raiz.join("Juego del ahorcado.mmcelt");
    let solicitud =
        crate::envio_agente_comando::solicitud_de_envio(&app.agentes, app.mapa.proyecto.revision())
            .expect("la solicitud se forma")
            .expect("hay una solicitud confirmable");
    let trabajo = crate::envio_agente_comando::construir_trabajo_de_envio(
        &app.mapa.proyecto,
        app.idioma(),
        solicitud,
        ruta_nueva.clone(),
    );
    assert!(app.agentes.trabajador.solicitar(trabajo));
    app.agentes.envio = EstadoEnvio::Preparando;
    ruta_nueva
}

/// Atiende el envío hasta que termina y comprueba que el archivo creado queda vigilado.
pub(super) fn esperar_el_envio_y_su_vigilancia(
    app: &mut crate::AplicacionMapaMental,
    ctx: &egui::Context,
    ruta_nueva: &Path,
) {
    let limite = Instant::now() + ESPERA_MAXIMA;
    while app.agentes.envio == EstadoEnvio::Preparando {
        app.procesar_eventos_de_envio(ctx);
        assert!(Instant::now() < limite, "el envío no terminó");
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(app.agentes.envio, EstadoEnvio::Preparado);
    assert_eq!(app.persistencia.ruta_actual.as_deref(), Some(ruta_nueva));
    let vigilante = app
        .persistencia
        .vigilante
        .as_ref()
        .expect("el archivo creado por el envío queda vigilado");
    assert_eq!(vigilante.ruta(), ruta_nueva);
}

/// Escribe en el mapa, como haría un agente desde fuera, en cuanto el envío lo ha guardado.
pub(super) fn escribir_como_el_agente(ruta: &Path) {
    let limite = Instant::now() + ESPERA_MAXIMA;
    let mut externo = loop {
        if let Ok(proyecto) = crate::storage::cargar_proyecto_de_archivo(ruta) {
            break proyecto;
        }
        assert!(
            Instant::now() < limite,
            "el envío no escribió un mapa válido"
        );
        std::thread::sleep(INTERVALO_DE_PRUEBA);
    };
    let raiz_del_mapa = externo.root_id;
    externo.nodes.get_mut(&raiz_del_mapa).expect("raíz").title = TITULO_DEL_AGENTE.to_owned();
    externo.marcar_modificado();
    crate::storage::guardar_proyecto_en_archivo(&externo, ruta).expect("escritura externa");
}

/// Espera a que la aplicación recargue lo que escribió el agente.
fn esperar_la_recarga(app: &mut crate::AplicacionMapaMental, caso: &str) {
    let limite = Instant::now() + ESPERA_MAXIMA;
    while app.mapa.proyecto.nodes[&app.mapa.proyecto.root_id].title != TITULO_DEL_AGENTE {
        app.procesar_eventos_de_vigilancia();
        assert!(Instant::now() < limite, "no se recargó: {caso}");
        std::thread::sleep(INTERVALO_DE_PRUEBA);
    }
}

/// Un mapa de ejemplo sin archivo enviado a un agente. El envío le dio uno y los cambios
/// del agente no llegaban porque no se vigilaba ese archivo.
#[test]
fn enviar_un_mapa_sin_archivo_vigila_el_archivo_que_crea_el_envio() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("vigilancia-tras-envio");
    let ruta_nueva = encolar_envio_de_un_mapa_sin_archivo(&mut app, &raiz);
    esperar_el_envio_y_su_vigilancia(&mut app, &ctx, &ruta_nueva);
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar el envío");
}

/// El agente escribe después de instalarse la vigilancia, el caso normal.
#[test]
fn lo_que_escribe_el_agente_despues_de_vigilar_se_recarga() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("escritura-tras-vigilar");
    let ruta_nueva = encolar_envio_de_un_mapa_sin_archivo(&mut app, &raiz);
    esperar_el_envio_y_su_vigilancia(&mut app, &ctx, &ruta_nueva);
    escribir_como_el_agente(&ruta_nueva);
    esperar_la_recarga(&mut app, "escritura posterior a la vigilancia");
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar el envío");
}

/// Caso hallado en una revisión: el agente escribe entre el guardado y la instalación de la vigilancia.
/// Esa escritura no da ningún aviso, y antes la aplicación la tomaba por propia.
#[test]
fn lo_que_escribe_el_agente_antes_de_vigilar_tambien_se_recarga() {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto("escritura-antes-de-vigilar");
    let ruta_nueva = encolar_envio_de_un_mapa_sin_archivo(&mut app, &raiz);
    assert!(app.persistencia.vigilante.is_none());
    escribir_como_el_agente(&ruta_nueva);
    esperar_el_envio_y_su_vigilancia(&mut app, &ctx, &ruta_nueva);
    esperar_la_recarga(&mut app, "escritura anterior a la vigilancia");
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar el envío");
}

/// Sin avisos del sistema, la comprobación periódica basta para ver que el archivo cambió.
#[test]
fn la_comprobacion_periodica_detecta_un_cambio_sin_avisos_del_sistema() {
    let carpeta = carpeta_temporal_contexto("comprobacion-periodica");
    let ruta = carpeta.join("mapa.mmcelt");
    std::fs::write(&ruta, "antes").expect("escribir el mapa");
    let vigilante =
        crate::vigilante::instalar(&ruta, egui::Context::default(), INTERVALO_DE_PRUEBA, false)
            .expect("instalar la comprobación periódica");

    std::thread::sleep(INTERVALO_DE_PRUEBA * 3);
    assert!(!vigilante.hay_cambios(), "sin cambios no hay aviso");

    std::fs::write(&ruta, "después, con otro tamaño").expect("cambiar el mapa");
    let limite = Instant::now() + ESPERA_MAXIMA;
    while !vigilante.hay_cambios() {
        assert!(
            Instant::now() < limite,
            "la comprobación periódica no vio el cambio"
        );
        std::thread::sleep(INTERVALO_DE_PRUEBA);
    }
    drop(vigilante);
    std::fs::remove_dir_all(carpeta).expect("limpiar la carpeta");
}
