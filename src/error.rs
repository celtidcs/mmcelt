//! # Módulo de Manejo Centralizado de Errores (`error.rs`)
//!
//! Define el tipo de error único de MMCelt, [`AppError`], y su alias de resultado
//! [`AppResult`].
//!
//! ## Por qué existe este módulo
//!
//! Antes de su introducción, todo el proyecto señalaba los fallos con
//! `Result<T, String>`, componiendo el mensaje en el mismo punto donde ocurrían. Eso
//! impedía que quien recibía el error pudiera distinguir programáticamente un archivo
//! inexistente de un JSON corrupto o de un mapa mental estructuralmente inválido:
//! todo llegaba como una cadena de texto ya formateada.
//!
//! [`AppError`] recupera esa información. Cada variante identifica **una causa
//! distinta**, lleva consigo el contexto necesario para depurarla y expone un nivel de
//! [`Severidad`] que la capa de interfaz usa para decidir cómo presentarla.
//!
//! ## Cómo se usa
//!
//! Las capas internas (modelo, persistencia, puente con la IA) devuelven `AppResult<T>`
//! y propagan hacia arriba con `?`. La capa de interfaz (`ui/`) es **el único lugar**
//! donde el error se convierte en un mensaje visible para el usuario, mediante
//! [`AppError::mensaje_usuario`].
//!
//! ```ignore
//! use crate::error::{AppError, AppResult};
//!
//! fn cargar(ruta: &Path) -> AppResult<Proyecto> {
//!     let contenido = std::fs::read_to_string(ruta)
//!         .map_err(|e| AppError::lectura(ruta, e))?;
//!     // ...
//! }
//! ```

use std::fmt;
use std::path::{Path, PathBuf};

/// Nivel de gravedad de un error, usado por la interfaz para decidir cómo mostrarlo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severidad {
    /// El usuario puede seguir trabajando; solo se le informa de lo ocurrido.
    Aviso,
    /// La operación solicitada no se pudo completar, pero la aplicación sigue estable.
    Error,
    /// Los datos implicados podrían dañar el estado de la aplicación si se usaran.
    Critica,
}

impl Severidad {
    /// Devuelve el emoji con el que se antepone el mensaje en la barra de estado.
    pub fn emoji(&self) -> &'static str {
        match self {
            Severidad::Aviso => "⚠️",
            Severidad::Error => "❌",
            Severidad::Critica => "🛑",
        }
    }
}

/// Motivo concreto por el que un mapa mental no supera la validación estructural.
///
/// Se separa de [`AppError`] porque la validación necesita señalar **qué** nodo
/// incumple la garantía, dato imprescindible para que el usuario o una IA puedan
/// reparar el archivo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FalloEstructural {
    /// El `root_id` del proyecto no corresponde a ningún nodo del mapa `nodes`.
    RaizInexistente { root_id: String },
    /// Un nodo referencia como hijo un identificador que no existe.
    HijoInexistente { padre: String, hijo: String },
    /// Un nodo referencia como padre un identificador que no existe.
    PadreInexistente { nodo: String, padre: String },
    /// Se alcanzó el mismo nodo dos veces: la estructura contiene un ciclo.
    CicloDetectado { nodo: String },
    /// Existen nodos que no se alcanzan descendiendo desde la raíz.
    NodosHuerfanos { cantidad: usize, ejemplo: String },
    /// La profundidad del árbol supera el límite admitido.
    ProfundidadExcesiva { limite: usize },
    /// El número de nodos supera el límite admitido.
    DemasiadosNodos { cantidad: usize, limite: usize },
    /// Un nodo tiene una coordenada que no es un número finito.
    ///
    /// Se rechaza en la frontera porque un infinito o un `NaN` **no se pueden volver a
    /// leer**: al serializar salen como `null`, y el archivo queda ilegible para siempre.
    PosicionNoFinita { nodo: String },
    /// El nodo raíz declara un padre, lo que rompe la definición de raíz del árbol.
    RaizConPadre { root_id: String, padre: String },
    /// Un nodo declara un padre, pero ese padre no lo incluye en su lista de hijos.
    PadreNoReciproco { nodo: String, padre: String },
    /// Un padre declara un hijo, pero el hijo no apunta a ese padre en su `parent_id`.
    HijoNoReciproco { padre: String, hijo: String },
    /// Una conexión cruzada declara como origen un nodo que no existe.
    ConexionConOrigenInexistente { conexion: String, origen: String },
    /// Una conexión cruzada declara como destino un nodo que no existe.
    ConexionConDestinoInexistente { conexion: String, destino: String },
    /// Un campo de texto supera la longitud máxima permitida en el modelo.
    TextoDemasiadoLargo {
        nodo: String,
        campo: &'static str,
        longitud: usize,
        limite: usize,
    },
}

impl fmt::Display for FalloEstructural {
    /// Describe el fallo estructural identificando el nodo concreto que lo provoca.
    ///
    /// El identificador es imprescindible: sin él, el usuario no puede localizar la
    /// parte dañada del archivo, ni una IA repararla.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FalloEstructural::RaizInexistente { root_id } => write!(
                f,
                "el nodo raíz declarado ({root_id}) no existe entre los nodos del mapa"
            ),
            FalloEstructural::HijoInexistente { padre, hijo } => {
                write!(f, "el nodo {padre} declara un hijo ({hijo}) que no existe")
            }
            FalloEstructural::PadreInexistente { nodo, padre } => {
                write!(f, "el nodo {nodo} declara un padre ({padre}) que no existe")
            }
            FalloEstructural::CicloDetectado { nodo } => write!(
                f,
                "el mapa contiene un ciclo: se llega dos veces al nodo {nodo}"
            ),
            FalloEstructural::NodosHuerfanos { cantidad, ejemplo } => write!(
                f,
                "hay {cantidad} nodo(s) que no cuelgan de la raíz (por ejemplo {ejemplo})"
            ),
            FalloEstructural::PosicionNoFinita { nodo } => write!(
                f,
                "El nodo {nodo} tiene una posición que no es un número válido. Un mapa con coordenadas infinitas o indefinidas no se puede volver a abrir después de guardarlo."
            ),
            FalloEstructural::ProfundidadExcesiva { limite } => write!(
                f,
                "el mapa supera la profundidad máxima admitida ({limite} niveles)"
            ),
            FalloEstructural::DemasiadosNodos { cantidad, limite } => write!(
                f,
                "el mapa tiene {cantidad} nodos y el máximo admitido es {limite}"
            ),
            FalloEstructural::RaizConPadre { root_id, padre } => write!(
                f,
                "el nodo raíz {root_id} declara tener como padre a {padre}, pero la raíz no puede tener padre"
            ),
            FalloEstructural::PadreNoReciproco { nodo, padre } => write!(
                f,
                "el nodo {nodo} declara como padre a {padre}, pero ese padre no lo incluye en su lista de hijos"
            ),
            FalloEstructural::HijoNoReciproco { padre, hijo } => write!(
                f,
                "el padre {padre} declara como hijo a {hijo}, pero el hijo no apunta a él como su padre"
            ),
            FalloEstructural::ConexionConOrigenInexistente { conexion, origen } => write!(
                f,
                "la conexión {conexion} declara como origen un nodo ({origen}) que no existe"
            ),
            FalloEstructural::ConexionConDestinoInexistente { conexion, destino } => write!(
                f,
                "la conexión {conexion} declara como destino un nodo ({destino}) que no existe"
            ),
            FalloEstructural::TextoDemasiadoLargo {
                nodo,
                campo,
                longitud,
                limite,
            } => write!(
                f,
                "el campo «{campo}» en {nodo} tiene {longitud} caracteres y supera el límite de {limite}"
            ),
        }
    }
}

/// Motivo tipado y cerrado por el que una carpeta no puede usarse como proyecto.
///
/// La traducción y presentación al usuario final se realiza exclusivamente en el módulo de textos,
/// garantizando que ningún texto libre o sin traducir viaje por la interfaz (defecto PH-0119-2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MotivoCarpetaInvalida {
    /// La ubicación es la raíz de un volumen o no es una carpeta de proyecto válida.
    RaizDeVolumenONoValida,
    /// La carpeta contiene otros proyectos en subdirectorios; cada proyecto debe conectarse en su propia carpeta.
    ContieneOtrosProyectos,
    /// El mapa reside en la raíz de una unidad; debe trasladarse a una carpeta de proyecto.
    MapaEnRaizDebeTrasladarse,
    /// El agente MCP está registrado para otra carpeta; actualiza la conexión.
    AgenteRegistradoParaOtraCarpeta,
    /// La carpeta declarada por el agente no existe.
    CarpetaAgenteNoExiste,
    /// El recibo del agente está incompleto.
    ReciboAgenteIncompleto,
    /// El ejecutable del agente está dentro de la carpeta dirigida.
    EjecutableDentroDeCarpetaDirigida { ruta_ejecutable: PathBuf },
    /// El nombre del proyecto está vacío.
    NombreVacio,
    /// La carpeta no existe o no se puede leer.
    NoExisteONoSePuedeLeer,
    /// La ubicación es demasiado amplia o no es una carpeta.
    UbicacionDemasiadoAmplia,
    /// La configuración del proyecto usa una versión no compatible.
    VersionNoCompatible,
    /// La identidad portable del proyecto está incompleta.
    IdentidadIncompleta,
    /// La nueva ubicación del proyecto todavía no está confirmada.
    UbicacionNoConfirmada,
    /// El archivo de mapa ya existe en la carpeta de destino.
    DestinoYaExiste,
    /// No se pudo mover el archivo de mapa a la carpeta de destino.
    NoSePudoMoverMapa,
    /// Error general encapsulado mediante clave tipada.
    ErrorGeneral(Box<ClaveError>),
}

/// Causa concreta por la que falló la interpretación de una respuesta de IA como JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FalloJsonIa {
    /// El texto recibido está vacío.
    TextoVacio,
    /// El JSON no contiene el campo obligatorio «root_node».
    FaltaNodoRaiz,
    /// Error de sintaxis devuelto por serde_json.
    Sintaxis(String),
}

/// Causa concreta por la que falló la interpretación de una respuesta de IA como esquema Markdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FalloEsquemaMarkdown {
    /// El texto recibido está vacío.
    TextoVacio,
    /// El texto no contiene líneas con contenido que procesar.
    SinLineasDeContenido,
    /// No se reconoció ningún título («# …») ni viñeta («- …») en el texto.
    SinTitulosNiVinetas,
    /// No se intentó porque el JSON era sintácticamente válido pero incompleto.
    NoIntentadoPorJsonIncompleto,
}

/// Campo de entrada validado en la configuración o sesión de agentes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampoEntrada {
    /// Archivo de instrucciones nativas.
    FuenteNativa,
    /// Archivo portable de configuración de agentes.
    ConfiguracionAgentes,
    /// Contrato de MMCelt.
    ContratoMmcelt,
    /// Texto del encargo del usuario.
    Encargo,
    /// Texto de las reglas comunes del proyecto.
    Reglas,
    /// Texto del contexto del mapa.
    Contexto,
}

/// Motivo tipado por el que un campo de entrada es rechazado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotivoEntradaInvalida {
    /// El nombre de archivo no pertenece al catálogo de fuentes nativas permitidas.
    NombreNoPermitido,
    /// La versión del formato de configuración no es compatible.
    VersionNoCompatible,
    /// Contiene una fuente de instrucciones no reconocida.
    FuenteNoReconocida,
    /// La huella criptográfica de la fuente no coincide con su contenido.
    HuellaNoCorresponde,
    /// Contiene caracteres de control invisibles no admitidos.
    CaracteresDeControlInvisibles,
}

/// Clave tipada que identifica la causa de un error para su presentación multilingüe al usuario.
///
/// Sustituye a la composición de cadenas de texto libre en [`AppError::mensaje_usuario`].
/// La resolución a texto visible en los seis idiomas oficiales se realiza en el catálogo de textos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaveError {
    /// No se pudo leer un archivo.
    Lectura { ruta: PathBuf, origen: String },
    /// No se pudo guardar en una carpeta o archivo.
    Escritura { ruta: PathBuf, origen: String },
    /// El archivo no tiene el formato esperado.
    Formato { contexto: String, origen: String },
    /// El mapa mental está dañado estructuralmente.
    EstructuraInvalida(FalloEstructural),
    /// No se pudo interpretar la respuesta de la IA (JSON ni esquema Markdown).
    ImportacionIa {
        error_json: FalloJsonIa,
        error_esquema: FalloEsquemaMarkdown,
    },
    /// La carpeta no puede usarse como proyecto.
    CarpetaInvalida {
        ruta: PathBuf,
        motivo: MotivoCarpetaInvalida,
    },
    /// La entrega pertenece a otro proyecto y no se aplicará al mapa activo.
    IdentidadProyectoDistinta,
    /// La operación se ha detenido porque la ruta sale de la carpeta del proyecto.
    RutaFueraDelProyecto,
    /// La entrada supera el límite de seguridad.
    EntradaDemasiadoGrande { origen: String, limite: u64 },
    /// Un campo no puede utilizarse por un motivo concreto.
    EntradaInvalida {
        campo: CampoEntrada,
        motivo: MotivoEntradaInvalida,
    },
    /// La vista previa ha caducado.
    ConfirmacionCaducada,
    /// No se pudo vigilar la carpeta del proyecto.
    Vigilancia { ruta: PathBuf, origen: String },
}

/// Error único de la aplicación MMCelt.
///
/// Sustituye al antiguo `Result<T, String>` en todas las capas internas. Cada variante
/// representa una causa distinta, de modo que quien la recibe puede reaccionar de
/// forma diferenciada en lugar de limitarse a mostrar un texto.
#[derive(Debug)]
pub enum AppError {
    /// No se pudo leer un archivo del disco.
    Lectura {
        ruta: PathBuf,
        origen: std::io::Error,
    },
    /// No se pudo escribir un archivo en el disco.
    Escritura {
        ruta: PathBuf,
        origen: std::io::Error,
    },
    /// El contenido no es un JSON válido, o no encaja con el formato de MMCelt.
    ///
    /// Incluye el caso de un valor de enumeración desconocido, que es el motivo más
    /// habitual cuando el archivo lo ha generado un modelo de IA.
    Formato {
        contexto: String,
        origen: serde_json::Error,
    },
    /// El JSON es correcto, pero el mapa mental que describe no es un árbol válido.
    ///
    /// Este error es la razón de ser de la validación estructural: los recorridos de
    /// `layout.rs` y `model.rs` dan por supuestas esas garantías, y su incumplimiento
    /// congelaba o mataba el proceso.
    EstructuraInvalida { fallo: FalloEstructural },
    /// No se pudo interpretar el texto pegado desde un modelo de IA.
    ///
    /// Conserva por separado el error del análisis JSON y el del análisis del esquema
    /// Markdown de respaldo, porque saber cuál de los dos falló y por qué es lo que
    /// permite al usuario corregir su entrada.
    ImportacionIa {
        error_json: FalloJsonIa,
        error_esquema: FalloEsquemaMarkdown,
    },
    /// La carpeta elegida no puede actuar como raíz segura de un proyecto.
    CarpetaProyectoInvalida {
        /// Ruta concreta que se intentó usar como raíz del proyecto.
        ruta: PathBuf,
        /// Motivo por el que no es válida.
        motivo: MotivoCarpetaInvalida,
    },
    /// La identidad recibida no corresponde al proyecto activo.
    IdentidadProyectoDistinta,
    /// Una ruta intentaba salir de la carpeta autorizada.
    RutaFueraDelProyecto,
    /// Una entrada supera el tamaño que la aplicación puede procesar con seguridad.
    EntradaDemasiadoGrande {
        /// Archivo o canal del que procedían los datos.
        origen: String,
        /// Número máximo de bytes admitido.
        limite: u64,
    },
    /// Un texto introducido no puede incorporarse de forma segura a la operación solicitada.
    EntradaInvalida {
        /// Campo que debe corregirse.
        campo: CampoEntrada,
        /// Motivo concreto por el que fue rechazado.
        motivo: MotivoEntradaInvalida,
    },
    /// La vista revisada dejó de representar el mapa o sus instrucciones actuales.
    ConfirmacionCaducada,
    /// El backend del sistema operativo no pudo crear o registrar un vigilante de archivos.
    Vigilancia {
        /// Archivo o carpeta cuya vigilancia se estaba preparando.
        ruta: PathBuf,
        /// Error original devuelto por `notify`.
        origen: notify::Error,
    },
}

impl AppError {
    /// Construye un error de lectura a partir de la ruta y el error de E/S original.
    ///
    /// # Parámetros
    /// - `ruta`: el archivo o la carpeta que no se pudo leer. Aparece en el mensaje que ve
    ///   el usuario, así que conviene que sea la ruta que él reconocería.
    /// - `origen`: el error que devolvió el sistema operativo, que se conserva para poder
    ///   distinguir después un «no existe» de un «no tienes permiso».
    pub fn lectura(ruta: impl Into<PathBuf>, origen: std::io::Error) -> Self {
        AppError::Lectura {
            ruta: ruta.into(),
            origen,
        }
    }

    /// Construye un error de escritura a partir de la ruta y el error de E/S original.
    ///
    /// # Parámetros
    /// - `ruta`: el archivo o la carpeta donde falló la escritura.
    /// - `origen`: el error del sistema operativo, que se conserva íntegro.
    pub fn escritura(ruta: impl Into<PathBuf>, origen: std::io::Error) -> Self {
        AppError::Escritura {
            ruta: ruta.into(),
            origen,
        }
    }

    /// Construye un error de formato indicando en qué operación se produjo.
    ///
    /// # Parámetros
    /// - `contexto`: qué se estaba haciendo, en palabras que el usuario entienda
    ///   («importar el mapa de la IA»). No es el nombre de la función.
    /// - `origen`: el error de `serde_json`, que indica la línea y la columna del fallo.
    pub fn formato(contexto: impl Into<String>, origen: serde_json::Error) -> Self {
        AppError::Formato {
            contexto: contexto.into(),
            origen,
        }
    }

    /// Construye un error de estructura inválida.
    ///
    /// # Parámetros
    /// - `fallo`: el defecto concreto que encontró la validación, ya clasificado. Es lo
    ///   que permite explicarle al usuario qué tiene mal el archivo en lugar de decirle
    ///   solo que no se puede abrir.
    pub fn estructura(fallo: FalloEstructural) -> Self {
        AppError::EstructuraInvalida { fallo }
    }

    /// Gravedad del error, que determina cómo lo presenta la interfaz.
    pub fn severidad(&self) -> Severidad {
        match self {
            // Un archivo estructuralmente inválido es crítico: usarlo puede congelar
            // o matar la aplicación, que es exactamente lo que la validación evita.
            AppError::EstructuraInvalida { .. } => Severidad::Critica,
            AppError::Lectura { .. } | AppError::Escritura { .. } | AppError::Formato { .. } => {
                Severidad::Error
            }
            AppError::ImportacionIa { .. } => Severidad::Aviso,
            AppError::CarpetaProyectoInvalida { .. }
            | AppError::IdentidadProyectoDistinta
            | AppError::RutaFueraDelProyecto
            | AppError::EntradaDemasiadoGrande { .. }
            | AppError::EntradaInvalida { .. }
            | AppError::ConfirmacionCaducada
            | AppError::Vigilancia { .. } => Severidad::Error,
        }
    }

    /// Devuelve la clave tipada del error para su resolución multilingüe en la capa de textos.
    pub fn mensaje_usuario(&self) -> ClaveError {
        match self {
            AppError::Lectura { ruta, origen } => ClaveError::Lectura {
                ruta: ruta.clone(),
                origen: origen.to_string(),
            },
            AppError::Escritura { ruta, origen } => ClaveError::Escritura {
                ruta: ruta.clone(),
                origen: origen.to_string(),
            },
            AppError::Formato { contexto, origen } => ClaveError::Formato {
                contexto: contexto.clone(),
                origen: origen.to_string(),
            },
            AppError::EstructuraInvalida { fallo } => ClaveError::EstructuraInvalida(fallo.clone()),
            AppError::ImportacionIa {
                error_json,
                error_esquema,
            } => ClaveError::ImportacionIa {
                error_json: error_json.clone(),
                error_esquema: error_esquema.clone(),
            },
            AppError::CarpetaProyectoInvalida { ruta, motivo } => ClaveError::CarpetaInvalida {
                ruta: ruta.clone(),
                motivo: motivo.clone(),
            },
            AppError::IdentidadProyectoDistinta => ClaveError::IdentidadProyectoDistinta,
            AppError::RutaFueraDelProyecto => ClaveError::RutaFueraDelProyecto,
            AppError::EntradaDemasiadoGrande { origen, limite } => {
                ClaveError::EntradaDemasiadoGrande {
                    origen: origen.clone(),
                    limite: *limite,
                }
            }
            AppError::EntradaInvalida { campo, motivo } => ClaveError::EntradaInvalida {
                campo: *campo,
                motivo: *motivo,
            },
            AppError::ConfirmacionCaducada => ClaveError::ConfirmacionCaducada,
            AppError::Vigilancia { ruta, origen } => ClaveError::Vigilancia {
                ruta: ruta.clone(),
                origen: origen.to_string(),
            },
        }
    }
}

impl fmt::Display for AppError {
    /// Representación técnica del error, pensada para registro y depuración.
    ///
    /// Para el texto que ve el usuario final, usa [`AppError::mensaje_usuario`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Lectura { ruta, origen } => {
                write!(f, "error de lectura en {}: {origen}", ruta.display())
            }
            AppError::Escritura { ruta, origen } => {
                write!(f, "error de escritura en {}: {origen}", ruta.display())
            }
            AppError::Formato { contexto, origen } => {
                write!(f, "error de formato ({contexto}): {origen}")
            }
            AppError::EstructuraInvalida { fallo } => {
                write!(f, "estructura inválida: {fallo}")
            }
            AppError::ImportacionIa {
                error_json,
                error_esquema,
            } => write!(
                f,
                "importación desde IA fallida (json: {error_json:?}; esquema: {error_esquema:?})"
            ),
            AppError::CarpetaProyectoInvalida { ruta, motivo } => {
                write!(
                    f,
                    "carpeta de proyecto inválida «{}»: {motivo:?}",
                    ruta.display()
                )
            }
            AppError::IdentidadProyectoDistinta => write!(f, "identidad de proyecto distinta"),
            AppError::RutaFueraDelProyecto => write!(f, "ruta fuera de la carpeta del proyecto"),
            AppError::EntradaDemasiadoGrande { origen, limite } => {
                write!(
                    f,
                    "entrada demasiado grande en {origen}: supera {limite} bytes"
                )
            }
            AppError::EntradaInvalida { campo, motivo } => {
                write!(f, "entrada inválida en {campo:?}: {motivo:?}")
            }
            AppError::ConfirmacionCaducada => write!(f, "confirmación de sesión caducada"),
            AppError::Vigilancia { ruta, origen } => {
                write!(f, "error de vigilancia en {}: {origen}", ruta.display())
            }
        }
    }
}

impl std::error::Error for AppError {
    /// Devuelve el error subyacente cuando lo hay, para permitir el encadenamiento.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Lectura { origen, .. } | AppError::Escritura { origen, .. } => Some(origen),
            AppError::Formato { origen, .. } => Some(origen),
            AppError::Vigilancia { origen, .. } => Some(origen),
            AppError::EstructuraInvalida { .. }
            | AppError::ImportacionIa { .. }
            | AppError::CarpetaProyectoInvalida { .. }
            | AppError::IdentidadProyectoDistinta
            | AppError::RutaFueraDelProyecto
            | AppError::EntradaDemasiadoGrande { .. }
            | AppError::EntradaInvalida { .. }
            | AppError::ConfirmacionCaducada => None,
        }
    }
}

/// Alias de resultado usado en todas las capas internas de MMCelt.
pub type AppResult<T> = Result<T, AppError>;

/// Ruta del archivo donde se registran los errores, junto al ejecutable.
const NOMBRE_ARCHIVO_REGISTRO: &str = "mmcelt-errores.log";

/// Nombre del archivo temporal con el que se comprueba si una carpeta admite escritura.
///
/// Lleva punto delante para que quede oculto en los sistemas que siguen esa convención, y
/// vive lo que tarda en crearse y borrarse.
const NOMBRE_ARCHIVO_PRUEBA_ESCRITURA: &str = ".mmcelt-prueba-escritura";

/// Registra un error en disco con marca de tiempo, para poder depurar incidencias
/// que ocurran en el equipo del usuario.
///
/// El registro es **best-effort**: si no se puede escribir (por permisos, por ejemplo),
/// la función no hace nada y no propaga el fallo. Registrar un error nunca debe
/// provocar otro error que agrave la situación.
///
/// # Parámetros
/// - `error`: el error que se desea registrar.
/// - `contexto`: descripción breve de la operación durante la que se produjo.
#[track_caller]
pub fn registrar(error: &AppError, contexto: &str) {
    use std::io::Write;

    let Some(ruta) = ruta_archivo_registro() else {
        return;
    };

    let Ok(mut archivo) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&ruta)
    else {
        return;
    };

    let descripcion = descripcion_para_registro(error, contexto);
    // Si el registro falla no hay nada que hacer: informar de un error al escribir el
    // archivo de errores exigiría escribir en el archivo de errores.
    let _ = writeln!(archivo, "{descripcion}");
}

/// Compone una entrada técnica con contexto, cadena de causas y traza cuando está habilitada.
///
/// Rust solo captura la traza si el entorno la habilita, normalmente mediante
/// `RUST_BACKTRACE=1`. Si está desactivada, el registro lo indica en lugar de inventar una.
#[track_caller]
pub(crate) fn descripcion_para_registro(error: &AppError, contexto: &str) -> String {
    use std::error::Error as _;
    use std::fmt::Write as _;

    let marca = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let ubicacion = std::panic::Location::caller();
    let mut salida = format!(
        "[{marca}] [{:?}] {contexto}: {error}\nOrigen: {}:{}:{}",
        error.severidad(),
        ubicacion.file(),
        ubicacion.line(),
        ubicacion.column()
    );
    let mut causa = error.source();
    if causa.is_some() {
        salida.push_str("\nCausas:");
    }
    let mut nivel = 1;
    while let Some(actual) = causa {
        let _ = write!(salida, "\n  {nivel}. {actual}");
        causa = actual.source();
        nivel += 1;
    }
    let traza = std::backtrace::Backtrace::capture();
    match traza.status() {
        std::backtrace::BacktraceStatus::Captured => {
            let _ = write!(salida, "\nTraza:\n{traza}");
        }
        _ => salida.push_str("\nTraza: no capturada; habilita RUST_BACKTRACE=1 para obtenerla."),
    }
    salida
}

/// Calcula la ruta del archivo de registro, junto al ejecutable de la aplicación.
///
/// Devuelve `None` si no se puede determinar la ubicación del ejecutable, en cuyo caso
/// el registro simplemente se omite.
fn ruta_archivo_registro() -> Option<PathBuf> {
    // Primero, junto al ejecutable: es lo que espera quien lleva la aplicación en un
    // lápiz de memoria, porque el registro viaja con ella.
    if let Ok(ejecutable) = std::env::current_exe() {
        if let Some(carpeta) = ejecutable.parent() {
            if carpeta_escribible(carpeta) {
                return Some(carpeta.join(NOMBRE_ARCHIVO_REGISTRO));
            }
        }
    }

    // Si esa carpeta no admite escritura —el caso de una instalación en
    // `C:\\Program Files`—, el registro se va a los datos del usuario. Sin esto la
    // escritura fallaba en silencio y la sesión entera se quedaba sin registro, que es lo
    // contrario de lo que persigue tener un archivo de errores.
    if let Some(datos) = crate::autoguardado::directorio_datos() {
        return Some(datos.join(NOMBRE_ARCHIVO_REGISTRO));
    }

    Some(std::env::temp_dir().join(NOMBRE_ARCHIVO_REGISTRO))
}

/// Comprueba si se puede escribir en una carpeta, intentándolo de verdad.
///
/// Consultar los permisos del sistema de archivos no sirve: en Windows, `readonly()` sobre
/// un directorio informa de un atributo que no tiene que ver con el permiso de escritura,
/// y con control de acceso de por medio la respuesta puede ser engañosa en cualquier
/// sistema. La única comprobación fiable es crear un archivo y borrarlo.
///
/// # Parámetros
/// - `carpeta`: la carpeta que se quiere comprobar.
///
/// # Devuelve
/// `true` si se pudo crear y borrar un archivo de prueba.
fn carpeta_escribible(carpeta: &Path) -> bool {
    let prueba = carpeta.join(NOMBRE_ARCHIVO_PRUEBA_ESCRITURA);
    match std::fs::write(&prueba, b"") {
        Ok(()) => {
            // Si el archivo de prueba no se puede borrar, la carpeta admite escritura
            // igualmente, que es lo único que se estaba comprobando.
            let _ = std::fs::remove_file(&prueba);
            true
        }
        Err(_) => false,
    }
}
