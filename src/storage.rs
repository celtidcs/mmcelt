//! # Módulo de Persistencia (`storage.rs`)
//!
//! Gestiona el guardado, la carga y la exportación de mapas mentales en disco,
//! serializando en JSON.
//!
//! ## La carga es una frontera de confianza
//!
//! Un archivo `.mmcelt` puede haber sido escrito a mano, haberse corrompido, haber
//! llegado por correo o por una carpeta sincronizada en la nube, o haberlo generado un
//! modelo de IA a través del servidor MCP. **Nada garantiza que describa un árbol
//! válido.**
//!
//! El resto del programa sí lo da por supuesto: los recorridos de `layout` y de
//! `model` asumen que no hay ciclos, que la raíz existe y que ningún hijo apunta al
//! vacío. Antes de que existiera esta validación, un archivo que incumpliera esas
//! suposiciones congelaba la aplicación o agotaba su pila, y con `panic = "abort"` en
//! el perfil de publicación eso significaba cerrarse de golpe perdiendo el trabajo del
//! usuario.
//!
//! Por eso [`cargar_proyecto_de_archivo`] llama siempre a
//! [`crate::model::Proyecto::validar_estructura`] **antes** de devolver el proyecto, y
//! rechaza el archivo si no la supera. Los diálogos nativos del sistema operativo
//! viven aparte, en [`crate::ui::dialogos`].

use crate::ai_export::exportar_markdown_para_ia;
use crate::error::{AppError, AppResult};
use crate::model::Proyecto;
use std::fs;
use std::io::Read;
use std::path::Path;

/// Tamaño máximo de un mapa que se carga y deserializa: 256 MiB.
pub(crate) const BYTES_MAXIMOS_MAPA: u64 = 256 * 1024 * 1024;

/// El sistema de archivos del que se sirve la capa de persistencia.
///
/// ## Por qué existe
///
/// Sin esto, `storage` —que es la capa que decide **qué** se guarda y **cuándo** un archivo
/// es válido— dependía directamente de `std::fs`, es decir, de un detalle de bajo nivel. La
/// regla dice lo contrario: que los dos dependan de una abstracción. Esta es la abstracción, y
/// [`DiscoLocal`] es el detalle, ya intercambiable.
///
/// ## Qué gana el proyecto con ella
///
/// No es adorno: la escritura atómica y la carga con validación se pueden ejercitar ahora
/// **sin tocar el disco**, poniendo otra implementación. Antes, cualquier prueba de esa
/// lógica tenía que crear archivos de verdad en una carpeta temporal, y eso obliga a
/// serializar las pruebas y deja basura si una falla a medias.
pub trait SistemaDeArchivos {
    /// Lee un archivo entero.
    ///
    /// # Parámetros
    /// - `ruta`: el archivo a leer.
    ///
    /// # Errores
    /// [`AppError::Lectura`] si no existe o no se puede leer.
    fn leer(&self, ruta: &Path) -> AppResult<Vec<u8>>;

    /// Lee como máximo `limite + 1` bytes y rechaza el archivo si los supera.
    ///
    /// Las implementaciones de prueba pueden apoyarse en [`SistemaDeArchivos::leer`]. El
    /// disco real sobrescribe este método para no cargar primero el archivo completo.
    fn leer_acotado(&self, ruta: &Path, limite: u64) -> AppResult<Vec<u8>> {
        let bytes = self.leer(ruta)?;
        if bytes.len() as u64 > limite {
            return Err(AppError::EntradaDemasiadoGrande {
                origen: ruta.display().to_string(),
                limite,
            });
        }
        Ok(bytes)
    }

    /// Escribe un archivo de forma que un corte no pueda dejarlo a medias.
    ///
    /// # Parámetros
    /// - `ruta`: el archivo de destino. Se sobrescribe si ya existe.
    /// - `contenido`: los bytes a escribir.
    ///
    /// # Errores
    /// [`AppError::Escritura`] si la escritura no se puede completar.
    fn escribir_de_forma_atomica(&self, ruta: &Path, contenido: &[u8]) -> AppResult<()>;

    /// Crea una carpeta y las que le falten por encima.
    ///
    /// # Parámetros
    /// - `ruta`: la carpeta a crear. Si ya existe, no hace nada y no es error.
    ///
    /// # Errores
    /// [`AppError::Escritura`] si no se puede crear.
    fn crear_carpeta(&self, ruta: &Path) -> AppResult<()>;
}

/// El sistema de archivos de verdad: el del sistema operativo.
///
/// Es la única implementación que usa el programa en marcha. Las pruebas pueden poner otra.
pub struct DiscoLocal;

/// El sistema de archivos que se usa mientras nadie diga lo contrario.
pub const DISCO: DiscoLocal = DiscoLocal;

impl SistemaDeArchivos for DiscoLocal {
    /// Lee del disco de verdad. Ver [`SistemaDeArchivos::leer`].
    fn leer(&self, ruta: &Path) -> AppResult<Vec<u8>> {
        fs::read(ruta).map_err(|e| AppError::lectura(ruta, e))
    }

    fn leer_acotado(&self, ruta: &Path, limite: u64) -> AppResult<Vec<u8>> {
        let archivo = fs::File::open(ruta).map_err(|e| AppError::lectura(ruta, e))?;
        leer_hasta(archivo, limite, ruta)
    }

    /// Escribe en el disco de verdad, al lado y renombrando encima.
    /// Ver [`SistemaDeArchivos::escribir_de_forma_atomica`].
    fn escribir_de_forma_atomica(&self, ruta: &Path, contenido: &[u8]) -> AppResult<()> {
        escribir_de_forma_atomica_en_disco(ruta, contenido)
    }

    /// Crea la carpeta en el disco de verdad, con sus intermedias.
    /// Ver [`SistemaDeArchivos::crear_carpeta`].
    fn crear_carpeta(&self, ruta: &Path) -> AppResult<()> {
        fs::create_dir_all(ruta).map_err(|e| AppError::escritura(ruta, e))
    }
}

/// Lee un archivo entero del disco.
///
/// Existe para que la interfaz no tenga que llamar a `std::fs` por su cuenta: el acceso al
/// disco es de esta capa, y quien dibuja no tiene por qué saber cómo se lee un archivo.
///
/// # Parámetros
/// - `ruta`: el archivo a leer.
///
/// # Errores
/// [`AppError::Lectura`] si no existe o no se puede leer.
pub fn leer_bytes(ruta: &Path) -> AppResult<Vec<u8>> {
    DISCO.leer(ruta)
}

/// Crea una carpeta y las que le falten por encima.
///
/// # Parámetros
/// - `ruta`: la carpeta a crear. Si ya existe, no hace nada y no es error.
///
/// # Errores
/// [`AppError::Escritura`] si no se puede crear.
pub fn crear_carpeta(ruta: &Path) -> AppResult<()> {
    DISCO.crear_carpeta(ruta)
}

/// Calcula la huella del contenido de un archivo, para saber si ha cambiado por fuera.
///
/// # Parámetros
/// - `ruta`: el archivo del que se quiere la huella.
///
/// # Devuelve
/// `None` si el archivo no se puede leer, cosa normal mientras otro proceso lo está
/// escribiendo. Quien llama decide qué hacer con eso; aquí no es un error digno de contarle
/// al usuario.
pub fn huella_del_archivo(ruta: &Path) -> Option<u64> {
    DISCO
        .leer(ruta)
        .ok()
        .map(|bytes| crate::vigilante::calcular_huella_bytes(&bytes))
}

/// Escribe un archivo de forma que un corte no pueda dejarlo a medias.
///
/// # El problema
///
/// `fs::write` abre el archivo de destino, lo trunca a cero y va escribiendo. Entre esas dos
/// cosas hay una ventana —corta, pero real— en la que el archivo del usuario **existe y está
/// vacío**. Un corte de luz, un cierre forzado o un disco lleno justo ahí dejan el mapa
/// truncado, y lo que había antes ya no está en ningún sitio.
///
/// No es un escenario exótico en este programa: el autoguardado reescribe la copia de
/// recuperación cada dos minutos, y el servidor MCP reescribe el mapa cada vez que un agente
/// informa de su progreso. Cuantas más escrituras, más ocasiones de caer dentro de la ventana.
///
/// # Cómo se evita
///
/// Se escribe **al lado**, en un archivo temporal de la misma carpeta, se fuerza el volcado a
/// disco, y solo entonces se renombra encima del destino. El renombrado dentro del mismo
/// volumen es atómico: quien lea el archivo verá el contenido viejo entero o el nuevo entero,
/// nunca la mitad de uno.
///
/// El temporal va en la **misma carpeta** que el destino a propósito, y no en la de temporales
/// del sistema: renombrar entre volúmenes distintos no es atómico —y en Windows falla—, y la
/// carpeta del usuario puede estar en otra unidad.
///
/// # Lo que esto NO garantiza
///
/// Que el contenido nuevo sobreviva al corte. Si la luz se va antes del renombrado, se conserva
/// el archivo **anterior**, no el que se estaba guardando. Eso es justo lo que se busca: es
/// preferible perder el último guardado que perder el mapa entero.
///
/// # Parámetros
/// - `ruta`: el archivo de destino. Se sobrescribe si ya existe.
/// - `contenido`: los bytes a escribir.
///
/// # Errores
/// [`AppError::Escritura`] si no se puede crear el temporal, escribirlo, volcarlo o renombrarlo.
/// Si algo falla después de crear el temporal, se intenta borrarlo: un fallo al limpiarlo no
/// cambia el error que se devuelve, porque el que importa es el primero.
pub fn escribir_de_forma_atomica(ruta: &Path, contenido: &[u8]) -> AppResult<()> {
    DISCO.escribir_de_forma_atomica(ruta, contenido)
}

/// La escritura atómica de verdad, la que toca el disco.
///
/// Va aparte de [`escribir_de_forma_atomica`] para que aquella siga siendo la puerta que usa
/// el resto del programa y esta sea solo el detalle que implementa [`DiscoLocal`].
///
/// # Parámetros
/// - `ruta`: el archivo de destino. Se sobrescribe si ya existe.
/// - `contenido`: los bytes a escribir.
///
/// # Errores
/// [`AppError::Escritura`] si no se puede crear el temporal, escribirlo, volcarlo o renombrarlo.
fn escribir_de_forma_atomica_en_disco(ruta: &Path, contenido: &[u8]) -> AppResult<()> {
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SIGUIENTE_TEMPORAL: AtomicU64 = AtomicU64::new(0);

    // El temporal se llama como el destino más un sufijo, para que se vea a simple vista de
    // qué archivo salió si alguna vez queda huérfano tras un corte.
    let nombre = ruta
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "mmcelt".to_string());
    let (temporal, mut archivo) = loop {
        let secuencia = SIGUIENTE_TEMPORAL.fetch_add(1, Ordering::Relaxed);
        let temporal =
            ruta.with_file_name(format!(".{nombre}.tmp{}-{secuencia}", std::process::id()));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporal)
        {
            Ok(archivo) => break (temporal, archivo),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(AppError::escritura(&temporal, error)),
        }
    };

    // Bloque propio para que el archivo se cierre antes de renombrar: en Windows no se puede
    // renombrar encima de un destino mientras el origen siga abierto.
    let escritura = (|| -> std::io::Result<()> {
        archivo.write_all(contenido)?;
        // Sin esto, el renombrado puede completarse con los datos todavía en la caché del
        // sistema, y un corte inmediatamente después deja el destino apuntando a un archivo
        // vacío. Es la parte que hace que el cambio sirva de algo.
        archivo.sync_all()?;
        Ok(())
    })();

    if let Err(e) = escritura {
        let _ = fs::remove_file(&temporal);
        return Err(AppError::escritura(&temporal, e));
    }

    if let Err(e) = fs::rename(&temporal, ruta) {
        let _ = fs::remove_file(&temporal);
        return Err(AppError::escritura(ruta, e));
    }

    Ok(())
}

/// Guarda el proyecto en disco en formato `.mmcelt` (JSON estructurado e indentado).
///
/// # Parámetros
/// - `proyecto`: proyecto a guardar.
/// - `path`: ruta de destino. Se sobrescribe si ya existe.
///
/// # Errores
/// - [`AppError::Formato`] si el proyecto no se puede serializar.
/// - [`AppError::Escritura`] si el archivo no se puede escribir (permisos, carpeta
///   inexistente, disco lleno).
pub fn guardar_proyecto_en_archivo(proyecto: &Proyecto, path: &Path) -> AppResult<()> {
    guardar_proyecto_con(&DISCO, proyecto, path)
}

/// Guarda el proyecto en el sistema de archivos que se le indique.
///
/// Es la misma operación de [`guardar_proyecto_en_archivo`], pero sin dar por hecho que
/// detrás hay un disco: así se puede ejercitar sin tocarlo.
///
/// # Parámetros
/// - `archivos`: dónde se escribe.
/// - `proyecto`: proyecto a guardar.
/// - `path`: ruta de destino. Se sobrescribe si ya existe.
///
/// # Errores
/// - [`AppError::Formato`] si el proyecto no se puede serializar.
/// - [`AppError::Escritura`] si no se puede escribir.
pub fn guardar_proyecto_con(
    archivos: &dyn SistemaDeArchivos,
    proyecto: &Proyecto,
    path: &Path,
) -> AppResult<()> {
    let json_data = serde_json::to_string_pretty(proyecto)
        .map_err(|e| AppError::formato("al serializar el proyecto", e))?;

    archivos.escribir_de_forma_atomica(path, json_data.as_bytes())?;
    Ok(())
}

/// Carga y deserializa un proyecto desde un archivo `.mmcelt` o `.json`.
///
/// **Valida la estructura del mapa antes de devolverlo.** Un archivo que describa un
/// árbol imposible (con ciclos, con la raíz ausente, con referencias colgantes o con
/// una profundidad desmesurada) se rechaza aquí, que es donde el usuario puede
/// entenderlo, en lugar de provocar un cuelgue o un cierre abrupto más adelante.
///
/// # Parámetros
/// - `path`: ruta del archivo a cargar.
///
/// # Devuelve
/// El proyecto cargado, garantizado estructuralmente válido.
///
/// # Errores
/// - [`AppError::Lectura`] si el archivo no se puede leer.
/// - [`AppError::Formato`] si el contenido no es JSON válido o no encaja con el
///   formato de MMCelt.
/// - [`AppError::EstructuraInvalida`] si el JSON es correcto pero el mapa que describe
///   no es un árbol válido.
pub fn cargar_proyecto_de_archivo(path: &Path) -> AppResult<Proyecto> {
    cargar_proyecto_con(&DISCO, path)
}

/// Carga el proyecto del sistema de archivos que se le indique, validándolo igual.
///
/// # Parámetros
/// - `archivos`: de dónde se lee.
/// - `path`: ruta del archivo a cargar.
///
/// # Devuelve
/// El proyecto cargado, garantizado estructuralmente válido.
///
/// # Errores
/// - [`AppError::Lectura`] si no se puede leer, o si lo leído no es texto válido.
/// - [`AppError::Formato`] si no es JSON de MMCelt.
/// - [`AppError::EstructuraInvalida`] si el mapa que describe no es un árbol válido.
pub fn cargar_proyecto_con(archivos: &dyn SistemaDeArchivos, path: &Path) -> AppResult<Proyecto> {
    cargar_proyecto_con_limite(archivos, path, BYTES_MAXIMOS_MAPA)
}

/// Carga un proyecto imponiendo un límite antes de interpretar su contenido.
pub(crate) fn cargar_proyecto_con_limite(
    archivos: &dyn SistemaDeArchivos,
    path: &Path,
    limite: u64,
) -> AppResult<Proyecto> {
    let bytes = archivos.leer_acotado(path, limite)?;
    // Se conserva el error que daba `read_to_string`: un archivo que no es texto es un fallo
    // de lectura, no de formato del mapa.
    let contenido = String::from_utf8(bytes).map_err(|e| {
        AppError::lectura(
            path,
            std::io::Error::new(std::io::ErrorKind::InvalidData, e),
        )
    })?;

    let proyecto: Proyecto = serde_json::from_str(&contenido)
        .map_err(|e| AppError::formato("al interpretar el mapa mental", e))?;

    // Frontera de confianza: nada de lo que hay debajo tolera un árbol inválido.
    proyecto.validar_estructura()?;

    Ok(proyecto)
}

/// Lee un flujo sin reservar nunca más de un byte por encima del límite indicado.
fn leer_hasta<R: Read>(lector: R, limite: u64, origen: &Path) -> AppResult<Vec<u8>> {
    let mut bytes = Vec::with_capacity(limite.min(8 * 1024) as usize);
    lector
        .take(limite.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|e| AppError::lectura(origen, e))?;
    if bytes.len() as u64 > limite {
        return Err(AppError::EntradaDemasiadoGrande {
            origen: origen.display().to_string(),
            limite,
        });
    }
    Ok(bytes)
}

/// Exporta el mapa mental a un archivo Markdown enriquecido, optimizado para IA.
///
/// # Parámetros
/// - `proyecto`: proyecto a exportar.
/// - `path`: ruta del archivo `.md` de destino. Se sobrescribe si ya existe.
/// - `idioma`: la lengua en que se escribe el documento, la que el usuario tenga elegida.
///
/// # Errores
/// - [`AppError::Escritura`] si el archivo no se puede escribir.
pub fn exportar_proyecto_a_markdown(
    proyecto: &Proyecto,
    path: &Path,
    idioma: crate::textos::Idioma,
) -> AppResult<()> {
    let md_content = exportar_markdown_para_ia(proyecto, idioma);
    // Atómica igual que el mapa: el servidor MCP reescribe este archivo en cada informe de
    // progreso, y suele vivir dentro del repositorio del usuario.
    escribir_de_forma_atomica(path, md_content.as_bytes())?;
    Ok(())
}
