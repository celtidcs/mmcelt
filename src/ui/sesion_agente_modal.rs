//! Estado y dibujo de la vista previa supervisada de una sesión de agente.

use crate::aplicacion::AplicacionMapaMental;
use crate::conectores::Agente;
use crate::configuracion_agente_proyecto::EstadoFuenteNativa;
use crate::error::{AppError, AppResult};
use crate::proyecto_trabajo::ContextoProyecto;
use crate::sesiones_agentes::BorradorSesion;
use crate::textos::{Idioma, Texto};
use egui::{RichText, ScrollArea};

/// Altura mínima que conserva útiles los bloques editables en una pantalla pequeña.
const ALTURA_MINIMA_BLOQUES: f32 = 120.0;
/// Altura mínima de la lectura final para distinguirla de los bloques editables.
const ALTURA_MINIMA_VISTA_COMPLETA: f32 = 96.0;
/// Altura máxima de la lectura final; por encima conviene desplazar su propio contenido.
const ALTURA_MAXIMA_VISTA_COMPLETA: f32 = 180.0;
/// Espacio reservado para el rótulo final, separadores y fila de acciones.
const ALTURA_RESERVADA_PIE: f32 = 90.0;
/// Proporción del espacio restante destinada a la lectura final.
const PROPORCION_VISTA_COMPLETA: f32 = 0.28;

/// Compone la representación exacta que el modal puede confirmar y el expediente persistirá.
///
/// # Errores
///
/// Propaga la validación centralizada de controles y tamaños del compositor de dominio.
pub(crate) fn componer_vista_previa(
    borrador: &BorradorSesion,
) -> AppResult<crate::sesiones_agentes::PromptSesionCompuesto> {
    crate::sesiones_agentes::componer_prompt_validado(borrador)
}

/// Borrador visible que existe únicamente entre elegir un agente y confirmar o cancelar.
#[derive(Clone)]
pub struct EditorSesionAgente {
    /// Agente elegido, incluida la ruta de su consola detectada.
    pub agente: Agente,
    /// Bloques exactos que mostrará y podrá editar el modal.
    pub borrador: BorradorSesion,
    /// Proyecto autorizado al que pertenecen mapa, fuentes y futuro expediente.
    pub contexto: ContextoProyecto,
    /// Revisión del mapa con la que se generó el contexto protegido.
    pub revision_mapa: crate::model::RevisionProyecto,
    /// Idioma del documento para el agente, independiente del idioma de la interfaz.
    pub idioma_documento: Idioma,
    /// Ejecutable exacto que se mostró al usuario, si este agente dispone de consola.
    pub ejecutable_confirmado: Option<crate::lanzador_agentes::EjecutableConfirmado>,
    /// Perfil portable tal como estaba antes de editar esta sesión.
    pub(crate) reglas_originales: String,
}

/// Acciones que la vista comunica al controlador después de terminar el fotograma.
#[derive(Default)]
struct AccionesEditor {
    cancelar: bool,
    abrir_configuracion_encargo: bool,
    iniciar: bool,
}

/// Dibuja la vista previa y aplica únicamente las acciones expresas de sus botones.
pub fn dibujar_modal_sesion_agente(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let Some(mut editor) = app.agentes_mut().editor_sesion.take() else {
        return;
    };
    let idioma = app.idioma();
    let preparando = app.agentes_mut().envio == crate::envio_agente::EstadoEnvio::Preparando;
    let mut abierta = true;
    let mut acciones = AccionesEditor::default();

    egui::Window::new(Texto::SesionAgenteTitulo.en(idioma))
        .open(&mut abierta)
        .min_width(760.0)
        .min_height(620.0)
        .show(ctx, |ui| {
            if preparando {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.strong(Texto::SesionAgentePreparando.en(idioma));
                });
                ui.separator();
            }
            ui.add_enabled_ui(!preparando, |ui| {
                acciones =
                    dibujar_contenido_editor(&mut editor, app.mapa().proyecto(), idioma, 480.0, ui);
            });
        });

    if !abierta || acciones.cancelar {
        if preparando {
            // El trabajo ya puede haber abierto la consola; cerrar la vista no lo cancela.
            // Conservamos el borrador visible hasta recibir el resultado.
            app.agentes_mut().editor_sesion = Some(editor);
        } else {
            app.cancelar_sesion_agente();
        }
        return;
    }
    if acciones.abrir_configuracion_encargo {
        app.cancelar_sesion_agente();
        app.abrir_proyecto_ia_en_pestana(
            crate::ui::proyecto_ia_modal::PestanaProyectoIa::Plantillas,
        );
        return;
    }
    app.agentes_mut().editor_sesion = Some(editor);
    if acciones.iniciar {
        match app.confirmar_sesion_agente(ctx) {
            Ok(true) => {}
            Ok(false) => {}
            Err(error @ AppError::ConfirmacionCaducada) => {
                app.reportar_error(&error, Texto::ProyectoIaConfirmacionCaducada.en(idioma))
            }
            Err(error) => app.reportar_error(&error, Texto::SesionAgenteTitulo.en(idioma)),
        }
    }
}

/// Dibuja el contenido compartido por la ventana real y el arnés de interfaz.
fn dibujar_contenido_editor(
    editor: &mut EditorSesionAgente,
    _proyecto: &crate::model::Proyecto,
    idioma: Idioma,
    altura_maxima: f32,
    ui: &mut egui::Ui,
) -> AccionesEditor {
    ui.heading(editor.agente.nombre);
    if let Some(ejecutable) = &editor.ejecutable_confirmado {
        ui.label(RichText::new(Texto::SesionAgenteEjecutable.en(idioma)).strong());
        ui.monospace(crate::ui::limpiar_ruta_para_interfaz(
            &ejecutable.ruta_canonica,
        ));
    } else {
        ui.label(
            RichText::new(Texto::SesionAgenteSoloMcpAviso.en(idioma))
                .strong()
                .color(ui.visuals().warn_fg_color),
        );
    }
    ui.monospace(crate::ui::limpiar_ruta_para_interfaz(
        editor.contexto.raiz(),
    ));
    ui.horizontal(|ui| {
        ui.label(RichText::new(Texto::ProyectoIaIdiomaDocumento.en(idioma)).strong());
        ui.label(editor.idioma_documento.nombre_para_interfaz());
    });
    ui.separator();

    let altura_disponible = ui.available_height();
    let altura_vista_completa = (altura_disponible * PROPORCION_VISTA_COMPLETA)
        .clamp(ALTURA_MINIMA_VISTA_COMPLETA, ALTURA_MAXIMA_VISTA_COMPLETA);
    let altura_bloques = (altura_disponible - altura_vista_completa - ALTURA_RESERVADA_PIE)
        .clamp(ALTURA_MINIMA_BLOQUES, altura_maxima);

    let mut acciones = AccionesEditor::default();

    ScrollArea::vertical()
        .id_salt("bloques_editables_sesion_agente")
        .max_height(altura_bloques)
        .show(ui, |ui| {
            ui.label(RichText::new(Texto::SesionAgenteContrato.en(idioma)).strong());
            ui.add(
                egui::TextEdit::multiline(&mut editor.borrador.contrato_mmcelt)
                    .desired_rows(7)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
            ui.label(RichText::new(Texto::SesionAgenteReglas.en(idioma)).strong());
            ui.add(
                egui::TextEdit::multiline(&mut editor.borrador.reglas_proyecto)
                    .desired_rows(7)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
            ui.label(RichText::new(Texto::SesionAgenteContexto.en(idioma)).strong());
            ui.add(
                egui::TextEdit::multiline(&mut editor.borrador.contexto_mapa)
                    .desired_rows(9)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
            ui.label(RichText::new(Texto::SesionAgenteEncargo.en(idioma)).strong());
            if editor.borrador.encargo.trim().is_empty() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(Texto::SesionAgenteEncargoVacioAviso.en(idioma))
                            .color(ui.visuals().warn_fg_color)
                            .strong(),
                    );
                    if ui
                        .button(
                            RichText::new(Texto::SesionAgenteBotonConfigurarEncargo.en(idioma))
                                .strong(),
                        )
                        .clicked()
                    {
                        acciones.abrir_configuracion_encargo = true;
                    }
                });
            }
            ui.add(
                egui::TextEdit::multiline(&mut editor.borrador.encargo)
                    .desired_rows(4)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
            ui.label(RichText::new(Texto::SesionAgenteFuentes.en(idioma)).strong());
            for fuente in &mut editor.borrador.fuentes_nativas {
                egui::CollapsingHeader::new(fuente.nombre).show(ui, |ui| {
                    ui.add_enabled(
                        false,
                        egui::Checkbox::new(&mut fuente.incorporada, fuente.nombre),
                    );
                    ui.monospace(crate::ui::limpiar_ruta_para_interfaz(&fuente.ruta));
                    let rotulo_estado = match fuente.estado {
                        EstadoFuenteNativa::Nueva => Texto::ProyectoIaFuenteNueva,
                        EstadoFuenteNativa::AceptadaSinCambios => Texto::ProyectoIaFuenteAceptada,
                        EstadoFuenteNativa::Modificada => Texto::ProyectoIaFuenteModificada,
                    };
                    ui.label(RichText::new(rotulo_estado.en(idioma)).strong());
                    if let Some(anterior) = &fuente.contenido_anterior {
                        ui.label(
                            RichText::new(Texto::ProyectoIaVersionAnterior.en(idioma)).strong(),
                        );
                        ui.label(RichText::new(anterior).strikethrough());
                    }
                    ui.label(RichText::new(Texto::ProyectoIaVersionActual.en(idioma)).strong());
                    ui.add(
                        egui::TextEdit::multiline(&mut fuente.contenido)
                            .desired_rows(5)
                            .desired_width(f32::INFINITY)
                            .interactive(false),
                    );
                });
            }
        });

    let composicion = componer_vista_previa(&editor.borrador);
    if let Ok(prompt) = &composicion {
        ui.separator();
        ui.label(RichText::new(Texto::PestanaProyectoIaVistaCompleta.en(idioma)).strong());
        ScrollArea::vertical()
            .id_salt("vista_completa_sesion_agente")
            .max_height(altura_vista_completa)
            .show(ui, |ui| {
                ui.monospace(prompt.texto());
            });
    } else if let Err(error) = &composicion {
        ui.colored_label(
            ui.visuals().error_fg_color,
            error.mensaje_usuario().en(idioma),
        );
    }

    ui.separator();
    ui.horizontal(|ui| {
        acciones.cancelar = ui.button(Texto::BotonCancelar.en(idioma)).clicked();
        let texto_inicio = if editor.agente.cli.is_some() {
            Texto::SesionAgenteIniciar
        } else {
            Texto::SesionAgentePrepararMcp
        };
        let puede_iniciar = composicion.is_ok() && !editor.borrador.encargo.trim().is_empty();
        acciones.iniciar = ui
            .add_enabled(
                puede_iniciar,
                egui::Button::new(RichText::new(texto_inicio.en(idioma)).strong()),
            )
            .clicked();
    });
    acciones
}

/// Dibuja en línea el mismo contenido que la ventana real para poder verificarlo sin pantalla.
#[cfg(test)]
pub(crate) fn dibujar_contenido_para_prueba(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();
    if let Some(mut editor) = app.agentes_mut().editor_sesion.clone() {
        // El arnés no puede desplazar una `ScrollArea`. Acorta únicamente los campos masivos
        // en la copia para que ejecute y observe todos los controles del mismo renderizador.
        editor.borrador.contrato_mmcelt = "contrato".to_owned();
        editor.borrador.contexto_mapa = "contexto".to_owned();
        for fuente in &mut editor.borrador.fuentes_nativas {
            fuente.contenido = "contenido detectado".to_owned();
        }
        let proyecto = app.mapa().proyecto().clone();
        let acciones = dibujar_contenido_editor(&mut editor, &proyecto, idioma, 10_000.0, ui);
        if acciones.abrir_configuracion_encargo {
            app.cancelar_sesion_agente();
            app.abrir_proyecto_ia_en_pestana(
                crate::ui::proyecto_ia_modal::PestanaProyectoIa::Plantillas,
            );
        }
    }
}

/// Dibuja los textos y dimensiones reales para las pruebas de alcanzabilidad de la ventana.
#[cfg(test)]
pub(crate) fn dibujar_contenido_real_para_prueba(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
) {
    let idioma = app.idioma();
    if let Some(mut editor) = app.agentes_mut().editor_sesion.clone() {
        let proyecto = app.mapa().proyecto().clone();
        let acciones = dibujar_contenido_editor(&mut editor, &proyecto, idioma, 480.0, ui);
        if acciones.abrir_configuracion_encargo {
            app.cancelar_sesion_agente();
            app.abrir_proyecto_ia_en_pestana(
                crate::ui::proyecto_ia_modal::PestanaProyectoIa::Plantillas,
            );
        }
    }
}
