//! # Módulo de Panel Lateral e Inspector de Propiedades (`ui/sidebar.rs`)
//!
//! Proporciona el inspector contextual de nodos:
//! - Edición de título, rol arquitectónico, estado de madurez y prioridad.
//! - Asignación de ruta de archivo de código (`file_path`).
//! - Controles de supervisión humana (*Human-in-the-loop*) y notas de corrección exigidas a la IA.
//! - Editor de notas multilínea detalladas.
//! - Gestor interactivo de etiquetas (tags).
//! - Lista de conexiones cruzadas con acceso para eliminarlas.
//! - Resumen de visión global del proyecto.

use crate::aplicacion::AplicacionMapaMental;
use crate::model::{EstadoNodo, EstadoRevision, PrioridadNodo, RolNodo};
use crate::ui::help_system::TemaDeAyuda;
use egui::{RichText, ScrollArea, Ui};

/// Renderiza el panel lateral derecho con el inspector de propiedades y el panel de ayuda detallada.
///
/// Son dos paneles distintos que comparten el lado derecho de la ventana: el de ayuda
/// aparece por encima cuando hay un tema abierto, y el inspector queda debajo mostrando el
/// nodo seleccionado o, si no hay ninguno, un resumen del proyecto.
///
/// # Parámetros
/// - `app`: el estado de la aplicación, cuyos campos edita el inspector directamente.
/// - `ui`: la interfaz de `egui` del fotograma en curso.
pub fn dibujar_panel_lateral(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma_del_panel = app.idioma();
    // -------------------------------------------------------------
    // Panel de Ayuda Detallada (+info) si está abierto
    // -------------------------------------------------------------
    if app.presentacion_mut().ventanas().panel_de_ayuda {
        egui::Panel::right("detailed_help_side_panel")
            .min_size(360.0)
            .default_size(420.0)
            .max_size(550.0)
            .show(ui, |ui| {
                dibujar_guia_detallada(app, ui);
            });
    }

    // -------------------------------------------------------------
    // Inspector de Nodos y Proyecto
    // -------------------------------------------------------------
    egui::Panel::right("inspector_sidebar")
        .min_size(320.0)
        .default_size(360.0)
        .max_size(450.0)
        .show(ui, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    ui.add_space(8.0);

                    dibujar_buscador_de_nodos(app, ui);

                    if let Some(selected_id) = app.mapa().nodo_seleccionado() {
                        dibujar_inspector_del_nodo(app, ui, selected_id);
                    } else {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(
                                RichText::new(
                                    crate::textos::Texto::InspectorSeleccionaUnNodo
                                        .en(idioma_del_panel),
                                )
                                .heading()
                                .color(app.presentacion().tema().texto_atenuado),
                            );
                            ui.label(
                                crate::textos::Texto::InspectorSeleccionaUnNodoDetalle
                                    .en(idioma_del_panel),
                            );
                        });
                        ui.add_space(10.0);
                        app.dibujar_galleta_de_ayuda(ui, TemaDeAyuda::PrimerosPasos);
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Proyecto Overview Section
                    dibujar_resumen_del_proyecto(app, ui);
                });
        });
}

/// Dibuja el panel de ayuda detallada del tema activo.
///
/// Interpreta el texto de la guia como Markdown sencillo (títulos, viñetas y bloques de
/// código) y lo presenta con el formato correspondiente.
///
/// # Parámetros
/// - `app`: el estado, del que salen el tema de ayuda activo, el idioma y los colores.
/// - `ui`: la interfaz de `egui` en la que se dibuja el panel.
pub(crate) fn dibujar_guia_detallada(app: &mut AplicacionMapaMental, ui: &mut Ui) {
    let idioma_del_panel = app.idioma();
    let topic = app
        .presentacion_mut()
        .guia_activa()
        .unwrap_or(TemaDeAyuda::PrimerosPasos);

    dibujar_la_cabecera_de_la_guia(app, ui, topic, idioma_del_panel);
    dibujar_el_selector_de_tema(app, ui, topic, idioma_del_panel);

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            dibujar_el_texto_de_la_guia(app, ui, topic, idioma_del_panel);
            dibujar_las_acciones_de_la_guia(app, ui, idioma_del_panel);
        });
}

/// Dibuja el título de la guía y su botón de cierre.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque el botón cierra el panel.
/// - `ui`: la interfaz de `egui`.
/// - `topic`: el tema de ayuda que se está mostrando.
/// - `idioma_del_panel`: el idioma en que se escriben los textos.
fn dibujar_la_cabecera_de_la_guia(
    app: &mut AplicacionMapaMental,
    ui: &mut Ui,
    topic: TemaDeAyuda,
    idioma_del_panel: crate::textos::Idioma,
) {
    // El botón de cierre se reserva primero a la derecha para que un título largo no empuje
    // el botón fuera del panel ni ensanche el `Ui`.
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(
                    RichText::new(format!(
                        "✖ {}",
                        crate::textos::Texto::BotonCerrar.en(idioma_del_panel)
                    ))
                    .small(),
                )
                .clicked()
            {
                app.presentacion_mut().ventanas().panel_de_ayuda = false;
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(
                    egui::Label::new(
                        RichText::new(topic.titulo(idioma_del_panel))
                            .heading()
                            .color(app.presentacion().tema().colores_de_las_ramas[0]),
                    )
                    .wrap(),
                );
            });
        });
    });
}

/// Dibuja el desplegable con el que se cambia de tema de ayuda.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque elegir un tema lo cambia.
/// - `ui`: la interfaz de `egui`.
/// - `topic`: el tema que está seleccionado ahora.
/// - `idioma_del_panel`: el idioma en que se escriben los textos.
fn dibujar_el_selector_de_tema(
    app: &mut AplicacionMapaMental,
    ui: &mut Ui,
    topic: TemaDeAyuda,
    idioma_del_panel: crate::textos::Idioma,
) {
    ui.add_space(6.0);
    ui.label(
        RichText::new(crate::textos::Texto::AyudaCambiarTema.en(idioma_del_panel))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );

    // Selector desplegable ajustado al ancho disponible con truncado de texto largo
    egui::ComboBox::from_id_salt("help_topic_selector")
        .selected_text(topic.titulo(idioma_del_panel))
        .truncate()
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for t in TemaDeAyuda::TODOS {
                if ui
                    .selectable_label(t == topic, t.titulo(idioma_del_panel))
                    .clicked()
                {
                    *app.presentacion_mut().guia_activa() = Some(t);
                }
            }
        });
}

/// Dibuja el cuerpo de la guía, interpretando su Markdown línea a línea.
///
/// # Parámetros
/// - `app`: el estado, del que salen los colores del tema.
/// - `ui`: la interfaz de `egui`.
/// - `topic`: el tema de ayuda cuyo texto se dibuja.
/// - `idioma_del_panel`: el idioma en que se pide la guía.
fn dibujar_el_texto_de_la_guia(
    app: &AplicacionMapaMental,
    ui: &mut Ui,
    topic: TemaDeAyuda,
    idioma_del_panel: crate::textos::Idioma,
) {
    // La guía se dibuja interpretando el Markdown a mano: encabezados, listas y espaciado.
    // `egui` no trae un visor de Markdown, y traer uno por esto sería desproporcionado.
    let guide = topic.guia_completa(idioma_del_panel);
    for line in guide.lines() {
        dibujar_una_linea_de_la_guia(app, ui, line);
    }
}

/// Dibuja una sola línea de la guía con el formato que le corresponda.
///
/// `egui` no trae un visor de Markdown, y traer uno por esto sería desproporcionado, así que
/// se interpretan a mano los encabezados, las viñetas, las tablas, las citas y los bloques de código,
/// así como las marcas en línea de negrita (`**`), código (`` ` ``) y cursiva (`_`).
///
/// # Parámetros
/// - `app`: el estado, del que salen los colores del tema.
/// - `ui`: la interfaz de `egui`.
/// - `line`: la línea de la guía, tal cual viene.
fn dibujar_una_linea_de_la_guia(app: &AplicacionMapaMental, ui: &mut Ui, line: &str) {
    // Se usa `strip_prefix` en lugar de indexar por posición (`&line[2..]`).
    // La indexación de cadenas en Rust va por bytes, no por caracteres, y
    // entraría en panic si el corte cayera en mitad de un carácter multibyte.
    // Estas guías están llenas de emojis y de texto en español con tildes, así
    // que el riesgo era real aunque el prefijo comprobado fuese ASCII.
    if let Some(titulo) = line.strip_prefix("# ") {
        ui.add(
            egui::Label::new(
                RichText::new(titulo)
                    .heading()
                    .strong()
                    .color(app.presentacion().tema().colores_de_las_ramas[0]),
            )
            .wrap(),
        );
        ui.add_space(4.0);
    } else if let Some(subtitulo) = line.strip_prefix("### ") {
        ui.add(
            egui::Label::new(
                RichText::new(subtitulo)
                    .strong()
                    .color(app.presentacion().tema().colores_de_las_ramas[1]),
            )
            .wrap(),
        );
        ui.add_space(2.0);
    } else if line.starts_with("```") {
        // Borde de bloque de código: no se dibuja texto.
    } else if let Some(cita) = line.strip_prefix("> ") {
        // Cita en bloque: texto secundario ligeramente destacado.
        dibujar_parrafo_con_marcas(
            ui,
            app,
            cita,
            app.presentacion().tema().texto_secundario,
            None,
        );
        ui.add_space(2.0);
    } else if let Some(vineta) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        // No se usa `ui.horizontal` porque desactiva la envoltura de texto en `egui`
        // y ensancha el panel hasta romper el diseño.
        dibujar_parrafo_con_marcas(
            ui,
            app,
            vineta,
            app.presentacion().tema().texto_principal,
            Some("• "),
        );
    } else if line.starts_with("|") {
        ui.add(
            egui::Label::new(
                RichText::new(line)
                    .small()
                    .color(app.presentacion().tema().texto_secundario)
                    .monospace(),
            )
            .wrap(),
        );
    } else if line.trim().is_empty() {
        ui.add_space(4.0);
    } else {
        dibujar_parrafo_con_marcas(
            ui,
            app,
            line,
            app.presentacion().tema().texto_principal,
            None,
        );
    }
}

/// Dibuja una línea o viñeta de texto interpretando las marcas Markdown de negrita (`**`),
/// código en línea (`'`'`) y cursiva (`_`).
///
/// Construye un único [`egui::text::LayoutJob`] con los formatos tipográficos apropiados
/// y lo entrega a un [`egui::Label`] con envoltura habilitada (`wrap()`). No fragmenta la línea
/// en llamadas separadas a la interfaz ni usa contenedores horizontales, garantizando que el
/// texto fluya de forma continua y respete estrictamente los límites del panel lateral.
fn dibujar_parrafo_con_marcas(
    ui: &mut Ui,
    app: &AplicacionMapaMental,
    texto: &str,
    color_base: egui::Color32,
    prefijo: Option<&str>,
) {
    let mut job = egui::text::LayoutJob::default();
    let style = ui.style();

    if let Some(p) = prefijo {
        RichText::new(p).color(color_base).append_to(
            &mut job,
            style,
            egui::FontSelection::Default,
            egui::Align::Center,
        );
    }

    formatear_segmentos_en_layout(
        &mut job,
        style,
        texto,
        color_base,
        app.presentacion().tema().colores_de_las_ramas[0],
    );

    ui.add(egui::Label::new(job).wrap());
}

/// Divide el texto por las marcas de negrita (`**`) y código en línea (`'`'`),
/// añadiendo cada fragmento al [`egui::text::LayoutJob`] con su formato correspondiente.
fn formatear_segmentos_en_layout(
    job: &mut egui::text::LayoutJob,
    style: &egui::Style,
    texto: &str,
    color_base: egui::Color32,
    color_codigo: egui::Color32,
) {
    let partes_negrita: Vec<&str> = texto.split("**").collect();
    let pares_cerrados = !partes_negrita.len().is_multiple_of(2);

    for (indice_negrita, trozo_negrita) in partes_negrita.iter().enumerate() {
        if trozo_negrita.is_empty() {
            continue;
        }

        let es_negrita = if pares_cerrados {
            indice_negrita % 2 == 1
        } else {
            indice_negrita % 2 == 1 && indice_negrita < partes_negrita.len() - 1
        };

        let trozo_efectivo = if !pares_cerrados && indice_negrita == partes_negrita.len() - 1 {
            RichText::new("**").color(color_base).append_to(
                job,
                style,
                egui::FontSelection::Default,
                egui::Align::Center,
            );
            *trozo_negrita
        } else {
            *trozo_negrita
        };

        let partes_codigo: Vec<&str> = trozo_efectivo.split('`').collect();
        let codigo_cerrado = !partes_codigo.len().is_multiple_of(2);

        for (indice_codigo, fragmento) in partes_codigo.iter().enumerate() {
            if fragmento.is_empty() {
                continue;
            }

            let es_codigo = if codigo_cerrado {
                indice_codigo % 2 == 1
            } else {
                indice_codigo % 2 == 1 && indice_codigo < partes_codigo.len() - 1
            };

            let frag_efectivo = if !codigo_cerrado && indice_codigo == partes_codigo.len() - 1 {
                RichText::new("`").color(color_base).append_to(
                    job,
                    style,
                    egui::FontSelection::Default,
                    egui::Align::Center,
                );
                *fragmento
            } else {
                *fragmento
            };

            if es_codigo {
                let mut rich = RichText::new(frag_efectivo).monospace().color(color_codigo);
                if es_negrita {
                    rich = rich.strong();
                }
                rich.append_to(
                    job,
                    style,
                    egui::FontSelection::Default,
                    egui::Align::Center,
                );
            } else {
                formatear_cursiva_y_anadir(job, style, frag_efectivo, es_negrita, color_base);
            }
        }
    }
}

/// Procesa la cursiva delimitada por guiones bajos (`_`) en texto que no es código,
/// distinguiendo la cursiva intencional de los guiones bajos dentro de palabras o identificadores.
fn formatear_cursiva_y_anadir(
    job: &mut egui::text::LayoutJob,
    style: &egui::Style,
    texto: &str,
    es_negrita: bool,
    color: egui::Color32,
) {
    let mut resto = texto;
    while !resto.is_empty() {
        if let Some(pos_abrir) = resto.find('_') {
            let char_previo = resto[..pos_abrir].chars().last();
            let abre_valido = char_previo.map(|c| !c.is_alphanumeric()).unwrap_or(true);

            if abre_valido {
                if let Some(pos_cierre_rel) = resto[pos_abrir + 1..].find('_') {
                    let pos_cierre = pos_abrir + 1 + pos_cierre_rel;
                    let contenido_cursiva = &resto[pos_abrir + 1..pos_cierre];

                    let char_cierre_prev = contenido_cursiva.chars().last();
                    let char_cierre_post = resto[pos_cierre + 1..].chars().next();

                    let cierra_valido = !contenido_cursiva.is_empty()
                        && !contenido_cursiva.starts_with(' ')
                        && char_cierre_prev
                            .map(|c| !c.is_whitespace())
                            .unwrap_or(false)
                        && char_cierre_post
                            .map(|c| !c.is_alphanumeric())
                            .unwrap_or(true);

                    if cierra_valido {
                        let previo = &resto[..pos_abrir];
                        if !previo.is_empty() {
                            let mut rich = RichText::new(previo).color(color);
                            if es_negrita {
                                rich = rich.strong();
                            }
                            rich.append_to(
                                job,
                                style,
                                egui::FontSelection::Default,
                                egui::Align::Center,
                            );
                        }

                        let mut rich = RichText::new(contenido_cursiva).italics().color(color);
                        if es_negrita {
                            rich = rich.strong();
                        }
                        rich.append_to(
                            job,
                            style,
                            egui::FontSelection::Default,
                            egui::Align::Center,
                        );

                        resto = &resto[pos_cierre + 1..];
                        continue;
                    }
                }
            }

            let trozo = &resto[..=pos_abrir];
            let mut rich = RichText::new(trozo).color(color);
            if es_negrita {
                rich = rich.strong();
            }
            rich.append_to(
                job,
                style,
                egui::FontSelection::Default,
                egui::Align::Center,
            );
            resto = &resto[pos_abrir + 1..];
        } else {
            let mut rich = RichText::new(resto).color(color);
            if es_negrita {
                rich = rich.strong();
            }
            rich.append_to(
                job,
                style,
                egui::FontSelection::Default,
                egui::Align::Center,
            );
            break;
        }
    }
}

/// Dibuja los botones de acción que cierran la guía: centrar, reorganizar y editar la visión.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque los tres botones actúan sobre él.
/// - `ui`: la interfaz de `egui`.
/// - `idioma_del_panel`: el idioma en que se escriben los textos.
pub(crate) fn dibujar_las_acciones_de_la_guia(
    app: &mut AplicacionMapaMental,
    ui: &mut Ui,
    idioma_del_panel: crate::textos::Idioma,
) {
    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);
    ui.label(
        RichText::new(crate::textos::Texto::InspectorConsejosYAcciones.en(idioma_del_panel))
            .strong()
            .color(app.presentacion().tema().colores_de_las_ramas[1]),
    );
    ui.horizontal_wrapped(|ui| {
        if ui
            .button(crate::textos::Texto::InspectorCentrarVista.en(idioma_del_panel))
            .clicked()
        {
            app.centrar_en_la_raiz();
        }
        if ui
            .button(crate::textos::Texto::InspectorReorganizar.en(idioma_del_panel))
            .clicked()
        {
            // Mismo criterio que su gemelo del menú «Ver y Diseño»: en «Posición Libre
            // Manual» la disposición automática no toca nada, y callarse deja al usuario
            // pulsando un botón que no hace nada ni lo dice. La corrección de la 0.4.3
            // se aplicó solo al del menú y este se quedó mudo.
            if app.mapa_mut().proyecto_para_editar().layout_mode
                == crate::model::ModoDisposicion::FreeDrag
            {
                app.establecer_estado(
                    crate::textos::Texto::AvisoReorganizarNoAplica.en(app.idioma()),
                );
            } else {
                crate::layout::aplicar_disposicion_automatica(
                    app.mapa_mut().proyecto_para_editar(),
                );
                app.establecer_estado(
                    crate::textos::Texto::AvisoNodosReorganizados.en(app.idioma()),
                );
            }
        }
        if ui
            .button(crate::textos::Texto::InspectorEditarVision.en(idioma_del_panel))
            .clicked()
        {
            app.abrir_proyecto_ia_en_pestana(
                crate::ui::proyecto_ia_modal::PestanaProyectoIa::Proyecto,
            );
        }
    });
}

/// Tamaño que ocupará la tarjeta de un nodo en el lienzo.
///
/// Se consulta antes y después de editar el nodo para saber si el cambio obliga a repartir
/// el mapa otra vez. Recolocar es un recorrido del árbol entero: hacerlo en cada pulsación
/// de tecla cuesta siete milisegundos con veinte mil nodos, y casi siempre para dejar las
/// posiciones exactamente como estaban.
///
/// # Parámetros
/// - `nodo`: el nodo que se está editando.
///
/// # Devuelve
/// El par `(ancho, alto)` estimado, el mismo que usa la disposición.
fn tamano_de_la_tarjeta(nodo: &crate::model::Nodo) -> (f32, f32) {
    crate::layout::estimar_tamano_del_nodo(&nodo.title, !nodo.notes.is_empty(), nodo.tags.len())
}

/// Los colores y el idioma con los que se dibuja el inspector.
///
/// Va aparte porque el nodo se toma prestado en exclusiva mientras se dibujan sus campos, y
/// con ese préstamo vivo no se puede volver a mirar `app` para consultar el tema. Se copia lo
/// que hace falta antes de pedir el préstamo.
#[derive(Clone, Copy)]
struct EstiloDelInspector {
    /// El idioma en que se escriben las etiquetas del panel.
    idioma_del_panel: crate::textos::Idioma,
    /// Color de acento, el de la primera rama.
    color_de_la_rama: egui::Color32,
    /// Color con el que se avisa de algo delicado, como una corrección exigida.
    color_de_peligro: egui::Color32,
    /// Color de los textos de apoyo.
    color_atenuado: egui::Color32,
}

/// Lo que el inspector recoge mientras el nodo está prestado y aplica al soltarlo.
#[derive(Default)]
struct EcosDelInspector {
    /// Tema de ayuda que el usuario ha pedido abrir con alguna galleta «+info».
    ayuda_a_abrir: Option<TemaDeAyuda>,
    /// Ha cambiado algo de lo que decide el tamaño de la tarjeta, así que hay que repartir
    /// el mapa otra vez.
    cambio_el_tamano_de_la_tarjeta: bool,
}

/// Dibuja el inspector del nodo seleccionado.
///
/// Es el panel donde se edita todo lo que un nodo lleva dentro: su título, su rol, la ruta de
/// código asociada, las notas, las etiquetas y el estado de revisión humana.
///
/// # Parámetros
/// - `app`: estado de la aplicación.
/// - `ui`: contenedor de `egui` donde se dibuja.
/// - `id_del_nodo`: nodo que se está inspeccionando. Si ya no existe, no se dibuja nada.
pub(crate) fn dibujar_inspector_del_nodo(
    app: &mut AplicacionMapaMental,
    ui: &mut Ui,
    id_del_nodo: uuid::Uuid,
) {
    if !app
        .mapa_mut()
        .proyecto_para_editar()
        .nodes
        .contains_key(&id_del_nodo)
    {
        return;
    }

    let estilo = EstiloDelInspector {
        idioma_del_panel: app.idioma(),
        color_de_la_rama: app.presentacion().tema().colores_de_las_ramas[0],
        color_de_peligro: app.presentacion().tema().peligro,
        color_atenuado: app.presentacion().tema().texto_atenuado,
    };
    let mut ecos = EcosDelInspector::default();

    ui.heading(
        RichText::new(crate::textos::Texto::InspectorTitulo.en(estilo.idioma_del_panel))
            .color(estilo.color_de_la_rama),
    );
    ui.add_space(4.0);

    // Se saca del estado para poder editarlo mientras el nodo está prestado, y se devuelve
    // al final, cuando ese préstamo ya ha terminado.
    let mut etiqueta_en_escritura = std::mem::take(app.presentacion_mut().etiqueta_en_escritura());

    // Se comprueba en lugar de usar `unwrap`: el inspector se dibuja en cada fotograma y el
    // nodo puede haber desaparecido entre la comprobación de existencia y este punto. Antes
    // eso derribaba la aplicación; ahora simplemente no se dibuja el panel.
    let Some(node) = app
        .mapa_mut()
        .proyecto_para_editar()
        .nodes
        .get_mut(&id_del_nodo)
    else {
        *app.presentacion_mut().etiqueta_en_escritura() = etiqueta_en_escritura;
        return;
    };
    let nodo_antes_de_editar = node.clone();

    dibujar_el_titulo_y_el_rol(node, ui, estilo, &mut ecos);
    dibujar_la_ruta_del_archivo(node, ui, estilo, &mut ecos);
    dibujar_el_estado_del_trabajo(node, ui, estilo, &mut ecos);
    dibujar_el_control_humano(node, ui, estilo, &mut ecos);
    dibujar_la_prioridad(node, ui, estilo);
    dibujar_las_etiquetas(node, ui, estilo, &mut etiqueta_en_escritura, &mut ecos);
    dibujar_las_notas(node, ui, estilo, &mut ecos);
    let cambio_el_contenido_del_nodo = *node != nodo_antes_de_editar;

    // Aquí termina el préstamo del nodo, y a partir de este punto se puede volver a tocar
    // la aplicación entera.
    dibujar_los_botones_del_nodo(app, ui, estilo);
    dibujar_las_conexiones_cruzadas_del_nodo(app, ui, estilo, id_del_nodo);

    *app.presentacion_mut().etiqueta_en_escritura() = etiqueta_en_escritura;

    if cambio_el_contenido_del_nodo {
        app.mapa_mut().proyecto_para_editar().marcar_modificado();
    }

    if ecos.cambio_el_tamano_de_la_tarjeta {
        app.recolocar_si_procede();
    }

    if let Some(topic) = ecos.ayuda_a_abrir {
        app.abrir_ayuda(topic);
    }
}

/// Dibuja el título del concepto y el desplegable de su rol.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `ecos`: donde se anota si hay que recolocar el mapa o abrir una ayuda.
fn dibujar_el_titulo_y_el_rol(
    node: &mut crate::model::Nodo,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    ecos: &mut EcosDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel,
        color_de_la_rama,
        ..
    } = estilo;

    // Título del concepto.
    ui.label(
        RichText::new(crate::textos::Texto::InspectorTituloDelConcepto.en(idioma_del_panel))
            .strong(),
    );
    ecos.cambio_el_tamano_de_la_tarjeta |= ui.text_edit_singleline(&mut node.title).changed();
    ui.add_space(6.0);

    // Rol arquitectónico del nodo.
    ui.horizontal(|ui| {
        ui.label(crate::textos::Texto::InspectorRol.en(idioma_del_panel));
        egui::ComboBox::from_id_salt("node_role_combo")
            .selected_text(node.role.nombre_para_interfaz(idioma_del_panel))
            .show_ui(ui, |ui| {
                for variante in RolNodo::TODOS {
                    ui.selectable_value(
                        &mut node.role,
                        variante,
                        variante.nombre_para_interfaz(idioma_del_panel),
                    );
                }
            });
        if ui
            .button(
                RichText::new(crate::textos::Texto::GalletaMasInfo.en(idioma_del_panel))
                    .small()
                    .color(color_de_la_rama),
            )
            .on_hover_text(crate::textos::Texto::InspectorRolAyuda.en(idioma_del_panel))
            .clicked()
        {
            ecos.ayuda_a_abrir = Some(TemaDeAyuda::RolesYArquitectura);
        }
    });
    ui.add_space(4.0);
}

/// Dibuja el campo de la ruta del archivo de código asociado al nodo.
///
/// El botón de ayuda se reserva **antes** que el campo de texto, con un reparto de derecha a
/// izquierda. El orden importa: si el campo con `desired_width(f32::INFINITY)` se añade
/// primero, se queda con todo el ancho libre, el botón ya no cabe y `egui` ensancha el `Ui`
/// para meterlo. Ese ensanchamiento arrastra al panel entero, que al estar anclado a la
/// derecha crece hacia la izquierda, queda debajo del lienzo y pierde los primeros píxeles de
/// cada línea.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `ecos`: donde se anota si hay que abrir una ayuda.
fn dibujar_la_ruta_del_archivo(
    node: &mut crate::model::Nodo,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    ecos: &mut EcosDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel,
        color_de_la_rama,
        ..
    } = estilo;

    ui.horizontal(|ui| {
        ui.label(crate::textos::Texto::InspectorArchivoRuta.en(idioma_del_panel));
        let mut path_str = node.file_path.clone().unwrap_or_default();
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(
                    RichText::new(crate::textos::Texto::GalletaMasInfo.en(idioma_del_panel))
                        .small()
                        .color(color_de_la_rama),
                )
                .on_hover_text(crate::textos::Texto::InspectorArchivoRutaAyuda.en(idioma_del_panel))
                .clicked()
            {
                ecos.ayuda_a_abrir = Some(TemaDeAyuda::MapeoArchivosCodigo);
            }
            if ui
                .add(
                    egui::TextEdit::singleline(&mut path_str)
                        .desired_width(f32::INFINITY)
                        .hint_text(
                            crate::textos::Texto::InspectorArchivoRutaPista.en(idioma_del_panel),
                        ),
                )
                .changed()
            {
                node.file_path = if path_str.trim().is_empty() {
                    None
                } else {
                    Some(path_str.trim().to_string())
                };
            }
        });
    });
    ui.add_space(4.0);
}

/// Dibuja el desplegable del estado del trabajo del nodo.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `ecos`: donde se anota si hay que abrir una ayuda.
fn dibujar_el_estado_del_trabajo(
    node: &mut crate::model::Nodo,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    ecos: &mut EcosDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel,
        color_de_la_rama,
        ..
    } = estilo;

    // Estado de madurez del nodo.
    ui.horizontal(|ui| {
        ui.label(crate::textos::Texto::InspectorEstado.en(idioma_del_panel));
        egui::ComboBox::from_id_salt("node_status_combo")
            .selected_text(node.status.nombre_para_interfaz(idioma_del_panel))
            .show_ui(ui, |ui| {
                for variante in EstadoNodo::TODOS {
                    ui.selectable_value(
                        &mut node.status,
                        variante,
                        variante.nombre_para_interfaz(idioma_del_panel),
                    );
                }
            });
        if ui
            .button(
                RichText::new(crate::textos::Texto::GalletaMasInfo.en(idioma_del_panel))
                    .small()
                    .color(color_de_la_rama),
            )
            .on_hover_text(crate::textos::Texto::InspectorEstadoAyuda.en(idioma_del_panel))
            .clicked()
        {
            ecos.ayuda_a_abrir = Some(TemaDeAyuda::EstadosYProgreso);
        }
    });
    ui.add_space(4.0);
}

/// Dibuja el control humano: el estado de revisión y la corrección exigida a la IA.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `ecos`: donde se anota si hay que abrir una ayuda.
fn dibujar_el_control_humano(
    node: &mut crate::model::Nodo,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    ecos: &mut EcosDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel,
        color_de_la_rama,
        ..
    } = estilo;

    // Controles de supervisión: el estado de revisión y la corrección exigida a la IA.
    ui.horizontal(|ui| {
        ui.label(crate::textos::Texto::InspectorControlHumano.en(idioma_del_panel));
        egui::ComboBox::from_id_salt("node_review_combo")
            .selected_text(node.review_status.nombre_para_interfaz(idioma_del_panel))
            .show_ui(ui, |ui| {
                for variante in EstadoRevision::TODOS {
                    ui.selectable_value(
                        &mut node.review_status,
                        variante,
                        variante.nombre_para_interfaz(idioma_del_panel),
                    );
                }
            });
        if ui
            .button(
                RichText::new(crate::textos::Texto::GalletaMasInfo.en(idioma_del_panel))
                    .small()
                    .color(color_de_la_rama),
            )
            .on_hover_text(crate::textos::Texto::InspectorControlHumanoAyuda.en(idioma_del_panel))
            .clicked()
        {
            ecos.ayuda_a_abrir = Some(TemaDeAyuda::ControlHumanoCorrecciones);
        }
    });

    if node.review_status == EstadoRevision::RequiereCorreccion
        || !node.correction_feedback.is_empty()
    {
        ui.add_space(4.0);
        ui.label(
            RichText::new(crate::textos::Texto::InspectorCorreccionExigida.en(idioma_del_panel))
                .strong()
                .color(estilo.color_de_peligro),
        );
        ui.add(
            egui::TextEdit::multiline(&mut node.correction_feedback)
                .desired_rows(3)
                .desired_width(f32::INFINITY)
                .hint_text(
                    crate::textos::Texto::InspectorCorreccionExigidaPista.en(idioma_del_panel),
                ),
        );
    }
    ui.add_space(4.0);
}

/// Dibuja el desplegable de la prioridad del nodo.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
fn dibujar_la_prioridad(node: &mut crate::model::Nodo, ui: &mut Ui, estilo: EstiloDelInspector) {
    let EstiloDelInspector {
        idioma_del_panel, ..
    } = estilo;

    // Nodo Priority
    ui.horizontal(|ui| {
        ui.label(crate::textos::Texto::InspectorPrioridad.en(idioma_del_panel));
        egui::ComboBox::from_id_salt("node_priority_combo")
            .selected_text(node.priority.nombre_para_interfaz(idioma_del_panel))
            .show_ui(ui, |ui| {
                for variante in PrioridadNodo::TODOS {
                    ui.selectable_value(
                        &mut node.priority,
                        variante,
                        variante.nombre_para_interfaz(idioma_del_panel),
                    );
                }
            });
    });
    ui.add_space(8.0);
}

/// Dibuja las etiquetas del nodo, con el botón de quitar cada una y el campo para añadir.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `etiqueta_en_escritura`: el texto que el usuario está tecleando, sacado del estado
///   porque el nodo está prestado mientras tanto.
/// - `ecos`: donde se anota si el tamaño de la tarjeta ha cambiado.
fn dibujar_las_etiquetas(
    node: &mut crate::model::Nodo,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    etiqueta_en_escritura: &mut String,
    ecos: &mut EcosDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel, ..
    } = estilo;

    // Tags Section
    ui.label(RichText::new(crate::textos::Texto::InspectorEtiquetas.en(idioma_del_panel)).strong());
    ui.horizontal_wrapped(|ui| {
        let mut indice_a_quitar = None;
        for (i, tag) in node.tags.iter().enumerate() {
            let tag_label = format!("#{}", tag);
            if ui
                .button(RichText::new(tag_label).small())
                .on_hover_text(crate::textos::Texto::InspectorEliminarTagAyuda.en(idioma_del_panel))
                .clicked()
            {
                indice_a_quitar = Some(i);
            }
        }
        if let Some(i) = indice_a_quitar {
            // Quitar la última etiqueta encoge la tarjeta igual que añadir la primera la
            // agranda, y antes solo se recolocaba al añadir: el mapa se quedaba con los
            // huecos de un tamaño que ya no era el de nadie. Misma asimetría que la 0.4.3
            // corrigió para el título.
            let antes = tamano_de_la_tarjeta(node);
            node.tags.remove(i);
            ecos.cambio_el_tamano_de_la_tarjeta |= tamano_de_la_tarjeta(node) != antes;
        }
    });

    ui.horizontal(|ui| {
        let resp = ui.add(
            egui::TextEdit::singleline(&mut *etiqueta_en_escritura)
                .hint_text(crate::textos::Texto::InspectorAnadirTagPista.en(idioma_del_panel)),
        );
        if resp.lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
            && !etiqueta_en_escritura.trim().is_empty()
        {
            let etiqueta = etiqueta_en_escritura
                .trim()
                .trim_start_matches('#')
                .to_string();
            if !node.tags.contains(&etiqueta) {
                let antes = tamano_de_la_tarjeta(node);
                node.tags.push(etiqueta);
                ecos.cambio_el_tamano_de_la_tarjeta |= tamano_de_la_tarjeta(node) != antes;
            }
            // El campo queda listo para la siguiente, que es lo que espera quien está
            // etiquetando varias cosas seguidas.
            etiqueta_en_escritura.clear();
        }
    });
    ui.add_space(8.0);
}

/// Dibuja las notas del nodo, que son lo que de verdad lee el modelo.
///
/// # Parámetros
/// - `node`: el nodo que se está inspeccionando.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `ecos`: donde se anota si el tamaño de la tarjeta ha cambiado.
fn dibujar_las_notas(
    node: &mut crate::model::Nodo,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    ecos: &mut EcosDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel, ..
    } = estilo;

    // Las notas del nodo, que son lo que de verdad lee el modelo.
    ui.label(
        RichText::new(crate::textos::Texto::InspectorExplicacionParaLaIa.en(idioma_del_panel))
            .strong(),
    );
    ui.label(
        RichText::new(crate::textos::Texto::InspectorNotasLeidasPorLaIa.en(idioma_del_panel))
            .small()
            .color(estilo.color_atenuado),
    );
    // El alto de la tarjeta solo distingue entre tener notas y no tenerlas, así que de todo
    // lo que se escriba aquí únicamente el primer carácter —y el borrado del último— cambia
    // algo. Antes se recolocaba el mapa entero en **cada pulsación**.
    let tamano_antes_de_escribir = tamano_de_la_tarjeta(node);
    let escribio_en_las_notas = ui
        .add(
            egui::TextEdit::multiline(&mut node.notes)
                .desired_rows(6)
                .desired_width(f32::INFINITY)
                .hint_text(crate::textos::Texto::InspectorNotasPista.en(idioma_del_panel)),
        )
        .changed();
    if escribio_en_las_notas {
        // Lo ha escrito una persona: el mapa deja de ser trabajo exclusivo de la IA y el
        // servidor tiene que dejar copia antes de sobrescribirlo. Sin esta línea, un mapa
        // creado por un agente y anotado a mano se sobrescribía sin `.bak`.
        node.marcar_editado_por_una_persona();
    }
    ecos.cambio_el_tamano_de_la_tarjeta |=
        escribio_en_las_notas && tamano_de_la_tarjeta(node) != tamano_antes_de_escribir;
    ui.add_space(8.0);
}

/// Dibuja los botones que actúan sobre el nodo: añadir hijo, añadir hermano y eliminar.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque los tres botones lo modifican.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
fn dibujar_los_botones_del_nodo(
    app: &mut AplicacionMapaMental,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
) {
    let EstiloDelInspector {
        idioma_del_panel, ..
    } = estilo;

    // Acciones sobre el nodo que se está inspeccionando.
    ui.horizontal(|ui| {
        if ui
            .button(crate::textos::Texto::InspectorBotonHijo.en(idioma_del_panel))
            .on_hover_text(format!(
                "{} (Tab)",
                crate::textos::Texto::InspectorBotonHijoAyuda.en(idioma_del_panel)
            ))
            .clicked()
        {
            app.anadir_hijo_al_seleccionado();
        }
        if ui
            .button(crate::textos::Texto::InspectorBotonHermano.en(idioma_del_panel))
            .on_hover_text(format!(
                "{} (Enter)",
                crate::textos::Texto::InspectorBotonHermanoAyuda.en(idioma_del_panel)
            ))
            .clicked()
        {
            app.anadir_hermano_al_seleccionado();
        }
        if ui
            .button(crate::textos::Texto::InspectorBotonConexion.en(idioma_del_panel))
            .on_hover_text(crate::textos::Texto::InspectorBotonConexionAyuda.en(idioma_del_panel))
            .clicked()
        {
            // Misma puerta que la entrada «Crear Conexión Cruzada» del menú «Edición»: se anota
            // el origen y se abre el cuadro que pide el destino. No lleva atajo en la ayuda
            // emergente porque, a diferencia de hijo y hermano, esta operación no tiene ninguno.
            //
            // El inspector solo se dibuja cuando hay un nodo seleccionado, así que este `if let`
            // entra siempre. Se deja escrito de todos modos: dar por supuesta esa garantía sin
            // comprobarla es exactamente lo que mantuvo muerta esta función desde el primer
            // commit —el cuadro leía un origen que nadie había anotado y se cerraba solo—.
            if let Some(origen) = app.mapa().nodo_seleccionado() {
                app.lienzo_mut().editar_conexion().origen = Some(origen);
                app.presentacion_mut().ventanas().modal_conexion_cruzada = true;
            }
        }
        if ui
            .button(
                RichText::new(crate::textos::Texto::InspectorBotonEliminar.en(idioma_del_panel))
                    .color(estilo.color_de_peligro),
            )
            .on_hover_text(format!(
                "{} ({})",
                crate::textos::Texto::InspectorBotonEliminarAyuda.en(idioma_del_panel),
                crate::textos::Texto::TeclaSuprimir.en(idioma_del_panel)
            ))
            .clicked()
        {
            app.eliminar_nodo_seleccionado();
        }
    });
}

/// Dibuja las conexiones cruzadas en las que participa el nodo, con su botón de quitar.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque se puede quitar una conexión.
/// - `ui`: la interfaz de `egui`.
/// - `estilo`: idioma y colores del panel.
/// - `id_del_nodo`: el nodo cuyas conexiones se listan.
fn dibujar_las_conexiones_cruzadas_del_nodo(
    app: &mut AplicacionMapaMental,
    ui: &mut Ui,
    estilo: EstiloDelInspector,
    id_del_nodo: uuid::Uuid,
) {
    let EstiloDelInspector {
        idioma_del_panel, ..
    } = estilo;

    // Las conexiones cruzadas en las que participa este nodo, sea como origen o destino.
    let related_conns: Vec<_> = app
        .mapa_mut()
        .proyecto()
        .connections
        .iter()
        .filter(|c| c.from == id_del_nodo || c.to == id_del_nodo)
        .cloned()
        .collect();
    if !related_conns.is_empty() {
        ui.add_space(8.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(
                    crate::textos::Texto::InspectorRelacionesCruzadas.en(idioma_del_panel),
                )
                .strong(),
            );
            if ui
                .button(
                    RichText::new(crate::textos::Texto::GalletaMasInfo.en(idioma_del_panel))
                        .small()
                        .color(app.presentacion().tema().colores_de_las_ramas[0]),
                )
                .on_hover_text(
                    crate::textos::Texto::InspectorRelacionesCruzadasAyuda.en(idioma_del_panel),
                )
                .clicked()
            {
                app.abrir_ayuda(TemaDeAyuda::ConexionesCruzadas);
            }
        });
        let mut conexion_a_quitar = None;
        for conn in &related_conns {
            let is_from = conn.from == id_del_nodo;
            let other_id = if is_from { conn.to } else { conn.from };
            let other_title = app
                .mapa_mut()
                .proyecto()
                .nodes
                .get(&other_id)
                .map(|n| n.title.clone())
                .unwrap_or_else(|| "?".to_string());

            ui.horizontal(|ui| {
                if is_from {
                    ui.label(
                        RichText::new(format!(
                            "➡️ {} `{}`",
                            conn.relation_type.nombre_para_interfaz(idioma_del_panel),
                            other_title
                        ))
                        .small(),
                    );
                } else {
                    ui.label(
                        RichText::new(format!(
                            "⬅️ {} `{}`",
                            conn.relation_type.nombre_para_interfaz(idioma_del_panel),
                            other_title
                        ))
                        .small(),
                    );
                }
                if ui.add(egui::Button::new("❌").small()).clicked() {
                    conexion_a_quitar = Some(conn.id);
                }
            });
            if !conn.label.is_empty() {
                ui.label(
                    RichText::new(format!(
                        "   {} {}",
                        crate::textos::Texto::InspectorMotivo.en(idioma_del_panel),
                        conn.label
                    ))
                    .small()
                    .italics(),
                );
            }
        }
        if let Some(id) = conexion_a_quitar {
            app.mapa_mut()
                .proyecto_para_editar()
                .quitar_conexion_cruzada(id);
        }
    }
}

/// Dibuja la ficha del proyecto: visión, objetivos y métricas del mapa.
///
/// Se muestra **siempre**, con nodo seleccionado o sin él: va debajo del inspector, no en
/// su lugar. El comentario anterior decía lo contrario de lo que hace la única llamada.
///
/// Presenta la visión del creador, los objetivos y las métricas generales del mapa.
pub(crate) fn dibujar_resumen_del_proyecto(app: &mut AplicacionMapaMental, ui: &mut Ui) {
    let idioma_del_panel = app.idioma();
    ui.horizontal(|ui| {
        ui.heading(
            RichText::new(crate::textos::Texto::FichaProyectoTitulo.en(idioma_del_panel))
                .color(app.presentacion().tema().colores_de_las_ramas[1]),
        );
        if ui
            .button(
                RichText::new(crate::textos::Texto::GalletaMasInfo.en(idioma_del_panel))
                    .small()
                    .color(app.presentacion().tema().colores_de_las_ramas[0]),
            )
            .on_hover_text(crate::textos::Texto::FichaProyectoAyuda.en(idioma_del_panel))
            .clicked()
        {
            app.abrir_ayuda(TemaDeAyuda::VisionDelCreador);
        }
    });
    ui.add_space(4.0);

    ui.label(
        RichText::new(crate::textos::Texto::FichaProyectoTituloDelProyecto.en(idioma_del_panel))
            .strong(),
    );
    if ui
        .text_edit_singleline(&mut app.mapa_mut().proyecto_para_editar().title)
        .changed()
    {
        app.mapa_mut().proyecto_para_editar().marcar_modificado();
    }
    ui.add_space(6.0);

    ui.label(
        RichText::new(crate::textos::Texto::FichaProyectoVisionDelCreador.en(idioma_del_panel))
            .strong(),
    );
    if app
        .mapa_mut()
        .proyecto_para_editar()
        .creator_vision
        .trim()
        .is_empty()
    {
        ui.label(
            RichText::new(crate::textos::Texto::FichaProyectoSinVision.en(idioma_del_panel))
                .small()
                .color(app.presentacion().tema().colores_de_las_ramas[2]),
        );
    } else {
        let total = app
            .mapa_mut()
            .proyecto_para_editar()
            .creator_vision
            .chars()
            .count();
        let preview: String = app
            .mapa_mut()
            .proyecto()
            .creator_vision
            .chars()
            .take(120)
            .collect();
        let texto_preview = if total > 120 {
            format!("\"{}...\"", preview)
        } else {
            format!("\"{}\"", preview)
        };
        ui.label(
            RichText::new(texto_preview)
                .italics()
                .small()
                .color(app.presentacion().tema().texto_secundario),
        );
    }

    ui.add_space(4.0);
    if ui
        .button(crate::textos::Texto::FichaProyectoEditarVisionYObjetivos.en(idioma_del_panel))
        .on_hover_text(crate::textos::Texto::FichaProyectoEditarVisionAyuda.en(idioma_del_panel))
        .clicked()
    {
        app.abrir_proyecto_ia_en_pestana(crate::ui::proyecto_ia_modal::PestanaProyectoIa::Proyecto);
    }
}

/// Dibuja el buscador de nodos, en lo alto del panel lateral.
///
/// En un mapa de cien nodos, dar con uno a ojo obliga a abrir ramas plegadas y a mover la
/// cámara sin saber hacia dónde. Al elegir un resultado, el nodo queda seleccionado —con lo
/// que el inspector pasa a mostrarlo— y la vista va hasta él.
///
/// Va arriba del todo y siempre visible, también sin ningún nodo seleccionado: es justo
/// cuando alguien está perdido en su propio mapa cuando lo necesita.
///
/// # Parámetros
/// - `app`: estado de la aplicación; de aquí sale el mapa y aquí se guarda lo tecleado.
/// - `ui`: interfaz del panel donde se dibuja.
pub(crate) fn dibujar_buscador_de_nodos(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    ui.add(
        egui::TextEdit::singleline(&mut app.presentacion_mut().filtro_de_busqueda().texto)
            .hint_text(crate::textos::Texto::PanelBuscarNodo.en(idioma))
            .desired_width(f32::INFINITY),
    );

    dibujar_filtros_del_buscador(app, ui, idioma);

    // Sin nada pedido no se dice nada: todavía no se ha buscado, y un «ningún resultado» ahí
    // sería un error que nadie ha cometido.
    if app.presentacion_mut().filtro_de_busqueda().sin_usar() {
        ui.add_space(4.0);
        ui.separator();
        return;
    }

    // **Aquí vive la política del orden, y en ningún otro sitio.** Quien solo escribe texto
    // está buscando un nodo que ya tiene en la cabeza, y lo reconoce por dónde estaba: ese se
    // deja como en el mapa. Quien pone un filtro está pidiendo una lista de trabajo —«las
    // críticas que están en progreso»—, y ahí lo que manda es la prioridad.
    let filtro = {
        let filtro = app.presentacion_mut().filtro_de_busqueda();
        filtro.orden = if filtro.estado.is_some() || filtro.prioridad.is_some() {
            crate::model::OrdenDeResultados::PorPrioridad
        } else {
            crate::model::OrdenDeResultados::ComoEnElMapa
        };
        filtro.clone()
    };
    let encontrados = crate::busqueda::buscar_nodos(app.mapa().proyecto(), &filtro);

    // Los títulos se copian antes de dibujar. Dentro del bucle no se puede tener prestado el
    // mapa mientras se decide seleccionar un nodo, que es escribir en la misma aplicación.
    let opciones: Vec<(uuid::Uuid, String)> = encontrados
        .iter()
        .filter_map(|id| {
            app.mapa_mut()
                .proyecto()
                .nodes
                .get(id)
                .map(|nodo| (*id, nodo.title.clone()))
        })
        .collect();

    if opciones.is_empty() {
        ui.add_space(4.0);
        ui.label(
            RichText::new(crate::textos::Texto::PanelBusquedaSinResultados.en(idioma))
                .small()
                .color(app.presentacion().tema().texto_atenuado),
        );
        ui.add_space(4.0);
        ui.separator();
        return;
    }

    let mut elegido = None;
    ui.add_space(4.0);
    for (id, titulo) in &opciones {
        if ui
            .selectable_label(app.mapa().nodo_seleccionado() == Some(*id), titulo)
            .clicked()
        {
            elegido = Some(*id);
        }
    }

    if let Some(id) = elegido {
        app.mapa_mut().seleccionar_nodo(Some(id));
        app.centrar_en_el_nodo(id);
    }

    ui.add_space(4.0);
    ui.separator();
}

/// Pinta los dos desplegables que acotan la búsqueda por estado y por prioridad.
///
/// Van debajo del campo de texto y no dentro de él porque no son texto: son una elección
/// cerrada entre las variantes que el dominio ya define, y escribirlas a mano obligaría a
/// interpretar lo tecleado, que es justo lo que se evita teniéndolas tipadas.
///
/// Cada desplegable ofrece primero la opción de no filtrar, que es la que trae puesta.
///
/// # Parámetros
/// - `app`: estado de la aplicación; aquí se guarda lo elegido.
/// - `ui`: interfaz del panel donde se dibuja.
/// - `idioma`: el idioma con el que se rotulan las opciones.
fn dibujar_filtros_del_buscador(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
) {
    let sin_estado = crate::textos::Texto::PanelFiltroTodosLosEstados.en(idioma);
    let sin_prioridad = crate::textos::Texto::PanelFiltroTodasLasPrioridades.en(idioma);
    let filtro = app.presentacion_mut().filtro_de_busqueda();

    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_id_salt("search_status_combo")
            .selected_text(match filtro.estado {
                Some(estado) => estado.nombre_para_interfaz(idioma),
                None => sin_estado,
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut filtro.estado, None, sin_estado);
                for variante in EstadoNodo::TODOS {
                    ui.selectable_value(
                        &mut filtro.estado,
                        Some(variante),
                        variante.nombre_para_interfaz(idioma),
                    );
                }
            });

        egui::ComboBox::from_id_salt("search_priority_combo")
            .selected_text(match filtro.prioridad {
                Some(prioridad) => prioridad.nombre_para_interfaz(idioma),
                None => sin_prioridad,
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut filtro.prioridad, None, sin_prioridad);
                for variante in PrioridadNodo::TODOS {
                    ui.selectable_value(
                        &mut filtro.prioridad,
                        Some(variante),
                        variante.nombre_para_interfaz(idioma),
                    );
                }
            });
    });
}
