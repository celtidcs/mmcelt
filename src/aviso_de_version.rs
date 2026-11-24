//! # Aviso de versión nueva (`aviso_de_version.rs`)
//!
//! El único módulo de MMCelt que habla con la biblioteca `aviso-version-github`. Al arrancar, y
//! solo si el usuario no lo ha desactivado, pregunta una vez a GitHub cuál es la última versión
//! publicada y deja preparado un aviso si es más nueva que la que se está ejecutando. **No
//! descarga ni instala nada**: el aviso enlaza la página de la versión y abrirla es decisión del
//! usuario.
//!
//! Conecta la biblioteca de la copia en `bibliotecas/aviso-version-github` de la forma que
//! ella misma recomienda, y está copiado de su integración de referencia, `examples/integracion_de_referencia.rs`. Lo único propio de MMCelt es el tipo de
//! las preferencias (los dos campos viven en [`crate::preferencias::Preferencias`]) y el
//! registro, que es `mmcelt-errores.log`.
//!
//! ## Garantías
//!
//! - **No bloquea la interfaz**: la consulta corre en un hilo de trabajo con nombre y la
//!   interfaz recoge el resultado con `try_recv` cuando está listo.
//! - **No molesta si falla**: sin red, con el límite de peticiones agotado o con una respuesta
//!   rara, no hay aviso ni ventana de error; el fallo se registra y ya está.
//! - **Desactivada, no hay tráfico**: si la opción está apagada no se crea ni la consulta.

use crate::preferencias::Preferencias;
use aviso_version_github::{
    comprobar, proxy_del_sistema, AjustesDeVersionNueva, AvisoDeVersionNueva, ConsultaDeVersion,
    ConsultaHttps, FalloDeComprobacion, OrigenDeLasVersiones, ProgramaQueConsulta,
};
use std::fmt;
use std::sync::{mpsc, Arc};
use std::thread;

/// Nombre con que MMCelt se presenta en la cabecera User-Agent de la consulta.
pub const NOMBRE_DEL_PROGRAMA: &str = "mmcelt";

/// Repositorio público de MMCelt, de donde salen las versiones publicadas. Es el valor de
/// fábrica de las preferencias; el usuario puede cambiarlo.
pub const REPOSITORIO_DE_VERSIONES_POR_DEFECTO: &str = "celtidcs/mmcelt";

/// Nombre del hilo de trabajo, para reconocerlo en un depurador o en un volcado.
const NOMBRE_DEL_HILO: &str = "aviso-de-version";

/// Contexto con el que se registran los fallos de la comprobación.
const CONTEXTO_DEL_REGISTRO: &str = "comprobar si hay una versión nueva en GitHub";

/// Dónde deja el programa los fallos. Nunca se muestran al usuario como un error.
pub type Registro = Arc<dyn Fn(&dyn fmt::Display) + Send + Sync>;

/// Lo que llega por el canal cuando termina la comprobación: el aviso, o nada si MMCelt ya
/// está al día o la comprobación falló (y el fallo ya se registró).
pub type ResultadoDeVersion = Option<AvisoDeVersionNueva>;

/// Ajustes de fábrica: el repositorio de MMCelt en github.com y los límites de la biblioteca.
///
/// La forma de los ajustes, sus valores por defecto neutros (API y web de GitHub, diez segundos
/// de espera y un mebibyte de respuesta) y su validación están en la biblioteca.
///
/// # Devuelve
/// Unos ajustes que superan la validación de `OrigenDeLasVersiones::desde_ajustes`.
pub fn ajustes_de_fabrica() -> AjustesDeVersionNueva {
    AjustesDeVersionNueva::para_repositorio(REPOSITORIO_DE_VERSIONES_POR_DEFECTO)
}

/// El registro de MMCelt: cada fallo va a `mmcelt-errores.log` como aviso.
pub fn registro_de_mmcelt() -> Registro {
    Arc::new(|fallo: &dyn fmt::Display| {
        let error = crate::error::AppError::ComprobacionDeVersion {
            detalle: fallo.to_string(),
        };
        crate::error::registrar(&error, CONTEXTO_DEL_REGISTRO);
    })
}

/// La consulta real: HTTPS, con el proxy del sistema y MMCelt identificado.
///
/// Es la que pasa el programa como `fabricar`. Las pruebas pasan una consulta falsa.
///
/// # Parámetros
/// - `origen`: de dónde se pregunta, ya validado.
pub fn consulta_real(origen: &OrigenDeLasVersiones) -> Box<dyn ConsultaDeVersion> {
    match ProgramaQueConsulta::nuevo(NOMBRE_DEL_PROGRAMA, crate::version::VERSION) {
        Ok(programa) => Box::new(ConsultaHttps::para_programa(
            origen,
            proxy_del_sistema(),
            &programa,
        )),
        // No ocurre con un nombre constante válido, que una prueba vigila; si ocurriera, se
        // registra y la consulta sale solo con el nombre de la biblioteca.
        Err(fallo) => {
            registro_de_mmcelt()(&fallo);
            Box::new(ConsultaHttps::nueva(origen, proxy_del_sistema()))
        }
    }
}

/// Arranca la comprobación en un hilo de trabajo y devuelve el canal por el que llegará el
/// resultado, o `None` si no hay nada que esperar.
///
/// Con la opción desactivada, o con unos ajustes que no superan la validación, **no se llama a
/// `fabricar`**: no llega a existir nada capaz de salir a la red. Un fallo de validación, de
/// red o de respuesta se registra y nunca provoca un pánico.
///
/// # Parámetros
/// - `preferencias`: de aquí salen la opción y los ajustes.
/// - `version_local`: la versión que se está ejecutando.
/// - `fabricar`: construye la consulta; el programa pasa [`consulta_real`].
/// - `registrar`: dónde se dejan los fallos; el programa pasa [`registro_de_mmcelt`].
pub fn arrancar<F>(
    preferencias: &Preferencias,
    version_local: &'static str,
    fabricar: F,
    registrar: Registro,
) -> Option<mpsc::Receiver<ResultadoDeVersion>>
where
    F: FnOnce(&OrigenDeLasVersiones) -> Box<dyn ConsultaDeVersion>,
{
    if !preferencias.comprobar_version_nueva {
        return None;
    }
    let origen = match OrigenDeLasVersiones::desde_ajustes(&preferencias.ajustes_de_version) {
        Ok(origen) => origen,
        Err(fallo) => {
            registrar(&fallo);
            return None;
        }
    };
    let consulta = fabricar(&origen);
    let (enviar, recibir) = mpsc::channel();
    let registrar_en_el_hilo = Arc::clone(&registrar);
    let hilo = thread::Builder::new()
        .name(NOMBRE_DEL_HILO.to_string())
        .spawn(move || {
            let aviso = decidir(
                comprobar(version_local, &origen, consulta.as_ref()),
                &registrar_en_el_hilo,
            );
            // Si la ventana ya se cerró, nadie espera el resultado: no es un fallo.
            let _ = enviar.send(aviso);
        });
    match hilo {
        Ok(_) => Some(recibir),
        Err(error) => {
            registrar(&error);
            None
        }
    }
}

/// Convierte el resultado de la biblioteca en lo que espera la interfaz, registrando el fallo
/// si lo hay.
fn decidir(
    resultado: Result<Option<AvisoDeVersionNueva>, FalloDeComprobacion>,
    registrar: &Registro,
) -> ResultadoDeVersion {
    resultado.unwrap_or_else(|fallo| {
        registrar(&fallo);
        None
    })
}
