//! Regresión de la limpieza de configuraciones temporales.

/// El agente temporal no deja su configuración cuando termina su ámbito.
#[test]
fn el_agente_de_prueba_retira_su_carpeta_al_terminar() {
    let carpeta;
    {
        let agente = super::agente_de_prueba(
            "limpieza.json",
            crate::conectores::FormatoConfiguracion::Json,
        );
        carpeta = agente
            .ruta_configuracion
            .parent()
            .expect("carpeta")
            .to_path_buf();
        std::fs::write(&agente.ruta_configuracion, "{}").expect("configuración simulada");
        assert!(carpeta.exists(), "calibración: la carpeta existía");
    }
    let persiste = carpeta.exists();
    // El rojo también se limpia: no deja basura para otras ejecuciones.
    if persiste {
        std::fs::remove_dir_all(&carpeta).expect("limpiar la reproducción del defecto");
    }
    assert!(!persiste, "el agente de prueba dejó su carpeta al terminar");
}
