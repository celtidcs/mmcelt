//! Ciclo de persistencia del mapa activo.
//!
//! Este módulo concentra las decisiones de autoguardado, recuperación y registro de guardados
//! sobre estado e inputs explícitos. No conoce `AplicacionMapaMental` ni dibuja diálogos: la capa
//! de presentación traduce sus resultados a avisos, modales y cambios visibles.

use super::ServiciosAplicacion;
use crate::autoguardado::Recuperacion;
use crate::error::{AppError, AppResult};
use crate::model::Proyecto;
use crate::ui::estado_persistencia::EstadoPersistencia;
use std::path::{Path, PathBuf};

/// Resultado semántico de una comprobación periódica de autoguardado.
pub(crate) enum ResultadoAutoguardado {
    /// No tocaba comprobar o el contenido no había cambiado.
    SinCambios,
    /// Se escribió una copia nueva del mapa.
    Guardado,
    /// La copia no pudo escribirse y la presentación debe informar sin interrumpir.
    Fallo(AppError),
}

/// Autoguarda el mapa cuando vence el intervalo y existe una carpeta de recuperación.
pub(crate) fn autoguardar_si_toca(
    estado: &mut EstadoPersistencia,
    proyecto: &Proyecto,
    es_solo_lectura_por_raiz: bool,
) -> ResultadoAutoguardado {
    if es_solo_lectura_por_raiz || !estado.autoguardado.toca_comprobar() {
        return ResultadoAutoguardado::SinCambios;
    }
    let Some(carpeta) = estado.carpeta_datos.as_deref() else {
        return ResultadoAutoguardado::SinCambios;
    };

    match estado.autoguardado.autoguardar_si_cambio(
        carpeta,
        proyecto,
        estado.ruta_actual.as_deref(),
    ) {
        Ok(true) => ResultadoAutoguardado::Guardado,
        Ok(false) => ResultadoAutoguardado::SinCambios,
        Err(error) => ResultadoAutoguardado::Fallo(error),
    }
}

/// Guarda el proyecto en el destino y registra la nueva referencia persistida.
pub(crate) fn guardar(
    servicios: &ServiciosAplicacion,
    estado: &mut EstadoPersistencia,
    proyecto: &Proyecto,
    destino: &Path,
) -> AppResult<()> {
    servicios.guardar(proyecto, destino)?;
    registrar_guardado(estado, proyecto, destino);
    Ok(())
}

/// Extrae la recuperación pendiente para que la presentación decida cómo aplicarla.
pub(crate) fn restaurar_recuperacion(estado: &mut EstadoPersistencia) -> Option<Recuperacion> {
    estado.recuperacion.take()
}

/// Escribe una copia de recuperación del proyecto actual.
pub(crate) fn escribir_copia_de_recuperacion(
    estado: &EstadoPersistencia,
    proyecto: &Proyecto,
) -> AppResult<()> {
    let Some(carpeta) = estado.carpeta_datos.as_deref() else {
        return Err(AppError::escritura(
            "la carpeta de datos del usuario",
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no se pudo determinar dónde guarda sus datos este usuario",
            ),
        ));
    };

    crate::autoguardado::escribir_recuperacion_en(carpeta, proyecto, estado.ruta_actual.as_deref())
}

/// Borra la copia de recuperación de esta ejecución, si existe.
fn borrar_copia_de_recuperacion(estado: &EstadoPersistencia) {
    if let Some(carpeta) = estado.carpeta_datos.as_deref() {
        if let Err(error) = crate::autoguardado::descartar_recuperacion_en(carpeta) {
            crate::error::registrar(&error, "descartar copia de recuperación obsoleta");
        }
    }
}

/// Descarta la recuperación pendiente tanto en memoria como en disco.
pub(crate) fn descartar_recuperacion(estado: &mut EstadoPersistencia) {
    estado.recuperacion = None;
    borrar_copia_de_recuperacion(estado);
}

/// Registra un guardado y calcula la huella del archivo resultante.
pub(crate) fn registrar_guardado(
    estado: &mut EstadoPersistencia,
    proyecto: &Proyecto,
    ruta: &Path,
) {
    registrar_guardado_con_huella(estado, proyecto, ruta, crate::proyectos::huella(ruta));
}

/// Registra un guardado cuya huella ya se calculó fuera del hilo gráfico.
pub(crate) fn registrar_guardado_con_huella(
    estado: &mut EstadoPersistencia,
    proyecto: &Proyecto,
    ruta: &Path,
    huella: Option<u64>,
) {
    estado.ruta_actual = Some(ruta.to_path_buf());
    estado.autoguardado.dar_por_copiado(proyecto);
    borrar_copia_de_recuperacion(estado);
    estado.recuperacion = None;
    estado.ultima_huella_guardada = huella;
    estado.cambio_externo_pendiente = false;
}

/// Indica si el mapa o una edición transitoria difieren del último guardado.
pub(crate) fn tiene_cambios_locales_sin_guardar(
    estado: &EstadoPersistencia,
    proyecto: &Proyecto,
    hay_edicion_activa: bool,
) -> bool {
    estado.ruta_actual.is_none()
        || hay_edicion_activa
        || estado.autoguardado.ha_cambiado_desde_la_copia(proyecto)
}

/// Sugiere la carpeta del archivo actual o, en su defecto, la inferida del mapa.
pub(crate) fn carpeta_sugerida_para_guardar(
    estado: &EstadoPersistencia,
    proyecto: &Proyecto,
) -> Option<PathBuf> {
    estado
        .ruta_actual
        .as_deref()
        .and_then(|ruta| ruta.parent().map(Path::to_path_buf))
        .or_else(|| proyecto.carpeta_proyecto_sugerida())
}
