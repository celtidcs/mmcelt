//! Contrato neutral con el que cualquier agente devuelve un mapa a MMCelt.

use crate::error::{registrar, AppError, AppResult, MotivoCarpetaInvalida};
use crate::proyecto_trabajo::{ContextoProyecto, IdProyecto};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Ruta del recibo de entrega dentro de la carpeta del proyecto.
///
/// Es parte del contrato: el servidor MCP lo escribe aquí y la aplicación lo lee aquí para saber
/// que el agente ha terminado. Las dos partes usan esta misma constante.
pub(crate) const RUTA_DEL_RECIBO: &str = ".mmcelt/devolucion.json";

/// Clase de cambio que un agente declara haber realizado sobre el mapa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperacionDevolucion {
    /// El agente creó un mapa nuevo.
    Generado,
    /// El agente actualizó un mapa existente.
    Modificado,
}

/// Recibo durable e independiente del proveedor que identifica una devolución.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReciboDevolucion {
    /// Identificador único de esta entrega concreta.
    pub id_entrega: Uuid,
    /// Proyecto al que afirma pertenecer el resultado.
    pub project_id: IdProyecto,
    /// Raíz canónica utilizada por el agente.
    pub base_path: PathBuf,
    /// Mapa devuelto, siempre relativo a `base_path`.
    pub map_path: PathBuf,
    /// Nombre legible del modelo o aplicación remitente.
    pub modelo: String,
    /// Si el mapa fue creado o modificado.
    pub operacion: OperacionDevolucion,
    /// El agente declara que con esta entrega termina su trabajo. Mientras no llega una entrega
    /// así, la aplicación mantiene el mapa bloqueado a la edición. Los recibos anteriores a este
    /// campo se leen como `false`.
    #[serde(default)]
    pub trabajo_terminado: bool,
}

impl ReciboDevolucion {
    /// Construye un recibo bien formado sin establecer diferencias entre proveedores.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la ruta del mapa es absoluta, está vacía, no termina en
    /// `.mmcelt` o el nombre del modelo no contiene texto.
    pub fn nuevo(
        contexto: &ContextoProyecto,
        map_path: impl Into<PathBuf>,
        modelo: impl Into<String>,
        operacion: OperacionDevolucion,
    ) -> AppResult<Self> {
        let recibo = Self {
            id_entrega: Uuid::new_v4(),
            project_id: contexto.id(),
            base_path: contexto.raiz().to_path_buf(),
            map_path: map_path.into(),
            modelo: modelo.into(),
            operacion,
            trabajo_terminado: false,
        };
        recibo.validar_formato()?;
        Ok(recibo)
    }

    /// Comprueba la coherencia del recibo con el proyecto y resuelve el mapa al que se refiere.
    ///
    /// Es una defensa en profundidad antes de conservar la trazabilidad. Un resultado correcto
    /// no concede permiso para escribir, no confirma que el contenido del mapa sea válido y no
    /// provoca su recarga: esas decisiones pertenecen a las fronteras de escritura y vigilancia.
    ///
    /// # Errores
    ///
    /// Rechaza recibos incompletos, una identidad que no sea la del proyecto activo, una raíz
    /// distinta de la configurada y cualquier ruta que pueda salir de esa raíz.
    pub fn validar(&self, contexto: &ContextoProyecto) -> AppResult<PathBuf> {
        self.validar_formato()?;
        contexto.comprobar_id(self.project_id)?;
        let base =
            self.base_path
                .canonicalize()
                .map_err(|_| AppError::CarpetaProyectoInvalida {
                    ruta: self.base_path.clone(),
                    motivo: MotivoCarpetaInvalida::CarpetaAgenteNoExiste,
                })?;
        if base != contexto.raiz() {
            return Err(AppError::RutaFueraDelProyecto);
        }
        contexto.resolver_relativa(&self.map_path)
    }

    /// Comprueba los campos que no dependen del proyecto activo.
    fn validar_formato(&self) -> AppResult<()> {
        let extension_valida = self
            .map_path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("mmcelt"));
        if self.modelo.trim().is_empty()
            || self.map_path == Path::new("")
            || self.map_path.is_absolute()
            || !extension_valida
        {
            return Err(AppError::CarpetaProyectoInvalida {
                ruta: self.base_path.clone(),
                motivo: MotivoCarpetaInvalida::ReciboAgenteIncompleto,
            });
        }
        Ok(())
    }
}

/// Contexto con el que se registran los fallos al leer el recibo.
const CONTEXTO_DEL_RECIBO: &str = "Lectura del recibo de entrega del agente";

/// Lee el recibo de entrega del proyecto, si hay uno legible.
///
/// Un recibo que no se puede leer o que no es un JSON válido se registra en
/// `mmcelt-errores.log`, porque deja el mapa bloqueado hasta que el agente lo reescriba o la
/// persona recupere el control. Que todavía no exista es lo normal y no se registra.
pub(crate) fn leer_recibo(raiz: &Path) -> Option<ReciboDevolucion> {
    consultar_recibo(raiz).unwrap_or_else(|error| {
        registrar(&error, CONTEXTO_DEL_RECIBO);
        None
    })
}

/// Lee el recibo de entrega del proyecto sin registrar nada.
///
/// # Devuelve
/// `Ok(None)` si todavía no hay recibo, y `Ok(Some(recibo))` si hay uno válido.
///
/// # Errores
/// [`AppError::Lectura`] si el archivo existe pero no se puede leer, y [`AppError::Formato`] si
/// no es un recibo en JSON válido.
pub(crate) fn consultar_recibo(raiz: &Path) -> Result<Option<ReciboDevolucion>, AppError> {
    let ruta = raiz.join(RUTA_DEL_RECIBO);
    let datos = match std::fs::read(&ruta) {
        Ok(datos) => datos,
        Err(origen) if origen.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(origen) => return Err(AppError::Lectura { ruta, origen }),
    };
    serde_json::from_slice(&datos)
        .map(Some)
        .map_err(|origen| AppError::Formato {
            contexto: ruta.display().to_string(),
            origen,
        })
}
