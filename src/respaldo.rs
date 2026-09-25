//! Copias de seguridad de un archivo antes de sobrescribirlo.
//!
//! Existe para que haya **una sola** forma de guardar lo que se va a pisar. Había dos, y
//! divergieron: la de la configuración de los agentes ponía marca de tiempo en el nombre, y
//! la del servidor MCP usaba siempre el mismo, `archivo.mmcelt.bak`.
//!
//! Esa diferencia no era cosmética. El servidor promete que no sobrescribe en silencio, pero
//! como el nombre era fijo y copiar sobrescribe sin avisar, bastaba con que un agente
//! escribiera dos veces sobre el mismo archivo para que la segunda copia machacara a la
//! primera: el trabajo del usuario quedaba fuera del disco, y lo que se conservaba era el
//! primer intento del propio agente.

use std::path::{Path, PathBuf};

/// Formato de la marca de tiempo que distingue una copia de otra.
///
/// Hasta el segundo: dos escrituras del mismo archivo dentro del mismo segundo son
/// improbables, y el desempate lo resuelve [`nombre_libre`] añadiendo un número.
const FORMATO_MARCA: &str = "%Y%m%d-%H%M%S";

/// Guarda una copia del archivo antes de que se sobrescriba.
///
/// La copia se llama como el original más la marca de tiempo y la extensión `.bak`, de modo
/// que las copias sucesivas conviven en vez de pisarse: `proyecto.mmcelt.20260823-101530.bak`.
///
/// # Parámetros
/// - `ruta`: el archivo que está a punto de sobrescribirse.
///
/// # Devuelve
/// La ruta de la copia creada, o `None` si el archivo no existía y no había nada que copiar.
///
/// # Errores
/// Devuelve el error del sistema si la copia no se puede escribir.
pub fn copiar_antes_de_sobrescribir(ruta: &Path) -> std::io::Result<Option<PathBuf>> {
    if !ruta.exists() {
        return Ok(None);
    }

    let destino = nombre_libre(ruta);
    std::fs::copy(ruta, &destino)?;
    Ok(Some(destino))
}

/// Compone un nombre de copia que no esté ya ocupado.
///
/// Parte del nombre con marca de tiempo y, si ese archivo ya existe —dos escrituras dentro
/// del mismo segundo—, va probando con un número al final hasta encontrar uno libre. Sin
/// esto, la copia de esas dos escrituras seguiría pisándose, que es justo lo que este módulo
/// existe para evitar.
///
/// # Parámetros
/// - `ruta`: el archivo original.
///
/// # Devuelve
/// Una ruta que no existe en el momento de comprobarlo.
fn nombre_libre(ruta: &Path) -> PathBuf {
    let extension = ruta.extension().and_then(|e| e.to_str()).unwrap_or("bak");
    let marca = chrono::Local::now().format(FORMATO_MARCA);

    let candidata = ruta.with_extension(format!("{extension}.{marca}.bak"));
    if !candidata.exists() {
        return candidata;
    }

    // El límite existe para no quedarse dando vueltas si algo impide crear archivos; a la
    // última se devuelve el nombre aunque esté ocupado, y el error lo dará la copia.
    const INTENTOS_MAXIMOS: u32 = 100;
    for intento in 2..=INTENTOS_MAXIMOS {
        let candidata = ruta.with_extension(format!("{extension}.{marca}-{intento}.bak"));
        if !candidata.exists() {
            return candidata;
        }
    }

    ruta.with_extension(format!("{extension}.{marca}-{INTENTOS_MAXIMOS}.bak"))
}
