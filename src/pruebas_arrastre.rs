//! Cobertura de pruebas: arrastre libre por eventos reales de egui y deshacer.

/// Mover a un espacio vacío conserva el desplazamiento y se deshace en un solo paso.
/// Verifica que el padre del nodo y el orden de los hermanos no se modifican.
#[test]
fn arrastrar_un_nodo_al_vacio_cambia_su_posicion_y_se_deshace() {
    let (mut app, ctx, nodo, _) = super::lienzo_preparado();
    let original = app.mapa.proyecto.nodes[&nodo].pos;
    let padre_original = app.mapa.proyecto.nodes[&nodo].parent_id;
    let hermanos_original = padre_original
        .and_then(|p| app.mapa.proyecto.nodes.get(&p))
        .map(|p| p.children.clone());

    let registrar_fotograma = |app: &mut crate::aplicacion::AplicacionMapaMental| {
        let revision = app.mapa.proyecto.revision();
        app.mapa.historial.observar_si_cambio(
            &app.mapa.proyecto,
            revision,
            std::time::Instant::now(),
        );
    };

    // Registro de fotograma base, tal como realiza eframe al final de cada ciclo de ui.
    registrar_fotograma(&mut app);

    let inicio = super::centro_en_pantalla(&app, nodo);
    let destino = inicio + egui::vec2(50.0, -180.0);
    let boton = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    super::fotograma_del_lienzo(
        &mut app,
        &ctx,
        vec![egui::Event::PointerMoved(inicio), boton(inicio, true)],
    );
    registrar_fotograma(&mut app);

    super::fotograma_del_lienzo(&mut app, &ctx, vec![egui::Event::PointerMoved(destino)]);
    registrar_fotograma(&mut app);

    super::fotograma_del_lienzo(&mut app, &ctx, vec![boton(destino, false)]);
    registrar_fotograma(&mut app);

    assert_ne!(
        app.mapa.proyecto.nodes[&nodo].pos, original,
        "el movimiento del ratón debe mover el nodo"
    );
    assert!(
        app.lienzo.vista.suelta_pendiente.is_none(),
        "el espacio está vacío"
    );

    // Comprueba que el arrastre a un área vacía no cambia el padre ni el orden de los hermanos.
    assert_eq!(
        app.mapa.proyecto.nodes[&nodo].parent_id, padre_original,
        "arrastrar a espacio vacío no debe cambiar el padre del nodo"
    );
    let hermanos_tras_arrastre = padre_original
        .and_then(|p| app.mapa.proyecto.nodes.get(&p))
        .map(|p| p.children.clone());
    assert_eq!(
        hermanos_tras_arrastre, hermanos_original,
        "arrastrar a espacio vacío no debe alterar el orden de los hermanos"
    );

    app.deshacer();
    assert_eq!(
        app.mapa.proyecto.nodes[&nodo].pos, original,
        "deshacer debe restaurar la posición anterior al arrastre"
    );
    assert_eq!(
        app.mapa.proyecto.nodes[&nodo].parent_id, padre_original,
        "deshacer debe conservar el padre del nodo"
    );
    let hermanos_tras_deshacer = padre_original
        .and_then(|p| app.mapa.proyecto.nodes.get(&p))
        .map(|p| p.children.clone());
    assert_eq!(
        hermanos_tras_deshacer, hermanos_original,
        "deshacer debe conservar el orden de los hermanos"
    );
}
