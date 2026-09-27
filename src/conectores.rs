//! # Módulo de Conexión con Agentes de IA (`conectores.rs`)
//!
//! Detecta qué agentes de IA hay instalados en el equipo y registra MMCelt en ellos como
//! servidor MCP, sin que el usuario tenga que editar ningún archivo a mano.
//!
//! Es lo que hay detrás de la opción de menú «Conectar con mis IAs».
//!
//! ## Por qué esto vive dentro de la aplicación
//!
//! Hubo dos scripts (`instalar_mcp.ps1` y `.sh`) que hacían lo mismo desde la terminal.
//! Se retiraron, y no solo por redundantes: mantenían una segunda copia de esta lógica que
//! acabó divergiendo de la de aquí, hasta el punto de no escapar las cadenas que escriben
//! en los archivos TOML y de conocer una lista de agentes distinta.
//!
//! Además nunca sirvieron para el caso que importa: **pasarle MMCelt a otra persona**.
//!
//! Esa persona no va a abrir PowerShell. Y si instala un agente nuevo la semana que
//! viene, tendría que acordarse de volver a ejecutar un script. Dentro de la aplicación,
//! la detección ocurre **cada vez que abre la ventana**, así que lo que se instaló
//! después aparece solo.
//!
//! ## Estos archivos se escriben de forma atómica
//!
//! Todo lo que este módulo escribe son archivos de configuración **que ya existen y que no
//! son suyos**: el `.mcp.json` de Claude o el `config.toml` de Codex, donde el usuario puede
//! tener declarados otros servidores MCP. Y no se escriben de cero: se leen, se les añade o
//! se les quita la sección de MMCelt, y se vuelven a escribir enteros.
//!
//! Por eso pasan todos por [`crate::storage::escribir_de_forma_atomica`] y no por
//! `fs::write`. Este último trunca el archivo a cero antes de empezar a escribir, así que un
//! corte de luz o un cierre forzado en esa ventana deja al usuario sin la configuración de
//! sus agentes —no solo sin la de MMCelt—. Escribir al lado y renombrar encima elimina esa
//! ventana: quien lea el archivo verá el contenido viejo entero o el nuevo entero.
//!
//! La copia previa de [`crate::respaldo`] sigue haciendo falta y no sobra: protege del error
//! humano y de una escritura correcta pero indeseada, que es otro problema distinto.
//!
//! ## Lo que se registra es el propio ejecutable
//!
//! El comando que se escribe en la configuración de cada agente es
//! `<ruta del ejecutable> --mcp-server` (ver [`crate::mcp_server`]).
//!
//! Esto es lo que hace la conexión realmente portable: no hace falta que el equipo de
//! destino tenga ninguna otra cosa instalada. Si el usuario puede ejecutar MMCelt,
//! puede conectarlo con su agente.
//!
//! ## Cada agente guarda su configuración en un sitio distinto
//!
//! Y no siempre donde uno esperaría:
//!
//! | Agente | Ubicación | Formato |
//! |---|---|---|
//! | Claude Code | `~/.claude.json` | JSON |
//! | Codex CLI | `~/.codex/config.toml` | **TOML** |
//! | Gemini CLI | `~/.gemini/settings.json` | JSON |
//!
//! ## Precauciones
//!
//! Estos archivos pertenecen a **otros programas** y suelen contener ajustes que costó
//! trabajo dejar bien. El de Claude Code, por ejemplo, guarda además el historial y las
//! preferencias del usuario. Por eso:
//!
//! - Siempre se hace **copia de seguridad** con marca de tiempo antes de modificar.
//! - Se **conserva todo lo demás**: los otros servidores MCP y el resto de claves.
//! - Un archivo con contenido ilegible **se deja intacto** y se informa del problema, en
//!   lugar de sobrescribirlo.

use crate::error::{AppError, AppResult, MotivoCarpetaInvalida};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Nombre con el que MMCelt se registra en los agentes.
const NOMBRE_SERVIDOR: &str = "mmcelt";

/// Argumento que arranca el ejecutable en modo servidor MCP.
const ARGUMENTO_SERVIDOR: &str = "--mcp-server";

/// Variable de entorno que delimita dónde puede operar el agente.
///
/// Se toma del servidor, que es quien la lee: son los dos extremos del mismo acuerdo y no
/// pueden desincronizarse.
use crate::mcp_server::VARIABLE_ESPACIO_TRABAJO;

/// Formato del archivo de configuración de un agente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatoConfiguracion {
    /// Estructura `{ "mcpServers": { … } }`. La usan casi todos.
    Json,
    /// Secciones `[mcp_servers.nombre]`. Solo la usa Codex.
    Toml,
}

/// Identidad estable de una aplicación o consola capaz de trabajar con MMCelt.
///
/// El nombre visible se traduce y puede cambiar; esta identidad es el contrato que permite
/// elegir un adaptador sin comparar textos ni confundir productos del mismo proveedor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdAgente {
    /// Consola oficial de Anthropic.
    ClaudeCode,
    /// Identidad histórica de Claude Desktop; ya no se ofrece ni se configura.
    ClaudeDesktop,
    /// Identidad histórica de Antigravity; ya no se ofrece ni se configura.
    Antigravity,
    /// Identidad histórica de Cursor; ya no se ofrece ni se configura.
    Cursor,
    /// Identidad histórica de Windsurf; ya no se ofrece ni se configura.
    Windsurf,
    /// Consola oficial de OpenAI.
    CodexCli,
    /// Consola oficial de Google.
    GeminiCli,
}

impl IdAgente {
    /// Posición estable en que se muestran los agentes detectados.
    ///
    /// La detección futura de consolas añadirá candidatos desde otra fuente. Ordenar por la
    /// identidad evita que el menú cambie según el orden en que el sistema devuelva archivos o
    /// ejecutables.
    fn orden(self) -> u8 {
        match self {
            Self::ClaudeCode => 0,
            Self::CodexCli => 1,
            Self::GeminiCli => 2,
            Self::ClaudeDesktop | Self::Antigravity | Self::Cursor | Self::Windsurf => u8::MAX,
        }
    }
}

/// Capacidad comprobada de abrir una consola oficial instalada.
///
/// Su presencia no implica que MCP esté registrado: ambas capacidades se detectan y se
/// presentan por separado para no prometer un recorrido que el equipo no puede ejecutar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapacidadCli {
    /// Ruta concreta localizada en `PATH`, pasada directamente a `Command`.
    pub ejecutable: PathBuf,
}

/// Capacidades efectivas que «Enviar a...» puede ofrecer para un agente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisponibilidadEnvio {
    /// No hay consola localizada ni servidor MCP utilizable.
    NoDisponible,
    /// Hay consola interactiva, pero falta registrar MCP para devolver el mapa.
    SoloConsola,
    /// MCP está registrado, pero MMCelt no puede abrir una consola de este agente.
    SoloMcp,
    /// Están disponibles la conversación por consola y el retorno estructurado por MCP.
    ConsolaYMcp,
}

/// Un agente de IA detectado en el equipo.
#[derive(Debug, Clone)]
pub struct Agente {
    /// Identidad estable usada por los adaptadores, independiente del rótulo visible.
    pub id: IdAgente,

    /// Nombre visible del agente.
    pub nombre: &'static str,

    /// Explicación breve de qué es, para quien no lo conozca.
    pub descripcion: &'static str,

    /// Archivo de configuración donde se registra el servidor.
    pub ruta_configuracion: PathBuf,

    /// Formato de ese archivo.
    pub formato: FormatoConfiguracion,

    /// Si MMCelt ya figura registrado en él.
    pub conectado: bool,

    /// Consola oficial localizada, si este agente ofrece una y está instalada.
    pub cli: Option<CapacidadCli>,

    /// Carpeta de trabajo que quedó escrita en su configuración, si se puede leer.
    ///
    /// Es el dato que faltaba para saber si lo que anuncia la ventana coincide con lo que
    /// el agente va a obedecer: la carpeta viaja **dentro** del archivo del agente, así que
    /// cambiarla en MMCelt no la cambia allí. Ver [`estado_de_la_conexion`].
    ///
    /// `None` cuando no hay entrada, no se puede leer o la entrada no declara la variable
    /// —por ejemplo, si la escribió una versión anterior de MMCelt o se editó a mano—.
    pub espacio_registrado: Option<String>,

    /// Si el archivo de configuración ya existe físicamente en el disco.
    pub existe_configuracion: bool,
}

impl Agente {
    /// Indica si el archivo de configuración existe ya en el disco.
    ///
    /// Se consulta el valor determinado durante la detección para no realizar
    /// lecturas de disco por fotograma en el bucle de dibujado de la interfaz.
    pub fn tiene_configuracion(&self) -> bool {
        self.existe_configuracion
    }

    /// Resume por separado las dos vías necesarias para el recorrido bidireccional.
    ///
    /// # Devuelve
    /// La combinación real de consola y MCP detectada para este agente.
    pub fn disponibilidad_de_envio(&self) -> DisponibilidadEnvio {
        match (self.cli.is_some(), self.conectado) {
            (false, false) => DisponibilidadEnvio::NoDisponible,
            (true, false) => DisponibilidadEnvio::SoloConsola,
            (false, true) => DisponibilidadEnvio::SoloMcp,
            (true, true) => DisponibilidadEnvio::ConsolaYMcp,
        }
    }
}

/// Devuelve la carpeta personal del usuario.
///
/// # Devuelve
/// La ruta, o `None` si no se puede determinar, en cuyo caso no se detectará ningún
/// agente y la ventana lo indicará.
fn carpeta_personal() -> Option<PathBuf> {
    std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from)
}

/// Busca los agentes de IA instalados en el equipo.
///
/// La detección se basa en la presencia de la carpeta de configuración de cada agente,
/// que es lo que sobrevive con independencia de dónde se haya instalado el programa.
///
/// **Se ejecuta cada vez que se abre la ventana de conexión**, de modo que un agente
/// instalado después aparece sin necesidad de reiniciar MMCelt.
///
/// # Devuelve
/// Los agentes encontrados, cada uno con su estado de conexión ya comprobado. La lista
/// está vacía si no hay ninguno.
pub fn detectar_agentes() -> Vec<Agente> {
    let Some(personal) = carpeta_personal() else {
        return Vec::new();
    };
    let path = std::env::var_os("PATH").unwrap_or_default();
    let pathext = std::env::var_os("PATHEXT").unwrap_or_default();
    detectar_agentes_en_con_entorno(&personal, &path, &pathext)
}

/// Busca agentes bajo una carpeta personal ya determinada.
///
/// Separar la fuente de la ruta permite comprobar la detección completa con una carpeta aislada,
/// sin cambiar `USERPROFILE`/`HOME` del proceso ni consultar configuraciones reales del usuario.
///
/// # Parámetros
/// - `personal`: carpeta que desempeña el papel de directorio personal durante la búsqueda.
///
/// # Devuelve
/// Los agentes cuyas carpetas o configuraciones existen dentro de esa raíz.
/// Detecta configuraciones y consolas usando un entorno suministrado expresamente.
pub(crate) fn detectar_agentes_en_con_entorno(
    personal: &Path,
    path: &OsStr,
    pathext: &OsStr,
) -> Vec<Agente> {
    // (identidad, nombre, descripción, carpeta que delata la instalación,
    //  archivo de configuración, formato)
    let candidatos: Vec<(IdAgente, &str, &str, PathBuf, PathBuf, FormatoConfiguracion)> = vec![
        (
            IdAgente::ClaudeCode,
            "Claude Code",
            "Asistente de programación de Anthropic en la terminal",
            personal.join(".claude"),
            personal.join(".claude.json"),
            FormatoConfiguracion::Json,
        ),
        (
            IdAgente::CodexCli,
            "Codex CLI",
            "Agente de programación de OpenAI en la terminal",
            personal.join(".codex"),
            personal.join(".codex/config.toml"),
            FormatoConfiguracion::Toml,
        ),
        (
            IdAgente::GeminiCli,
            "Gemini CLI",
            "Agente de Google en una consola interactiva",
            personal.join(".gemini"),
            personal.join(".gemini/settings.json"),
            FormatoConfiguracion::Json,
        ),
    ];

    let mut agentes: Vec<_> = candidatos
        .into_iter()
        .filter_map(
            |(id, nombre, descripcion, carpeta, ruta_configuracion, formato)| {
                let ejecutable = match id {
                    IdAgente::ClaudeCode => buscar_ejecutable("claude", path, pathext),
                    IdAgente::CodexCli => buscar_ejecutable("codex", path, pathext),
                    IdAgente::GeminiCli => buscar_ejecutable("gemini", path, pathext),
                    _ => None,
                };
                let conectado = esta_conectado(&ruta_configuracion, formato);
                let espacio_registrado = espacio_registrado(&ruta_configuracion, formato);
                let existe_configuracion = ruta_configuracion.exists();
                (carpeta.exists() || existe_configuracion || ejecutable.is_some()).then(|| Agente {
                    id,
                    nombre,
                    descripcion,
                    ruta_configuracion,
                    formato,
                    conectado,
                    cli: ejecutable.map(|ejecutable| CapacidadCli { ejecutable }),
                    espacio_registrado,
                    existe_configuracion,
                })
            },
        )
        .collect();
    agentes.sort_by_key(|agente| agente.id.orden());
    agentes
}

/// Busca un ejecutable por las entradas separadas de `PATH` y, en Windows, `PATHEXT`.
pub(crate) fn buscar_ejecutable(nombre: &str, path: &OsStr, pathext: &OsStr) -> Option<PathBuf> {
    let nombres = nombres_ejecutables_candidatos(nombre, pathext);
    std::env::split_paths(path)
        .flat_map(|carpeta| nombres.iter().map(move |nombre| carpeta.join(nombre)))
        .find(|candidato| es_archivo_ejecutable(candidato))
}

/// Genera nombres físicos sin evaluar contenido como una orden.
fn nombres_ejecutables_candidatos(nombre: &str, pathext: &OsStr) -> Vec<String> {
    #[cfg(windows)]
    {
        if Path::new(nombre).extension().is_some() {
            return vec![nombre.to_string()];
        }
        pathext
            .to_string_lossy()
            .split(';')
            .map(str::trim)
            .filter(|extension| !extension.is_empty())
            .map(|extension| format!("{nombre}{extension}"))
            .collect()
    }
    #[cfg(not(windows))]
    {
        let _ = pathext;
        vec![nombre.to_string()]
    }
}

/// Comprueba el tipo de archivo y el permiso ejecutable cuando el sistema lo representa.
fn es_archivo_ejecutable(ruta: &Path) -> bool {
    if !ruta.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(ruta)
            .map(|metadatos| metadatos.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// En qué situación está un agente respecto a la carpeta de trabajo que se está mostrando.
///
/// Antes esto era un `bool`, y por eso el programa podía decir «Conectado» mientras la
/// configuración del agente autorizaba una carpeta distinta de la que la ventana acababa de
/// prometer. Son tres situaciones con tres acciones distintas, así que son tres valores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstadoDeLaConexion {
    /// MMCelt no figura en su configuración, o la entrada no puede arrancar el servidor.
    SinConectar,

    /// Registrado, y con la misma carpeta de trabajo que se está mostrando.
    Conectado,

    /// Registrado, pero **no** con la carpeta que se está mostrando.
    ///
    /// Cubre también el caso de que la entrada no declare ninguna carpeta legible: en las
    /// dos situaciones lo que el agente obedecerá no es lo que la ventana promete, y la
    /// reparación es la misma —volver a registrarlo—.
    ConectadoAOtraCarpeta,
}

/// Compara lo que el agente tiene registrado con la carpeta que MMCelt está mostrando.
///
/// # Parámetros
/// - `agente`: el agente detectado, con su estado de conexión ya leído del disco.
/// - `espacio_en_uso`: la carpeta que la ventana está proponiendo ahora mismo.
///
/// # Devuelve
/// Cuál de las tres situaciones de [`EstadoDeLaConexion`] se da.
pub fn estado_de_la_conexion(agente: &Agente, espacio_en_uso: &Path) -> EstadoDeLaConexion {
    if !agente.conectado {
        return EstadoDeLaConexion::SinConectar;
    }

    let en_uso = ruta_para_configuracion(espacio_en_uso);

    match agente.espacio_registrado.as_deref() {
        Some(registrado) if registrado == en_uso => EstadoDeLaConexion::Conectado,
        _ => EstadoDeLaConexion::ConectadoAOtraCarpeta,
    }
}

/// Lee la carpeta de trabajo que quedó escrita en la configuración de un agente.
///
/// Se lee del disco, no de lo que MMCelt recuerde haber escrito: el archivo pertenece a
/// otro programa y pudo cambiarlo cualquiera, incluido el propio usuario a mano.
///
/// El texto se devuelve **tal cual está escrito**, sin deshacer los escapes del TOML. No
/// hace falta: [`conectar`] normaliza las rutas con barras normales antes de escribirlas,
/// así que en la práctica no queda ningún escape que deshacer. Una carpeta cuyo nombre
/// contuviera comillas o saltos de línea se leería distinta de la real y el agente
/// aparecería como pendiente de actualizar, que es el lado reparable del error.
///
/// # Parámetros
/// - `ruta`: archivo de configuración del agente.
/// - `formato`: cómo está escrito ese archivo.
///
/// # Devuelve
/// La carpeta registrada, o `None` si no hay entrada, no se puede leer o no declara la
/// variable [`VARIABLE_ESPACIO_TRABAJO`].
pub(crate) fn espacio_registrado(ruta: &Path, formato: FormatoConfiguracion) -> Option<String> {
    let contenido = std::fs::read_to_string(ruta).ok()?;

    match formato {
        FormatoConfiguracion::Json => {
            let raiz: Value = serde_json::from_str(&contenido).ok()?;

            raiz.get("mcpServers")?
                .get(NOMBRE_SERVIDOR)?
                .get("env")?
                .get(VARIABLE_ESPACIO_TRABAJO)?
                .as_str()
                .map(str::to_string)
        }

        FormatoConfiguracion::Toml => {
            let cabecera = format!("[mcp_servers.{NOMBRE_SERVIDOR}]");

            seccion_toml(&contenido, &cabecera)?
                .iter()
                .copied()
                .find_map(|linea| {
                    valor_toml(linea, "env")
                        .and_then(|tabla| valor_de_tabla_toml(tabla, VARIABLE_ESPACIO_TRABAJO))
                })
                .map(str::to_string)
        }
    }
}

/// Comprueba si MMCelt está **realmente** conectado en un archivo de configuración.
///
/// No basta con que la entrada `mmcelt` figure en el archivo. Esa entrada sobrevive a que
/// el programa se mueva, se renombre o se borre, y también puede haberse escrito a mano
/// sin el argumento que arranca el servidor. En cualquiera de esos casos el agente no
/// puede levantar MMCelt, así que anunciar «Conectado» engaña a quien lo lee y, peor
/// todavía, esconde el botón «Conectar», que es justamente lo que lo arreglaría.
///
/// Por eso se exigen dos cosas: que el comando registrado sea el ejecutable de MMCelt
/// **que está haciendo la comprobación**, y que [`ARGUMENTO_SERVIDOR`] figure entre sus
/// argumentos. Que un candidato anterior siga existiendo en disco no lo convierte en la
/// conexión del candidato actual.
///
/// # Parámetros
/// - `ruta`: archivo de configuración del agente.
/// - `formato`: cómo está escrito ese archivo.
///
/// # Devuelve
/// `true` solo si la entrada existe y pertenece al ejecutable actual. Ante cualquier duda
/// devuelve `false`: la ventana ofrece entonces «Conectar» y [`conectar`] reescribe la
/// entrada con los valores correctos, de modo que equivocarse por este lado cuesta un
/// clic y equivocarse por el otro deja al usuario sin explicación.
pub(crate) fn esta_conectado(ruta: &Path, formato: FormatoConfiguracion) -> bool {
    let Ok(contenido) = std::fs::read_to_string(ruta) else {
        return false;
    };

    match formato {
        FormatoConfiguracion::Json => serde_json::from_str::<Value>(&contenido)
            .ok()
            .and_then(|v| {
                v.get("mcpServers")
                    .and_then(Value::as_object)
                    .and_then(|m| m.get(NOMBRE_SERVIDOR))
                    .map(entrada_json_utilizable)
            })
            .unwrap_or(false),

        FormatoConfiguracion::Toml => {
            let cabecera = format!("[mcp_servers.{NOMBRE_SERVIDOR}]");
            seccion_toml(&contenido, &cabecera)
                .map(|lineas| seccion_toml_utilizable(&lineas))
                .unwrap_or(false)
        }
    }
}

/// Decide si una entrada `mmcelt` de un archivo JSON puede arrancar el servidor.
///
/// # Parámetros
/// - `entrada`: el valor asociado a la clave `mmcelt` dentro de `mcpServers`. Puede ser
///   cualquier cosa, porque el archivo pertenece a otro programa y pudo editarse a mano;
///   lo que no tenga la forma esperada se considera inservible en lugar de un error.
///
/// # Devuelve
/// `true` si el comando sigue existiendo y entre los argumentos está
/// [`ARGUMENTO_SERVIDOR`].
fn entrada_json_utilizable(entrada: &Value) -> bool {
    let comando = entrada
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or_default();

    // Se comparan argumentos completos, no fragmentos de texto: `--mcp-server-antiguo`
    // empieza igual y no arrancaría nada.
    let arranca_el_servidor = entrada
        .get("args")
        .and_then(Value::as_array)
        .is_some_and(|args| args.iter().any(|a| a.as_str() == Some(ARGUMENTO_SERVIDOR)));

    es_el_ejecutable_en_curso(comando) && arranca_el_servidor
}

/// Devuelve las líneas de una sección del TOML, si la sección existe.
///
/// Se lee el texto tal cual, sin analizador de TOML, por la misma razón que en
/// [`conectar_toml`]: la operación es acotada y no justifica otra dependencia.
///
/// La sección termina en la siguiente cabecera, de modo que el contenido de otro servidor
/// no puede colarse en la lectura del de MMCelt. Una cabecera comentada no abre ninguna
/// sección, que es lo correcto: comentarla es la forma habitual de desactivarla.
///
/// # Parámetros
/// - `contenido`: el archivo entero.
/// - `cabecera`: la línea de cabecera que se busca, ya escrita entre corchetes.
///
/// # Devuelve
/// Las líneas que hay entre la cabecera y la siguiente sección, o `None` si la cabecera
/// no aparece en el archivo.
fn seccion_toml<'a>(contenido: &'a str, cabecera: &str) -> Option<Vec<&'a str>> {
    let mut lineas = Vec::new();
    let mut dentro_de_la_seccion = false;

    for linea in contenido.lines() {
        let limpia = linea.trim();

        if limpia == cabecera {
            dentro_de_la_seccion = true;
            continue;
        }

        if dentro_de_la_seccion {
            // Otra cabecera: aquí termina la sección de MMCelt.
            if limpia.starts_with('[') {
                break;
            }
            lineas.push(linea);
        }
    }

    dentro_de_la_seccion.then_some(lineas)
}

/// Decide si la sección `[mcp_servers.mmcelt]` puede arrancar el servidor.
///
/// # Parámetros
/// - `lineas`: las líneas de la sección, tal como las devuelve [`seccion_toml`].
///
/// # Devuelve
/// `true` si la sección define como `command` el ejecutable MMCelt en curso y unos
/// `args` que incluyen [`ARGUMENTO_SERVIDOR`].
fn seccion_toml_utilizable(lineas: &[&str]) -> bool {
    let comando = lineas
        .iter()
        .copied()
        .find_map(|linea| valor_de_texto_toml(linea, "command"))
        .unwrap_or_default();

    let arranca_el_servidor = lineas
        .iter()
        .copied()
        .find_map(|linea| valor_toml(linea, "args"))
        .is_some_and(|args| elementos_de_lista_toml(args).contains(&ARGUMENTO_SERVIDOR));

    es_el_ejecutable_en_curso(comando) && arranca_el_servidor
}

/// Devuelve lo que una línea de TOML asigna a una clave.
///
/// La clave tiene que coincidir entera: en `command_anterior = "…"` lo que sigue al
/// nombre buscado no es un `=`, así que la línea no define `command`.
///
/// # Parámetros
/// - `linea`: una línea del archivo, con la sangría que traiga.
/// - `clave`: el nombre exacto de la clave.
///
/// # Devuelve
/// El texto que sigue al `=`, o `None` si la línea no define esa clave.
fn valor_toml<'a>(linea: &'a str, clave: &str) -> Option<&'a str> {
    Some(
        linea
            .trim()
            .strip_prefix(clave)?
            .trim_start()
            .strip_prefix('=')?
            .trim_start(),
    )
}

/// Extrae el valor de una clave de TOML escrita entre comillas dobles.
///
/// El valor termina en la **primera** comilla de cierre, de modo que un comentario al
/// final de la línea no acaba dentro del valor por muchas comillas que lleve.
///
/// El texto se devuelve tal cual, sin deshacer las secuencias de escape. MMCelt registra
/// siempre la ruta con barras normales, así que ahí no hay ninguna que deshacer; una ruta
/// que sí las necesitara —solo posible si contuviera comillas o saltos de línea— se leerá
/// distinta de la real y la conexión constará como rota.
///
/// # Devuelve
/// El valor, o `None` si la línea no define esa clave o no la define como texto.
fn valor_de_texto_toml<'a>(linea: &'a str, clave: &str) -> Option<&'a str> {
    let interior = valor_toml(linea, clave)?.strip_prefix('"')?;
    let fin = interior.find('"')?;

    Some(&interior[..fin])
}

/// Devuelve los textos entrecomillados de una lista de TOML escrita en una sola línea.
///
/// Buscar `"--mcp-server"` como fragmento de la línea sería más corto y estaría mal: un
/// comentario al final que mencionara el argumento bastaría para dar por buena una
/// conexión que no puede funcionar. Por eso la lectura se detiene en el corchete de
/// cierre y solo cuenta lo que hay dentro de la lista.
///
/// MMCelt escribe siempre la lista en una sola línea. Una repartida en varias se leerá
/// incompleta y la conexión constará como rota, que es el error que sabemos reparar.
///
/// # Parámetros
/// - `valor`: el texto que sigue al `=`, tal como lo devuelve [`valor_toml`].
///
/// # Devuelve
/// Los elementos escritos entre comillas dobles, o una lista vacía si el texto no es una
/// lista cerrada en esa misma línea.
fn elementos_de_lista_toml(valor: &str) -> Vec<&str> {
    let Some(interior) = interior_de_lista_toml(valor) else {
        return Vec::new();
    };

    // Al partir por comillas, los elementos quedan en los trozos impares: el 0 es lo que
    // hay antes de la primera comilla, el 1 es el primer elemento, el 2 la coma que lo
    // separa del siguiente, y así sucesivamente.
    interior.split('"').skip(1).step_by(2).collect()
}

/// Devuelve el texto encerrado entre los corchetes de una lista de TOML.
///
/// # Devuelve
/// El interior de la lista, o `None` si el texto no empieza por `[` o la lista no se
/// cierra en la misma línea.
fn interior_de_lista_toml(valor: &str) -> Option<&str> {
    interior_delimitado_toml(valor, '[', ']')
}

/// Devuelve lo que una tabla de TOML escrita en una línea asigna a una clave.
///
/// Es la forma que tiene la línea `env = { MMCELT_WORKSPACE = "…" }` que escribe
/// [`conectar_toml`]. La clave se busca entre **todos** los pares de la tabla, no solo en
/// el primero: el archivo pertenece a otro programa y puede haber añadido variables suyas.
///
/// # Parámetros
/// - `valor`: el texto que sigue al `=`, tal como lo devuelve [`valor_toml`].
/// - `clave`: el nombre exacto de la variable que se busca dentro de la tabla.
///
/// # Devuelve
/// El valor entrecomillado de esa clave, o `None` si el texto no es una tabla cerrada en
/// la misma línea o la clave no aparece en ella.
fn valor_de_tabla_toml<'a>(valor: &'a str, clave: &str) -> Option<&'a str> {
    let interior = interior_delimitado_toml(valor, '{', '}')?;

    pares_de_tabla_toml(interior)
        .into_iter()
        .find_map(|par| valor_de_texto_toml(par, clave))
}

/// Parte el interior de una tabla de TOML por sus comas.
///
/// Las comas escritas **dentro** de las comillas no separan nada: forman parte del valor.
/// Sin esa distinción, una ruta con una coma partiría el par en dos y la clave dejaría de
/// leerse.
///
/// # Parámetros
/// - `interior`: lo que hay entre las llaves, ya sin ellas.
///
/// # Devuelve
/// Cada par `clave = valor`, con la sangría y los espacios que traiga.
fn pares_de_tabla_toml(interior: &str) -> Vec<&str> {
    let mut pares = Vec::new();
    let mut inicio = 0;
    let mut dentro_de_una_cadena = false;

    for (posicion, caracter) in interior.char_indices() {
        match caracter {
            '"' => dentro_de_una_cadena = !dentro_de_una_cadena,
            ',' if !dentro_de_una_cadena => {
                pares.push(&interior[inicio..posicion]);
                inicio = posicion + caracter.len_utf8();
            }
            _ => {}
        }
    }

    pares.push(&interior[inicio..]);
    pares
}

/// Devuelve el texto encerrado entre los delimitadores de una lista o una tabla de TOML.
///
/// El delimitador de cierre se busca **fuera** de las comillas, para que un `]` o un `}`
/// escritos dentro de un valor no den por terminada la estructura antes de tiempo.
///
/// # Parámetros
/// - `valor`: el texto que sigue al `=`, tal como lo devuelve [`valor_toml`].
/// - `apertura`: `[` para una lista, `{` para una tabla en línea.
/// - `cierre`: el delimitador que la termina.
///
/// # Devuelve
/// El interior, o `None` si el texto no empieza por `apertura` o no se cierra en la misma
/// línea.
fn interior_delimitado_toml(valor: &str, apertura: char, cierre: char) -> Option<&str> {
    let interior = valor.trim_start().strip_prefix(apertura)?;
    let mut dentro_de_una_cadena = false;

    for (posicion, caracter) in interior.char_indices() {
        if caracter == '"' {
            dentro_de_una_cadena = !dentro_de_una_cadena;
        } else if caracter == cierre && !dentro_de_una_cadena {
            return Some(&interior[..posicion]);
        }
    }

    None
}

/// Comprueba que el comando registrado sea el ejecutable MMCelt en curso.
///
/// No basta con que exista: los candidatos anteriores se conservan y sus rutas permanecen
/// válidas aunque la persona haya abierto una versión nueva. Se canonizan ambos extremos
/// para admitir enlaces y diferencias de representación sin confundir dos archivos
/// distintos. Ante un error se devuelve `false`, dejando disponible la acción reparadora.
///
/// # Parámetros
/// - `comando`: el comando tal como está escrito en la configuración.
fn es_el_ejecutable_en_curso(comando: &str) -> bool {
    let registrado = Path::new(comando);
    if comando.trim().is_empty() || !registrado.is_file() {
        return false;
    }

    let Ok(actual) = ruta_ejecutable() else {
        return false;
    };
    let (Ok(registrado), Ok(actual)) = (
        std::fs::canonicalize(registrado),
        std::fs::canonicalize(actual),
    ) else {
        return false;
    };

    registrado == actual
}

/// Devuelve la ruta del ejecutable de MMCelt en curso.
///
/// Es lo que se registra en los agentes, y la razón de que la conexión no necesite
/// ninguna otra cosa instalada.
///
/// # Errores
/// Devuelve error si el sistema no puede informar de la ruta del propio proceso, algo
/// muy poco habitual pero que impediría configurar nada.
pub fn ruta_ejecutable() -> AppResult<PathBuf> {
    std::env::current_exe().map_err(|e| AppError::lectura("el ejecutable de MMCelt", e))
}

/// Escribe una ruta tal como debe quedar dentro del archivo de configuración de un agente.
///
/// Siempre con barras normales: es lo que esperan estos archivos, también en Windows, y
/// además evita que el escape de las cadenas de TOML tenga que lidiar con `C:\Users\…`.
///
/// Existe como función porque la usan **los dos extremos**: [`conectar`] al escribir y
/// [`estado_de_la_conexion`] al comparar. Si cada uno normalizara por su cuenta, un cambio
/// en uno haría que todos los agentes aparecieran de golpe como pendientes de actualizar.
///
/// # Parámetros
/// - `ruta`: la ruta tal como la maneja el sistema operativo.
fn ruta_para_configuracion(ruta: &Path) -> String {
    ruta.display().to_string().replace('\\', "/")
}

/// Crea una copia de seguridad del archivo antes de modificarlo.
///
/// # Devuelve
/// La ruta de la copia, o `None` si el archivo no existía todavía.
fn copia_de_seguridad(ruta: &Path) -> AppResult<Option<PathBuf>> {
    crate::respaldo::copiar_antes_de_sobrescribir(ruta).map_err(|e| AppError::escritura(ruta, e))
}

/// Lee una configuración sin confundir «no existe» con «no puedo leerla».
///
/// # Parámetros
/// - `ruta`: archivo de configuración perteneciente al agente.
///
/// # Devuelve
/// `None` únicamente cuando el sistema operativo informa de que el archivo no existe. Un
/// archivo existente y vacío se devuelve como `Some("")` porque sí ha podido leerse.
///
/// # Errores
/// [`AppError::Lectura`] ante permisos insuficientes, rutas que no son archivos y cualquier
/// otro fallo. Quien conecta debe detenerse antes de crear una copia o escribir.
pub(crate) fn leer_configuracion(ruta: &Path) -> AppResult<Option<String>> {
    match std::fs::read_to_string(ruta) {
        Ok(contenido) => Ok(Some(contenido)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(AppError::lectura(ruta, error)),
    }
}

/// Registra MMCelt como servidor MCP en el agente indicado.
///
/// # Parámetros
/// - `agente`: agente a configurar.
/// - `espacio_trabajo`: carpeta dentro de la cual podrá leer y escribir el agente.
///
/// # Devuelve
/// La ruta de la copia de seguridad creada, si hubo que crear alguna.
///
/// # Errores
/// - [`AppError::Formato`] si el archivo existe pero su contenido no es legible. En ese
///   caso **no se modifica nada**.
/// - [`AppError::Escritura`] si no se puede escribir el archivo o su copia.
pub fn conectar(agente: &Agente, espacio_trabajo: &Path) -> AppResult<Option<PathBuf>> {
    let motivo = if !es_carpeta_valida_para_espacio_trabajo(espacio_trabajo) {
        Some(MotivoCarpetaInvalida::RaizDeVolumenONoValida)
    } else if carpeta_contiene_otros_proyectos(espacio_trabajo) {
        Some(MotivoCarpetaInvalida::ContieneOtrosProyectos)
    } else {
        None
    };
    if let Some(motivo) = motivo {
        return Err(AppError::CarpetaProyectoInvalida {
            ruta: espacio_trabajo.to_path_buf(),
            motivo,
        });
    }

    let ejecutable = ruta_ejecutable()?;

    let ejecutable = ruta_para_configuracion(&ejecutable);
    let espacio = ruta_para_configuracion(espacio_trabajo);

    match agente.formato {
        FormatoConfiguracion::Json => conectar_json(agente, &ejecutable, &espacio),
        FormatoConfiguracion::Toml => conectar_toml(agente, &ejecutable, &espacio),
    }
}

/// Registra el servidor en un archivo con estructura `{ "mcpServers": { … } }`.
fn conectar_json(agente: &Agente, ejecutable: &str, espacio: &str) -> AppResult<Option<PathBuf>> {
    let ruta = &agente.ruta_configuracion;

    // Se parte de lo que ya hubiera. Algunos clientes crean el archivo vacío antes del primer
    // servidor; tratarlo como error impediría configurarlos.
    let mut raiz: Value = match leer_configuracion(ruta)? {
        Some(texto) if !texto.trim().is_empty() => serde_json::from_str(&texto)
            .map_err(|e| AppError::formato(format!("al leer {}", ruta.display()), e))?,
        Some(_) | None => Value::Object(Map::new()),
    };

    if !raiz.is_object() {
        return Err(AppError::EstructuraInvalida {
            fallo: crate::error::FalloEstructural::RaizInexistente {
                root_id: format!("el archivo {} no contiene un objeto JSON", ruta.display()),
            },
        });
    }

    let respaldo = copia_de_seguridad(ruta)?;

    let objeto = raiz
        .as_object_mut()
        .expect("ya se comprobó que es un objeto");

    // Se conserva lo que hubiera: otros servidores MCP y, en el caso de Claude Code,
    // también el historial y las preferencias del usuario, que viven en este archivo.
    let servidores = objeto
        .entry("mcpServers")
        .or_insert_with(|| Value::Object(Map::new()));

    if !servidores.is_object() {
        *servidores = Value::Object(Map::new());
    }

    servidores
        .as_object_mut()
        .expect("acaba de asegurarse")
        .insert(
            NOMBRE_SERVIDOR.to_string(),
            json!({
                "command": ejecutable,
                "args": [ARGUMENTO_SERVIDOR],
                "env": { VARIABLE_ESPACIO_TRABAJO: espacio }
            }),
        );

    if let Some(carpeta) = ruta.parent() {
        std::fs::create_dir_all(carpeta).map_err(|e| AppError::escritura(carpeta, e))?;
    }

    let texto = serde_json::to_string_pretty(&raiz)
        .map_err(|e| AppError::formato("al generar la configuración", e))?;

    crate::storage::escribir_de_forma_atomica(ruta, texto.as_bytes())?;

    Ok(respaldo)
}

/// Registra el servidor en el `config.toml` de Codex.
///
/// No se usa un analizador de TOML —haría falta otra dependencia solo para esto— sino
/// que se trabaja sobre el texto. Es viable porque la operación es acotada: comprobar si
/// la sección existe y, si no, añadirla al final. El resto del archivo no se toca.
fn conectar_toml(agente: &Agente, ejecutable: &str, espacio: &str) -> AppResult<Option<PathBuf>> {
    let ejecutable = &escapar_cadena_toml(ejecutable);
    let espacio = &escapar_cadena_toml(espacio);
    let ruta = &agente.ruta_configuracion;
    let cabecera = format!("[mcp_servers.{NOMBRE_SERVIDOR}]");

    let contenido = leer_configuracion(ruta)?.unwrap_or_default();
    let respaldo = copia_de_seguridad(ruta)?;

    let nuevo = if contenido.lines().any(|l| l.trim() == cabecera) {
        // Ya existe: se reemplaza la sección entera, desde su cabecera hasta la siguiente.
        reemplazar_seccion_toml(&contenido, &cabecera, ejecutable, espacio)
    } else {
        format!(
            "{contenido}\n\
             # --- MMCelt: mapas mentales para dirigir modelos de IA ---\n\
             {cabecera}\n\
             command = \"{ejecutable}\"\n\
             args = [\"{ARGUMENTO_SERVIDOR}\"]\n\
             env = {{ {VARIABLE_ESPACIO_TRABAJO} = \"{espacio}\" }}\n"
        )
    };

    if let Some(carpeta) = ruta.parent() {
        std::fs::create_dir_all(carpeta).map_err(|e| AppError::escritura(carpeta, e))?;
    }

    crate::storage::escribir_de_forma_atomica(ruta, nuevo.as_bytes())?;

    Ok(respaldo)
}

/// Escapa un texto para poder incrustarlo en una cadena de TOML entre comillas dobles.
///
/// Las rutas se normalizan antes con barras normales, así que el caso de Windows —donde
/// `C:\\Users\\...` haría que TOML leyera `\\U` como un escape Unicode inválido— ya está
/// resuelto de raíz. Queda el resto: en Linux y macOS un nombre de carpeta **puede**
/// contener comillas dobles o saltos de línea, y sin escapar romperían la sintaxis del
/// archivo de configuración del agente, que dejaría de arrancar.
///
/// # Parámetros
/// - `texto`: la ruta o el valor que se va a incrustar entre comillas.
///
/// # Devuelve
/// El texto con las barras invertidas, las comillas y los saltos de línea escapados.
fn escapar_cadena_toml(texto: &str) -> String {
    texto
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// Sustituye una sección existente del TOML por una nueva.
///
/// Recorre las líneas copiándolas, y al encontrar la cabecera buscada omite tanto esa
/// tabla como sus subsecciones hasta llegar a una tabla ajena. Codex representa mapas
/// como `env` mediante una subsección; conservarla al sustituir la tabla padre definiría
/// la misma clave dos veces y dejaría todo el archivo inválido.
fn reemplazar_seccion_toml(
    contenido: &str,
    cabecera: &str,
    ejecutable: &str,
    espacio: &str,
) -> String {
    let mut salida = String::new();
    let mut dentro_de_la_seccion = false;

    for linea in contenido.lines() {
        let limpia = linea.trim();

        if limpia == cabecera {
            dentro_de_la_seccion = true;
            salida.push_str(cabecera);
            salida.push('\n');
            salida.push_str(&format!("command = \"{ejecutable}\"\n"));
            salida.push_str(&format!("args = [\"{ARGUMENTO_SERVIDOR}\"]\n"));
            salida.push_str(&format!(
                "env = {{ {VARIABLE_ESPACIO_TRABAJO} = \"{espacio}\" }}\n"
            ));
            continue;
        }

        // Una subsección pertenece todavía a la tabla reemplazada y también se retira.
        if dentro_de_la_seccion && es_subseccion_toml_de(limpia, cabecera) {
            continue;
        }

        // Una cabecera ajena marca el final del árbol de tablas que se reemplaza.
        if dentro_de_la_seccion && limpia.starts_with('[') {
            dentro_de_la_seccion = false;
        }

        if !dentro_de_la_seccion {
            salida.push_str(linea);
            salida.push('\n');
        }
    }

    salida
}

/// Indica si una cabecera TOML es descendiente de otra tabla concreta.
///
/// # Parámetros
/// - `candidata`: línea recortada que podría contener una cabecera de tabla.
/// - `padre`: cabecera completa de la tabla padre, incluidos los corchetes.
///
/// # Devuelve
/// `true` únicamente cuando la candidata amplía la ruta del padre con un punto. Así,
/// `[mcp_servers.mmcelt.env]` pertenece a `[mcp_servers.mmcelt]`, pero
/// `[mcp_servers.otro]` y `[mcp_servers.mmcelt_otro]` no.
fn es_subseccion_toml_de(candidata: &str, padre: &str) -> bool {
    let Some(ruta_padre) = padre.strip_suffix(']') else {
        return false;
    };
    let Some(descendiente) = candidata.strip_prefix(ruta_padre) else {
        return false;
    };

    descendiente.starts_with('.') && descendiente.ends_with(']')
}

/// Elimina el registro de MMCelt del agente indicado.
///
/// # Parámetros
/// - `agente`: el agente del que hay que retirar la entrada de MMCelt. De él se toman la
///   ruta de su archivo de configuración y el formato en que está escrito.
///
/// # Devuelve
/// La ruta de la copia de seguridad creada, si hubo que crear alguna.
///
/// # Errores
/// Los mismos casos que [`conectar`].
pub fn desconectar(agente: &Agente) -> AppResult<Option<PathBuf>> {
    let ruta = &agente.ruta_configuracion;

    if !ruta.exists() {
        return Ok(None);
    }

    let respaldo = copia_de_seguridad(ruta)?;

    match agente.formato {
        FormatoConfiguracion::Json => {
            let texto = std::fs::read_to_string(ruta).map_err(|e| AppError::lectura(ruta, e))?;

            let mut raiz: Value = serde_json::from_str(&texto)
                .map_err(|e| AppError::formato(format!("al leer {}", ruta.display()), e))?;

            if let Some(servidores) = raiz.get_mut("mcpServers").and_then(Value::as_object_mut) {
                servidores.remove(NOMBRE_SERVIDOR);
            }

            let nuevo = serde_json::to_string_pretty(&raiz)
                .map_err(|e| AppError::formato("al generar la configuración", e))?;

            crate::storage::escribir_de_forma_atomica(ruta, nuevo.as_bytes())?;
        }

        FormatoConfiguracion::Toml => {
            let contenido =
                std::fs::read_to_string(ruta).map_err(|e| AppError::lectura(ruta, e))?;
            let cabecera = format!("[mcp_servers.{NOMBRE_SERVIDOR}]");

            let mut salida = String::new();
            let mut dentro_de_la_seccion = false;

            for linea in contenido.lines() {
                let limpia = linea.trim();

                if limpia == cabecera {
                    dentro_de_la_seccion = true;
                    continue;
                }

                if dentro_de_la_seccion && es_subseccion_toml_de(limpia, &cabecera) {
                    continue;
                }

                if dentro_de_la_seccion && limpia.starts_with('[') {
                    dentro_de_la_seccion = false;
                }

                // También se retira el comentario con el que se anunció la sección.
                if !dentro_de_la_seccion && !limpia.starts_with("# --- MMCelt:") {
                    salida.push_str(linea);
                    salida.push('\n');
                }
            }

            crate::storage::escribir_de_forma_atomica(ruta, salida.as_bytes())?;
        }
    }

    Ok(respaldo)
}

/// Devuelve la carpeta de trabajo que se propone por defecto para los mapas.
///
/// Es `Documentos/MapasMentales` (o su equivalente), no la carpeta del programa: en una
/// instalación portable esta puede estar en una memoria USB o en una ruta sin permiso de
/// escritura.
pub fn espacio_trabajo_sugerido() -> PathBuf {
    let personal = carpeta_personal().unwrap_or_else(|| PathBuf::from("."));

    // No hay forma sin dependencias de localizar «Documentos» en cualquier idioma, así
    // que se prueba el nombre habitual y se recurre a la carpeta personal si no está.
    for candidata in ["Documents", "Documentos"] {
        let ruta = personal.join(candidata);
        if ruta.is_dir() {
            return ruta.join("MapasMentales");
        }
    }

    personal.join("MapasMentales")
}

/// Decide qué carpeta de trabajo se le propone al usuario en la ventana de conexión.
///
/// El orden importa y es el de siempre en este programa: **manda lo que el usuario eligió**;
/// si no eligió nada, se deduce de lo que está haciendo; y solo si no hay nada de donde
/// deducirlo se recurre a la propuesta de fábrica.
///
/// Deducirla del mapa abierto no es un adorno. En la prueba real con Claude Code, el mapa y
/// el proyecto del usuario estaban en una carpeta y MMCelt autorizó `Documentos/MapasMentales`,
/// que es donde no había nada: el agente recibió permiso para trabajar en el sitio
/// equivocado y cada ruta que probó chocó contra la valla del espacio de trabajo.
/// Comprueba si la carpeta contiene múltiples proyectos `.mmcelt` en subdirectorios,
/// o es una carpeta contenedora de otros proyectos sin ser la carpeta atómica del proyecto.
pub fn carpeta_contiene_otros_proyectos(carpeta: &Path) -> bool {
    // La carpeta temporal del sistema alberga residuos de pruebas y no representa una carpeta madre de proyectos
    if let (Ok(canon_carpeta), Ok(canon_temp)) =
        (carpeta.canonicalize(), std::env::temp_dir().canonicalize())
    {
        if canon_carpeta == canon_temp {
            return false;
        }
    }

    let mut tiene_mapa_propio = false;
    let mut subcarpetas_con_mapa = 0;

    if let Ok(entradas) = std::fs::read_dir(carpeta) {
        for entrada in entradas.flatten() {
            let ruta = entrada.path();
            if ruta.is_file() && ruta.extension().and_then(|ext| ext.to_str()) == Some("mmcelt") {
                tiene_mapa_propio = true;
            } else if ruta.is_dir() {
                if let Ok(sub_entradas) = std::fs::read_dir(&ruta) {
                    for sub in sub_entradas.flatten() {
                        let sub_path = sub.path();
                        if sub_path.is_file()
                            && sub_path.extension().and_then(|ext| ext.to_str()) == Some("mmcelt")
                        {
                            subcarpetas_con_mapa += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    subcarpetas_con_mapa > 1 || (subcarpetas_con_mapa > 0 && !tiene_mapa_propio)
}

/// Resuelve una ruta a su forma canónica cuando la carpeta existe; si no (por ejemplo, se borró
/// entre proponerla y fijarla), conserva la propuesta tal cual en vez de fallar.
///
/// Único punto de cálculo de canonicalización del espacio de trabajo (A-2): tanto sincronizar el
/// espacio de trabajo con el mapa abierto como crear un proyecto directamente en una carpeta
/// elegida pasan por aquí, para que una carpeta alcanzada por una unión de directorio no quede
/// canónica en un flujo y sin canonizar en el otro.
pub(crate) fn canonizar_o_conservar(ruta: PathBuf) -> PathBuf {
    ruta.canonicalize().unwrap_or(ruta)
}

/// Comprueba si una ruta representa una carpeta de trabajo válida para operar con agentes.
///
/// Rechaza nombres vacíos y raíces de volumen (como `O:\` en Windows o `/` en Unix)
/// que no contienen ningún componente normal propio. Una raíz de volumen es demasiado amplia
/// y dejaría a los agentes operar sobre el disco entero sin límite.
pub fn es_carpeta_valida_para_espacio_trabajo(ruta: &Path) -> bool {
    let ruta_limpia = crate::ui::limpiar_ruta_para_interfaz(ruta);
    let path = Path::new(&ruta_limpia);
    !path.as_os_str().is_empty()
        && path
            .components()
            .any(|c| matches!(c, std::path::Component::Normal(_)))
}

/// Único punto que decide qué texto de la carpeta de trabajo válida se le muestra a la persona
/// (A-2).
///
/// Antes de esto, «Proyecto e instrucciones» y «Conectar con mis IAs» decidían cada una por su
/// cuenta si limpiar el prefijo técnico de Windows y sobre qué copia de la ruta hacerlo; la
/// segunda ni siquiera lo hacía. Ahora las dos consumen esta función y ninguna vuelve a llamar a
/// [`crate::ui::limpiar_ruta_para_interfaz`] por su cuenta.
///
/// El aviso de carpeta inválida (`ai_modal.rs`) queda fuera a propósito: su cometido es mostrar
/// precisamente la ruta que falló, que por definición puede no ser válida — lo que esta función,
/// con su [`None`] para esos casos, no puede ofrecer. Ese tercer sitio hereda la canonicalización
/// igualmente, pero desde el único punto donde se calcula el valor: sincronizar el espacio de
/// trabajo con el mapa abierto, no esta función.
///
/// # Devuelve
/// `None` cuando `ruta` no es una carpeta de trabajo válida; en ese caso la pantalla decide su
/// propio mensaje de «sin carpeta», que sí depende del contexto de cada una.
pub fn ruta_de_trabajo_para_interfaz(ruta: &Path) -> Option<String> {
    if !es_carpeta_valida_para_espacio_trabajo(ruta) {
        return None;
    }
    Some(crate::ui::limpiar_ruta_para_interfaz(ruta))
}

/// Determina si un archivo de mapa reside directamente en la raíz de una unidad o volumen
/// (por ejemplo `O:\mapa.mmcelt` o `/mapa.mmcelt`), donde no existe una carpeta de proyecto
/// propia que delimite el alcance seguro de los agentes de IA.
pub fn es_mapa_en_raiz_de_volumen(ruta: &Path) -> bool {
    let ruta_limpia = crate::ui::limpiar_ruta_para_interfaz(ruta);
    let path = Path::new(&ruta_limpia);
    let ruta_completa = if path.is_absolute() {
        path.to_path_buf()
    } else if let Ok(actual) = std::env::current_dir() {
        actual.join(path)
    } else {
        path.to_path_buf()
    };
    match ruta_completa.parent() {
        Some(padre) => !es_carpeta_valida_para_espacio_trabajo(padre),
        None => true,
    }
}

/// Propone la carpeta de trabajo del proyecto a partir de la preferencia guardada y del mapa activo.
///
/// Si hay un mapa activo abierto:
/// - El espacio de trabajo del proyecto es siempre la carpeta donde reside dicho mapa.
/// - Si el mapa estuviera en la raíz de un volumen (o fuera de una carpeta válida),
///   no se adopta como espacio de trabajo ni se asocia a la preferencia de otro proyecto previo:
///   devuelve una ruta vacía (modo solo lectura / sin espacio de trabajo).
///
/// Si no hay ningún mapa activo:
/// - Se adopta la preferencia guardada si es válida, o se sugiere la carpeta por omisión.
pub fn espacio_trabajo_propuesto(preferencia: &str, mapa_activo: Option<&Path>) -> PathBuf {
    if let Some(mapa) = mapa_activo {
        return mapa
            .parent()
            .filter(|c| es_carpeta_valida_para_espacio_trabajo(c))
            .map_or_else(PathBuf::new, Path::to_path_buf);
    }

    let pref_limpia = preferencia.trim();
    if !pref_limpia.is_empty() {
        let ruta_pref = Path::new(pref_limpia);
        if es_carpeta_valida_para_espacio_trabajo(ruta_pref) {
            return ruta_pref.to_path_buf();
        }
    }

    espacio_trabajo_sugerido()
}

#[cfg(test)]
mod pruebas {
    use super::{esta_conectado, FormatoConfiguracion, ARGUMENTO_SERVIDOR};

    /// Devuelve otro ejecutable real del sistema para simular un candidato anterior que
    /// todavía existe en disco, pero no es el proceso de MMCelt que está comprobando la conexión.
    fn ejecutable_ajeno_existente() -> std::path::PathBuf {
        #[cfg(windows)]
        {
            std::env::var_os("ComSpec")
                .map(std::path::PathBuf::from)
                .expect("Windows debe declarar el intérprete de órdenes en ComSpec")
        }

        #[cfg(not(windows))]
        {
            std::path::PathBuf::from("/bin/sh")
        }
    }

    /// Un registro que apunta a otro binario existente no pertenece al MMCelt en curso.
    ///
    /// Reproduce un caso real: en un equipo quedaba instalado el ejecutable de una versión
    /// anterior, y la ventana de la nueva lo daba por conectado porque solo comprobaba que el
    /// archivo antiguo siguiera existiendo. Eso ocultaba el botón que podía actualizarlo.
    #[test]
    fn un_ejecutable_anterior_existente_no_cuenta_como_conexion_actual() {
        let ruta = std::env::temp_dir().join(format!(
            "mmcelt_registro_envejecido_{}.json",
            uuid::Uuid::new_v4()
        ));
        let comando = ejecutable_ajeno_existente()
            .display()
            .to_string()
            .replace('\\', "/");
        let contenido = serde_json::json!({
            "mcpServers": {
                "mmcelt": {
                    "command": comando,
                    "args": [ARGUMENTO_SERVIDOR]
                }
            }
        });
        std::fs::write(
            &ruta,
            serde_json::to_vec_pretty(&contenido).expect("serializar JSON"),
        )
        .expect("escribir configuración envejecida");

        let conectado = esta_conectado(&ruta, FormatoConfiguracion::Json);
        let _ = std::fs::remove_file(&ruta);

        assert!(
            !conectado,
            "otro ejecutable existente no debe ocultar la acción que registra el MMCelt actual"
        );
    }

    /// Dos ejecutables con el mismo nombre pero rutas distintas no son la misma versión.
    ///
    /// Es la reproducción fiel del hallazgo: `0.11.2/mmcelt.exe` seguía existiendo cuando
    /// se abrió `0.11.3/mmcelt.exe`. Comparar solo el nombre del archivo dejaría pasar el
    /// registro viejo y volvería a ocultar el botón reparador.
    #[test]
    fn una_copia_con_el_mismo_nombre_en_otra_ruta_no_es_la_conexion_actual() {
        let actual = std::env::current_exe().expect("localizar el ejecutable de pruebas");
        let carpeta =
            std::env::temp_dir().join(format!("mmcelt_otro_candidato_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&carpeta).expect("crear carpeta del candidato anterior");
        let anterior = carpeta.join(actual.file_name().expect("el ejecutable debe tener nombre"));
        std::fs::copy(&actual, &anterior).expect("copiar el ejecutable como candidato anterior");
        let ruta_configuracion = carpeta.join("mcp.json");
        let comando = anterior.display().to_string().replace('\\', "/");
        let contenido = serde_json::json!({
            "mcpServers": {
                "mmcelt": {
                    "command": comando,
                    "args": [ARGUMENTO_SERVIDOR]
                }
            }
        });
        std::fs::write(
            &ruta_configuracion,
            serde_json::to_vec_pretty(&contenido).expect("serializar JSON"),
        )
        .expect("escribir configuración del candidato anterior");

        let conectado = esta_conectado(&ruta_configuracion, FormatoConfiguracion::Json);
        std::fs::remove_dir_all(&carpeta).expect("limpiar candidato anterior de prueba");

        assert!(
            !conectado,
            "un mmcelt homónimo en otra ruta no debe contar como el ejecutable en curso"
        );
    }

    /// C22-B — sobre una carpeta real, la función limpia el prefijo técnico de Windows en vez
    /// de devolver la ruta cruda que entrega `canonicalize()`.
    #[test]
    fn c22_b_ruta_de_trabajo_para_interfaz_limpia_una_carpeta_valida() {
        let carpeta =
            std::env::temp_dir().join(format!("mmcelt_c22b_ruta_valida_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&carpeta).expect("crear carpeta de prueba");
        let canonica = carpeta.canonicalize().expect("canonizar carpeta de prueba");

        let obtenida = super::ruta_de_trabajo_para_interfaz(&canonica)
            .expect("una carpeta existente y válida debe producir texto");

        assert!(
            !obtenida.starts_with(r"\\?\"),
            "debe limpiar el prefijo técnico de Windows: {obtenida}"
        );
        assert_eq!(
            obtenida,
            crate::ui::limpiar_ruta_para_interfaz(&canonica),
            "debe coincidir exactamente con el limpiador de rutas del dominio"
        );

        std::fs::remove_dir_all(&carpeta).expect("limpiar carpeta de prueba");
    }

    /// C22-B — una raíz de volumen, que no delimita ningún proyecto, no produce texto: la
    /// pantalla debe caer a su propio mensaje de «sin carpeta».
    #[test]
    fn c22_b_ruta_de_trabajo_para_interfaz_devuelve_nada_para_una_raiz_de_volumen() {
        #[cfg(windows)]
        let raiz = std::path::PathBuf::from(r"O:\");
        #[cfg(not(windows))]
        let raiz = std::path::PathBuf::from("/");

        assert_eq!(
            super::ruta_de_trabajo_para_interfaz(&raiz),
            None,
            "una raíz de volumen no es una carpeta de trabajo válida"
        );
    }
}
