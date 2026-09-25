//! Ventana unificada para describir el proyecto y preparar instrucciones para cualquier agente.
//!
//! El dibujo solo modifica memoria y devuelve acciones. Las lecturas y escrituras portables se
//! ejecutan antes o después del callback de `egui`, cuando no existe un préstamo visual activo.

use crate::ai_export::PlantillaEncargo;
use crate::aplicacion::AplicacionMapaMental;
use crate::configuracion_agente_proyecto::EstadoFuenteNativa;
use crate::error::AppResult;
use crate::sesiones_agentes::{BorradorSesion, FuenteInstrucciones};
use crate::textos::{Idioma, Texto};
use egui::{RichText, ScrollArea};
use std::path::PathBuf;

/// Sección activa de la ventana de proyecto e instrucciones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PestanaProyectoIa {
    /// Visión, objetivos, público y autor que pertenecen al mapa.
    Proyecto,
    /// Reglas portables y fuentes nativas que requieren consentimiento.
    Instrucciones,
    /// Ayudas para elegir un único encargo de sesión.
    Plantillas,
    /// Texto exacto compuesto a partir de los cuatro bloques.
    VistaCompleta,
}

impl PestanaProyectoIa {
    /// Orden estable de las cuatro secciones de la ventana.
    pub const TODAS: [Self; 4] = [
        Self::Proyecto,
        Self::Instrucciones,
        Self::Plantillas,
        Self::VistaCompleta,
    ];

    /// Devuelve el rótulo visible de esta pestaña en el idioma indicado.
    pub fn titulo(self, idioma: Idioma) -> &'static str {
        match self {
            Self::Proyecto => Texto::PestanaProyectoIaProyecto.en(idioma),
            Self::Instrucciones => Texto::PestanaProyectoIaInstrucciones.en(idioma),
            Self::Plantillas => Texto::PestanaProyectoIaPlantillas.en(idioma),
            Self::VistaCompleta => Texto::PestanaProyectoIaVistaCompleta.en(idioma),
        }
    }
}

/// Borradores que deben sobrevivir entre fotogramas sin escribir por el mero hecho de dibujarse.
#[derive(Clone)]
pub struct EstadoEditorProyectoIa {
    /// Raíz canónica a la que pertenecen tanto las fuentes como el encargo transitorio.
    pub raiz_proyecto: PathBuf,
    /// Pestaña que la persona está consultando.
    pub pestana: PestanaProyectoIa,
    /// Reglas comunes editables del proyecto.
    pub reglas: String,
    /// Único encargo editable que alimentará la próxima sesión.
    pub encargo: String,
    /// Idioma del documento para el agente, independiente del idioma de la aplicación.
    pub idioma_documento: Idioma,
    /// Instrucciones nativas detectadas y comparadas con su consentimiento durable.
    pub fuentes: Vec<FuenteInstrucciones>,
    /// Índice de la plantilla elegida en el selector desplegable.
    pub indice_plantilla_seleccionada: usize,
    /// Búfer para el nombre de una nueva plantilla de usuario a guardar.
    pub nombre_nueva_plantilla: String,
}

/// Decisión expresa que el controlador aplicará después de cerrar el préstamo de interfaz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AccionProyectoIa {
    /// No se ha pedido ningún efecto persistente.
    Ninguna,
    /// Cambiar y persistir el idioma del documento para la IA.
    CambiarIdiomaDocumento(Idioma),
    /// Guardar las reglas y el idioma elegidos para este proyecto.
    GuardarReglas,
    /// Aceptar la versión actual de una fuente nativa concreta.
    AceptarFuente(String),
    /// Cerrar y descartar el estado transitorio de la ventana.
    Cerrar,
    /// Cerrar después de confirmar la edición inmediata de los datos del mapa.
    GuardarProyecto,
    /// Guardar el encargo actual como plantilla global de usuario.
    GuardarComoPlantilla(String),
}

/// Dibuja la ventana real y aplica fuera del callback las acciones que escriben en disco.
pub fn dibujar_modal_proyecto_ia(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    if !app.presentacion_mut().ventanas().modal_datos_del_proyecto {
        return;
    }
    let raiz_actual = match crate::proyecto_trabajo::RepositorioProyecto::contexto_para_vista_previa(
        &app.agentes().espacio_de_trabajo,
    ) {
        Ok(contexto) => Some(contexto.raiz().to_path_buf()),
        Err(_) => None,
    };
    let necesita_recargar = match app.presentacion_mut().editor_proyecto_ia() {
        None => true,
        Some(editor) => match &raiz_actual {
            Some(raiz) => &editor.raiz_proyecto != raiz,
            None => true,
        },
    };
    if necesita_recargar {
        let pestana_previa = if let Some(viejo_editor) =
            app.presentacion_mut().editor_proyecto_ia().take()
        {
            if !viejo_editor.encargo.trim().is_empty() {
                app.agentes_mut()
                    .conservar_encargo_preparado(viejo_editor.raiz_proyecto, viejo_editor.encargo);
            }
            Some(viejo_editor.pestana)
        } else {
            None
        };
        match cargar_editor(app) {
            Ok(mut editor) => {
                if let Some(pestana) = pestana_previa {
                    editor.pestana = pestana;
                }
                *app.presentacion_mut().editor_proyecto_ia() = Some(editor);
            }
            Err(error) => {
                app.presentacion_mut().ventanas().modal_datos_del_proyecto = false;
                app.reportar_error(&error, "abrir proyecto e instrucciones para la IA");
                app.mostrar_aviso_error_proyecto_ia(&error);
                return;
            }
        }
    }

    let idioma = app.idioma();
    let mut editor = app
        .presentacion_mut()
        .editor_proyecto_ia()
        .take()
        .expect("el editor acaba de inicializarse");
    let mut abierta = true;
    let mut accion = AccionProyectoIa::Ninguna;
    egui::Window::new(Texto::ModalProyectoEInstruccionesIa.en(idioma))
        .open(&mut abierta)
        .min_width(760.0)
        .min_height(620.0)
        .show(ctx, |ui| {
            accion = dibujar_contenido(app, &mut editor, idioma, ui);
        });
    if !abierta {
        accion = AccionProyectoIa::Cerrar;
    }

    let conservar = aplicar_accion(app, &mut editor, accion, idioma);
    if conservar {
        *app.presentacion_mut().editor_proyecto_ia() = Some(editor);
    } else {
        app.presentacion_mut().ventanas().modal_datos_del_proyecto = false;
    }
}

/// Carga los borradores y consentimientos antes de prestar la aplicación al dibujante.
pub(crate) fn cargar_editor(app: &AplicacionMapaMental) -> AppResult<EstadoEditorProyectoIa> {
    let contexto = crate::proyecto_trabajo::RepositorioProyecto::contexto_para_vista_previa(
        &app.agentes().espacio_de_trabajo,
    )?;
    let configuracion = crate::configuracion_agente_proyecto::cargar_configuracion(&contexto)?;
    let encargo_memoria = app.agentes().encargo_preparado_para(contexto.raiz());
    let encargo = if !encargo_memoria.is_empty() {
        encargo_memoria
    } else {
        configuracion.encargo.clone()
    };
    Ok(EstadoEditorProyectoIa {
        raiz_proyecto: contexto.raiz().to_path_buf(),
        pestana: PestanaProyectoIa::Proyecto,
        reglas: crate::sesiones_agentes::cargar_reglas_comunes(&contexto)?,
        encargo,
        idioma_documento: configuracion
            .idioma_documento
            .unwrap_or_else(|| app.idioma()),
        fuentes: crate::sesiones_agentes::detectar_fuentes_nativas(&contexto)?,
        indice_plantilla_seleccionada: 0,
        nombre_nueva_plantilla: String::new(),
    })
}

/// Ejecuta la única escritura solicitada y comunica si la ventana debe conservarse.
fn aplicar_accion(
    app: &mut AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    accion: AccionProyectoIa,
    idioma: Idioma,
) -> bool {
    let toco_proyecto = editor.pestana == PestanaProyectoIa::Proyecto
        || matches!(accion, AccionProyectoIa::GuardarProyecto);

    let resultado = match &accion {
        AccionProyectoIa::Ninguna => return true,
        AccionProyectoIa::Cerrar => {
            let res = persistir_cierre(app, editor, toco_proyecto);
            if let Err(error) = res {
                app.reportar_error(&error, Texto::ModalProyectoEInstruccionesIa.en(idioma));
            }
            return false;
        }
        AccionProyectoIa::GuardarProyecto => {
            let res = persistir_cierre(app, editor, true);
            if let Err(error) = res {
                app.reportar_error(&error, Texto::ModalProyectoEInstruccionesIa.en(idioma));
            } else {
                app.establecer_estado(Texto::ModalMetadatosYVisionDel.en(idioma));
            }
            return false;
        }
        AccionProyectoIa::CambiarIdiomaDocumento(nuevo_idioma) => {
            cambiar_idioma_documento(app, editor, *nuevo_idioma)
        }
        AccionProyectoIa::GuardarReglas => guardar_reglas(app, editor),
        AccionProyectoIa::AceptarFuente(nombre) => aceptar_y_recargar(app, editor, nombre),
        AccionProyectoIa::GuardarComoPlantilla(nombre) => {
            guardar_como_plantilla(app, editor, nombre.clone())
        }
    };
    match resultado {
        Ok(()) => {
            if matches!(accion, AccionProyectoIa::GuardarReglas) {
                app.establecer_estado(Texto::SesionAgenteGuardarReglas.en(idioma));
            } else if matches!(accion, AccionProyectoIa::GuardarComoPlantilla(_)) {
                app.establecer_estado(Texto::ProyectoIaPlantillaGuardadaAviso.en(idioma));
            }
            true
        }
        Err(error) => {
            app.reportar_error(&error, Texto::ModalProyectoEInstruccionesIa.en(idioma));
            true
        }
    }
}

/// Persiste en disco encargo, reglas y estado al cerrar el modal de proyecto e instrucciones.
fn persistir_cierre(
    app: &mut AplicacionMapaMental,
    editor: &EstadoEditorProyectoIa,
    toco_proyecto: bool,
) -> AppResult<()> {
    let contexto = crate::proyecto_trabajo::RepositorioProyecto::contexto_para_vista_previa(
        &editor.raiz_proyecto,
    )?;

    // 1. Persistir encargo e idioma en .mmcelt/configuracion-agente.json
    let mut configuracion = crate::configuracion_agente_proyecto::cargar_configuracion(&contexto)?;
    configuracion.encargo = editor.encargo.clone();
    configuracion.idioma_documento = Some(editor.idioma_documento);
    crate::configuracion_agente_proyecto::guardar_configuracion(&contexto, &configuracion)?;

    // 2. Persistir reglas comunes en .mmcelt/instrucciones-agente.md
    crate::sesiones_agentes::guardar_reglas_comunes(&contexto, &editor.reglas)?;

    // 3. Sincronizar en memoria para la sesión activa
    app.agentes_mut()
        .conservar_encargo_preparado(editor.raiz_proyecto.clone(), editor.encargo.clone());

    // 4. Si tocó «Proyecto», guardar el mapa si tiene ruta asignada
    if toco_proyecto && app.persistencia().ruta_actual.is_some() {
        app.solicitar_guardado(false);
    }

    Ok(())
}

/// Persiste de inmediato el idioma elegido para el documento del proyecto en curso.
fn cambiar_idioma_documento(
    _app: &AplicacionMapaMental,
    editor: &EstadoEditorProyectoIa,
    nuevo_idioma: Idioma,
) -> AppResult<()> {
    let contexto = crate::proyecto_trabajo::RepositorioProyecto::contexto_para_vista_previa(
        &editor.raiz_proyecto,
    )?;
    let mut configuracion = crate::configuracion_agente_proyecto::cargar_configuracion(&contexto)?;
    configuracion.idioma_documento = Some(nuevo_idioma);
    crate::configuracion_agente_proyecto::guardar_configuracion(&contexto, &configuracion)
}

/// Persiste reglas e idioma únicamente tras la acción de guardado.
fn guardar_reglas(_app: &AplicacionMapaMental, editor: &EstadoEditorProyectoIa) -> AppResult<()> {
    let contexto = crate::proyecto_trabajo::RepositorioProyecto::contexto_para_vista_previa(
        &editor.raiz_proyecto,
    )?;
    crate::sesiones_agentes::guardar_reglas_comunes(&contexto, &editor.reglas)?;
    let mut configuracion = crate::configuracion_agente_proyecto::cargar_configuracion(&contexto)?;
    configuracion.idioma_documento = Some(editor.idioma_documento);
    crate::configuracion_agente_proyecto::guardar_configuracion(&contexto, &configuracion)
}

/// Acepta desde disco y vuelve a comparar todas las fuentes para actualizar la vista.
fn aceptar_y_recargar(
    _app: &AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    nombre: &str,
) -> AppResult<()> {
    let contexto = crate::proyecto_trabajo::RepositorioProyecto::contexto_para_vista_previa(
        &editor.raiz_proyecto,
    )?;
    crate::configuracion_agente_proyecto::aceptar_fuente(&contexto, nombre)?;
    editor.fuentes = crate::sesiones_agentes::detectar_fuentes_nativas(&contexto)?;
    Ok(())
}

/// Guarda el encargo actual como plantilla de usuario en las preferencias globales.
fn guardar_como_plantilla(
    app: &mut AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    nombre: String,
) -> AppResult<()> {
    let nombre_limpio = nombre.trim();
    if nombre_limpio.is_empty() || editor.encargo.trim().is_empty() {
        return Ok(());
    }
    let plantillas = &mut app.presentacion_mut().preferencias().plantillas_usuario;
    if let Some(existente) = plantillas.iter_mut().find(|p| p.nombre == nombre_limpio) {
        existente.contenido = editor.encargo.clone();
    } else {
        plantillas.push(crate::preferencias::PlantillaUsuario {
            nombre: nombre_limpio.to_owned(),
            contenido: editor.encargo.clone(),
        });
    }
    if let Err(error) = app.guardar_preferencias() {
        if !error
            .to_string()
            .contains("no se pudo determinar dónde guardar")
        {
            return Err(error);
        }
    }
    editor.nombre_nueva_plantilla.clear();
    Ok(())
}

/// Altura reservada en la base de la ventana para los controles del pie común.
const ALTURA_RESERVADA_PIE_MODAL: f32 = 48.0;

/// Dibuja pestañas y contenido; no realiza lecturas ni escrituras persistentes.
fn dibujar_contenido(
    app: &mut AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    idioma: Idioma,
    ui: &mut egui::Ui,
) -> AccionProyectoIa {
    ui.label(Texto::ProyectoIaAyudaInicial.en(idioma));
    ui.horizontal_wrapped(|ui| {
        for pestana in PestanaProyectoIa::TODAS {
            ui.selectable_value(&mut editor.pestana, pestana, pestana.titulo(idioma));
        }
    });
    ui.separator();
    let altura_pestana = (ui.available_height() - ALTURA_RESERVADA_PIE_MODAL).max(100.0);
    let mut accion = ui
        .allocate_ui_with_layout(
            egui::vec2(ui.available_width(), altura_pestana),
            egui::Layout::top_down(egui::Align::Min),
            |ui| match editor.pestana {
                PestanaProyectoIa::Proyecto => dibujar_proyecto(app, idioma, ui),
                PestanaProyectoIa::Instrucciones => dibujar_instrucciones(app, editor, idioma, ui),
                PestanaProyectoIa::Plantillas => dibujar_plantillas(app, editor, idioma, ui),
                PestanaProyectoIa::VistaCompleta => dibujar_vista_completa(app, editor, idioma, ui),
            },
        )
        .inner;
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(RichText::new(Texto::ProyectoIaIdiomaDocumento.en(idioma)).strong());
        let idioma_previo = editor.idioma_documento;
        egui::ComboBox::from_id_salt("idioma_documento_proyecto_ia")
            .selected_text(editor.idioma_documento.nombre_para_interfaz())
            .show_ui(ui, |ui| {
                for candidato in Idioma::TODOS {
                    ui.selectable_value(
                        &mut editor.idioma_documento,
                        candidato,
                        candidato.nombre_para_interfaz(),
                    );
                }
            });
        if editor.idioma_documento != idioma_previo {
            accion = AccionProyectoIa::CambiarIdiomaDocumento(editor.idioma_documento);
        }
        if ui
            .button(RichText::new(Texto::SesionAgenteGuardarReglas.en(idioma)).strong())
            .clicked()
        {
            accion = AccionProyectoIa::GuardarReglas;
        }
        if ui.button(Texto::BotonCerrar.en(idioma)).clicked() {
            accion = AccionProyectoIa::Cerrar;
        }
    });
    accion
}

/// Edita los cuatro datos que pertenecen al mapa y registra una sola mutación semántica.
fn dibujar_proyecto(
    app: &mut AplicacionMapaMental,
    idioma: Idioma,
    ui: &mut egui::Ui,
) -> AccionProyectoIa {
    let proyecto = app.mapa_mut().proyecto_para_editar();
    let mut cambio = false;
    let mut guardar = false;
    ScrollArea::vertical().show(ui, |ui| {
        ui.heading(Texto::ModalVisionDelCreadorY.en(idioma));
        ui.label(Texto::ModalEstaInformacionSeColocara.en(idioma));
        ui.label(RichText::new(Texto::ModalQueHasIntentadoPlasmar.en(idioma)).strong());
        cambio |= ui
            .add(
                egui::TextEdit::multiline(&mut proyecto.creator_vision)
                    .desired_rows(5)
                    .hint_text(Texto::ModalExplicaConTusPropias.en(idioma)),
            )
            .changed();
        ui.label(RichText::new(Texto::ModalQueObjetivosYResultados.en(idioma)).strong());
        cambio |= ui
            .add(
                egui::TextEdit::multiline(&mut proyecto.project_goals)
                    .desired_rows(4)
                    .hint_text(Texto::ModalEntregablesEsperadosMetricasDe.en(idioma)),
            )
            .changed();
        ui.label(RichText::new(Texto::ModalPublicoObjetivoContextoDe.en(idioma)).strong());
        cambio |= ui
            .add(
                egui::TextEdit::singleline(&mut proyecto.target_audience_or_context)
                    .hint_text(Texto::ModalEjDesarrolladoresRustEmprendedores.en(idioma)),
            )
            .changed();
        ui.label(RichText::new(Texto::ModalAutorCreador.en(idioma)).strong());
        cambio |= ui.text_edit_singleline(&mut proyecto.author).changed();
        guardar = ui
            .button(RichText::new(Texto::ModalGuardarMetadatosDelProyecto.en(idioma)).strong())
            .clicked();
    });
    if cambio {
        proyecto.marcar_modificado();
    }
    if guardar {
        AccionProyectoIa::GuardarProyecto
    } else {
        AccionProyectoIa::Ninguna
    }
}

/// Dibuja las reglas editables y el consentimiento separado de cada fuente nativa.
fn dibujar_instrucciones(
    app: &AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    idioma: Idioma,
    ui: &mut egui::Ui,
) -> AccionProyectoIa {
    // Misma ruta que consultan «Conectar con mis IAs» y el aviso de error (A-2): un único punto de
    // verdad en el dominio, no una canonicalización propia de esta pantalla.
    let ruta_mostrada =
        crate::conectores::ruta_de_trabajo_para_interfaz(&app.agentes().espacio_de_trabajo)
            .unwrap_or_else(|| crate::ui::limpiar_ruta_para_interfaz(&editor.raiz_proyecto));
    let mut accion = AccionProyectoIa::Ninguna;
    ScrollArea::vertical().show(ui, |ui| {
        ui.label(RichText::new(Texto::ProyectoIaCarpetaTrabajo.en(idioma)).strong());
        ui.monospace(&ruta_mostrada);
        ui.label(Texto::ProyectoIaExplicacionCarpetaTrabajo.en(idioma));
        ui.add_space(4.0);
        ui.label(RichText::new(Texto::SesionAgenteReglas.en(idioma)).strong());
        ui.add(
            egui::TextEdit::multiline(&mut editor.reglas)
                .desired_rows(12)
                .desired_width(f32::INFINITY),
        );
        if ui
            .button(Texto::ProyectoIaRestaurarPlantilla.en(idioma))
            .clicked()
        {
            editor.reglas = Texto::ProyectoIaPlantillaRecomendadaContenido
                .en(editor.idioma_documento)
                .to_owned();
        }
        ui.separator();
        ui.label(RichText::new(Texto::SesionAgenteFuentes.en(idioma)).strong());
        if editor.fuentes.is_empty() {
            ui.label(Texto::ProyectoIaFuentesVacias.en(idioma));
            ui.monospace(crate::sesiones_agentes::NOMBRES_FUENTES_NATIVAS.join(", "));
            ui.monospace(&ruta_mostrada);
        }
        for fuente in &editor.fuentes {
            egui::CollapsingHeader::new(fuente.nombre).show(ui, |ui| {
                ui.monospace(crate::ui::limpiar_ruta_para_interfaz(&fuente.ruta));
                let rotulo_estado = match fuente.estado {
                    EstadoFuenteNativa::Nueva => Texto::ProyectoIaFuenteNueva,
                    EstadoFuenteNativa::AceptadaSinCambios => Texto::ProyectoIaFuenteAceptada,
                    EstadoFuenteNativa::Modificada => Texto::ProyectoIaFuenteModificada,
                };
                ui.label(RichText::new(rotulo_estado.en(idioma)).strong());
                if let Some(anterior) = &fuente.contenido_anterior {
                    ui.label(RichText::new(Texto::ProyectoIaVersionAnterior.en(idioma)).strong());
                    ui.label(anterior);
                }
                ui.label(RichText::new(Texto::ProyectoIaVersionActual.en(idioma)).strong());
                ui.label(&fuente.contenido);
                if !fuente.incorporada
                    && ui
                        .button(Texto::ProyectoIaAceptarFuente.en(idioma))
                        .clicked()
                {
                    accion = AccionProyectoIa::AceptarFuente(fuente.nombre.to_string());
                }
            });
        }
    });
    accion
}

/// Permite elegir plantillas de encargo (de serie o de usuario), aplicarlas explícitamente
/// al encargo y guardar el encargo actual como plantilla global de usuario (P3-B).
fn dibujar_plantillas(
    app: &AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    idioma: Idioma,
    ui: &mut egui::Ui,
) -> AccionProyectoIa {
    let mut accion = AccionProyectoIa::Ninguna;
    let plantillas_usuario = &app.presentacion().preferencias_ref().plantillas_usuario;

    let mut opciones: Vec<String> = PlantillaEncargo::TODAS
        .iter()
        .map(|p| p.titulo_interfaz(editor.idioma_documento).to_string())
        .collect();
    for pu in plantillas_usuario {
        opciones.push(pu.nombre.clone());
    }

    if editor.indice_plantilla_seleccionada >= opciones.len() {
        editor.indice_plantilla_seleccionada = 0;
    }

    let texto_seleccionado = if !opciones.is_empty() {
        opciones[editor.indice_plantilla_seleccionada].as_str()
    } else {
        ""
    };

    ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("selector_plantillas_encargo_ia")
                .selected_text(texto_seleccionado)
                .show_ui(ui, |ui| {
                    for (i, opcion) in opciones.iter().enumerate() {
                        ui.selectable_value(&mut editor.indice_plantilla_seleccionada, i, opcion);
                    }
                });

            if ui
                .button(RichText::new(Texto::ProyectoIaAplicarPlantilla.en(idioma)).strong())
                .clicked()
            {
                if editor.indice_plantilla_seleccionada < PlantillaEncargo::TODAS.len() {
                    let plantilla = PlantillaEncargo::TODAS[editor.indice_plantilla_seleccionada];
                    editor.encargo =
                        plantilla.contenido(app.mapa().proyecto(), editor.idioma_documento);
                } else {
                    let idx_usuario =
                        editor.indice_plantilla_seleccionada - PlantillaEncargo::TODAS.len();
                    if let Some(pu) = plantillas_usuario.get(idx_usuario) {
                        editor.encargo = pu.contenido.clone();
                    }
                }
            }
        });

        ui.add_space(4.0);
        ui.label(RichText::new(Texto::SesionAgenteEncargo.en(idioma)).strong());
        ui.add(
            egui::TextEdit::multiline(&mut editor.encargo)
                .desired_rows(10)
                .desired_width(f32::INFINITY),
        );

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut editor.nombre_nueva_plantilla)
                    .hint_text(Texto::ProyectoIaNombrePlantillaHint.en(idioma))
                    .desired_width(220.0),
            );
            let puede_guardar = !editor.nombre_nueva_plantilla.trim().is_empty()
                && !editor.encargo.trim().is_empty();
            if ui
                .add_enabled(
                    puede_guardar,
                    egui::Button::new(Texto::ProyectoIaGuardarComoPlantilla.en(idioma)),
                )
                .clicked()
            {
                accion = AccionProyectoIa::GuardarComoPlantilla(
                    editor.nombre_nueva_plantilla.trim().to_owned(),
                );
            }
        });
    });

    accion
}

/// Presenta los cuatro bloques y el texto exacto a partir del único compositor de dominio.
fn dibujar_vista_completa(
    app: &AplicacionMapaMental,
    editor: &EstadoEditorProyectoIa,
    idioma: Idioma,
    ui: &mut egui::Ui,
) -> AccionProyectoIa {
    let borrador = BorradorSesion {
        agente: crate::conectores::IdAgente::CodexCli,
        contrato_mmcelt: crate::ai_export::contrato_mmcelt_para_agente(editor.idioma_documento),
        reglas_proyecto: editor.reglas.clone(),
        contexto_mapa: crate::ai_export::exportar_contexto_mapa_para_agente(
            app.mapa().proyecto(),
            editor.idioma_documento,
        ),
        encargo: editor.encargo.clone(),
        fuentes_nativas: editor.fuentes.clone(),
    };
    match crate::sesiones_agentes::componer_prompt_validado(&borrador) {
        Ok(compuesto) => {
            for (rotulo, contenido) in [
                (Texto::SesionAgenteContrato, compuesto.contrato()),
                (Texto::SesionAgenteReglas, compuesto.reglas_proyecto()),
                (Texto::SesionAgenteContexto, compuesto.contexto_mapa()),
                (Texto::SesionAgenteEncargo, compuesto.encargo()),
            ] {
                egui::CollapsingHeader::new(RichText::new(rotulo.en(idioma)).strong()).show(
                    ui,
                    |ui| {
                        ui.label(contenido);
                    },
                );
            }
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                ui.monospace(compuesto.texto());
                ui.small(format!("SHA-256: {:02x?}", compuesto.huella().bytes()));
            });
        }
        Err(error) => {
            ui.colored_label(
                ui.visuals().error_fg_color,
                error.mensaje_usuario().en(idioma),
            );
        }
    }
    AccionProyectoIa::Ninguna
}

/// Expone el mismo dibujante al arnés sin abrir una ventana nativa ni aplicar sus acciones.
#[cfg(test)]
pub(crate) fn dibujar_contenido_para_prueba(
    app: &mut AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    idioma: Idioma,
    ui: &mut egui::Ui,
) -> AccionProyectoIa {
    dibujar_contenido(app, editor, idioma, ui)
}

/// Aplica una acción al editor desde las pruebas sin simular una ventana nativa.
#[cfg(test)]
pub(crate) fn aplicar_accion_para_prueba(
    app: &mut AplicacionMapaMental,
    editor: &mut EstadoEditorProyectoIa,
    accion: AccionProyectoIa,
    idioma: Idioma,
) -> bool {
    aplicar_accion(app, editor, accion, idioma)
}

/// Reconstruye el editor desde el estado real para comprobar su reapertura.
#[cfg(test)]
pub(crate) fn cargar_editor_para_prueba(
    app: &AplicacionMapaMental,
) -> AppResult<EstadoEditorProyectoIa> {
    cargar_editor(app)
}
