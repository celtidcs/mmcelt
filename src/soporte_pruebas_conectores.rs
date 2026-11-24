//! Agentes de prueba cuya configuración temporal se limpia al terminar su ámbito.

use crate::conectores::{Agente, FormatoConfiguracion, IdAgente};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

/// Mantiene juntos el agente simulado y la carpeta temporal que le pertenece.
pub(super) struct AgenteTemporal {
    /// Datos del agente, accesibles sin trasladar la propiedad de su carpeta.
    agente: Agente,
    /// Carpeta única creada por este auxiliar; nunca procede de datos externos.
    carpeta: PathBuf,
}

impl AgenteTemporal {
    /// Crea una configuración aislada y vacía, que se retira al destruir el auxiliar.
    pub(super) fn nuevo(nombre: &str, formato: FormatoConfiguracion) -> Self {
        let carpeta = std::env::temp_dir().join(format!(
            "mmcelt_pruebas_conectores_{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir(&carpeta).expect("crear carpeta de configuración de prueba");
        Self {
            agente: Agente {
                id: IdAgente::ClaudeCode,
                nombre: "Agente de prueba",
                ruta_configuracion: carpeta.join(nombre),
                formato,
                conectado: false,
                cli: None,
                espacio_registrado: None,
                existe_configuracion: false,
                cuenta: deteccion_agentes::Cuenta {
                    herramienta: deteccion_agentes::Herramienta::ClaudeCode,
                    carpeta: carpeta.clone(),
                    origen: deteccion_agentes::OrigenDeLaCuenta::PorDefecto,
                },
            },
            carpeta,
        }
    }
}

impl Deref for AgenteTemporal {
    type Target = Agente;
    fn deref(&self) -> &Self::Target {
        &self.agente
    }
}

impl DerefMut for AgenteTemporal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agente
    }
}

impl Drop for AgenteTemporal {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.carpeta) {
            if error.kind() != std::io::ErrorKind::NotFound {
                eprintln!("no se pudo retirar {}: {error}", self.carpeta.display());
            }
        }
    }
}
