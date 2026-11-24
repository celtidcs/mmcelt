//! Orden de los hermanos al soltar un nodo: «Mover aquí sin tapar» entre hermanos también cambia
//! el orden, y el menú ya no ofrece una entrada aparte para hacerlo.

use crate::ui::suelta_de_nodo::{resolver, OpcionDeSuelta, SueltaPendiente};

/// Rótulos que tuvo la entrada «Colocar después», ya retirada del menú, en los seis idiomas. Se
/// guardan aquí como literales porque el texto ya no existe en `Texto`.
const ROTULOS_RETIRADOS: [&str; 6] = [
    "Colocar después",
    "Place after",
    "Placer après",
    "Danach platzieren",
    "Поместить после",
    "移到其后",
];

/// Una suelta del nodo `arrastrado` sobre `destino`, con el punto de suelta encima del destino.
fn suelta_sobre(
    proyecto: &crate::model::Proyecto,
    arrastrado: uuid::Uuid,
    destino: uuid::Uuid,
) -> SueltaPendiente {
    SueltaPendiente {
        arrastrado,
        destino,
        posicion_original: proyecto.nodes[&arrastrado].pos,
        punto: proyecto.nodes[&destino].pos,
        en_pantalla: [0.0, 0.0],
    }
}

/// Entre hermanos, el menú ofrece «Mover aquí sin tapar» y no pinta la entrada retirada en
/// ninguno de los seis idiomas. Las restricciones anteriores se conservan.
#[test]
fn la_suelta_entre_hermanos_no_ofrece_colocar_despues() {
    use crate::textos::{Idioma, Texto};

    for idioma in Idioma::TODOS {
        let (mut app, ctx, a, b) = super::aplicacion_con_mapa_para_soltar();
        app.presentacion.preferencias.idioma = idioma;
        let mut suelta = suelta_sobre(&app.mapa.proyecto, a, b);
        suelta.en_pantalla = [400.0, 200.0];
        app.lienzo.vista.suelta_pendiente = Some(suelta);
        super::fotograma_del_lienzo(&mut app, &ctx, Vec::new());
        let textos = super::fotograma_del_lienzo(&mut app, &ctx, Vec::new());
        let mover = Texto::SueltaMoverSinTapar.en(idioma);
        assert!(
            textos.iter().any(|texto| texto.contains(mover)),
            "en {idioma:?}, el arnés debe haber dibujado el menú con «{mover}»: {textos:?}"
        );
        for retirado in ROTULOS_RETIRADOS {
            assert!(
                !textos.iter().any(|texto| texto.contains(retirado)),
                "en {idioma:?}, el menú vuelve a ofrecer «{retirado}»: {textos:?}"
            );
        }
        assert_eq!(
            app.mapa.proyecto.comprobar_que_puede_ser_hermano(a, b),
            Err(crate::model::MovimientoRechazado::YaSonHermanos)
        );
    }
}

/// Entre hermanos, «Mover aquí sin tapar» deja el nodo sin tapar y además lo pone detrás del
/// destino, sin cambiar de padre ni perder descendientes; un solo deshacer lo devuelve todo.
#[test]
fn mover_sin_tapar_entre_hermanos_ordena_y_se_deshace_de_una_vez() {
    let (mut app, _) = crate::arnes_interfaz::aplicacion_de_prueba();
    let (mut proyecto, raiz, a, b, nieto) = super::mapa_para_soltar();
    let c = proyecto.anadir_hijo(raiz, "C");
    app.mapa.proyecto = proyecto;
    let observar = |app: &mut crate::aplicacion::AplicacionMapaMental| {
        app.mapa.historial.observar_si_cambio(
            &app.mapa.proyecto,
            app.mapa.proyecto.revision(),
            std::time::Instant::now(),
        );
    };
    observar(&mut app);
    let suelta = suelta_sobre(&app.mapa.proyecto, a, c);
    let original = suelta.posicion_original;
    resolver(
        app.mapa.proyecto_para_editar(),
        &suelta,
        OpcionDeSuelta::MoverSinTapar,
    )
    .expect("mover entre hermanos");
    observar(&mut app);
    assert_eq!(app.mapa.proyecto.nodes[&raiz].children, [b, c, a]);
    assert_eq!(app.mapa.proyecto.nodes[&a].parent_id, Some(raiz));
    assert_eq!(app.mapa.proyecto.nodes[&nieto].parent_id, Some(a));
    assert_ne!(
        app.mapa.proyecto.nodes[&a].pos, original,
        "el nodo se mueve"
    );
    assert!(
        !super::caja_de(&app.mapa.proyecto, a).intersects(super::caja_de(&app.mapa.proyecto, c)),
        "sin tapar significa sin tapar al destino"
    );
    app.deshacer();
    assert_eq!(app.mapa.proyecto.nodes[&raiz].children, [a, b, c]);
    assert_eq!(app.mapa.proyecto.nodes[&a].pos, original);
}

/// Soltar el último hermano sobre el primero lo lleva hacia atrás, delante del intermedio.
#[test]
fn mover_sin_tapar_lleva_un_hermano_hacia_atras() {
    let (mut proyecto, raiz, a, b, _) = super::mapa_para_soltar();
    let c = proyecto.anadir_hijo(raiz, "C");
    let suelta = suelta_sobre(&proyecto, c, a);
    resolver(&mut proyecto, &suelta, OpcionDeSuelta::MoverSinTapar).expect("mover hacia atrás");
    assert_eq!(proyecto.nodes[&raiz].children, [a, c, b]);
}

/// En un modo automático, cambiar el orden entre hermanos recoloca el mapa entero según el
/// nuevo orden.
#[test]
fn en_un_modo_automatico_mover_entre_hermanos_recoloca_el_mapa() {
    let (mut proyecto, raiz, a, b, _) = super::mapa_para_soltar();
    let c = proyecto.anadir_hijo(raiz, "C");
    proyecto.layout_mode = crate::model::ModoDisposicion::HorizontalTree;
    let suelta = suelta_sobre(&proyecto, a, c);
    resolver(&mut proyecto, &suelta, OpcionDeSuelta::MoverSinTapar).expect("mover entre hermanos");
    assert_eq!(proyecto.nodes[&raiz].children, [b, c, a]);
    let recolocado = super::recolocado_por_la_disposicion(&proyecto);
    for (id, nodo) in &proyecto.nodes {
        assert_eq!(
            nodo.pos, recolocado.nodes[id].pos,
            "«{}» no está donde lo pone la disposición automática",
            nodo.title
        );
    }
}

/// Sobre un nodo que no es su hermano, «Mover aquí sin tapar» solo lo deja sin tapar: no cambia
/// de padre ni toca el orden de ninguna lista.
#[test]
fn mover_sin_tapar_sobre_un_no_hermano_no_toca_el_orden() {
    let (mut proyecto, raiz, a, b, nieto) = super::mapa_para_soltar();
    let c = proyecto.anadir_hijo(raiz, "C");
    let ordenes: Vec<_> = [raiz, a, b, c, nieto]
        .iter()
        .map(|id| proyecto.nodes[id].children.clone())
        .collect();
    let suelta = suelta_sobre(&proyecto, b, nieto);
    resolver(&mut proyecto, &suelta, OpcionDeSuelta::MoverSinTapar)
        .expect("mover sobre un nodo que no es hermano");
    assert_eq!(proyecto.nodes[&b].parent_id, Some(raiz));
    let despues: Vec<_> = [raiz, a, b, c, nieto]
        .iter()
        .map(|id| proyecto.nodes[id].children.clone())
        .collect();
    assert_eq!(despues, ordenes, "ninguna lista de hijos cambia");
    assert!(
        !super::caja_de(&proyecto, b).intersects(super::caja_de(&proyecto, nieto)),
        "sin tapar significa sin tapar al destino"
    );
}

/// Mover el último hermano después del primero conserva al hermano intermedio detrás.
#[test]
fn colocar_despues_mueve_un_hermano_hacia_atras() {
    let (mut proyecto, raiz, a, b, _) = super::mapa_para_soltar();
    let c = proyecto.anadir_hijo(raiz, "C");
    assert_eq!(proyecto.colocar_despues(c, a), Ok(true));
    assert_eq!(proyecto.nodes[&raiz].children, [a, c, b]);
}

/// Repetir una posición existente no crea una revisión nueva.
#[test]
fn colocar_despues_en_la_posicion_actual_no_cambia_el_mapa() {
    let (mut proyecto, raiz, a, b, _) = super::mapa_para_soltar();
    let revision = proyecto.revision();
    assert_eq!(proyecto.colocar_despues(b, a), Ok(false));
    assert_eq!(proyecto.nodes[&raiz].children, [a, b]);
    assert_eq!(proyecto.revision(), revision);
}
