//! Preparación neutral de las instrucciones que MMCelt entrega a un agente externo.
//!
//! Este módulo no abre procesos ni escribe expedientes. Su única responsabilidad es reunir las
//! fuentes autorizadas y componer un prompt visible y reproducible antes del lanzamiento.

use crate::conectores::IdAgente;
use crate::configuracion_agente_proyecto::EstadoFuenteNativa;
use crate::error::{AppError, AppResult, CampoEntrada, MotivoEntradaInvalida};
use crate::proyecto_trabajo::ContextoProyecto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Nombres portables que MMCelt reconoce como instrucciones nativas en la raíz del proyecto.
pub(crate) const NOMBRES_FUENTES_NATIVAS: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "GEMINI.md"];
use uuid::Uuid;

/// Ruta portable de las instrucciones comunes que el usuario decide compartir con todo agente.
pub const RUTA_REGLAS_COMUNES: &str = ".mmcelt/instrucciones-agente.md";

/// Tamaño máximo de cada bloque que puede escribir directamente la persona.
pub const BYTES_MAXIMOS_BLOQUE_EDITABLE: usize = 65_536;
/// Tamaño máximo de la cadena completa antes de crear un expediente o abrir un proceso.
pub const BYTES_MAXIMOS_PROMPT: usize = 16 * 1024 * 1024;

/// Archivo de instrucciones propio de un producto detectado en la raíz del proyecto.
///
/// Estas fuentes se presentan en la vista previa para que el usuario conozca las reglas que cada
/// herramienta puede descubrir por su cuenta. MMCelt no las incorpora al prompt común sin una
/// decisión expresa, porque sus ámbitos y precedencias no son equivalentes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuenteInstrucciones {
    /// Nombre canónico del archivo nativo.
    pub nombre: &'static str,
    /// Ruta absoluta validada dentro del proyecto.
    pub ruta: PathBuf,
    /// Contenido disponible para la vista previa.
    pub contenido: String,
    /// Indica si el usuario decidió incorporarlo al prompt común.
    pub incorporada: bool,
    /// Resultado de compararlo con la última versión aceptada por la persona.
    pub estado: EstadoFuenteNativa,
    /// Versión aceptada anterior, disponible para comparar cuando el archivo cambia.
    pub contenido_anterior: Option<String>,
}

/// Contenido editable de una sesión antes de abrir la consola del agente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorradorSesion {
    /// Producto que recibirá el prompt, sin inferir reglas por su nombre visible.
    pub agente: IdAgente,
    /// Contrato compartido para devolver trabajo a MMCelt mediante MCP.
    pub contrato_mmcelt: String,
    /// Reglas comunes y portables del proyecto.
    pub reglas_proyecto: String,
    /// Representación del mapa que el usuario está enviando.
    pub contexto_mapa: String,
    /// Petición concreta para esta ejecución.
    pub encargo: String,
    /// Fuentes nativas mostradas aparte y no incorporadas silenciosamente.
    pub fuentes_nativas: Vec<FuenteInstrucciones>,
}

/// Huella SHA-256 del texto exacto que se mostró antes de confirmar una sesión.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuellaPrompt([u8; 32]);

impl HuellaPrompt {
    /// Devuelve los 32 bytes de la huella para compararlos o persistirlos sin reformatearlos.
    pub fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// Resultado inmutable del único compositor de prompts de sesión.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptSesionCompuesto {
    agente: IdAgente,
    contrato: String,
    reglas_proyecto: String,
    contexto_mapa: String,
    encargo: String,
    texto: String,
    huella: HuellaPrompt,
    fuentes_nativas: Vec<FuenteInstrucciones>,
}

/// Instantánea que demuestra qué se revisó y qué estado debía seguir vigente al lanzarlo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmacionSesion {
    /// Prompt inmutable que se mostrará, persistirá y entregará sin recomponerlo.
    pub prompt: PromptSesionCompuesto,
    /// Revisión del mapa con la que se generó su contexto.
    pub revision_mapa: crate::model::RevisionProyecto,
    /// Huella de las reglas portables observadas al preparar la vista.
    pub huella_reglas: [u8; 32],
    /// Huella del encargo exacto confirmado.
    pub huella_encargo: [u8; 32],
    /// Huellas de todas las fuentes nativas detectadas, indexadas por nombre.
    pub huellas_fuentes: BTreeMap<String, [u8; 32]>,
}

impl ConfirmacionSesion {
    /// Cierra una vista validada junto con las identidades de sus entradas cambiantes.
    pub fn nueva(
        prompt: PromptSesionCompuesto,
        revision_mapa: crate::model::RevisionProyecto,
        reglas: &str,
        encargo: &str,
        fuentes: &[FuenteInstrucciones],
    ) -> Self {
        Self {
            prompt,
            revision_mapa,
            huella_reglas: huella_texto(reglas),
            huella_encargo: huella_texto(encargo),
            huellas_fuentes: huellas_de_fuentes(fuentes),
        }
    }

    /// Comprueba que ninguna entrada cambió desde que la persona revisó la vista.
    ///
    /// # Errores
    ///
    /// Devuelve [`AppError::ConfirmacionCaducada`] ante la primera diferencia.
    pub fn validar(
        &self,
        revision_mapa: crate::model::RevisionProyecto,
        reglas: &str,
        encargo: &str,
        fuentes: &[FuenteInstrucciones],
    ) -> AppResult<()> {
        if self.revision_mapa != revision_mapa
            || self.huella_reglas != huella_texto(reglas)
            || self.huella_encargo != huella_texto(encargo)
            || self.huellas_fuentes != huellas_de_fuentes(fuentes)
        {
            return Err(AppError::ConfirmacionCaducada);
        }
        Ok(())
    }
}

/// Calcula una huella SHA-256 estable sobre los bytes UTF-8 visibles.
fn huella_texto(texto: &str) -> [u8; 32] {
    Sha256::digest(texto.as_bytes()).into()
}

/// Identifica el conjunto completo de fuentes, incluida una adición o eliminación.
fn huellas_de_fuentes(fuentes: &[FuenteInstrucciones]) -> BTreeMap<String, [u8; 32]> {
    fuentes
        .iter()
        .map(|fuente| (fuente.nombre.to_string(), huella_texto(&fuente.contenido)))
        .collect()
}

impl PromptSesionCompuesto {
    /// Devuelve el producto para el que se confirmó esta composición.
    pub fn agente(&self) -> IdAgente {
        self.agente
    }

    /// Devuelve el contrato protegido que se incluyó en la composición.
    pub fn contrato(&self) -> &str {
        &self.contrato
    }

    /// Devuelve las reglas del proyecto, incluidas las fuentes aceptadas.
    pub fn reglas_proyecto(&self) -> &str {
        &self.reglas_proyecto
    }

    /// Devuelve la representación protegida del mapa.
    pub fn contexto_mapa(&self) -> &str {
        &self.contexto_mapa
    }

    /// Devuelve el único encargo de esta sesión.
    pub fn encargo(&self) -> &str {
        &self.encargo
    }

    /// Devuelve la cadena exacta que ve el agente y que se guarda en `inicio.md`.
    pub fn texto(&self) -> &str {
        &self.texto
    }

    /// Devuelve la huella SHA-256 calculada sobre [`Self::texto`].
    pub fn huella(&self) -> HuellaPrompt {
        self.huella
    }
}

/// Valida y compone una sola representación inmutable de los cuatro bloques de la sesión.
///
/// # Errores
///
/// Rechaza controles invisibles, bloques editables de más de 65.536 bytes y una composición final
/// superior a 16 MiB. Ningún llamador debe reconstruir la cadena por su cuenta.
pub fn componer_prompt_validado(borrador: &BorradorSesion) -> AppResult<PromptSesionCompuesto> {
    validar_bloque_editable(
        CampoEntrada::Reglas,
        "reglas del proyecto",
        &borrador.reglas_proyecto,
    )?;
    validar_bloque_editable(CampoEntrada::Encargo, "encargo", &borrador.encargo)?;
    validar_sin_controles(CampoEntrada::ContratoMmcelt, &borrador.contrato_mmcelt)?;
    validar_sin_controles(CampoEntrada::Contexto, &borrador.contexto_mapa)?;

    let mut reglas_proyecto = borrador.reglas_proyecto.clone();
    for fuente in borrador
        .fuentes_nativas
        .iter()
        .filter(|fuente| fuente.incorporada)
    {
        validar_sin_controles(CampoEntrada::FuenteNativa, &fuente.contenido)?;
        reglas_proyecto.push_str("\n\n## FUENTE NATIVA ACEPTADA: ");
        reglas_proyecto.push_str(fuente.nombre);
        reglas_proyecto.push_str("\n\n");
        reglas_proyecto.push_str(&fuente.contenido);
    }

    let mut texto = String::new();
    texto.push_str("# CONTRATO MMCELT\n\n");
    texto.push_str(
        "Precedencia: Correcciones humanas > contrato MMCelt > reglas del proyecto > encargo.\n\n",
    );
    texto.push_str(&borrador.contrato_mmcelt);
    texto.push_str("\n# REGLAS COMUNES DEL PROYECTO\n\n");
    texto.push_str(&reglas_proyecto);
    texto.push_str("\n# CONTEXTO DEL MAPA\n\n");
    texto.push_str(&borrador.contexto_mapa);
    texto.push_str("\n# ENCARGO DE ESTA SESIÓN\n\n");
    texto.push_str(&borrador.encargo);
    if texto.len() > BYTES_MAXIMOS_PROMPT {
        return Err(AppError::EntradaDemasiadoGrande {
            origen: "prompt de sesión".to_string(),
            limite: BYTES_MAXIMOS_PROMPT as u64,
        });
    }
    let huella = HuellaPrompt(Sha256::digest(texto.as_bytes()).into());

    Ok(PromptSesionCompuesto {
        agente: borrador.agente,
        contrato: borrador.contrato_mmcelt.clone(),
        reglas_proyecto,
        contexto_mapa: borrador.contexto_mapa.clone(),
        encargo: borrador.encargo.clone(),
        texto,
        huella,
        fuentes_nativas: borrador.fuentes_nativas.clone(),
    })
}

/// Comprueba tamaño y controles de un campo que la persona puede editar.
fn validar_bloque_editable(
    campo: CampoEntrada,
    nombre_origen: &'static str,
    contenido: &str,
) -> AppResult<()> {
    if contenido.len() > BYTES_MAXIMOS_BLOQUE_EDITABLE {
        return Err(AppError::EntradaDemasiadoGrande {
            origen: nombre_origen.to_string(),
            limite: BYTES_MAXIMOS_BLOQUE_EDITABLE as u64,
        });
    }
    validar_sin_controles(campo, contenido)
}

/// Rechaza controles distintos del salto de línea y el tabulador, que sí forman parte del texto.
fn validar_sin_controles(campo: CampoEntrada, contenido: &str) -> AppResult<()> {
    if contenido
        .chars()
        .any(|caracter| caracter.is_control() && caracter != '\n' && caracter != '\t')
    {
        return Err(AppError::EntradaInvalida {
            campo,
            motivo: MotivoEntradaInvalida::CaracteresDeControlInvisibles,
        });
    }
    Ok(())
}

/// Lee las reglas comunes de la ubicación portable del proyecto.
///
/// La ausencia del archivo representa un perfil común todavía vacío. Cualquier otro fallo de
/// lectura se propaga con su ruta, y un enlace que salga de la raíz se rechaza antes de leerlo.
///
/// # Errores
///
/// Devuelve un error si la ruta resuelve fuera del proyecto o si el archivo existe pero no se
/// puede leer como texto UTF-8.
pub fn cargar_reglas_comunes(contexto: &ContextoProyecto) -> AppResult<String> {
    let ruta = contexto.resolver_relativa(Path::new(RUTA_REGLAS_COMUNES))?;
    if !ruta.exists() {
        return Ok(String::new());
    }
    std::fs::read_to_string(&ruta).map_err(|error| AppError::lectura(&ruta, error))
}

/// Guarda las reglas comunes dentro de la carpeta portable mediante reemplazo atómico.
///
/// # Errores
///
/// Devuelve un error si el destino sale del proyecto o no se puede crear ni escribir.
///
/// # Efectos secundarios
///
/// Crea `.mmcelt` cuando aún no existe y reemplaza `instrucciones-agente.md` atómicamente.
pub fn guardar_reglas_comunes(contexto: &ContextoProyecto, contenido: &str) -> AppResult<()> {
    let ruta = contexto.resolver_relativa(Path::new(RUTA_REGLAS_COMUNES))?;
    let carpeta = ruta.parent().ok_or(AppError::RutaFueraDelProyecto)?;
    std::fs::create_dir_all(carpeta).map_err(|error| AppError::escritura(carpeta, error))?;
    crate::storage::escribir_de_forma_atomica(&ruta, contenido.as_bytes())
}

/// Detecta los archivos de instrucciones propios de los agentes en la raíz autorizada.
///
/// Solo reconoce nombres concretos y no recorre subcarpetas. Cada ruta pasa por el límite
/// canónico de [`ContextoProyecto`] antes de tocar el archivo, incluida la resolución de enlaces.
///
/// # Errores
///
/// Devuelve un error si una ruta nativa escapa del proyecto o si un archivo detectado no se puede
/// leer como texto UTF-8.
pub fn detectar_fuentes_nativas(
    contexto: &ContextoProyecto,
) -> AppResult<Vec<FuenteInstrucciones>> {
    let configuracion = crate::configuracion_agente_proyecto::cargar_configuracion(contexto)?;
    let mut fuentes = Vec::new();
    for nombre in NOMBRES_FUENTES_NATIVAS {
        let ruta = contexto.resolver_relativa(Path::new(nombre))?;
        if !ruta.exists() {
            continue;
        }
        let contenido =
            std::fs::read_to_string(&ruta).map_err(|error| AppError::lectura(&ruta, error))?;
        let (estado, incorporada, contenido_anterior) =
            crate::configuracion_agente_proyecto::evaluar_fuente(
                &configuracion,
                nombre,
                &contenido,
            );
        fuentes.push(FuenteInstrucciones {
            nombre,
            ruta,
            contenido,
            incorporada,
            estado,
            contenido_anterior,
        });
    }
    Ok(fuentes)
}

/// Versión inicial del contrato JSON de los expedientes de sesión.
const VERSION_FORMATO_SESION: u32 = 1;
/// Carpeta portable que agrupa las sesiones confirmadas por el usuario.
const RUTA_SESIONES: &str = ".mmcelt/sesiones";
/// Nombre del archivo que conserva el prompt exacto aprobado.
const NOMBRE_PROMPT: &str = "inicio.md";
/// Nombre del archivo que conserva los datos mínimos del lanzamiento.
const NOMBRE_METADATOS: &str = "sesion.json";

/// Estado durable del intento de lanzamiento de una consola externa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstadoLanzamiento {
    /// El prompt está persistido, pero todavía no se ha abierto la consola.
    Preparado,
    /// El sistema operativo confirmó la creación del proceso.
    Iniciado,
    /// La apertura de la consola devolvió un error.
    Fallido,
}

/// Datos reproducibles de una sesión, sin información de cuenta ni credenciales.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadatosSesion {
    /// Versión del esquema persistido.
    pub version_formato: u32,
    /// Identidad aleatoria de esta confirmación concreta.
    pub id_sesion: Uuid,
    /// Momento UTC en que se confirmó el prompt.
    pub fecha_utc: DateTime<Utc>,
    /// Producto elegido por identidad estable.
    pub agente: IdAgente,
    /// Ruta del mapa relativa a la raíz autorizada.
    pub mapa_relativo: PathBuf,
    /// Fuentes que el usuario incorporó expresamente al perfil común.
    pub fuentes_incorporadas: Vec<PathBuf>,
    /// Fuentes nativas visibles que la herramienta puede descubrir por su cuenta.
    pub fuentes_nativas_detectadas: Vec<PathBuf>,
    /// Longitud en bytes UTF-8 del prompt exacto guardado en `inicio.md`.
    pub prompt_bytes: usize,
    /// Último estado persistido del lanzamiento.
    pub estado: EstadoLanzamiento,
}

/// Expediente ya confirmado y cerrado, listo para pasarlo al lanzador de consola.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SesionPreparada {
    /// Carpeta definitiva de esta sesión.
    pub carpeta: PathBuf,
    /// Archivo que contiene el prompt aprobado.
    pub ruta_inicio: PathBuf,
    /// Archivo JSON cuyos cambios de estado se escriben atómicamente.
    pub ruta_metadatos: PathBuf,
    /// Copia en memoria de los mismos metadatos persistidos.
    pub metadatos: MetadatosSesion,
}

/// Persiste un prompt confirmado antes de abrir ningún proceso externo.
///
/// Primero valida el mapa y todas las fuentes. Después escribe dentro de una carpeta temporal y
/// solo la renombra al nombre definitivo cuando ambos archivos están cerrados. Un error retira
/// exclusivamente los dos archivos conocidos y esa carpeta temporal ya validada.
///
/// # Errores
///
/// Devuelve un error si el mapa o alguna fuente están fuera del proyecto, si no puede crear el
/// expediente, si falla su serialización o si cualquiera de las escrituras atómicas falla.
///
/// # Efectos secundarios
///
/// Crea una carpeta bajo `.mmcelt/sesiones` y guarda en ella el prompt aprobado y sus metadatos.
pub fn preparar_sesion(
    contexto: &ContextoProyecto,
    ruta_mapa: &Path,
    prompt: &PromptSesionCompuesto,
) -> AppResult<SesionPreparada> {
    let mapa_absoluto = resolver_ruta_del_mapa(contexto, ruta_mapa)?;
    let mapa_relativo = hacer_relativa(contexto, &mapa_absoluto)?;
    let fuentes_incorporadas = rutas_relativas_de_fuentes(contexto, prompt, true)?;
    let fuentes_nativas_detectadas = rutas_relativas_de_fuentes(contexto, prompt, false)?;
    let id_sesion = Uuid::new_v4();
    let fecha_utc = Utc::now();
    let metadatos = MetadatosSesion {
        version_formato: VERSION_FORMATO_SESION,
        id_sesion,
        fecha_utc,
        agente: prompt.agente,
        mapa_relativo,
        fuentes_incorporadas,
        fuentes_nativas_detectadas,
        prompt_bytes: prompt.texto.len(),
        estado: EstadoLanzamiento::Preparado,
    };
    let json = serde_json::to_vec_pretty(&metadatos)
        .map_err(|error| AppError::formato("al serializar el expediente de sesión", error))?;

    let carpeta_sesiones = contexto.resolver_relativa(Path::new(RUTA_SESIONES))?;
    let nombre_final = nombre_de_sesion(fecha_utc, prompt.agente, id_sesion);
    let carpeta_final = carpeta_sesiones.join(nombre_final);
    let carpeta_temporal = carpeta_sesiones.join(format!(".tmp-{id_sesion}"));
    comprobar_ruta_interior(contexto, &carpeta_final)?;
    comprobar_ruta_interior(contexto, &carpeta_temporal)?;

    std::fs::create_dir_all(&carpeta_sesiones)
        .map_err(|error| AppError::escritura(&carpeta_sesiones, error))?;
    std::fs::create_dir(&carpeta_temporal)
        .map_err(|error| AppError::escritura(&carpeta_temporal, error))?;

    let ruta_inicio_temporal = carpeta_temporal.join(NOMBRE_PROMPT);
    let ruta_metadatos_temporal = carpeta_temporal.join(NOMBRE_METADATOS);
    let escritura = (|| -> AppResult<()> {
        crate::storage::escribir_de_forma_atomica(&ruta_inicio_temporal, prompt.texto.as_bytes())?;
        crate::storage::escribir_de_forma_atomica(&ruta_metadatos_temporal, &json)?;
        std::fs::rename(&carpeta_temporal, &carpeta_final)
            .map_err(|error| AppError::escritura(&carpeta_final, error))?;
        Ok(())
    })();
    if let Err(error) = escritura {
        limpiar_temporal_conocido(&carpeta_temporal);
        return Err(error);
    }

    Ok(SesionPreparada {
        ruta_inicio: carpeta_final.join(NOMBRE_PROMPT),
        ruta_metadatos: carpeta_final.join(NOMBRE_METADATOS),
        carpeta: carpeta_final,
        metadatos,
    })
}

/// Persiste una transición de lanzamiento y solo después actualiza la copia en memoria.
///
/// # Errores
///
/// Devuelve un error si el expediente ya no pertenece al proyecto, su estructura o identidad no
/// coinciden con los metadatos, no se puede serializar o falla la escritura atómica.
///
/// # Efectos secundarios
///
/// Reemplaza de forma atómica `sesion.json`; la copia en memoria solo cambia si la escritura acaba.
pub fn actualizar_estado(
    contexto: &ContextoProyecto,
    sesion: &mut SesionPreparada,
    estado: EstadoLanzamiento,
) -> AppResult<()> {
    validar_destinos_sesion(contexto, sesion)?;
    let mut actualizados = sesion.metadatos.clone();
    actualizados.estado = estado;
    let json = serde_json::to_vec_pretty(&actualizados)
        .map_err(|error| AppError::formato("al serializar el estado de la sesión", error))?;
    crate::storage::escribir_de_forma_atomica(&sesion.ruta_metadatos, &json)?;
    sesion.metadatos = actualizados;
    Ok(())
}

/// Comprueba que una transición solo pueda reescribir el JSON del expediente que la originó.
fn validar_destinos_sesion(contexto: &ContextoProyecto, sesion: &SesionPreparada) -> AppResult<()> {
    comprobar_ruta_interior(contexto, &sesion.carpeta)?;
    comprobar_ruta_interior(contexto, &sesion.ruta_inicio)?;
    comprobar_ruta_interior(contexto, &sesion.ruta_metadatos)?;
    let carpeta_sesiones = contexto.resolver_relativa(Path::new(RUTA_SESIONES))?;
    let base_canonica = carpeta_sesiones
        .canonicalize()
        .map_err(|error| AppError::lectura(&carpeta_sesiones, error))?;
    let carpeta_canonica = sesion
        .carpeta
        .canonicalize()
        .map_err(|error| AppError::lectura(&sesion.carpeta, error))?;
    let nombre_esperado = nombre_de_sesion(
        sesion.metadatos.fecha_utc,
        sesion.metadatos.agente,
        sesion.metadatos.id_sesion,
    );
    let estructura_valida = carpeta_canonica.parent() == Some(base_canonica.as_path())
        && sesion
            .carpeta
            .file_name()
            .and_then(|nombre| nombre.to_str())
            == Some(nombre_esperado.as_str())
        && sesion.ruta_inicio == sesion.carpeta.join(NOMBRE_PROMPT)
        && sesion.ruta_metadatos == sesion.carpeta.join(NOMBRE_METADATOS);
    if !estructura_valida {
        return Err(AppError::RutaFueraDelProyecto);
    }

    let json_existente = std::fs::read(&sesion.ruta_metadatos)
        .map_err(|error| AppError::lectura(&sesion.ruta_metadatos, error))?;
    let metadatos_existentes: MetadatosSesion = serde_json::from_slice(&json_existente)
        .map_err(|error| AppError::formato("al validar el expediente de sesión", error))?;
    if metadatos_existentes == sesion.metadatos {
        Ok(())
    } else {
        Err(AppError::RutaFueraDelProyecto)
    }
}

/// Resuelve una ruta absoluta o relativa y exige que permanezca dentro del proyecto.
fn resolver_ruta_del_mapa(contexto: &ContextoProyecto, ruta: &Path) -> AppResult<PathBuf> {
    let propuesta = if ruta.is_absolute() {
        ruta.to_path_buf()
    } else {
        contexto.resolver_relativa(ruta)?
    };
    comprobar_ruta_interior(contexto, &propuesta)?;
    let canonica = propuesta
        .canonicalize()
        .map_err(|error| AppError::lectura(&propuesta, error))?;
    comprobar_ruta_interior(contexto, &canonica)?;
    if canonica.is_file() {
        Ok(canonica)
    } else {
        Err(AppError::lectura(
            &canonica,
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "la ruta de la sesión no es un archivo de mapa",
            ),
        ))
    }
}

/// Rechaza cualquier ruta cuya resolución canónica salga de la raíz autorizada.
fn comprobar_ruta_interior(contexto: &ContextoProyecto, ruta: &Path) -> AppResult<()> {
    if contexto.contiene(ruta)? {
        Ok(())
    } else {
        Err(AppError::RutaFueraDelProyecto)
    }
}

/// Convierte una ruta validada a su representación portable dentro del proyecto.
fn hacer_relativa(contexto: &ContextoProyecto, ruta: &Path) -> AppResult<PathBuf> {
    comprobar_ruta_interior(contexto, ruta)?;
    ruta.strip_prefix(contexto.raiz())
        .map(Path::to_path_buf)
        .map_err(|_| AppError::RutaFueraDelProyecto)
}

/// Reúne rutas relativas de fuentes incorporadas o de todas las nativas detectadas.
fn rutas_relativas_de_fuentes(
    contexto: &ContextoProyecto,
    prompt: &PromptSesionCompuesto,
    solo_incorporadas: bool,
) -> AppResult<Vec<PathBuf>> {
    prompt
        .fuentes_nativas
        .iter()
        .filter(|fuente| !solo_incorporadas || fuente.incorporada)
        .map(|fuente| hacer_relativa(contexto, &fuente.ruta))
        .collect()
}

/// Forma un nombre portable sin títulos, encargos ni ningún otro texto del usuario.
fn nombre_de_sesion(fecha: DateTime<Utc>, agente: IdAgente, id: Uuid) -> String {
    let agente = match agente {
        IdAgente::ClaudeCode => "claude-code",
        IdAgente::ClaudeDesktop => "claude-desktop",
        IdAgente::Antigravity => "antigravity",
        IdAgente::Cursor => "cursor",
        IdAgente::Windsurf => "windsurf",
        IdAgente::CodexCli => "codex-cli",
        IdAgente::GeminiCli => "gemini-cli",
    };
    format!("{}-{agente}-{id}", fecha.format("%Y%m%dT%H%M%S%.3fZ"))
}

/// Retira solo los archivos que esta operación conoce y después intenta retirar su carpeta vacía.
fn limpiar_temporal_conocido(carpeta: &Path) {
    let _ = std::fs::remove_file(carpeta.join(NOMBRE_PROMPT));
    let _ = std::fs::remove_file(carpeta.join(NOMBRE_METADATOS));
    let _ = std::fs::remove_dir(carpeta);
}
