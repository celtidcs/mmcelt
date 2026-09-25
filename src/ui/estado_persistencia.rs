//! Estado de persistencia y vigilancia asociado al mapa abierto.
//!
//! Reúne los datos que cambian conjuntamente al abrir, guardar, recuperar o vigilar un archivo.
//! La ruta, su última huella y el observador dejan así de ser campos independientes que puedan
//! trasladarse u olvidarse por separado en la raíz de la interfaz.

use crate::autoguardado::{ControlAutoguardado, Recuperacion};
use crate::carga_en_segundo_plano::CargadorExterno;
use crate::vigilante::VigilanteArchivo;
use std::path::PathBuf;

/// Estado durable y reactivo del archivo que representa el mapa activo.
pub(crate) struct EstadoPersistencia {
    /// Archivo asociado al mapa activo, si ya fue guardado o abierto desde disco.
    pub(crate) ruta_actual: Option<PathBuf>,
    /// Controla cuándo toca autoguardar y evita reescribir si nada ha cambiado.
    pub(crate) autoguardado: ControlAutoguardado,
    /// Carpeta donde esta ejecución guarda la copia de recuperación.
    pub(crate) carpeta_datos: Option<PathBuf>,
    /// Copia de una sesión anterior pendiente de una decisión de la persona.
    pub(crate) recuperacion: Option<Recuperacion>,
    /// Observador del archivo activo, cuando la vigilancia está encendida.
    pub(crate) vigilante: Option<VigilanteArchivo>,
    /// Trabajador que lee y valida cambios externos fuera del hilo gráfico.
    pub(crate) cargador_externo: CargadorExterno,
    /// Huella de la última versión escrita por esta aplicación.
    pub(crate) ultima_huella_guardada: Option<u64>,
    /// Señala un cambio externo retenido porque hay trabajo local sin guardar.
    pub(crate) cambio_externo_pendiente: bool,
}

impl EstadoPersistencia {
    /// Construye el estado inicial y arranca el único trabajador de carga externa.
    pub(crate) fn nuevo(
        ctx: egui::Context,
        autoguardado: ControlAutoguardado,
        carpeta_datos: Option<PathBuf>,
        recuperacion: Option<Recuperacion>,
    ) -> Self {
        Self {
            ruta_actual: None,
            autoguardado,
            carpeta_datos,
            recuperacion,
            vigilante: None,
            cargador_externo: CargadorExterno::nuevo(ctx),
            ultima_huella_guardada: None,
            cambio_externo_pendiente: false,
        }
    }
}
