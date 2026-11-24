//! Aviso de versión nueva con `aviso-version-github` 0.2.4.
//!
//! Las cinco pruebas mínimas que la biblioteca pide a todo programa que la use, más la de las
//! constantes. Usan una consulta falsa y un registro de prueba: ninguna
//! sale a la red ni escribe en `mmcelt-errores.log`.

use crate::aviso_de_version::{
    arrancar, Registro, NOMBRE_DEL_PROGRAMA, REPOSITORIO_DE_VERSIONES_POR_DEFECTO,
};
use crate::preferencias::Preferencias;
use aviso_version_github::{
    AjustesDeVersionNueva, ConsultaDeVersion, FalloDeComprobacion, OrigenDeLasVersiones,
    ProgramaQueConsulta,
};
use std::cell::Cell;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

/// Versión local de las pruebas.
const VERSION_LOCAL: &str = "1.2.0";

/// Respuesta de la API con la etiqueta dada.
fn publicada(etiqueta: &str) -> Result<String, FalloDeComprobacion> {
    Ok(format!(r#"{{"tag_name":"{etiqueta}"}}"#))
}

/// Un registro que guarda lo que recibe, para comprobarlo.
fn registro_de_prueba() -> (Registro, Arc<Mutex<Vec<String>>>) {
    let apuntes = Arc::new(Mutex::new(Vec::new()));
    let destino = Arc::clone(&apuntes);
    let registro: Registro = Arc::new(move |fallo: &dyn std::fmt::Display| {
        destino
            .lock()
            .expect("registro no envenenado")
            .push(fallo.to_string());
    });
    (registro, apuntes)
}

/// Arranca con las preferencias y la respuesta dadas y devuelve si se fabricó la consulta, lo
/// que llegó por el canal (`None` si no hubo canal) y lo registrado.
fn arrancar_con(
    preferencias: &Preferencias,
    respuesta: Result<String, FalloDeComprobacion>,
) -> (bool, Option<Option<String>>, Vec<String>) {
    let fabricada = Cell::new(false);
    let (registro, apuntes) = registro_de_prueba();
    let canal = arrancar(
        preferencias,
        VERSION_LOCAL,
        |_origen: &OrigenDeLasVersiones| -> Box<dyn ConsultaDeVersion> {
            fabricada.set(true);
            // La consulta falsa es la de las barreras, el módulo padre de estas pruebas.
            Box::new(super::ConsultaFalsa {
                respuesta,
                llamadas: Arc::new(AtomicUsize::new(0)),
            })
        },
        registro,
    );
    let llegado = canal.map(|recibir| {
        recibir
            .recv()
            .expect("el hilo tiene que enviar un resultado")
            .map(|aviso| aviso.version)
    });
    let registrado = apuntes.lock().expect("registro no envenenado").clone();
    (fabricada.get(), llegado, registrado)
}

/// Con la opción desactivada no se fabrica ninguna consulta.
#[test]
fn desactivada_no_fabrica_la_consulta() {
    let preferencias = Preferencias {
        comprobar_version_nueva: false,
        ..Preferencias::default()
    };
    assert_eq!(
        arrancar_con(&preferencias, publicada("v9.0.0")),
        (false, None, Vec::new())
    );
}

/// Con ajustes no válidos no se fabrica la consulta y el fallo se registra.
#[test]
fn ajustes_no_validos_se_registran_sin_fabricar() {
    let mut preferencias = Preferencias::default();
    preferencias.ajustes_de_version.url_api = "http://api.github.com".into();
    let (fabricada, llegado, registrado) = arrancar_con(&preferencias, publicada("v9.0.0"));
    assert!(!fabricada && llegado.is_none());
    assert_eq!(
        registrado.len(),
        1,
        "el fallo tiene que registrarse: {registrado:?}"
    );
}

/// Una versión publicada mayor llega como aviso.
#[test]
fn version_mayor_llega_como_aviso() {
    let (fabricada, llegado, registrado) =
        arrancar_con(&Preferencias::default(), publicada("v1.3.0"));
    assert!(fabricada && registrado.is_empty());
    assert_eq!(llegado, Some(Some("1.3.0".to_string())));
}

/// La misma versión no produce aviso ni fallo.
#[test]
fn misma_version_no_avisa() {
    let (_, llegado, registrado) = arrancar_con(&Preferencias::default(), publicada("v1.2.0"));
    assert_eq!(llegado, Some(None));
    assert!(registrado.is_empty());
}

/// Un fallo de red no avisa, se registra y no provoca un pánico.
#[test]
fn fallo_de_red_se_registra_sin_avisar() {
    let (_, llegado, registrado) = arrancar_con(
        &Preferencias::default(),
        Err(FalloDeComprobacion::Red("sin conexión".into())),
    );
    assert_eq!(llegado, Some(None));
    assert_eq!(
        registrado.len(),
        1,
        "el fallo tiene que registrarse: {registrado:?}"
    );
}

/// Las constantes de MMCelt son válidas para la biblioteca, y unas preferencias leídas sin
/// estos campos quedan activadas con los ajustes de fábrica.
#[test]
fn las_constantes_y_los_valores_de_fabrica_son_validos() {
    assert!(ProgramaQueConsulta::nuevo(NOMBRE_DEL_PROGRAMA, crate::version::VERSION).is_ok());
    assert!(
        OrigenDeLasVersiones::desde_ajustes(&AjustesDeVersionNueva::para_repositorio(
            REPOSITORIO_DE_VERSIONES_POR_DEFECTO
        ))
        .is_ok()
    );
    let leidas = Preferencias::desde_texto("{}");
    assert!(leidas.comprobar_version_nueva);
    assert_eq!(
        leidas.ajustes_de_version,
        crate::aviso_de_version::ajustes_de_fabrica()
    );
}

/// La consulta corre en el hilo de trabajo con su nombre, para reconocerlo en un depurador
/// o en un volcado. El registro de un fallo se hace en ese hilo, así que anota su nombre. Se
/// compara con el nombre que fija la integración de la biblioteca, no con la
/// constante del código, para que cambiarla también dé rojo.
#[test]
fn la_consulta_corre_en_el_hilo_con_su_nombre() {
    let hilos = Arc::new(Mutex::new(Vec::new()));
    let anotados = Arc::clone(&hilos);
    let registro: Registro = Arc::new(move |_fallo: &dyn std::fmt::Display| {
        anotados
            .lock()
            .expect("hilos no envenenados")
            .push(std::thread::current().name().map(str::to_owned));
    });
    let canal = arrancar(
        &Preferencias::default(),
        VERSION_LOCAL,
        |_origen: &OrigenDeLasVersiones| -> Box<dyn ConsultaDeVersion> {
            Box::new(super::ConsultaFalsa {
                respuesta: Err(FalloDeComprobacion::Red("sin conexión".into())),
                llamadas: Arc::new(AtomicUsize::new(0)),
            })
        },
        registro,
    )
    .expect("la comprobación tiene que arrancar");
    canal.recv().expect("el hilo tiene que enviar un resultado");

    assert_eq!(
        *hilos.lock().expect("hilos no envenenados"),
        vec![Some("aviso-de-version".to_owned())]
    );
}
