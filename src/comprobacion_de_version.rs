//! # Comprobación de versión nueva (`comprobacion_de_version.rs`)
//!
//! Al arrancar, y solo si el usuario no lo ha desactivado, pregunta una vez a GitHub cuál es
//! la última versión publicada y deja preparado un aviso si es más nueva que la que se está
//! ejecutando. **No descarga ni instala nada**: el aviso enlaza la página de la versión y
//! abrirla es decisión del usuario.
//!
//! ## Garantías
//!
//! - **No bloquea la interfaz**: la consulta corre en un hilo de trabajo y la interfaz solo
//!   recoge el resultado cuando está listo.
//! - **No molesta si falla**: sin red, con el límite de peticiones agotado o con una
//!   respuesta rara, no hay aviso ni ventana de error; el fallo se registra en
//!   `mmcelt-errores.log` y ya está.
//! - **Desactivada, no hay tráfico**: si la opción está apagada no se crea ni la consulta.
//! - **Solo HTTPS verificado**: la consulta real usa `rustls` con las raíces de
//!   `webpki-roots` y rechaza cualquier dirección que no sea `https://`.

use crate::preferencias::AjustesDeVersionNueva;
use crate::version_publicada::{
    decidir_aviso, interpretar_respuesta, AvisoDeVersionNueva, FalloDeComprobacion,
    OrigenDeLasVersiones,
};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

/// Cabecera `Accept` que recomienda GitHub para su API REST.
///
/// Fuente: <https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types>.
const TIPO_DE_RESPUESTA_DE_GITHUB: &str = "application/vnd.github+json";

/// Quién pide la versión: GitHub exige una cabecera `User-Agent` en toda petición a su API.
///
/// Fuente: <https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api#user-agent>.
const AGENTE_DE_USUARIO: &str = concat!("mmcelt/", env!("CARGO_PKG_VERSION"));

/// Contexto con el que se registran los fallos de la comprobación.
const CONTEXTO_DEL_REGISTRO: &str = "comprobar si hay una versión nueva en GitHub";

/// Algo capaz de pedir una dirección y devolver el cuerpo de la respuesta.
///
/// Existe para que la decisión se pueda probar sin red: las pruebas usan una consulta falsa
/// y el programa, [`ConsultaHttps`].
pub trait ConsultaDeVersion: Send {
    /// Pide la dirección y devuelve el cuerpo de la respuesta como texto.
    ///
    /// # Errores
    /// [`FalloDeComprobacion::Red`] si no se obtuvo una respuesta correcta.
    fn pedir(&self, url: &str) -> Result<String, FalloDeComprobacion>;
}

/// Consulta real por HTTPS con `ureq` y `rustls`.
pub struct ConsultaHttps {
    /// Cliente ya configurado: solo HTTPS, tiempo de espera y agente de usuario.
    agente: ureq::Agent,
    /// Tamaño máximo del cuerpo que se acepta leer.
    limite_de_respuesta_en_bytes: u64,
}

impl ConsultaHttps {
    /// Prepara el cliente con los ajustes de las preferencias.
    ///
    /// # Parámetros
    /// - `ajustes`: de aquí salen el tiempo de espera y el límite de respuesta.
    pub fn nueva(ajustes: &AjustesDeVersionNueva) -> Self {
        let configuracion = ureq::Agent::config_builder()
            .https_only(true)
            .timeout_global(Some(Duration::from_secs(ajustes.segundos_de_espera)))
            .user_agent(AGENTE_DE_USUARIO)
            .build();
        Self {
            agente: configuracion.into(),
            limite_de_respuesta_en_bytes: ajustes.limite_de_respuesta_en_bytes,
        }
    }
}

impl ConsultaDeVersion for ConsultaHttps {
    /// Hace la petición `GET` y lee el cuerpo, como mucho hasta el límite configurado.
    ///
    /// Un estado HTTP de error (404, 403 por límite de peticiones…) llega como `Err`, porque
    /// `ureq` los trata así por defecto.
    fn pedir(&self, url: &str) -> Result<String, FalloDeComprobacion> {
        let red = |error: ureq::Error| FalloDeComprobacion::Red(error.to_string());
        let mut respuesta = self
            .agente
            .get(url)
            .header("Accept", TIPO_DE_RESPUESTA_DE_GITHUB)
            .call()
            .map_err(red)?;
        respuesta
            .body_mut()
            .with_config()
            .limit(self.limite_de_respuesta_en_bytes)
            .read_to_string()
            .map_err(red)
    }
}

/// Pregunta por la última versión y decide si hay que avisar.
///
/// # Parámetros
/// - `version_local`: la versión que se está ejecutando.
/// - `origen`: de dónde se pregunta y adónde se enlaza.
/// - `consulta`: quien hace la petición.
///
/// # Errores
/// Cualquier [`FalloDeComprobacion`] de la consulta o de la respuesta.
pub fn comprobar(
    version_local: &str,
    origen: &OrigenDeLasVersiones,
    consulta: &dyn ConsultaDeVersion,
) -> Result<Option<AvisoDeVersionNueva>, FalloDeComprobacion> {
    let cuerpo = consulta.pedir(&origen.url_de_consulta())?;
    let etiqueta = interpretar_respuesta(&cuerpo)?;
    decidir_aviso(version_local, &etiqueta, origen)
}

/// Deja constancia de un fallo sin molestar al usuario.
///
/// # Parámetros
/// - `fallo`: lo que impidió comprobar.
pub fn registrar_fallo(fallo: &FalloDeComprobacion) {
    let error = crate::error::AppError::ComprobacionDeVersion {
        detalle: fallo.to_string(),
    };
    crate::error::registrar(&error, CONTEXTO_DEL_REGISTRO);
}

/// Lanza la comprobación en un hilo de trabajo y devuelve por dónde llegará el resultado.
///
/// El hilo envía `Some(aviso)` si hay versión nueva y `None` si no la hay o si algo falló
/// (el fallo queda registrado). Después pide un repintado para que la interfaz lo recoja.
///
/// # Parámetros
/// - `version_local`: la versión que se está ejecutando.
/// - `origen`: de dónde se pregunta, ya validado.
/// - `consulta`: quien hace la petición.
/// - `ctx`: contexto de `egui` al que pedir el repintado.
pub fn lanzar_en_segundo_plano(
    version_local: &'static str,
    origen: OrigenDeLasVersiones,
    consulta: Box<dyn ConsultaDeVersion>,
    ctx: egui::Context,
) -> Receiver<Option<AvisoDeVersionNueva>> {
    let (emisor, receptor) = channel();
    std::thread::spawn(move || {
        let aviso = comprobar(version_local, &origen, consulta.as_ref()).unwrap_or_else(|fallo| {
            registrar_fallo(&fallo);
            None
        });
        // Si la ventana ya se cerró, nadie espera el resultado: no es un error.
        let _ = emisor.send(aviso);
        ctx.request_repaint();
    });
    receptor
}
