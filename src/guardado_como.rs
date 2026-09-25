//! Gestión y evaluación de destinos para «Guardar como» (PH-0119-3).
//!
//! Desacopla de `aplicacion.rs` la lógica de decisión cuando el usuario guarda un mapa
//! en disco:
//! - Si el destino es la misma carpeta del proyecto activo, se guarda directamente sin aviso (C12-J).
//! - Si el destino es una carpeta distinta, se ofrecen dos salidas explícitas (C12-I):
//!   1. Guardar una copia aquí (el proyecto activo no se mueve).
//!   2. Guardar y trabajar en la carpeta nueva (persiste cambios previos y traslada la sesión).

use crate::error::AppResult;
use crate::AplicacionMapaMental;
use std::path::{Path, PathBuf};

/// Acción elegida por el usuario ante un guardado en otra carpeta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccionGuardarComo {
    /// Guarda una copia del archivo en la ruta destino, pero la sesión de trabajo y el proyecto
    /// activo continúan en su carpeta original.
    GuardarCopiaEnDestino,
    /// Resuelve y asegura los cambios del proyecto actual, guarda en la nueva carpeta y traslada
    /// la sesión activa para trabajar en la nueva carpeta.
    GuardarYTrabajarEnCarpetaNueva,
}

/// Clasificación del destino elegido en «Guardar como».
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TipoDestinoGuardarComo {
    /// La ruta destino está en la misma carpeta que el proyecto activo actual.
    MismaCarpeta,
    /// La ruta destino reside en una carpeta diferente a la del proyecto activo.
    OtraCarpeta {
        /// Carpeta física del nuevo destino.
        carpeta_nueva: PathBuf,
    },
}

impl TipoDestinoGuardarComo {
    /// Determina si este destino requiere confirmación del usuario por implicar otra carpeta.
    #[must_use]
    pub fn requiere_confirmacion(&self) -> bool {
        matches!(self, Self::OtraCarpeta { .. })
    }
}
/// Evalúa la relación entre la carpeta del proyecto activo y la nueva ruta destino elegida.
#[must_use]
pub fn evaluar_destino_guardado_como(
    carpeta_actual: Option<&Path>,
    nueva_ruta: &Path,
) -> TipoDestinoGuardarComo {
    let Some(carpeta_nueva) = nueva_ruta.parent() else {
        return TipoDestinoGuardarComo::MismaCarpeta;
    };

    let Some(actual) = carpeta_actual else {
        return TipoDestinoGuardarComo::MismaCarpeta;
    };

    let actual_canon = actual
        .canonicalize()
        .unwrap_or_else(|_| actual.to_path_buf());
    let nueva_canon = carpeta_nueva
        .canonicalize()
        .unwrap_or_else(|_| carpeta_nueva.to_path_buf());

    if actual_canon == nueva_canon {
        TipoDestinoGuardarComo::MismaCarpeta
    } else {
        TipoDestinoGuardarComo::OtraCarpeta {
            carpeta_nueva: carpeta_nueva.to_path_buf(),
        }
    }
}

/// Ejecuta la acción elegida en «Guardar como» sobre el estado de la aplicación.
pub fn ejecutar_guardado_como(
    app: &mut AplicacionMapaMental,
    nueva_ruta: &Path,
    accion: AccionGuardarComo,
) -> AppResult<()> {
    match accion {
        AccionGuardarComo::GuardarCopiaEnDestino => {
            // Guarda una copia del mapa en el nuevo destino sin alterar el proyecto activo
            crate::storage::guardar_proyecto_en_archivo(app.mapa().proyecto(), nueva_ruta)?;
            Ok(())
        }
        AccionGuardarComo::GuardarYTrabajarEnCarpetaNueva => {
            // 1. C12-K: Asegura los cambios pendientes en el archivo actual antes de cerrar/trasladar
            if let Some(ruta_previa) = app.persistencia().ruta_actual.clone() {
                if ruta_previa.exists() {
                    let _ = crate::storage::guardar_proyecto_en_archivo(
                        app.mapa().proyecto(),
                        &ruta_previa,
                    );
                }
            }
            // 2. Guarda el proyecto en el nuevo archivo
            crate::storage::guardar_proyecto_en_archivo(app.mapa().proyecto(), nueva_ruta)?;

            // 3. Fija la nueva ruta actual y el espacio de trabajo en la nueva carpeta
            app.anotar_guardado(nueva_ruta);
            if let Some(carpeta_nueva) = nueva_ruta.parent() {
                app.agentes_mut().espacio_de_trabajo = carpeta_nueva.to_path_buf();
            }
            Ok(())
        }
    }
}

/// Estado del aviso modal cuando se guarda como en una carpeta distinta a la del proyecto.
#[derive(Debug, Clone)]
pub struct AvisoGuardarComoOtraCarpeta {
    /// Archivo de destino donde se guardará el mapa.
    pub ruta_destino: PathBuf,
    /// Carpeta física que contiene el nuevo destino.
    pub carpeta_nueva: PathBuf,
}

/// Dibuja la ventana modal interactiva cuando se guarda como a una carpeta distinta.
pub fn dibujar_modal_guardar_como(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let Some(aviso) = app.presentacion().aviso_guardar_como_otra_carpeta.clone() else {
        return;
    };

    let idioma = app.idioma();
    let mut abierta = true;
    let mut accion: Option<AccionGuardarComo> = None;

    egui::Window::new(crate::textos::Texto::ModalGuardarComoOtraCarpetaTitulo.en(idioma))
        .open(&mut abierta)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .min_width(540.0)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new(
                crate::textos::Texto::ModalGuardarComoOtraCarpetaExplicacion.en(idioma),
            ));
            ui.add_space(8.0);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📁").size(16.0));
                    ui.label(
                        egui::RichText::new(crate::ui::limpiar_ruta_para_interfaz(
                            &aviso.carpeta_nueva,
                        ))
                        .monospace()
                        .strong(),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📄").size(16.0));
                    ui.label(
                        egui::RichText::new(crate::ui::limpiar_ruta_para_interfaz(
                            &aviso.ruta_destino,
                        ))
                        .monospace(),
                    );
                });
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(10.0);

            // Opción 1: Guardar una copia aquí (no traslada la sesión activa)
            if ui
                .button(crate::textos::Texto::BotonGuardarCopiaAqui.en(idioma))
                .clicked()
            {
                accion = Some(AccionGuardarComo::GuardarCopiaEnDestino);
            }

            ui.add_space(6.0);

            // Opción 2: Guardar y trabajar en la carpeta nueva (traslada sesión)
            if ui
                .button(
                    egui::RichText::new(
                        crate::textos::Texto::BotonGuardarYTrabajarEnNueva.en(idioma),
                    )
                    .strong(),
                )
                .clicked()
            {
                accion = Some(AccionGuardarComo::GuardarYTrabajarEnCarpetaNueva);
            }

            ui.add_space(4.0);
        });

    if !abierta {
        app.presentacion_mut().aviso_guardar_como_otra_carpeta = None;
        return;
    }

    if let Some(elegida) = accion {
        app.presentacion_mut().aviso_guardar_como_otra_carpeta = None;
        if let Err(e) = ejecutar_guardado_como(app, &aviso.ruta_destino, elegida) {
            app.reportar_error(
                &e,
                crate::textos::Texto::ContextoGuardarMapaMental.en(idioma),
            );
        } else {
            let aviso_msg = crate::textos::Texto::ModalGuardadoEn
                .en(idioma)
                .replace("{}", &aviso.ruta_destino.display().to_string());
            app.establecer_estado(aviso_msg);
        }
    }
}

/// Evalúa si el destino de «Guardar como» es otra carpeta y devuelve los datos del aviso modal si aplica.
#[must_use]
pub fn evaluar_destino_guardado_como_aviso(
    app: &AplicacionMapaMental,
    destino: &Path,
) -> Option<AvisoGuardarComoOtraCarpeta> {
    let actual = app
        .persistencia()
        .ruta_actual
        .as_deref()
        .and_then(|p| p.parent());
    let destino_tipo = evaluar_destino_guardado_como(actual, destino);
    if destino_tipo.requiere_confirmacion() {
        if let TipoDestinoGuardarComo::OtraCarpeta { carpeta_nueva } = destino_tipo {
            return Some(AvisoGuardarComoOtraCarpeta {
                ruta_destino: destino.to_path_buf(),
                carpeta_nueva,
            });
        }
    }
    None
}
