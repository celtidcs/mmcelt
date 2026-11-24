//! Pruebas mecánicas: la relación explícita entre padre e hijo no se confunde con la jerarquía.

/// Separación mínima requerida en el tramo central entre la curva de jerarquía y la conexión cruzada.
/// Origen: margen visual mínimo para garantizar legibilidad y distinción de trazos en conexiones padre-hijo.
pub const SEPARACION_MINIMA_CENTRAL_EN_PIXELES: f32 = 8.0;

/// Evalúa un punto a lo largo de una curva de Bézier cúbica para el parámetro `t` en [0.0, 1.0].
fn punto_en_bezier(puntos: &[egui::Pos2; 4], t: f32) -> egui::Pos2 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    let x = uuu * puntos[0].x
        + 3.0 * uu * t * puntos[1].x
        + 3.0 * u * tt * puntos[2].x
        + ttt * puntos[3].x;
    let y = uuu * puntos[0].y
        + 3.0 * uu * t * puntos[1].y
        + 3.0 * u * tt * puntos[2].y
        + ttt * puntos[3].y;

    egui::pos2(x, y)
}

/// Recoge curvas reales pintadas, también cuando egui las agrupa.
fn recoger_curvas(forma: &egui::Shape, curvas: &mut Vec<[egui::Pos2; 4]>) {
    match forma {
        egui::Shape::CubicBezier(curva) => curvas.push(curva.points),
        egui::Shape::Vec(formas) => {
            for forma in formas {
                recoger_curvas(forma, curvas);
            }
        }
        _ => {}
    }
}

/// La relación padre-hijo importada se conserva y se separa visualmente de la jerarquía.
/// Verifica ambos sentidos horizontales (derecha e izquierda) y alturas iguales y distintas.
#[test]
fn una_relacion_explicita_padre_hijo_no_se_superpone_a_la_jerarquia() {
    // Casos a probar: derecha/mismo nivel, izquierda/mismo nivel, derecha/altura distinta, izquierda/altura distinta.
    for posicion in [
        [400.0, 0.0],
        [-400.0, 0.0],
        [400.0, 150.0],
        [-400.0, -150.0],
    ] {
        let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
        let mut proyecto = super::Proyecto::nuevo_vacio("Raíz");
        let raiz = proyecto.root_id;
        proyecto.nodes.get_mut(&raiz).expect("raíz").pos = [0.0, 0.0];
        let hijo = super::Nodo::new("Hijo", Some(raiz), posicion);
        let id_hijo = hijo.id;
        proyecto
            .nodes
            .get_mut(&raiz)
            .expect("raíz")
            .children
            .push(id_hijo);
        proyecto.nodes.insert(id_hijo, hijo);
        let relacion = proyecto
            .anadir_conexion_cruzada(
                raiz,
                id_hijo,
                "Relación importada",
                crate::model::TipoRelacion::Dependencia,
            )
            .expect("el modelo conserva la relación explícita");
        app.mapa.proyecto = proyecto;
        app.lienzo.indice_espacial_del_lienzo.invalidar();
        let salida = crate::arnes_interfaz::ejecutar_fotograma_completo(
            &mut app,
            &ctx,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1920.0, 1080.0)),
        );
        let mut curvas = Vec::new();
        for forma in &salida.shapes {
            recoger_curvas(&forma.shape, &mut curvas);
        }
        assert!(
            curvas.len() >= 2,
            "deben dibujarse jerarquía y relación explícita: {curvas:?}"
        );

        // Muestrear en el tramo central (t = 0.5) y verificar que existe separación mínima efectiva
        let p_central_0 = punto_en_bezier(&curvas[0], 0.5);
        let p_central_1 = punto_en_bezier(&curvas[1], 0.5);
        let separacion_central = (p_central_0.y - p_central_1.y).abs();

        assert!(
            separacion_central >= SEPARACION_MINIMA_CENTRAL_EN_PIXELES,
            "las curvas se solapan en el centro para posición {posicion:?}: separación {separacion_central} < {SEPARACION_MINIMA_CENTRAL_EN_PIXELES}"
        );

        // Muestrear además en t = 0.45 y t = 0.55 para asegurar el tramo central completo
        for t in [0.45, 0.55] {
            let p0 = punto_en_bezier(&curvas[0], t);
            let p1 = punto_en_bezier(&curvas[1], t);
            let sep = (p0.y - p1.y).abs();
            assert!(
                sep >= SEPARACION_MINIMA_CENTRAL_EN_PIXELES,
                "las curvas se solapan en t={t} para posición {posicion:?}: separación {sep} < {SEPARACION_MINIMA_CENTRAL_EN_PIXELES}"
            );
        }

        let texto = serde_json::to_string(&app.mapa.proyecto).expect("guardar mapa");
        let recargado: super::Proyecto = serde_json::from_str(&texto).expect("releer mapa");
        assert!(recargado
            .connections
            .iter()
            .any(|conexion| conexion.id == relacion));
        assert_eq!(recargado.nodes[&id_hijo].parent_id, Some(raiz));
    }
}
