//! Contrato neutral con el que cualquier agente devuelve un mapa a MMCelt.

use crate::error::{AppError, AppResult, MotivoCarpetaInvalida};
use crate::proyecto_trabajo::{ContextoProyecto, IdProyecto};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

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
