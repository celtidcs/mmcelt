//! # Módulo de Lienzo Interactivo 2D (`ui/canvas.rs`)
//!
//! Implementa la renderización acelerada del plano infinito:
//! - Paneo suave y zoom continuo centrado en el cursor.
//! - Cuadrícula infinita con atenuación dinámica.
//! - Curvas vectoriales Bézier con color temático por nivel de profundidad.
//! - Dibujo de nodos estilizados con cajas de color, iconos de estado, indicadores de prioridad y control humano, notas y etiquetas.
//! - Edición de texto interactiva en el propio lienzo (in-place text edit).

use crate::aplicacion::AplicacionMapaMental;
use crate::layout::{calcular_curva_bezier, estimar_tamano_del_nodo};
use crate::model::{
    EstadoNodo, EstadoRevision, ModoDisposicion, PrioridadNodo, Proyecto, RevisionProyecto,
};
use egui::{
    epaint::CubicBezierShape, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, StrokeKind, Vec2,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Margen total que se reserva a los lados del título dentro de la caja del nodo.
///
/// Diez a la izquierda, donde empieza el texto, y otros tantos a la derecha para que no
/// llegue a tocar el borde ni se meta debajo del área de indicadores de la derecha.
const MARGEN_DEL_TEXTO: f32 = 20.0;

/// Hueco que se reserva a la derecha y por debajo de la tarjeta para sus botones.
///
/// El «+» se centra a diez píxeles por debajo del borde inferior y el círculo de plegar a
/// ocho a la derecha; ambos tienen siete de radio. Veinte cubre los dos con holgura, de modo
/// que el cursor puede ir de la tarjeta al botón sin pasar por ningún punto muerto.
const HUECO_DE_LOS_BOTONES: f32 = 20.0;

/// Cuánto se agranda la ventana al decidir qué nodos merece la pena dibujar.
///
/// Un nodo que asoma por el borde debe entrar entero, y al desplazar el mapa conviene que lo
/// que va a aparecer ya esté compuesto. Es holgado a propósito: basta con el ancho máximo de
/// un nodo, y pasarse solo cuesta dibujar unos pocos de más.
const MARGEN_DE_DIBUJADO: f32 = 400.0;

/// Lado de cada celda del índice espacial, en coordenadas del mapa.
const LADO_CELDA_ESPACIAL: f32 = 512.0;

/// Zoom mínimo del lienzo: el mapa a una quinta parte de su tamaño.
///
/// Por debajo, los nodos dejan de ser legibles y la vista sirve de poco.
const ZOOM_MINIMO: f32 = 0.2;

/// Zoom máximo del lienzo.
///
/// Por encima, cada nodo ocupa la pantalla y se pierde la visión de conjunto, que es la
/// razón de tener un mapa.
const ZOOM_MAXIMO: f32 = 3.5;

/// Cuánto zoom aplica cada muesca de la rueda del ratón.
///
/// El desplazamiento de la rueda se multiplica por este valor y se pasa por una
/// exponencial, de modo que el zoom avanza en proporción y no a saltos fijos: acercarse
/// desde muy lejos cuesta lo mismo que alejarse desde muy cerca.
const SENSIBILIDAD_DEL_ZOOM: f32 = 0.0015;

/// Grosor mínimo con el que se dibuja una conexión cruzada, en puntos.
const GROSOR_MINIMO_CONEXION: f32 = 1.0;

/// Grosor máximo de una conexión cruzada, en puntos.
///
/// Coincide en valor con [`ZOOM_MAXIMO`] por casualidad, no por relación: son dos cosas
/// distintas y por eso llevan nombres distintos.
const GROSOR_MAXIMO_CONEXION: f32 = 3.5;

/// Margen que se deja alrededor del mapa al encuadrarlo en la ventana.
///
/// Un mapa pegado a los bordes se lee mal y no deja claro que no siga más allá. El valor es
/// una proporción del área visible, no una cantidad fija, para que se vea igual de holgado
/// en un portátil que en el televisor para el que existe el módulo de escala.
const MARGEN_DEL_ENCUADRE: f32 = 0.92;

/// Tamaño de letra de la etiqueta que acompaña a una conexión cruzada, a zoom 1.
const LETRA_DE_LA_ETIQUETA_DE_CONEXION: f32 = 11.0;

/// Tamaño de letra del título de un nodo mientras se está editando, a zoom 1.
///
/// Es mayor que el de los adornos porque es texto que el usuario está escribiendo, no un
/// indicador: tiene que leerse cómodamente.
const LETRA_DEL_TITULO_EN_EDICION: f32 = 14.0;

/// Tamaño de letra del icono que avisa de que un nodo tiene notas, a zoom 1.
const LETRA_DEL_ICONO_DE_NOTAS: f32 = 11.0;

/// Tamaño de letra de las etiquetas que se muestran bajo el título del nodo, a zoom 1.
///
/// El más pequeño de la tarjeta: son información secundaria y no deben competir con el
/// título.
const LETRA_DE_LAS_ETIQUETAS: f32 = 10.0;

/// Tamaño de letra del «+» del botón de añadir un hijo, a zoom 1.
const LETRA_DEL_BOTON_ANADIR: f32 = 12.0;

/// Grosor del borde de un nodo seleccionado, a zoom 1.
///
/// Es el doble largo que el normal a propósito: el borde es lo único que distingue al nodo
/// seleccionado, y a zoom bajo una diferencia menor no se aprecia.
const GROSOR_DEL_BORDE_SELECCIONADO: f32 = 2.5;

/// Grosor del borde de un nodo que no está seleccionado, a zoom 1.
const GROSOR_DEL_BORDE_NORMAL: f32 = 1.2;

/// Grosor de la curva de una conexión cruzada, a zoom 1.
const GROSOR_DE_LA_CONEXION_CRUZADA: f32 = 2.2;

/// Grosor mínimo de cualquier trazo dibujado en el lienzo, en píxeles de pantalla.
///
/// Los grosores se multiplican por el zoom, así que sin un suelo un trazo desaparecería al
/// alejarse. Y sin un techo, al acercarse engordaría hasta comerse el nodo.
const GROSOR_MINIMO_DEL_TRAZO: f32 = 1.0;

/// Grosor máximo de cualquier trazo dibujado en el lienzo, en píxeles de pantalla.
const GROSOR_MAXIMO_DEL_TRAZO: f32 = 4.0;

/// Radio de las esquinas redondeadas de la tarjeta de un nodo, a zoom 1.
const RADIO_DE_LA_ESQUINA: f32 = 8.0;

/// Radio mínimo de esquina, en píxeles de pantalla.
///
/// Por debajo, la tarjeta parece un rectángulo cortado; por encima del máximo, una pastilla.
const RADIO_MINIMO_DE_LA_ESQUINA: f32 = 2.0;

/// Radio máximo de esquina, en píxeles de pantalla.
const RADIO_MAXIMO_DE_LA_ESQUINA: f32 = 16.0;

/// Calcula el zoom y el desplazamiento con los que un mapa cabe entero en la ventana.
///
/// # El problema que resuelve
///
/// «Centrar vista» ponía el zoom en 1,0 y el desplazamiento a cero, sin mirar el mapa: en un
/// mapa grande —y la disposición radial reparte las hojas sobre una circunferencia, así que
/// crece deprisa— se veía la tarjeta central y poco más, sin ninguna forma de abarcarlo
/// salvo alejarse a rueda hasta dar con él. No existía ningún «ajustar a la ventana».
///
/// # Parámetros
/// - `caja`: el rectángulo que ocupa el mapa, en coordenadas del lienzo.
/// - `visible`: el tamaño del área de pantalla donde se dibuja.
///
/// # Devuelve
/// El zoom que hay que aplicar y el desplazamiento que deja el mapa centrado.
///
/// El zoom nunca pasa de 1,0: un mapa de tres nodos no se amplía hasta llenar la pantalla,
/// porque eso desorienta más que ayudar. Por abajo se limita a [`ZOOM_MINIMO`], el mismo
/// suelo que la rueda del ratón; un mapa que ni así quepa se verá recortado, pero centrado.
pub(crate) fn encuadre_para_ver_el_mapa(caja: Rect, visible: Vec2) -> (f32, Vec2) {
    // El desplazamiento lleva el centro del mapa al centro de la pantalla, porque la
    // conversión a pantalla es `centro + (posición + desplazamiento) * zoom`.
    let desplazamiento = -caja.center().to_vec2();

    let ancho = caja.width().max(f32::EPSILON);
    let alto = caja.height().max(f32::EPSILON);

    if visible.x <= 0.0 || visible.y <= 0.0 {
        // Todavía no se ha dibujado ningún fotograma y no se sabe cuánto sitio hay. Se
        // conserva el tamaño natural, que es lo que hacía siempre.
        return (1.0, desplazamiento);
    }

    let cabe = (visible.x / ancho).min(visible.y / alto) * MARGEN_DEL_ENCUADRE;
    let zoom = cabe.clamp(ZOOM_MINIMO, 1.0);

    (zoom, desplazamiento)
}

/// Lo que el puntero está haciendo en el fotograma que se dibuja.
///
/// Se lee una sola vez, al principio, en lugar de preguntarle al contexto dentro del bucle:
/// así todas las tarjetas deciden con exactamente la misma foto del ratón.
#[derive(Clone, Copy)]
struct PulsacionesDelPuntero {
    /// Dónde está el cursor, si es que está sobre la ventana.
    posicion: Option<Pos2>,
    /// El botón principal sigue bajado, que es lo que sostiene un arrastre.
    boton_pulsado: bool,
    /// Se ha completado un clic: pulsar y soltar.
    clic_completo: bool,
    /// El botón se acaba de bajar **en este fotograma**, que es cuando se engancha un nodo.
    recien_pulsado: bool,
    /// Doble clic, que abre la edición del título.
    doble_clic: bool,
}

/// Lee de una vez todo lo que hace el puntero en el fotograma.
///
/// El ratón se lee crudo del contexto, así que hay que descartar lo que no es del lienzo
/// (PH-1007-6). El clic es de otro si el puntero está fuera de la zona del mapa —el inspector y
/// la barra comparten capa con el lienzo, y detrás de ellos sigue habiendo nodos que no se ven—
/// o si encima hay otra capa interactiva: la lista de un desplegable, un menú, una ventana. Sin
/// este filtro, pulsar un desplegable del inspector con un nodo oculto detrás lo seleccionaba y el
/// valor se aplicaba al nodo nuevo. Un arrastre ya empezado conserva la posición, para que pasar
/// por encima de algo no lo suelte.
///
/// # Parámetros
/// - `ctx`: el contexto de `egui` del fotograma en curso.
/// - `capa_del_lienzo`: la capa en la que se dibuja el lienzo.
/// - `zona_del_mapa`: el rectángulo en pantalla que ocupa el lienzo.
///
/// # Devuelve
/// La foto del ratón con la que van a decidir todas las tarjetas.
fn leer_las_pulsaciones(
    ctx: &egui::Context,
    capa_del_lienzo: egui::LayerId,
    zona_del_mapa: Rect,
) -> PulsacionesDelPuntero {
    let posicion = ctx.input(|i| i.pointer.hover_pos());
    let tapado = posicion.is_some_and(|punto| {
        !zona_del_mapa.contains(punto) || ctx.layer_id_at(punto) != Some(capa_del_lienzo)
    });
    let boton_pulsado = ctx.input(|i| i.pointer.primary_down());
    let recien_pulsado = ctx.input(|i| i.pointer.primary_pressed());
    let empezo_en_el_mapa = pulsacion_empezada_en_el_mapa(ctx, recien_pulsado, !tapado);
    PulsacionesDelPuntero {
        posicion: posicion.filter(|_| !tapado || (boton_pulsado && empezo_en_el_mapa)),
        boton_pulsado,
        clic_completo: !tapado && empezo_en_el_mapa && ctx.input(|i| i.pointer.primary_clicked()),
        recien_pulsado: !tapado && recien_pulsado,
        doble_clic: !tapado
            && empezo_en_el_mapa
            && ctx.input(|i| {
                i.pointer
                    .button_double_clicked(egui::PointerButton::Primary)
            }),
    }
}

/// Recuerda si la última pulsación del botón principal empezó sobre el mapa.
///
/// El lienzo solo responde a la ventana en la que se pulsó: pulsar en el inspector y soltar
/// encima de un nodo no es un clic del mapa. `egui` olvida dónde empezó la pulsación al soltar,
/// así que se anota en su memoria temporal en el fotograma en que se pulsa.
///
/// # Parámetros
/// - `ctx`: el contexto de `egui`, en cuya memoria temporal se guarda la marca.
/// - `recien_pulsado`: el botón se ha bajado en este fotograma.
/// - `sobre_el_mapa`: el puntero está ahora sobre el mapa y no hay nada encima.
///
/// # Devuelve
/// `true` si la pulsación en curso (o la que acaba de terminar) empezó sobre el mapa.
fn pulsacion_empezada_en_el_mapa(
    ctx: &egui::Context,
    recien_pulsado: bool,
    sobre_el_mapa: bool,
) -> bool {
    let marca = egui::Id::new("pulsacion_empezada_en_el_mapa");
    if recien_pulsado {
        ctx.data_mut(|datos| datos.insert_temp(marca, sobre_el_mapa));
    }
    ctx.data(|datos| datos.get_temp::<bool>(marca))
        .unwrap_or(false)
}

/// Lo que no cambia mientras se dibuja un fotograma del lienzo.
///
/// Existe por dos motivos. El primero, que `dibujar_conexiones_de_la_rama` arrastraba ocho
/// parámetros y hubo que silenciarle el aviso de `clippy`; agrupar lo que siempre viaja junto
/// es la corrección, y no callar el aviso. El segundo, que `dibujar_lienzo` medía más de
/// seiscientas líneas: al partirlo, cada trozo necesitaba estos mismos valores y sin agrupar
/// habrían vuelto a repetirse uno por uno en cada firma.
///
/// Se desestructura al entrar en cada función, de modo que los cuerpos siguen nombrando las
/// cosas como siempre.
#[derive(Clone, Copy)]
struct Fotograma<'a> {
    /// Dónde se pinta.
    painter: &'a egui::Painter,
    /// Nivel de acercamiento, al que van a escala todos los tamaños.
    zoom: f32,
    /// La ventana con su margen: fuera de ella no se dibuja nada.
    area_visible: Rect,
    /// De las coordenadas del mapa a las de la pantalla.
    a_pantalla: &'a dyn Fn(Pos2) -> Pos2,
    /// De las coordenadas de la pantalla a las del mapa.
    a_mundo: &'a dyn Fn(Pos2) -> Pos2,
    /// Nodos que no deben verse por tener algún ancestro plegado.
    ocultos: &'a HashSet<Uuid>,
    /// Lo que hace el ratón en este fotograma.
    puntero: PulsacionesDelPuntero,
    /// Si el mapa admite cambios en este fotograma (no está en solo lectura).
    ///
    /// Se pregunta **una vez** por fotograma y viaja con él: antes el lienzo lo repreguntaba
    /// en cada sitio que cambiaba el mapa (arrastre, botón «+», hijo rápido), y cada sitio
    /// nuevo era una guarda más que alguien podía olvidar (barrera P7, «solo-lectura»).
    editable: bool,
}

/// Lo que el usuario ha pedido sobre el lienzo y todavía está sin aplicar.
///
/// Se recoge mientras se dibuja y se aplica al terminar: dentro del bucle no se puede,
/// porque el mapa está prestado para leerlo.
#[derive(Default)]
struct AccionesDelLienzo {
    /// Nodo sobre el que se ha hecho clic.
    nodo_pulsado: Option<Uuid>,
    /// Nodo sobre el que se ha hecho doble clic, que abre su edición.
    nodo_con_doble_clic: Option<Uuid>,
    /// Nodo cuyo botón de plegar se ha pulsado.
    nodo_a_plegar: Option<Uuid>,
    /// Nodo al que se le ha pedido un hijo con el botón «+».
    nodo_para_hijo_rapido: Option<Uuid>,
    /// Nodo cuyo título se ha terminado de editar, con el texto nuevo.
    nodo_a_renombrar: Option<(Uuid, String)>,
    /// Texto emergente del icono que tiene el puntero encima, y dónde está el puntero.
    texto_emergente: Option<(String, Pos2)>,
}

/// Índice de las tarjetas por región, reconstruido solo cuando cambia el mapa.
#[derive(Default)]
pub(crate) struct IndiceEspacialLienzo {
    version: Option<(Uuid, RevisionProyecto, usize)>,
    celdas: HashMap<(i32, i32), Vec<Uuid>>,
    orden: HashMap<Uuid, usize>,
}

impl IndiceEspacialLienzo {
    /// Invalida una posición transitoria que no cuenta como cambio persistente del mapa.
    pub(crate) fn invalidar(&mut self) {
        self.version = None;
    }

    /// Reconstruye las celdas únicamente cuando la identidad o la revisión han cambiado.
    fn actualizar(&mut self, proyecto: &Proyecto) {
        let version = (proyecto.root_id, proyecto.revision(), proyecto.nodes.len());
        if self.version == Some(version) {
            return;
        }
        self.celdas.clear();
        self.orden.clear();
        for (posicion, (&id, nodo)) in proyecto.nodes.iter().enumerate() {
            self.orden.insert(id, posicion);
            let (ancho, alto) = estimar_tamano_del_nodo(&nodo.title, nodo.tags.len());
            let minimo_x = celda_de(nodo.pos[0] - ancho / 2.0);
            let maximo_x = celda_de(nodo.pos[0] + ancho / 2.0);
            let minimo_y = celda_de(nodo.pos[1] - alto / 2.0);
            let maximo_y = celda_de(nodo.pos[1] + alto / 2.0);
            for x in minimo_x..=maximo_x {
                for y in minimo_y..=maximo_y {
                    self.celdas.entry((x, y)).or_default().push(id);
                }
            }
        }
        self.version = Some(version);
    }

    /// Devuelve solo las tarjetas cuyas celdas tocan la región visible del mundo.
    fn candidatos(&self, fotograma: &Fotograma, arrastrado: Option<Uuid>) -> Vec<Uuid> {
        let esquina_a = (fotograma.a_mundo)(fotograma.area_visible.min);
        let esquina_b = (fotograma.a_mundo)(fotograma.area_visible.max);
        let mut unicos = HashSet::new();
        for x in celda_de(esquina_a.x.min(esquina_b.x))..=celda_de(esquina_a.x.max(esquina_b.x)) {
            for y in celda_de(esquina_a.y.min(esquina_b.y))..=celda_de(esquina_a.y.max(esquina_b.y))
            {
                if let Some(ids) = self.celdas.get(&(x, y)) {
                    unicos.extend(ids.iter().copied());
                }
            }
        }
        if let Some(id) = arrastrado {
            unicos.insert(id);
        }
        let mut candidatos: Vec<_> = unicos.into_iter().collect();
        candidatos.sort_unstable_by_key(|id| self.orden.get(id).copied().unwrap_or(usize::MAX));
        candidatos
    }
}

fn celda_de(coordenada: f32) -> i32 {
    (coordenada / LADO_CELDA_ESPACIAL).floor() as i32
}

/// Indicador visual representativo de una dimensión relevante del nodo en el lienzo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndicadorVisual {
    /// Símbolo o emoji que se dibuja en la tarjeta.
    pub simbolo: &'static str,
    /// Si proviene de la prioridad estratégica (`true`) o de la supervisión humana (`false`).
    pub es_prioridad: bool,
}

/// Decide de forma pura qué indicadores visuales deben dibujarse en la tarjeta del nodo (C19-A, C19-C).
///
/// Implementa la filosofía de lienzo limpio:
/// - Prioridad: solo se dibuja si es `Critica` (🔥) o `Alta` (⚡).
/// - Control humano: solo se dibuja si es `GeneradoPorIA` (🤖), `AprobadoPorHumano` (🛡️)
///   o `RequiereCorreccion` (⚠️). `PendienteRevision` no se dibuja para evitar saturación.
///
/// # Parámetros
/// - `prioridad`: urgencia estratégica del nodo.
/// - `revision`: estado de supervisión humana.
///
/// # Devuelve
/// Vector con los indicadores aplicables en orden consistente: primero prioridad (si aplica),
/// luego control humano (si aplica).
pub fn calcular_indicadores_del_nodo(
    prioridad: PrioridadNodo,
    revision: EstadoRevision,
) -> Vec<IndicadorVisual> {
    let mut indicadores = Vec::new();
    if let Some(simbolo) = prioridad.simbolo() {
        indicadores.push(IndicadorVisual {
            simbolo,
            es_prioridad: true,
        });
    }
    if let Some(simbolo) = revision.simbolo_en_nodo() {
        indicadores.push(IndicadorVisual {
            simbolo,
            es_prioridad: false,
        });
    }
    indicadores
}

/// Calcula el margen horizontal reservado a la derecha del título para alojar los indicadores (C19-D).
///
/// Garantiza cero solapamiento con el texto del título: el ancho asignado al galley del título
/// se reduce exactamente por esta cantidad cuando hay indicadores activos.
pub fn reserva_horizontal_de_indicadores(cantidad_indicadores: usize, zoom: f32) -> f32 {
    if cantidad_indicadores == 0 {
        0.0
    } else {
        cantidad_indicadores as f32 * 16.0 * zoom + 4.0 * zoom
    }
}

/// Lo que hace falta saber de un nodo para dibujar su tarjeta.
///
/// Se copia del mapa antes de dibujar nada porque el bucle modifica la aplicación más abajo
/// y no puede sostener un préstamo del nodo mientras tanto. De las etiquetas se guardan solo
/// cuántas hay y cuál es la primera: clonar el vector entero suponía reservar y copiar una
/// cadena por etiqueta y por nodo, sesenta veces por segundo.
struct DatosDeLaTarjeta {
    /// Título del nodo.
    title: String,
    /// El nodo no tiene notas, y por tanto no lleva el icono que lo indica.
    sin_notas: bool,
    /// Cuántas etiquetas tiene, que influye en el alto de la tarjeta.
    numero_de_etiquetas: usize,
    /// La primera etiqueta, que es la única que se dibuja.
    primera_etiqueta: Option<String>,
    /// Estado del trabajo, que aporta el emoji que precede al título.
    status: EstadoNodo,
    /// Prioridad estratégica para evaluar indicadores de urgencia.
    priority: PrioridadNodo,
    /// Estado de supervisión humana para indicadores de control.
    review_status: EstadoRevision,
    /// Rol del nodo, que se pinta abajo a la derecha (PH-1007-3).
    role: crate::model::RolNodo,
    /// Posición en las coordenadas del mapa.
    pos: [f32; 2],
    /// El nodo no tiene hijos, así que no lleva botón de plegar.
    sin_hijos: bool,
    /// La rama que cuelga de él está plegada.
    collapsed: bool,
    /// Es el nodo raíz, que se dibuja con otro fondo y otro tamaño de letra.
    es_la_raiz: bool,
    /// Estaba seleccionado al empezar el fotograma, antes de atender ningún gesto.
    esta_seleccionado: bool,
    /// A qué rama pertenece, que decide el color de su barra izquierda.
    indice_de_rama: usize,
}

/// Dónde cae la tarjeta en la pantalla y qué relación tiene el cursor con ella.
#[derive(Clone, Copy)]
struct GeometriaDeLaTarjeta {
    /// El rectángulo de la tarjeta, en coordenadas de pantalla.
    caja: Rect,
    /// Ancho en pantalla, ya con el zoom aplicado.
    screen_w: f32,
    /// Alto en pantalla, ya con el zoom aplicado.
    screen_h: f32,
    /// El cursor está sobre la tarjeta.
    cursor_encima: bool,
    /// El cursor está sobre la tarjeta o sobre el hueco de sus botones.
    cursor_en_la_zona: bool,
}

/// Por dónde empieza una rama al dibujar sus conexiones.
struct ArranqueDeLaRama {
    /// Nodo del que sale la primera curva.
    parent_id: Uuid,
    /// Dónde está ese nodo, en coordenadas del mapa.
    parent_pos: Pos2,
    /// El hijo al que llega, y desde el que se sigue descendiendo.
    id_del_nodo: Uuid,
    /// Color de toda la rama, que se hereda hacia abajo.
    color_de_la_rama: Color32,
}

/// Renderiza el lienzo infinito 2D, capturando eventos de ratón, dibujando conexiones y nodos.
///
/// # Parámetros
/// - `app`: el estado de la aplicación. Se recibe como préstamo mutable porque el lienzo no
///   solo dibuja: también aplica lo que el usuario hace sobre él, como seleccionar, mover,
///   plegar o renombrar un nodo.
/// - `ui`: la interfaz de `egui` del fotograma en curso, de la que se leen el ratón y el
///   teclado a través de su contexto.
pub fn dibujar_lienzo(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(app.presentacion().tema().fondo_del_lienzo))
        .show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                ui.available_size_before_wrap(),
                egui::Sense::click_and_drag(),
            );
            atender_navegacion_del_lienzo(app, ui, &response);
            dibujar_fotograma_del_lienzo(app, ui, &response, &painter);
        });
}

/// Actualiza tamaño visible, desplazamiento y zoom a partir del gesto actual.
fn atender_navegacion_del_lienzo(
    app: &mut AplicacionMapaMental,
    ui: &egui::Ui,
    response: &egui::Response,
) {
    let area = response.rect;
    app.lienzo_mut().actualizar_vista().tamano_visible = area.size();
    if response.dragged_by(egui::PointerButton::Middle)
        || response.dragged_by(egui::PointerButton::Secondary)
    {
        let zoom = app.lienzo().vista().zoom;
        app.lienzo_mut().actualizar_vista().desplazamiento += response.drag_delta() / zoom;
    }

    let desplazamiento_de_rueda = ui.ctx().input(|i| i.smooth_scroll_delta.y);
    if desplazamiento_de_rueda == 0.0 || !response.hovered() {
        return;
    }
    let Some(cursor) = ui.ctx().input(|i| i.pointer.hover_pos()) else {
        return;
    };
    let zoom_anterior = app.lienzo().vista().zoom;
    let factor = (desplazamiento_de_rueda * SENSIBILIDAD_DEL_ZOOM).exp();
    let zoom_nuevo = (zoom_anterior * factor).clamp(ZOOM_MINIMO, ZOOM_MAXIMO);
    let punto_bajo_el_cursor = (cursor - area.center()) / zoom_anterior
        - app.lienzo_mut().actualizar_vista().desplazamiento;
    app.lienzo_mut().actualizar_vista().zoom = zoom_nuevo;
    app.lienzo_mut().actualizar_vista().desplazamiento =
        (cursor - area.center()) / zoom_nuevo - punto_bajo_el_cursor;
}

/// Prepara las conversiones del fotograma y ordena sus cuatro fases de dibujo.
fn dibujar_fotograma_del_lienzo(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    response: &egui::Response,
    painter: &egui::Painter,
) {
    let area = response.rect;
    let centro = area.center();
    let zoom = app.lienzo().vista().zoom;
    let desplazamiento = app.lienzo_mut().actualizar_vista().desplazamiento;
    let a_pantalla = |pos: Pos2| centro + (pos.to_vec2() + desplazamiento) * zoom;
    let a_mundo = |pos: Pos2| Pos2::ZERO + ((pos - centro) / zoom - desplazamiento);

    dibujar_reticula(
        painter,
        area,
        desplazamiento,
        zoom,
        app.presentacion().tema().linea_de_la_reticula,
    );
    let ocultos = app.mapa().proyecto().nodos_ocultos_por_plegado();
    let fotograma = Fotograma {
        painter,
        zoom,
        area_visible: area.expand(MARGEN_DE_DIBUJADO),
        a_pantalla: &a_pantalla,
        a_mundo: &a_mundo,
        ocultos: &ocultos,
        puntero: leer_las_pulsaciones(ui.ctx(), ui.layer_id(), response.rect),
        editable: !app.es_solo_lectura_por_raiz(),
    };
    anotar_el_clic_derecho(app, response, &fotograma);

    // Las conexiones se pintan primero para que las tarjetas queden encima.
    dibujar_las_conexiones_de_la_jerarquia(app, &fotograma);
    dibujar_las_conexiones_cruzadas(app, &fotograma);
    let mut acciones = AccionesDelLienzo::default();
    dibujar_las_tarjetas(app, ui, &fotograma, &mut acciones);
    if let Some((texto, puntero)) = acciones.texto_emergente.take() {
        pintar_el_texto_emergente(ui.ctx(), texto, puntero);
    }
    aplicar_las_acciones_del_lienzo(
        app,
        acciones,
        response,
        fotograma.puntero,
        fotograma.editable,
    );
}

/// Anota el clic derecho sin arrastre de este fotograma, si el mapa admite cambios.
///
/// Lo interpreta `ui::menu_contextual_nodo`. `egui` solo lo da por clic si el puntero no superó
/// el umbral de arrastre, así que mover la cámara con el botón derecho no lo dispara. En solo
/// lectura no se anota: casi todo lo que ofrece el menú cambia el mapa.
///
/// # Parámetros
/// - `app`: el estado, donde se anota el clic.
/// - `response`: la respuesta del lienzo en este fotograma.
/// - `fotograma`: la conversión a coordenadas del lienzo y si el mapa es editable.
fn anotar_el_clic_derecho(
    app: &mut AplicacionMapaMental,
    response: &egui::Response,
    fotograma: &Fotograma,
) {
    if !fotograma.editable || !response.secondary_clicked() {
        return;
    }
    if let Some(en_pantalla) = response.interact_pointer_pos() {
        let en_el_lienzo = (fotograma.a_mundo)(en_pantalla);
        app.lienzo_mut().actualizar_vista().clic_derecho = Some((en_el_lienzo, en_pantalla));
    }
}

/// Dibuja las curvas de la jerarquía, una rama por cada hijo de la raíz.
///
/// # Parámetros
/// - `app`: el estado, del que se leen el mapa y los colores.
/// - `fotograma`: el pintor, la ventana visible y la conversión de coordenadas.
fn dibujar_las_conexiones_de_la_jerarquia(app: &AplicacionMapaMental, fotograma: &Fotograma) {
    let root_id = app.mapa().proyecto().root_id;
    let posicion_de_la_raiz = app
        .mapa()
        .proyecto()
        .nodes
        .get(&root_id)
        .map(|r| Pos2::new(r.pos[0], r.pos[1]))
        .unwrap_or(Pos2::ZERO);

    let hijos_de_la_raiz = app
        .mapa()
        .proyecto()
        .nodes
        .get(&root_id)
        .map(|r| r.children.clone())
        .unwrap_or_default();

    for (branch_idx, &id_de_la_rama) in hijos_de_la_raiz.iter().enumerate() {
        let color_de_la_rama = app.presentacion().tema().color_de_la_rama(branch_idx);
        dibujar_conexiones_de_la_rama(
            app,
            fotograma,
            ArranqueDeLaRama {
                parent_id: root_id,
                parent_pos: posicion_de_la_raiz,
                id_del_nodo: id_de_la_rama,
                color_de_la_rama,
            },
        );
    }
}

/// Dibuja las conexiones cruzadas, las que no siguen la jerarquía, con su etiqueta.
///
/// # Parámetros
/// - `app`: el estado, del que se leen las conexiones y el tema.
/// - `fotograma`: el pintor, el zoom, la conversión de coordenadas y los nodos ocultos.
fn dibujar_las_conexiones_cruzadas(app: &AplicacionMapaMental, fotograma: &Fotograma) {
    let Fotograma {
        painter,
        zoom,
        a_pantalla,
        ocultos,
        ..
    } = *fotograma;

    for conn in &app.mapa().proyecto().connections {
        // Una conexión con un extremo escondido no se dibuja.
        //
        // Al plegar una rama dejaron de verse sus tarjetas, pero las curvas ámbar
        // que salían de ellas seguían pintándose, ancladas a la última posición
        // conocida —que la disposición automática ya no actualiza—, así que acababan
        // cruzando por encima de otras tarjetas con su etiqueta a cuestas.
        if ocultos.contains(&conn.from) || ocultos.contains(&conn.to) {
            continue;
        }

        if let (Some(from_node), Some(to_node)) = (
            app.mapa().proyecto().nodes.get(&conn.from),
            app.mapa().proyecto().nodes.get(&conn.to),
        ) {
            let origen_en_pantalla = a_pantalla(Pos2::new(from_node.pos[0], from_node.pos[1]));
            let destino_en_pantalla = a_pantalla(Pos2::new(to_node.pos[0], to_node.pos[1]));

            let pts = calcular_curva_bezier(origen_en_pantalla, destino_en_pantalla);
            let stroke = Stroke::new(
                (2.0 * zoom).clamp(GROSOR_MINIMO_CONEXION, GROSOR_MAXIMO_CONEXION),
                app.presentacion().tema().linea_de_conexion_cruzada,
            );
            painter.add(CubicBezierShape::from_points_stroke(
                pts,
                false,
                Color32::TRANSPARENT,
                stroke,
            ));

            // La etiqueta va en el punto medio de la curva.
            let punto_medio = Pos2::new((pts[1].x + pts[2].x) / 2.0, (pts[1].y + pts[2].y) / 2.0);
            let texto_de_la_etiqueta = if conn.label.is_empty() {
                conn.relation_type.nombre_para_interfaz(app.idioma())
            } else {
                conn.label.as_str()
            };

            painter.text(
                punto_medio,
                egui::Align2::CENTER_CENTER,
                texto_de_la_etiqueta,
                FontId::proportional(LETRA_DE_LA_ETIQUETA_DE_CONEXION * zoom),
                app.presentacion().tema().linea_de_conexion_cruzada,
            );
        }
    }
}

/// Dibuja la tarjeta de cada nodo visible y anota lo que el usuario pide sobre ellas.
///
/// No aplica nada: lo que el usuario pide se recoge en `acciones` y se aplica al salir,
/// porque dentro del bucle el mapa está prestado para leerlo.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque el arrastre mueve el nodo en el acto.
/// - `ui`: la interfaz de `egui`, necesaria para el editor del título.
/// - `fotograma`: lo que no cambia durante el dibujado.
/// - `acciones`: donde se anota lo que habrá que aplicar después.
fn dibujar_las_tarjetas(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    fotograma: &Fotograma,
    acciones: &mut AccionesDelLienzo,
) {
    let (mapa, lienzo) = app.mapa_y_lienzo_mut();
    let nodo_arrastrado = lienzo.vista().nodo_arrastrado;
    let identificadores = {
        let indice = lienzo.indice_espacial();
        indice.actualizar(mapa.proyecto());
        indice.candidatos(fotograma, nodo_arrastrado)
    };
    let ramas = mapa.proyecto().rama_de_cada_nodo();

    for id_del_nodo in identificadores {
        if fotograma.ocultos.contains(&id_del_nodo) {
            continue;
        }

        let Some(datos) = datos_de_la_tarjeta(app, id_del_nodo, &ramas) else {
            continue;
        };
        // Sin geometría, la tarjeta cae fuera de la ventana y no hay nada que dibujar.
        let Some(geometria) = geometria_de_la_tarjeta(app, fotograma, id_del_nodo, &datos) else {
            continue;
        };

        atender_el_arrastre(app, fotograma, id_del_nodo, &datos, &geometria);
        anotar_los_clics(fotograma, id_del_nodo, &geometria, acciones);
        pintar_la_tarjeta(app, fotograma, &datos, &geometria);

        if app.lienzo_mut().editar_titulo().nodo == Some(id_del_nodo) {
            dibujar_el_editor_del_titulo(app, ui, fotograma, id_del_nodo, &geometria, acciones);
        } else {
            pintar_el_titulo_y_los_indicadores(app, fotograma, &datos, &geometria);
            anotar_el_icono_bajo_el_cursor(app, fotograma, &datos, &geometria, acciones);
        }

        pintar_el_boton_de_plegar(app, fotograma, id_del_nodo, &datos, &geometria, acciones);
        if fotograma.editable {
            pintar_el_boton_de_anadir_hijo(app, fotograma, id_del_nodo, &geometria, acciones);
        }
    }
}

/// Copia del mapa lo justo que hace falta para dibujar la tarjeta de un nodo.
///
/// # Parámetros
/// - `app`: el estado, del que se lee el nodo.
/// - `id_del_nodo`: el nodo del que se quieren los datos.
/// - `ramas`: a qué rama pertenece cada nodo, calculado una vez por fotograma.
///
/// # Devuelve
/// `None` si el nodo ya no existe, cosa posible porque el bucle recorre una lista de
/// identificadores tomada antes de empezar.
fn datos_de_la_tarjeta(
    app: &AplicacionMapaMental,
    id_del_nodo: Uuid,
    ramas: &HashMap<Uuid, usize>,
) -> Option<DatosDeLaTarjeta> {
    let node = app.mapa().proyecto().nodes.get(&id_del_nodo)?;
    Some(DatosDeLaTarjeta {
        title: node.title.clone(),
        sin_notas: node.notes.is_empty(),
        numero_de_etiquetas: node.tags.len(),
        primera_etiqueta: node.tags.first().cloned(),
        status: node.status,
        priority: node.priority,
        review_status: node.review_status,
        role: node.role,
        pos: node.pos,
        sin_hijos: node.children.is_empty(),
        collapsed: node.collapsed,
        es_la_raiz: id_del_nodo == app.mapa().proyecto().root_id,
        esta_seleccionado: app.mapa().nodo_seleccionado() == Some(id_del_nodo),
        indice_de_rama: ramas.get(&id_del_nodo).copied().unwrap_or(0),
    })
}

/// Calcula dónde cae la tarjeta y descarta la que no se ve.
///
/// # Parámetros
/// - `app`: el estado, del que se lee qué nodo se está arrastrando.
/// - `fotograma`: el zoom, la ventana visible, la conversión de coordenadas y el puntero.
/// - `id_del_nodo`: el nodo del que se calcula la geometría.
/// - `datos`: su título, sus notas y sus etiquetas, que deciden el tamaño de la tarjeta.
///
/// # Devuelve
/// `None` cuando la tarjeta queda fuera de la ventana y no hay que dibujarla.
fn geometria_de_la_tarjeta(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    id_del_nodo: Uuid,
    datos: &DatosDeLaTarjeta,
) -> Option<GeometriaDeLaTarjeta> {
    let Fotograma {
        zoom,
        area_visible,
        a_pantalla,
        puntero,
        ..
    } = *fotograma;
    let posicion_del_cursor = puntero.posicion;
    let pos = datos.pos;
    let (w, h) = estimar_tamano_del_nodo(&datos.title, datos.numero_de_etiquetas);

    let screen_w = w * zoom;
    let screen_h = h * zoom;
    let centro_de_pantalla = a_pantalla(Pos2::new(pos[0], pos[1]));
    let caja_del_nodo = Rect::from_center_size(centro_de_pantalla, Vec2::new(screen_w, screen_h));

    // Lo que queda fuera de la ventana no se dibuja.
    //
    // Antes se componía y se pintaba cada nodo del mapa, estuviera o no a la
    // vista. En un mapa salido del escáner de repositorios —cientos o miles de
    // nodos, de los que en pantalla caben unas decenas— eso es casi todo el
    // trabajo del fotograma tirado.
    //
    // El nodo que se está arrastrando se dibuja siempre, aunque el puntero lo
    // haya sacado por el borde: si dejara de dibujarse, dejaría también de seguir
    // al ratón y el arrastre se quedaría a medias.
    let se_esta_arrastrando = app.lienzo().vista().nodo_arrastrado == Some(id_del_nodo);
    if !se_esta_arrastrando && !area_visible.intersects(caja_del_nodo) {
        return None;
    }

    let cursor_encima = posicion_del_cursor
        .map(|p| caja_del_nodo.contains(p))
        .unwrap_or(false);

    // La tarjeta más el hueco de los botones que cuelgan de ella: el «+» por
    // debajo y el de plegar por la derecha. Solo se ensancha hacia esos dos
    // lados, para no invadir el sitio de los nodos vecinos.
    let zona_del_nodo = Rect::from_min_max(
        caja_del_nodo.min,
        caja_del_nodo.max + Vec2::splat(HUECO_DE_LOS_BOTONES * zoom),
    );
    let cursor_en_la_zona_del_nodo = posicion_del_cursor
        .map(|p| zona_del_nodo.contains(p))
        .unwrap_or(false);

    Some(GeometriaDeLaTarjeta {
        caja: caja_del_nodo,
        screen_w,
        screen_h,
        cursor_encima,
        cursor_en_la_zona: cursor_en_la_zona_del_nodo,
    })
}

/// Engancha, mueve y suelta el nodo que se está arrastrando.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque el arrastre mueve el nodo en el acto.
/// - `fotograma`: el puntero y la conversión de la pantalla al mapa.
/// - `id_del_nodo`: el nodo sobre el que se decide.
/// - `datos`: de donde sale su posición actual en el mapa.
/// - `geometria`: para saber si el cursor está encima.
fn atender_el_arrastre(
    app: &mut AplicacionMapaMental,
    fotograma: &Fotograma,
    id_del_nodo: Uuid,
    datos: &DatosDeLaTarjeta,
    geometria: &GeometriaDeLaTarjeta,
) {
    if !fotograma.editable {
        return;
    }
    let Fotograma {
        a_mundo, puntero, ..
    } = *fotograma;
    let PulsacionesDelPuntero {
        posicion: posicion_del_cursor,
        boton_pulsado,
        recien_pulsado,
        ..
    } = puntero;
    let cursor_encima = geometria.cursor_encima;
    let pos = datos.pos;

    // Arrastre del nodo.
    // El nodo se engancha en el fotograma en que se **pulsa**, no mientras el
    // botón siga bajado. Con lo segundo, empezar a arrastrar sobre el fondo
    // vacío y pasar por encima de un nodo se lo llevaba por delante.
    if cursor_encima
        && recien_pulsado
        && app.lienzo().vista().nodo_arrastrado.is_none()
        && app.lienzo().edicion().nodo.is_none()
    {
        app.lienzo_mut().actualizar_vista().nodo_arrastrado = Some(id_del_nodo);
        // Se recuerda por dónde se agarró, para que el nodo se mueva **con** el
        // cursor en lugar de centrarse debajo de él.
        app.lienzo_mut().actualizar_vista().agarre_del_arrastre = posicion_del_cursor
            .map(|p| a_mundo(p) - Pos2::new(pos[0], pos[1]))
            .unwrap_or(Vec2::ZERO);
        app.lienzo_mut().actualizar_vista().posicion_al_agarrar = pos;

        // Con la colocación automática puesta, el arrastre se acepta y la
        // siguiente recolocación lo deshace —ocurre al escribir en el inspector,
        // al añadir un nodo, al plegar una rama—, así que desde fuera parece que
        // el programa ignora el gesto. Medido: `[220, 0]` → `[340, 90]` → `[220, 0]`.
        //
        // El criterio del usuario fue no cambiar el comportamiento sino explicarlo,
        // y por eso aquí no se impide nada: solo se dice, una vez, al enganchar.
        // El aviso nombra la opción del menú a la que hay que ir, y la nombra tal
        // como se llama en el idioma que esté puesto.
        if app.mapa().proyecto().layout_mode != ModoDisposicion::FreeDrag {
            app.establecer_estado(
                crate::textos::Texto::AvisoArrastreEnModoAutomatico.en(app.idioma()),
            );
        }
    }

    if app.lienzo().vista().nodo_arrastrado == Some(id_del_nodo) {
        if boton_pulsado {
            if let Some(mpos) = posicion_del_cursor {
                let world_pos =
                    a_mundo(mpos) - app.lienzo_mut().actualizar_vista().agarre_del_arrastre;
                let posicion_es_trabajo =
                    app.mapa().proyecto().layout_mode == ModoDisposicion::FreeDrag;
                let mut posicion_cambiada = false;
                if let Some(n) = app
                    .mapa_mut()
                    .proyecto_para_editar()
                    .nodes
                    .get_mut(&id_del_nodo)
                {
                    let posicion = [world_pos.x, world_pos.y];
                    if n.pos != posicion {
                        n.pos = posicion;
                        posicion_cambiada = true;
                        if posicion_es_trabajo {
                            app.mapa_mut().proyecto_para_editar().marcar_modificado();
                        }
                    }
                }
                if posicion_cambiada {
                    app.lienzo_mut().invalidar_indice_espacial();
                }
            }
        } else {
            app.lienzo_mut().actualizar_vista().nodo_arrastrado = None;
            // Quién interpreta la suelta —si cayó encima de otro nodo— es
            // `ui::suelta_de_nodo`; aquí solo se anota qué se soltó y dónde se ve.
            app.lienzo_mut().actualizar_vista().nodo_recien_soltado = Some((
                id_del_nodo,
                (fotograma.a_pantalla)(Pos2::new(pos[0], pos[1])),
            ));
        }
    }
}

/// Anota el clic y el doble clic sobre la tarjeta, sin aplicarlos.
///
/// # Parámetros
/// - `fotograma`: de donde se leen el clic y el doble clic.
/// - `id_del_nodo`: el nodo sobre el que ha ocurrido.
/// - `geometria`: para saber si el cursor está encima.
/// - `acciones`: donde se anota lo ocurrido.
fn anotar_los_clics(
    fotograma: &Fotograma,
    id_del_nodo: Uuid,
    geometria: &GeometriaDeLaTarjeta,
    acciones: &mut AccionesDelLienzo,
) {
    let PulsacionesDelPuntero {
        clic_completo,
        doble_clic,
        ..
    } = fotograma.puntero;
    let cursor_encima = geometria.cursor_encima;

    if cursor_encima && clic_completo {
        acciones.nodo_pulsado = Some(id_del_nodo);
    }

    if cursor_encima && doble_clic {
        acciones.nodo_con_doble_clic = Some(id_del_nodo);
    }
}

/// Pinta el fondo, el borde y la barra de color de la tarjeta.
///
/// # Parámetros
/// - `app`: el estado, del que se lee el tema.
/// - `fotograma`: el pintor y el zoom.
/// - `datos`: si es la raíz, si está seleccionado y de qué rama viene.
/// - `geometria`: dónde cae la tarjeta y si el cursor está encima.
fn pintar_la_tarjeta(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    datos: &DatosDeLaTarjeta,
    geometria: &GeometriaDeLaTarjeta,
) {
    let Fotograma { painter, zoom, .. } = *fotograma;
    let GeometriaDeLaTarjeta {
        caja: caja_del_nodo,
        cursor_encima,
        ..
    } = *geometria;
    let DatosDeLaTarjeta {
        es_la_raiz,
        esta_seleccionado,
        indice_de_rama,
        ..
    } = *datos;

    // Fondo y borde de la tarjeta del nodo
    let color_de_fondo = if es_la_raiz {
        app.presentacion().tema().fondo_de_la_raiz
    } else {
        app.presentacion().tema().fondo_del_nodo
    };

    let color_del_borde = if esta_seleccionado {
        app.presentacion().tema().borde_del_nodo_seleccionado
    } else if cursor_encima {
        app.presentacion().tema().texto_secundario
    } else {
        app.presentacion().tema().borde_del_nodo
    };

    let grosor_del_borde = if esta_seleccionado {
        GROSOR_DEL_BORDE_SELECCIONADO * zoom
    } else {
        GROSOR_DEL_BORDE_NORMAL * zoom
    };
    let radio_de_esquina = (RADIO_DE_LA_ESQUINA * zoom)
        .clamp(RADIO_MINIMO_DE_LA_ESQUINA, RADIO_MAXIMO_DE_LA_ESQUINA)
        as u8;

    painter.rect(
        caja_del_nodo,
        CornerRadius::same(radio_de_esquina),
        color_de_fondo,
        Stroke::new(
            grosor_del_borde.clamp(GROSOR_MINIMO_DEL_TRAZO, GROSOR_MAXIMO_DEL_TRAZO),
            color_del_borde,
        ),
        StrokeKind::Inside,
    );

    // La barra de color del borde izquierdo, que dice de qué rama viene el nodo.
    //
    // Antes se coloreaba por profundidad, con la misma paleta que las líneas usan
    // por rama: dos significados compartiendo colores. El resultado era que una
    // tarjeta a la que llegaba una línea azul podía llevar la barra verde, y la
    // ayuda promete justo lo contrario —que el color permite «seguir cualquier
    // hilo hasta el final y saber de dónde viene».
    let color_de_la_barra = if es_la_raiz {
        app.presentacion().tema().colores_de_las_ramas[0]
    } else {
        app.presentacion().tema().color_de_la_rama(indice_de_rama)
    };

    let caja_de_la_barra = Rect::from_min_max(
        caja_del_nodo.min,
        Pos2::new(caja_del_nodo.min.x + 5.0 * zoom, caja_del_nodo.max.y),
    );
    painter.rect_filled(
        caja_de_la_barra,
        CornerRadius {
            nw: radio_de_esquina,
            sw: radio_de_esquina,
            ne: 0,
            se: 0,
        },
        color_de_la_barra,
    );
}

/// Dibuja el cuadro de edición del título dentro de la propia tarjeta.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque el cuadro escribe en él lo tecleado.
/// - `ui`: la interfaz de `egui`, en la que se abre el cuadro.
/// - `fotograma`: el zoom, al que va a escala el tamaño de la letra.
/// - `id_del_nodo`: el nodo que se está editando.
/// - `geometria`: dónde cae la tarjeta, que es donde se encaja el cuadro.
/// - `acciones`: donde se anota el título nuevo al terminar.
fn dibujar_el_editor_del_titulo(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    fotograma: &Fotograma,
    id_del_nodo: Uuid,
    geometria: &GeometriaDeLaTarjeta,
    acciones: &mut AccionesDelLienzo,
) {
    let zoom = fotograma.zoom;
    let GeometriaDeLaTarjeta {
        caja: caja_del_nodo,
        screen_w,
        screen_h,
        ..
    } = *geometria;

    let caja_de_edicion = Rect::from_min_size(
        Pos2::new(
            caja_del_nodo.min.x + 10.0 * zoom,
            caja_del_nodo.min.y + 6.0 * zoom,
        ),
        Vec2::new(screen_w - 20.0 * zoom, screen_h - 12.0 * zoom),
    );

    let mut ui_child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(caja_de_edicion)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let respuesta_de_edicion = ui_child.add(
        egui::TextEdit::singleline(&mut app.lienzo_mut().editar_titulo().texto)
            .font(FontId::proportional(LETRA_DEL_TITULO_EN_EDICION * zoom))
            .desired_width(screen_w - 20.0 * zoom),
    );
    // El foco se pide una sola vez, al abrir la edición. Pedirlo en cada
    // fotograma dejaba `lost_focus()` permanentemente en falso —así que
    // salir con un clic fuera tiraba lo escrito— y le robaba el teclado a
    // todo lo demás: el panel lateral se dibuja antes que el lienzo, así que
    // sus cuadros lo perdían en el acto y las teclas acababan en el título.
    if app.lienzo_mut().editar_titulo().foco_pendiente {
        respuesta_de_edicion.request_focus();
        app.lienzo_mut().editar_titulo().foco_pendiente = false;
    }

    if respuesta_de_edicion.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        acciones.nodo_a_renombrar =
            Some((id_del_nodo, app.lienzo_mut().editar_titulo().texto.clone()));
        app.lienzo_mut().editar_titulo().nodo = None;
    }
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.lienzo_mut().editar_titulo().nodo = None;
    }
}

/// Pinta el título del nodo, los indicadores de prioridad y revisión, y las notas y etiquetas.
///
/// # Parámetros
/// - `app`: el estado, del que se lee el tema.
/// - `fotograma`: el pintor y el zoom.
/// - `datos`: el título, el estado, la prioridad, la revisión, las notas y la primera etiqueta.
/// - `geometria`: dónde cae la tarjeta.
fn pintar_el_titulo_y_los_indicadores(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    datos: &DatosDeLaTarjeta,
    geometria: &GeometriaDeLaTarjeta,
) {
    let Fotograma { painter, zoom, .. } = *fotograma;
    let caja_del_nodo = geometria.caja;
    let DatosDeLaTarjeta {
        title,
        status,
        priority,
        review_status,
        role,
        es_la_raiz,
        sin_notas,
        primera_etiqueta,
        ..
    } = datos;
    let (es_la_raiz, sin_notas) = (*es_la_raiz, *sin_notas);

    let indicadores = calcular_indicadores_del_nodo(*priority, *review_status);
    let reserva_indicadores = reserva_horizontal_de_indicadores(indicadores.len(), zoom);

    // El título del nodo
    //
    // Se compone con un ancho máximo para que **se envuelva dentro de la
    // caja**. El espacio disponible tiene en cuenta la reserva horizontal
    // para los indicadores de la esquina superior derecha (C19-D).
    let posicion_del_titulo = posicion_del_titulo(caja_del_nodo, zoom);

    let texto_del_titulo = format!("{} {}", status.emoji(), title);
    let tamano_de_letra = tamano_de_letra_del_titulo(es_la_raiz) * zoom;
    let ancho_para_el_texto =
        (caja_del_nodo.width() - (MARGEN_DEL_TEXTO * zoom + reserva_indicadores)).max(1.0);

    let texto_compuesto = painter.layout(
        texto_del_titulo,
        FontId::proportional(tamano_de_letra),
        app.presentacion().tema().texto_principal,
        ancho_para_el_texto,
    );
    painter.galley(
        posicion_del_titulo,
        texto_compuesto,
        app.presentacion().tema().texto_principal,
    );

    // Indicadores visuales de prioridad y control humano (C19-C, C19-D)
    if !indicadores.is_empty() {
        let tamano_letra_indicador = 12.0 * zoom;

        for (orden_desde_la_derecha, indicador) in indicadores.iter().rev().enumerate() {
            let color = if indicador.es_prioridad {
                match priority {
                    PrioridadNodo::Critica => app.presentacion().tema().peligro,
                    PrioridadNodo::Alta => app.presentacion().tema().aviso,
                    _ => app.presentacion().tema().texto_secundario,
                }
            } else {
                match review_status {
                    EstadoRevision::RequiereCorreccion => app.presentacion().tema().peligro,
                    EstadoRevision::AprobadoPorHumano => app.presentacion().tema().exito,
                    EstadoRevision::GeneradoPorIA => app.presentacion().tema().acento,
                    _ => app.presentacion().tema().texto_secundario,
                }
            };

            painter.text(
                ancla_del_indicador(caja_del_nodo, zoom, orden_desde_la_derecha),
                egui::Align2::RIGHT_TOP,
                indicador.simbolo,
                FontId::proportional(tamano_letra_indicador),
                color,
            );
        }
    }

    // Etiquetas e indicador de notas.
    let linea_de_los_iconos = caja_del_nodo.max.y - 14.0 * zoom;
    painter.text(
        ancla_del_rol(caja_del_nodo, zoom),
        egui::Align2::RIGHT_BOTTOM,
        role.simbolo(),
        FontId::proportional(LETRA_DEL_ICONO_DE_NOTAS * zoom),
        app.presentacion().tema().texto_atenuado,
    );
    if !sin_notas {
        painter.text(
            ancla_de_las_notas(caja_del_nodo, zoom),
            egui::Align2::RIGHT_BOTTOM,
            "📝",
            FontId::proportional(LETRA_DEL_ICONO_DE_NOTAS * zoom),
            app.presentacion().tema().texto_atenuado,
        );
    }

    if let Some(primera) = &primera_etiqueta {
        let vista_de_la_etiqueta = format!("#{primera}");
        painter.text(
            Pos2::new(caja_del_nodo.min.x + 12.0 * zoom, linea_de_los_iconos),
            egui::Align2::LEFT_BOTTOM,
            vista_de_la_etiqueta,
            FontId::proportional(LETRA_DE_LAS_ETIQUETAS * zoom),
            app.presentacion().tema().texto_secundario,
        );
    }
}

/// Pinta el botón de plegar y desplegar, y anota si se ha pulsado.
///
/// # Parámetros
/// - `app`: el estado, del que se lee el tema.
/// - `fotograma`: el pintor, el zoom y el puntero.
/// - `id_del_nodo`: el nodo al que pertenece el botón.
/// - `datos`: si tiene hijos —si no, no hay botón— y si está plegado.
/// - `geometria`: dónde cae la tarjeta, a cuya derecha se coloca el botón.
/// - `acciones`: donde se anota la pulsación.
fn pintar_el_boton_de_plegar(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    id_del_nodo: Uuid,
    datos: &DatosDeLaTarjeta,
    geometria: &GeometriaDeLaTarjeta,
    acciones: &mut AccionesDelLienzo,
) {
    let Fotograma {
        painter,
        zoom,
        puntero,
        ..
    } = *fotograma;
    let PulsacionesDelPuntero {
        posicion: posicion_del_cursor,
        clic_completo,
        ..
    } = puntero;
    let caja_del_nodo = geometria.caja;
    let (sin_hijos, collapsed) = (datos.sin_hijos, datos.collapsed);

    // Botón de plegar y desplegar, solo si el nodo tiene hijos
    if !sin_hijos {
        let centro_del_boton_plegar =
            Pos2::new(caja_del_nodo.max.x + 8.0 * zoom, caja_del_nodo.center().y);
        let radio_del_boton_plegar = 7.0 * zoom;
        let caja_del_boton_plegar = Rect::from_center_size(
            centro_del_boton_plegar,
            Vec2::splat(radio_del_boton_plegar * 2.0),
        );

        let cursor_sobre_el_boton_plegar = posicion_del_cursor
            .map(|p| caja_del_boton_plegar.contains(p))
            .unwrap_or(false);

        painter.circle(
            centro_del_boton_plegar,
            radio_del_boton_plegar,
            if cursor_sobre_el_boton_plegar {
                app.presentacion().tema().fondo_de_la_insignia
            } else {
                app.presentacion().tema().fondo_del_nodo
            },
            Stroke::new(1.0_f32, app.presentacion().tema().borde_del_nodo),
        );

        let icono_del_plegado = if collapsed { "+" } else { "−" };
        painter.text(
            centro_del_boton_plegar,
            egui::Align2::CENTER_CENTER,
            icono_del_plegado,
            FontId::monospace(10.0 * zoom),
            app.presentacion().tema().texto_principal,
        );

        if cursor_sobre_el_boton_plegar && clic_completo {
            acciones.nodo_a_plegar = Some(id_del_nodo);
        }
    }
}

/// Pinta el botón «+» que cuelga un hijo, mientras el cursor ronda el nodo.
///
/// # Parámetros
/// - `app`: el estado, del que se leen el tema y si hay una edición abierta.
/// - `fotograma`: el pintor, el zoom y el puntero.
/// - `id_del_nodo`: el nodo del que colgaría el hijo.
/// - `geometria`: dónde cae la tarjeta y si el cursor está en su zona.
/// - `acciones`: donde se anota la pulsación.
fn pintar_el_boton_de_anadir_hijo(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    id_del_nodo: Uuid,
    geometria: &GeometriaDeLaTarjeta,
    acciones: &mut AccionesDelLienzo,
) {
    let Fotograma {
        painter,
        zoom,
        puntero,
        ..
    } = *fotograma;
    let PulsacionesDelPuntero {
        posicion: posicion_del_cursor,
        clic_completo,
        ..
    } = puntero;
    let caja_del_nodo = geometria.caja;
    let cursor_en_la_zona_del_nodo = geometria.cursor_en_la_zona;

    // El botón «+» para colgar un hijo, mientras el cursor ronda el nodo.
    //
    // La zona que lo enciende es la tarjeta **más el hueco donde vive el botón**,
    // que está justo debajo de ella. Antes bastaba con la tarjeta, y el resultado
    // era que el botón huía del cursor: al bajar el ratón hacia él se salía de la
    // tarjeta, el botón dejaba de dibujarse y no había forma de llegar a pulsarlo.
    if cursor_en_la_zona_del_nodo && app.lienzo().edicion().nodo.is_none() {
        let centro_del_boton_anadir =
            Pos2::new(caja_del_nodo.center().x, caja_del_nodo.max.y + 10.0 * zoom);
        let caja_del_boton_anadir =
            Rect::from_center_size(centro_del_boton_anadir, Vec2::splat(14.0 * zoom));
        let cursor_sobre_el_boton_anadir = posicion_del_cursor
            .map(|p| caja_del_boton_anadir.contains(p))
            .unwrap_or(false);

        painter.circle_filled(
            centro_del_boton_anadir,
            7.0 * zoom,
            if cursor_sobre_el_boton_anadir {
                app.presentacion().tema().colores_de_las_ramas[0]
            } else {
                app.presentacion().tema().fondo_de_la_insignia
            },
        );
        // El signo tiene dos fondos posibles, y no vale un color fijo para
        // los dos: en reposo se dibuja sobre la etiqueta y, al señalarlo,
        // sobre el color de la rama. En blanco fijo era invisible sobre el
        // marfil del tema claro.
        painter.text(
            centro_del_boton_anadir,
            egui::Align2::CENTER_CENTER,
            "+",
            FontId::proportional(LETRA_DEL_BOTON_ANADIR * zoom),
            if cursor_sobre_el_boton_anadir {
                app.presentacion().tema().texto_sobre_la_rama
            } else {
                app.presentacion().tema().texto_principal
            },
        );

        if cursor_sobre_el_boton_anadir && clic_completo {
            acciones.nodo_para_hijo_rapido = Some(id_del_nodo);
        }
    }
}

/// Aplica sobre el mapa lo que el usuario ha pedido mientras se dibujaba.
///
/// Va aparte y al final a propósito: dentro del bucle no se puede tocar el mapa, porque
/// está prestado para leerlo.
///
/// # Parámetros
/// - `app`: el estado sobre el que se aplican los cambios.
/// - `acciones`: lo que se anotó durante el dibujado.
/// - `response`: la respuesta del área del lienzo, que dice si se pulsó el fondo.
/// - `puntero`: la posición del cursor, que distingue un clic en el fondo de uno sin cursor.
fn aplicar_las_acciones_del_lienzo(
    app: &mut AplicacionMapaMental,
    acciones: AccionesDelLienzo,
    response: &egui::Response,
    puntero: PulsacionesDelPuntero,
    editable: bool,
) {
    let posicion_del_cursor = puntero.posicion;

    if let Some((id, new_title)) = acciones.nodo_a_renombrar {
        let mut cambio = false;
        if let Some(n) = app.mapa_mut().proyecto_para_editar().nodes.get_mut(&id) {
            if n.title != new_title {
                n.title = new_title;
                n.marcar_editado_por_una_persona();
                cambio = true;
            }
        }
        if cambio {
            app.mapa_mut().proyecto_para_editar().marcar_modificado();
        }
        // El título decide el ancho de la tarjeta, así que cambiarlo cambia el sitio
        // que ocupa y hay que repartir el mapa otra vez.
        app.recolocar_si_procede();
    }

    // Las acciones que el usuario ha pedido sobre el lienzo se aplican aquí, fuera del
    // bucle de dibujado, porque modifican el proyecto mientras aquel lo está leyendo.
    if let Some(id) = acciones.nodo_pulsado {
        // Seleccionar otro nodo cierra la edición abierta sobre el anterior. Sin
        // esto quedaban dos nodos en juego: el inspector mostraba el recién pulsado
        // mientras el editor seguía abierto sobre el otro, acaparando el teclado.
        if app
            .lienzo_mut()
            .edicion()
            .nodo
            .is_some_and(|en_edicion| en_edicion != id)
        {
            app.confirmar_edicion_del_titulo();
        }
        app.mapa_mut().seleccionar_nodo(Some(id));
    } else if response.clicked()
        && posicion_del_cursor.is_some()
        && acciones.nodo_a_plegar.is_none()
        && acciones.nodo_para_hijo_rapido.is_none()
    {
        // Clic en el fondo del lienzo, fuera de cualquier nodo.
        //
        // Los botones de plegar y de añadir hijo viven **fuera** de la tarjeta, a su
        // derecha, así que pulsarlos no cuenta como pulsar el nodo y el clic llegaba
        // hasta aquí: plegar una rama vaciaba el inspector de lo que el usuario
        // estuviera consultando. El de añadir hijo se libraba de casualidad, porque
        // más abajo vuelve a seleccionar el nodo recién creado.
        app.mapa_mut().seleccionar_nodo(None);
        // Y si había un título a medio escribir, se da por bueno en vez de tirarlo.
        app.confirmar_edicion_del_titulo();
    }

    if let Some(id) = acciones.nodo_con_doble_clic {
        if let Some(titulo) = app
            .mapa()
            .proyecto()
            .nodes
            .get(&id)
            .map(|n| n.title.clone())
        {
            app.abrir_edicion_del_nodo(id, titulo);
        }
    }

    if let Some(id) = acciones.nodo_a_plegar {
        if let Some(n) = app.mapa_mut().proyecto_para_editar().nodes.get_mut(&id) {
            n.collapsed = !n.collapsed;
            app.mapa_mut().proyecto_para_editar().marcar_modificado();
        }
        if app.mapa().proyecto().layout_mode != ModoDisposicion::FreeDrag {
            crate::layout::aplicar_disposicion_automatica(app.mapa_mut().proyecto_para_editar());
        }
    }

    if let Some(id) = acciones.nodo_para_hijo_rapido {
        if editable {
            let titulo = crate::textos::Texto::NodoNuevaIdea.en(app.idioma());
            let id_del_hijo = app
                .mapa_mut()
                .proyecto_para_editar()
                .anadir_hijo(id, titulo);
            if app.mapa().proyecto().layout_mode != ModoDisposicion::FreeDrag {
                crate::layout::aplicar_disposicion_automatica(
                    app.mapa_mut().proyecto_para_editar(),
                );
            }
            app.mapa_mut().seleccionar_nodo(Some(id_del_hijo));
            app.abrir_edicion_del_nodo(
                id_del_hijo,
                crate::textos::Texto::NodoNuevaIdea.en(app.idioma()),
            );
        }
    }
}

/// Dibuja la retícula de fondo del lienzo infinito.
///
/// El espaciado se adapta al nivel de zoom para que la retícula siga siendo útil tanto
/// muy alejado como muy cerca.
fn dibujar_reticula(
    painter: &egui::Painter,
    area_de_pantalla: Rect,
    desplazamiento_del_lienzo: Vec2,
    zoom: f32,
    grid_color: Color32,
) {
    let paso_de_la_reticula = 40.0 * zoom;
    if paso_de_la_reticula < 12.0 {
        return;
    }

    let center = area_de_pantalla.center();
    let offset_x = (desplazamiento_del_lienzo.x * zoom + center.x) % paso_de_la_reticula;
    let offset_y = (desplazamiento_del_lienzo.y * zoom + center.y) % paso_de_la_reticula;

    let mut x = area_de_pantalla.min.x + offset_x;
    while x < area_de_pantalla.max.x {
        painter.line_segment(
            [
                Pos2::new(x, area_de_pantalla.min.y),
                Pos2::new(x, area_de_pantalla.max.y),
            ],
            Stroke::new(1.0_f32, grid_color),
        );
        x += paso_de_la_reticula;
    }

    let mut y = area_de_pantalla.min.y + offset_y;
    while y < area_de_pantalla.max.y {
        painter.line_segment(
            [
                Pos2::new(area_de_pantalla.min.x, y),
                Pos2::new(area_de_pantalla.max.x, y),
            ],
            Stroke::new(1.0_f32, grid_color),
        );
        y += paso_de_la_reticula;
    }
}

/// Dibuja las curvas que unen un nodo con todos sus descendientes.
///
/// Las conexiones se trazan como curvas Bézier entre los bordes de los nodos, eligiendo el
/// lado izquierdo o derecho según sus posiciones relativas. Los nodos plegados detienen el
/// descenso.
///
/// # Lo que se corrigió aquí
///
/// Era la única travesía recursiva del programa que no llevaba marca de nodos visitados: un
/// mapa con un ciclo en `children` —alcanzable con un archivo manipulado— la dejaba
/// descendiendo hasta agotar la pila del proceso. Y dibujaba **todas** las curvas del mapa
/// en cada fotograma, estuvieran o no en pantalla; el recorte por ventana que la 0.3.7 dio
/// por hecho solo cubría las tarjetas.
///
/// Descender sí hay que hacerlo siempre, aunque la curva del padre quede fuera: un hijo
/// puede estar dentro de la ventana con el padre lejos. Lo que se salta es solo el trazado.
///
/// # Parámetros
/// - `app`: el estado, del que se leen los nodos, el zoom y el tema.
/// - `fotograma`: el pintor, la ventana visible fuera de la cual no se traza nada y la
///   conversión de coordenadas del mapa a las de la pantalla.
/// - `arranque`: de qué nodo sale la rama, dónde está, a qué hijo llega y con qué color.
fn dibujar_conexiones_de_la_rama(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    arranque: ArranqueDeLaRama,
) {
    let Fotograma {
        painter,
        area_visible,
        a_pantalla,
        ..
    } = *fotograma;
    let ArranqueDeLaRama {
        parent_id,
        parent_pos,
        id_del_nodo,
        color_de_la_rama,
    } = arranque;

    // Iterativa, como el resto de recorridos del programa: una rama muy profunda agotaba la
    // pila. Cada entrada es la conexión que falta por dibujar: de quién sale y a quién va.
    let mut visitados: HashSet<Uuid> = HashSet::new();
    let mut pendientes: Vec<(Uuid, Pos2, Uuid)> = vec![(parent_id, parent_pos, id_del_nodo)];

    while let Some((parent_id, parent_pos, id_del_nodo)) = pendientes.pop() {
        if !visitados.insert(id_del_nodo) {
            continue; // Ya dibujado: hay un ciclo o un hijo compartido.
        }

        let Some(node) = app.mapa().proyecto().nodes.get(&id_del_nodo) else {
            continue;
        };

        // El nodo padre se busca en lugar de indexarlo directamente. La indexación entraba
        // en panic si el padre no existía, situación alcanzable con un mapa cuyo `parent_id`
        // apunte a un nodo ya borrado. Si falta, simplemente no se dibuja esta conexión.
        let Some(parent_node) = app.mapa().proyecto().nodes.get(&parent_id) else {
            continue;
        };

        let posicion_del_nodo = Pos2::new(node.pos[0], node.pos[1]);
        let parent_screen = a_pantalla(parent_pos);
        let node_screen = a_pantalla(posicion_del_nodo);

        let (pw, _) = estimar_tamano_del_nodo(&parent_node.title, parent_node.tags.len());
        let (nw, _) = estimar_tamano_del_nodo(&node.title, node.tags.len());

        // La línea sale del lado por el que queda el hijo, para que no cruce por encima del
        // propio nodo.
        let (from_pt, to_pt) = if posicion_del_nodo.x >= parent_pos.x {
            (
                Pos2::new(
                    parent_screen.x + (pw * app.lienzo().vista().zoom) / 2.0,
                    parent_screen.y,
                ),
                Pos2::new(
                    node_screen.x - (nw * app.lienzo().vista().zoom) / 2.0,
                    node_screen.y,
                ),
            )
        } else {
            (
                Pos2::new(
                    parent_screen.x - (pw * app.lienzo().vista().zoom) / 2.0,
                    parent_screen.y,
                ),
                Pos2::new(
                    node_screen.x + (nw * app.lienzo().vista().zoom) / 2.0,
                    node_screen.y,
                ),
            )
        };

        let puntos_de_la_curva = calcular_curva_bezier(from_pt, to_pt);

        // Una curva de Bézier no se sale del rectángulo que encierra sus puntos de control,
        // así que si ese rectángulo no toca la ventana, la curva tampoco.
        if area_visible.intersects(Rect::from_points(&puntos_de_la_curva)) {
            let grosor_del_trazo = (GROSOR_DE_LA_CONEXION_CRUZADA * app.lienzo().vista().zoom)
                .clamp(GROSOR_MINIMO_DEL_TRAZO, GROSOR_MAXIMO_DEL_TRAZO);
            painter.add(CubicBezierShape::from_points_stroke(
                puntos_de_la_curva,
                false,
                Color32::TRANSPARENT,
                Stroke::new(grosor_del_trazo, color_de_la_rama),
            ));
        }

        if !node.collapsed {
            for &id_del_hijo in &node.children {
                pendientes.push((id_del_nodo, posicion_del_nodo, id_del_hijo));
            }
        }
    }
}

/// Separación horizontal entre dos indicadores de la esquina superior derecha, a escala natural.
/// Es también la anchura que reserva cada uno en [`reserva_horizontal_de_indicadores`].
const PASO_ENTRE_INDICADORES: f32 = 16.0;

/// Distancia del borde derecho de la tarjeta al indicador más a la derecha, a escala natural.
const MARGEN_DERECHO_DE_LOS_INDICADORES: f32 = 10.0;

/// Distancia del borde superior de la tarjeta al título y a los indicadores, a escala natural.
const MARGEN_SUPERIOR_DEL_TITULO: f32 = 12.0;

/// Distancia del borde izquierdo de la tarjeta al título, a escala natural.
const MARGEN_IZQUIERDO_DEL_TITULO: f32 = 10.0;

/// Distancia del icono de notas a la esquina inferior derecha de la tarjeta, a escala natural.
const MARGEN_DEL_ICONO_DE_NOTAS: f32 = 14.0;

/// Lado de la zona sensible de un indicador o del icono de notas, a escala natural.
///
/// Algo menor que [`PASO_ENTRE_INDICADORES`] para que dos zonas vecinas no se toquen: el puntero
/// en la frontera no puede pertenecer a dos iconos a la vez.
const LADO_DE_LA_ZONA_DE_UN_ICONO: f32 = 14.0;

/// Un icono de una tarjeta que tiene explicación al pasar el ratón.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconoDelNodo {
    /// El emoji de estado delante del título.
    Estado,
    /// El indicador de prioridad, arriba a la derecha.
    Prioridad,
    /// El indicador de control humano, arriba a la derecha.
    Revision,
    /// El 📝 de las notas, abajo a la derecha.
    Notas,
    /// El icono del rol, abajo a la derecha, a la izquierda del de notas (PH-1007-3).
    Rol,
}

/// Dónde está, en pantalla, un icono de una tarjeta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZonaDeIcono {
    /// La zona sensible al ratón.
    pub caja: Rect,
    /// Qué icono es.
    pub icono: IconoDelNodo,
}

/// Esquina superior izquierda del título (y del emoji de estado, que va delante).
fn posicion_del_titulo(caja: Rect, zoom: f32) -> Pos2 {
    Pos2::new(
        caja.min.x + MARGEN_IZQUIERDO_DEL_TITULO * zoom,
        caja.min.y + MARGEN_SUPERIOR_DEL_TITULO * zoom,
    )
}

/// Esquina superior derecha de un indicador; el pintado lo alinea a ella.
///
/// # Parámetros
/// - `caja`: la tarjeta en pantalla.
/// - `zoom`: el acercamiento.
/// - `orden_desde_la_derecha`: 0 para el de más a la derecha, 1 para el siguiente…
pub fn ancla_del_indicador(caja: Rect, zoom: f32, orden_desde_la_derecha: usize) -> Pos2 {
    Pos2::new(
        caja.max.x
            - MARGEN_DERECHO_DE_LOS_INDICADORES * zoom
            - orden_desde_la_derecha as f32 * PASO_ENTRE_INDICADORES * zoom,
        caja.min.y + MARGEN_SUPERIOR_DEL_TITULO * zoom,
    )
}

/// Esquina inferior derecha del icono de rol: un paso a la izquierda del de notas, haya notas
/// o no, para que el rol no cambie de sitio al escribir una nota.
pub fn ancla_del_rol(caja: Rect, zoom: f32) -> Pos2 {
    ancla_de_las_notas(caja, zoom) - Vec2::new(PASO_ENTRE_INDICADORES * zoom, 0.0)
}

/// Esquina inferior derecha del icono de notas; el pintado lo alinea a ella.
pub fn ancla_de_las_notas(caja: Rect, zoom: f32) -> Pos2 {
    Pos2::new(
        caja.max.x - MARGEN_DEL_ICONO_DE_NOTAS * zoom,
        caja.max.y - MARGEN_DEL_ICONO_DE_NOTAS * zoom,
    )
}

/// Las zonas sensibles de los iconos de una tarjeta, con las mismas anclas que el pintado.
///
/// # Parámetros
/// - `caja`: la tarjeta en pantalla.
/// - `zoom`: el acercamiento.
/// - `indicadores`: los de [`calcular_indicadores_del_nodo`], en su orden.
/// - `con_notas`: si se pinta el icono de notas.
/// - `tamano_del_estado`: lo que ocupa el emoji de estado con la letra del título.
///
/// # Devuelve
/// Estado, después los indicadores en su orden, el rol y por último las notas, si las hay.
pub fn zonas_de_los_iconos(
    caja: Rect,
    zoom: f32,
    indicadores: &[IndicadorVisual],
    con_notas: bool,
    tamano_del_estado: Vec2,
) -> Vec<ZonaDeIcono> {
    let lado = LADO_DE_LA_ZONA_DE_UN_ICONO * zoom;
    let mut zonas = vec![ZonaDeIcono {
        caja: Rect::from_min_size(posicion_del_titulo(caja, zoom), tamano_del_estado),
        icono: IconoDelNodo::Estado,
    }];
    let cantidad = indicadores.len();
    for (posicion, indicador) in indicadores.iter().enumerate() {
        let ancla = ancla_del_indicador(caja, zoom, cantidad - 1 - posicion);
        zonas.push(ZonaDeIcono {
            caja: Rect::from_min_max(
                Pos2::new(ancla.x - lado, ancla.y),
                Pos2::new(ancla.x, ancla.y + lado),
            ),
            icono: if indicador.es_prioridad {
                IconoDelNodo::Prioridad
            } else {
                IconoDelNodo::Revision
            },
        });
    }
    let ancla = ancla_del_rol(caja, zoom);
    zonas.push(ZonaDeIcono {
        caja: Rect::from_min_max(Pos2::new(ancla.x - lado, ancla.y - lado), ancla),
        icono: IconoDelNodo::Rol,
    });
    if con_notas {
        let ancla = ancla_de_las_notas(caja, zoom);
        zonas.push(ZonaDeIcono {
            caja: Rect::from_min_max(Pos2::new(ancla.x - lado, ancla.y - lado), ancla),
            icono: IconoDelNodo::Notas,
        });
    }
    zonas
}

/// El texto emergente de un icono: «categoría: valor», con los nombres del inspector.
///
/// Es el único sitio donde se compone (las cadenas que ve el usuario se componen en un
/// solo sitio).
fn texto_del_icono(
    icono: IconoDelNodo,
    datos: &DatosDeLaTarjeta,
    idioma: crate::textos::Idioma,
) -> String {
    use crate::textos::{sin_dos_puntos, Texto};
    let categoria_y_valor = |categoria: Texto, valor: &str| {
        format!("{}: {valor}", sin_dos_puntos(categoria.en(idioma)))
    };
    match icono {
        IconoDelNodo::Estado => categoria_y_valor(
            Texto::InspectorEstado,
            datos.status.nombre_para_interfaz(idioma),
        ),
        IconoDelNodo::Prioridad => categoria_y_valor(
            Texto::InspectorPrioridad,
            datos.priority.nombre_para_interfaz(idioma),
        ),
        IconoDelNodo::Revision => categoria_y_valor(
            Texto::InspectorControlHumano,
            datos.review_status.nombre_para_interfaz(idioma),
        ),
        IconoDelNodo::Notas => Texto::IconoTieneNotas.en(idioma).to_string(),
        IconoDelNodo::Rol => {
            categoria_y_valor(Texto::InspectorRol, datos.role.nombre_para_interfaz(idioma))
        }
    }
}

/// Anota el texto emergente del icono que tiene el puntero encima, si hay alguno.
///
/// No se anota nada mientras se arrastra un nodo o se mantiene pulsado el botón: un texto
/// emergente persiguiendo al cursor durante un arrastre solo estorba.
fn anotar_el_icono_bajo_el_cursor(
    app: &AplicacionMapaMental,
    fotograma: &Fotograma,
    datos: &DatosDeLaTarjeta,
    geometria: &GeometriaDeLaTarjeta,
    acciones: &mut AccionesDelLienzo,
) {
    let Some(puntero) = fotograma.puntero.posicion else {
        return;
    };
    if fotograma.puntero.boton_pulsado
        || app.lienzo().vista().nodo_arrastrado.is_some()
        || !geometria.caja.contains(puntero)
    {
        return;
    }
    let zoom = fotograma.zoom;
    let tamano_del_estado = fotograma
        .painter
        .layout_no_wrap(
            datos.status.emoji().to_string(),
            FontId::proportional(tamano_de_letra_del_titulo(datos.es_la_raiz) * zoom),
            app.presentacion().tema().texto_principal,
        )
        .size();
    let indicadores = calcular_indicadores_del_nodo(datos.priority, datos.review_status);
    let icono = zonas_de_los_iconos(
        geometria.caja,
        zoom,
        &indicadores,
        !datos.sin_notas,
        tamano_del_estado,
    )
    .into_iter()
    .find(|zona| zona.caja.contains(puntero))
    .map(|zona| zona.icono);
    if let Some(icono) = icono {
        acciones.texto_emergente = Some((texto_del_icono(icono, datos, app.idioma()), puntero));
    }
}

/// Tamaño de letra del título, a escala natural: la raíz algo mayor.
fn tamano_de_letra_del_titulo(es_la_raiz: bool) -> f32 {
    if es_la_raiz {
        15.0
    } else {
        13.0
    }
}

/// Distancia del texto emergente al puntero, para que el cursor no lo tape.
const SEPARACION_DEL_TEXTO_EMERGENTE: Vec2 = Vec2::new(14.0, 14.0);

/// Pinta junto al puntero el texto emergente anotado en este fotograma.
///
/// # Parámetros
/// - `ctx`: el contexto de egui del fotograma.
/// - `texto`: lo que dice la nube, ya traducido.
/// - `puntero`: dónde está el cursor; la nube se abre un poco más abajo y a la derecha.
pub fn pintar_el_texto_emergente(ctx: &egui::Context, texto: String, puntero: Pos2) {
    egui::Area::new(egui::Id::new("texto_emergente_de_icono"))
        .order(egui::Order::Tooltip)
        .fixed_pos(puntero + SEPARACION_DEL_TEXTO_EMERGENTE)
        .interactable(false)
        .show(ctx, |ui| {
            // El área recuerda el tamaño del fotograma anterior y lo da como ancho máximo; todas
            // las nubes comparten esta área, así que sin fijar el ancho la de un texto largo
            // heredaba el de la última corta y salía con tres letras por línea (PH-1007-5). Se
            // fija el mismo que usan los textos emergentes propios de egui.
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(ui.spacing().tooltip_width);
                ui.label(texto)
            });
        });
}
