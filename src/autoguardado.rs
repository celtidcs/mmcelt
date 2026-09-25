//! # Módulo de Autoguardado y Recuperación (`autoguardado.rs`)
//!
//! Guarda periódicamente una copia del proyecto en curso para que un cierre inesperado
//! no se lleve por delante el trabajo del usuario.
//!
//! ## Qué es y qué no es
//!
//! El autoguardado **no sustituye a guardar**. Es una red de seguridad, y esa distinción
//! determina todo su diseño:
//!
//! | | Guardado explícito (`Ctrl+S`) | Autoguardado |
//! |---|---|---|
//! | Lo decide | El usuario | El programa |
//! | Dónde escribe | Donde el usuario elija | Siempre en el directorio de datos del usuario |
//! | Pregunta | Sí, la primera vez | **Nunca** |
//! | Para qué sirve | Conservar el trabajo | Recuperarlo tras un cierre inesperado |
//!
//! De ahí las dos reglas que sigue este módulo:
//!
//! 1. **No interrumpe.** Un diálogo modal cada pocos minutos sería inaceptable, así que
//!    el autoguardado no puede preguntar dónde escribir: necesita una ubicación conocida
//!    de antemano.
//! 2. **No escribe donde sorprenda.** Nunca toca el archivo del usuario ni deja
//!    artefactos junto a él. Escribe en el directorio de datos de la aplicación, y solo
//!    ahí.
//!
//! ## Por qué el directorio de datos y no la carpeta de la aplicación
//!
//! Escribir junto al ejecutable parece cómodo, pero falla en el caso más habitual: si el
//! binario está en `Program Files` (o en `/usr/local/bin`), **el proceso no tiene permiso
//! de escritura** y el autoguardado fallaría en silencio justo cuando más falta hace.
//!
//! Además, un usuario que busque sus datos no mira en la carpeta del programa.
//!
//! | Sistema | Ubicación |
//! |---|---|
//! | Windows | `%APPDATA%\MMCelt\recuperacion\` |
//! | macOS | `~/Library/Application Support/MMCelt/recuperacion/` |
//! | Linux | `$XDG_DATA_HOME/mmcelt/recuperacion/` o `~/.local/share/mmcelt/recuperacion/` |
//!
//! ## Ciclo de vida
//!
//! ```text
//!   arranque ──► ¿hay recuperación pendiente? ──► sí ──► se ofrece restaurarla
//!       │                                                        │
//!       ▼                                                        ▼
//!   trabajo normal ◄─────────────────────────────── restaurar o descartar
//!       │
//!       ├─ cada 2 minutos: ¿ha cambiado el proyecto? ─► sí ─► escribir recuperación
//!       │
//!       └─ al guardar con Ctrl+S ──────────────────────────► descartar recuperación
//! ```
//!
//! La recuperación se descarta al guardar de verdad, porque a partir de ese momento el
//! archivo del usuario ya contiene el trabajo y mantener la copia solo serviría para
//! ofrecer una restauración obsoleta en el próximo arranque.

use crate::error::{AppError, AppResult};
use crate::model::{ModoDisposicion, Proyecto, RevisionProyecto};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Cada cuánto se comprueba si el proyecto ha cambiado para autoguardarlo.
///
/// Dos minutos es un compromiso: lo bastante frecuente para que perder ese trabajo no
/// duela, y lo bastante espaciado para que serializar el mapa no se note en el pulso de
/// la interfaz.
pub const INTERVALO_AUTOGUARDADO: Duration = Duration::from_secs(120);

/// Cuántos minutos puede elegir el usuario entre copia y copia.
///
/// El **cero desactiva** el autoguardado; no es un intervalo de duración nula. Los demás
/// valores cubren el rango útil: un minuto para quien escribe deprisa y no quiere perder
/// nada, diez para quien trabaja sobre mapas grandes y prefiere que el disco descanse.
///
/// Son valores cerrados y no un número libre a propósito: un intervalo de dos segundos
/// escribiría el mapa entero continuamente, y uno de ocho horas es no tener autoguardado
/// mientras aparenta tenerlo.
pub const MINUTOS_DE_AUTOGUARDADO_OFRECIDOS: [u64; 5] = [0, 1, 2, 5, 10];

/// Cuántos minutos separan dos copias mientras el usuario no diga otra cosa.
pub const MINUTOS_DE_AUTOGUARDADO_POR_DEFECTO: u64 = 2;

/// Nombre de la carpeta de la aplicación dentro del directorio de datos del usuario.
const CARPETA_APLICACION: &str = "MMCelt";

/// Subcarpeta donde se deja la copia de recuperación.
const CARPETA_RECUPERACION: &str = "recuperacion";

/// Nombre del archivo de recuperación.
///
/// Es único: solo se conserva la última sesión. Guardar un histórico convertiría la
/// carpeta en un vertedero sin aportar nada, porque lo que interesa recuperar siempre es
/// lo último que se estaba haciendo.
const ARCHIVO_RECUPERACION: &str = "sesion.autoguardado.json";

/// Contenido del archivo de recuperación.
///
/// Envuelve el proyecto junto con los datos que hacen falta para presentarle al usuario
/// una decisión informada: de qué archivo procedía y de cuándo es la copia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recuperacion {
    /// Proyecto tal como estaba en el momento del autoguardado.
    pub proyecto: Proyecto,

    /// Archivo del usuario al que correspondía, si el proyecto ya se había guardado.
    ///
    /// Es `None` cuando el usuario nunca llegó a guardar: el caso en el que la
    /// recuperación más valor tiene, porque no hay ningún otro rastro del trabajo.
    pub ruta_original: Option<PathBuf>,

    /// Momento en que se escribió la copia.
    pub momento: DateTime<Utc>,
}

impl Recuperacion {
    /// Describe la copia en una frase, para mostrarla al usuario al arrancar.
    ///
    /// # Devuelve
    /// Un texto del estilo «del 21/08/2026 a las 19:45, de `mapa.mmcelt`».
    pub fn descripcion(&self) -> String {
        let momento = self.momento.format("%d/%m/%Y a las %H:%M");

        match &self.ruta_original {
            Some(ruta) => {
                let nombre = ruta
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| ruta.display().to_string());
                format!("del {momento}, de «{nombre}»")
            }
            None => format!("del {momento}, de un mapa que nunca llegó a guardarse"),
        }
    }
}

/// Variable de entorno que permite reubicar el directorio de datos de la aplicación.
///
/// Sirve para dos casos reales:
/// - **Instalación portable**: llevar MMCelt en una memoria USB con sus datos al lado.
/// - **Pruebas**: aislar el archivo de recuperación para que no interfiera con el del
///   usuario ni entre pruebas que se ejecutan a la vez.
pub const VARIABLE_DIRECTORIO_DATOS: &str = "MMCELT_DATOS";

/// Devuelve el directorio de datos de la aplicación para el usuario actual.
///
/// Si la variable de entorno [`VARIABLE_DIRECTORIO_DATOS`] está definida, se usa su
/// valor. En caso contrario se sigue la convención de cada sistema operativo.
///
/// No crea el directorio: de eso se encarga [`escribir_recuperacion_en`] cuando hace falta.
///
/// # Devuelve
/// La ruta del directorio, o `None` si no se puede determinar (por ejemplo, si las
/// variables de entorno esperadas no existen). En ese caso el autoguardado queda
/// desactivado sin más consecuencias: la aplicación sigue funcionando, simplemente sin
/// red de seguridad.
pub fn directorio_datos() -> Option<PathBuf> {
    if let Some(personalizado) = std::env::var_os(VARIABLE_DIRECTORIO_DATOS) {
        let ruta = PathBuf::from(personalizado);
        if !ruta.as_os_str().is_empty() {
            return Some(ruta);
        }
    }

    let base = if cfg!(target_os = "windows") {
        // %APPDATA% apunta a AppData\Roaming, que es donde va la configuración que
        // acompaña al usuario entre equipos de un dominio.
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support"))
    } else {
        // Linux y demás: se respeta XDG_DATA_HOME si está definida.
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
    }?;

    Some(base.join(CARPETA_APLICACION))
}

/// Devuelve la ruta del archivo de recuperación **dentro de la carpeta que se le indique**.
///
/// Esta es la función de verdad; [`ruta_recuperacion_en`] es el envoltorio que resuelve la
/// carpeta del usuario y llama aquí.
///
/// Que la carpeta llegue por parámetro no es un capricho de diseño: mientras dependía de una
/// variable de entorno, aislar una prueba obligaba a **escribir** esa variable, y escribir el
/// entorno mientras otro hilo lo lee es una carrera de datos que Rust marca como `unsafe`. La
/// batería corre más de trescientas pruebas a la vez.
///
/// # Parámetros
/// - `carpeta_datos`: carpeta de datos de la aplicación.
///
/// # Devuelve
/// La ruta del archivo de recuperación bajo esa carpeta.
pub fn ruta_recuperacion_en(carpeta_datos: &Path) -> PathBuf {
    carpeta_datos
        .join(CARPETA_RECUPERACION)
        .join(ARCHIVO_RECUPERACION)
}

/// Escribe la copia de recuperación **en la carpeta que se le indique**.
///
/// Crea el directorio si no existe.
///
/// # Parámetros
/// - `carpeta_datos`: carpeta de datos donde vive la copia.
/// - `proyecto`: proyecto a copiar.
/// - `ruta_original`: archivo del usuario al que corresponde, si lo hay.
///
/// # Errores
/// - [`AppError::Escritura`] si no se puede crear el directorio o escribir el archivo.
/// - [`AppError::Formato`] si el proyecto no se puede serializar.
pub fn escribir_recuperacion_en(
    carpeta_datos: &Path,
    proyecto: &Proyecto,
    ruta_original: Option<&Path>,
) -> AppResult<()> {
    let destino = ruta_recuperacion_en(carpeta_datos);

    if let Some(carpeta) = destino.parent() {
        std::fs::create_dir_all(carpeta).map_err(|e| AppError::escritura(carpeta, e))?;
    }

    let contenido = Recuperacion {
        proyecto: proyecto.clone(),
        ruta_original: ruta_original.map(PathBuf::from),
        momento: Utc::now(),
    };

    let json = serde_json::to_string(&contenido)
        .map_err(|e| AppError::formato("al serializar la copia de recuperación", e))?;

    // Atómica: la copia de recuperación se reescribe cada dos minutos, así que es la
    // escritura con más ocasiones de caer dentro de la ventana en que el archivo está
    // truncado. Y es, por definición, lo último que le queda al usuario si algo va mal.
    crate::storage::escribir_de_forma_atomica(&destino, json.as_bytes())?;

    Ok(())
}

/// Estado de la copia de recuperación encontrado al evaluar el disco en el arranque.
#[derive(Debug)]
pub enum EstadoRecuperacion {
    /// No existe ningún archivo de recuperación en la ubicación esperada.
    NoExiste,
    /// Se encontró una copia íntegra y estructuralmente válida para ofrecer al usuario.
    Valida(Box<Recuperacion>),
    /// El archivo de recuperación existe pero no se pudo leer o su formato JSON es ilegible.
    Danada(AppError),
    /// El archivo de recuperación se pudo deserializar, pero no describe un árbol válido.
    EstructuraInvalida(AppError),
}

/// Evalúa el estado de la copia de recuperación **de la carpeta que se le indique**.
///
/// # Parámetros
/// - `carpeta_datos`: carpeta de datos donde vive la copia.
///
/// # Devuelve
/// Un [`EstadoRecuperacion`] con el diagnóstico exacto.
pub fn evaluar_recuperacion_en(carpeta_datos: &Path) -> EstadoRecuperacion {
    let ruta = ruta_recuperacion_en(carpeta_datos);

    let contenido = match std::fs::read_to_string(&ruta) {
        Ok(c) => c,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return EstadoRecuperacion::NoExiste;
        }
        Err(error) => return EstadoRecuperacion::Danada(AppError::lectura(&ruta, error)),
    };

    let recuperacion: Recuperacion = match serde_json::from_str(&contenido) {
        Ok(r) => r,
        Err(error) => {
            return EstadoRecuperacion::Danada(AppError::formato(
                "al leer la copia de recuperación",
                error,
            ));
        }
    };

    // Una copia que no describa un árbol válido no sirve: restaurarla reintroduciría el
    // problema que el resto del programa se esfuerza en rechazar.
    if let Err(e) = recuperacion.proyecto.validar_estructura() {
        return EstadoRecuperacion::EstructuraInvalida(e);
    }

    EstadoRecuperacion::Valida(Box::new(recuperacion))
}

/// Busca una copia de recuperación **en la carpeta que se le indique**.
///
/// # Parámetros
/// - `carpeta_datos`: carpeta de datos donde vive la copia.
///
/// # Devuelve
/// La copia encontrada, `None` si no existe, o el error tipado que impide utilizarla.
pub fn buscar_recuperacion_en(carpeta_datos: &Path) -> AppResult<Option<Recuperacion>> {
    utilizable(evaluar_recuperacion_en(carpeta_datos))
}

/// Reduce un diagnóstico a la copia que se le puede ofrecer al usuario, si la hay.
fn utilizable(estado: EstadoRecuperacion) -> AppResult<Option<Recuperacion>> {
    match estado {
        EstadoRecuperacion::Valida(recuperacion) => Ok(Some(*recuperacion)),
        EstadoRecuperacion::NoExiste => Ok(None),
        EstadoRecuperacion::Danada(error) | EstadoRecuperacion::EstructuraInvalida(error) => {
            Err(error)
        }
    }
}

/// Elimina la copia de recuperación **de la carpeta que se le indique**.
///
/// La ausencia del archivo ya equivale a haberlo descartado. Los demás fallos se devuelven para
/// que la interfaz pueda registrarlos sin convertirlos en un simple booleano.
///
/// # Parámetros
/// - `carpeta_datos`: carpeta de datos donde vive la copia.
pub fn descartar_recuperacion_en(carpeta_datos: &Path) -> AppResult<()> {
    let ruta = ruta_recuperacion_en(carpeta_datos);
    match std::fs::remove_file(&ruta) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(AppError::escritura(&ruta, error)),
    }
}

/// Controla cuándo toca autoguardar y evita escribir si nada ha cambiado.
///
/// Conserva cuándo fue la última comprobación y la revisión que ya está a salvo. Comparar esa
/// revisión es constante aunque el mapa tenga miles de nodos.
pub struct ControlAutoguardado {
    /// Momento de la última comprobación.
    ultima_comprobacion: Instant,

    /// Revisión que ya está a salvo; permite responder sin recorrer el mapa.
    revision_copiada: Option<RevisionProyecto>,

    /// Si el usuario ha desactivado el autoguardado desde la interfaz.
    pub activo: bool,

    /// Cuánto se espera entre dos comprobaciones.
    ///
    /// Sale de las preferencias del usuario. Arranca en el valor de fábrica para que un
    /// control recién creado se comporte como siempre lo ha hecho, aunque nadie le haya
    /// dicho todavía qué prefiere quien lo usa.
    pub(crate) intervalo: Duration,
}

impl Default for ControlAutoguardado {
    /// El mismo control que devuelve [`ControlAutoguardado::nuevo`].
    ///
    /// Existe para que la estructura se pueda construir con `..Default::default()` sin
    /// que nadie tenga que recordar cuál de los dos constructores es el bueno: hay uno.
    fn default() -> Self {
        Self::nuevo()
    }
}

impl ControlAutoguardado {
    /// Crea el control con el temporizador puesto en marcha y el autoguardado activo.
    pub fn nuevo() -> Self {
        Self {
            ultima_comprobacion: Instant::now(),
            revision_copiada: None,
            activo: true,
            intervalo: INTERVALO_AUTOGUARDADO,
        }
    }

    /// Indica si ha pasado el intervalo y toca comprobar si hay que autoguardar.
    ///
    /// Se consulta en cada fotograma, así que debe ser barato: solo compara instantes.
    pub fn toca_comprobar(&self) -> bool {
        self.debe_comprobar_tras(self.ultima_comprobacion.elapsed())
    }

    /// La decisión de [`Self::toca_comprobar`], separada del reloj.
    ///
    /// Va aparte para poder comprobarse sin esperar minutos de verdad. Sin esto, que el
    /// temporizador respetara el intervalo **elegido** —y no la constante de fábrica— no lo
    /// vigilaba nada: la única forma de notarlo habría sido esperar diez minutos delante del
    /// programa, que es exactamente lo que no hace ninguna batería de pruebas.
    ///
    /// # Parámetros
    /// - `transcurrido`: cuánto ha pasado desde la última comprobación.
    ///
    /// # Devuelve
    /// `true` si el autoguardado está encendido y ya ha pasado el intervalo elegido.
    pub(crate) fn debe_comprobar_tras(&self, transcurrido: Duration) -> bool {
        self.activo && transcurrido >= self.intervalo
    }

    /// Cuánto se espera hoy entre dos comprobaciones.
    ///
    /// Lo usa la interfaz para pedirle a `egui` que vuelva a pintar dentro de ese plazo: sin
    /// esa petición, una ventana inactiva deja de recibir fotogramas y el autoguardado no se
    /// dispara, que es justo el caso en que más falta hace.
    pub fn intervalo(&self) -> Duration {
        self.intervalo
    }

    /// Cambia cada cuántos minutos se autoguarda.
    ///
    /// # Parámetros
    /// - `minutos`: uno de [`MINUTOS_DE_AUTOGUARDADO_OFRECIDOS`]. **Cero desactiva** el
    ///   autoguardado en lugar de fijar un intervalo nulo, que dejaría el programa
    ///   escribiendo el mapa en cada fotograma.
    ///
    /// Un valor que no esté entre los ofrecidos se ignora y no cambia nada: llega de las
    /// preferencias, que son un archivo de texto que cualquiera puede editar a mano.
    pub fn fijar_minutos(&mut self, minutos: u64) {
        if !MINUTOS_DE_AUTOGUARDADO_OFRECIDOS.contains(&minutos) {
            return;
        }

        if minutos == 0 {
            self.activo = false;
            return;
        }

        self.activo = true;
        self.intervalo = Duration::from_secs(minutos * 60);
    }

    /// Cada cuántos minutos se autoguarda, o cero si está desactivado.
    ///
    /// Es lo que marca el desplegable de la interfaz, así que tiene que contar lo mismo que
    /// hace el programa: con el autoguardado apagado, el intervalo que quede guardado no
    /// describe nada.
    pub fn minutos(&self) -> u64 {
        if !self.activo {
            return 0;
        }
        self.intervalo.as_secs() / 60
    }

    /// Autoguarda el proyecto si ha cambiado desde la última copia.
    ///
    /// Reinicia el temporizador tanto si escribe como si no, de modo que un proyecto sin
    /// cambios no se comprueba más de una vez por intervalo.
    ///
    /// # Parámetros
    /// - `carpeta_datos`: carpeta de datos donde vive la copia de recuperación.
    /// - `proyecto`: proyecto en curso.
    /// - `ruta_original`: archivo del usuario al que corresponde, si lo hay.
    ///
    /// # Devuelve
    /// `Ok(true)` si se escribió una copia nueva, `Ok(false)` si no hacía falta.
    ///
    /// # Errores
    /// Propaga los de [`escribir_recuperacion_en`].
    pub fn autoguardar_si_cambio(
        &mut self,
        carpeta_datos: &Path,
        proyecto: &Proyecto,
        ruta_original: Option<&Path>,
    ) -> AppResult<bool> {
        self.ultima_comprobacion = Instant::now();

        if self.revision_copiada == Some(proyecto.revision()) {
            return Ok(false);
        }

        escribir_recuperacion_en(carpeta_datos, proyecto, ruta_original)?;
        self.revision_copiada = Some(proyecto.revision());

        Ok(true)
    }

    /// Da por copiado el estado actual sin escribir nada.
    ///
    /// Sirve para dos situaciones distintas, y por eso el nombre habla de la copia y no del
    /// guardado:
    ///
    /// - Tras un guardado explícito: el trabajo ya está a salvo en el archivo del usuario,
    ///   así que la próxima comprobación no debe considerarlo pendiente.
    /// - Tras sustituir el mapa por uno importado: la copia de recuperación acaba de
    ///   escribirse **con el mapa anterior**, a propósito, para poder volver atrás. Si no se
    ///   diera por copiado el estado actual, la siguiente comprobación vería un mapa distinto
    ///   del que hay en disco y pisaría esa copia con el mapa importado, dejando al usuario
    ///   sin la red de seguridad que el propio programa le acababa de prometer.
    ///
    /// # Parámetros
    /// - `proyecto`: el mapa que pasa a estar en pantalla, del que se toma la revisión.
    pub fn dar_por_copiado(&mut self, proyecto: &Proyecto) {
        self.revision_copiada = Some(proyecto.revision());
        self.ultima_comprobacion = Instant::now();
    }

    /// Comprueba si el proyecto actual tiene cambios respecto a la última huella copiada/guardada.
    pub fn ha_cambiado_desde_la_copia(&self, proyecto: &Proyecto) -> bool {
        self.revision_copiada != Some(proyecto.revision())
    }
}

/// Calcula una huella del contenido relevante del proyecto.
///
/// Solo entra lo que el usuario percibe como su trabajo: título, metadatos y el contenido
/// de cada nodo y conexión.
///
/// **Queda fuera a propósito** la marca `updated_at`, que cambia sola: incluirla haría que la
/// huella difiriera siempre y el autoguardado escribiría en cada intervalo aunque el usuario
/// no hubiera tocado nada.
///
/// Las **posiciones** entran solo en «Posición Libre Manual». En los dos modos automáticos
/// las reescribe la disposición cada vez que se recalcula, así que tenerlas en cuenta haría
/// que la huella cambiara sin que el usuario hubiera hecho nada. Pero en el modo manual son
/// justo al revés: colocar los nodos **es** el trabajo, y nada más lo refleja.
///
/// Antes quedaban fuera siempre, con lo que el modo manual se quedaba sin red: quien pasara
/// veinte minutos ordenando a mano un mapa recién escaneado, sin tocar ningún texto, no tenía
/// copia de recuperación ninguna si el programa se cerraba de golpe. La justificación decía
/// además que las posiciones «cambian con solo arrastrar la vista», y eso no es cierto:
/// desplazar la vista mueve la cámara, no los nodos.
///
/// # Parámetros
/// - `proyecto`: proyecto del que se calcula la huella.
///
/// # Devuelve
/// Un número que cambia cuando cambia el contenido significativo.
pub fn calcular_huella(proyecto: &Proyecto) -> u64 {
    let mut hasher = DefaultHasher::new();

    proyecto.title.hash(&mut hasher);
    proyecto.creator_vision.hash(&mut hasher);
    proyecto.project_goals.hash(&mut hasher);
    proyecto.target_audience_or_context.hash(&mut hasher);
    proyecto.author.hash(&mut hasher);
    proyecto.nodes.len().hash(&mut hasher);
    proyecto.connections.len().hash(&mut hasher);
    // La disposición elegida también es trabajo del usuario: sin esto, pasar de árbol
    // horizontal a radial no disparaba el guardado y la elección se perdía en un cierre
    // inesperado.
    proyecto.layout_mode.hash(&mut hasher);

    // En el modo manual, dónde está cada nodo es el trabajo del usuario. En los automáticos
    // lo decide el programa, así que incluirlo dispararía copias que nadie ha pedido.
    let las_posiciones_son_del_usuario = proyecto.layout_mode == ModoDisposicion::FreeDrag;

    // Los nodos se recorren ordenados por identificador, o la huella variaría sin que
    // cambiara nada. El orden lo da la propia estructura: `Proyecto::nodes` es un mapa
    // ordenado por identificador y sus claves salen así.
    //
    // Aquí había un `sort()` sobre esas claves, de cuando era una tabla dispersa. No hacía
    // nada, y su comentario seguía nombrando el tipo antiguo. Lo que sostiene la promesa
    // ahora es el tipo, y lo vigila `las_claves_del_mapa_de_nodos_salen_ordenadas`.
    for id in proyecto.nodes.keys() {
        let Some(nodo) = proyecto.nodes.get(id) else {
            continue;
        };

        id.hash(&mut hasher);
        nodo.title.hash(&mut hasher);
        nodo.notes.hash(&mut hasher);
        nodo.file_path.hash(&mut hasher);
        nodo.tags.hash(&mut hasher);
        nodo.status.nombre_canonico().hash(&mut hasher);
        nodo.priority.nombre_canonico().hash(&mut hasher);
        nodo.role.nombre_canonico().hash(&mut hasher);
        nodo.review_status.nombre_canonico().hash(&mut hasher);
        nodo.correction_feedback.hash(&mut hasher);
        nodo.children.hash(&mut hasher);
        nodo.collapsed.hash(&mut hasher);

        if las_posiciones_son_del_usuario {
            // `f32` no se puede usar como clave de tabla —no admite comparación total—, así
            // que se toma su representación en bits, que sí es exacta.
            nodo.pos[0].to_bits().hash(&mut hasher);
            nodo.pos[1].to_bits().hash(&mut hasher);
        }
    }

    for conexion in &proyecto.connections {
        conexion.from.hash(&mut hasher);
        conexion.to.hash(&mut hasher);
        conexion.label.hash(&mut hasher);
        conexion.relation_type.nombre_canonico().hash(&mut hasher);
    }

    hasher.finish()
}
