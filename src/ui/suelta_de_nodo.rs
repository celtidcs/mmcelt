//! # Soltar un nodo encima de otro (`ui/suelta_de_nodo.rs`)
//!
//! Cuando el usuario arrastra un nodo y lo suelta con su centro encima de otra tarjeta, no está
//! claro qué quiere: colgarlo de ese nodo, relacionarlos, o solo dejarlo cerca. En vez de
//! adivinar, se le ofrece un mini-menú en el punto de suelta:
//!
//! | Opción | Qué hace |
//! |---|---|
//! | Hacer hijo | Cuelga el nodo del destino, con toda su descendencia |
//! | Hacer hermano | Lo cuelga del padre del destino, justo detrás de él (PH-1007-1) |
//! | Conectar con enlace | Crea una conexión cruzada hacia el destino y devuelve el nodo a su sitio; no si ya son padre e hijo o ya están conectados (PH-1007-2) |
//! | Mover aquí sin tapar | Lo deja junto al destino, en el hueco libre más cercano |
//! | Cancelar | Lo devuelve a donde estaba (también `Esc` o un clic fuera) |
//!
//! La decisión ([`resolver`]) trabaja sobre el [`Proyecto`] y se prueba sin ventana; el dibujo
//! del menú solo traduce clics en una [`OpcionDeSuelta`].

use crate::aplicacion::AplicacionMapaMental;
use crate::model::{ModoDisposicion, MovimientoRechazado, Proyecto, TipoRelacion};
use crate::textos::Texto;
use egui::Pos2;
use uuid::Uuid;

/// Una suelta encima de otro nodo que espera a que el usuario elija qué hacer.
#[derive(Debug, Clone, PartialEq)]
pub struct SueltaPendiente {
    /// El nodo que se arrastró.
    pub arrastrado: Uuid,
    /// El nodo sobre el que cayó.
    pub destino: Uuid,
    /// Dónde estaba el arrastrado antes de agarrarlo, para devolverlo.
    pub posicion_original: [f32; 2],
    /// Dónde lo soltó el usuario, en coordenadas del lienzo.
    pub punto: [f32; 2],
    /// Dónde se ve ese punto en pantalla, que es donde se abre el menú.
    pub en_pantalla: [f32; 2],
}

/// Lo que el usuario puede elegir en el menú de suelta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcionDeSuelta {
    /// Colgar el arrastrado del destino.
    HacerHijo,
    /// Colgar el arrastrado del padre del destino, justo detrás de él.
    HacerHermano,
    /// Crear una conexión cruzada arrastrado → destino y devolver el arrastrado a su sitio.
    Conectar,
    /// Dejar el arrastrado junto al destino sin solaparse con nada.
    MoverSinTapar,
    /// Devolver el arrastrado a su sitio sin cambiar nada más.
    Cancelar,
}

/// Aplica al mapa la opción elegida.
///
/// # Parámetros
/// - `proyecto`: el mapa, con el arrastrado todavía en el punto de suelta.
/// - `suelta`: qué se soltó, dónde y desde dónde.
/// - `opcion`: lo que eligió el usuario.
///
/// # Errores
/// [`MovimientoRechazado`] si «Hacer hijo» rompería el árbol o «Conectar» no encuentra los
/// nodos. En ese caso el nodo vuelve a su posición original y nada más cambia.
pub fn resolver(
    proyecto: &mut Proyecto,
    suelta: &SueltaPendiente,
    opcion: OpcionDeSuelta,
) -> Result<(), MovimientoRechazado> {
    match opcion {
        OpcionDeSuelta::HacerHijo => {
            if let Err(motivo) = proyecto.hacer_hijo_de(suelta.arrastrado, suelta.destino) {
                colocar(proyecto, suelta.arrastrado, suelta.posicion_original);
                return Err(motivo);
            }
            if proyecto.layout_mode == ModoDisposicion::FreeDrag {
                // Recién colgado, queda encima de su padre: se aparta lo justo para verse.
                dejar_junto_al_destino(proyecto, suelta);
            } else {
                crate::layout::aplicar_disposicion_automatica(proyecto);
            }
            Ok(())
        }
        OpcionDeSuelta::HacerHermano => {
            if let Err(motivo) = proyecto.hacer_hermano_de(suelta.arrastrado, suelta.destino) {
                colocar(proyecto, suelta.arrastrado, suelta.posicion_original);
                return Err(motivo);
            }
            if proyecto.layout_mode == ModoDisposicion::FreeDrag {
                dejar_junto_al_destino(proyecto, suelta);
            } else {
                crate::layout::aplicar_disposicion_automatica(proyecto);
            }
            Ok(())
        }
        OpcionDeSuelta::Conectar => {
            colocar(proyecto, suelta.arrastrado, suelta.posicion_original);
            if !proyecto.conexion_cruzada_admitida(suelta.arrastrado, suelta.destino) {
                return Err(MovimientoRechazado::YaEsSuPadre);
            }
            proyecto
                .anadir_conexion_cruzada(
                    suelta.arrastrado,
                    suelta.destino,
                    String::new(),
                    TipoRelacion::default(),
                )
                .map(|_| ())
                .ok_or(MovimientoRechazado::NodoInexistente)
        }
        OpcionDeSuelta::MoverSinTapar => {
            dejar_junto_al_destino(proyecto, suelta);
            Ok(())
        }
        OpcionDeSuelta::Cancelar => {
            colocar(proyecto, suelta.arrastrado, suelta.posicion_original);
            Ok(())
        }
    }
}

/// Coloca el arrastrado en el hueco libre más cercano al punto de suelta, junto al destino.
fn dejar_junto_al_destino(proyecto: &mut Proyecto, suelta: &SueltaPendiente) {
    let libre = crate::layout::posicion_libre_junto_a(
        proyecto,
        suelta.arrastrado,
        suelta.destino,
        suelta.punto,
    );
    colocar(proyecto, suelta.arrastrado, libre);
}

/// Pone un nodo en una posición, marcando el mapa como modificado si la posición cambia.
fn colocar(proyecto: &mut Proyecto, id: Uuid, posicion: [f32; 2]) {
    let Some(nodo) = proyecto.nodes.get_mut(&id) else {
        return;
    };
    if nodo.pos != posicion {
        nodo.pos = posicion;
        proyecto.marcar_modificado();
    }
}

/// Atiende la suelta de este fotograma y el menú que haya pendiente.
///
/// Se llama en cada fotograma, después de dibujar el lienzo:
///
/// 1. Si el lienzo acaba de soltar un nodo que **se movió de verdad** y cuyo centro cayó
///    encima de otra tarjeta, deja pendiente el menú. Un simple clic también agarra y suelta,
///    y no puede abrir un menú solo porque la tarjeta ya estuviera encima de otra.
/// 2. Si hay un menú pendiente, lo dibuja y aplica lo que se elija. `Esc` y un clic fuera del
///    menú equivalen a «Cancelar». «Hacer hijo» aparece deshabilitado, con la explicación al
///    pasar por encima, cuando el árbol no lo admite.
///
/// # Parámetros
/// - `app`: el estado; de aquí sale la suelta y aquí se aplica la opción.
/// - `ctx`: contexto de `egui`, donde se dibuja el menú flotante.
pub(crate) fn atender_la_suelta(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    if let Some((id, en_pantalla)) = app
        .lienzo_mut()
        .actualizar_vista()
        .nodo_recien_soltado
        .take()
    {
        anotar_si_cayo_encima_de_otro(app, id, en_pantalla);
    }
    let Some(suelta) = app.lienzo().vista().suelta_pendiente.clone() else {
        return;
    };
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        elegir(app, &suelta, OpcionDeSuelta::Cancelar);
        return;
    }

    let idioma = app.idioma();
    let disponibles = opciones_disponibles(app.mapa().proyecto(), &suelta);
    let menu = egui::Area::new(egui::Id::new("menu_de_suelta"))
        .order(egui::Order::Foreground)
        .fixed_pos(Pos2::new(suelta.en_pantalla[0], suelta.en_pantalla[1]))
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style())
                .show(ui, |ui| botones_del_menu(ui, idioma, disponibles))
                .inner
        });

    if let Some(opcion) = menu.inner {
        elegir(app, &suelta, opcion);
    } else if menu.response.clicked_elsewhere() {
        elegir(app, &suelta, OpcionDeSuelta::Cancelar);
    }
}

/// Deja pendiente el menú si el nodo soltado se movió y su centro cayó encima de otro.
///
/// # Parámetros
/// - `app`: el estado, del que se lee el mapa y donde se anota la suelta.
/// - `id`: el nodo que se acaba de soltar.
/// - `en_pantalla`: dónde se ve, para abrir allí el menú.
fn anotar_si_cayo_encima_de_otro(app: &mut AplicacionMapaMental, id: Uuid, en_pantalla: Pos2) {
    let original = app.lienzo().vista().posicion_al_agarrar;
    let Some(punto) = app.mapa().proyecto().nodes.get(&id).map(|n| n.pos) else {
        return;
    };
    if punto == original {
        return;
    }
    if let Some(destino) = crate::layout::nodo_bajo_el_centro(app.mapa().proyecto(), id) {
        app.lienzo_mut().actualizar_vista().suelta_pendiente = Some(SueltaPendiente {
            arrastrado: id,
            destino,
            posicion_original: original,
            punto,
            en_pantalla: [en_pantalla.x, en_pantalla.y],
        });
    }
}

/// Qué opciones del menú de suelta admite el mapa ahora mismo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcionesDisponibles {
    /// El árbol admite colgar el arrastrado del destino.
    pub hijo: bool,
    /// El árbol admite colgar el arrastrado del padre del destino.
    pub hermano: bool,
    /// Una conexión cruzada entre los dos no repite la jerarquía ni otra conexión.
    pub conexion: bool,
}

/// Pregunta al modelo, sin cambiar nada, qué se puede hacer con esta suelta.
///
/// # Parámetros
/// - `proyecto`: el mapa, que no se modifica.
/// - `suelta`: qué nodo se soltó y sobre cuál.
///
/// # Devuelve
/// Qué opciones aparecen habilitadas en el menú.
pub fn opciones_disponibles(proyecto: &Proyecto, suelta: &SueltaPendiente) -> OpcionesDisponibles {
    OpcionesDisponibles {
        hijo: proyecto
            .comprobar_que_se_puede_colgar(suelta.arrastrado, suelta.destino)
            .is_ok(),
        hermano: proyecto
            .comprobar_que_puede_ser_hermano(suelta.arrastrado, suelta.destino)
            .is_ok(),
        conexion: proyecto.conexion_cruzada_admitida(suelta.arrastrado, suelta.destino),
    }
}

/// Por qué no se puede hacer una opción del menú de suelta, para el texto emergente del botón
/// deshabilitado y para la barra de estado si al aplicarla el mapa la rechaza.
///
/// Cada opción lleva su propia explicación: «Hacer hermano» usaba la de «Hacer hijo», que habla
/// de «ya cuelga de este nodo» y no describe el caso de dos hermanos (PH-1007-7).
///
/// # Devuelve
/// `None` para las opciones que siempre se pueden hacer.
pub fn explicacion_si_no_se_puede(opcion: OpcionDeSuelta) -> Option<Texto> {
    match opcion {
        OpcionDeSuelta::HacerHijo => Some(Texto::SueltaNoSePuedeHacerHijo),
        OpcionDeSuelta::HacerHermano => Some(Texto::SueltaNoSePuedeHacerHermano),
        OpcionDeSuelta::Conectar => Some(Texto::SueltaNoSePuedeConectar),
        OpcionDeSuelta::MoverSinTapar | OpcionDeSuelta::Cancelar => None,
    }
}

/// Los botones del menú: hacer hijo, hacer hermano y conectar, más mover y cancelar.
///
/// Las opciones que el mapa no admite aparecen deshabilitadas, con la explicación al pasar el
/// ratón.
///
/// # Devuelve
/// La opción pulsada en este fotograma, si alguna.
fn botones_del_menu(
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
    disponibles: OpcionesDisponibles,
) -> Option<OpcionDeSuelta> {
    let mut elegida = None;
    for (opcion, rotulo, disponible) in [
        (
            OpcionDeSuelta::HacerHijo,
            Texto::SueltaHacerHijo,
            disponibles.hijo,
        ),
        (
            OpcionDeSuelta::HacerHermano,
            Texto::SueltaHacerHermano,
            disponibles.hermano,
        ),
        (
            OpcionDeSuelta::Conectar,
            Texto::SueltaConectar,
            disponibles.conexion,
        ),
    ] {
        let mut boton = ui.add_enabled(disponible, egui::Button::new(rotulo.en(idioma)));
        if let Some(explicacion) = explicacion_si_no_se_puede(opcion) {
            boton = boton.on_disabled_hover_text(explicacion.en(idioma));
        }
        if boton.clicked() {
            elegida = Some(opcion);
        }
    }
    for (opcion, texto) in [
        (OpcionDeSuelta::MoverSinTapar, Texto::SueltaMoverSinTapar),
        (OpcionDeSuelta::Cancelar, Texto::SueltaCancelar),
    ] {
        if ui.button(texto.en(idioma)).clicked() {
            elegida = Some(opcion);
        }
    }
    elegida
}

/// Cierra el menú y aplica la opción, avisando en la barra de estado si no se pudo.
fn elegir(app: &mut AplicacionMapaMental, suelta: &SueltaPendiente, opcion: OpcionDeSuelta) {
    app.lienzo_mut().actualizar_vista().suelta_pendiente = None;
    let resultado = resolver(app.mapa_mut().proyecto_para_editar(), suelta, opcion);
    app.lienzo_mut().invalidar_indice_espacial();
    if resultado.is_err() {
        let explicacion =
            explicacion_si_no_se_puede(opcion).unwrap_or(Texto::SueltaNoSePuedeHacerHijo);
        app.establecer_estado(explicacion.en(app.idioma()));
    }
}
