//! Adaptación de la salida textual del ejecutable gráfico en Windows.
//!
//! La compilación distribuida usa el subsistema gráfico para que abrir MMCelt desde el
//! Explorador no cree una consola vacía. El modo `--version`, sin embargo, sigue necesitando
//! escribir en la terminal desde la que se invoca. Este módulo reúne esa única diferencia de
//! plataforma y no interviene en el servidor MCP, cuyas tuberías estándar se conservan intactas.

/// Prepara la salida estándar antes de imprimir la versión en Windows.
///
/// Un ejecutable del subsistema gráfico arranca separado de la consola. Si su proceso padre tiene
/// una, `AttachConsole` lo asocia a ella y Windows repone los manejadores de consola que falten. Si
/// la salida ya era una tubería redirigida, Windows la conserva. El fallo es deliberadamente
/// silencioso: también es normal ejecutar el programa desde un padre sin consola.
#[cfg(windows)]
pub(crate) fn preparar_salida_de_version() {
    /// Valor reservado por Windows para pedir la consola del proceso padre.
    const ATTACH_PARENT_PROCESS: u32 = u32::MAX;

    #[link(name = "kernel32")]
    extern "system" {
        fn AttachConsole(process_id: u32) -> i32;
    }

    // SEGURIDAD: `AttachConsole` no recibe punteros ni conserva memoria de Rust. Se invoca con la
    // constante documentada por Windows y no se depende de su resultado: tanto «ya asociado» como
    // «el padre no tiene consola» son estados normales para este ejecutable.
    let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
}

/// Fuera de Windows no hay subsistemas de consola separados y no hace falta preparar nada.
#[cfg(not(windows))]
pub(crate) fn preparar_salida_de_version() {}
