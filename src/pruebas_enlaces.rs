//! Pruebas mecánicas: las pruebas de enlaces no deben pasar por omitir la creación del enlace.

/// No se convierte un fallo de crear el enlace en éxito de la prueba de seguridad.
#[test]
fn las_pruebas_de_enlaces_exigen_crear_el_enlace_real() {
    let fuente = include_str!("audit_checks.rs");
    for inicio in [
        "std::os::windows::fs::symlink_dir(",
        "std::os::windows::fs::symlink_file(",
        "std::os::unix::fs::symlink(",
    ] {
        let mut encontrados = 0;
        for linea in fuente.lines().filter(|linea| linea.contains(inicio)) {
            encontrados += 1;
            assert!(
                !linea.contains(".is_ok()"),
                "crear el enlace es requisito de la prueba, no una omisión permitida: {linea}"
            );
        }
        assert!(
            encontrados > 0,
            "calibración: no se localizaron las pruebas de {inicio}"
        );
    }
}
