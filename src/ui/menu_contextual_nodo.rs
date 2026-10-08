//! # Menú contextual de un nodo (`ui/menu_contextual_nodo.rs`)
//!
//! Clic derecho sin arrastrar sobre una tarjeta: el nodo queda seleccionado y aparece, en ese
//! punto, un menú con lo que se suele hacer con él. Arrastrar con el botón derecho sigue moviendo la cámara: `egui` solo da por
//! hecho un clic si el puntero no superó el umbral de arrastre.
//!
//! El lienzo solo anota dónde cayó el clic; aquí se decide sobre qué nodo, se dibuja el menú y
//! cada opción se traduce en una [`AccionDelMenu`], que [`aplicar`] ejecuta por las mismas
//! operaciones que el teclado y el inspector.

use crate::aplicacion::AplicacionMapaMental;
use crate::model::{EstadoNodo, EstadoRevision, PrioridadNodo, RolNodo};
use crate::textos::{sin_dos_puntos, Idioma, Texto};
use egui::Pos2;
use uuid::Uuid;

/// Un menú contextual abierto sobre un nodo.
#[derive(Debug, Clone, PartialEq)]
pub struct MenuContextual {
    /// El nodo sobre el que se hizo clic derecho.
    pub nodo: Uuid,
    /// Dónde se abrió, en pantalla.
    pub en_pantalla: [f32; 2],
}

/// Lo que se puede pedir desde el menú contextual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccionDelMenu {
    /// Añadir un hijo al nodo, como `Tab`.
    AnadirHijo,
    /// Añadir un hermano al nodo, como `Enter`.
    AnadirHermano,
    /// Abrir el cuadro de conexión cruzada con el nodo como origen.
    CrearConexion,
    /// Abrir el título del nodo en edición, como `F2`.
    EditarTitulo,
    /// Eliminar el nodo con su descendencia, como `Supr`.
    Eliminar,
    /// Cambiar su estado de madurez.
    Estado(EstadoNodo),
    /// Cambiar su prioridad.
    Prioridad(PrioridadNodo),
    /// Cambiar su estado de control humano.
    Revision(EstadoRevision),
    /// Cambiar su rol (PH-1007-3).
    Rol(RolNodo),
}

/// Ejecuta una acción del menú sobre su nodo.
///
/// El nodo se selecciona antes de actuar: las operaciones de alta, edición y borrado son las
/// mismas que usan los atajos, y esas trabajan sobre la selección. Así el menú no puede acabar
/// actuando sobre otro nodo que estuviera seleccionado antes del clic derecho.
///
/// # Parámetros
/// - `app`: el estado.
/// - `nodo`: el nodo del menú.
/// - `accion`: lo elegido.
pub fn aplicar(app: &mut AplicacionMapaMental, nodo: Uuid, accion: AccionDelMenu) {
    app.mapa_mut().seleccionar_nodo(Some(nodo));
    match accion {
        AccionDelMenu::AnadirHijo => app.anadir_hijo_al_seleccionado(),
        AccionDelMenu::AnadirHermano => app.anadir_hermano_al_seleccionado(),
        AccionDelMenu::CrearConexion => {
            app.lienzo_mut().editar_conexion().origen = Some(nodo);
            app.presentacion_mut().ventanas().modal_conexion_cruzada = true;
        }
        AccionDelMenu::EditarTitulo => app.abrir_el_titulo_en_edicion(),
        AccionDelMenu::Eliminar => app.eliminar_nodo_seleccionado(),
        AccionDelMenu::Estado(valor) => {
            app.mapa_mut()
                .proyecto_para_editar()
                .fijar_estado(nodo, valor);
        }
        AccionDelMenu::Prioridad(valor) => {
            app.mapa_mut()
                .proyecto_para_editar()
                .fijar_prioridad(nodo, valor);
        }
        AccionDelMenu::Rol(valor) => {
            app.mapa_mut().proyecto_para_editar().fijar_rol(nodo, valor);
        }
        AccionDelMenu::Revision(valor) => {
            app.mapa_mut()
                .proyecto_para_editar()
                .fijar_revision(nodo, valor);
        }
    }
}

/// Atiende el clic derecho de este fotograma y el menú que haya abierto.
///
/// Se llama en cada fotograma, después de dibujar el lienzo. Con el mapa en solo lectura no se
/// abre: casi todas sus opciones cambian el mapa.
///
/// # Parámetros
/// - `app`: el estado.
/// - `ctx`: contexto de `egui`, donde se dibuja el menú flotante.
pub(crate) fn atender_el_menu_contextual(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    // El clic que abre el menú es también un clic fuera de él —el menú todavía no tiene ni
    // tamaño—, así que en el fotograma en que se abre no puede cerrarse por ese motivo.
    let recien_abierto = match app.lienzo_mut().actualizar_vista().clic_derecho.take() {
        Some((en_el_lienzo, en_pantalla)) => {
            abrir_si_cayo_en_un_nodo(app, en_el_lienzo, en_pantalla)
        }
        None => false,
    };
    let Some(menu) = app.lienzo().vista().menu_contextual.clone() else {
        return;
    };
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.lienzo_mut().actualizar_vista().menu_contextual = None;
        return;
    }

    let idioma = app.idioma();
    let Some(datos) = DatosDelMenu::de(app, menu.nodo) else {
        app.lienzo_mut().actualizar_vista().menu_contextual = None;
        return;
    };
    let respuesta = egui::Area::new(egui::Id::new("menu_contextual_nodo"))
        .order(egui::Order::Foreground)
        .fixed_pos(Pos2::new(menu.en_pantalla[0], menu.en_pantalla[1]))
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style())
                .show(ui, |ui| opciones_del_menu(ui, idioma, datos))
                .inner
        });

    if let Some(accion) = respuesta.inner {
        app.lienzo_mut().actualizar_vista().menu_contextual = None;
        aplicar(app, menu.nodo, accion);
    } else if !recien_abierto
        && respuesta.response.clicked_elsewhere()
        && !hay_un_submenu_abierto(ctx)
    {
        app.lienzo_mut().actualizar_vista().menu_contextual = None;
    }
}

/// Selecciona el nodo bajo el clic derecho y abre su menú, si el clic cayó en uno.
///
/// No pregunta por el modo de solo lectura: el lienzo ya no anota el clic derecho si el mapa
/// no admite cambios, que es una sola guarda en vez de dos.
///
/// # Devuelve
/// `true` si se ha abierto el menú.
fn abrir_si_cayo_en_un_nodo(
    app: &mut AplicacionMapaMental,
    en_el_lienzo: Pos2,
    en_pantalla: Pos2,
) -> bool {
    let Some(nodo) = crate::layout::nodo_en_el_punto(
        app.mapa().proyecto(),
        [en_el_lienzo.x, en_el_lienzo.y],
        None,
    ) else {
        return false;
    };
    app.mapa_mut().seleccionar_nodo(Some(nodo));
    app.lienzo_mut().actualizar_vista().menu_contextual = Some(MenuContextual {
        nodo,
        en_pantalla: [en_pantalla.x, en_pantalla.y],
    });
    true
}

/// Si hay abierto algún submenú (Estado, Prioridad, Control humano).
///
/// Un clic dentro de un submenú cae fuera del área del menú principal; sin esta pregunta,
/// elegir una prioridad cerraría el menú antes de aplicarla.
fn hay_un_submenu_abierto(ctx: &egui::Context) -> bool {
    egui::Popup::is_any_open(ctx)
}

/// Lo que el menú necesita saber del nodo para dibujarse.
#[derive(Clone, Copy)]
struct DatosDelMenu {
    /// Si es la raíz, que no admite hermanos.
    es_la_raiz: bool,
    /// Estado actual, para marcarlo en el submenú.
    estado: EstadoNodo,
    /// Prioridad actual.
    prioridad: PrioridadNodo,
    /// Estado de control humano actual.
    revision: EstadoRevision,
    /// Rol actual.
    rol: RolNodo,
}

impl DatosDelMenu {
    /// Lee los datos del nodo, o `None` si ya no existe.
    fn de(app: &AplicacionMapaMental, id: Uuid) -> Option<Self> {
        let proyecto = app.mapa().proyecto();
        let nodo = proyecto.nodes.get(&id)?;
        Some(Self {
            es_la_raiz: id == proyecto.root_id,
            estado: nodo.status,
            prioridad: nodo.priority,
            revision: nodo.review_status,
            rol: nodo.role,
        })
    }
}

/// Dibuja las opciones del menú y devuelve la elegida en este fotograma, si alguna.
///
/// Los rótulos y atajos son los mismos del menú «Edición», para que el usuario reconozca cada
/// opción esté donde esté.
fn opciones_del_menu(
    ui: &mut egui::Ui,
    idioma: Idioma,
    datos: DatosDelMenu,
) -> Option<AccionDelMenu> {
    let mut elegida = None;
    ui.label(
        egui::RichText::new(Texto::MenuContextualTitulo.en(idioma))
            .small()
            .weak(),
    );
    let con_atajo = |rotulo: Texto, atajo: &str| format!("{}\t{}", rotulo.en(idioma), atajo);
    if ui
        .button(con_atajo(
            Texto::EdicionAnadirHijo,
            &format!("Tab / {}", Texto::TeclaInsertar.en(idioma)),
        ))
        .clicked()
    {
        elegida = Some(AccionDelMenu::AnadirHijo);
    }
    if ui
        .add_enabled(
            !datos.es_la_raiz,
            egui::Button::new(con_atajo(Texto::EdicionAnadirHermano, "Enter")),
        )
        .clicked()
    {
        elegida = Some(AccionDelMenu::AnadirHermano);
    }
    if ui
        .button(Texto::EdicionCrearConexionCruzada.en(idioma))
        .clicked()
    {
        elegida = Some(AccionDelMenu::CrearConexion);
    }
    let editar = format!("{} / F2", Texto::TeclaEspacio.en(idioma));
    if ui
        .button(con_atajo(Texto::EdicionEditarTextoDelNodo, &editar))
        .clicked()
    {
        elegida = Some(AccionDelMenu::EditarTitulo);
    }
    if ui
        .button(con_atajo(
            Texto::EdicionEliminarNodo,
            Texto::TeclaSuprimir.en(idioma),
        ))
        .clicked()
    {
        elegida = Some(AccionDelMenu::Eliminar);
    }
    ui.separator();
    elegida = elegida.or(submenus_de_clasificacion(ui, idioma, datos));
    elegida
}

/// Los submenús Estado, Prioridad, Control humano y Rol, con la variante actual marcada.
fn submenus_de_clasificacion(
    ui: &mut egui::Ui,
    idioma: Idioma,
    datos: DatosDelMenu,
) -> Option<AccionDelMenu> {
    let mut elegida = None;
    ui.menu_button(sin_dos_puntos(Texto::InspectorEstado.en(idioma)), |ui| {
        for valor in EstadoNodo::TODOS {
            if ui
                .selectable_label(valor == datos.estado, valor.nombre_para_interfaz(idioma))
                .clicked()
            {
                elegida = Some(AccionDelMenu::Estado(valor));
            }
        }
    });
    ui.menu_button(sin_dos_puntos(Texto::InspectorPrioridad.en(idioma)), |ui| {
        for valor in PrioridadNodo::TODOS {
            if ui
                .selectable_label(valor == datos.prioridad, valor.nombre_para_interfaz(idioma))
                .clicked()
            {
                elegida = Some(AccionDelMenu::Prioridad(valor));
            }
        }
    });
    ui.menu_button(
        sin_dos_puntos(Texto::InspectorControlHumano.en(idioma)),
        |ui| {
            for valor in EstadoRevision::TODOS {
                if ui
                    .selectable_label(valor == datos.revision, valor.nombre_para_interfaz(idioma))
                    .clicked()
                {
                    elegida = Some(AccionDelMenu::Revision(valor));
                }
            }
        },
    );
    ui.menu_button(sin_dos_puntos(Texto::InspectorRol.en(idioma)), |ui| {
        for valor in RolNodo::TODOS {
            if ui
                .selectable_label(valor == datos.rol, valor.nombre_para_interfaz(idioma))
                .clicked()
            {
                elegida = Some(AccionDelMenu::Rol(valor));
            }
        }
    });
    elegida
}
