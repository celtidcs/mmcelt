//! Reproducción pendiente de medir: un error no desaparece al llegar otro estado informativo.

/// El aviso conserva la operación y el motivo hasta que se cierre expresamente.
#[test]
fn un_error_permanece_visible_aunque_cambie_la_barra_informativa() {
    let (mut app, _ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    app.presentacion.preferencias.idioma = crate::textos::Idioma::Espanol;
    let error = super::AppError::RutaFueraDelProyectoConDetalle {
        ruta: "mapa-ajeno.mmcelt".into(),
        raiz: "proyecto-activo".into(),
    };
    let motivo = error.mensaje_usuario().en(crate::textos::Idioma::Espanol);
    app.reportar_error(&error, "Enviar al agente");
    if let Some((_, momento)) = &mut app.presentacion.mensaje_de_estado {
        *momento -= std::time::Duration::from_secs(9);
    }
    let textos = crate::arnes_interfaz::textos_de(&mut app, |app, ui| {
        app.dibujar_barra_de_estado(ui);
    });
    assert!(
        textos
            .iter()
            .any(|texto| texto.contains("Enviar al agente")),
        "operación perdida: {textos:?}"
    );
    assert!(
        textos.iter().any(|texto| texto.contains(&motivo)),
        "motivo perdido: {textos:?}"
    );
    for dato in ["mapa-ajeno.mmcelt", "proyecto-activo"] {
        assert!(textos.iter().any(|texto| texto.contains(dato)));
    }
    let cerrar = crate::textos::Texto::BotonCerrar.en(crate::textos::Idioma::Espanol);
    assert!(crate::arnes_interfaz::hacer_clic_en_texto(
        &mut app,
        |app, ui| super::super::dibujar_aviso_de_error(
            ui,
            &mut app.presentacion.avisos_de_error,
            app.presentacion.tema.peligro,
            cerrar,
        ),
        cerrar
    ));
    assert!(app.presentacion.avisos_de_error.is_empty());
}
