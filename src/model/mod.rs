//! Modelo de dominio de MMCelt.
//!
//! La fachada conserva `crate::model::*` como punto estable para el resto del programa. Los
//! tipos y sus operaciones propias viven en `tipos`; las invariantes estructurales se agrupan
//! en `validacion`, y lo que se le pide al buscador en `filtro_de_busqueda`. Las plantillas y el escáner están deliberadamente fuera de este directorio
//! porque dependen de textos de presentación y del sistema de archivos.

mod clasificacion;
mod conexiones;
mod filtro_de_busqueda;
mod jerarquia;
mod metadatos_externos;
mod tipos;
mod validacion;

pub use filtro_de_busqueda::*;
pub use jerarquia::*;
pub use metadatos_externos::*;
pub use tipos::*;
