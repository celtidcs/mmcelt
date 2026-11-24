//! # Pregunta al cerrar con cambios sin guardar
//!
//! Cuando la persona cierra la ventana y el mapa tiene algo que no está en disco, MMCelt no se
//! cierra en silencio: pregunta si guarda, si cierra sin guardar o si cancela el cierre.
//!
//! Aquí solo vive la decisión, sin `egui` ni disco, para que se pruebe sola:
//! - [`hay_que_preguntar_al_cerrar`] dice si hace falta la pregunta.
//! - [`resolver_respuesta_al_cerrar`] dice si la ventana se cierra después de la respuesta.
//!
//! La ventana que pregunta, y quien atiende la petición de cierre del sistema, están en
//! `ui::dialogo_de_cierre`.

/// Lo que eligió la persona en la ventana del cierre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RespuestaAlCerrar {
    /// Guardar el mapa y cerrar. Sin archivo, abre «Guardar como».
    Guardar,
    /// Cerrar sin guardar.
    NoGuardar,
    /// Anular el cierre: la aplicación sigue abierta y no se guarda nada.
    Cancelar,
}

/// Qué pasa con la ventana después de la respuesta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesenlaceDelCierre {
    /// La aplicación se cierra.
    Cerrar,
    /// La aplicación sigue abierta.
    SeguirAbierta,
}

/// Lo que todavía no está en disco en el momento de pedir el cierre.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct CambiosPendientes {
    /// Hay un título a medio escribir en el lienzo.
    pub hay_edicion_activa: bool,
    /// El mapa cambió desde que se guardó o se abrió por última vez.
    pub mapa_cambiado: bool,
    /// La persona ha abierto nodos nuevos de la IA y ese resalte quitado aún no se ha guardado.
    /// Solo cuenta para esta pregunta: no retiene las recargas del archivo ni es un paso de
    /// deshacer.
    pub vistos_sin_guardar: bool,
    /// El mapa no tiene archivo: nada de lo que hay en pantalla está en disco.
    pub sin_archivo: bool,
    /// No hay nada que perder aunque falte el archivo: es el ejemplo del arranque o un mapa
    /// nuevo vacío creado con «Nuevo mapa», y la persona no lo ha tocado. No cuenta el número de
    /// nodos: una importación con solo la raíz puede traer notas que se perderían.
    pub mapa_desechable: bool,
}

/// Indica si hay que preguntar antes de cerrar.
///
/// Basta con que haya cualquiera de los tres cambios pendientes, o con que el mapa no tenga
/// archivo y no sea desechable: un escaneo, una importación o una plantilla sin guardar son
/// trabajo que se perdería. Sin nada de eso, la aplicación se cierra sin preguntar.
#[must_use]
pub(crate) fn hay_que_preguntar_al_cerrar(cambios: CambiosPendientes) -> bool {
    cambios.hay_edicion_activa
        || cambios.mapa_cambiado
        || cambios.vistos_sin_guardar
        || (cambios.sin_archivo && !cambios.mapa_desechable)
}

/// Decide si la aplicación se cierra después de la respuesta de la persona.
///
/// # Parámetros
/// - `respuesta`: el botón que pulsó.
/// - `guardar`: guarda el mapa y devuelve `true` si quedó guardado. Solo se llama con
///   [`RespuestaAlCerrar::Guardar`]. Devuelve `false` si la persona cancela «Guardar como» o si
///   el guardado falla, y entonces la aplicación no se cierra.
pub(crate) fn resolver_respuesta_al_cerrar(
    respuesta: RespuestaAlCerrar,
    guardar: impl FnOnce() -> bool,
) -> DesenlaceDelCierre {
    match respuesta {
        RespuestaAlCerrar::Cancelar => DesenlaceDelCierre::SeguirAbierta,
        RespuestaAlCerrar::NoGuardar => DesenlaceDelCierre::Cerrar,
        RespuestaAlCerrar::Guardar => {
            if guardar() {
                DesenlaceDelCierre::Cerrar
            } else {
                DesenlaceDelCierre::SeguirAbierta
            }
        }
    }
}
