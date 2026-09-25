//! Fachada del servidor MCP integrado.
//!
//! El resto de la aplicación solo necesita arrancar el servidor o, en pruebas, entregar una
//! línea JSON-RPC. El protocolo, los argumentos, los errores y las operaciones permanecen en
//! módulos separados para que cada frontera tenga un único motivo de cambio.

mod actualizaciones;
mod argumentos;
mod catalogo;
pub(crate) mod error;
mod operaciones;
pub(crate) mod protocolo;
mod seguridad;

pub use operaciones::ejecutar;
pub(crate) use operaciones::VARIABLE_ESPACIO_TRABAJO;
#[cfg(test)]
pub(crate) use operaciones::{el_mapa_es_solo_de_la_ia, procesar_peticion};
