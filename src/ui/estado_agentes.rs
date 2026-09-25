//! Estado operativo de la comunicación con agentes de inteligencia artificial.
//!
//! Agrupa la sesión que la persona está preparando, el trabajador que realiza el envío y el
//! inventario de clientes disponibles. La estructura raíz de `eframe` ya no necesita conocer
//! cómo se inicializa ni qué campos forman conjuntamente este recorrido.

use crate::conectores::Agente;
use crate::envio_agente::{EnvioEnSegundoPlano, EstadoEnvio};
use crate::ui::sesion_agente_modal::EditorSesionAgente;
use std::path::PathBuf;

/// Encargo transitorio asociado exclusivamente al proyecto donde se redactó.
struct EncargoPreparado {
    /// Raíz canónica que identifica el ámbito del borrador mientras la aplicación está abierta.
    raiz_proyecto: PathBuf,
    /// Texto que se recuperará al reabrir la preparación o al elegir un agente.
    texto: String,
}

/// Estado completo del recorrido «Enviar a…» mientras la aplicación está abierta.
pub(crate) struct EstadoAgentes {
    /// Encargo preparado antes de elegir agente, aislado por la raíz donde se redactó.
    encargo_preparado: Option<EncargoPreparado>,
    /// Vista previa de un envío aún no confirmado; su presencia mantiene abierto el modal.
    pub(crate) editor_sesion: Option<EditorSesionAgente>,
    /// Fase de la confirmación, visible para impedir dobles clics y explicar la espera.
    pub(crate) envio: EstadoEnvio,
    /// Trabajador estable que prepara los archivos y abre la consola fuera del fotograma.
    pub(crate) trabajador: EnvioEnSegundoPlano,
    /// Agentes detectados en el equipo, o `None` mientras no se haya buscado.
    pub(crate) detectados: Option<Vec<Agente>>,
    /// Carpeta dentro de la cual podrán operar los agentes.
    pub(crate) espacio_de_trabajo: PathBuf,
}

impl EstadoAgentes {
    /// Construye el recorrido inactivo y arranca su único trabajador estable.
    pub(crate) fn nuevo(ctx: egui::Context, espacio_de_trabajo: PathBuf) -> Self {
        Self {
            encargo_preparado: None,
            editor_sesion: None,
            envio: EstadoEnvio::Inactivo,
            trabajador: EnvioEnSegundoPlano::nuevo(ctx),
            detectados: None,
            espacio_de_trabajo,
        }
    }

    /// Conserva un borrador para un proyecto concreto sin trasladarlo a otras carpetas.
    pub(crate) fn conservar_encargo_preparado(&mut self, raiz_proyecto: PathBuf, texto: String) {
        self.encargo_preparado = Some(EncargoPreparado {
            raiz_proyecto,
            texto,
        });
    }

    /// Devuelve el encargo de la raíz indicada o un borrador vacío si pertenece a otro proyecto.
    pub(crate) fn encargo_preparado_para(&self, raiz_proyecto: &std::path::Path) -> String {
        self.encargo_preparado
            .as_ref()
            .filter(|encargo| encargo.raiz_proyecto == raiz_proyecto)
            .map(|encargo| encargo.texto.clone())
            .unwrap_or_default()
    }
}
