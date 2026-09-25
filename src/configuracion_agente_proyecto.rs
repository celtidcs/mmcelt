//! Persistencia portable del consentimiento para instrucciones nativas de agentes.
//!
//! Este módulo conserva qué versión exacta de `AGENTS.md`, `CLAUDE.md` o `GEMINI.md` aceptó la
//! persona. No decide cómo se dibujan ni compone prompts: compara bytes y guarda la decisión.

use crate::error::{AppError, AppResult, CampoEntrada, MotivoEntradaInvalida};
use crate::proyecto_trabajo::ContextoProyecto;
use crate::textos::Idioma;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

/// Ubicación portable de las decisiones sobre instrucciones de agentes.
pub const RUTA_CONFIGURACION_AGENTE: &str = ".mmcelt/configuracion-agente.json";
/// Versión vigente del contrato JSON de esta configuración.
const VERSION_FORMATO: u32 = 1;
/// Límite defensivo para una configuración que solo contiene tres fuentes de texto.
const BYTES_MAXIMOS_CONFIGURACION: u64 = 4 * 1024 * 1024;
/// Únicos nombres nativos que MMCelt reconoce en la raíz del proyecto.
const NOMBRES_NATIVOS: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "GEMINI.md"];

/// Resultado de comparar una fuente actual con la última versión aceptada por la persona.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstadoFuenteNativa {
    /// La fuente no tiene todavía una aceptación guardada.
    Nueva,
    /// El contenido coincide byte por byte con la versión aceptada.
    AceptadaSinCambios,
    /// Existe una aceptación anterior, pero el archivo ha cambiado.
    Modificada,
}

/// Versión exacta de una fuente que recibió consentimiento humano.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuenteAceptada {
    /// SHA-256 del contenido UTF-8 aceptado.
    pub huella: [u8; 32],
    /// Contenido anterior necesario para enseñar una diferencia real si cambia.
    pub contenido: String,
}

/// Configuración portable de la relación entre un proyecto y los agentes externos.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfiguracionAgenteProyecto {
    /// Versión del formato persistido.
    pub version_formato: u32,
    /// Idioma independiente elegido para el documento destinado al agente.
    pub idioma_documento: Option<Idioma>,
    /// Único encargo preparado para la próxima sesión con un agente.
    #[serde(default)]
    pub encargo: String,
    /// Versiones nativas aceptadas, indexadas por su nombre canónico.
    pub fuentes_aceptadas: BTreeMap<String, FuenteAceptada>,
}

impl Default for ConfiguracionAgenteProyecto {
    fn default() -> Self {
        Self {
            version_formato: VERSION_FORMATO,
            idioma_documento: None,
            encargo: String::new(),
            fuentes_aceptadas: BTreeMap::new(),
        }
    }
}

impl ConfiguracionAgenteProyecto {
    /// Comprueba versión, nombres y huellas antes de que una decisión persistida pueda usarse.
    fn validar(&self) -> AppResult<()> {
        if self.version_formato != VERSION_FORMATO {
            return Err(configuracion_invalida(
                MotivoEntradaInvalida::VersionNoCompatible,
            ));
        }
        for (nombre, aceptada) in &self.fuentes_aceptadas {
            if !NOMBRES_NATIVOS.contains(&nombre.as_str()) {
                return Err(configuracion_invalida(
                    MotivoEntradaInvalida::FuenteNoReconocida,
                ));
            }
            if huella(&aceptada.contenido) != aceptada.huella {
                return Err(configuracion_invalida(
                    MotivoEntradaInvalida::HuellaNoCorresponde,
                ));
            }
        }
        Ok(())
    }
}

/// Carga las decisiones del proyecto o devuelve una configuración inicial si aún no existen.
///
/// # Errores
///
/// Devuelve un error si la ruta sale del proyecto, el archivo es demasiado grande, no es JSON
/// válido o incumple la versión, los nombres o las huellas del contrato.
pub fn cargar_configuracion(contexto: &ContextoProyecto) -> AppResult<ConfiguracionAgenteProyecto> {
    let ruta = contexto.resolver_relativa(Path::new(RUTA_CONFIGURACION_AGENTE))?;
    if !ruta.exists() {
        return Ok(ConfiguracionAgenteProyecto::default());
    }
    let metadatos = std::fs::metadata(&ruta).map_err(|error| AppError::lectura(&ruta, error))?;
    if metadatos.len() > BYTES_MAXIMOS_CONFIGURACION {
        return Err(AppError::EntradaDemasiadoGrande {
            origen: RUTA_CONFIGURACION_AGENTE.to_string(),
            limite: BYTES_MAXIMOS_CONFIGURACION,
        });
    }
    let contenido = std::fs::read(&ruta).map_err(|error| AppError::lectura(&ruta, error))?;
    let configuracion: ConfiguracionAgenteProyecto = serde_json::from_slice(&contenido)
        .map_err(|error| AppError::formato("al leer la configuración de agentes", error))?;
    configuracion.validar()?;
    Ok(configuracion)
}

/// Guarda el idioma y los consentimientos mediante reemplazo atómico.
///
/// # Errores
///
/// Rechaza una configuración incoherente y propaga cualquier fallo al validar, crear o escribir
/// la ruta portable.
///
/// # Efectos secundarios
///
/// Crea `.mmcelt` cuando sea necesario y reemplaza `configuracion-agente.json` atómicamente.
pub fn guardar_configuracion(
    contexto: &ContextoProyecto,
    configuracion: &ConfiguracionAgenteProyecto,
) -> AppResult<()> {
    configuracion.validar()?;
    let ruta = contexto.resolver_relativa(Path::new(RUTA_CONFIGURACION_AGENTE))?;
    let carpeta = ruta.parent().ok_or(AppError::RutaFueraDelProyecto)?;
    std::fs::create_dir_all(carpeta).map_err(|error| AppError::escritura(carpeta, error))?;
    let contenido = serde_json::to_vec_pretty(configuracion)
        .map_err(|error| AppError::formato("al guardar la configuración de agentes", error))?;
    crate::storage::escribir_de_forma_atomica(&ruta, &contenido)
}

/// Acepta la versión que existe ahora en disco, nunca una copia procedente de la interfaz.
///
/// # Errores
///
/// Rechaza nombres no reconocidos, enlaces que salen del proyecto, archivos ausentes o ilegibles
/// y configuraciones previas inválidas. Si falla, no cambia el consentimiento anterior.
///
/// # Efectos secundarios
///
/// Actualiza atómicamente la entrada de la fuente en la configuración portable.
pub fn aceptar_fuente(contexto: &ContextoProyecto, nombre: &str) -> AppResult<()> {
    validar_nombre(nombre)?;
    let ruta = contexto.resolver_relativa(Path::new(nombre))?;
    let contenido =
        std::fs::read_to_string(&ruta).map_err(|error| AppError::lectura(&ruta, error))?;
    let mut configuracion = cargar_configuracion(contexto)?;
    configuracion.fuentes_aceptadas.insert(
        nombre.to_string(),
        FuenteAceptada {
            huella: huella(&contenido),
            contenido,
        },
    );
    guardar_configuracion(contexto, &configuracion)
}

/// Compara una fuente detectada con la decisión durable sin modificar ninguna de las dos.
pub fn evaluar_fuente(
    configuracion: &ConfiguracionAgenteProyecto,
    nombre: &str,
    contenido: &str,
) -> (EstadoFuenteNativa, bool, Option<String>) {
    let Some(anterior) = configuracion.fuentes_aceptadas.get(nombre) else {
        return (EstadoFuenteNativa::Nueva, false, None);
    };
    if anterior.huella == huella(contenido) {
        return (
            EstadoFuenteNativa::AceptadaSinCambios,
            true,
            Some(anterior.contenido.clone()),
        );
    }
    (
        EstadoFuenteNativa::Modificada,
        false,
        Some(anterior.contenido.clone()),
    )
}

/// Calcula la identidad estable de un contenido UTF-8.
fn huella(contenido: &str) -> [u8; 32] {
    Sha256::digest(contenido.as_bytes()).into()
}

/// Limita toda lectura y escritura a los tres archivos nativos previstos por el contrato.
fn validar_nombre(nombre: &str) -> AppResult<()> {
    if NOMBRES_NATIVOS.contains(&nombre) {
        return Ok(());
    }
    Err(AppError::EntradaInvalida {
        campo: CampoEntrada::FuenteNativa,
        motivo: MotivoEntradaInvalida::NombreNoPermitido,
    })
}

/// Construye el error semántico común para una configuración manipulada o incoherente.
fn configuracion_invalida(motivo: MotivoEntradaInvalida) -> AppError {
    AppError::EntradaInvalida {
        campo: CampoEntrada::ConfiguracionAgentes,
        motivo,
    }
}
