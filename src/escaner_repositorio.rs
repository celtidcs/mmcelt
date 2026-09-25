//! Adaptador que convierte la estructura acotada de una carpeta en un mapa mental.

use crate::error::AppResult;
use crate::model::{Proyecto, RolNodo};
use std::path::Path;
use uuid::Uuid;

/// Profundidad máxima a la que desciende el escaneo de un repositorio de código.
///
/// Tres niveles bastan para captar la arquitectura de un proyecto sin convertir el mapa
/// en un listado ilegible de cada archivo del disco.
pub const PROFUNDIDAD_ESCANEO: usize = 3;

/// Cuántos archivos de código se mapean como mucho por carpeta.
///
/// El escáner existe para llevar el mapa hasta el código, no para reproducir el árbol de
/// archivos entero: pasado cierto número, los nodos dejan de ayudar y estorban.
const ARCHIVOS_DE_CODIGO_POR_CARPETA: usize = 15;

/// Extensiones que el escáner considera código y mapea como nodos.
///
/// Están aquí, y no dentro del recorrido, porque añadir un lenguaje es lo que más a menudo se
/// va a querer cambiar de todo el escáner.
const EXTENSIONES_DE_CODIGO: &[&str] = &[
    "rs", "py", "js", "ts", "vue", "go", "c", "cpp", "java", "toml", "json", "md", "sql",
];

/// Construye un mapa a partir de una carpeta de código existente.
///
/// Esta es la puerta pública del caso de uso. El constructor asociado se conserva para que los
/// consumidores anteriores sigan compilando mientras migran a esta dependencia explícita.
pub fn escanear_carpeta(ruta: &Path, idioma: crate::textos::Idioma) -> AppResult<Proyecto> {
    Proyecto::desde_escaneo_de_carpeta(ruta, idioma)
}

/// Indica si un archivo debe mapearse como nodo de código.
///
/// # Parámetros
/// - `ruta`: la ruta del archivo encontrado al escanear.
///
/// # Devuelve
/// `true` si su extensión está entre las que el escáner reconoce, sin distinguir mayúsculas:
/// un `README.MD` cuenta igual que un `readme.md`.
fn es_archivo_de_codigo(ruta: &Path) -> bool {
    let Some(extension) = ruta.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    let extension = extension.to_lowercase();
    EXTENSIONES_DE_CODIGO.contains(&extension.as_str())
}

impl Proyecto {
    /// Construye un mapa a partir de una carpeta de código que ya existe en el disco.
    ///
    /// Es lo que hay detrás de «Escanear Carpeta de Código». Cada subcarpeta entra como nodo
    /// estructural y cada archivo reconocido como hoja, con su ruta anotada en `file_path`,
    /// de modo que el mapa queda enganchado al código real y una IA puede saber de qué
    /// archivo se le está hablando.
    ///
    /// # Lo que deja fuera, y por qué
    ///
    /// El recorrido está acotado a propósito por [`PROFUNDIDAD_ESCANEO`] niveles y a
    /// [`ARCHIVOS_DE_CODIGO_POR_CARPETA`] archivos por carpeta, y se salta las carpetas
    /// ocultas y las de compilación (`target`, `node_modules`, `dist`, `build`). Sin esos
    /// topes, un repositorio corriente daría un mapa de miles de nodos que no se puede leer,
    /// que es justo lo contrario de para lo que sirve un mapa mental.
    ///
    /// El coste de este recorrido lo vigila la prueba
    /// `las_operaciones_del_hilo_de_la_interfaz_siguen_costando_milisegundos`, porque ocurre
    /// en el mismo hilo que dibuja la ventana.
    ///
    /// # Parámetros
    /// - `root_path`: la carpeta que el usuario ha elegido. Da nombre a la raíz del mapa.
    /// - `idioma`: el del usuario, para los títulos que no salen del disco.
    ///
    /// # Devuelve
    /// El mapa ya construido y validado.
    ///
    /// # Errores
    /// [`crate::error::AppError::EstructuraInvalida`] si el árbol resultante no supera la
    /// validación. No debería ocurrir —lo construye esta misma función—, y por eso se
    /// comprueba: significaría que el escaneo tiene un defecto, y más vale decirlo que
    /// entregar un mapa que después congele la aplicación al dibujarlo.
    pub fn desde_escaneo_de_carpeta(
        root_path: &std::path::Path,
        idioma: crate::textos::Idioma,
    ) -> AppResult<Self> {
        use crate::textos::Texto;
        let dir_name = root_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| Texto::EscaneoRepositorioDefecto.en(idioma));

        let mut proyecto = Self::nuevo_vacio(
            Texto::EscaneoEstructuraDe
                .en(idioma)
                .replace("{}", dir_name),
        );
        proyecto.creator_vision = Texto::EscaneoVision
            .en(idioma)
            .replace("{}", &root_path.display().to_string());
        proyecto.project_goals = Texto::EscaneoObjetivos.en(idioma).to_string();

        let root_id = proyecto.root_id;
        if let Some(r) = proyecto.nodes.get_mut(&root_id) {
            r.title = format!("📂 {}", dir_name);
            r.file_path = Some(root_path.to_string_lossy().to_string());
        }

        escanear_carpeta_recursiva(&mut proyecto, root_id, root_path, 1, PROFUNDIDAD_ESCANEO);

        // Se valida antes de disponer los nodos: el escaneo lee del sistema de archivos,
        // que es una fuente externa, y un enlace simbólico circular podría producir una
        // estructura que los recorridos de `layout` no saben manejar.
        proyecto.validar_estructura()?;

        crate::layout::aplicar_disposicion_automatica(&mut proyecto);
        Ok(proyecto)
    }
}

/// Recorre una carpeta y añade sus subcarpetas y archivos de código al mapa.
///
/// El descenso se detiene al superar `max_depth`, lo que además acota el efecto de un
/// eventual enlace simbólico circular en el sistema de archivos.
///
/// Por cada carpeta se limita a los quince primeros archivos, para que el mapa siga
/// siendo legible en repositorios grandes.
///
/// # Parámetros
/// - `proyecto`: proyecto en construcción.
/// - `parent_id`: nodo bajo el que se cuelga el contenido de esta carpeta.
/// - `dir`: carpeta que se está recorriendo.
/// - `current_depth`: profundidad actual del recorrido.
/// - `max_depth`: profundidad máxima permitida.
fn escanear_carpeta_recursiva(
    proyecto: &mut Proyecto,
    parent_id: Uuid,
    dir: &std::path::Path,
    current_depth: usize,
    max_depth: usize,
) {
    if current_depth > max_depth {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut carpetas = Vec::new();
    let mut archivos = Vec::new();

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        // Las carpetas ocultas, las de control de versiones y las de compilación no
        // aportan nada al mapa: son ruido que taparía el código de verdad.
        if name.starts_with('.')
            || name == "target"
            || name == "node_modules"
            || name == "dist"
            || name == "build"
        {
            continue;
        }

        let path = entry.path();
        if path.is_dir() {
            carpetas.push((name, path));
        } else {
            archivos.push((name, path));
        }
    }

    carpetas.sort_by(|a, b| a.0.cmp(&b.0));
    archivos.sort_by(|a, b| a.0.cmp(&b.0));

    // Las carpetas entran como subtemas estructurales.
    for (name, path) in carpetas {
        let id_del_hijo = proyecto.anadir_hijo(parent_id, format!("📁 {}", name));
        if let Some(n) = proyecto.nodes.get_mut(&id_del_hijo) {
            n.file_path = Some(path.to_string_lossy().to_string());
            n.tags = vec!["directorio".into()];
        }
        escanear_carpeta_recursiva(proyecto, id_del_hijo, &path, current_depth + 1, max_depth);
    }

    // El recorte se aplica **después** de quedarse con los archivos de código, no antes.
    //
    // Al revés, que es como estaba, el límite se gastaba en lo que luego se iba a descartar:
    // una carpeta `assets` con veinte imágenes ordenadas alfabéticamente antes que
    // `index.ts` producía cero nodos de código, en silencio. Y la razón de ser del escáner es
    // justamente llevar el mapa hasta el código real.
    let archivos_de_codigo = archivos
        .into_iter()
        .filter(|(_, ruta)| es_archivo_de_codigo(ruta))
        .take(ARCHIVOS_DE_CODIGO_POR_CARPETA);

    for (name, path) in archivos_de_codigo {
        let id_del_hijo = proyecto.anadir_hijo(parent_id, format!("📄 {}", name));
        let Some(n) = proyecto.nodes.get_mut(&id_del_hijo) else {
            continue;
        };
        n.file_path = Some(path.to_string_lossy().to_string());
        n.role = RolNodo::RecursoHerramienta;

        // La extensión sirve de etiqueta, para poder filtrar el mapa por lenguaje.
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            n.tags = vec![extension.to_lowercase()];
        }
    }
}
