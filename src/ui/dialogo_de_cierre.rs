//! # Ventana «¿Guardar los cambios antes de cerrar?»
//!
//! Se abre cuando la persona cierra MMCelt con cambios sin guardar. Ofrece Guardar, No guardar
//! y Cancelar. La decisión de abrirla y lo que pasa después de cada botón están en
//! [`crate::cierre_con_cambios`]; aquí solo se pinta y se recoge el botón pulsado.

use crate::aplicacion::AplicacionMapaMental;
use crate::cierre_con_cambios::{
    hay_que_preguntar_al_cerrar, resolver_respuesta_al_cerrar, CambiosPendientes,
    DesenlaceDelCierre, RespuestaAlCerrar,
};
use crate::textos::Texto;

/// Separación vertical entre la explicación y los botones, en puntos de `egui`.
const SEPARACION_ANTES_DE_LOS_BOTONES: f32 = 12.0;

/// Ancho mínimo de la ventana, en puntos de `egui`: cabe la explicación en dos líneas.
const ANCHO_MINIMO: f32 = 420.0;

/// Dibuja la ventana del cierre, si está abierta, y atiende el botón pulsado.
///
/// Cerrar la ventana con su aspa equivale a «Cancelar»: la aplicación sigue abierta.
pub(crate) fn dibujar_dialogo_de_cierre(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    if !app.presentacion().pregunta_al_cerrar {
        return;
    }

    let idioma = app.idioma();
    let mut abierta = true;
    let mut respuesta: Option<RespuestaAlCerrar> = None;

    egui::Window::new(Texto::CierreConCambiosTitulo.en(idioma))
        .open(&mut abierta)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .min_width(ANCHO_MINIMO)
        .show(ctx, |ui| {
            ui.label(Texto::CierreConCambiosExplicacion.en(idioma));
            ui.add_space(SEPARACION_ANTES_DE_LOS_BOTONES);
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new(Texto::CierreBotonGuardar.en(idioma)).strong())
                    .clicked()
                {
                    respuesta = Some(RespuestaAlCerrar::Guardar);
                }
                if ui.button(Texto::CierreBotonNoGuardar.en(idioma)).clicked() {
                    respuesta = Some(RespuestaAlCerrar::NoGuardar);
                }
                if ui.button(Texto::CierreBotonCancelar.en(idioma)).clicked() {
                    respuesta = Some(RespuestaAlCerrar::Cancelar);
                }
            });
        });

    if !abierta {
        respuesta = Some(RespuestaAlCerrar::Cancelar);
    }
    if let Some(elegida) = respuesta {
        responder_al_cierre(app, ctx, elegida);
    }
}

/// Lo que todavía no está en disco, para decidir si se pregunta al cerrar.
pub(crate) fn cambios_pendientes_al_cerrar(app: &AplicacionMapaMental) -> CambiosPendientes {
    let mapa_cambiado = app
        .persistencia()
        .autoguardado
        .ha_cambiado_desde_la_copia(app.mapa().proyecto());
    CambiosPendientes {
        hay_edicion_activa: app.lienzo().edicion().nodo.is_some(),
        mapa_cambiado,
        vistos_sin_guardar: app.mapa().hay_vistos_sin_guardar(),
        sin_archivo: app.persistencia().ruta_actual.is_none(),
        mapa_desechable: !mapa_cambiado && app.mapa().es_un_mapa_de_partida(),
    }
}

/// Atiende la petición de cierre de la ventana que llega del sistema.
///
/// Si hay cambios sin guardar y la persona aún no ha respondido, anula el cierre con
/// `ViewportCommand::CancelClose` y abre la ventana que pregunta.
pub(crate) fn atender_peticion_de_cierre(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let pedido = ctx.input(|entrada| entrada.viewport().close_requested());
    if !pedido || app.presentacion().cierre_confirmado {
        return;
    }
    if hay_que_preguntar_al_cerrar(cambios_pendientes_al_cerrar(app)) {
        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        app.presentacion_mut().pregunta_al_cerrar = true;
    }
}

/// Aplica la respuesta de la ventana del cierre: guarda si hace falta y cierra o no.
///
/// «Guardar» confirma primero el título que se esté escribiendo y luego usa el mismo camino que
/// Ctrl+S: sin archivo abre «Guardar como», y si ahí se cancela o el guardado falla, la
/// aplicación sigue abierta.
pub(crate) fn responder_al_cierre(
    app: &mut AplicacionMapaMental,
    ctx: &egui::Context,
    respuesta: RespuestaAlCerrar,
) {
    app.presentacion_mut().pregunta_al_cerrar = false;
    let desenlace = resolver_respuesta_al_cerrar(respuesta, || {
        // Un título a medio escribir es parte de lo que se guarda: se da por bueno antes,
        // igual que al hacer clic fuera del nodo.
        app.confirmar_edicion_del_titulo();
        app.solicitar_guardado(false)
    });
    if desenlace == DesenlaceDelCierre::Cerrar {
        app.presentacion_mut().cierre_confirmado = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}
