//! # Script de Compilación (`build.rs`)
//!
//! Captura, en el momento de compilar, los datos que identifican **esta compilación
//! concreta** y los deja disponibles para el programa como variables de entorno.
//!
//! ## Para qué sirve
//!
//! El número de versión de `Cargo.toml` no distingue una compilación de otra: sigue
//! siendo el mismo tras cambiar medio proyecto. Eso deja una duda incómoda cuando hay más
//! de un ejecutable por el disco —uno en `target/release`, otro en una copia portable,
//! otro en un worktree— y no se sabe cuál se está abriendo.
//!
//! Con la fecha de compilación y el identificador del commit, esa duda se resuelve de un
//! vistazo desde la propia aplicación.
//!
//! ## Qué se expone
//!
//! | Variable | Contenido |
//! |---|---|
//! | `MMCELT_FECHA_COMPILACION` | Fecha y hora UTC en que se compiló |
//! | `MMCELT_COMMIT` | Identificador corto del commit, o `desconocido` |
//! | `MMCELT_RAMA` | Rama de git, o `desconocida` |
//! | `MMCELT_ARBOL_SUCIO` | `sí` si había cambios sin commitear al compilar |
//!
//! ## El icono del ejecutable
//!
//! Al compilar en Windows, este script incrusta además `assets/icono/mmcelt.ico` como
//! recurso del ejecutable. Es lo que hace que el icono aparezca en el explorador de
//! archivos y en los accesos directos; el de la ventana y la barra de tareas lo pone la
//! propia aplicación en `main.rs`.
//!
//! ## Degradación
//!
//! Tanto los datos de git como el icono son **opcionales**. Si `git` no está instalado, o
//! el código se compila desde un archivo comprimido sin historial, el script no falla:
//! rellena los valores con «desconocido» y la compilación sigue adelante. Si el icono no
//! puede incrustarse, avisa y continúa. Sería absurdo impedir compilar por no poder
//! etiquetar la compilación o por no poder ponerle un dibujo al archivo.

use std::process::Command;

fn main() {
    // Cuidado con estas directivas: dicen **cuándo** se reejecuta este script, y por
    // omisión Cargo lo reejecuta al cambiar cualquier archivo del paquete. En cuanto se
    // emite la primera, el disparo queda restringido a lo que se enumere.
    //
    // Aquí había solo `build.rs`, con un comentario que decía justo lo contrario: que sin
    // ella la fecha se congelaría. Lo que la congelaba era ella. Comprobado tocando un
    // archivo de `src/`: Cargo recompilaba el binario, este script no se reejecutaba, y el
    // ejecutable seguía declarando la fecha de la compilación anterior. Con
    // `MMCELT_ARBOL_SUCIO` era peor: podía afirmar que el árbol estaba limpio cuando ya no
    // lo estaba, que es exactamente lo contrario de lo que `--version` promete.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    // El resto de lo que git rastrea. Sin esto, un cambio sin confirmar en la documentación
    // no reejecutaba este guion, y el binario seguía declarando que correspondía exactamente
    // a un commit cuando ya no era verdad.
    //
    // **Solo se vigila lo que git rastrea.** Aquí llegaron a estar también rutas ignoradas,
    // `CLAUDE.md`, `GEMINI.md` y `AGENTS.md`, y las cuatro están en `.gitignore`. Vigilarlas
    // no detecta nada —un archivo ignorado no puede ensuciar el árbol— y a cambio recompila
    // el crate entero cada vez que se tocan. Alguna de ellas se reescribe a cada
    // sesión, así que era una recompilación completa por cada nota que se apunta.
    // Comprobado ejecutándolo antes de quitarlo.
    println!("cargo:rerun-if-changed=documentacion");
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=skills");
    println!("cargo:rerun-if-changed=README.md");

    // `.git/HEAD` no basta: en una rama normal contiene `ref: refs/heads/<rama>` y **no
    // cambia al hacer un commit**. Lo que cambia es el archivo de la rama. Vigilando solo
    // HEAD, el binario seguía declarando el commit de la compilación anterior, que es
    // justo lo contrario de lo que esta funcionalidad pretende.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
    if let Some(ref_rama) = archivo_de_la_rama_actual() {
        println!("cargo:rerun-if-changed={ref_rama}");
    }

    let fecha = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    println!("cargo:rustc-env=MMCELT_FECHA_COMPILACION={fecha}");

    let commit =
        ejecutar_git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "desconocido".into());
    println!("cargo:rustc-env=MMCELT_COMMIT={commit}");

    let rama = ejecutar_git(&["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|| "desconocida".into());
    println!("cargo:rustc-env=MMCELT_RAMA={rama}");

    // Un árbol con cambios sin commitear significa que el binario no corresponde
    // exactamente a ningún commit, y conviene que se note.
    let sucio = match ejecutar_git(&["status", "--porcelain"]) {
        Some(salida) if !salida.trim().is_empty() => "sí",
        Some(_) => "no",
        None => "desconocido",
    };
    println!("cargo:rustc-env=MMCELT_ARBOL_SUCIO={sucio}");

    incrustar_icono_del_ejecutable();
}

/// Incrusta el icono como recurso del ejecutable de Windows.
///
/// Sin esto, el archivo `.exe` se ve en el explorador con el icono blanco genérico, que es
/// precisamente donde más se nota que un programa no tiene identidad propia.
///
/// No aborta la compilación si falla: un ejecutable sin icono sigue siendo un ejecutable
/// perfectamente utilizable.
#[cfg(windows)]
fn incrustar_icono_del_ejecutable() {
    const ICONO: &str = "assets/icono/mmcelt.ico";
    println!("cargo:rerun-if-changed={ICONO}");

    let mut recursos = winresource::WindowsResource::new();
    recursos.set_icon(ICONO);

    if let Err(error) = recursos.compile() {
        println!("cargo:warning=No se pudo incrustar el icono del ejecutable: {error}");
    }
}

/// Fuera de Windows no hay recurso que incrustar: el icono de la ventana lo pone `main.rs`.
#[cfg(not(windows))]
fn incrustar_icono_del_ejecutable() {}

/// Ejecuta un comando de git y devuelve su salida ya recortada.
///
/// # Devuelve
/// La salida del comando, o `None` si git no está disponible, si el directorio no es un
/// repositorio, o si el comando falla por cualquier motivo.
fn ejecutar_git(argumentos: &[&str]) -> Option<String> {
    let salida = Command::new("git").args(argumentos).output().ok()?;

    if !salida.status.success() {
        return None;
    }

    let texto = String::from_utf8(salida.stdout).ok()?;
    let texto = texto.trim().to_string();

    if texto.is_empty() {
        None
    } else {
        Some(texto)
    }
}

/// Devuelve la ruta del archivo que guarda el commit de la rama activa.
///
/// `.git/HEAD` contiene, en una rama normal, una línea `ref: refs/heads/<rama>`. El
/// identificador del commit vive en el archivo al que apunta esa referencia, y es ese el
/// que cambia con cada commit.
///
/// # Devuelve
/// La ruta del archivo de la rama, o `None` si no se puede leer `HEAD`, si el repositorio
/// está en estado «HEAD desacoplado» (donde `HEAD` ya contiene el commit directamente), o
/// si el archivo referenciado no existe todavía.
fn archivo_de_la_rama_actual() -> Option<String> {
    let head = std::fs::read_to_string(".git/HEAD").ok()?;
    let referencia = head.trim().strip_prefix("ref: ")?;

    let ruta = format!(".git/{referencia}");
    if std::path::Path::new(&ruta).exists() {
        Some(ruta)
    } else {
        None
    }
}
