//! # Traslado y Reubicación de Mapas Mentales
//!
//! Este módulo centraliza las operaciones del sistema de archivos y flujos de usuario
//! necesarios para reubicar mapas mentales fuera de la raíz de unidades o volúmenes hacia
//! carpetas de proyecto dedicadas.
//!
//! Cumple con los principios SOLID (específicamente SRP, separando las operaciones de disco
//! y flujos de traslado de la lógica de aplicación principal), parte de esa
//! refactorización (A-3).

use std::path::{Path, PathBuf};

use crate::aplicacion::AplicacionMapaMental;
use crate::error::{AppError, MotivoCarpetaInvalida};
use crate::textos::Texto;

/// Mueve el archivo de un mapa mental desde su ubicación actual a una carpeta de proyecto de destino.
///
/// # Reglas y Comprobaciones
/// - Comprueba que la carpeta de destino sea una ubicación válida para proyecto (no raíz de disco ni restringida).
/// - Extrae y valida el nombre del archivo de origen.
/// - Comprueba que en la carpeta de destino no exista un archivo con el mismo nombre para evitar sobreescrituras.
/// - Intenta un renombramiento atómico en el mismo sistema de archivos. Si falla (por ejemplo, entre distintos volúmenes),
///   realiza copia y posterior eliminación del archivo original.
///
/// # Retorno
/// - `Ok(PathBuf)`: Ruta absoluta final del archivo trasladado en su nuevo destino.
/// - `Err(MotivoCarpetaInvalida)`: Causa estructurada y tipada del rechazo.
pub fn trasladar_archivo_mapa(
    ruta_origen: &Path,
    carpeta_destino: &Path,
) -> Result<PathBuf, MotivoCarpetaInvalida> {
    if !crate::conectores::es_carpeta_valida_para_espacio_trabajo(carpeta_destino) {
        return Err(MotivoCarpetaInvalida::RaizDeVolumenONoValida);
    }

    let nombre_archivo = ruta_origen
        .file_name()
        .ok_or(MotivoCarpetaInvalida::NoSePudoMoverMapa)?;

    let ruta_destino = carpeta_destino.join(nombre_archivo);
    if ruta_destino.exists() {
        return Err(MotivoCarpetaInvalida::DestinoYaExiste);
    }

    // Intenta renombrar atómicamente; si falla entre distintos volúmenes, copia y borra.
    let movido_con_exito = if std::fs::rename(ruta_origen, &ruta_destino).is_ok() {
        true
    } else if std::fs::copy(ruta_origen, &ruta_destino).is_ok() {
        let _ = std::fs::remove_file(ruta_origen);
        true
    } else {
        false
    };

    if !movido_con_exito {
        return Err(MotivoCarpetaInvalida::NoSePudoMoverMapa);
    }

    Ok(ruta_destino)
}

/// Ejecuta el traslado completo de un mapa mental a una carpeta de proyecto, actualizando
/// de forma coherente el estado de persistencia, sincronización y presentación de la aplicación.
pub fn ejecutar_traslado_de_mapa(
    app: &mut AplicacionMapaMental,
    ruta_origen: &Path,
    carpeta_destino: &Path,
) -> Result<PathBuf, MotivoCarpetaInvalida> {
    let ruta_destino = trasladar_archivo_mapa(ruta_origen, carpeta_destino)?;

    app.anotar_guardado(&ruta_destino);
    app.sincronizar_espacio_de_trabajo_con_el_mapa();
    app.presentacion_mut().aviso_mapa_en_raiz = None;
    let idioma = app.idioma();
    app.establecer_estado(Texto::AvisoMapaMovidoConExito.en(idioma));

    Ok(ruta_destino)
}

/// Solicita al usuario elegir una carpeta mediante diálogo nativo y traslada el mapa actual a ella.
///
/// Si el traslado falla (ej. colisión de nombres o carpeta inválida), se notifica adecuadamente
/// mediante el sistema centralizado de errores de la aplicación.
pub fn pedir_y_trasladar_mapa_en_raiz(app: &mut AplicacionMapaMental, ruta_origen: &Path) {
    let idioma = app.idioma();
    if let Some(carpeta) =
        crate::ui::dialogos::pedir_carpeta(Texto::BotonMoverMapaACarpeta.en(idioma))
    {
        if let Err(motivo) = ejecutar_traslado_de_mapa(app, ruta_origen, &carpeta) {
            app.reportar_error(
                &AppError::CarpetaProyectoInvalida {
                    ruta: carpeta,
                    motivo,
                },
                Texto::ModalMapaEnRaizTitulo.en(idioma),
            );
        }
    }
}

/// Solicita al usuario elegir una carpeta y crea un mapa nuevo dentro de ella con proyecto configurado.
pub fn pedir_y_crear_nuevo_en_carpeta(app: &mut AplicacionMapaMental) {
    let idioma = app.idioma();
    if let Some(carpeta) =
        crate::ui::dialogos::pedir_carpeta(Texto::BotonCrearNuevoEnCarpeta.en(idioma))
    {
        if crate::conectores::es_carpeta_valida_para_espacio_trabajo(&carpeta) {
            let nuevo =
                crate::model::Proyecto::nuevo_vacio(Texto::ArchivoNuevoProyectoDefecto.en(idioma));
            app.sustituir_el_mapa_abierto(nuevo, None, Texto::AvisoProyectoNuevo.en(idioma));
            // Misma canonicalización que `sincronizar_espacio_de_trabajo_con_el_mapa` (A-2): sin
            // esto, la carpeta recién elegida podía mostrarse distinta a como la pintan las
            // pantallas que sí pasan por el punto de cálculo compartido.
            let carpeta_canonica = crate::conectores::canonizar_o_conservar(carpeta);
            app.agentes_mut().espacio_de_trabajo = carpeta_canonica.clone();
            app.presentacion_mut().preferencias().espacio_trabajo_ia =
                carpeta_canonica.display().to_string();
            if let Err(error) = app.guardar_preferencias() {
                crate::error::registrar(&error, "guardar preferencia tras crear en carpeta");
            }
            app.presentacion_mut().aviso_mapa_en_raiz = None;
        } else {
            app.reportar_error(
                &AppError::CarpetaProyectoInvalida {
                    ruta: carpeta,
                    motivo: MotivoCarpetaInvalida::UbicacionDemasiadoAmplia,
                },
                Texto::ModalMapaEnRaizTitulo.en(idioma),
            );
        }
    }
}
