//! # Arnés de interfaz (`arnes_interfaz.rs`)
//!
//! Permite **ejecutar la interfaz dentro de una prueba**, sin abrir ninguna ventana, y mirar
//! qué ha salido pintado.
//!
//! ## Por qué existe
//!
//! Hasta la 0.4.9, la capa de interfaz no tenía una sola prueba que la ejecutara. Todo lo que
//! la vigilaba **leía el código fuente buscando texto**, y esa técnica falla de dos formas que
//! ya han ocurrido de verdad en este proyecto: un comentario cuenta como código, y `cargo fmt`
//! rompe la búsqueda al partir una línea. La decimotercera pasada encontró seis vallas que no
//! sujetaban nada, y las seis fallaban por eso.
//!
//! Lo que se perdía con ese método no es un detalle. Han aparecido **ocho controles que no
//! hacían absolutamente nada**, uno de ellos roto desde el primer commit, con las pruebas en
//! verde: el campo existía, el código compilaba y nadie pulsaba nunca ese botón.
//!
//! Con este arnés se puede comprobar lo que el programa **hace**, no cómo está escrito:
//!
//! - Que un menú tiene las entradas que debe tener, y que no se ha borrado ninguna.
//! - Que dos entradas distintas no acabaron apuntando al mismo texto.
//! - Que al cambiar de idioma cambia de verdad lo que se ve.
//!
//! ## Cómo se usa
//!
//! ```ignore
//! let mut app = aplicacion_de_prueba();
//! let textos = textos_de(&mut app, crate::ui::toolbar::menu_archivo);
//! assert!(textos.iter().any(|t| t.contains("Guardar")));
//! ```
//!
//! ## Lo que no alcanza
//!
//! No sustituye a probar el programa a mano. Los diálogos nativos del sistema —abrir y guardar
//! archivo— los pinta Windows, no `egui`, así que aquí no aparecen. Y la gestión del foco del
//! teclado en modo inmediato depende de la interacción real, que esto no reproduce.

use crate::aplicacion::AplicacionMapaMental;

/// Construye una aplicación lista para dibujarse en una prueba.
///
/// A diferencia de [`AplicacionMapaMental::nueva_con_contexto`], parte de un estado inicial
/// limpio y en memoria sin consultar preferencias ni archivos de recuperación en el disco del host.
///
/// Devuelve también el contexto, porque hace falta vivo mientras se dibuje.
///
/// # Devuelve
/// La aplicación y el contexto de `egui` sobre el que se ha construido.
pub fn aplicacion_de_prueba() -> (AplicacionMapaMental, egui::Context) {
    let ctx = egui::Context::default();
    let app = AplicacionMapaMental::nueva_limpia(&ctx);
    (app, ctx)
}

/// Dibuja algo de la interfaz y devuelve todos los textos que han quedado pintados.
///
/// Ejecuta un fotograma completo de `egui` en memoria. Lo que se recoge son los textos ya
/// compuestos —lo que **vería una persona**—, no los literales del código: si un rótulo sale de
/// una clave de traducción, aquí llega ya traducido.
///
/// # Parámetros
/// - `app`: la aplicación, que el dibujado puede modificar.
/// - `dibujar`: qué parte de la interfaz se dibuja. Recibe la aplicación y el `Ui` donde
///   pintar, igual que las funciones de menú de la barra de herramientas.
///
/// # Devuelve
/// Los textos pintados, en el orden en que se pintaron y sin los vacíos.
pub fn textos_de<F>(app: &mut AplicacionMapaMental, dibujar: F) -> Vec<String>
where
    F: FnMut(&mut AplicacionMapaMental, &mut egui::Ui),
{
    textos_de_en_pantalla(app, egui::vec2(1920.0, 1400.0), dibujar)
}

/// Dibuja una sección en una pantalla concreta y devuelve solo los textos realmente pintados.
///
/// Permite reproducir recortes verticales que una superficie holgada ocultaría.
pub fn textos_de_en_pantalla<F>(
    app: &mut AplicacionMapaMental,
    tamano: egui::Vec2,
    mut dibujar: F,
) -> Vec<String>
where
    F: FnMut(&mut AplicacionMapaMental, &mut egui::Ui),
{
    let ctx = egui::Context::default();
    let mut recogidos = Vec::new();

    let entrada = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, tamano)),
        ..Default::default()
    };
    let mut salida = ctx.run_ui(entrada, |ui| {
        dibujar(app, ui);
    });

    for forma in salida.shapes {
        recoger_textos(&forma.shape, &mut recogidos);
    }
    salida.textures_delta.clear();

    recogidos
}

/// Ejecuta una ventana real de `egui` en una pantalla concreta y devuelve sus textos visibles.
///
/// A diferencia de [`textos_de_en_pantalla`], el callback recibe el contexto completo. Esto
/// conserva la geometría, los márgenes y los límites que aplica [`egui::Window`] en producción.
pub fn textos_de_ventana_en_pantalla<F>(
    app: &mut AplicacionMapaMental,
    tamano: egui::Vec2,
    mut dibujar: F,
) -> Vec<String>
where
    F: FnMut(&mut AplicacionMapaMental, &egui::Context),
{
    let ctx = egui::Context::default();
    let mut recogidos = Vec::new();
    // Las ventanas piden descartar el primer pase para medir su tamaño. Un segundo pase es el
    // que contiene las formas realmente presentables, igual que en el bucle de integración.
    for _ in 0..2 {
        let entrada = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, tamano)),
            ..Default::default()
        };
        ctx.begin_pass(entrada);
        dibujar(app, &ctx);
        let mut salida = ctx.end_pass();
        recogidos.clear();
        for forma in salida.shapes {
            recoger_textos(&forma.shape, &mut recogidos);
        }
        salida.textures_delta.clear();
    }
    recogidos
}

/// Dibuja la barra de menús completa y devuelve los textos que han quedado pintados.
///
/// A diferencia de [`textos_de`], esto ejercita la barra tal como la dibuja el programa, con
/// su panel superior y su barra de galletas.
///
/// # Parámetros
/// - `app`: la aplicación, que el dibujado puede modificar.
///
/// # Devuelve
/// Los textos pintados.
pub fn textos_de_la_barra(app: &mut AplicacionMapaMental) -> Vec<String> {
    let ctx = egui::Context::default();
    let mut recogidos = Vec::new();

    let mut salida = ctx.run_ui(entrada_de_prueba(), |ui| {
        crate::ui::toolbar::dibujar_barra_de_herramientas(app, ui);
    });

    for forma in salida.shapes {
        recoger_textos(&forma.shape, &mut recogidos);
    }
    salida.textures_delta.clear();

    recogidos
}

/// Dibuja la vista previa de sesión y devuelve lo que una persona ve en la ventana.
pub fn textos_del_modal_sesion_agente(
    app: &mut AplicacionMapaMental,
    _ctx: &egui::Context,
) -> Vec<String> {
    textos_de(app, |app, ui| {
        crate::ui::sesion_agente_modal::dibujar_contenido_para_prueba(app, ui);
    })
}

/// Ejecuta una función de interfaz en dos fotogramas: en el primero localiza el centro del
/// texto indicado, y en el segundo simula un clic de ratón sobre esa posición exacta.
///
/// Esto permite ejercitar los bloques `if ui.button(...).clicked()` o
/// `if ui.selectable_label(...).clicked()` del código real de la interfaz.
///
/// # Parámetros
/// - `app`: la aplicación sobre la que se ejecuta la acción.
/// - `dibujar`: qué parte de la interfaz se dibuja y recibe la interacción.
/// - `texto_a_pulsar`: fragmento de texto del control que se desea pulsar.
///
/// # Devuelve
/// `true` si el texto fue encontrado y pulsado; `false` si no se encontró en pantalla.
pub fn hacer_clic_en_texto<F>(
    app: &mut AplicacionMapaMental,
    mut dibujar: F,
    texto_a_pulsar: &str,
) -> bool
where
    F: FnMut(&mut AplicacionMapaMental, &mut egui::Ui),
{
    let ctx = egui::Context::default();
    let mut pos_objetivo = None;

    // Fotograma 1: medir y encontrar las coordenadas del texto
    let mut salida1 = ctx.run_ui(entrada_de_prueba(), |ui| {
        dibujar(app, ui);
    });

    /// Recorre el árbol de formas dibujadas buscando dónde quedó un texto concreto.
    ///
    /// Se para en la primera coincidencia: si el mismo rótulo aparece dos veces, el clic
    /// simulado va al primero, que es el que vería primero una persona leyendo de arriba
    /// abajo.
    ///
    /// # Parámetros
    /// - `forma`: la forma a inspeccionar, que puede contener otras dentro.
    /// - `texto_buscado`: el rótulo que se busca, comparado ya recortado.
    /// - `encontrado`: dónde se deja el centro del texto si aparece. Se usa como salida y
    ///   además como señal de parada.
    fn buscar_posicion(
        forma: &egui::Shape,
        texto_buscado: &str,
        encontrado: &mut Option<egui::Pos2>,
    ) {
        if encontrado.is_some() {
            return;
        }
        match forma {
            egui::Shape::Text(texto) => {
                if texto.galley.text().contains(texto_buscado) {
                    let rect = texto.galley.rect.translate(texto.pos.to_vec2());
                    *encontrado = Some(rect.center());
                }
            }
            egui::Shape::Vec(formas) => {
                for f in formas {
                    buscar_posicion(f, texto_buscado, encontrado);
                }
            }
            _ => {}
        }
    }

    for forma in &salida1.shapes {
        buscar_posicion(&forma.shape, texto_a_pulsar, &mut pos_objetivo);
    }
    salida1.textures_delta.clear();

    let Some(pos) = pos_objetivo else {
        return false;
    };

    // Fotograma 2: simular el clic del ratón sobre esa posición
    let entrada_clic = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1920.0, 1400.0),
        )),
        events: vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
        ..Default::default()
    };

    let mut salida2 = ctx.run_ui(entrada_clic, |ui| {
        dibujar(app, ui);
    });
    salida2.textures_delta.clear();

    true
}

/// Pulsa varios controles **en orden, dentro de un mismo contexto de egui**, para poder
/// ejercitar interacciones que dependen del estado dejado por el clic anterior.
///
/// # Por qué existe, además de [`hacer_clic_en_texto`]
/// Ese ayudante crea un `egui::Context` nuevo en cada llamada, así que **la memoria de la
/// interfaz se pierde entre llamadas**: un `CollapsingHeader` que se abrió con el primer clic
/// aparece otra vez cerrado en el segundo. Aquí el contexto se conserva, de modo que
/// «despliega la fuente **y luego** pulsa su botón» funciona como para una persona.
///
/// El caso que lo motivó: `Aceptar esta versión` vive dentro de un `CollapsingHeader` que
/// nace plegado, así que **el botón no existe en el árbol de widgets** hasta desplegarlo.
///
/// # Parámetros
/// - `app`: la aplicación sobre la que se actúa.
/// - `dibujar`: la parte de interfaz que se dibuja y recibe los clics.
/// - `textos`: los rótulos a pulsar, **en el orden en que los pulsaría una persona**.
///
/// # Devuelve
/// El número de textos que se encontraron y se pulsaron. Si es menor que `textos.len()`,
/// el primero que faltó es `textos[devuelto]`, lo que permite un mensaje de fallo preciso.
pub fn hacer_clics_en_textos<F>(
    app: &mut AplicacionMapaMental,
    mut dibujar: F,
    textos: &[&str],
) -> usize
where
    F: FnMut(&mut AplicacionMapaMental, &mut egui::Ui),
{
    // Un solo contexto para toda la secuencia: es lo que conserva qué está desplegado.
    let ctx = egui::Context::default();
    // Sin animaciones: un `CollapsingHeader` recién abierto tarda varios fotogramas en
    // alcanzar su altura final, y hasta entonces su contenido no se pinta. Con la animación
    // a cero, el contenido existe ya en el fotograma siguiente al clic y la prueba es
    // determinista en lugar de depender de cuántos fotogramas se ejecuten.
    ctx.all_styles_mut(|estilo| estilo.animation_time = 0.0);
    let mut pulsados = 0;

    for texto in textos {
        // Dos pases antes de cada clic: `egui::Window` necesita el primero para medir y
        // colocar su contenido, y el segundo entrega ya las posiciones definitivas.
        let mut posicion = None;
        for _ in 0..2 {
            let mut salida = ctx.run_ui(entrada_de_prueba(), |ui| {
                dibujar(app, ui);
            });
            // epaint se queja si los deltas de textura se acumulan entre fotogramas del
            // mismo contexto. Aquí no se pinta de verdad, así que se descartan.
            salida.textures_delta.clear();
            posicion = None;
            for forma in &salida.shapes {
                buscar_posicion_de_texto(&forma.shape, texto, &mut posicion);
            }
        }

        let Some(destino) = posicion else {
            return pulsados;
        };

        let entrada = egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(destino),
                egui::Event::PointerButton {
                    pos: destino,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: destino,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
            ],
            ..entrada_de_prueba()
        };
        let mut salida_clic = ctx.run_ui(entrada, |ui| {
            dibujar(app, ui);
        });
        salida_clic.textures_delta.clear();
        pulsados += 1;
    }

    pulsados
}

/// Intenta enfocar un control de texto haciendo clic sobre él y escribir caracteres simulando el teclado.
///
/// Si el control es interactivo (`.interactive(true)`), `egui` le otorga el foco tras el clic
/// y el evento de texto es consumido por el widget, modificando su contenido.
/// Si el control es de solo lectura (`.interactive(false)` o etiqueta estática), el widget no
/// admite clics ni foco, y el evento de texto es ignorado sin mutar el búfer.
///
/// # Parámetros
/// - `app`: la aplicación sobre la que se actúa.
/// - `dibujar`: la parte de interfaz que se dibuja y recibe la interacción.
/// - `texto_a_buscar`: fragmento de texto del control sobre el que se hace clic.
/// - `texto_a_escribir`: caracteres a enviar por teclado.
///
/// # Devuelve
/// `(encontrado, obtuvo_foco)`:
/// - `encontrado`: `true` si el texto inicial fue localizado en pantalla para hacer clic sobre él.
/// - `obtuvo_foco`: `true` si algún widget de la interfaz obtuvo foco tras el clic y la pulsación.
pub fn intentar_escribir_en_texto_visible<F>(
    app: &mut AplicacionMapaMental,
    mut dibujar: F,
    texto_a_buscar: &str,
    texto_a_escribir: &str,
) -> (bool, bool)
where
    F: FnMut(&mut AplicacionMapaMental, &mut egui::Ui),
{
    let ctx = egui::Context::default();
    ctx.all_styles_mut(|estilo| estilo.animation_time = 0.0);

    // Pase 1 y 2: localizar la posición del texto en pantalla
    let mut posicion = None;
    for _ in 0..2 {
        let mut salida = ctx.run_ui(entrada_de_prueba(), |ui| {
            dibujar(app, ui);
        });
        salida.textures_delta.clear();
        posicion = None;
        for forma in &salida.shapes {
            buscar_posicion_de_texto(&forma.shape, texto_a_buscar, &mut posicion);
        }
    }

    let Some(destino) = posicion else {
        return (false, false);
    };

    // Pase 3: enviar clic del ratón para intentar otorgar foco
    let entrada_clic = egui::RawInput {
        events: vec![
            egui::Event::PointerMoved(destino),
            egui::Event::PointerButton {
                pos: destino,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: destino,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
        ..entrada_de_prueba()
    };
    let mut salida_clic = ctx.run_ui(entrada_clic, |ui| {
        dibujar(app, ui);
    });
    salida_clic.textures_delta.clear();

    // Pase 4: enviar evento de texto por teclado
    let entrada_teclado = egui::RawInput {
        events: vec![egui::Event::Text(texto_a_escribir.to_owned())],
        ..entrada_de_prueba()
    };
    let mut salida_teclado = ctx.run_ui(entrada_teclado, |ui| {
        dibujar(app, ui);
    });
    salida_teclado.textures_delta.clear();

    let obtuvo_foco = ctx.memory(|m| m.focused().is_some());
    (true, obtuvo_foco)
}

/// Recorre el árbol de formas buscando el centro del primer texto que contenga `texto_buscado`.
///
/// Extraído para que lo compartan los ayudantes de clic en lugar de duplicarlo.
///
/// # Parámetros
/// - `forma`: forma a inspeccionar; puede contener otras dentro.
/// - `texto_buscado`: fragmento buscado.
/// - `encontrado`: salida y a la vez señal de parada en la primera coincidencia.
fn buscar_posicion_de_texto(
    forma: &egui::Shape,
    texto_buscado: &str,
    encontrado: &mut Option<egui::Pos2>,
) {
    if encontrado.is_some() {
        return;
    }
    match forma {
        egui::Shape::Text(texto) => {
            if texto.galley.text().contains(texto_buscado) {
                let rect = texto.galley.rect.translate(texto.pos.to_vec2());
                *encontrado = Some(rect.center());
            }
        }
        egui::Shape::Vec(formas) => {
            for f in formas {
                buscar_posicion_de_texto(f, texto_buscado, encontrado);
            }
        }
        _ => {}
    }
}

/// Entrada de `egui` para un fotograma de prueba.
///
/// Da una ventana de tamaño holgado: con el rectángulo por omisión, que es diminuto, los
/// menús largos se recortan y sus últimas entradas no llegan a pintarse.
fn entrada_de_prueba() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1920.0, 1400.0),
        )),
        ..Default::default()
    }
}

/// Recorre una forma de `egui` y acumula los textos que contenga.
///
/// Las formas vienen anidadas: un `Vec` de formas puede contener otras, y hay que descender.
///
/// # Parámetros
/// - `forma`: la forma que se examina.
/// - `recogidos`: donde se acumulan los textos encontrados.
pub(crate) fn recoger_textos(forma: &egui::Shape, recogidos: &mut Vec<String>) {
    match forma {
        egui::Shape::Text(texto) => {
            let escrito = texto.galley.text().trim().to_string();
            if !escrito.is_empty() {
                recogidos.push(escrito);
            }
        }
        egui::Shape::Vec(formas) => {
            for interior in formas {
                recoger_textos(interior, recogidos);
            }
        }
        _ => {}
    }
}

/// Ejecuta un fotograma completo como lo procesa [`crate::aplicacion::AplicacionMapaMental`].
///
/// Dibuja la barra de menús superior, la barra de estado inferior, los paneles laterales
/// (ayuda e inspector de nodo) y el lienzo central con el mapa.
///
/// # Parámetros
/// - `app`: estado mutable de la aplicación.
/// - `ctx`: contexto de `egui` sobre el que se ejecuta el pase de interfaz.
/// - `screen_rect`: dimensiones de la pantalla simulada.
///
/// # Devuelve
/// La salida completa de renderizado de `egui` con todas las formas y texturas generadas.
pub fn ejecutar_fotograma_completo(
    app: &mut AplicacionMapaMental,
    ctx: &egui::Context,
    screen_rect: egui::Rect,
) -> egui::FullOutput {
    let entrada = egui::RawInput {
        screen_rect: Some(screen_rect),
        ..Default::default()
    };
    let mut salida = ctx.run_ui(entrada, |ui| {
        crate::ui::toolbar::dibujar_barra_de_herramientas(app, ui);
        egui::Panel::bottom("bottom_status_bar")
            .min_size(24.0)
            .show(ui, |_ui| {});
        crate::ui::sidebar::dibujar_panel_lateral(app, ui);
        crate::ui::canvas::dibujar_lienzo(app, ui);
    });

    // Las texturas se sueltan aquí, y no en cada prueba, porque olvidarlo tiene una
    // consecuencia desproporcionada: si una comprobación falla con la salida sin consumir,
    // `egui` aborta el proceso al destruirla y el fallo llega como
    // `STATUS_STACK_BUFFER_OVERRUN`, **sin el mensaje del aserto**. Quien lo viera pensaría
    // que la prueba no detecta el problema, cuando lo que pasa es que no lo puede contar.
    //
    // Ninguna prueba las usa: solo las limpiaba, una por una, y bastaba con que la siguiente
    // se olvidara.
    salida.textures_delta.clear();
    salida
}

/// Comprueba que ningún texto dibujado en paneles fijos o laterales quede recortado por la izquierda.
///
/// Verifica que cada forma de texto situada en paneles (`clip_rect.min.x > 1.0`) tenga
/// `rect.min.x >= clip_rect.min.x - 0.5`. Excluye el lienzo infinito (`clip_rect.min.x <= 1.0`)
/// donde los nodos navegan y pueden estar parcialmente fuera del viewport.
///
/// # Parámetros
/// - `salida`: la salida completa de `egui` del fotograma a verificar.
pub fn comprobar_que_ningun_texto_de_paneles_se_recorta(salida: &egui::FullOutput) {
    let mut recortados = Vec::new();
    for forma in &salida.shapes {
        recoger_textos_recortados_por_la_izquierda(&forma.shape, forma.clip_rect, &mut recortados);
    }
    assert!(
        recortados.is_empty(),
        "Se encontraron {} textos recortados por la izquierda en paneles: {:?}",
        recortados.len(),
        recortados
    );
}

/// Recorre las formas de un fotograma anotando los textos que se salen de su recorte.
///
/// Solo mira los paneles (`clip_rect.min.x > 1.0`), nunca el lienzo: ahí los nodos navegan
/// y salirse del borde es lo normal.
///
/// # Parámetros
/// - `forma`: la forma a inspeccionar, que puede contener otras dentro.
/// - `clip_rect`: la región de recorte que le aplica `egui`.
/// - `recortados`: donde se acumula lo encontrado, con el texto y las dos coordenadas que
///   no cuadran, para que el mensaje de la prueba diga cuánto se sale y por dónde.
fn recoger_textos_recortados_por_la_izquierda(
    forma: &egui::Shape,
    clip_rect: egui::Rect,
    recortados: &mut Vec<(String, f32, f32)>,
) {
    match forma {
        egui::Shape::Text(texto) => {
            let escrito = texto.galley.text().trim().to_string();
            let rect = texto.galley.rect.translate(texto.pos.to_vec2());
            if clip_rect.min.x > 1.0 && !escrito.is_empty() && rect.min.x < clip_rect.min.x - 0.5 {
                recortados.push((escrito, rect.min.x, clip_rect.min.x));
            }
        }
        egui::Shape::Vec(formas) => {
            for interior in formas {
                recoger_textos_recortados_por_la_izquierda(interior, clip_rect, recortados);
            }
        }
        _ => {}
    }
}

/// Mide el ancho de cada panel anclado al borde derecho en un fotograma ya dibujado.
///
/// Los paneles no publican su tamaño, así que se deduce de las regiones de recorte de las
/// formas dibujadas: un panel de la derecha recorta contra un rectángulo que llega al borde
/// derecho de la ventana y no empieza en el izquierdo.
///
/// # Parámetros
/// - `salida`: la salida completa de `egui` del fotograma.
/// - `ancho_de_la_ventana`: el ancho con el que se dibujó.
///
/// # Devuelve
/// Los anchos encontrados, de mayor a menor y sin repetidos.
pub fn anchos_de_los_paneles_de_la_derecha(
    salida: &egui::FullOutput,
    ancho_de_la_ventana: f32,
) -> Vec<f32> {
    let mut anchos: Vec<f32> = Vec::new();

    for forma in &salida.shapes {
        let clip = forma.clip_rect;
        let toca_el_borde_derecho = clip.max.x >= ancho_de_la_ventana - 1.0;
        let no_es_la_ventana_entera = clip.min.x > 1.0;

        if toca_el_borde_derecho && no_es_la_ventana_entera && clip.width().is_finite() {
            let ancho = clip.width();
            if !anchos.iter().any(|otro| (otro - ancho).abs() < 0.5) {
                anchos.push(ancho);
            }
        }
    }

    anchos.sort_by(|uno, otro| otro.total_cmp(uno));
    anchos
}
