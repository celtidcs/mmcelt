//! Servicios de persistencia ya compuestos para la aplicación de escritorio.
//!
//! La interfaz expresa aquí lo que necesita hacer y no elige en cada botón qué adaptador de
//! disco debe construir. Las abstracciones permanecen separadas para que un consumidor o un
//! doble no tenga que implementar operaciones ajenas.

use crate::error::AppResult;
use crate::model::Proyecto;
use crate::proyectos::{PreparadorCarpetas, RepositorioDocumentos, RepositorioMapas};
use crate::textos::Idioma;
use std::path::Path;
use std::rc::Rc;

/// Capacidades de persistencia que utiliza la aplicación gráfica.
#[derive(Clone)]
pub(crate) struct ServiciosAplicacion {
    mapas: Rc<dyn RepositorioMapas>,
    carpetas: Rc<dyn PreparadorCarpetas>,
    documentos: Rc<dyn RepositorioDocumentos>,
}

impl ServiciosAplicacion {
    /// Reúne implementaciones elegidas por el punto de composición de la aplicación.
    pub(super) fn nuevo(
        mapas: Rc<dyn RepositorioMapas>,
        carpetas: Rc<dyn PreparadorCarpetas>,
        documentos: Rc<dyn RepositorioDocumentos>,
    ) -> Self {
        Self {
            mapas,
            carpetas,
            documentos,
        }
    }

    /// Compone los servicios de producción sobre el almacenamiento local.
    pub(super) fn locales() -> Self {
        let repositorio = Rc::new(RepositorioLocal);
        Self::nuevo(repositorio.clone(), repositorio.clone(), repositorio)
    }

    /// Abre y valida un mapa desde la ruta elegida.
    pub(super) fn abrir(&self, ruta: &Path) -> AppResult<Proyecto> {
        self.mapas.abrir(ruta)
    }

    /// Guarda el mapa activo en su archivo principal.
    pub(crate) fn guardar(&self, proyecto: &Proyecto, ruta: &Path) -> AppResult<()> {
        self.mapas.guardar(proyecto, ruta)
    }

    /// Prepara una carpeta elegida como espacio de trabajo.
    pub(super) fn preparar_carpeta(&self, ruta: &Path) -> AppResult<()> {
        self.carpetas.preparar_carpeta(ruta)
    }

    /// Exporta un mapa como documento Markdown para agentes.
    pub(crate) fn exportar(
        &self,
        proyecto: &Proyecto,
        ruta: &Path,
        idioma: Idioma,
    ) -> AppResult<()> {
        self.documentos.exportar(proyecto, ruta, idioma)
    }

    /// Guarda un documento auxiliar y prepara su carpeta contenedora.
    pub(super) fn guardar_documento(&self, ruta: &Path, contenido: &[u8]) -> AppResult<()> {
        self.documentos.guardar_documento(ruta, contenido)
    }
}

/// Adaptador de producción, privado al punto donde se compone la aplicación.
#[derive(Debug, Clone, Copy, Default)]
struct RepositorioLocal;

impl RepositorioMapas for RepositorioLocal {
    fn abrir(&self, ruta: &Path) -> AppResult<Proyecto> {
        crate::storage::cargar_proyecto_de_archivo(ruta)
    }

    fn guardar(&self, proyecto: &Proyecto, ruta: &Path) -> AppResult<()> {
        crate::storage::guardar_proyecto_en_archivo(proyecto, ruta)
    }
}

impl PreparadorCarpetas for RepositorioLocal {
    fn preparar_carpeta(&self, ruta: &Path) -> AppResult<()> {
        crate::storage::crear_carpeta(ruta)
    }
}

impl RepositorioDocumentos for RepositorioLocal {
    fn exportar(&self, proyecto: &Proyecto, ruta: &Path, idioma: Idioma) -> AppResult<()> {
        crate::storage::exportar_proyecto_a_markdown(proyecto, ruta, idioma)
    }

    fn guardar_documento(&self, ruta: &Path, contenido: &[u8]) -> AppResult<()> {
        let carpeta = ruta
            .parent()
            .ok_or(crate::error::AppError::RutaFueraDelProyecto)?;
        crate::storage::crear_carpeta(carpeta)?;
        crate::storage::escribir_de_forma_atomica(ruta, contenido)
    }
}
