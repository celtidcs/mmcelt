//! Error único de las operaciones MCP.
//!
//! La clasificación estable sirve al cliente para decidir si debe corregir argumentos,
//! respetar una restricción de seguridad o reintentar una operación. La causa opcional se
//! conserva para diagnóstico sin convertirla prematuramente en una cadena.

use std::fmt;

/// Clases de fallo que una herramienta puede devolver dentro de su resultado MCP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TipoErrorHerramienta {
    /// La petición intenta superar el espacio o los permisos autorizados.
    Seguridad,
    /// Los datos recibidos no cumplen el contrato de la herramienta.
    Argumento,
    /// Un archivo o texto no tiene el formato esperado.
    Formato,
    /// La operación validada falló al ejecutarse por una causa del entorno.
    Ejecucion,
}

impl TipoErrorHerramienta {
    /// Identificador estable publicado en `error_type`.
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Seguridad => "security",
            Self::Argumento => "argument",
            Self::Formato => "format",
            Self::Ejecucion => "runtime",
        }
    }
}

/// Fallo producido después de aceptar una llamada a una herramienta.
#[derive(Debug)]
pub(crate) struct ErrorHerramienta {
    mensaje: String,
    tipo: TipoErrorHerramienta,
    causa: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ErrorHerramienta {
    fn nuevo(tipo: TipoErrorHerramienta, mensaje: impl Into<String>) -> Self {
        Self {
            mensaje: mensaje.into(),
            tipo,
            causa: None,
        }
    }

    /// Crea un rechazo de seguridad que no debe revelar rutas absolutas.
    pub(crate) fn seguridad(mensaje: impl Into<String>) -> Self {
        Self::nuevo(TipoErrorHerramienta::Seguridad, mensaje)
    }

    /// Crea un rechazo porque los argumentos no cumplen el contrato publicado.
    pub(crate) fn argumento(mensaje: impl Into<String>) -> Self {
        Self::nuevo(TipoErrorHerramienta::Argumento, mensaje)
    }

    /// Crea un fallo ocurrido después de validar la petición y sus permisos.
    pub(crate) fn ejecucion(mensaje: impl Into<String>) -> Self {
        Self::nuevo(TipoErrorHerramienta::Ejecucion, mensaje)
    }

    /// Crea un fallo de ejecución conservando el error original.
    pub(crate) fn ejecucion_con_causa(
        mensaje: impl Into<String>,
        causa: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            mensaje: mensaje.into(),
            tipo: TipoErrorHerramienta::Ejecucion,
            causa: Some(Box::new(causa)),
        }
    }

    /// Crea un fallo de formato conservando el error que explica el archivo inválido.
    pub(crate) fn formato_con_causa(
        mensaje: impl Into<String>,
        causa: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            mensaje: mensaje.into(),
            tipo: TipoErrorHerramienta::Formato,
            causa: Some(Box::new(causa)),
        }
    }

    /// Explicación segura que se devolverá al cliente MCP.
    pub(crate) fn mensaje(&self) -> &str {
        &self.mensaje
    }

    /// Clasificación estable que se traduce al código JSON-RPC correspondiente.
    pub(crate) const fn tipo(&self) -> TipoErrorHerramienta {
        self.tipo
    }
}

impl fmt::Display for ErrorHerramienta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.mensaje)
    }
}

impl std::error::Error for ErrorHerramienta {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.causa
            .as_deref()
            .map(|causa| causa as &(dyn std::error::Error + 'static))
    }
}
