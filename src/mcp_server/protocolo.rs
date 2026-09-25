//! # Sobre JSON-RPC del servidor MCP (`mcp_server/protocolo.rs`)
//!
//! Aquí está la **frontera** entre el texto que llega por la entrada estándar y el resto del
//! servidor. Todo lo que cruza esta frontera hacia dentro ya es un [`PeticionJsonRpc`]: un
//! método en texto, un identificador que cumple el protocolo y unos parámetros con forma de
//! objeto. Lo que no encaja no pasa; se contesta con un error y no llega al despacho.
//!
//! ## Por qué existe este módulo
//!
//! El servidor interpretaba la línea recibida como un [`Value`] suelto y sacaba de él lo que
//! necesitaba con `get("...").unwrap_or(...)`. Con eso, un sobre que no cumplía JSON-RPC 2.0
//! —sin `jsonrpc`, con la versión equivocada, con `method` numérico o con un `id` que era un
//! objeto— **llegaba igualmente al despacho de herramientas**: el servidor ejecutaba trabajo
//! real —leer o escribir archivos del usuario— a partir de una petición que debía haber
//! rechazado, y contestaba con un `result` que le decía al cliente que su sobre era correcto.
//!
//! Cada `unwrap_or` era además una decisión escondida: `method` ausente se convertía en la
//! cadena vacía, y la cadena vacía llegaba al `match` de métodos como si fuera un método más.
//! Al validar primero y despachar después, esas decisiones dejan de estar repartidas por el
//! código y se toman todas en un sitio, con un nombre y un motivo.
//!
//! ## Qué se comprueba, y en qué orden
//!
//! 1. Que el texto sea JSON. Si no, error `-32700`.
//! 2. Que sea un **objeto**. Un array o un número son JSON válido pero no son una petición:
//!    error `-32600`.
//! 3. Que el `id`, si viene, sea texto, número o nulo. Un objeto o un array como identificador
//!    no cumplen el protocolo y, además, no se pueden devolver como referencia útil.
//! 4. Que `jsonrpc` valga exactamente `"2.0"`.
//! 5. Que `method` sea una cadena.
//!
//! El identificador se valida **antes** que el resto justo para poder devolverlo: una respuesta
//! de error sin el `id` de la petición obliga al cliente a adivinar a cuál de sus llamadas
//! contesta.
//!
//! ## Una decisión que conviene conocer
//!
//! Un sobre inválido **se contesta aunque no traiga `id`**. La regla de JSON-RPC de no
//! responder a las notificaciones se aplica a las notificaciones *válidas*; de una petición
//! rota no se puede saber si quien la envió esperaba respuesta, y callar dejaría al cliente
//! esperando o, peor, convencido de que su llamada se atendió. Una notificación bien formada
//! sí sigue sin respuesta, que es lo que manda el protocolo.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::error::Category;
use serde_json::{json, Map, Number, Value};
use std::io::BufRead;

use crate::error::{AppError, AppResult};

/// Máximo que puede ocupar una petición MCP antes de analizar su JSON: 16 MiB.
pub(crate) const BYTES_MAXIMOS_MENSAJE_MCP: u64 = 16 * 1024 * 1024;

/// Única versión de JSON-RPC que este servidor habla, y la única que acepta.
const VERSION_JSON_RPC: &str = "2.0";

/// Código que JSON-RPC 2.0 reserva para «el texto recibido no es JSON».
const CODIGO_JSON_MAL_FORMADO: i32 = -32700;

/// Código que JSON-RPC 2.0 reserva para «esto es JSON, pero no es una petición válida».
const CODIGO_PETICION_INVALIDA: i32 = -32600;

/// Código que JSON-RPC 2.0 reserva para «el método solicitado no existe».
const CODIGO_METODO_NO_ENCONTRADO: i32 = -32601;

/// Código que JSON-RPC 2.0 reserva para parámetros que no cumplen el contrato del método.
const CODIGO_PARAMETROS_INVALIDOS: i32 = -32602;

/// Código que JSON-RPC 2.0 reserva para «el fallo es del servidor, no de la petición».
const CODIGO_ERROR_INTERNO: i32 = -32603;

// ============================================================================
// Tipos del sobre
// ============================================================================

/// Identificador de una petición JSON-RPC 2.0.
///
/// El protocolo admite exactamente tres formas —texto, número o nulo— y este enumerado no
/// deja sitio para ninguna más. Antes el identificador viajaba como [`Value`], así que un
/// objeto o un array entraban sin más y volvían tal cual en la respuesta: el cliente recibía
/// como referencia algo que él mismo no podía usar para casar la llamada.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IdJsonRpc {
    /// Identificador de texto, como `"a-1"`.
    Texto(String),
    /// Identificador numérico, entero o con decimales.
    Numero(Number),
    /// Identificador nulo. El protocolo lo admite, y es el que se devuelve cuando la petición
    /// venía tan rota que no se pudo averiguar cuál era.
    Nulo,
}

impl IdJsonRpc {
    /// Convierte el valor recibido en un identificador del protocolo.
    ///
    /// # Devuelve
    /// El identificador, o `None` si el valor no es ninguna de las tres formas admitidas.
    fn desde_valor(valor: Value) -> Option<Self> {
        match valor {
            Value::String(texto) => Some(Self::Texto(texto)),
            Value::Number(numero) => Some(Self::Numero(numero)),
            Value::Null => Some(Self::Nulo),
            Value::Bool(_) | Value::Array(_) | Value::Object(_) => None,
        }
    }

    /// Devuelve el identificador tal como debe viajar en la respuesta.
    fn a_valor(&self) -> Value {
        match self {
            Self::Texto(texto) => Value::String(texto.clone()),
            Self::Numero(numero) => Value::Number(numero.clone()),
            Self::Nulo => Value::Null,
        }
    }
}

/// Los parámetros de una petición MCP, que el protocolo define siempre como un objeto.
///
/// Se distingue el objeto de su ausencia porque son cosas distintas para quien llama, aunque
/// el servidor acabe tratándolas igual en casi todas partes. Un valor que no sea objeto
/// —un número, un array— se recoge como [`ParametrosMcp::Ausentes`]: MCP nombra sus argumentos,
/// así que de un array no se puede sacar ningún campo, y es exactamente el mismo resultado que
/// daba el código anterior al buscar un campo dentro de algo que no era un objeto.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParametrosMcp {
    /// No venían parámetros, o lo que venía no era un objeto.
    Ausentes,
    /// Parámetros con nombre, que es la única forma que usa MCP.
    PorNombre(Map<String, Value>),
}

impl ParametrosMcp {
    /// Recoge el campo `params` de la petición.
    fn desde_valor(valor: Option<Value>) -> Self {
        match valor {
            Some(Value::Object(campos)) => Self::PorNombre(campos),
            _ => Self::Ausentes,
        }
    }

    /// Devuelve el valor de un parámetro, si viene.
    pub(super) fn valor(&self, campo: &str) -> Option<&Value> {
        match self {
            Self::PorNombre(campos) => campos.get(campo),
            Self::Ausentes => None,
        }
    }

    /// Devuelve un parámetro de texto, si viene y es texto.
    pub(super) fn texto(&self, campo: &str) -> Option<&str> {
        self.valor(campo).and_then(Value::as_str)
    }
}

/// Una petición JSON-RPC 2.0 que **ya ha superado la validación del sobre**.
///
/// Tener este tipo en la mano significa que la versión del protocolo es la correcta, que el
/// método es una cadena y que el identificador —si lo hay— cumple el protocolo. El despacho
/// no tiene que volver a comprobar nada de eso ni decidir qué hacer si falta.
pub(crate) struct PeticionJsonRpc {
    /// Identificador de la llamada, o `None` si es una notificación y no espera respuesta.
    id: Option<IdJsonRpc>,
    /// Nombre del método solicitado.
    metodo: String,
    /// Parámetros con nombre de la llamada.
    parametros: ParametrosMcp,
}

impl PeticionJsonRpc {
    /// El método solicitado.
    pub(super) fn metodo(&self) -> &str {
        &self.metodo
    }

    /// Los parámetros de la llamada.
    pub(super) fn parametros(&self) -> &ParametrosMcp {
        &self.parametros
    }
}

// ============================================================================
// Tipos de la respuesta
// ============================================================================

/// Un error del protocolo, con el código que JSON-RPC 2.0 asigna a cada situación.
#[derive(Debug, Serialize)]
pub(crate) struct ErrorJsonRpc {
    /// Código numérico del error, tal como lo nombra el protocolo.
    #[serde(rename = "code")]
    codigo: i32,
    /// Explicación para quien lea el registro o depure el cliente.
    #[serde(rename = "message")]
    mensaje: String,
}

impl ErrorJsonRpc {
    /// El texto recibido no era JSON.
    fn json_mal_formado(mensaje: impl Into<String>) -> Self {
        Self {
            codigo: CODIGO_JSON_MAL_FORMADO,
            mensaje: mensaje.into(),
        }
    }

    /// Era JSON, pero no una petición que cumpla el protocolo.
    fn peticion_invalida(mensaje: impl Into<String>) -> Self {
        Self {
            codigo: CODIGO_PETICION_INVALIDA,
            mensaje: mensaje.into(),
        }
    }

    /// La llamada existe, pero sus parámetros no tienen la forma publicada.
    pub(super) fn parametros_invalidos(mensaje: impl Into<String>) -> Self {
        Self {
            codigo: CODIGO_PARAMETROS_INVALIDOS,
            mensaje: mensaje.into(),
        }
    }

    /// El método pedido no lo ofrece este servidor.
    ///
    /// Un método que no se ofrece se contesta con el error que manda JSON-RPC, no con un
    /// éxito vacío. Los clientes tantean durante el descubrimiento —`resources/list`,
    /// `prompts/list`— y un `result: {}` les dice que el método existe y no devolvió nada,
    /// que es distinto de «no soportado» y les lleva a insistir.
    pub(super) fn metodo_no_encontrado(metodo: &str) -> Self {
        Self {
            codigo: CODIGO_METODO_NO_ENCONTRADO,
            mensaje: format!("Método no soportado: {metodo}"),
        }
    }
}

/// La respuesta que se devuelve por la salida estándar.
///
/// El campo `jsonrpc` no se recibe como dato ni se compone a mano en cada sitio: lo pone el
/// tipo, siempre con el mismo valor. Y el cuerpo es un enumerado, de modo que una respuesta no
/// puede llevar a la vez `result` y `error`, que el protocolo prohíbe y que un `json!` escrito
/// a mano no impide.
#[derive(Serialize)]
pub(crate) struct RespuestaJsonRpc {
    /// Versión del protocolo, siempre `"2.0"`.
    jsonrpc: &'static str,
    /// Identificador de la petición a la que se contesta.
    id: Value,
    /// Resultado o error, nunca los dos.
    #[serde(flatten)]
    cuerpo: CuerpoRespuesta,
}

/// Lo que lleva una respuesta: o lo que se pidió, o el motivo de que no se pueda dar.
#[derive(Serialize)]
enum CuerpoRespuesta {
    /// La llamada salió bien y este es su resultado.
    #[serde(rename = "result")]
    Resultado(Value),
    /// La llamada no se pudo atender.
    #[serde(rename = "error")]
    Fallo(ErrorJsonRpc),
}

impl RespuestaJsonRpc {
    /// Respuesta afirmativa a una petición atendida.
    fn con_resultado(id: &IdJsonRpc, resultado: Value) -> Self {
        Self {
            jsonrpc: VERSION_JSON_RPC,
            id: id.a_valor(),
            cuerpo: CuerpoRespuesta::Resultado(resultado),
        }
    }

    /// Respuesta de error, con el identificador de la petición cuando se pudo averiguar.
    fn con_error(id: &IdJsonRpc, error: ErrorJsonRpc) -> Self {
        Self {
            jsonrpc: VERSION_JSON_RPC,
            id: id.a_valor(),
            cuerpo: CuerpoRespuesta::Fallo(error),
        }
    }

    /// Convierte la respuesta en la línea de texto que se escribe en la salida estándar.
    ///
    /// # Devuelve
    /// El JSON de la respuesta en una sola línea. Si la conversión fallara —cosa que no puede
    /// ocurrir con estos tipos, porque todos sus campos son JSON válido por construcción—, se
    /// devuelve un error de protocolo antes que dejar al cliente sin respuesta.
    fn en_linea(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            json!({
                "jsonrpc": VERSION_JSON_RPC,
                "id": Value::Null,
                "error": {
                    "code": CODIGO_ERROR_INTERNO,
                    "message": "La respuesta no se pudo convertir a JSON"
                }
            })
            .to_string()
        })
    }
}

// ============================================================================
// Lectura y validación del sobre
// ============================================================================

/// Lee una petición completa sin permitir que una línea hostil agote la memoria.
///
/// La línea que excede el límite se consume por completo antes de devolver el error. Así, el
/// siguiente intento empieza en una petición nueva y el servidor puede seguir atendiendo al
/// cliente. El salto de línea —también cuando es `CRLF`— no cuenta como contenido.
pub(crate) fn leer_linea_acotada<R: BufRead>(
    entrada: &mut R,
    limite: u64,
) -> AppResult<Option<String>> {
    let mut contenido = Vec::with_capacity(limite.min(8 * 1024) as usize);
    let mut recibidos = 0_u64;
    let mut ultimo = None;

    loop {
        let (consumir, finaliza_linea, permite_retorno, parte) = {
            let disponibles = entrada
                .fill_buf()
                .map_err(|e| AppError::lectura("entrada estándar MCP", e))?;
            if disponibles.is_empty() {
                if recibidos == 0 {
                    return Ok(None);
                }
                (0, true, false, disponibles)
            } else if let Some(posicion) = disponibles.iter().position(|byte| *byte == b'\n') {
                (posicion + 1, true, true, &disponibles[..posicion])
            } else {
                (disponibles.len(), false, false, disponibles)
            }
        };

        recibidos = recibidos.saturating_add(parte.len() as u64);
        ultimo = parte.last().copied().or(ultimo);
        let espacio = limite
            .saturating_add(1)
            .saturating_sub(contenido.len() as u64);
        contenido.extend_from_slice(&parte[..parte.len().min(espacio as usize)]);
        entrada.consume(consumir);

        if finaliza_linea {
            let lleva_retorno = permite_retorno && ultimo == Some(b'\r');
            let longitud_real = recibidos.saturating_sub(u64::from(lleva_retorno));
            if longitud_real > limite {
                return Err(AppError::EntradaDemasiadoGrande {
                    origen: "entrada estándar MCP".to_string(),
                    limite,
                });
            }
            if lleva_retorno {
                contenido.pop();
            }
            let linea = String::from_utf8(contenido).map_err(|e| {
                AppError::lectura(
                    "entrada estándar MCP",
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e),
                )
            })?;
            return Ok(Some(linea));
        }
    }
}

/// Compone la respuesta que permite al cliente reconocer un mensaje descartado por tamaño.
pub(super) fn respuesta_entrada_demasiado_grande(limite: u64) -> String {
    RespuestaJsonRpc::con_error(
        &IdJsonRpc::Nulo,
        ErrorJsonRpc::peticion_invalida(format!(
            "La petición supera el límite de seguridad de {limite} bytes"
        )),
    )
    .en_linea()
}

/// El sobre tal como llega, sin interpretar todavía.
///
/// Cada campo se recoge como [`Value`] envuelto en `Option` para conservar la diferencia entre
/// **ausente** y **presente con un valor incorrecto**. Es la distinción que hace falta para
/// decidir bien: un `id` ausente es una notificación legítima, mientras que un `id` presente
/// que resulta ser un objeto es una petición rota; y un `method` ausente no es lo mismo que un
/// `method` numérico, aunque de ambos salga el mismo rechazo.
#[derive(Deserialize)]
struct SobreCrudo {
    /// Campo `jsonrpc`, que debe valer `"2.0"`.
    #[serde(rename = "jsonrpc", default, deserialize_with = "campo_presente")]
    version: Option<Value>,
    /// Campo `method`, que debe ser el nombre del método en texto.
    #[serde(rename = "method", default, deserialize_with = "campo_presente")]
    metodo: Option<Value>,
    /// Campo `id`, ausente en las notificaciones.
    #[serde(rename = "id", default, deserialize_with = "campo_presente")]
    identificador: Option<Value>,
    /// Campo `params`, con los argumentos de la llamada.
    #[serde(rename = "params", default, deserialize_with = "campo_presente")]
    parametros: Option<Value>,
}

/// Recoge un campo del sobre conservando que estaba, aunque su valor sea `null`.
///
/// Sin esto, `Option<Value>` convertiría un `"id": null` explícito en `None`, es decir, en lo
/// mismo que no traer `id`: una petición con identificador nulo se tomaría por una notificación
/// y se quedaría sin respuesta.
fn campo_presente<'de, D>(deserializador: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    Value::deserialize(deserializador).map(Some)
}

impl SobreCrudo {
    /// Comprueba el sobre entero y lo convierte en una petición utilizable.
    ///
    /// # Devuelve
    /// La petición validada, o el identificador que debe llevar la respuesta de error junto al
    /// error que corresponde.
    fn validar(self) -> Result<PeticionJsonRpc, (IdJsonRpc, ErrorJsonRpc)> {
        let SobreCrudo {
            version,
            metodo,
            identificador,
            parametros,
        } = self;

        let id = match identificador {
            None => None,
            Some(valor) => match IdJsonRpc::desde_valor(valor) {
                Some(id) => Some(id),
                None => {
                    return Err((
                        IdJsonRpc::Nulo,
                        ErrorJsonRpc::peticion_invalida(
                            "El campo «id» solo admite texto, número o nulo",
                        ),
                    ))
                }
            },
        };

        // A partir de aquí ya se sabe a qué llamada contestar, si es que hay que contestar.
        let referencia = id.clone().unwrap_or(IdJsonRpc::Nulo);

        if version.as_ref().and_then(Value::as_str) != Some(VERSION_JSON_RPC) {
            return Err((
                referencia,
                ErrorJsonRpc::peticion_invalida(format!(
                    "El campo «jsonrpc» debe estar y valer «{VERSION_JSON_RPC}»"
                )),
            ));
        }

        let Some(metodo) = metodo.as_ref().and_then(Value::as_str) else {
            return Err((
                referencia,
                ErrorJsonRpc::peticion_invalida(
                    "El campo «method» debe estar y ser el nombre del método, en texto",
                ),
            ));
        };

        Ok(PeticionJsonRpc {
            id,
            metodo: metodo.to_string(),
            parametros: ParametrosMcp::desde_valor(parametros),
        })
    }
}

// ============================================================================
// Entrada del módulo
// ============================================================================

/// Interpreta una línea de la entrada estándar y produce la respuesta que toca.
///
/// # Parámetros
/// - `linea`: el texto recibido, una petición JSON-RPC completa.
///
/// # Devuelve
/// La respuesta a enviar, o `None` si la petición era una notificación válida y por tanto no
/// espera ninguna.
pub(crate) fn procesar_linea(linea: &str) -> Option<String> {
    // Se descarta la marca de orden de bytes si el cliente la envía. Los clientes MCP
    // reales mandan UTF-8 limpio, pero algunos intérpretes de órdenes la anteponen al
    // encauzar texto hacia un programa, y sin quitarla el análisis fallaría en el primer
    // carácter con un mensaje que no ayuda a entender por qué.
    let linea = linea.trim_start_matches('\u{feff}');

    let sobre: SobreCrudo = match serde_json::from_str(linea) {
        Ok(sobre) => sobre,
        Err(fallo) => {
            return Some(
                RespuestaJsonRpc::con_error(&IdJsonRpc::Nulo, traducir_fallo(&fallo)).en_linea(),
            )
        }
    };

    match sobre.validar() {
        Ok(peticion) => atender(peticion),
        Err((id, error)) => Some(RespuestaJsonRpc::con_error(&id, error).en_linea()),
    }
}

/// Traduce el fallo de lectura al error del protocolo que le corresponde.
///
/// Hay dos maneras distintas de que una línea no llegue a ser un sobre, y el protocolo les da
/// códigos distintos: que el texto no sea JSON (`-32700`) o que sea JSON válido pero no un
/// objeto de petición —un array, un número, una cadena— (`-32600`).
fn traducir_fallo(fallo: &serde_json::Error) -> ErrorJsonRpc {
    if matches!(fallo.classify(), Category::Data) {
        ErrorJsonRpc::peticion_invalida(format!("La petición no es un objeto JSON-RPC: {fallo}"))
    } else {
        ErrorJsonRpc::json_mal_formado(format!("JSON mal formado: {fallo}"))
    }
}

/// Despacha una petición ya validada y compone su respuesta.
///
/// # Devuelve
/// La respuesta, o `None` si la petición era una notificación.
fn atender(peticion: PeticionJsonRpc) -> Option<String> {
    // Las notificaciones no llevan `id` y no esperan respuesta, ni siquiera de error.
    let id = peticion.id.clone()?;

    let respuesta = match super::operaciones::atender_metodo(&peticion) {
        Ok(resultado) => RespuestaJsonRpc::con_resultado(&id, resultado),
        Err(error) => RespuestaJsonRpc::con_error(&id, error),
    };

    Some(respuesta.en_linea())
}
