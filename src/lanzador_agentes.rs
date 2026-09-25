//! Adaptadores seguros para convertir una sesión confirmada en una orden de consola.
//!
//! Las órdenes se representan como programa, argumentos y directorio independientes. Este módulo
//! no construye cadenas de shell y nunca coloca el prompt completo en la línea de comandos.

use crate::conectores::{Agente, IdAgente};
use crate::error::{AppError, AppResult, MotivoCarpetaInvalida};
use crate::proyecto_trabajo::ContextoProyecto;
use crate::sesiones_agentes::SesionPreparada;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Rasgos portables con los que MMCelt reconoce el mismo archivo antes de abrirlo.
///
/// No es una firma criptográfica. Su finalidad es detectar que el ejecutable visible en la
/// confirmación ha desaparecido o ha sido sustituido durante el recorrido de «Enviar a...».
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentidadArchivo {
    /// Tamaño del archivo medido en bytes.
    pub bytes: u64,
    /// Fecha de modificación proporcionada por el sistema de archivos.
    pub modificado: SystemTime,
}

/// Ejecutable concreto aceptado por el usuario para una sesión.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EjecutableConfirmado {
    /// Ruta absoluta y canónica que se muestra completa en la vista previa.
    pub ruta_canonica: PathBuf,
    /// Identidad que debe mantenerse hasta el instante del lanzamiento.
    pub identidad: IdentidadArchivo,
}

/// Programa y argumentos ya separados para abrir una consola oficial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrdenCli {
    /// Ejecutable localizado previamente por MMCelt.
    pub programa: PathBuf,
    /// Argumentos individuales que recibirá `Command`, sin intérprete de órdenes.
    pub argumentos: Vec<String>,
    /// Raíz canónica y autorizada que actuará como directorio de trabajo.
    pub directorio: PathBuf,
    /// Confirmación que el adaptador real vuelve a comprobar justo antes de crear el proceso.
    pub ejecutable_confirmado: EjecutableConfirmado,
}

/// Confirmación mínima devuelta por el sistema al crear el proceso de consola.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResultadoLanzamiento {
    /// Identificador del proceso recién creado.
    pub pid: u32,
}

/// Contrato inyectable que permite verificar el lanzamiento sin abrir ventanas en las pruebas.
pub trait EjecutorDeConsola {
    /// Abre la consola representada por una orden ya validada.
    fn abrir(&self, orden: &OrdenCli) -> AppResult<ResultadoLanzamiento>;
}

/// Ejecutor real que delega la creación de la ventana en el sistema operativo.
pub struct ConsolaDelSistema;

impl EjecutorDeConsola for ConsolaDelSistema {
    /// Abre la consola de verdad, delegando en el sistema operativo.
    ///
    /// Es la única implementación que llega a lanzar un proceso. Las pruebas usan otra que
    /// anota la orden sin ejecutarla, que es lo que permite comprobar **qué** se iba a
    /// lanzar sin abrir ventanas en el ordenador de quien ejecuta la batería.
    fn abrir(&self, orden: &OrdenCli) -> AppResult<ResultadoLanzamiento> {
        abrir_en_el_sistema(orden)
    }
}

/// Construye la orden neutral de un agente a partir del expediente ya persistido.
///
/// Solo se transmite una referencia relativa a `inicio.md`. El prompt permanece en el expediente,
/// fuera de la lista de procesos del sistema y de las reglas de escape de cada terminal.
pub fn orden_para(
    agente: IdAgente,
    ejecutable: EjecutableConfirmado,
    contexto: &ContextoProyecto,
    sesion: &SesionPreparada,
) -> AppResult<OrdenCli> {
    verificar_ejecutable(&ejecutable)?;
    let inicio = sesion
        .ruta_inicio
        .canonicalize()
        .map_err(|error| AppError::lectura(&sesion.ruta_inicio, error))?;
    if !contexto.contiene(&inicio)? || !inicio.is_file() {
        return Err(AppError::RutaFueraDelProyecto);
    }
    let relativa = inicio
        .strip_prefix(contexto.raiz())
        .map_err(|_| AppError::RutaFueraDelProyecto)?;
    let referencia = ruta_portable(relativa);
    let mensaje = format!(
        "Lee y sigue el expediente de inicio `{referencia}`. Usa MCP para devolver avances al mapa."
    );
    let argumentos = match agente {
        IdAgente::ClaudeCode | IdAgente::CodexCli => vec![mensaje],
        IdAgente::GeminiCli => vec!["-i".to_string(), mensaje],
        IdAgente::ClaudeDesktop | IdAgente::Antigravity | IdAgente::Cursor | IdAgente::Windsurf => {
            return Err(AppError::lectura(
                "consola del agente",
                std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "el producto seleccionado no ofrece una consola compatible",
                ),
            ))
        }
    };

    Ok(OrdenCli {
        programa: ejecutable.ruta_canonica.clone(),
        argumentos,
        directorio: contexto.raiz().to_path_buf(),
        ejecutable_confirmado: ejecutable,
    })
}

/// Resuelve y mide la consola del agente antes de presentar la confirmación al usuario.
///
/// La carpeta dirigida no puede contener el ejecutable: los agentes trabajan dentro de esa
/// carpeta y, por tanto, podrían sustituir un programa almacenado en ella.
pub fn confirmar_ejecutable(
    agente: &Agente,
    carpeta_dirigida: &Path,
) -> AppResult<EjecutableConfirmado> {
    let capacidad = agente.cli.as_ref().ok_or_else(|| {
        AppError::lectura(
            "consola del agente",
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "el agente no tiene una consola disponible",
            ),
        )
    })?;
    let ruta_canonica = capacidad
        .ejecutable
        .canonicalize()
        .map_err(|error| AppError::lectura(&capacidad.ejecutable, error))?;
    let proyecto_canonico = carpeta_dirigida
        .canonicalize()
        .map_err(|error| AppError::lectura(carpeta_dirigida, error))?;
    if ruta_canonica.starts_with(&proyecto_canonico) {
        return Err(AppError::CarpetaProyectoInvalida {
            ruta: proyecto_canonico,
            motivo: MotivoCarpetaInvalida::EjecutableDentroDeCarpetaDirigida {
                ruta_ejecutable: ruta_canonica,
            },
        });
    }
    let identidad = medir_ejecutable(&ruta_canonica)?;
    Ok(EjecutableConfirmado {
        ruta_canonica,
        identidad,
    })
}

/// Comprueba que el archivo sigue siendo el que se mostró al usuario.
fn verificar_ejecutable(ejecutable: &EjecutableConfirmado) -> AppResult<()> {
    let identidad_actual = medir_ejecutable(&ejecutable.ruta_canonica)?;
    if identidad_actual != ejecutable.identidad {
        return Err(AppError::lectura(
            &ejecutable.ruta_canonica,
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "el ejecutable cambió después de mostrar la vista previa",
            ),
        ));
    }
    Ok(())
}

/// Obtiene la identidad portable de un archivo regular.
fn medir_ejecutable(ruta: &Path) -> AppResult<IdentidadArchivo> {
    let metadatos = std::fs::metadata(ruta).map_err(|error| AppError::lectura(ruta, error))?;
    if !metadatos.is_file() {
        return Err(AppError::lectura(
            ruta,
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "la ruta de la consola no corresponde a un archivo",
            ),
        ));
    }
    let modificado = metadatos
        .modified()
        .map_err(|error| AppError::lectura(ruta, error))?;
    Ok(IdentidadArchivo {
        bytes: metadatos.len(),
        modificado,
    })
}

/// Normaliza únicamente el separador de una ruta relativa ya validada.
fn ruta_portable(ruta: &Path) -> String {
    ruta.to_string_lossy().replace('\\', "/")
}

/// Abre directamente una consola nueva en Windows, sin construir una cadena para `cmd.exe`.
#[cfg(windows)]
fn abrir_en_el_sistema(orden: &OrdenCli) -> AppResult<ResultadoLanzamiento> {
    use std::os::windows::process::CommandExt;

    const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;
    verificar_ejecutable(&orden.ejecutable_confirmado)?;
    let proceso = std::process::Command::new(&orden.programa)
        .args(&orden.argumentos)
        .current_dir(&orden.directorio)
        .creation_flags(CREATE_NEW_CONSOLE)
        .spawn()
        .map_err(|error| AppError::escritura(&orden.programa, error))?;
    Ok(ResultadoLanzamiento { pid: proceso.id() })
}

/// Abre una terminal conocida en Unix y le pasa programa y argumentos como elementos separados.
#[cfg(unix)]
fn abrir_en_el_sistema(orden: &OrdenCli) -> AppResult<ResultadoLanzamiento> {
    use std::ffi::OsStr;

    verificar_ejecutable(&orden.ejecutable_confirmado)?;
    let path = std::env::var_os("PATH").unwrap_or_default();
    let candidatos = [
        "x-terminal-emulator",
        "gnome-terminal",
        "konsole",
        "xfce4-terminal",
    ];
    let terminal = candidatos
        .iter()
        .find_map(|nombre| crate::conectores::buscar_ejecutable(nombre, &path, OsStr::new("")))
        .ok_or_else(|| {
            AppError::lectura(
                "terminal externa",
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no hay una terminal compatible instalada",
                ),
            )
        })?;
    let nombre = terminal.file_name().and_then(OsStr::to_str).unwrap_or("");
    let separador = match nombre {
        "gnome-terminal" => "--",
        "x-terminal-emulator" | "konsole" => "-e",
        "xfce4-terminal" => "--execute",
        otro => {
            return Err(AppError::lectura(
                &terminal,
                std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("terminal no admitida: {otro}"),
                ),
            ))
        }
    };
    let proceso = std::process::Command::new(&terminal)
        .arg(separador)
        .arg(&orden.programa)
        .args(&orden.argumentos)
        .current_dir(&orden.directorio)
        .spawn()
        .map_err(|error| AppError::escritura(&terminal, error))?;
    Ok(ResultadoLanzamiento { pid: proceso.id() })
}
