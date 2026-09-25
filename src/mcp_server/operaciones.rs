//! # Servidor MCP integrado
//!
//! Permite que el propio ejecutable de MMCelt haga de servidor del Model Context
//! Protocol, sin depender de nada más:
//!
//! ```text
//! mmcelt --mcp-server
//! ```
//!
//! ## Por qué el servidor vive dentro del ejecutable
//!
//! Hubo un servidor equivalente escrito aparte, en Python, que se eliminó del proyecto.
//! Tenía dos problemas que este modo resuelve:
//!
//! 1. **Exigía Python instalado**, lo que rompe el caso que más importa: pasarle la
//!    aplicación a otra persona. Si alguien recibe MMCelt como programa portable y no
//!    tiene Python, el servidor no arranca y la conexión con su agente falla, por muy
//!    cómoda que sea la interfaz que se la ofrezca.
//! 2. **Era una implementación paralela.** Reimplementaba el modelo, la validación y la
//!    exportación a Markdown, así que había que mantener a mano dos copias de lo mismo,
//!    con la garantía de que antes o después divergirían.
//!
//! Con este modo, lo que se registra en el agente es el **propio ejecutable que el
//! usuario ya tiene**: sin requisitos, sin instalar nada y con la misma ruta en cualquier
//! sistema operativo. Y al reutilizar `model`, `storage` y `ai_export`, **no puede
//! divergir** de lo que hace la aplicación: es literalmente el mismo código.
//!
//! ## Protocolo
//!
//! JSON-RPC 2.0 sobre entrada y salida estándar, una petición por línea. No abre puertos
//! ni escucha en la red: el agente lo lanza como proceso hijo y le habla por las tuberías.
//!
//! El sobre de cada petición se valida en el submódulo [`protocolo`] **antes** de despachar
//! nada. Lo que llega a este archivo es siempre una [`protocolo::PeticionJsonRpc`]: método en
//! texto, identificador que cumple el protocolo y parámetros con nombre. Un sobre que no
//! cumple JSON-RPC 2.0 se contesta con un error y no toca ninguna herramienta.
//!
//! ## Seguridad
//!
//! Se concede a un agente autónomo la capacidad de escribir en el disco del usuario,
//! así que se aplican cuatro restricciones.
//!
//! 1. **Espacio de trabajo acotado** por la variable `MMCELT_WORKSPACE`. Toda ruta se
//!    resuelve y se comprueba que quede dentro.
//! 2. **Extensiones por operación**: un mapa se escribe como `.mmcelt` o `.json`, y un
//!    documento como `.md`. No hay una lista común: cuando la había, el exportador de
//!    Markdown podía escribir sobre un mapa y convertirlo en texto, que era la forma de
//!    rodear el punto 4. Al leer no se exige extensión, porque el agente puede querer abrir
//!    un archivo que se llame de cualquier manera.
//! 3. **Sin sobrescritura silenciosa**: antes de pisar un archivo se guarda una copia con
//!    la fecha y la hora en el nombre, de modo que las sucesivas no se pisan entre sí.
//!
//!    Tiene **una** excepción, `mmcelt_sync_ai_progress`, que actualiza el mapa una y otra
//!    vez conforme avanza el trabajo; dejar una copia por cada informe de progreso llenaría
//!    la carpeta del usuario de archivos que no ha pedido. Esa herramienta no borra nodos
//!    —solo actualiza los que encuentra y añade los que faltan—, no puede retirar un
//!    descarte ni firmar una aprobación, así que lo que puede estropear está acotado. La
//!    excepción está escrita en el código, en [`PoliticaDeCopia`], y no se puede pedir desde
//!    fuera.
//!
//!    Hubo una segunda: el argumento `overwrite`, que el esquema definía como «pisa el
//!    archivo existente sin conservar copia» y se justificaba como «una decisión explícita
//!    de quien llama». Quien llama es el modelo. Un agente se eximía a sí mismo de dejar
//!    copia con solo pedirlo, y sobre un mapa escrito a mano —que el punto 4 tampoco
//!    protegía entonces— eso destruía el trabajo del usuario sin dejar ni un `.bak`. El
//!    argumento se retiró del esquema; si llega, se ignora.
//!
//! 4. **Lo que no ha escrito la IA no se pisa.** Crear un mapa nuevo sobre un archivo cuyos
//!    nodos no sean todos trabajo de la IA se rechaza. Sin esto, la valla del punto anterior
//!    se rodeaba sin esfuerzo: bastaba con llamar a `mmcelt_create_mindmap` en vez de a
//!    `mmcelt_sync_ai_progress` sobre la misma ruta. El criterio se comprueba sobre el
//!    estado de revisión de cada nodo, no sobre la presencia de correcciones: un mapa que la
//!    persona escribió sin tocar el desplegable «Control Humano» no tiene ninguna marca de
//!    supervisión, y era justo el caso que quedaba desprotegido.

use super::actualizaciones::{
    ActualizacionTipada, ConexionTipada, SincronizacionTipada, ValorEnumerado,
};
use super::argumentos::{
    ArgumentosCrearMapa, ArgumentosExportar, ArgumentosLeerMapa, ArgumentosSinContenido,
    ArgumentosSincronizar, ReferenciaNodoPrestada,
};
use super::error::ErrorHerramienta;
use super::protocolo::{self, ErrorJsonRpc, ParametrosMcp, PeticionJsonRpc};
use super::seguridad::{
    escribir_con_respaldo, espacio_de_trabajo, preparar_devolucion, publicar_devolucion,
    validar_ruta, PoliticaDeCopia, UsoDeLaRuta,
};
use crate::ai_export::exportar_markdown_para_ia;
use crate::devolucion_agentes::OperacionDevolucion;
use crate::model::{EstadoNodo, PrioridadNodo, Proyecto, TipoRelacion};
use crate::textos::{Idioma, Texto};
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// El idioma en que este servidor escribe los documentos Markdown que deja en el disco.
///
/// # De dónde sale
///
/// De la inyección de entorno `MMCELT_IDIOMA` si está presente (para pruebas del sistema),
/// o de las preferencias del usuario guardadas en disco. El servidor corre sin ventana
/// —lo lanza el agente de IA por su cuenta—, así que en producción lo lee del archivo
/// de preferencias, igual que haría la aplicación al arrancar.
///
/// # Por qué se lee una sola vez en producción
///
/// Porque el idioma no cambia mientras el proceso vive, y este servidor reescribe el `_AI.md`
/// en cada informe de progreso de un agente. Leer el archivo de preferencias en cada llamada
/// sería una lectura de disco por herramienta invocada, sin ganar nada. La inyección por
/// variable de entorno se comprueba antes para que las pruebas puedan fijar el idioma sin
/// contaminar la caché estática de preferencias.
///
/// # Devuelve
/// El idioma elegido por el usuario o inyectado por entorno. Si las preferencias no se
/// pueden leer, el que trae de fábrica el programa.
pub(crate) fn idioma_del_usuario() -> Idioma {
    if let Ok(variable) = std::env::var("MMCELT_IDIOMA") {
        if let Some(idioma) = idioma_desde_cadena(&variable) {
            return idioma;
        }
    }

    static ELEGIDO: OnceLock<Idioma> = OnceLock::new();
    *ELEGIDO.get_or_init(|| match crate::preferencias::Preferencias::cargar() {
        Ok(preferencias) => preferencias.idioma,
        Err(error) => {
            crate::error::registrar(&error, "cargar preferencias para el servidor MCP");
            crate::preferencias::Preferencias::primer_arranque().idioma
        }
    })
}

/// Interpreta una cadena como uno de los idiomas soportados por el programa.
///
/// Admite los nombres de variante tipados, con o sin acento, códigos de dos letras
/// y nombres en inglés o del sistema.
fn idioma_desde_cadena(cadena: &str) -> Option<Idioma> {
    let limpia = cadena.trim();
    match limpia {
        "Espanol" | "Español" | "es" | "ES" | "spanish" | "Spanish" => Some(Idioma::Espanol),
        "Ingles" | "Inglés" | "en" | "EN" | "english" | "English" => Some(Idioma::Ingles),
        "Frances" | "Francés" | "fr" | "FR" | "french" | "French" => Some(Idioma::Frances),
        "Aleman" | "Alemán" | "de" | "DE" | "german" | "German" => Some(Idioma::Aleman),
        "Ruso" | "ru" | "RU" | "russian" | "Russian" => Some(Idioma::Ruso),
        "ChinoSimplificado" | "zh" | "ZH" | "chinese" | "Chinese" => {
            Some(Idioma::ChinoSimplificado)
        }
        _ => Idioma::desde_etiqueta_del_sistema(limpia),
    }
}

/// Variable de entorno que delimita dónde puede leer y escribir el servidor.
///
/// Es pública dentro del programa porque hay dos partes que deben coincidir: la que la
/// **escribe** en la configuración del agente (`conectores`) y la que la **lee** al arrancar
/// el servidor (este módulo). Estuvo declarada por duplicado, una copia en cada sitio;
/// renombrar una de las dos habría dejado al servidor sin espacio definido sin que nada
/// fallara al compilar.
pub(crate) const VARIABLE_ESPACIO_TRABAJO: &str = "MMCELT_WORKSPACE";

/// Sufijo del archivo Markdown que acompaña a cada mapa.
const SUFIJO_MARKDOWN_IA: &str = "_AI.md";

/// Versión que el servidor declara en el saludo inicial.
const VERSION_SERVIDOR: &str = env!("CARGO_PKG_VERSION");

/// Versión del protocolo MCP que se implementa.
const VERSION_PROTOCOLO: &str = "2024-11-05";

/// Versiones MCP que esta implementación entiende de verdad.
const VERSIONES_PROTOCOLO_SOPORTADAS: [&str; 1] = [VERSION_PROTOCOLO];

/// Negocia la versión MCP sin anunciar contratos que este servidor no implementa.
///
/// MMCelt implementa actualmente una única versión. Si el cliente pide esa, se confirma; si
/// pide otra o no declara ninguna, se devuelve la única soportada para que el cliente decida si
/// puede continuar según las reglas de MCP.
fn version_de_protocolo_negociada(parametros: &ParametrosMcp) -> &str {
    let solicitada = parametros.texto("protocolVersion");
    VERSIONES_PROTOCOLO_SOPORTADAS
        .iter()
        .copied()
        .find(|version| Some(*version) == solicitada)
        .unwrap_or(VERSION_PROTOCOLO)
}

/// Resultado de una herramienta MCP.
type ResultadoHerramienta = Result<Value, ErrorHerramienta>;

/// Arranca el bucle del servidor y no retorna hasta que se cierra la entrada estándar.
///
/// # Devuelve
/// El código de salida del proceso: 0 siempre que el bucle termine con normalidad.
pub fn ejecutar() -> i32 {
    let entrada = std::io::stdin();
    let mut entrada = entrada.lock();
    let mut salida = std::io::stdout();

    loop {
        let linea =
            match protocolo::leer_linea_acotada(&mut entrada, protocolo::BYTES_MAXIMOS_MENSAJE_MCP)
            {
                Ok(Some(linea)) => linea,
                Ok(None) => break,
                Err(crate::error::AppError::EntradaDemasiadoGrande { limite, .. }) => {
                    let respuesta = protocolo::respuesta_entrada_demasiado_grande(limite);
                    if writeln!(salida, "{respuesta}").is_err() || salida.flush().is_err() {
                        break;
                    }
                    continue;
                }
                Err(_) => break, // Entrada cerrada o ilegible: el agente ha terminado.
            };

        if linea.trim().is_empty() {
            continue;
        }

        let respuesta = procesar_peticion(&linea);

        // Una petición de notificación (sin `id`) no lleva respuesta.
        if let Some(respuesta) = respuesta {
            if writeln!(salida, "{respuesta}").is_err() || salida.flush().is_err() {
                break; // El agente cerró la tubería.
            }
        }
    }

    0
}

/// Interpreta una línea de petición JSON-RPC y produce la respuesta.
///
/// El trabajo está en [`protocolo::procesar_linea`], que valida el sobre antes de dejar que
/// nada llegue al despacho. Este nombre se conserva porque es el que usan el bucle del
/// servidor y las pruebas.
///
/// Es `pub(crate)` para que las pruebas puedan ejercitar el servidor **sin lanzar un
/// proceso aparte**: se le pasa una petición y se comprueba la respuesta, que es todo lo
/// que hace el bucle principal.
///
/// # Devuelve
/// La respuesta a enviar, o `None` si la petición era una notificación.
pub(crate) fn procesar_peticion(linea: &str) -> Option<String> {
    protocolo::procesar_linea(linea)
}

/// Atiende un método MCP a partir de una petición **cuyo sobre ya está validado**.
///
/// Es la función adaptadora entre la frontera del protocolo —[`protocolo`]— y el despacho de
/// herramientas de este módulo. Aquí ya no se comprueba la versión, ni el identificador, ni
/// que el método sea texto: si esta función se ejecuta, es porque todo eso estaba bien.
///
/// # Parámetros
/// - `peticion`: la llamada validada, con su método y sus parámetros.
///
/// # Devuelve
/// El contenido del campo `result` de la respuesta, o el error del protocolo cuando el método
/// no se ofrece.
pub(super) fn atender_metodo(peticion: &PeticionJsonRpc) -> Result<Value, ErrorJsonRpc> {
    match peticion.metodo() {
        // `instructions` es lo primero que lee un agente al conectarse, y es donde el
        // protocolo espera que el servidor explique cómo se usa. Sin ese campo, el agente
        // empezaba sin saber que existe un espacio acotado ni cuál era, así que inventaba
        // rutas y cada intento chocaba contra una valla que nadie le había anunciado.
        "initialize" => Ok(json!({
            "protocolVersion": version_de_protocolo_negociada(peticion.parametros()),
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "mmcelt-mcp", "version": VERSION_SERVIDOR },
            "instructions": instrucciones_para_el_agente()
        })),

        "tools/list" => Ok(json!({ "tools": super::catalogo::descripcion_herramientas() })),

        "tools/call" => llamar_a_una_herramienta(peticion.parametros()),

        otro => Err(ErrorJsonRpc::metodo_no_encontrado(otro)),
    }
}

/// Ejecuta la herramienta que piden los parámetros y envuelve su salida como contenido MCP.
///
/// El error de una herramienta **no** es un error del protocolo: la llamada se atendió y lo
/// que se devuelve es lo que ha pasado al ejecutarla, en un texto que el modelo pueda leer y
/// con el que pueda reaccionar. Por eso viaja dentro de `result` y no dentro de `error`.
///
/// # Devuelve
/// El contenido de la respuesta a `tools/call`.
fn llamar_a_una_herramienta(parametros: &ParametrosMcp) -> Result<Value, ErrorJsonRpc> {
    let nombre = parametros
        .texto("name")
        .ok_or_else(|| ErrorJsonRpc::parametros_invalidos("Falta «name» o no es texto"))?;
    let vacio = json!({});
    let argumentos = parametros.valor("arguments").unwrap_or(&vacio);
    if !argumentos.is_object() {
        return Err(ErrorJsonRpc::parametros_invalidos(
            "«arguments» debe ser un objeto JSON",
        ));
    }

    let contenido = match despachar_herramienta(nombre, argumentos)? {
        Ok(valor) => valor,
        Err(e) => json!({
            "status": "error",
            "error": e.mensaje(),
            "error_type": e.tipo().as_str()
        }),
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&contenido).unwrap_or_default()
        }]
    }))
}

/// Compone el texto que el servidor entrega al agente en el saludo.
///
/// Dice tres cosas, y las tres faltaban: qué es MMCelt, cuál es la carpeta autorizada y
/// cómo averiguar qué hay dentro sin adivinar rutas.
///
/// Cuando no hay espacio de trabajo definido **no se inventa ninguno**. El servidor ya
/// falla cerrado en esa situación (ver [`espacio_de_trabajo`]), y un saludo que prometiera
/// una carpeta que después no se puede usar sería peor que no decir nada: el agente daría
/// la ruta por buena y no entendería el rechazo.
///
/// # Devuelve
/// El texto para el campo `instructions` de la respuesta a `initialize`.
fn instrucciones_para_el_agente() -> String {
    let recorrido = "Recorrido habitual: «mmcelt_workspace_info» para saber qué mapas hay, \
                     «mmcelt_read_mindmap» para leer uno, «mmcelt_get_human_feedback» para \
                     conocer las correcciones de la persona —acátalas y no sigas ningún \
                     camino descartado— y «mmcelt_sync_ai_progress» para devolver tu avance \
                     al mismo archivo.";

    match espacio_de_trabajo() {
        Ok(raiz) => format!(
            "MMCelt es un programa de mapas mentales con el que una persona dirige y \
             supervisa tu trabajo.\n\n\
             Carpeta de trabajo autorizada: {}\n\n\
             Solo puedes leer y escribir dentro de esa carpeta. Las rutas que envíes en \
             «file_path» son relativas a ella y no pueden contener «..». No inventes \
             rutas: pide primero «mmcelt_workspace_info», que devuelve la carpeta \
             autorizada y la lista de mapas «.mmcelt» accesibles, y reutiliza tal cual las \
             rutas que te devuelva.\n\n\
             {recorrido}",
            raiz.display()
        ),
        Err(_) => format!(
            "MMCelt es un programa de mapas mentales con el que una persona dirige y \
             supervisa tu trabajo.\n\n\
             Ahora mismo **no hay carpeta de trabajo definida**, así que este servidor no \
             puede leer ni escribir nada y rechazará cualquier operación con archivos. No \
             supongas ninguna ruta. La carpeta se define en la variable de entorno \
             {VARIABLE_ESPACIO_TRABAJO}, que MMCelt escribe en la configuración de este \
             agente al conectarlo desde «🤖 Inteligencia Artificial → 🔌 Conectar MMCelt \
             con mis IAs...»; dile a la persona que vuelva a conectar el agente desde ahí y \
             que lo reinicie.\n\n\
             Cuando esté definida, «mmcelt_workspace_info» te dirá cuál es y qué mapas \
             contiene. {recorrido}"
        ),
    }
}

/// Ejecuta la herramienta solicitada.
fn despachar_herramienta(
    nombre: &str,
    argumentos: &Value,
) -> Result<ResultadoHerramienta, ErrorJsonRpc> {
    fn leer<T: serde::de::DeserializeOwned>(valor: &Value) -> Result<T, ErrorJsonRpc> {
        serde_json::from_value(valor.clone()).map_err(|error| {
            ErrorJsonRpc::parametros_invalidos(format!("Argumentos no válidos: {error}"))
        })
    }

    let resultado = match nombre {
        "mmcelt_workspace_info" => {
            let _: ArgumentosSinContenido = leer(argumentos)?;
            informar_del_espacio()
        }
        "mmcelt_create_mindmap" => crear_mapa(leer(argumentos)?),
        "mmcelt_read_mindmap" => leer_mapa(&leer(argumentos)?),
        "mmcelt_get_human_feedback" => obtener_correcciones(&leer(argumentos)?),
        "mmcelt_sync_ai_progress" => {
            let argumentos: ArgumentosSincronizar = leer(argumentos)?;
            argumentos
                .validar()
                .map_err(ErrorJsonRpc::parametros_invalidos)?;
            sincronizar_progreso(&argumentos.into())
        }
        "mmcelt_export_ai_markdown" => exportar_markdown(&leer(argumentos)?),
        otro => Err(ErrorHerramienta::argumento(format!(
            "Herramienta desconocida: {otro}"
        ))),
    };
    Ok(resultado)
}

// ============================================================================
// Inventario del espacio de trabajo
// ============================================================================

/// Extensión de los mapas que se enumeran en el informe del espacio.
///
/// Solo `.mmcelt`, aunque un mapa también pueda guardarse como `.json`. El espacio de
/// trabajo suele ser la carpeta del proyecto del usuario, donde casi todos los `.json` son
/// suyos —`package.json`, `tsconfig.json`, ajustes del editor—, y enumerarlos le daría al
/// agente un inventario de archivos ajenos en lugar de la lista de mapas que necesita.
const EXTENSION_DE_MAPA: &str = "mmcelt";

/// Hasta qué hondura se busca dentro del espacio de trabajo.
///
/// Es un tope, no una preferencia: el espacio puede ser la raíz de un proyecto con miles de
/// carpetas anidadas, y el informe tiene que responder en un tiempo acotado. Los mapas de
/// un proyecto viven cerca de la raíz; seis niveles cubren de sobra el caso real.
const PROFUNDIDAD_MAXIMA_DEL_INVENTARIO: usize = 6;

/// Cuántos mapas se enumeran como mucho.
///
/// Pasado ese número el informe lo dice con `truncated`, en lugar de devolver una lista
/// interminable que el agente no puede procesar ni la respuesta transportar con holgura.
const MAPAS_MAXIMOS_ENUMERADOS: usize = 500;

/// Un mapa encontrado dentro del espacio de trabajo.
///
/// Se recopilan todos antes de componer la respuesta porque hay que **ordenarlos**: dos
/// consultas seguidas sobre lo mismo tienen que devolver lo mismo en el mismo orden, y el
/// orden en que el sistema de archivos entrega las entradas no lo garantiza.
struct MapaDelEspacio {
    /// Ruta relativa al espacio de trabajo, con barras normales.
    ///
    /// Es la forma en que el agente tiene que volver a enviarla en `file_path`, así que se
    /// devuelve tal cual para que no tenga que componer nada.
    ruta_relativa: String,
    /// Tamaño del archivo en bytes.
    bytes: u64,
    /// Última modificación en formato RFC 3339, o `None` si el sistema no la informa.
    modificado: Option<String>,
}

/// Recorre el espacio de trabajo recogiendo los mapas que hay dentro.
///
/// # Lo que no hace
///
/// - **No sigue enlaces simbólicos.** Un enlace dentro del espacio apuntando fuera
///   convertiría el informe en una ventana al resto del disco, que es exactamente lo que
///   `validar_ruta` impide al escribir. El tipo de entrada se consulta sin resolver el
///   enlace, así que un enlace se descarta entero, sea a carpeta o a archivo.
/// - **No entra en carpetas ocultas.** Ahí no hay mapas del usuario y sí puede haber un
///   `.git` con decenas de miles de archivos.
/// - **No abre ningún archivo.** Solo mira el nombre y los datos que el sistema ya tiene.
///
/// # Parámetros
/// - `carpeta`: la carpeta que se examina en esta vuelta.
/// - `raiz`: el espacio de trabajo, para calcular las rutas relativas.
/// - `profundidad`: cuántos niveles se han bajado ya desde la raíz.
/// - `mapas`: donde se acumula lo encontrado.
///
/// # Devuelve
/// `false` si se alcanzó el tope de [`MAPAS_MAXIMOS_ENUMERADOS`] y quedaron mapas sin
/// enumerar; `true` si el recorrido llegó hasta el final.
fn recopilar_mapas(
    carpeta: &Path,
    raiz: &Path,
    profundidad: usize,
    mapas: &mut Vec<MapaDelEspacio>,
) -> bool {
    if profundidad > PROFUNDIDAD_MAXIMA_DEL_INVENTARIO {
        return true;
    }

    // Una carpeta ilegible no detiene el informe: se omite. El agente recibe lo que sí se
    // ha podido enumerar, que es más útil que un error por un permiso de otra carpeta.
    let Ok(entradas) = std::fs::read_dir(carpeta) else {
        return true;
    };

    for entrada in entradas.flatten() {
        if mapas.len() >= MAPAS_MAXIMOS_ENUMERADOS {
            return false;
        }

        // `file_type` no resuelve el enlace: informa de que **es** un enlace, que es lo que
        // hace falta para descartarlo sin llegar a mirar adónde apunta.
        let Ok(tipo) = entrada.file_type() else {
            continue;
        };
        if tipo.is_symlink() {
            continue;
        }

        let ruta = entrada.path();

        if tipo.is_dir() {
            // El nombre solo hace falta para esta comprobación, así que se pide aquí y no
            // para cada entrada: el recorrido pasa por todos los archivos del espacio.
            if entrada.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            if !recopilar_mapas(&ruta, raiz, profundidad + 1, mapas) {
                return false;
            }
            continue;
        }

        if !tipo.is_file() || !es_un_mapa(&ruta) {
            continue;
        }

        let Ok(relativa) = ruta.strip_prefix(raiz) else {
            continue;
        };

        let metadatos = entrada.metadata().ok();

        mapas.push(MapaDelEspacio {
            ruta_relativa: relativa.display().to_string().replace('\\', "/"),
            bytes: metadatos.as_ref().map(|m| m.len()).unwrap_or(0),
            modificado: metadatos
                .and_then(|m| m.modified().ok())
                .map(|instante| chrono::DateTime::<chrono::Utc>::from(instante).to_rfc3339()),
        });
    }

    true
}

/// Dice si un archivo es un mapa de MMCelt por su extensión.
///
/// Sin distinguir mayúsculas, como el resto del módulo: `PLAN.MMCELT` es el mismo archivo
/// que `plan.mmcelt` en Windows y en macOS.
fn es_un_mapa(ruta: &Path) -> bool {
    ruta.extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .as_deref()
        == Some(EXTENSION_DE_MAPA)
}

/// Implementa `mmcelt_workspace_info`.
///
/// Es la herramienta que faltaba: la única de solo lectura que no toca ningún archivo del
/// usuario y responde a la pregunta con la que empieza cualquier agente —«¿dónde puedo
/// trabajar y qué hay ahí?»—. Sin ella, el agente deducía la carpeta autorizada de los
/// mensajes de error, o se la inventaba.
///
/// No se exceptúa del fallo cerrado. Al contrario: sin espacio de trabajo definido, decir
/// qué hay en el disco es precisamente lo que no se puede hacer.
///
/// # Devuelve
/// La carpeta autorizada ya resuelta y la lista de mapas que contiene, en orden estable y
/// con rutas relativas listas para reenviar en `file_path`.
///
/// # Errores
/// Error de seguridad si no hay espacio de trabajo definido o no se puede resolver.
fn informar_del_espacio() -> ResultadoHerramienta {
    let raiz = espacio_de_trabajo()?;

    let mut mapas = Vec::new();
    let completo = recopilar_mapas(&raiz, &raiz, 0, &mut mapas);

    // El orden lo fija la ruta, no el sistema de archivos: dos consultas seguidas sobre lo
    // mismo deben devolver lo mismo, o el agente cree que algo se ha movido.
    mapas.sort_by(|uno, otro| uno.ruta_relativa.cmp(&otro.ruta_relativa));

    let listado: Vec<Value> = mapas
        .iter()
        .map(|mapa| {
            json!({
                "path": mapa.ruta_relativa,
                "bytes": mapa.bytes,
                "modified": mapa.modificado,
            })
        })
        .collect();

    Ok(json!({
        "status": "success",
        "workspace": raiz.display().to_string(),
        "total_mindmaps": listado.len(),
        "truncated": !completo,
        "mindmaps": listado,
        "instructions_for_ai": "Estas rutas son relativas a «workspace» y se envían tal cual \
                                en «file_path». No compongas rutas absolutas ni salgas de la \
                                carpeta: cualquier ruta con «..» se rechaza.",
    }))
}

/// Dice si todo lo que hay en un mapa lo ha escrito la IA.
///
/// Es **el** criterio para distinguir un borrador del agente del trabajo de una persona, y
/// vive aquí, en una sola función, porque lo usan dos sitios que tienen que estar de
/// acuerdo: la valla que impide rehacer un mapa encima de otro y la excepción de copia de
/// la sincronización de progreso. Estuvieron en desacuerdo, y la segunda destruía lo que la
/// primera protegía.
///
/// Solo lleva [`crate::model::EstadoRevision::GeneradoPorIA`] lo que ha pasado por
/// [`crate::ai_bridge::marcar_todo_como_trabajo_de_la_ia`] o por la creación de nodos de la
/// propia sincronización. Cualquier otro estado significa que hay una persona detrás.
///
/// # Parámetros
/// - `proyecto`: el mapa que se está a punto de escribir.
///
/// # Devuelve
/// `true` si ningún nodo tiene un estado de revisión distinto del de la IA.
pub(crate) fn el_mapa_es_solo_de_la_ia(proyecto: &Proyecto) -> bool {
    proyecto
        .nodes
        .values()
        .all(|nodo| nodo.review_status == crate::model::EstadoRevision::GeneradoPorIA)
}

/// Índice del título normalizado a **todos** los nodos que lo llevan.
///
/// Que el valor sea una lista y no un identificador suelto es el hallazgo H4 hecho tipo: un
/// título no identifica a un nodo, y el programa no puede fingir que sí.
type IndiceDeTitulos = std::collections::HashMap<String, Vec<uuid::Uuid>>;

/// Construye un índice de título en minúsculas a los nodos que lo llevan.
///
/// La sincronización del progreso busca cada nodo por su título. Se hacía recorriendo el mapa
/// entero por cada actualización y pasando a minúsculas el título de **cada** nodo en el
/// camino, lo que reserva una cadena nueva por nodo y por actualización: con el máximo
/// admitido de cien mil nodos y un lote de mil actualizaciones salían cien millones de
/// reservas para resolver mil búsquedas.
///
/// Con el índice, el mapa se recorre una vez y cada búsqueda es una consulta directa.
///
/// # Por qué guarda una lista y no un identificador
///
/// Es el hallazgo H4. Antes, cuando dos nodos compartían título —cosa nada rara: dos
/// «Pruebas», dos «README.md» del escáner, y sobre todo los dos «Sí» y los dos «No» de
/// cualquier diagrama de decisiones—, ganaba el de identificador menor. La elección era
/// estable entre ejecuciones, pero seguía siendo una elección a ciegas: el UUID es aleatorio y
/// no guarda ninguna relación con el nodo que el agente quería tocar. Escribir sobre el nodo
/// equivocado devolviendo `success` es exactamente el fallo que corrompe en silencio.
///
/// Guardando los candidatos, quien pregunta puede distinguir los tres casos que importan:
/// ninguno —hay que crear el nodo—, uno —se sabe cuál es— y varios —hay que rechazar la
/// operación y decir cuáles son—. Ver [`resolver_nodo_senalado`].
///
/// # Parámetros
/// - `proyecto`: el mapa sobre el que se van a aplicar las actualizaciones.
///
/// # Devuelve
/// Un índice del título normalizado —sin espacios alrededor y en minúsculas— a la lista de
/// identificadores que lo llevan, en el orden estable de `Proyecto::nodes`, que es un mapa
/// ordenado por identificador. Ese orden es el que hace reproducible la lista de candidatos
/// del rechazo, no un criterio para elegir entre ellos.
fn indexar_por_titulo(proyecto: &Proyecto) -> IndiceDeTitulos {
    let mut indice: IndiceDeTitulos = std::collections::HashMap::new();
    for (id, nodo) in &proyecto.nodes {
        indice
            .entry(nodo.title.trim().to_lowercase())
            .or_default()
            .push(*id);
    }
    indice
}

/// Lee un campo de texto que puede no venir, ya recortado y descartando el vacío.
///
/// Una cadena vacía no es un valor: significa que el agente no dice nada de ese campo. Es el
/// mismo criterio que ya se aplicaba a las notas y a la ruta de código.
fn texto_no_vacio(texto: Option<&str>) -> Option<&str> {
    texto.map(str::trim).filter(|texto| !texto.is_empty())
}

/// Escribe la lista de candidatos de un título ambiguo, con su título y su UUID.
///
/// Es lo único que permite al agente reintentar sin adivinar: sin los identificadores, el
/// rechazo le diría que el título no vale y no le daría con qué sustituirlo.
fn listar_candidatos(proyecto: &Proyecto, ids: &[uuid::Uuid]) -> String {
    ids.iter()
        .map(|id| {
            let titulo = proyecto
                .nodes
                .get(id)
                .map_or("(sin título)", |nodo| nodo.title.as_str());
            format!("«{titulo}» = {id}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Averigua a qué nodo del mapa se refiere una entrada que puede traer título, identificador
/// o los dos.
///
/// Es la pieza central de los hallazgos H4 y H5, y la usan los cuatro sitios donde el agente
/// nombra un nodo: el nodo de una actualización, su padre y los dos extremos de una conexión
/// cruzada. Que sea una sola función es lo que garantiza que las cuatro se comporten igual;
/// tener una regla distinta en cada sitio fue justamente lo que hubo que documentar en H3.
///
/// # La regla
///
/// - **Manda el identificador.** Si viene, se usa; si además viene el título, tiene que
///   corresponder al nodo, y si no corresponde se rechaza. Cuando llegan los dos no sobra
///   ninguno: el identificador dice cuál es el nodo y el título dice cuál creía el agente que
///   era, así que la discrepancia significa que su tabla está caducada —alguien renombró el
///   nodo, o la leyó de otro mapa— y aplicar el cambio de todas formas lo pondría sobre un
///   nodo pensado para otro.
/// - **Un identificador que no existe se rechaza.** Un `id` es una afirmación: «este nodo ya
///   está en el mapa». Tratarlo como un título desconocido y crear un nodo nuevo escondería el
///   error del agente debajo de un nodo que nadie ha pedido.
/// - **Un título único vale exactamente como antes.** Es la mitad compatible del contrato: la
///   llamada que un agente escribía antes de este arreglo sigue haciendo lo mismo.
/// - **Un título repetido se rechaza enumerando los candidatos.** Nunca se elige por orden
///   interno.
/// - **Un título que no está en el mapa devuelve `None`**, que no es un error: quien pregunta
///   decide si eso significa «crea el nodo», «cuelga de la raíz y avisa» o «no dibujes la
///   arista». No saber dónde va algo no es lo mismo que tener dos sitios y elegir uno a
///   escondidas.
///
/// # Parámetros
/// - `proyecto`: el mapa donde se busca.
/// - `indice`: el índice de títulos, al día con los nodos creados en esta misma llamada.
/// - `objeto`: el fragmento JSON que mandó el agente.
/// - `campo_id`: cómo se llama ahí el identificador (`id`, `parent_id`, `from_id`, `to_id`).
/// - `campo_titulo`: cómo se llama ahí el título (`title`, `parent_title`, …).
///
/// # Devuelve
/// El identificador del nodo señalado, o `None` si no se nombró ninguno o si el título que se
/// nombró no está en el mapa.
///
/// # Errores
/// [`ErrorHerramienta::argumento`] si el identificador no es un UUID, si no existe, si
/// contradice al título que lo acompaña o si el título nombra a varios nodos. Los cuatro son
/// problemas de identidad, y ninguno se resuelve adivinando: quien llama debe abandonar la
/// operación entera sin escribir nada.
fn resolver_nodo_senalado(
    proyecto: &Proyecto,
    indice: &IndiceDeTitulos,
    referencia: ReferenciaNodoPrestada<'_>,
    campo_id: &str,
) -> Result<Option<uuid::Uuid>, ErrorHerramienta> {
    let titulo = texto_no_vacio(referencia.titulo);

    if let Some(texto) = texto_no_vacio(referencia.id) {
        let id = uuid::Uuid::parse_str(texto).map_err(|_| {
            ErrorHerramienta::argumento(format!(
                "«{campo_id}» no es un identificador válido: «{texto}». Los identificadores son \
                 los UUID que devuelve «mmcelt_read_mindmap» en cada nodo. No se ha escrito nada."
            ))
        })?;

        let Some(nodo) = proyecto.nodes.get(&id) else {
            return Err(ErrorHerramienta::argumento(format!(
                "No hay ningún nodo con el identificador {id} en este mapa. Vuelve a leerlo con \
                 «mmcelt_read_mindmap»: puede que el nodo ya no exista o que ese identificador \
                 sea de otro mapa. No se ha escrito nada."
            )));
        };

        if let Some(titulo) = titulo {
            if nodo.title.trim().to_lowercase() != titulo.to_lowercase() {
                return Err(ErrorHerramienta::argumento(format!(
                    "El identificador {id} corresponde al nodo «{}», no a «{titulo}». Si el mapa \
                     ha cambiado, vuelve a leerlo con «mmcelt_read_mindmap». No se ha escrito \
                     nada.",
                    nodo.title
                )));
            }
        }

        return Ok(Some(id));
    }

    let Some(titulo) = titulo else {
        return Ok(None);
    };

    match indice.get(&titulo.to_lowercase()).map(Vec::as_slice) {
        None | Some([]) => Ok(None),
        Some([id]) => Ok(Some(*id)),
        Some(candidatos) => Err(ErrorHerramienta::argumento(format!(
            "El título «{titulo}» corresponde a {} nodos de este mapa, así que no basta para \
             señalar uno y no se elige por su cuenta. Repite la llamada indicando «{campo_id}» \
             con el UUID del nodo que quieres. Candidatos: {}. No se ha escrito nada.",
            candidatos.len(),
            listar_candidatos(proyecto, candidatos)
        ))),
    }
}

// ============================================================================
// Herramientas
// ============================================================================

/// Se niega a escribir encima de un mapa que no sea íntegramente obra de la IA.
///
/// `mmcelt_sync_ai_progress` está vallada con cuidado: no revive un nodo descartado, caduca
/// la aprobación humana en cuanto la IA toca el contenido y nunca borra las correcciones. La
/// cabecera de este módulo concluye que «lo que puede estropear está acotado».
///
/// No lo estaba. `mmcelt_create_mindmap` recibe la misma ruta, no comprobaba nada y escribía
/// encima: un agente al que no le gustara un veto solo tenía que llamar a esta herramienta en
/// lugar de a la de sincronizar, y **todos** los descartes, las correcciones exigidas y las
/// aprobaciones del usuario desaparecían de golpe, sustituidos por un mapa que se marca
/// entero como trabajo de la IA. La valla de al lado se rodeaba sin esfuerzo.
///
/// # Por qué no basta con contar las marcas de supervisión
///
/// La primera versión contaba los nodos con corrección, descarte o aprobación. Un mapa que
/// la persona haya escrito a mano en la aplicación, sin tocar nunca el desplegable «Control
/// Humano», **no tiene ninguna de esas tres marcas**: sus nodos nacen en
/// [`crate::model::EstadoRevision::PendienteRevision`], que es el valor por omisión. Es decir, el trabajo
/// más habitual del usuario —abrir el programa y escribir su mapa— era justo el que no
/// protegía nada, y un agente podía sustituirlo entero por un nodo suyo.
///
/// El criterio correcto no es «cuánta supervisión hay», sino «¿esto lo escribió la IA?».
/// Solo lleva [`crate::model::EstadoRevision::GeneradoPorIA`] lo que ha pasado por
/// [`crate::ai_bridge::marcar_todo_como_trabajo_de_la_ia`], que es exactamente lo que hace
/// esta herramienta con lo que crea. Cualquier otro estado en cualquier nodo significa que
/// hay una persona detrás, y entonces el mapa no se rehace: se sincroniza.
///
/// Rehacer un borrador que escribió la propia IA sigue permitido; no le quita nada a nadie y
/// es el uso normal del agente.
///
/// # Parámetros
/// - `ruta`: el archivo donde se quiere crear el mapa, ya validado.
///
/// # Errores
/// Devuelve un error de seguridad si el archivo existe, se puede leer y alguno de sus nodos
/// no es trabajo de la IA. Un archivo ilegible o con otro formato no detiene la creación: no
/// hay nada reconocible que proteger, y de lo que hubiera queda copia igualmente, porque
/// [`PoliticaDeCopia::DejarCopia`] no admite excepciones.
fn rechazar_si_pisa_trabajo_de_la_persona(ruta: &Path) -> Result<(), ErrorHerramienta> {
    if !ruta.exists() {
        return Ok(());
    }

    let Ok(existente) = crate::storage::cargar_proyecto_de_archivo(ruta) else {
        // Existe y no es un mapa: entonces es un archivo de la persona, y esta herramienta no
        // tiene nada que hacer ahí.
        //
        // Antes esto devolvía `Ok(())` —«no es un mapa, no hay trabajo que proteger»—, y el
        // razonamiento estaba justo del revés. `.json` es una extensión válida para escribir
        // mapas y el espacio de trabajo suele ser la carpeta del proyecto del usuario, así
        // que el blanco natural eran su `package.json`, su `tsconfig.json` o su
        // `.vscode/settings.json`. Comprobado: un agente los sobrescribía enteros con
        // respuesta `success`, dejando el archivo vivo inservible.
        //
        // Que quedara un `.bak` no lo salva: quien se encuentra el proyecto sin arrancar no
        // sabe que hay una copia con marca de tiempo al lado.
        return Err(ErrorHerramienta::seguridad(format!(
            "«{}» ya existe y no es un mapa mental de MMCelt. No se sobrescribe: si de \
             verdad quieres escribir ahí, elige otro nombre o retira antes ese archivo.",
            ruta.file_name()
                .map(|nombre| nombre.to_string_lossy().into_owned())
                .unwrap_or_else(|| "el destino".to_string())
        )));
    };

    // El criterio es **uno solo**: un nodo es de la persona cuando su estado de revisión no
    // dice que lo escribió la IA.
    //
    // Hubo un segundo criterio en paralelo, y estaba mal. Contaba también los nodos que
    // `exige_correccion_del_usuario()` señala, y ese método incluye los **descartados** sin
    // mirar quién los descartó. El esquema publicado le ofrece «Descartado» al agente y la
    // habilidad se lo documenta, así que en cuanto un agente descartaba una vía propia, su
    // propio mapa quedaba cerrado para siempre a esta herramienta —y el mensaje le echaba la
    // culpa a una persona que no existía—. Una valla que da falsos positivos sobre el uso
    // normal acaba desactivada, así que el error no era menor por ser en la dirección segura.
    if el_mapa_es_solo_de_la_ia(&existente) {
        return Ok(());
    }

    let de_la_persona: Vec<&crate::model::Nodo> = existente
        .nodes
        .values()
        .filter(|nodo| nodo.review_status != crate::model::EstadoRevision::GeneradoPorIA)
        .collect();

    // Entre lo que es de la persona, se distingue lo que además lleva un veto explícito:
    // el mensaje no dice lo mismo en los dos casos.
    let supervisados = de_la_persona
        .iter()
        .filter(|nodo| {
            nodo.exige_correccion_del_usuario()
                || nodo.review_status == crate::model::EstadoRevision::AprobadoPorHumano
        })
        .count();

    if supervisados > 0 {
        return Err(ErrorHerramienta::seguridad(format!(
            "Ese archivo ya contiene un mapa con {supervisados} nodos revisados por la \
             persona (correcciones exigidas, descartes o aprobaciones). Crear uno nuevo \
             encima los borraría todos. Usa «mmcelt_sync_ai_progress» para actualizar el \
             mapa existente, o crea el nuevo en otra ruta."
        )));
    }

    Err(ErrorHerramienta::seguridad(format!(
        "Ese archivo contiene un mapa con {} nodos que no ha escrito la IA. Crear uno nuevo \
         encima lo sustituiría entero. Usa «mmcelt_sync_ai_progress» para añadir tu trabajo \
         al mapa existente, o crea el nuevo en otra ruta.",
        de_la_persona.len()
    )))
}

/// Implementa `mmcelt_create_mindmap`.
fn crear_mapa(argumentos: ArgumentosCrearMapa) -> ResultadoHerramienta {
    let ruta = validar_ruta(&argumentos.file_path, UsoDeLaRuta::EscrituraDeMapa)?;
    if argumentos.file_path.trim().is_empty() {
        return Err(ErrorHerramienta::argumento(
            "La ruta del archivo no puede estar vacía.",
        ));
    }
    let titulo = argumentos.title.trim();
    if titulo.is_empty() {
        return Err(ErrorHerramienta::argumento(
            "El título del proyecto no puede estar vacío.",
        ));
    }

    rechazar_si_pisa_trabajo_de_la_persona(&ruta)?;

    // El mapa se construye con el mismo conversor que usa la importación desde la
    // interfaz. Antes había aquí una segunda implementación, sobre `serde_json::Value`, que
    // leía el mismo formato: se desincronizó dos veces por el mismo motivo —campos del nodo
    // que uno entendía y el otro no— hasta que se unificaron.
    let mut proyecto = crate::ai_bridge::construir_proyecto_desde_json(argumentos.mindmap_data)
        .map_err(|e| ErrorHerramienta::argumento(e.mensaje_usuario().en(idioma_del_usuario())))?;

    // Lo construye un agente: todo lo que hay en el mapa lo ha escrito la IA, así que no
    // puede quedar ni un nodo con apariencia de revisado ni con una corrección del usuario.
    crate::ai_bridge::marcar_todo_como_trabajo_de_la_ia(&mut proyecto);

    // El título y el contexto llegan en los argumentos de la herramienta, no dentro del
    // mapa, así que se aplican después de construirlo.
    proyecto.title = titulo.to_string();
    proyecto.author = "Agente MCP".to_string();
    // El contexto del proyecto puede llegar por dos vías: como argumento de la herramienta
    // o dentro del propio `mindmap_data`, que es donde lo pone la habilidad para agentes.
    // El argumento manda cuando trae algo; si no, se conserva lo que trajera el mapa.
    //
    // Antes se asignaba siempre, con `unwrap_or("")`, y era inofensivo porque el servidor
    // construía el mapa con un conversor propio que no leía estos campos. Al unificarlo con
    // el de la importación —que sí los lee— el supuesto dejó de valer: un agente que
    // siguiera la habilidad al pie de la letra guardaba el mapa con la visión en blanco.
    // Es el mismo defecto que la pérdida de `file_path`, en otro sitio.
    if let Some(valor) = texto_no_vacio(argumentos.creator_vision.as_deref()) {
        proyecto.creator_vision = valor.to_string();
    }
    if let Some(valor) = texto_no_vacio(argumentos.project_goals.as_deref()) {
        proyecto.project_goals = valor.to_string();
    }
    if let Some(valor) = texto_no_vacio(argumentos.target_audience.as_deref()) {
        proyecto.target_audience_or_context = valor.to_string();
    }

    crate::layout::aplicar_disposicion_automatica(&mut proyecto);

    let devolucion = preparar_devolucion(
        &ruta,
        argumentos.agent_name.as_deref(),
        OperacionDevolucion::Generado,
    )?;

    validar_el_mapa_antes_de_escribirlo(&proyecto)?;

    let (ruta_md, markdown, respaldos) = escribir_el_mapa_y_su_markdown(&ruta, &proyecto)?;

    if let Some((contexto, recibo)) = &devolucion {
        publicar_devolucion(contexto, recibo)?;
    }

    Ok(json!({
        "status": "success",
        "delivery_receipt": devolucion.as_ref().map(|(_, recibo)| recibo),
        "message": format!("Mapa mental creado en {} y versión IA en {}", ruta.display(), ruta_md.display()),
        "total_nodes": proyecto.nodes.len(),
        "backups_created": respaldos,
        "md_preview": markdown.chars().take(500).collect::<String>(),
    }))
}

/// Comprueba que un mapa recién construido por un agente se puede escribir sin arriesgar nada.
///
/// # Las dos garantías, y por qué son distintas
///
/// - **Estructura**: la misma frontera de confianza que aplica la aplicación al abrir un
///   archivo. Lo que se escribe en disco tiene que poder abrirse después; un mapa con un ciclo
///   o con la raíz colgando de otro nodo congela los recorridos de `layout` y de `model`.
/// - **Longitud de los textos**: esta **solo** se aplica aquí, y no al abrir un archivo del
///   usuario. Lo que se vigila no es la corrección del mapa, sino que un agente en bucle no
///   llene el disco con textos enormes. Una persona puede escribir lo que quiera en sus notas.
///
/// # Parámetros
/// - `proyecto`: el mapa a punto de escribirse.
///
/// # Errores
/// [`ErrorHerramienta::ejecucion`] con el mensaje que la validación le daría al usuario.
fn validar_el_mapa_antes_de_escribirlo(proyecto: &Proyecto) -> Result<(), ErrorHerramienta> {
    let idioma = idioma_del_usuario();
    proyecto
        .validar_estructura()
        .map_err(|e| ErrorHerramienta::ejecucion(e.mensaje_usuario().en(idioma)))?;

    proyecto
        .validar_longitudes_de_texto()
        .map_err(|e| ErrorHerramienta::ejecucion(e.mensaje_usuario().en(idioma)))?;

    Ok(())
}

/// Escribe el mapa y su Markdown acompañante, dejando copia de lo que hubiera antes.
///
/// # Por qué se validan las dos rutas antes de escribir ninguna
///
/// El orden era: validar el mapa, escribirlo, y solo entonces componer y validar el Markdown.
/// Si esa segunda validación fallaba, el agente recibía un error de seguridad y el mapa ya
/// estaba en el disco, sin que el mensaje lo dijera: un rechazo que llega después de haber
/// escrito no es un rechazo, es un aviso a toro pasado. Además disimulaba el escape por
/// enlace, porque el error del `.md` tapaba que el `.mmcelt` sí había salido.
///
/// # Parámetros
/// - `ruta`: la del mapa, ya validada.
/// - `proyecto`: el mapa a escribir.
///
/// # Devuelve
/// La ruta del Markdown, su contenido —que el llamador usa para la vista previa de la
/// respuesta— y las copias de seguridad creadas.
///
/// # Errores
/// De seguridad si la ruta del Markdown no supera la validación, o de ejecución si falla la
/// serialización o alguna de las dos escrituras.
fn escribir_el_mapa_y_su_markdown(
    ruta: &Path,
    proyecto: &Proyecto,
) -> Result<(PathBuf, String, Vec<String>), ErrorHerramienta> {
    let json = serde_json::to_string_pretty(proyecto)
        .map_err(|e| ErrorHerramienta::ejecucion(format!("No se pudo serializar: {e}")))?;

    let ruta_md = ruta.with_extension("").display().to_string() + SUFIJO_MARKDOWN_IA;
    let ruta_md = validar_ruta(&ruta_md, UsoDeLaRuta::EscrituraDeMarkdown)?;

    let mut respaldos = Vec::new();
    if let Some(r) = escribir_con_respaldo(ruta, &json, PoliticaDeCopia::DejarCopia)? {
        respaldos.push(r.display().to_string());
    }

    let markdown = exportar_markdown_para_ia(proyecto, idioma_del_usuario());
    if let Some(r) = escribir_con_respaldo(&ruta_md, &markdown, PoliticaDeCopia::DejarCopia)? {
        respaldos.push(r.display().to_string());
    }

    Ok((ruta_md, markdown, respaldos))
}

/// Carga un proyecto desde una ruta ya validada.
fn cargar(ruta: &Path) -> Result<Proyecto, ErrorHerramienta> {
    crate::storage::cargar_proyecto_de_archivo(ruta).map_err(|error| {
        let mensaje = error.mensaje_usuario().en(idioma_del_usuario());
        if matches!(error, crate::error::AppError::Formato { .. }) {
            ErrorHerramienta::formato_con_causa(mensaje, error)
        } else {
            ErrorHerramienta::ejecucion_con_causa(mensaje, error)
        }
    })
}

/// Implementa `mmcelt_read_mindmap`.
fn leer_mapa(argumentos: &ArgumentosLeerMapa) -> ResultadoHerramienta {
    let ruta = validar_ruta(&argumentos.file_path, UsoDeLaRuta::Lectura)?;

    if !ruta.exists() {
        return Err(ErrorHerramienta::ejecucion(format!(
            "Archivo no encontrado: {}",
            ruta.display()
        )));
    }

    // Un `.md` no es un mapa: se devuelve su texto tal cual.
    // Sin distinguir mayúsculas, como hace la validación de rutas. Comparando tal cual,
    // `NOTAS.MD` —el mismo archivo que `notas.md` en Windows y en macOS— se mandaba al
    // analizador de mapas y el agente recibía «el archivo no tiene el formato esperado».
    let extension = ruta
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase);
    if extension.as_deref() == Some("md") {
        let contenido = std::fs::read_to_string(&ruta)
            .map_err(|e| ErrorHerramienta::ejecucion(format!("No se pudo leer: {e}")))?;
        return Ok(json!({ "status": "success", "content": contenido }));
    }

    let proyecto = cargar(&ruta)?;

    // H5: cada conexión habla los dos idiomas del contrato. Los UUID se conservan para no
    // romper clientes existentes y los títulos se añaden para poder reutilizar directamente
    // la lectura en las herramientas de escritura.
    //
    // El título se busca con `get` y no por índice a propósito. `validar_estructura`
    // comprueba el árbol de nodos, no las conexiones, así que un archivo retocado a mano o
    // venido de otra versión puede traer una arista cuyo extremo ya no está. Indexar ahí
    // sería un pánico, y un pánico en el servidor MCP no devuelve error: se lleva la sesión
    // por delante. El título ausente sale como `null` y el UUID sale igual, que es lo que
    // permite diagnosticar el archivo en vez de quedarse sin respuesta.
    let titulo_de = |id: &uuid::Uuid| proyecto.nodes.get(id).map(|nodo| nodo.title.clone());
    let conexiones = proyecto
        .connections
        .iter()
        .map(|conexion| {
            json!({
                "id": conexion.id,
                "from": conexion.from,
                "to": conexion.to,
                "from_title": titulo_de(&conexion.from),
                "to_title": titulo_de(&conexion.to),
                "label": conexion.label,
                "relation_type": conexion.relation_type,
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "status": "success",
        "title": proyecto.title,
        "creator_vision": proyecto.creator_vision,
        "project_goals": proyecto.project_goals,
        "total_nodes": proyecto.nodes.len(),
        "nodes": proyecto.nodes,
        "connections": conexiones,
    }))
}

/// Implementa `mmcelt_get_human_feedback`.
fn obtener_correcciones(argumentos: &ArgumentosLeerMapa) -> ResultadoHerramienta {
    use crate::model::EstadoRevision;

    let ruta = validar_ruta(&argumentos.file_path, UsoDeLaRuta::Lectura)?;
    let proyecto = cargar(&ruta)?;

    let mut correcciones = Vec::new();
    let mut aprobados = Vec::new();

    // En orden de identificador: dos consultas seguidas sobre el mismo mapa deben devolver
    // lo mismo en el mismo orden, o el agente cree que algo se ha movido cuando no se ha
    // movido nada. Lo garantiza el tipo —`Proyecto::nodes` es un mapa ordenado—, y antes se
    // copiaban las claves a un vector para ordenarlas, cosa que ya no hacía nada.
    for nodo in proyecto.nodes.values() {
        // Lo que la propia IA escribió no se le devuelve como criterio de la persona.
        //
        // `exige_correccion_del_usuario()` da por corrección cualquier nodo descartado, sin
        // mirar quién lo descartó. Como el esquema le ofrece «Descartado» al agente, sus
        // propios descartes volvían aquí dentro de `corrections_required`, junto a la
        // instrucción «acata inmediatamente las corrections_required». El único indicio de
        // que no venían de nadie era el `review_status` embebido en el objeto.
        //
        // Este programa existe para que el criterio de la persona se distinga del de la
        // máquina. Un agente que se fabrica órdenes con apariencia humana —para la sesión
        // siguiente, o para otro agente que lea el mismo mapa— es justo lo contrario.
        if nodo.review_status == EstadoRevision::GeneradoPorIA {
            continue;
        }

        let descartado = nodo.status == EstadoNodo::Descartado;

        if nodo.exige_correccion_del_usuario() {
            correcciones.push(json!({
                "title": nodo.title,
                "review_status": nodo.review_status.nombre_canonico(),
                "feedback": nodo.correction_feedback,
                "discarded": descartado,
            }));
        } else if nodo.review_status == EstadoRevision::AprobadoPorHumano {
            aprobados.push(nodo.title.clone());
        }
    }

    Ok(json!({
        "status": "success",
        "project_title": proyecto.title,
        "total_corrections": correcciones.len(),
        "corrections_required": correcciones,
        "approved_nodes": aprobados,
        "instructions_for_ai": "Acata inmediatamente las corrections_required y no sigas ningún camino marcado como discarded.",
    }))
}

/// Lo que hubo que apañar durante una sincronización y hay que contarle al agente.
///
/// Va junto porque siempre viaja junto: las dos listas se llenan en sitios distintos del
/// recorrido y se leen las dos al final, al componer la respuesta.
#[derive(Default)]
struct AvisosDeSincronizacion {
    /// Los `parent_title` que el agente nombró y no existían en el mapa.
    padres_no_encontrados: Vec<String>,
    /// Valores de enumeración (status, priority, role) que no se reconocieron.
    valores_no_reconocidos: Vec<String>,
    /// Las conexiones cruzadas que se rechazaron, con el motivo de cada una.
    ///
    /// Se cuentan aparte de los valores no reconocidos porque no son lo mismo: un valor que
    /// no se entiende degrada un campo, y una conexión rechazada no llega a existir. El
    /// agente necesita distinguirlo para saber si el flujo que quería dibujar está dibujado.
    conexiones_no_creadas: Vec<String>,
}

/// Interpreta el `role` que declara el agente sobre un nodo.
///
/// Devuelve `None` cuando el agente no dice nada del rol —que no significa «ponlo por
/// omisión», sino «no hablo de eso»— y también cuando el texto no se reconoce, en cuyo caso
/// deja constancia en los avisos.
///
/// # Por qué no se usa `RolNodo::desde_texto`
///
/// Esa función cae en `Subtema` ante cualquier texto desconocido, que es lo correcto al abrir
/// un archivo antiguo y lo peor posible en la frontera con un agente: es el hallazgo H6, el de
/// los fallos que corrompen en silencio. Adivinar mal no daría error, el mapa quedaría mal y
/// nadie se enteraría. `status` y `priority` ya entraban por `reconocer_texto` por este mismo
/// motivo; el rol entra por la misma puerta.
///
/// # Parámetros
/// - `actualizacion`: el objeto JSON que mandó el agente.
/// - `titulo`: el título del nodo, para el aviso.
/// - `avisos`: donde se anota lo que no se reconoció.
fn rol_declarado(
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    avisos: &mut AvisosDeSincronizacion,
) -> Option<crate::model::RolNodo> {
    match &actualizacion.rol {
        ValorEnumerado::Ausente => None,
        ValorEnumerado::Reconocido(rol) => Some(*rol),
        ValorEnumerado::Desconocido(texto) => {
            avisos
                .valores_no_reconocidos
                .push(format!("role «{texto}» en «{titulo}»"));
            None
        }
    }
}

/// Añade a un nodo las etiquetas que declara el agente, **sin quitar ninguna**.
///
/// Es la diferencia entre enriquecer y pisar. Las etiquetas de un nodo humano las escribió una
/// persona y por ellas filtra en el programa; escribir las del agente encima sería borrar
/// trabajo humano sin que se note, que es la peor forma de borrarlo. Se aplica el mismo
/// criterio que ya rige para las notas: lo que el agente no nombra, no se toca.
///
/// La comparación ignora mayúsculas y minúsculas para que un agente que repite la llamada con
/// otra caja no acabe llenando el nodo de variantes de la misma etiqueta.
///
/// # Parámetros
/// - `nodo`: el nodo al que se añaden.
/// - `actualizacion`: el objeto JSON que mandó el agente.
///
/// # Devuelve
/// `true` si el nodo ha quedado con alguna etiqueta que antes no tenía.
fn anadir_las_etiquetas_declaradas(
    nodo: &mut crate::model::Nodo,
    actualizacion: &ActualizacionTipada,
) -> bool {
    let mut ha_cambiado = false;
    for entrante in &actualizacion.etiquetas {
        let entrante = entrante.trim();
        if entrante.is_empty() {
            continue;
        }
        let ya_esta = nodo
            .tags
            .iter()
            .any(|puesta| puesta.eq_ignore_ascii_case(entrante));
        if !ya_esta {
            nodo.tags.push(entrante.to_string());
            ha_cambiado = true;
        }
    }

    ha_cambiado
}

/// Aplica sobre un nodo que ya existe lo que el agente informa de él.
///
/// Solo escribe lo que el agente dice de verdad: un campo ausente, o unas notas vacías, no
/// significan «bórralo», sino «no digo nada de eso».
///
/// # Parámetros
/// - `proyecto`: el mapa sobre el que se escribe.
/// - `id`: el nodo a actualizar.
/// - `actualizacion`: el objeto JSON que mandó el agente.
/// - `titulo`: el título del nodo, para los avisos.
/// - `avisos`: donde se anota lo que no se reconoció.
fn actualizar_nodo_existente(
    proyecto: &mut Proyecto,
    id: uuid::Uuid,
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    avisos: &mut AvisosDeSincronizacion,
) {
    let Some(nodo) = proyecto.nodes.get_mut(&id) else {
        return;
    };

    let ha_cambiado = aplicar_actualizacion(
        nodo,
        actualizacion,
        titulo,
        avisos,
        DestinoActualizacion::Existente,
    );

    if ha_cambiado {
        caducar_la_aprobacion_humana(nodo);
    }
}

/// Distingue los valores por omisión de un nodo nuevo de la conservación de uno existente.
#[derive(Clone, Copy)]
enum DestinoActualizacion {
    Existente,
    Nuevo,
}

/// Aplica todos los campos modificables por el agente a través de una única puerta.
///
/// Un nodo existente conserva cualquier campo ausente; uno nuevo recibe los valores de
/// inicio documentados. Compartir esta función evita que creación y actualización interpreten
/// de manera distinta el mismo rol, estado, prioridad, notas, etiquetas o ruta.
fn aplicar_actualizacion(
    nodo: &mut crate::model::Nodo,
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    avisos: &mut AvisosDeSincronizacion,
    destino: DestinoActualizacion,
) -> bool {
    let mut ha_cambiado = false;

    match destino {
        DestinoActualizacion::Existente => {
            ha_cambiado |= aplicar_el_estado(nodo, actualizacion, titulo, avisos);
            ha_cambiado |= aplicar_la_prioridad(nodo, actualizacion, titulo, avisos);
            ha_cambiado |= aplicar_las_notas(nodo, actualizacion);
            ha_cambiado |= aplicar_la_ruta_del_archivo(nodo, actualizacion);
        }
        DestinoActualizacion::Nuevo => {
            nodo.status = match &actualizacion.estado {
                ValorEnumerado::Reconocido(estado) => *estado,
                ValorEnumerado::Desconocido(texto) => {
                    avisos
                        .valores_no_reconocidos
                        .push(format!("status «{texto}» en «{titulo}»"));
                    EstadoNodo::EnProgreso
                }
                ValorEnumerado::Ausente => EstadoNodo::EnProgreso,
            };
            nodo.priority = match &actualizacion.prioridad {
                ValorEnumerado::Reconocido(prioridad) => *prioridad,
                ValorEnumerado::Desconocido(texto) => {
                    avisos
                        .valores_no_reconocidos
                        .push(format!("priority «{texto}» en «{titulo}»"));
                    PrioridadNodo::default()
                }
                ValorEnumerado::Ausente => PrioridadNodo::default(),
            };
            nodo.notes = actualizacion.notas.clone().unwrap_or_default();
            nodo.file_path =
                texto_no_vacio(actualizacion.ruta_archivo.as_deref()).map(str::to_string);
            ha_cambiado = true;
        }
    }

    if let Some(rol) = rol_declarado(actualizacion, titulo, avisos) {
        ha_cambiado |= nodo.role != rol;
        nodo.role = rol;
    }
    ha_cambiado |= anadir_las_etiquetas_declaradas(nodo, actualizacion);
    ha_cambiado
}

/// Aplica el `status` que declara el agente, si lo declara y si se le permite.
///
/// # Lo que no puede hacer un agente
///
/// **Revivir un nodo descartado.** Descartar es la forma más rotunda que tiene la persona de
/// decir «no sigas por ahí», y el propio servidor se lo comunica al agente en
/// `mmcelt_get_human_feedback`, con la instrucción de no seguir ningún camino marcado así.
/// Pero luego aceptaba un `status` cualquiera y lo escribía encima.
///
/// El resultado era que el agente podía anular el veto con un campo, y **sin dejar rastro**:
/// el nodo dejaba de aparecer entre las correcciones, el documento para la IA dejaba de decir
/// que estaba descartado y la propia herramienta de consulta dejaba de devolverlo. El usuario
/// no tenía forma de enterarse.
///
/// Volver a descartarlo sí se admite: eso no contradice a nadie.
///
/// # Parámetros
/// - `nodo`: el nodo a modificar.
/// - `actualizacion`: lo que ha enviado el agente.
/// - `titulo`: el del nodo, solo para poder nombrarlo en los avisos.
/// - `avisos`: donde se anotan los valores que no se reconocen, para devolvérselos al agente.
///
/// # Devuelve
/// `true` si el estado ha cambiado de verdad.
fn aplicar_el_estado(
    nodo: &mut crate::model::Nodo,
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    avisos: &mut AvisosDeSincronizacion,
) -> bool {
    let nuevo = match &actualizacion.estado {
        ValorEnumerado::Ausente => return false,
        ValorEnumerado::Reconocido(nuevo) => *nuevo,
        ValorEnumerado::Desconocido(texto) => {
            avisos
                .valores_no_reconocidos
                .push(format!("status «{texto}» en «{titulo}»"));
            return false;
        }
    };

    let revive_lo_descartado =
        nodo.status == EstadoNodo::Descartado && nuevo != EstadoNodo::Descartado;
    if revive_lo_descartado {
        return false;
    }

    let cambia = nodo.status != nuevo;
    nodo.status = nuevo;
    cambia
}

/// Aplica la `priority` que declara el agente, si la declara y se reconoce.
///
/// # Parámetros
/// - `nodo`: el nodo a modificar.
/// - `actualizacion`: lo que ha enviado el agente.
/// - `titulo`: el del nodo, para poder nombrarlo en los avisos.
/// - `avisos`: donde se anotan los valores no reconocidos.
///
/// # Devuelve
/// `true` si la prioridad ha cambiado de verdad.
fn aplicar_la_prioridad(
    nodo: &mut crate::model::Nodo,
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    avisos: &mut AvisosDeSincronizacion,
) -> bool {
    let nueva = match &actualizacion.prioridad {
        ValorEnumerado::Ausente => return false,
        ValorEnumerado::Reconocido(nueva) => *nueva,
        ValorEnumerado::Desconocido(texto) => {
            avisos
                .valores_no_reconocidos
                .push(format!("priority «{texto}» en «{titulo}»"));
            return false;
        }
    };

    let cambia = nodo.priority != nueva;
    nodo.priority = nueva;
    cambia
}

/// Aplica las `notes` que envía el agente, si envía algo.
///
/// # Por qué una nota vacía no borra
///
/// Una nota vacía significa «no digo nada de las notas», no «bórralas». Antes se escribía tal
/// cual, así que `{"title": "X", "notes": ""}` —lo que manda un agente que solo quiere cambiar
/// el estado— vaciaba lo que la persona hubiera escrito en ese nodo, y encima sin dejar copia.
///
/// # Parámetros
/// - `nodo`: el nodo a modificar.
/// - `actualizacion`: lo que ha enviado el agente.
///
/// # Devuelve
/// `true` si las notas han cambiado de verdad.
fn aplicar_las_notas(nodo: &mut crate::model::Nodo, actualizacion: &ActualizacionTipada) -> bool {
    let Some(v) = actualizacion.notas.as_deref() else {
        return false;
    };
    if v.trim().is_empty() {
        return false;
    }

    let cambia = nodo.notes != v;
    nodo.notes = v.to_string();
    cambia
}

/// Aplica el `file_path` que envía el agente, si envía algo.
///
/// Un agente que descubre a qué archivo corresponde un nodo ya existente tiene que poder
/// anotarlo; antes se ignoraba. Como con las notas, una cadena vacía no borra lo que hubiera.
///
/// # Parámetros
/// - `nodo`: el nodo a modificar.
/// - `actualizacion`: lo que ha enviado el agente.
///
/// # Devuelve
/// `true` si la ruta ha cambiado de verdad.
fn aplicar_la_ruta_del_archivo(
    nodo: &mut crate::model::Nodo,
    actualizacion: &ActualizacionTipada,
) -> bool {
    let Some(v) = actualizacion.ruta_archivo.as_deref() else {
        return false;
    };
    let v = v.trim();
    if v.is_empty() {
        return false;
    }

    let cambia = nodo.file_path.as_deref() != Some(v);
    nodo.file_path = Some(v.to_string());
    cambia
}

/// Retira la aprobación humana de un nodo que la IA acaba de modificar.
///
/// Sin esto, un agente podía reescribir las notas de un nodo que el usuario había marcado como
/// «Aprobado por humano» y el nodo seguía luciendo esa aprobación. El usuario veía su propio
/// visto bueno sobre un texto que no había leído nunca, que es justo lo contrario de lo que
/// este programa promete.
///
/// Los demás estados de revisión no se tocan: solo la aprobación afirma algo que el cambio
/// invalida. Y `correction_feedback` se conserva siempre, para que no se pierda lo que el
/// usuario exigió.
///
/// # Parámetros
/// - `nodo`: el nodo que acaba de cambiar.
fn caducar_la_aprobacion_humana(nodo: &mut crate::model::Nodo) {
    if nodo.review_status == crate::model::EstadoRevision::AprobadoPorHumano {
        nodo.review_status = crate::model::EstadoRevision::PendienteRevision;
    }
}

/// Crea un nodo nuevo a partir de lo que informa el agente y lo cuelga donde corresponda.
///
/// # Dónde cuelga
///
/// El padre se resuelve con [`resolver_nodo_senalado`], igual que el nodo de una
/// actualización: vale `parent_id`, vale `parent_title`, y si vienen los dos tienen que
/// corresponder. De ahí salen tres desenlaces que conviene no confundir, porque son
/// deliberadamente distintos:
///
/// - **El padre se identifica**: el nodo cuelga de él.
/// - **El `parent_title` no está en el mapa**: el nodo cuelga de la raíz —es lo único que se
///   puede hacer— y se anota para decírselo en la respuesta, en lugar de hacerlo en
///   silencio. Antes ocurría callando, y el agente daba por hecha una jerarquía que el mapa
///   no tenía.
/// - **El `parent_title` lo llevan varios nodos, o el `parent_id` está mal**: se propaga el
///   error y no se escribe nada. No saber dónde va un nodo no es lo mismo que tener dos
///   sitios y elegir uno a escondidas.
///
/// # Parámetros
/// - `proyecto`: el mapa sobre el que se escribe.
/// - `indice`: los títulos ya conocidos, que se mantiene al día con el nodo nuevo.
/// - `actualizacion`: el objeto JSON que mandó el agente.
/// - `titulo`: el título del nodo nuevo.
/// - `clave`: ese mismo título en minúsculas, que es como se indexa.
/// - `avisos`: donde se anota lo que no se reconoció.
///
/// # Errores
/// [`ErrorHerramienta`] si el mapa ya está en el máximo de nodos, o si el padre se señaló mal.
fn crear_nodo_desde_la_actualizacion(
    proyecto: &mut Proyecto,
    indice: &mut IndiceDeTitulos,
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    clave: String,
    avisos: &mut AvisosDeSincronizacion,
) -> Result<(), ErrorHerramienta> {
    if proyecto.nodes.len() >= crate::model::NODOS_MAXIMOS {
        return Err(ErrorHerramienta::ejecucion(format!(
            "El mapa ha alcanzado el máximo de {} nodos.",
            crate::model::NODOS_MAXIMOS
        )));
    }
    let padre = resolver_padre_del_nodo_nuevo(proyecto, indice, actualizacion, avisos)?;
    let id_nuevo = proyecto.anadir_hijo(padre, titulo);
    indice.entry(clave).or_default().push(id_nuevo);
    if let Some(nodo) = proyecto.nodes.get_mut(&id_nuevo) {
        inicializar_nodo_del_agente(nodo, actualizacion, titulo, avisos);
    }
    Ok(())
}

/// Resuelve el padre solicitado y registra el uso de la raíz cuando aquel no existe.
fn resolver_padre_del_nodo_nuevo(
    proyecto: &Proyecto,
    indice: &IndiceDeTitulos,
    actualizacion: &ActualizacionTipada,
    avisos: &mut AvisosDeSincronizacion,
) -> Result<uuid::Uuid, ErrorHerramienta> {
    let padre_pedido = texto_no_vacio(actualizacion.parent_title.as_deref());
    let padre_resuelto =
        resolver_nodo_senalado(proyecto, indice, actualizacion.padre(), "parent_id")?;
    let Some(padre) = padre_resuelto else {
        if let Some(titulo_padre) = padre_pedido {
            avisos.padres_no_encontrados.push(titulo_padre.to_string());
        }
        return Ok(proyecto.root_id);
    };
    Ok(padre)
}

/// Marca la procedencia del nodo y aplica por la misma puerta todos los campos recibidos.
fn inicializar_nodo_del_agente(
    nodo: &mut crate::model::Nodo,
    actualizacion: &ActualizacionTipada,
    titulo: &str,
    avisos: &mut AvisosDeSincronizacion,
) {
    nodo.tags = vec!["ia_progress".to_string()];
    nodo.review_status = crate::model::EstadoRevision::GeneradoPorIA;
    aplicar_actualizacion(
        nodo,
        actualizacion,
        titulo,
        avisos,
        DestinoActualizacion::Nuevo,
    );
}

/// Crea las conexiones cruzadas que declara el agente, validando los dos extremos.
///
/// # Por qué existe
///
/// Es la otra mitad del hallazgo H3, el que bloqueó la prueba real. Un flujo con un bucle no
/// se puede expresar con una jerarquía: hace falta una arista que vuelva atrás. La única
/// herramienta autorizada a escribir sobre un mapa con trabajo humano no sabía crearlas, así
/// que el agente tenía que elegir entre entregar otra cosa o editar el archivo por su cuenta.
/// Una valla sin puerta empuja fuera de la API justo a quien la respeta.
///
/// El contrato es el mismo de `mmcelt_create_mindmap` —`from_title`, `to_title`, `label` y
/// `relation_type`— para que el agente no tenga que aprenderse dos formatos según la puerta
/// por la que entre. Aquí, además, cada extremo admite su UUID: ver más abajo.
///
/// # Qué se rechaza, y por qué se dice
///
/// - **Un extremo que no existe.** Una conexión que apunta a un identificador ausente deja el
///   archivo corrupto. El conversor de la importación las descarta en silencio; aquí se
///   informa, porque un `success` sobre un flujo a medio dibujar es peor que un aviso.
/// - **Un nodo consigo mismo.** El modelo ya lo impide; el servidor tiene que enterarse para
///   no contarla como creada.
/// - **Un extremo descartado.** Descartar es la forma más rotunda que tiene la persona de
///   decir «no sigas por ahí». El `status` ya está blindado contra revivir un descarte;
///   enlazar el flujo con él lo devolvería al recorrido por la puerta de al lado, sin
///   cambiarle el estado y sin que nadie lo aprobara.
/// - **Una conexión que ya está.** Los agentes reintentan. Sin esto, cada reintento añadiría
///   otra arista idéntica y el usuario tendría que borrarlas a mano una por una.
///
/// Un `relation_type` que no se reconoce **no** cancela la conexión: se avisa y se usa el
/// valor por omisión, igual que ya se hace con el estado de un nodo nuevo. Perder la arista
/// entera por una palabra mal escrita costaría más que dibujarla con la relación genérica.
///
/// # Cómo se señala cada extremo
///
/// Con [`resolver_nodo_senalado`], la misma función que usa `node_updates`: `from_id` o
/// `from_title`, `to_id` o `to_title`, o los dos, que entonces tienen que concordar. Que sea
/// la misma función es lo que evita que la herramienta tenga dos reglas de identidad según
/// por dónde se entre.
///
/// La distinción entre lo que **rechaza la llamada** y lo que solo **se avisa** es el eje de
/// este bloque, y no es la misma que en el resto:
///
/// - **Rechazan la llamada entera**, propagando el error hacia arriba antes de escribir
///   nada, los problemas de identidad: un UUID que no es un UUID, uno que no está en el
///   mapa, uno que no case con el título que lo acompaña, y un título que llevan varios
///   nodos. En los cuatro casos el agente ha señalado mal, y dibujar la arista contra un
///   candidato elegido a ciegas sería peor que no dibujarla: una arista cambia lo que el
///   diagrama afirma, y el `success` haría que nadie fuera a mirarla. Como el mapa se
///   escribe al final de la sincronización y no aquí, propagar el error deja el archivo
///   intacto, incluidos los nodos que venían en la misma llamada.
/// - **Se avisan y se sigue** los problemas de H3, que son de contenido y no de identidad: un
///   título que sencillamente no está en el mapa, la autoconexión, el extremo descartado y la
///   relación duplicada. Ahí no hay nada que adivinar ni nada corrupto que evitar, solo una
///   arista de la lista que no procede, y perder por ella las demás sería peor servicio.
///
/// # Parámetros
/// - `proyecto`: el mapa sobre el que se escribe.
/// - `indice`: títulos en minúsculas y sin espacios sobrantes, ya con los nodos recién creados.
/// - `conexiones`: el array `cross_connections` tal como lo mandó el agente.
/// - `avisos`: donde se anota lo rechazado y lo que no se reconoció.
///
/// # Devuelve
/// Cuántas conexiones se han añadido de verdad al mapa.
///
/// # Errores
/// [`ErrorHerramienta::argumento`] si algún extremo señala mal a su nodo, con el detalle que
/// componga [`resolver_nodo_senalado`]. Quien llama debe abandonar la sincronización entera.
fn anadir_las_conexiones_cruzadas(
    proyecto: &mut Proyecto,
    indice: &IndiceDeTitulos,
    conexiones: &[ConexionTipada],
    avisos: &mut AvisosDeSincronizacion,
) -> Result<usize, ErrorHerramienta> {
    let mut creadas = 0;

    for conexion in conexiones {
        let id_origen = resolver_nodo_senalado(proyecto, indice, conexion.origen(), "from_id")?;
        let id_destino = resolver_nodo_senalado(proyecto, indice, conexion.destino(), "to_id")?;

        let nombre_extremo = |id: Option<uuid::Uuid>, titulo: Option<&str>| {
            texto_no_vacio(titulo)
                .map(str::to_string)
                .or_else(|| id.and_then(|id| proyecto.nodes.get(&id).map(|n| n.title.clone())))
                .unwrap_or_else(|| "(sin indicar)".to_string())
        };
        let origen = nombre_extremo(id_origen, conexion.from_title.as_deref());
        let destino = nombre_extremo(id_destino, conexion.to_title.as_deref());

        let (Some(id_origen), Some(id_destino)) = (id_origen, id_destino) else {
            avisos.conexiones_no_creadas.push(
                format!(
                    "«{origen}» → «{destino}»: falta un extremo o no existe en el mapa; indica su título o su UUID."
                ),
            );
            continue;
        };

        if id_origen == id_destino {
            avisos.conexiones_no_creadas.push(format!(
                "«{origen}» → «{destino}»: un nodo no se conecta consigo mismo."
            ));
            continue;
        }

        let descartado = |id: &uuid::Uuid| {
            proyecto
                .nodes
                .get(id)
                .is_some_and(|nodo| nodo.status == EstadoNodo::Descartado)
        };
        if descartado(&id_origen) || descartado(&id_destino) {
            avisos.conexiones_no_creadas.push(format!(
                "«{origen}» → «{destino}»: uno de los extremos está descartado, y el agente no \
                 reabre un camino que descartó el usuario."
            ));
            continue;
        }

        let relacion = match &conexion.relacion {
            ValorEnumerado::Reconocido(relacion) => *relacion,
            ValorEnumerado::Desconocido(texto) => {
                avisos.valores_no_reconocidos.push(format!(
                    "relation_type «{texto}» en «{origen}» → «{destino}»"
                ));
                TipoRelacion::default()
            }
            ValorEnumerado::Ausente => TipoRelacion::default(),
        };

        let ya_esta = proyecto.connections.iter().any(|puesta| {
            puesta.from == id_origen && puesta.to == id_destino && puesta.relation_type == relacion
        });
        if ya_esta {
            avisos.conexiones_no_creadas.push(format!(
                "«{origen}» → «{destino}»: esa relación ya estaba en el mapa, no se duplica."
            ));
            continue;
        }

        let etiqueta = conexion.etiqueta.as_deref().unwrap_or("").trim();

        if proyecto
            .anadir_conexion_cruzada(id_origen, id_destino, etiqueta, relacion)
            .is_some()
        {
            creadas += 1;
        } else {
            avisos.conexiones_no_creadas.push(format!(
                "«{origen}» → «{destino}»: el mapa la ha rechazado."
            ));
        }
    }

    Ok(creadas)
}

/// Valida el mapa sincronizado y lo escribe, junto a su documento para la IA.
///
/// # Parámetros
/// - `ruta`: el archivo `.mmcelt` de destino.
/// - `proyecto`: el mapa ya actualizado.
///
/// # Errores
/// [`ErrorHerramienta`] si el mapa no supera la validación estructural o la de longitudes,
/// si no se puede serializar o si no se puede escribir.
fn guardar_el_mapa_sincronizado(ruta: &Path, proyecto: &Proyecto) -> Result<(), ErrorHerramienta> {
    // Escribir es una frontera, igual que leer: el mapa que sale de aquí tiene que poder
    // abrirse después. Una sucesión de sincronizaciones puede añadir nodos hasta pasarse
    // de profundidad, y sin esta comprobación el archivo quedaría en disco dañado y la
    // aplicación no podría volver a abrirlo.
    let idioma = idioma_del_usuario();
    proyecto
        .validar_estructura()
        .map_err(|e| ErrorHerramienta::ejecucion(e.mensaje_usuario().en(idioma)))?;

    // Y el tope de longitud, que **solo** se aplica aquí y no al abrir un archivo del
    // usuario: lo que se vigila es que un agente en bucle no llene el disco con textos
    // enormes. Ver `Proyecto::validar_longitudes_de_texto`.
    proyecto
        .validar_longitudes_de_texto()
        .map_err(|e| ErrorHerramienta::ejecucion(e.mensaje_usuario().en(idioma)))?;

    let json = serde_json::to_string_pretty(&proyecto)
        .map_err(|e| ErrorHerramienta::ejecucion(format!("No se pudo serializar: {e}")))?;

    // La excepción de copia vale **solo para los borradores de la propia IA**.
    //
    // Se justificaba diciendo que esta herramienta «no borra nodos, así que lo que puede
    // estropear está acotado». No borra nodos, pero cambia el estado, la prioridad y las
    // notas de los que hay, y sobre un mapa escrito a mano eso es destruir el trabajo de la
    // persona sin dejar ni un `.bak`. La valla de al lado impide rehacer ese mapa con
    // `mmcelt_create_mindmap`; esta herramienta llegaba al mismo sitio por la puerta
    // contigua, que es el patrón que más veces ha fallado en este servidor.
    //
    // El motivo original de la excepción sigue en pie donde tiene sentido: un mapa que solo
    // ha tocado la IA se sincroniza una y otra vez conforme avanza el trabajo, y una copia
    // por informe llenaría la carpeta de archivos que nadie ha pedido.
    let politica = if el_mapa_es_solo_de_la_ia(proyecto) {
        PoliticaDeCopia::SinCopiaPorSincronizacion
    } else {
        PoliticaDeCopia::DejarCopia
    };

    escribir_con_respaldo(ruta, &json, politica)?;

    let ruta_md = ruta.with_extension("").display().to_string() + SUFIJO_MARKDOWN_IA;
    let ruta_md = validar_ruta(&ruta_md, UsoDeLaRuta::EscrituraDeMarkdown)?;
    // El documento acompañante sigue la misma política: si la persona tenía ahí un
    // `nombre_AI.md` escrito a mano, tampoco puede desaparecer sin copia.
    escribir_con_respaldo(
        &ruta_md,
        &exportar_markdown_para_ia(proyecto, idioma_del_usuario()),
        politica,
    )?;

    Ok(())
}

/// Compone el mensaje que se le devuelve al agente al terminar.
///
/// # Parámetros
/// - `actualizaciones_recibidas`: cuántas actualizaciones traía la llamada.
/// - `conexiones_creadas`: cuántas conexiones cruzadas se han añadido de verdad.
/// - `avisos`: lo que hubo que apañar, y que el agente necesita saber para no repetirlo.
///
/// # Devuelve
/// El texto para el campo `message` de la respuesta.
fn mensaje_de_sincronizacion(
    actualizaciones_recibidas: usize,
    conexiones_creadas: usize,
    avisos: &AvisosDeSincronizacion,
) -> String {
    let mut mensaje = format!(
        "Mapa mental sincronizado con {} actualizaciones. El usuario puede verlo en MMCelt.",
        actualizaciones_recibidas
    );
    if conexiones_creadas > 0 {
        mensaje.push_str(&format!(
            " Se han añadido {conexiones_creadas} conexiones cruzadas."
        ));
    }
    // Se cuenta lo rechazado aunque el agente pueda leerlo en `connections_not_created`: el
    // motivo del hallazgo H3 es que un flujo a medio dibujar no puede devolver «success» a
    // secas, y el `message` es lo único que muchos clientes enseñan.
    if !avisos.conexiones_no_creadas.is_empty() {
        mensaje.push_str(&format!(
            " Atención: estas conexiones cruzadas no se crearon: {}",
            avisos.conexiones_no_creadas.join(" ")
        ));
    }
    if !avisos.padres_no_encontrados.is_empty() {
        mensaje.push_str(&format!(
            " Atención: estos «parent_title» no existían en el mapa, así que sus nodos han \
         quedado colgando de la raíz: {}.",
            avisos.padres_no_encontrados.join(", ")
        ));
    }
    if !avisos.valores_no_reconocidos.is_empty() {
        mensaje.push_str(&format!(
        " Atención: estos valores de enumeración no se reconocieron y se ignoraron o degradaron por omisión: {}.",
        avisos.valores_no_reconocidos.join(", ")
    ));
    }

    mensaje
}

/// Implementa `mmcelt_sync_ai_progress`.
fn sincronizar_progreso(argumentos: &SincronizacionTipada) -> ResultadoHerramienta {
    let ruta = validar_ruta(&argumentos.file_path, UsoDeLaRuta::EscrituraDeMapa)?;

    let actualizaciones = &argumentos.node_updates;

    let mut proyecto = cargar(&ruta)?;

    // Se construye una vez y se mantiene al día conforme se crean nodos: dentro del bucle,
    // cada búsqueda es una consulta directa en lugar de un recorrido del mapa entero.
    let mut indice = indexar_por_titulo(&proyecto);

    let mut avisos = AvisosDeSincronizacion::default();

    for actualizacion in actualizaciones {
        let nodo_resuelto =
            resolver_nodo_senalado(&proyecto, &indice, actualizacion.nodo.prestada(), "id")?;

        match nodo_resuelto {
            Some(id) => {
                let titulo = proyecto.nodes[&id].title.clone();
                actualizar_nodo_existente(&mut proyecto, id, actualizacion, &titulo, &mut avisos)
            }
            None => {
                let Some(titulo) = texto_no_vacio(actualizacion.nodo.title.as_deref()) else {
                    continue;
                };
                crear_nodo_desde_la_actualizacion(
                    &mut proyecto,
                    &mut indice,
                    actualizacion,
                    titulo,
                    titulo.to_lowercase(),
                    &mut avisos,
                )?
            }
        }
    }

    // Las conexiones van **después** del recorrido de nodos, y no es un detalle de orden: es
    // lo que permite que un extremo sea un nodo creado en esta misma llamada, que es el caso
    // normal cuando el agente añade un paso al flujo y lo enlaza con lo que ya había.
    let conexiones_creadas = anadir_las_conexiones_cruzadas(
        &mut proyecto,
        &indice,
        &argumentos.cross_connections,
        &mut avisos,
    )?;

    proyecto.updated_at = chrono::Utc::now();
    crate::layout::aplicar_disposicion_automatica(&mut proyecto);

    let devolucion = preparar_devolucion(
        &ruta,
        argumentos.agent_name.as_deref(),
        OperacionDevolucion::Modificado,
    )?;

    guardar_el_mapa_sincronizado(&ruta, &proyecto)?;

    if let Some((contexto, recibo)) = &devolucion {
        publicar_devolucion(contexto, recibo)?;
    }

    Ok(json!({
        "status": "success",
        "delivery_receipt": devolucion.as_ref().map(|(_, recibo)| recibo),
        "message": mensaje_de_sincronizacion(actualizaciones.len(), conexiones_creadas, &avisos),
        "total_nodes": proyecto.nodes.len(),
        "connections_added": conexiones_creadas,
        "connections_not_created": avisos.conexiones_no_creadas,
        "parents_not_found": avisos.padres_no_encontrados,
        "unrecognized_values": avisos.valores_no_reconocidos,
    }))
}

/// Impide que la exportación pise un documento que no haya escrito ella misma.
///
/// El destino de la exportación es un `.md`, y en la carpeta de un proyecto los `.md` que hay
/// son del usuario: el `README.md`, las notas, la documentación. Sin esta comprobación, un
/// agente podía apuntar la exportación a cualquiera de ellos y sustituirlo entero por el
/// documento generado, con respuesta `success`. Comprobado sobre un `README.md`.
///
/// Lo que distingue a un documento propio es la cabecera que le pone el exportador, que va en
/// las primeras líneas y ninguna otra cosa escribe. Rehacer un documento ya generado es
/// justamente lo que se espera de esta herramienta, así que ese caso sigue permitido.
///
/// # Parámetros
/// - `destino`: la ruta del `.md` que se va a escribir, ya validada.
///
/// # Errores
/// Error de seguridad si el archivo existe y no lleva la cabecera del exportador, incluido el
/// caso de que no se pueda leer: ante la duda no se pisa.
fn rechazar_si_el_markdown_no_lo_escribio_mmcelt(destino: &Path) -> Result<(), ErrorHerramienta> {
    if !destino.exists() {
        return Ok(());
    }

    /// Con cuánto del principio del archivo basta para reconocer la cabecera.
    const CABECERA: usize = 4096;

    let escrito_por_mmcelt = std::fs::read_to_string(destino)
        .map(|contenido| {
            let principio: String = contenido.chars().take(CABECERA).collect();
            Idioma::TODOS.iter().any(|idioma| {
                let firma = Texto::MdDocumentoDeMapaMental.en(*idioma).trim();
                principio.contains(firma)
            })
        })
        .unwrap_or(false);

    if escrito_por_mmcelt {
        return Ok(());
    }

    Err(ErrorHerramienta::seguridad(format!(
        "«{}» ya existe y no lo escribió MMCelt. No se sobrescribe: elige otro nombre para \
         el documento exportado.",
        destino
            .file_name()
            .map(|nombre| nombre.to_string_lossy().into_owned())
            .unwrap_or_else(|| "el destino".to_string())
    )))
}

/// Implementa `mmcelt_export_ai_markdown`.
fn exportar_markdown(argumentos: &ArgumentosExportar) -> ResultadoHerramienta {
    let origen = validar_ruta(&argumentos.mmcelt_file_path, UsoDeLaRuta::Lectura)?;
    let destino = validar_ruta(&argumentos.output_md_path, UsoDeLaRuta::EscrituraDeMarkdown)?;
    rechazar_si_el_markdown_no_lo_escribio_mmcelt(&destino)?;

    let proyecto = cargar(&origen)?;
    let respaldo = escribir_con_respaldo(
        &destino,
        &exportar_markdown_para_ia(&proyecto, idioma_del_usuario()),
        PoliticaDeCopia::DejarCopia,
    )?;

    Ok(json!({
        "status": "success",
        "message": format!("Markdown exportado a {}", destino.display()),
        "backup_created": respaldo.map(|r| r.display().to_string()),
    }))
}
