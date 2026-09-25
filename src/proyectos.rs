//! Casos de uso de persistencia que separan la interfaz gráfica del disco.
//!
//! La interfaz elige rutas mediante los diálogos nativos y presenta el resultado. Este módulo
//! decide cómo se abre, guarda o exporta un mapa y cómo se preparan sus carpetas. Así, el mismo
//! recorrido puede probarse con un repositorio en memoria sin abrir ventanas ni crear archivos.

#[cfg(test)]
use crate::error::AppError;
use crate::error::AppResult;
use crate::model::Proyecto;
use crate::textos::Idioma;
use std::path::Path;

/// Lectura y escritura del archivo principal de un mapa.
pub trait RepositorioMapas {
    /// Abre y valida un mapa almacenado.
    fn abrir(&self, ruta: &Path) -> AppResult<Proyecto>;

    /// Guarda un mapa de forma segura.
    fn guardar(&self, proyecto: &Proyecto, ruta: &Path) -> AppResult<()>;
}

/// Preparación de carpetas elegidas como espacio de trabajo.
pub trait PreparadorCarpetas {
    /// Crea una carpeta de trabajo, incluidas las carpetas intermedias necesarias.
    fn preparar_carpeta(&self, ruta: &Path) -> AppResult<()>;
}

/// Escritura de documentos derivados o auxiliares de un mapa.
pub trait RepositorioDocumentos {
    /// Exporta el mapa al formato de intercambio con agentes.
    fn exportar(&self, proyecto: &Proyecto, ruta: &Path, idioma: Idioma) -> AppResult<()>;

    /// Guarda un documento perteneciente al proyecto y prepara su carpeta.
    fn guardar_documento(&self, ruta: &Path, contenido: &[u8]) -> AppResult<()>;
}

/// Lee bytes para el vigilante sin devolver esa responsabilidad a la capa gráfica.
pub fn leer_bytes(ruta: &Path) -> AppResult<Vec<u8>> {
    crate::storage::leer_bytes(ruta)
}

/// Calcula la huella de un archivo mediante la capa de persistencia.
pub fn huella(ruta: &Path) -> Option<u64> {
    crate::storage::huella_del_archivo(ruta)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[derive(Default)]
    struct RepositorioEnMemoria {
        mapas: RefCell<HashMap<PathBuf, Proyecto>>,
        carpetas: RefCell<Vec<PathBuf>>,
    }

    impl RepositorioMapas for RepositorioEnMemoria {
        fn abrir(&self, ruta: &Path) -> AppResult<Proyecto> {
            self.mapas.borrow().get(ruta).cloned().ok_or_else(|| {
                AppError::lectura(
                    ruta,
                    std::io::Error::new(std::io::ErrorKind::NotFound, "mapa inexistente"),
                )
            })
        }

        fn guardar(&self, proyecto: &Proyecto, ruta: &Path) -> AppResult<()> {
            self.mapas
                .borrow_mut()
                .insert(ruta.to_path_buf(), proyecto.clone());
            Ok(())
        }
    }

    impl PreparadorCarpetas for RepositorioEnMemoria {
        fn preparar_carpeta(&self, ruta: &Path) -> AppResult<()> {
            self.carpetas.borrow_mut().push(ruta.to_path_buf());
            Ok(())
        }
    }

    #[test]
    fn proyectos_abre_guarda_y_propaga_errores_sin_tocar_el_disco() {
        let repositorio = RepositorioEnMemoria::default();
        let ruta = Path::new("memoria/proyecto.mmcelt");
        let proyecto = Proyecto::nuevo_vacio("Mapa en memoria");

        repositorio
            .guardar(&proyecto, ruta)
            .expect("guardar en memoria");
        let abierto = repositorio.abrir(ruta).expect("abrir desde memoria");
        assert_eq!(abierto.title, "Mapa en memoria");

        let carpeta = Path::new("memoria/proyecto");
        repositorio
            .preparar_carpeta(carpeta)
            .expect("preparar la carpeta lógica");
        assert_eq!(repositorio.carpetas.borrow().as_slice(), [carpeta]);

        let error = repositorio
            .abrir(Path::new("memoria/ausente.mmcelt"))
            .expect_err("el error del repositorio no debe ocultarse");
        assert!(matches!(error, AppError::Lectura { .. }));
    }
}
