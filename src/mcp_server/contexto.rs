//! Contexto de autorización de una instancia del servidor, independiente del entorno global.

use std::path::{Path, PathBuf};

/// Carpeta configurada al arrancar una instancia; la validación de rutas sigue en seguridad.
///
/// Conservar la ruta sin resolver permite informar de una carpeta que aún no existe y volver
/// a comprobar enlaces y pertenencia en cada operación. Ninguna petición puede sustituirla.
pub(crate) struct EspacioMcp {
    raiz: Option<PathBuf>,
}

impl EspacioMcp {
    /// Captura una sola vez el contrato de entorno del proceso al arrancar el servidor.
    pub(crate) fn desde_entorno() -> Self {
        Self::nuevo(std::env::var_os(super::VARIABLE_ESPACIO_TRABAJO).map(PathBuf::from))
    }

    /// Crea una instancia con la raíz explícita; None conserva el rechazo por falta de raíz.
    pub(crate) fn nuevo(raiz: Option<PathBuf>) -> Self {
        Self { raiz }
    }

    /// Consulta la raíz de esta instancia, sin leer ni modificar el entorno del proceso.
    pub(super) fn ruta_configurada(&self) -> Option<&Path> {
        self.raiz.as_deref()
    }
}
