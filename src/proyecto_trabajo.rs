//! Identidad estable y límite seguro de la carpeta de un proyecto.

use crate::error::{AppError, AppResult, MotivoCarpetaInvalida};
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use uuid::Uuid;

/// Versión inicial de la identidad portable de una carpeta de proyecto.
const VERSION_CONFIGURACION_PROYECTO: u32 = 1;
/// Ruta fija de la identidad dentro de la carpeta autorizada.
const RUTA_CONFIGURACION_PROYECTO: &str = ".mmcelt/configuracion.json";

/// Identificador estable de un proyecto, independiente de la carpeta que lo contiene.
///
/// El tipo evita mezclar accidentalmente la identidad del proyecto con UUID de mapas, nodos o
/// entregas. Su representación JSON sigue siendo un UUID textual para que el contrato sea portable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdProyecto(Uuid);

impl IdProyecto {
    /// Crea una identidad nueva para un proyecto recién inicializado.
    pub fn nuevo() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Proyecto validado que autoriza operaciones únicamente dentro de una raíz canónica.
///
/// Conservar juntos la identidad y el límite de rutas permite que las futuras entregas y recibos
/// comprueben ambas condiciones antes de leer o escribir en el disco.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextoProyecto {
    id: IdProyecto,
    nombre: String,
    raiz: PathBuf,
}

impl ContextoProyecto {
    /// Construye un contexto sobre una carpeta existente y concreta.
    ///
    /// Rechaza nombres vacíos, archivos, raíces de volumen y ubicaciones que no puedan
    /// canonizarse. La raíz se guarda ya canonizada para comparar rutas sin ambigüedad léxica.
    pub fn nuevo(
        id: IdProyecto,
        nombre: impl Into<String>,
        raiz: impl AsRef<Path>,
    ) -> AppResult<Self> {
        let nombre = nombre.into();
        let nombre = nombre.trim();
        let raiz_ref = raiz.as_ref();
        if nombre.is_empty() {
            return Err(AppError::CarpetaProyectoInvalida {
                ruta: raiz_ref.to_path_buf(),
                motivo: MotivoCarpetaInvalida::NombreVacio,
            });
        }

        let raiz = raiz_ref
            .canonicalize()
            .map_err(|_| AppError::CarpetaProyectoInvalida {
                ruta: raiz_ref.to_path_buf(),
                motivo: MotivoCarpetaInvalida::NoExisteONoSePuedeLeer,
            })?;
        if !raiz.is_dir() || raiz.parent().is_none() {
            return Err(AppError::CarpetaProyectoInvalida {
                ruta: raiz,
                motivo: MotivoCarpetaInvalida::UbicacionDemasiadoAmplia,
            });
        }

        Ok(Self {
            id,
            nombre: nombre.to_string(),
            raiz,
        })
    }

    /// Devuelve la identidad estable del proyecto.
    pub fn id(&self) -> IdProyecto {
        self.id
    }

    /// Devuelve el nombre presentado a la persona y a los agentes.
    pub fn nombre(&self) -> &str {
        &self.nombre
    }

    /// Devuelve la raíz canónica que limita todas las operaciones del proyecto.
    pub fn raiz(&self) -> &Path {
        &self.raiz
    }

    /// Comprueba que una identidad recibida corresponde al proyecto activo.
    ///
    /// # Errores
    ///
    /// Devuelve [`AppError::IdentidadProyectoDistinta`] cuando los identificadores no coinciden.
    pub fn comprobar_id(&self, recibida: IdProyecto) -> AppResult<()> {
        if recibida == self.id {
            Ok(())
        } else {
            Err(AppError::IdentidadProyectoDistinta)
        }
    }

    /// Resuelve una ruta relativa, incluso si su tramo final aún no existe.
    ///
    /// Rechaza rutas vacías, absolutas y cualquier componente que ascienda o introduzca una raíz.
    /// También canoniza el ancestro existente más cercano para detectar enlaces que salgan del
    /// proyecto antes de devolver el destino.
    ///
    /// # Errores
    ///
    /// Devuelve [`AppError::RutaFueraDelProyecto`] si la ruta no es relativa y descendente o si
    /// un enlace simbólico de alguno de sus ancestros conduce fuera de la raíz autorizada.
    pub fn resolver_relativa(&self, relativa: &Path) -> AppResult<PathBuf> {
        if relativa.as_os_str().is_empty()
            || relativa.is_absolute()
            || relativa.components().any(|componente| {
                matches!(
                    componente,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(AppError::RutaFueraDelProyecto);
        }

        let destino = self.raiz.join(relativa);
        if self.contiene(&destino)? {
            Ok(destino)
        } else {
            Err(AppError::RutaFueraDelProyecto)
        }
    }

    /// Indica si una ruta existente o futura permanece dentro de la raíz autorizada.
    ///
    /// # Errores
    ///
    /// Propaga el fallo del sistema de archivos si no puede inspeccionar el ancestro existente
    /// necesario para decidir con seguridad.
    pub fn contiene(&self, ruta: &Path) -> AppResult<bool> {
        let absoluta = if ruta.is_absolute() {
            ruta.to_path_buf()
        } else {
            self.raiz.join(ruta)
        };
        let comprobada = canonizar_hasta_ancestro_existente(&absoluta)?;
        Ok(comprobada.starts_with(&self.raiz))
    }
}

/// Datos persistentes que permiten reconocer el mismo proyecto después de reiniciar o moverlo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfiguracionProyecto {
    /// Versión del contrato de esta configuración.
    pub version_formato: u32,
    /// Identidad estable del proyecto.
    pub id: IdProyecto,
    /// Nombre legible derivado de la carpeta al inicializarla.
    pub nombre: String,
    /// Última raíz canónica confirmada expresamente desde MMCelt.
    pub ultima_raiz_confirmada: PathBuf,
}

impl ConfiguracionProyecto {
    /// Construye la configuración inicial a partir de un contexto ya validado.
    fn nueva(contexto: &ContextoProyecto) -> Self {
        Self {
            version_formato: VERSION_CONFIGURACION_PROYECTO,
            id: contexto.id(),
            nombre: contexto.nombre().to_string(),
            ultima_raiz_confirmada: contexto.raiz().to_path_buf(),
        }
    }

    /// Rechaza contratos desconocidos o datos básicos incompletos antes de usarlos.
    fn validar(&self) -> AppResult<()> {
        if self.version_formato != VERSION_CONFIGURACION_PROYECTO {
            return Err(AppError::CarpetaProyectoInvalida {
                ruta: self.ultima_raiz_confirmada.clone(),
                motivo: MotivoCarpetaInvalida::VersionNoCompatible,
            });
        }
        if self.nombre.trim().is_empty() || !self.ultima_raiz_confirmada.is_absolute() {
            return Err(AppError::CarpetaProyectoInvalida {
                ruta: self.ultima_raiz_confirmada.clone(),
                motivo: MotivoCarpetaInvalida::IdentidadIncompleta,
            });
        }
        Ok(())
    }
}

/// Resultado de comparar la carpeta detectada con la última ubicación confirmada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CargaProyecto {
    /// La identidad y la ubicación siguen siendo las aprobadas.
    Confirmado(ContextoProyecto),
    /// La identidad se conserva, pero la ubicación nueva aún no se ha aceptado.
    Trasladado {
        /// Configuración leída sin modificar.
        configuracion: ConfiguracionProyecto,
        /// Raíz canónica donde se encontró ahora.
        raiz_detectada: PathBuf,
    },
}

/// Persistencia atómica de la identidad portable del proyecto.
pub struct RepositorioProyecto;

impl RepositorioProyecto {
    /// Obtiene un límite seguro para una vista previa sin inicializar todavía el proyecto.
    ///
    /// Si la identidad existe, exige que siga en su raíz confirmada. Si falta —caso normal al
    /// abrir con una versión anterior— crea solo un contexto efímero en memoria. La identidad
    /// durable se escribirá mediante [`Self::abrir_o_inicializar`] después de la confirmación.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la raíz no se puede canonizar, la identidad persistida es inválida o
    /// se detecta un traslado que la persona todavía no ha confirmado.
    pub fn contexto_para_vista_previa(raiz: &Path) -> AppResult<ContextoProyecto> {
        let ruta = raiz.join(RUTA_CONFIGURACION_PROYECTO);
        if ruta.exists() {
            return match Self::cargar(raiz)? {
                CargaProyecto::Confirmado(contexto) => Ok(contexto),
                CargaProyecto::Trasladado { .. } => Err(AppError::CarpetaProyectoInvalida {
                    ruta: raiz.to_path_buf(),
                    motivo: MotivoCarpetaInvalida::UbicacionNoConfirmada,
                }),
            };
        }
        contexto_nuevo_para_raiz(raiz)
    }

    /// Abre una identidad existente o crea una sola vez la de una carpeta recién autorizada.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la carpeta no es accesible, la configuración existente es inválida o
    /// no se puede persistir atómicamente una identidad nueva.
    ///
    /// # Efectos secundarios
    ///
    /// En una carpeta nueva crea `.mmcelt/proyecto.json`; nunca confirma por sí solo un traslado.
    pub fn abrir_o_inicializar(raiz: &Path) -> AppResult<CargaProyecto> {
        let ruta = raiz.join(RUTA_CONFIGURACION_PROYECTO);
        if ruta.is_file() {
            return Self::cargar(raiz);
        }

        let contexto = contexto_nuevo_para_raiz(raiz)?;
        let configuracion = ConfiguracionProyecto::nueva(&contexto);
        Self::guardar(&configuracion, contexto.raiz())?;
        Ok(CargaProyecto::Confirmado(contexto))
    }

    /// Carga una identidad y detecta un traslado sin aprobarlo automáticamente.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la raíz o el archivo no se pueden leer, el JSON es inválido o usa una
    /// versión de contrato desconocida.
    pub fn cargar(raiz: &Path) -> AppResult<CargaProyecto> {
        let raiz_detectada =
            raiz.canonicalize()
                .map_err(|_| AppError::CarpetaProyectoInvalida {
                    ruta: raiz.to_path_buf(),
                    motivo: MotivoCarpetaInvalida::NoExisteONoSePuedeLeer,
                })?;
        let ruta = raiz_detectada.join(RUTA_CONFIGURACION_PROYECTO);
        let bytes = std::fs::read(&ruta).map_err(|error| AppError::lectura(&ruta, error))?;
        let configuracion: ConfiguracionProyecto = serde_json::from_slice(&bytes)
            .map_err(|error| AppError::formato("al leer la identidad del proyecto", error))?;
        configuracion.validar()?;

        if configuracion.ultima_raiz_confirmada == raiz_detectada {
            Ok(CargaProyecto::Confirmado(ContextoProyecto::nuevo(
                configuracion.id,
                &configuracion.nombre,
                &raiz_detectada,
            )?))
        } else {
            Ok(CargaProyecto::Trasladado {
                configuracion,
                raiz_detectada,
            })
        }
    }

    /// Confirma un traslado después de una acción explícita y conserva el UUID anterior.
    ///
    /// # Errores
    ///
    /// Devuelve un error si la nueva raíz no es válida o si la configuración actualizada no puede
    /// escribirse de forma atómica.
    ///
    /// # Efectos secundarios
    ///
    /// Actualiza `.mmcelt/proyecto.json` con la nueva raíz y conserva la identidad anterior.
    pub fn confirmar_traslado(
        mut configuracion: ConfiguracionProyecto,
        raiz_detectada: &Path,
    ) -> AppResult<ContextoProyecto> {
        let contexto =
            ContextoProyecto::nuevo(configuracion.id, &configuracion.nombre, raiz_detectada)?;
        configuracion.ultima_raiz_confirmada = contexto.raiz().to_path_buf();
        Self::guardar(&configuracion, contexto.raiz())?;
        Ok(contexto)
    }

    /// Guarda la configuración mediante la misma escritura atómica que protege los mapas.
    fn guardar(configuracion: &ConfiguracionProyecto, raiz: &Path) -> AppResult<()> {
        configuracion.validar()?;
        let contexto = ContextoProyecto::nuevo(configuracion.id, &configuracion.nombre, raiz)?;
        let ruta = contexto.resolver_relativa(Path::new(RUTA_CONFIGURACION_PROYECTO))?;
        let carpeta = ruta.parent().ok_or(AppError::RutaFueraDelProyecto)?;
        std::fs::create_dir_all(carpeta).map_err(|error| AppError::escritura(carpeta, error))?;
        let contenido = serde_json::to_vec_pretty(configuracion)
            .map_err(|error| AppError::formato("al guardar la identidad del proyecto", error))?;
        crate::storage::escribir_de_forma_atomica(&ruta, &contenido)
    }
}

/// Construye la identidad efímera o inicial usando el mismo nombre estable de carpeta.
fn contexto_nuevo_para_raiz(raiz: &Path) -> AppResult<ContextoProyecto> {
    let nombre = raiz
        .file_name()
        .and_then(|valor| valor.to_str())
        .map(str::trim)
        .filter(|valor| !valor.is_empty())
        .unwrap_or("Proyecto MMCelt");
    ContextoProyecto::nuevo(IdProyecto::nuevo(), nombre, raiz)
}

/// Canoniza una ruta futura conservando los tramos que todavía no existen.
fn canonizar_hasta_ancestro_existente(ruta: &Path) -> AppResult<PathBuf> {
    let mut ancestro = ruta;
    let mut pendientes = Vec::new();
    while !ancestro.exists() {
        let nombre = ancestro.file_name().ok_or(AppError::RutaFueraDelProyecto)?;
        pendientes.push(nombre.to_os_string());
        ancestro = ancestro.parent().ok_or(AppError::RutaFueraDelProyecto)?;
    }

    let mut resultado = ancestro
        .canonicalize()
        .map_err(|_| AppError::RutaFueraDelProyecto)?;
    for nombre in pendientes.iter().rev() {
        resultado.push(nombre);
    }
    Ok(resultado)
}
