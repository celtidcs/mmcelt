//! # Módulo de Identificación de la Compilación (`version.rs`)
//!
//! Expone qué versión, de qué fecha y de qué commit es el ejecutable que se está
//! ejecutando.
//!
//! ## Por qué no basta el número de versión
//!
//! El número de versión no cambia entre compilaciones. Dice qué
//! versión *pretende* ser el programa, no **qué binario concreto** se ha abierto.
//!
//! Y esa es justo la pregunta que surge cuando hay varios ejecutables por el disco: el de
//! `target/release`, el de una copia portable en una memoria USB, el de una carpeta de
//! trabajo aparte. Todos se llaman igual y todos declaran la misma versión.
//!
//! La fecha de compilación y el identificador del commit sí lo distinguen. Se capturan al
//! compilar en `build.rs` y se incrustan en el binario.
//!
//! ## Dónde se muestra
//!
//! | Sitio | Qué se ve |
//! |---|---|
//! | Barra de estado | Versión y fecha, discreto, siempre visible |
//! | Ventana «Acerca de» | Todo el detalle, con un botón para copiarlo |
//! | `mmcelt --version` | Todo el detalle, en texto |
//!
//! El último es útil para comprobar un ejecutable **sin abrirlo**, que es lo que hace
//! falta cuando uno duda de cuál de las copias que tiene es la buena.

/// Versión declarada en `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nombre del programa.
pub const NOMBRE: &str = "MMCelt";

/// Fecha y hora UTC en que se compiló este ejecutable.
pub const FECHA_COMPILACION: &str = env!("MMCELT_FECHA_COMPILACION");

/// Identificador corto del commit desde el que se compiló.
///
/// Vale `desconocido` si se compiló sin git disponible o fuera de un repositorio.
pub const COMMIT: &str = env!("MMCELT_COMMIT");

/// Rama de git desde la que se compiló.
pub const RAMA: &str = env!("MMCELT_RAMA");

/// Si había cambios sin commitear en el momento de compilar.
///
/// Cuando vale `sí`, el binario **no corresponde exactamente a ningún commit**: contiene
/// además modificaciones que en ese momento solo estaban en el disco.
pub const ARBOL_SUCIO: &str = env!("MMCELT_ARBOL_SUCIO");

/// Comprueba si el binario fue compilado con cambios locales sin confirmar en git.
pub fn es_arbol_sucio() -> bool {
    ARBOL_SUCIO == "sí"
}

/// Devuelve una línea corta para la barra de estado.
///
/// # Devuelve
/// Un texto del estilo `v0.3.0 · 2026-08-23 09:48 UTC`.
pub fn resumen_corto() -> String {
    format!("v{VERSION} · {FECHA_COMPILACION}")
}

/// Devuelve la identificación completa de la compilación, en varias líneas.
///
/// Es lo que se muestra en la ventana «Acerca de» y lo que imprime `--version`. Incluye
/// todo lo necesario para saber si un ejecutable es el que uno cree.
pub fn detalle_completo() -> String {
    let mut texto = format!(
        "{NOMBRE} v{VERSION}\n\
         Compilado: {FECHA_COMPILACION}\n\
         Commit:    {COMMIT}\n\
         Rama:      {RAMA}"
    );

    if ARBOL_SUCIO == "sí" {
        texto.push_str(
            "\n\nAVISO: se compiló con cambios sin guardar en el repositorio, así que \
             este ejecutable no corresponde exactamente al commit indicado.",
        );
    }

    if let Ok(ruta) = std::env::current_exe() {
        texto.push_str(&format!("\n\nEjecutable: {}", ruta.display()));
    }

    texto
}

/// Imprime la identificación de la compilación por la salida estándar.
///
/// Responde a `mmcelt --version`. Permite comprobar qué es un ejecutable **sin abrirlo**,
/// que es lo práctico cuando hay varias copias y se duda de cuál es la actual.
pub fn imprimir_version() {
    println!("{}", detalle_completo());
}
