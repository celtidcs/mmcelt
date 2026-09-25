//! # Módulo de Diálogos Nativos del Sistema (`ui/dialogos.rs`)
//!
//! Envuelve los diálogos de archivo nativos del sistema operativo (biblioteca `rfd`)
//! para abrir, guardar y exportar mapas mentales.
//!
//! ## Por qué está aquí y no en `storage.rs`
//!
//! Estas funciones **no persisten nada**: abren una ventana modal y preguntan al
//! usuario por una ruta. Eso es interacción con el usuario, no acceso a datos, y
//! mezclar ambas responsabilidades en un mismo módulo incumplía el principio de
//! responsabilidad única.
//!
//! La separación tiene además una consecuencia práctica: `storage.rs` queda como
//! lógica pura sobre rutas y contenidos, y por tanto se puede probar sin abrir
//! ninguna ventana.
//!
//! ## Advertencia sobre el bloqueo
//!
//! Todas las funciones de este módulo son **bloqueantes**: detienen el hilo que las
//! llama hasta que el usuario cierra el diálogo. Deben invocarse únicamente desde el
//! hilo de la interfaz y como reacción directa a una acción del usuario.

use std::path::PathBuf;

/// Filtro de archivos de mapa mental compartido por los diálogos de abrir y guardar.
const EXTENSIONES_MAPA: &[&str] = &["mmcelt", "json"];

// La descripción del filtro ya no es una constante: se traduce, y vive en `textos.rs` como
// `Texto::ModalMapaMentalMmceltMmcelt`. Las extensiones sí siguen aquí porque no son texto
// para leer, son la lista de extensiones que el sistema debe aceptar.

/// Muestra el diálogo nativo para elegir dónde guardar un mapa mental.
///
/// El diálogo puede abrirse en una carpeta concreta, lo que se usa para **proponer** la
/// del proyecto de código al que se refiere el mapa, deducida con
/// [`crate::model::Proyecto::carpeta_proyecto_sugerida`]. Un mapa que describe un proyecto
/// suele querer vivir junto a él, y sugerirlo ahorra navegar hasta allí.
///
/// Es una sugerencia, no una imposición: el diálogo se abre ahí y el usuario puede ir a
/// donde prefiera. **La aplicación nunca escribe sola en una carpeta deducida**, porque
/// eso podría colar archivos dentro de un repositorio ajeno sin que se diera cuenta.
///
/// # Parámetros
/// - `default_filename`: nombre propuesto en el cuadro de diálogo.
/// - `carpeta_sugerida`: carpeta en la que abrir el diálogo. Si es `None` o no existe, se
///   usa la que el sistema recuerde.
/// - `idioma`: el elegido por el usuario. El título del diálogo y el nombre del filtro de
///   archivos los pinta el sistema operativo, pero el texto lo pone el programa.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_para_guardar(
    default_filename: &str,
    carpeta_sugerida: Option<&std::path::Path>,
    idioma: crate::textos::Idioma,
) -> Option<PathBuf> {
    let mut dialogo = rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ModalGuardarMapaMental.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalMapaMentalMmceltMmcelt.en(idioma),
            EXTENSIONES_MAPA,
        )
        .set_file_name(default_filename);

    if let Some(carpeta) = carpeta_sugerida {
        if carpeta.is_dir() {
            dialogo = dialogo.set_directory(carpeta);
        }
    }

    dialogo.save_file()
}

/// Muestra el diálogo nativo para elegir un mapa mental que abrir.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_para_abrir(idioma: crate::textos::Idioma) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ModalAbrirMapaMental.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalMapaMentalMmceltMmcelt.en(idioma),
            EXTENSIONES_MAPA,
        )
        .pick_file()
}

/// Muestra el diálogo nativo para elegir dónde exportar el Markdown para IA.
///
/// # Parámetros
/// - `default_filename`: nombre propuesto en el cuadro de diálogo.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_para_exportar(
    default_filename: &str,
    idioma: crate::textos::Idioma,
) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ModalExportarMarkdownOptimizadoPara.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalArchivoMarkdownMd.en(idioma),
            &["md"],
        )
        .set_file_name(default_filename)
        .save_file()
}

/// Muestra el diálogo nativo para elegir dónde exportar el mapa a OPML.
///
/// # Parámetros
/// - `default_filename`: nombre propuesto en el cuadro de diálogo.
/// - `idioma`: el elegido por el usuario, para el título y el nombre del filtro.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_opml_para_exportar(
    default_filename: &str,
    idioma: crate::textos::Idioma,
) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ArchivoExportarOpml.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalOpmlEsquemaOpml.en(idioma),
            &["opml"],
        )
        .set_file_name(default_filename)
        .save_file()
}

/// Muestra el diálogo nativo para elegir un archivo OPML que importar.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_opml_para_importar(idioma: crate::textos::Idioma) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ArchivoImportarOpml.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalOpmlEsquemaOpml.en(idioma),
            &["opml"],
        )
        .pick_file()
}

/// Muestra el diálogo nativo para elegir dónde exportar el mapa a `.mm` de FreeMind/Freeplane.
///
/// # Parámetros
/// - `default_filename`: nombre propuesto en el cuadro de diálogo.
/// - `idioma`: el elegido por el usuario, para el título y el nombre del filtro.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_freemind_para_exportar(
    default_filename: &str,
    idioma: crate::textos::Idioma,
) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ArchivoExportarFreemind.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalFreemindEsquemaMm.en(idioma),
            &["mm"],
        )
        .set_file_name(default_filename)
        .save_file()
}

/// Muestra el diálogo nativo para elegir un archivo `.mm` que importar.
///
/// # Devuelve
/// La ruta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_archivo_freemind_para_importar(idioma: crate::textos::Idioma) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title(crate::textos::Texto::ArchivoImportarFreemind.en(idioma))
        .add_filter(
            crate::textos::Texto::ModalFreemindEsquemaMm.en(idioma),
            &["mm"],
        )
        .pick_file()
}

/// Muestra el diálogo nativo para elegir una carpeta de código que mapear.
///
/// # Devuelve
/// La carpeta elegida por el usuario, o `None` si canceló.
///
/// # Efectos secundarios
/// Abre una ventana modal y **bloquea el hilo actual** hasta que el usuario responde.
pub fn pedir_carpeta(titulo: &str) -> Option<PathBuf> {
    rfd::FileDialog::new().set_title(titulo).pick_folder()
}
