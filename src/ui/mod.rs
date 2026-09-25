//! # Presentación gráfica de MMCelt
//!
//! Los módulos de esta carpeta dibujan el lienzo, los paneles y las ventanas de `egui`. El estado
//! raíz pertenece a [`crate::aplicacion`], que es un módulo hermano: la presentación no obtiene
//! acceso privilegiado a sus invariantes por la jerarquía de módulos.

pub mod ai_modal;
pub mod ayuda_textos;
pub mod canvas;
pub mod conexiones_modal;
pub mod dialogos;
pub(crate) mod estado_agentes;
pub(crate) mod estado_persistencia;
pub mod help_system;
pub mod proyecto_ia_modal;
pub mod sesion_agente_modal;
pub mod sidebar;
pub mod toolbar;

/// Devuelve la representación legible de una ruta para la interfaz gráfica, retirando
/// prefijos técnicos del sistema operativo (como el prefijo verbatim `\\?\` en Windows).
pub(crate) fn limpiar_ruta_para_interfaz(ruta: &std::path::Path) -> String {
    let texto = ruta.display().to_string();
    if let Some(resto) = texto.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{resto}")
    } else if let Some(resto) = texto.strip_prefix(r"\\?\") {
        resto.to_string()
    } else {
        texto
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn limpiar_ruta_retira_prefijo_verbatim_de_windows() {
        assert_eq!(
            limpiar_ruta_para_interfaz(Path::new(r"\\?\O:\pruebas_mmcelt\AGENTS.md")),
            r"O:\pruebas_mmcelt\AGENTS.md"
        );
    }

    #[test]
    fn limpiar_ruta_retira_prefijo_unc_de_windows() {
        assert_eq!(
            limpiar_ruta_para_interfaz(Path::new(r"\\?\UNC\servidor\recurso\AGENTS.md")),
            r"\\servidor\recurso\AGENTS.md"
        );
    }

    #[test]
    fn limpiar_ruta_conserva_rutas_estandar() {
        assert_eq!(
            limpiar_ruta_para_interfaz(Path::new(r"O:\pruebas_mmcelt\AGENTS.md")),
            r"O:\pruebas_mmcelt\AGENTS.md"
        );
    }
}
