//! # Versión publicada (`version_publicada.rs`)
//!
//! Decide, sin tocar la red, si la última versión publicada en GitHub es más nueva que la que
//! se está ejecutando y, si lo es, qué aviso dar.
//!
//! ## Por qué está aparte de la consulta
//!
//! Todo lo que aquí se decide —leer la respuesta, comparar versiones, construir el enlace—
//! se comprueba en las pruebas con textos fijos. La consulta real a GitHub vive en
//! [`crate::comprobacion_de_version`], detrás de un rasgo, y ninguna prueba la necesita.
//!
//! ## La respuesta es un dato ajeno
//!
//! De la respuesta de `releases/latest` solo se toma `tag_name`, y solo si es exactamente
//! `X.Y.Z` con una `v` opcional delante. El enlace del aviso **no** sale de la respuesta
//! (`html_url` podría apuntar a cualquier sitio): se compone con la dirección web y el
//! repositorio de la configuración, más la etiqueta ya validada, que solo contiene dígitos,
//! puntos y quizá una `v`.

use crate::preferencias::AjustesDeVersionNueva;
use std::fmt;

/// Prefijo opcional con el que GitHub suele nombrar las etiquetas de versión (`v0.12.0`).
const PREFIJO_DE_ETIQUETA: char = 'v';

/// Número de componentes de una versión `mayor.menor.parche`.
const COMPONENTES_DE_UNA_VERSION: usize = 3;

/// Por qué no se pudo comprobar si hay una versión nueva.
///
/// Ninguno de estos fallos llega al usuario como error: la comprobación es una cortesía y,
/// si falla, simplemente no se avisa. Se registran para poder depurarlos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FalloDeComprobacion {
    /// No se pudo hablar con el servidor: sin red, tiempo agotado, certificado, estado HTTP
    /// de error (incluido el límite de peticiones).
    Red(String),
    /// La respuesta no es el JSON esperado o no trae `tag_name` como texto.
    RespuestaIlegible(String),
    /// La etiqueta publicada, o la versión local, no tiene la forma `X.Y.Z`.
    EtiquetaNoValida(String),
    /// Los ajustes guardados en las preferencias no se pueden usar con seguridad.
    AjustesNoValidos(String),
}

impl fmt::Display for FalloDeComprobacion {
    /// Texto técnico del fallo, para el registro.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FalloDeComprobacion::Red(detalle) => write!(f, "fallo de red: {detalle}"),
            FalloDeComprobacion::RespuestaIlegible(detalle) => {
                write!(f, "respuesta ilegible: {detalle}")
            }
            FalloDeComprobacion::EtiquetaNoValida(etiqueta) => {
                write!(f, "etiqueta de versión no válida: «{etiqueta}»")
            }
            FalloDeComprobacion::AjustesNoValidos(detalle) => {
                write!(f, "ajustes de comprobación no válidos: {detalle}")
            }
        }
    }
}

/// Versión `mayor.menor.parche` comparable como números.
///
/// El orden derivado compara campo a campo en el orden de declaración, que es justo el de
/// la numeración semántica: por eso `0.12.0` es mayor que `0.9.9`, cosa que una comparación
/// de texto haría al revés.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionSemantica {
    /// Primer número.
    mayor: u64,
    /// Segundo número.
    menor: u64,
    /// Tercer número.
    parche: u64,
}

impl VersionSemantica {
    /// Lee una etiqueta como `v0.12.0` o `0.12.0`.
    ///
    /// Se rechaza todo lo que no sea exactamente tres números separados por puntos, con una
    /// `v` minúscula opcional delante: sufijos de prepublicación (`-rc1`), metadatos
    /// (`+compilacion`), espacios, números que no caben en `u64`. Una versión que no se puede
    /// comparar con seguridad no puede provocar un aviso.
    ///
    /// # Parámetros
    /// - `etiqueta`: el texto de la etiqueta.
    ///
    /// # Devuelve
    /// La versión, o `None` si la etiqueta no tiene la forma esperada.
    pub fn desde_etiqueta(etiqueta: &str) -> Option<Self> {
        let numeros = etiqueta
            .strip_prefix(PREFIJO_DE_ETIQUETA)
            .unwrap_or(etiqueta);
        let partes: Vec<&str> = numeros.split('.').collect();
        if partes.len() != COMPONENTES_DE_UNA_VERSION {
            return None;
        }
        let mut valores = [0_u64; COMPONENTES_DE_UNA_VERSION];
        for (valor, parte) in valores.iter_mut().zip(&partes) {
            // `u64::from_str` admite un `+` delante; aquí solo valen dígitos.
            if parte.is_empty() || !parte.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            *valor = parte.parse().ok()?;
        }
        let [mayor, menor, parche] = valores;
        Some(Self {
            mayor,
            menor,
            parche,
        })
    }
}

impl fmt::Display for VersionSemantica {
    /// Escribe la versión como `mayor.menor.parche`, sin prefijo.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.mayor, self.menor, self.parche)
    }
}

/// Lo que se le dice al usuario cuando hay una versión más nueva.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvisoDeVersionNueva {
    /// La versión publicada, sin prefijo (`0.13.0`).
    pub version: String,
    /// Página de esa versión en GitHub, compuesta desde la configuración.
    pub enlace: String,
}

/// Dónde se pregunta por la última versión y dónde se enlaza, ya validado.
///
/// Solo se construye con [`Self::desde_ajustes`], que rechaza lo que no sea seguro: así,
/// quien tiene uno puede componer direcciones sin volver a comprobar nada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrigenDeLasVersiones {
    /// Raíz de la API, sin barra final (`https://api.github.com`).
    url_api: String,
    /// Raíz de la web, sin barra final (`https://github.com`).
    url_web: String,
    /// `dueño/nombre` del repositorio.
    repositorio: String,
}

impl OrigenDeLasVersiones {
    /// Valida los ajustes de las preferencias.
    ///
    /// # Parámetros
    /// - `ajustes`: lo guardado en `preferencias.json`, que es texto y cualquiera puede
    ///   haber editado a mano.
    ///
    /// # Errores
    /// [`FalloDeComprobacion::AjustesNoValidos`] si alguna dirección no es `https://` con
    /// solo un nombre de servidor, si el repositorio no es `dueño/nombre` con caracteres
    /// seguros, o si el tiempo de espera o el límite de respuesta son cero.
    pub fn desde_ajustes(ajustes: &AjustesDeVersionNueva) -> Result<Self, FalloDeComprobacion> {
        if ajustes.segundos_de_espera == 0 {
            return Err(FalloDeComprobacion::AjustesNoValidos(
                "el tiempo de espera es cero".to_string(),
            ));
        }
        if ajustes.limite_de_respuesta_en_bytes == 0 {
            return Err(FalloDeComprobacion::AjustesNoValidos(
                "el límite de respuesta es cero".to_string(),
            ));
        }
        Ok(Self {
            url_api: raiz_https_valida(&ajustes.url_api)?,
            url_web: raiz_https_valida(&ajustes.url_web)?,
            repositorio: repositorio_valido(&ajustes.repositorio)?,
        })
    }

    /// Dirección de la API que devuelve la última versión publicada.
    ///
    /// Documentada en <https://docs.github.com/en/rest/releases/releases#get-the-latest-release>:
    /// «the most recent non-prerelease, non-draft release».
    pub fn url_de_consulta(&self) -> String {
        format!(
            "{}/repos/{}/releases/latest",
            self.url_api, self.repositorio
        )
    }

    /// Página web de la versión con la etiqueta dada.
    ///
    /// # Parámetros
    /// - `etiqueta`: una etiqueta ya validada por [`VersionSemantica::desde_etiqueta`].
    fn enlace_de_la_version(&self, etiqueta: &str) -> String {
        format!(
            "{}/{}/releases/tag/{etiqueta}",
            self.url_web, self.repositorio
        )
    }
}

/// Comprueba que una dirección es `https://servidor` y nada más.
///
/// Se admite solo el nombre del servidor (letras, dígitos, puntos y guiones) y, como mucho,
/// una barra final. Sin ruta, credenciales, puerto ni parámetros, que es lo que necesitan
/// GitHub y GitHub Enterprise y deja fuera cualquier forma de colar otra cosa.
///
/// # Devuelve
/// La dirección sin la barra final.
fn raiz_https_valida(url: &str) -> Result<String, FalloDeComprobacion> {
    const ESQUEMA: &str = "https://";
    let no_valida = || FalloDeComprobacion::AjustesNoValidos(format!("dirección «{url}»"));
    let servidor = url
        .strip_prefix(ESQUEMA)
        .ok_or_else(no_valida)?
        .trim_end_matches('/');
    let caracteres_seguros = servidor
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-');
    if servidor.is_empty() || !caracteres_seguros {
        return Err(no_valida());
    }
    Ok(format!("{ESQUEMA}{servidor}"))
}

/// Comprueba que el repositorio es `dueño/nombre` con caracteres que GitHub admite.
fn repositorio_valido(repositorio: &str) -> Result<String, FalloDeComprobacion> {
    let no_valido =
        || FalloDeComprobacion::AjustesNoValidos(format!("repositorio «{repositorio}»"));
    let partes: Vec<&str> = repositorio.split('/').collect();
    let parte_valida = |parte: &&str| {
        !parte.is_empty()
            && !parte.starts_with('.')
            && parte
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
    };
    if partes.len() != 2 || !partes.iter().all(parte_valida) {
        return Err(no_valido());
    }
    Ok(repositorio.to_string())
}

/// Saca `tag_name` de la respuesta de `releases/latest`.
///
/// # Parámetros
/// - `cuerpo`: el texto de la respuesta, ya acotado en tamaño por quien lo leyó.
///
/// # Errores
/// [`FalloDeComprobacion::RespuestaIlegible`] si no es un objeto JSON con `tag_name` de
/// texto. Es lo que llega, por ejemplo, cuando GitHub responde con una página de error.
pub fn interpretar_respuesta(cuerpo: &str) -> Result<String, FalloDeComprobacion> {
    /// Lo único que interesa de la respuesta; el resto de campos se ignora.
    #[derive(serde::Deserialize)]
    struct UltimaVersion {
        /// Etiqueta git de la versión publicada.
        tag_name: String,
    }
    serde_json::from_str::<UltimaVersion>(cuerpo)
        .map(|ultima| ultima.tag_name)
        .map_err(|error| FalloDeComprobacion::RespuestaIlegible(error.to_string()))
}

/// Decide si hay que avisar de una versión nueva.
///
/// # Parámetros
/// - `version_local`: la versión que se está ejecutando (`version::VERSION`).
/// - `etiqueta_publicada`: `tag_name` de la última versión publicada.
/// - `origen`: de dónde salen los enlaces.
///
/// # Devuelve
/// `Some` con el aviso solo si la publicada es estrictamente mayor que la local.
///
/// # Errores
/// [`FalloDeComprobacion::EtiquetaNoValida`] si alguna de las dos versiones no es `X.Y.Z`.
pub fn decidir_aviso(
    version_local: &str,
    etiqueta_publicada: &str,
    origen: &OrigenDeLasVersiones,
) -> Result<Option<AvisoDeVersionNueva>, FalloDeComprobacion> {
    let no_valida = |etiqueta: &str| FalloDeComprobacion::EtiquetaNoValida(etiqueta.to_string());
    let local =
        VersionSemantica::desde_etiqueta(version_local).ok_or_else(|| no_valida(version_local))?;
    let publicada = VersionSemantica::desde_etiqueta(etiqueta_publicada)
        .ok_or_else(|| no_valida(etiqueta_publicada))?;

    if publicada <= local {
        return Ok(None);
    }
    Ok(Some(AvisoDeVersionNueva {
        version: publicada.to_string(),
        enlace: origen.enlace_de_la_version(etiqueta_publicada),
    }))
}
