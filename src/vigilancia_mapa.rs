//! Orquestación explícita de la vigilancia del mapa abierto.
//!
//! Este módulo separa la reacción a cambios del archivo de la aplicación gráfica. La interfaz
//! seguirá decidiendo cómo presentar cada resultado, mientras este límite conserva la identidad
//! del archivo vigilado, descarta auto-disparos y clasifica las cargas externas.

use crate::carga_en_segundo_plano::ResultadoCargaExterna;
use crate::error::AppResult;
use crate::model::Proyecto;
use crate::ui::estado_persistencia::EstadoPersistencia;

/// Decisión ya clasificada que la interfaz debe presentar tras una carga externa.
pub(crate) enum AccionCargaExterna {
    /// El resultado era atrasado, propio o inválido y no requiere ninguna reacción visual.
    Ignorar,
    /// Hay trabajo local sin guardar; se conserva y se avisa del cambio externo pendiente.
    RetenerPorCambiosLocales,
    /// El mapa externo es válido y puede sustituir al actual después de guardar su recuperación.
    Sustituir(Box<Proyecto>),
}

/// Inicia la vigilancia del archivo activo sin conocer la aplicación gráfica completa.
///
/// Devuelve `true` cuando existe un archivo que vigilar y el observador quedó instalado. Si el
/// mapa aún no tiene ruta, no hace nada y devuelve `false`.
pub(crate) fn iniciar_vigilancia(
    estado: &mut EstadoPersistencia,
    ctx: &egui::Context,
) -> AppResult<bool> {
    let Some(ruta) = &estado.ruta_actual else {
        return Ok(false);
    };
    let vigilante = crate::vigilante::VigilanteArchivo::iniciar(ruta, ctx.clone())?;
    if let Some(huella) = crate::proyectos::huella(ruta) {
        estado.ultima_huella_guardada = Some(huella);
    }
    estado.vigilante = Some(vigilante);
    Ok(true)
}

/// Detiene el observador y descarta cualquier aviso externo ya retenido.
pub(crate) fn detener_vigilancia(estado: &mut EstadoPersistencia) {
    estado.vigilante = None;
    estado.cambio_externo_pendiente = false;
}

/// Solicita las lecturas notificadas y clasifica todos los resultados ya disponibles.
pub(crate) fn atender_vigilancia(
    estado: &mut EstadoPersistencia,
    hay_cambios_locales: bool,
) -> Vec<AccionCargaExterna> {
    if let Some(vigilante) = &estado.vigilante {
        if vigilante.hay_cambios() {
            estado
                .cargador_externo
                .solicitar(vigilante.ruta().to_path_buf());
        }
    }

    let mut acciones = Vec::new();
    while let Some(resultado) = estado.cargador_externo.intentar_recibir() {
        acciones.push(aplicar_carga_externa(
            estado,
            resultado,
            hay_cambios_locales,
        ));
    }
    acciones
}

/// Descarta resultados obsoletos o propios y convierte una carga válida en una acción cerrada.
fn aplicar_carga_externa(
    estado: &mut EstadoPersistencia,
    resultado: ResultadoCargaExterna,
    hay_cambios_locales: bool,
) -> AccionCargaExterna {
    let sigue_siendo_el_archivo_vigilado = estado
        .vigilante
        .as_ref()
        .is_some_and(|vigilante| vigilante.ruta() == resultado.ruta);
    if !sigue_siendo_el_archivo_vigilado || Some(resultado.huella) == estado.ultima_huella_guardada
    {
        return AccionCargaExterna::Ignorar;
    }
    let Ok(nuevo_proyecto) = resultado.proyecto else {
        return AccionCargaExterna::Ignorar;
    };

    if hay_cambios_locales {
        estado.cambio_externo_pendiente = true;
        return AccionCargaExterna::RetenerPorCambiosLocales;
    }

    estado.autoguardado.dar_por_copiado(&nuevo_proyecto);
    estado.ultima_huella_guardada = Some(resultado.huella);
    estado.cambio_externo_pendiente = false;
    AccionCargaExterna::Sustituir(Box::new(nuevo_proyecto))
}
