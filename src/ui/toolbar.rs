//! # Módulo de Barra de Menús Estructurada (`ui/toolbar.rs`)
//!
//! Implementa la barra de menús de escritorio según los estándares de diseño:
//! - Menú `📁 Archivo`: Nuevo, Plantillas, Escanear Código, Abrir, Guardar, Exportar.
//! - Menú `✏️ Edición`: Añadir Hijo, Hermano, Eliminar, Editar, Conexiones, Visión.
//! - Menú `🤖 Inteligencia Artificial`: Importar, Prompt Maestro, Enviar Correcciones, Previsualizar, Exportar.
//! - Menú `🎨 Ver y Diseño`: disposición automática, centrar la vista y elección de tema.
//! - Menú `❓ Ayuda`: Selector de Galletas Guiadas, Guías Temáticas con [+info], Manual y Atajos.

use crate::aplicacion::AplicacionMapaMental;
use crate::model::ModoDisposicion;
use crate::textos::{Idioma, Texto};
use crate::theme::AppThemeMode;
use crate::ui::help_system::TemaDeAyuda;
use egui::RichText;

/// Dibuja la barra de herramientas superior con todos sus menús.
///
/// Agrupa las acciones de archivo, edición, disposición espacial, integración con IA,
/// tema visual y ayuda.
///
/// # Parámetros
/// - `app`: estado de la aplicación, que las acciones modifican.
/// - `ui`: interfaz de `egui` donde se dibuja.
pub fn dibujar_barra_de_herramientas(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    barra_de_menus(app, ui, idioma);

    if app.presentacion_mut().ventanas().galletas_de_ayuda {
        barra_de_la_galleta_de_ayuda(app, ui, idioma);
    }
}

/// La barra superior: el nombre del programa, los cinco menús y los indicadores de la derecha.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas de los menús.
/// - `ui`: el `Ui` en el que se ancla el panel superior.
/// - `idioma`: el del usuario, para los rótulos.
fn barra_de_menus(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    egui::Panel::top("top_menu_bar")
        .min_size(32.0)
        .show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.heading(
                    RichText::new("🧠 MMCelt")
                        .strong()
                        .color(app.presentacion().tema().colores_de_las_ramas[0]),
                );
                ui.separator();

                ui.menu_button(Texto::MenuArchivo.en(idioma), |ui| menu_archivo(app, ui));
                ui.menu_button(Texto::MenuEdicion.en(idioma), |ui| menu_edicion(app, ui));
                ui.menu_button(Texto::MenuInteligenciaArtificial.en(idioma), |ui| {
                    menu_inteligencia_artificial(app, ui)
                });
                ui.menu_button(Texto::MenuVerYDiseno.en(idioma), |ui| {
                    menu_ver_y_diseno(app, ui)
                });
                ui.menu_button(Texto::MenuAyuda.en(idioma), |ui| menu_ayuda(app, ui));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    indicadores_del_lado_derecho(app, ui, idioma);
                });
            });
        });
}

/// Lo que se muestra en el extremo derecho de la barra: el botón de la ayuda detallada, qué
/// archivo está abierto y los dos avisos del vigilante.
///
/// El punto verde dice que MMCelt está vigilando el archivo por si lo cambia un agente. El
/// triángulo dice algo más serio: el archivo ha cambiado en el disco **y** hay trabajo local
/// sin guardar, así que uno de los dos se va a perder si no se decide.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde salen la ruta abierta y el estado del vigilante.
/// - `ui`: el `Ui` de la barra, ya dispuesto de derecha a izquierda.
/// - `idioma`: el del usuario, para los rótulos.
fn indicadores_del_lado_derecho(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    if ui
        .button(
            RichText::new(if app.presentacion_mut().ventanas().panel_de_ayuda {
                Texto::BarraCerrarAyudaDetallada.en(idioma)
            } else {
                Texto::BarraAbrirAyudaDetallada.en(idioma)
            })
            .strong()
            .color(app.presentacion().tema().colores_de_las_ramas[0]),
        )
        .clicked()
    {
        app.presentacion_mut().ventanas().panel_de_ayuda =
            !app.presentacion_mut().ventanas().panel_de_ayuda;
        if app.presentacion_mut().ventanas().panel_de_ayuda
            && app.presentacion_mut().guia_activa().is_none()
        {
            *app.presentacion_mut().guia_activa() = Some(TemaDeAyuda::PrimerosPasos);
        }
    }

    if let Some(ref path) = app.persistencia_mut().ruta_actual {
        let nombre_del_archivo = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(Texto::BarraMapaSinNombre.en(idioma));
        ui.label(
            RichText::new(format!("📄 {}", nombre_del_archivo))
                .small()
                .color(app.presentacion().tema().texto_secundario),
        );
    } else {
        ui.label(
            RichText::new(Texto::BarraSinGuardar.en(idioma))
                .small()
                .italics()
                .color(app.presentacion().tema().texto_atenuado),
        );
    }

    if app.es_solo_lectura_por_raiz()
        && ui
            .button(
                RichText::new(format!("🔒 {}", Texto::BotonMoverMapaACarpeta.en(idioma)))
                    .small()
                    .strong()
                    .color(app.presentacion().tema().peligro),
            )
            .on_hover_text(Texto::ModalMapaEnRaizExplicacion.en(idioma))
            .clicked()
    {
        app.abrir_aviso_mapa_en_raiz();
    }

    if app.persistencia_mut().vigilante.is_some() {
        ui.label(RichText::new("🟢").small())
            .on_hover_text(Texto::IaVigilandoCambios.en(idioma));
    }

    if app.persistencia_mut().cambio_externo_pendiente {
        ui.label(
            RichText::new("⚠️")
                .small()
                .color(app.presentacion().tema().peligro),
        )
        .on_hover_text(Texto::AvisoCambioExternoConCambiosLocales.en(idioma));
    }
}

/// Qué tema de la ayuda viene a cuento según lo que el usuario tenga seleccionado.
///
/// Es la única decisión de la galleta de ayuda, y estaba escrita dentro del closure de
/// dibujado, anidada a seis niveles. Aquí es una función que se lee de un vistazo y que se
/// puede comprobar sin abrir una ventana.
///
/// El orden de las preguntas es el orden de urgencia: si el nodo pide una corrección, eso es
/// lo que hay que explicar; si no, si apunta a un archivo, lo que interesa es el mapeo de
/// código; y así hasta caer en la guía de primeros pasos, que es la respuesta cuando no hay
/// nada seleccionado o el nodo ya no existe.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde sale el nodo seleccionado.
///
/// # Devuelve
/// El tema de ayuda que se resume en la barra y que abre el botón «+info».
fn tema_de_galleta_segun_la_seleccion(app: &AplicacionMapaMental) -> TemaDeAyuda {
    let Some(id_seleccionado) = app.mapa().nodo_seleccionado() else {
        return TemaDeAyuda::PrimerosPasos;
    };
    let Some(nodo) = app.mapa().proyecto().nodes.get(&id_seleccionado) else {
        return TemaDeAyuda::PrimerosPasos;
    };

    if nodo.review_status == crate::model::EstadoRevision::RequiereCorreccion {
        TemaDeAyuda::ControlHumanoCorrecciones
    } else if nodo.file_path.is_some() {
        TemaDeAyuda::MapeoArchivosCodigo
    } else if nodo.status == crate::model::EstadoNodo::DudaBloqueo {
        TemaDeAyuda::EstadosYProgreso
    } else {
        TemaDeAyuda::AtajosYCreacionNodos
    }
}

/// La segunda barra: el aviso breve para quien empieza, con su botón «+info».
///
/// Solo se dibuja si el usuario no la ha apagado. La ✖ del extremo derecho la apaga, y desde
/// ese momento no vuelve salvo que se reactive en el menú de ayuda.
///
/// # Parámetros
/// - `app`: estado de la aplicación; de aquí sale el nodo seleccionado y aquí se apaga la barra.
/// - `ui`: el `Ui` en el que se ancla el panel.
/// - `idioma`: el del usuario, para los rótulos y el resumen.
fn barra_de_la_galleta_de_ayuda(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    egui::Panel::top("cookie_help_bar")
        .min_size(26.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let tema = tema_de_galleta_segun_la_seleccion(app);

                ui.label(
                    RichText::new(Texto::GalletaGuiaRapida.en(idioma))
                        .strong()
                        .color(app.presentacion().tema().colores_de_las_ramas[1]),
                );
                ui.label(
                    RichText::new(tema.resumen_de_galleta(idioma))
                        .small()
                        .color(app.presentacion().tema().texto_secundario),
                );

                if ui
                    .button(
                        RichText::new(Texto::GalletaMasInfo.en(idioma))
                            .small()
                            .strong()
                            .color(app.presentacion().tema().colores_de_las_ramas[0]),
                    )
                    .on_hover_text(Texto::GalletaMasInfoAyuda.en(idioma))
                    .clicked()
                {
                    app.abrir_ayuda(tema);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(
                            RichText::new("✖")
                                .small()
                                .color(app.presentacion().tema().texto_atenuado),
                        )
                        .on_hover_text(Texto::GalletaOcultarAyuda.en(idioma))
                        .clicked()
                    {
                        app.presentacion_mut().ventanas().galletas_de_ayuda = false;
                    }
                });
            });
        });
}

/// Contenido del menú «Archivo».
///
/// Nuevo mapa, ejemplos, plantillas, abrir, guardar, exportar y escanear código.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
pub(crate) fn menu_archivo(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    entradas_para_empezar_un_mapa(app, ui, idioma);
    ui.separator();
    entradas_de_apertura_y_guardado(app, ui, idioma);
    ui.separator();
    entrada_de_exportacion(app, ui, idioma);
}

/// Las cuatro formas de empezar un mapa: en blanco, desde un ejemplo, desde una plantilla o
/// escaneando una carpeta de código.
///
/// Los ejemplos van antes que las plantillas a propósito: quien abre el programa por primera
/// vez necesita ver un mapa *terminado*, con notas y dudas, antes que una estructura vacía que
/// rellenar.
///
/// # Parámetros
/// - `app`: estado de la aplicación; cualquiera de las cuatro sustituye el mapa abierto.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos y para los títulos del mapa que se cree.
fn entradas_para_empezar_un_mapa(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    if ui
        .button(format!("{}\tCtrl+N", Texto::ArchivoNuevoMapa.en(idioma)))
        .clicked()
    {
        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nuevo_vacio(Texto::ArchivoNuevoProyectoDefecto.en(idioma)),
            None,
            Texto::AvisoProyectoNuevo.en(idioma),
        );
        ui.close();
    }

    ui.menu_button(Texto::ArchivoEjemplos.en(idioma), |ui| {
        submenu_ejemplos(app, ui)
    });

    ui.menu_button(Texto::ArchivoPlantillas.en(idioma), |ui| {
        submenu_plantillas(app, ui)
    });

    if ui
        .button(Texto::ArchivoEscanearCarpeta.en(idioma))
        .on_hover_text(Texto::ArchivoEscanearCarpetaAyuda.en(idioma))
        .clicked()
    {
        if let Some(folder) =
            crate::ui::dialogos::pedir_carpeta(Texto::ArchivoDialogoSeleccionarCarpeta.en(idioma))
        {
            match crate::escaner_repositorio::escanear_carpeta(&folder, idioma) {
                Ok(proyecto_escaneado) => {
                    let aviso = Texto::AvisoRepositorioMapeado
                        .en(idioma)
                        .replace("{}", &folder.display().to_string());
                    app.sustituir_el_mapa_abierto(proyecto_escaneado, None, &aviso);
                }
                Err(e) => {
                    // Por el punto central, no con un texto a mano: así
                    // el error queda registrado en el archivo y el usuario
                    // recibe la sugerencia que lleva asociada su tipo.
                    app.reportar_error(&e, Texto::ContextoEscanearCarpeta.en(idioma));
                }
            }
        }
        ui.close();
    }
}

/// Abrir un mapa del disco, guardarlo y guardarlo con otro nombre.
///
/// Los dos errores posibles —no se puede leer el archivo, o lo que hay dentro no es un mapa
/// válido— se comunican por el punto central de errores, que es lo que deja constancia en el
/// registro y le da al usuario la sugerencia que corresponde al tipo de fallo.
///
/// # Parámetros
/// - `app`: estado de la aplicación; abrir sustituye el mapa y guardar fija su ruta.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos y los avisos.
fn entradas_de_apertura_y_guardado(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    if ui.button(Texto::ArchivoAbrirMapa.en(idioma)).clicked() {
        if let Some(path) = crate::ui::dialogos::pedir_archivo_para_abrir(idioma) {
            match app.abrir_mapa_desde(&path) {
                Ok(proyecto_abierto) => {
                    let aviso = Texto::AvisoArchivoAbierto
                        .en(idioma)
                        .replace("{}", &path.display().to_string());
                    app.sustituir_el_mapa_abierto(proyecto_abierto, Some(path.clone()), &aviso);
                }
                Err(e) => {
                    app.reportar_error(&e, Texto::ContextoAbrirMapaMental.en(idioma));
                }
            }
        }
        ui.close();
    }

    if ui
        .button(format!("{}\tCtrl+S", Texto::ArchivoGuardar.en(idioma)))
        .clicked()
    {
        app.solicitar_guardado(false);
        ui.close();
    }

    if ui.button(Texto::ArchivoGuardarComo.en(idioma)).clicked() {
        app.solicitar_guardado(true);
        ui.close();
    }

    if ui.button(Texto::ArchivoImportarOpml.en(idioma)).clicked() {
        crate::exportacion_opml_comando::importar(app);
        ui.close();
    }

    if ui
        .button(Texto::ArchivoImportarFreemind.en(idioma))
        .clicked()
    {
        crate::exportacion_freemind_comando::importar(app);
        ui.close();
    }
}

/// La exportación del mapa a Markdown para dárselo a un modelo de IA.
///
/// Va destacada —en negrita y con el color de la primera rama— y sola en su sección porque es
/// la razón de ser del programa: lo demás sirve para construir el mapa, y esto para entregarlo.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde sale el mapa que se exporta.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para el rótulo.
fn entrada_de_exportacion(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    if ui
        .button(
            RichText::new(format!(
                "{}\tCtrl+E",
                Texto::ArchivoExportarMarkdown.en(idioma)
            ))
            .strong()
            .color(app.presentacion().tema().colores_de_las_ramas[0]),
        )
        .clicked()
    {
        app.exportar_markdown_dialogo();
        ui.close();
    }

    if ui.button(Texto::ArchivoExportarOpml.en(idioma)).clicked() {
        crate::exportacion_opml_comando::exportar(app);
        ui.close();
    }

    if ui
        .button(Texto::ArchivoExportarFreemind.en(idioma))
        .clicked()
    {
        crate::exportacion_freemind_comando::exportar(app);
        ui.close();
    }
}

/// Contenido del menú «Ejemplos».
///
/// Mapas ya terminados, con notas, dudas y conexiones, para ver cómo se anota un mapa de verdad.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
fn submenu_ejemplos(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    ui.label(
        RichText::new(Texto::EjemplosEncabezado.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    ui.separator();

    if ui
        .button(Texto::EjemploAhorcado.en(idioma))
        .on_hover_text(Texto::EjemploAhorcadoAyuda.en(idioma))
        .clicked()
    {
        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nuevo_ejemplo_juego_ahorcado(idioma),
            None,
            Texto::AvisoEjemploTecnicoCargado.en(idioma),
        );
        ui.close();
    }

    if ui
        .button(Texto::EjemploNegocio.en(idioma))
        .on_hover_text(Texto::EjemploNegocioAyuda.en(idioma))
        .clicked()
    {
        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nuevo_ejemplo_negocio_comida(idioma),
            None,
            Texto::AvisoEjemploNegocioCargado.en(idioma),
        );
        ui.close();
    }
}

/// Contenido del menú «Plantillas de arquitectura».
///
/// Crea un mapa con la estructura de una arquitectura conocida, listo para rellenar.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
fn submenu_plantillas(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    if ui
        .button(Texto::PlantillaArquitecturaLimpia.en(idioma))
        .clicked()
    {
        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nueva_plantilla_arquitectura_limpia(idioma),
            None,
            Texto::AvisoPlantillaArquitecturaCargada.en(idioma),
        );
        ui.close();
    }
    if ui.button(Texto::PlantillaFullstack.en(idioma)).clicked() {
        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nueva_plantilla_fullstack(idioma),
            None,
            Texto::AvisoPlantillaFullstackCargada.en(idioma),
        );
        ui.close();
    }
}

/// Contenido del menú «Edición».
///
/// Añadir nodos hijos y hermanos, eliminar el seleccionado, trazar conexiones cruzadas y
/// editar la visión del proyecto.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
pub(crate) fn menu_edicion(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    entradas_de_deshacer_y_rehacer(app, ui, idioma);
    ui.separator();
    entradas_de_edicion_de_nodos(app, ui, idioma);
    ui.separator();
    entradas_de_relaciones_y_vision(app, ui, idioma);
}

/// Deshacer y rehacer, que encabezan el menú como en cualquier editor.
///
/// Van deshabilitados cuando no hay nada que deshacer o rehacer: un botón que no responde y no
/// explica por qué es el defecto que este proyecto ya se ha encontrado ocho veces.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el historial dice si cada uno está disponible.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn entradas_de_deshacer_y_rehacer(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    let solo_lectura = app.es_solo_lectura_por_raiz();
    if ui
        .add_enabled(
            app.mapa().puede_deshacer() && !solo_lectura,
            egui::Button::new(format!("{}\tCtrl+Z", Texto::EdicionDeshacer.en(idioma))),
        )
        .clicked()
    {
        app.deshacer();
        ui.close();
    }
    if ui
        .add_enabled(
            app.mapa().puede_rehacer() && !solo_lectura,
            egui::Button::new(format!("{}\tCtrl+Y", Texto::EdicionRehacer.en(idioma))),
        )
        .clicked()
    {
        app.rehacer();
        ui.close();
    }
}

/// Las cuatro operaciones sobre el nodo seleccionado: añadir hijo, añadir hermano, eliminar y
/// editar su texto.
///
/// Cada una lleva al lado el atajo que hace lo mismo, para que quien use el menú acabe
/// aprendiéndose el teclado.
///
/// # Parámetros
/// - `app`: estado de la aplicación; las cuatro actúan sobre el nodo seleccionado.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos y los nombres de las teclas.
fn entradas_de_edicion_de_nodos(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    let solo_lectura = app.es_solo_lectura_por_raiz();
    if ui
        .add_enabled(
            !solo_lectura,
            egui::Button::new(format!("{}\tTab", Texto::EdicionAnadirHijo.en(idioma))),
        )
        .clicked()
    {
        app.anadir_hijo_al_seleccionado();
        ui.close();
    }
    if ui
        .add_enabled(
            !solo_lectura,
            egui::Button::new(format!("{}\tEnter", Texto::EdicionAnadirHermano.en(idioma))),
        )
        .clicked()
    {
        app.anadir_hermano_al_seleccionado();
        ui.close();
    }
    if ui
        .add_enabled(
            !solo_lectura,
            egui::Button::new(format!(
                "{}\t{}",
                Texto::EdicionEliminarNodo.en(idioma),
                Texto::TeclaSuprimir.en(idioma)
            )),
        )
        .clicked()
    {
        app.eliminar_nodo_seleccionado();
        ui.close();
    }
    if ui
        .add_enabled(
            !solo_lectura,
            egui::Button::new(format!(
                "{}\t{} / F2",
                Texto::EdicionEditarTextoDelNodo.en(idioma),
                Texto::TeclaEspacio.en(idioma)
            )),
        )
        .clicked()
    {
        // Sin nodo seleccionado no había ni edición ni aviso: la entrada de menú parecía
        // estropeada. La de al lado sí avisa; esta callaba.
        //
        // Se llama al mismo método que el atajo `Espacio`, en vez de repetir aquí la
        // comprobación. Antes eran dos copias de la misma lógica, y pasó lo que pasa siempre:
        // se corrigió una y la otra se quedó muda. Con una sola, arreglar una arregla las dos.
        app.abrir_el_titulo_en_edicion();
        ui.close();
    }
}

/// Crear una conexión cruzada y abrir la visión y las metas del proyecto.
///
/// Las dos abren una ventana en vez de actuar directamente sobre el mapa, y por eso van juntas
/// y al final, separadas de lo que sí lo modifica en el acto.
///
/// # Parámetros
/// - `app`: estado de la aplicación; aquí se anota el origen de la conexión y qué ventana abrir.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos y el aviso.
pub(crate) fn entradas_de_relaciones_y_vision(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    if ui
        .button(Texto::EdicionCrearConexionCruzada.en(idioma))
        .clicked()
    {
        // El origen se anota aquí. Sin esta línea, el modal se cerraba solo en el mismo
        // fotograma en que se abría —lee el origen y se rinde si no hay ninguno—, así que
        // pulsar esta entrada del menú no hacía absolutamente nada visible. Era la única
        // forma de crear una conexión cruzada desde el programa: las que existían llegaban
        // importadas de una IA, de un ejemplo o de editar el archivo.
        if let Some(origen) = app.mapa().nodo_seleccionado() {
            app.lienzo_mut().editar_conexion().origen = Some(origen);
            app.presentacion_mut().ventanas().modal_conexion_cruzada = true;
        } else {
            app.establecer_estado(Texto::AvisoSeleccionaNodoDeOrigen.en(idioma));
        }
        ui.close();
    }
    if ui
        .button(Texto::ModalProyectoEInstruccionesIa.en(idioma))
        .clicked()
    {
        app.abrir_proyecto_ia_en_pestana(crate::ui::proyecto_ia_modal::PestanaProyectoIa::Proyecto);
        ui.close();
    }
}

/// Contenido del menú «Inteligencia Artificial».
///
/// Exportar para la IA, importar su respuesta, enviar correcciones y conectar agentes.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
pub(crate) fn menu_inteligencia_artificial(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    if app.persistencia_mut().vigilante.is_some() {
        aviso_de_vigilancia_activa(app, ui, idioma);
        ui.separator();
    }

    submenu_enviar_a_un_agente(app, ui, idioma);
    entrada_de_conexion_con_los_agentes(app, ui, idioma);

    ui.separator();
    entradas_de_intercambio_con_la_ia(app, ui, idioma);

    ui.separator();
    entradas_de_revision_y_entrega(app, ui, idioma);
}

/// Aviso de que MMCelt está vigilando el archivo, con el botón para dejar de hacerlo.
///
/// Solo aparece mientras hay vigilancia en marcha. Sin él, el usuario no tendría forma de
/// saber que el programa está pendiente del disco ni de pararlo.
///
/// # Parámetros
/// - `app`: estado de la aplicación, donde vive el vigilante.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn aviso_de_vigilancia_activa(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(Texto::IaVigilandoCambios.en(idioma))
                .small()
                .strong(),
        );
        if ui.button(Texto::IaDetenerSeguimiento.en(idioma)).clicked() {
            app.detener_seguimiento_del_mapa();
            ui.close();
        }
    });
}

/// Submenú «Enviar a…»: los agentes de IA detectados en el equipo.
///
/// El icono de cada uno dice por dónde se le puede hablar, que no es lo mismo para todos:
/// consola y MCP, solo consola, solo MCP, o ninguno de los dos porque no está instalado.
///
/// Si no hay ninguno detectado se dice, en vez de dejar un submenú vacío que parece
/// estropeado.
///
/// # Parámetros
/// - `app`: estado de la aplicación; de aquí salen los agentes detectados.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn submenu_enviar_a_un_agente(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    /// Ancho mínimo del submenú, para que quepan los nombres de los agentes sin cortarse.
    const ANCHO_MINIMO: f32 = 220.0;

    let menu_enviar = ui.menu_button(
        RichText::new(Texto::IaMenuEnviarA.en(idioma))
            .strong()
            .color(app.presentacion().tema().colores_de_las_ramas[1]),
        |ui| {
            ui.set_min_width(ANCHO_MINIMO);
            let agentes = app
                .agentes_mut()
                .detectados
                .clone()
                .unwrap_or_else(crate::conectores::detectar_agentes);
            if agentes.is_empty() {
                ui.label(
                    RichText::new(Texto::IaSinAgentesDetectados.en(idioma))
                        .small()
                        .color(app.presentacion().tema().texto_atenuado),
                );
                return;
            }
            for agente in &agentes {
                let icono = match agente.disponibilidad_de_envio() {
                    crate::conectores::DisponibilidadEnvio::ConsolaYMcp => "✨",
                    crate::conectores::DisponibilidadEnvio::SoloConsola => "🖥",
                    crate::conectores::DisponibilidadEnvio::SoloMcp => "🔗",
                    crate::conectores::DisponibilidadEnvio::NoDisponible => "🔌",
                };
                if ui
                    .button(format!("{icono} {}", agente.nombre))
                    .on_hover_text(agente.descripcion)
                    .clicked()
                {
                    app.preparar_sesion_agente(agente);
                    ui.close();
                }
            }
        },
    );
    menu_enviar
        .response
        .on_hover_text(Texto::IaMenuEnviarAAyuda.en(idioma));
}

/// La entrada que abre la ventana de conexión con los agentes instalados.
///
/// Va destacada y de las primeras porque es el paso que desbloquea todo lo demás: sin
/// conectar, el resto del menú obliga a copiar y pegar a mano.
///
/// # Parámetros
/// - `app`: estado de la aplicación; aquí se descarta la detección anterior.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn entrada_de_conexion_con_los_agentes(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    if ui
        .button(
            RichText::new(Texto::IaConectarConMisIas.en(idioma))
                .strong()
                .color(app.presentacion().tema().colores_de_las_ramas[0]),
        )
        .on_hover_text(Texto::IaConectarConMisIasAyuda.en(idioma))
        .clicked()
    {
        // Se fuerza una búsqueda nueva: puede haberse instalado algún agente desde la última
        // vez que se abrió la ventana.
        app.agentes_mut().detectados = None;
        app.presentacion_mut().ventanas().modal_conexiones = true;
        ui.close();
    }
}

/// Traer un mapa escrito por una IA, y copiar el texto maestro que se le entrega.
///
/// Son las dos mitades del intercambio manual, el que funciona con cualquier modelo aunque no
/// esté conectado por MCP.
///
/// # Parámetros
/// - `app`: estado de la aplicación; las dos abren su ventana.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn entradas_de_intercambio_con_la_ia(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    if ui
        .button(RichText::new(Texto::IaImportarDesdeIa.en(idioma)).strong())
        .clicked()
    {
        app.presentacion_mut().ventanas().modal_importar = true;
        ui.close();
    }
    if ui.button(Texto::IaCopiarPromptMaestro.en(idioma)).clicked() {
        app.presentacion_mut().ventanas().modal_prompt_maestro = true;
        ui.close();
    }
}

/// Revisar lo que ha hecho la IA y entregarle el mapa: correcciones, vista previa y exportación.
///
/// «Enviar correcciones» va en el color de peligro a propósito: es la orden con la que una
/// persona le enmienda la plana a la máquina, y conviene que no se pulse por inercia.
///
/// # Parámetros
/// - `app`: estado de la aplicación; las dos primeras abren ventana y la tercera exporta.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn entradas_de_revision_y_entrega(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
) {
    if ui
        .button(
            RichText::new(Texto::IaEnviarCorrecciones.en(idioma))
                .color(app.presentacion().tema().peligro),
        )
        .clicked()
    {
        app.presentacion_mut().ventanas().modal_correcciones = true;
        ui.close();
    }
    if ui
        .button(Texto::IaPrevisualizarMarkdown.en(idioma))
        .clicked()
    {
        app.presentacion_mut().ventanas().modal_vista_previa = true;
        ui.close();
    }
    if ui
        .button(format!("{}\tCtrl+E", Texto::IaExportarArchivoMd.en(idioma)))
        .clicked()
    {
        app.exportar_markdown_dialogo();
        ui.close();
    }
}

/// Contenido del menú «Ver y Diseño».
///
/// Tamaño de la interfaz, disposición de los nodos y elección de tema.
///
/// El tamaño de la interfaz no es el zoom del mapa: aquello agranda textos y botones, y
/// esto acerca el lienzo. Se manejan con controles distintos a propósito.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
pub(crate) fn menu_ver_y_diseno(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    escala_de_la_interfaz(app, ui, idioma);
    ui.separator();
    disposicion_de_los_nodos(app, ui, idioma);
    ui.separator();
    encuadre_del_mapa(app, ui, idioma);
    ui.separator();
    eleccion_del_idioma(app, ui, idioma);
    ui.separator();
    eleccion_del_tema(app, ui, idioma);
    ui.separator();
    intervalo_de_autoguardado(app, ui, idioma);
}

/// Sección «Tamaño de la interfaz» del menú «Ver y Diseño».
///
/// Va la primera del menú a propósito: quien usa la aplicación en un televisor o en una
/// pantalla grande necesita encontrarlo sin buscar, y con la interfaz demasiado pequeña
/// rebuscar en submenús es justamente lo que cuesta.
///
/// Ofrece las escalas sugeridas, un par de botones para ajustar a mano de décima en décima, y
/// el aviso de que la elección se recuerda entre sesiones.
///
/// # Parámetros
/// - `app`: estado de la aplicación; la escala se guarda en sus preferencias.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn escala_de_la_interfaz(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    ui.label(
        RichText::new(Texto::VerTamanoDeLaInterfaz.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );

    let escala_actual = app.presentacion_mut().preferencias().escala_interfaz;

    for (escala, texto_descripcion) in crate::preferencias::ESCALAS_SUGERIDAS {
        let seleccionada = (escala_actual - escala).abs() < 0.01;
        let etiqueta = format!("{:.0} %", escala * 100.0);

        if ui
            .selectable_label(seleccionada, etiqueta)
            .on_hover_text(texto_descripcion.en(idioma))
            .clicked()
        {
            app.cambiar_escala_interfaz(ui.ctx(), *escala);
            ui.close();
        }
    }

    ui.horizontal(|ui| {
        if ui
            .button("➖")
            .on_hover_text(Texto::VerReducirTamanoAyuda.en(idioma))
            .clicked()
        {
            app.cambiar_escala_interfaz(ui.ctx(), escala_actual - 0.1);
        }

        ui.label(
            RichText::new(format!("{:.0} %", escala_actual * 100.0))
                .strong()
                .color(app.presentacion().tema().colores_de_las_ramas[0]),
        );

        if ui
            .button("➕")
            .on_hover_text(Texto::VerAumentarTamanoAyuda.en(idioma))
            .clicked()
        {
            app.cambiar_escala_interfaz(ui.ctx(), escala_actual + 0.1);
        }
    });

    ui.label(
        RichText::new(Texto::VerTamanoSeRecuerda.en(idioma))
            .small()
            .italics()
            .color(app.presentacion().tema().texto_atenuado),
    );
}

/// Sección «Disposición espacial»: cómo se colocan los nodos en el lienzo.
///
/// Elegir árbol horizontal o radial recoloca el mapa en el acto. «Posición Libre Manual» no
/// recoloca nada: es la opción que respeta lo que el usuario haya arrastrado.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el modo se guarda en el propio proyecto.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn disposicion_de_los_nodos(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    let disposicion_anterior = app.mapa_mut().proyecto_para_editar().layout_mode;
    ui.label(
        RichText::new(Texto::VerDisposicionEspacial.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    if ui
        .selectable_value(
            &mut app.mapa_mut().proyecto_para_editar().layout_mode,
            ModoDisposicion::HorizontalTree,
            format!(
                "🌳 {}",
                ModoDisposicion::HorizontalTree.nombre_para_interfaz(idioma)
            ),
        )
        .clicked()
    {
        crate::layout::aplicar_disposicion_automatica(app.mapa_mut().proyecto_para_editar());
        ui.close();
    }
    if ui
        .selectable_value(
            &mut app.mapa_mut().proyecto_para_editar().layout_mode,
            ModoDisposicion::RadialTree,
            format!(
                "⭕ {}",
                ModoDisposicion::RadialTree.nombre_para_interfaz(idioma)
            ),
        )
        .clicked()
    {
        crate::layout::aplicar_disposicion_automatica(app.mapa_mut().proyecto_para_editar());
        ui.close();
    }
    if ui
        .selectable_value(
            &mut app.mapa_mut().proyecto_para_editar().layout_mode,
            ModoDisposicion::FreeDrag,
            format!(
                "🖐️ {}",
                ModoDisposicion::FreeDrag.nombre_para_interfaz(idioma)
            ),
        )
        .clicked()
    {
        ui.close();
    }
    if app.mapa_mut().proyecto_para_editar().layout_mode != disposicion_anterior {
        app.mapa_mut().proyecto_para_editar().marcar_modificado();
    }
}

/// Sección de encuadre: centrar la vista y volver a repartir los nodos.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las dos órdenes.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn encuadre_del_mapa(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    if ui
        .button(format!("{}\tCtrl+F", Texto::VerCentrarVista.en(idioma)))
        .clicked()
    {
        app.centrar_en_la_raiz();
        ui.close();
    }
    if ui
        .button(Texto::VerReorganizarLosNodos.en(idioma))
        .clicked()
    {
        // En «Posición Libre Manual» la disposición automática no toca nada, y antes se
        // anunciaba «Nodos reorganizados» igualmente: el usuario arrastraba los nodos, se
        // liaba, pulsaba aquí, no pasaba nada y el programa le decía que sí había pasado.
        if app.mapa_mut().proyecto_para_editar().layout_mode == ModoDisposicion::FreeDrag {
            app.establecer_estado(Texto::AvisoReorganizarNoAplica.en(app.idioma()));
        } else {
            crate::layout::aplicar_disposicion_automatica(app.mapa_mut().proyecto_para_editar());
            app.establecer_estado(Texto::AvisoNodosReorganizados.en(app.idioma()));
        }
        ui.close();
    }
}

/// Sección «Idioma»: en cuál de los seis se dirige la aplicación al usuario.
///
/// La elección se guarda en el acto, como la del tema: si el programa se cierra de golpe, no
/// se pierde.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el idioma vive en sus preferencias.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el que está puesto ahora, para el rótulo de la sección.
fn eleccion_del_idioma(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    ui.label(
        RichText::new(Texto::VerIdioma.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    for idioma_ofrecido in Idioma::TODOS {
        let elegido = app.presentacion_mut().preferencias().idioma == idioma_ofrecido;
        let rotulo = format!(
            "{} {}",
            idioma_ofrecido.bandera(),
            idioma_ofrecido.nombre_para_interfaz()
        );
        if ui.selectable_label(elegido, rotulo).clicked() {
            app.presentacion_mut().preferencias().idioma = idioma_ofrecido;
            // Se guarda al momento, como el tema: si el programa se cierra de golpe, la
            // elección no se pierde.
            if let Err(error) = app.guardar_preferencias() {
                crate::error::registrar(&error, "guardar preferencia de idioma");
            }
            ui.close();
        }
    }
}

/// Sección «Tema visual»: claro, oscuro o de alto contraste.
///
/// Los nombres salen de [`AppThemeMode::nombre_para_interfaz`], que es donde se definen:
/// repetirlos aquí obligaba a tocar dos sitios por cada tema.
///
/// Van como opciones marcables y no como botones sueltos para que se vea cuál está puesto:
/// con tres botones iguales, quien abría el menú no tenía forma de saber en qué tema estaba.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el tema se aplica y se guarda al elegirlo.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn eleccion_del_tema(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    ui.label(
        RichText::new(Texto::VerTemaVisual.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    for modo in AppThemeMode::TODOS {
        let activo = app.presentacion().tema().modo == modo;
        if ui
            .selectable_label(activo, modo.nombre_para_interfaz(idioma))
            .clicked()
        {
            app.establecer_tema(ui.ctx(), modo);
            ui.close();
        }
    }
}

/// Sección «Autoguardado»: cada cuánto se guarda solo, o si no se guarda.
///
/// Va la última del menú porque es lo que menos se toca: se elige una vez y se olvida. Pero
/// tiene que estar, porque antes eran dos minutos fijos en el código y no había forma de
/// cambiarlos ni de apagarlo sin recompilar el programa.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el intervalo vive en el control de autoguardado.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
fn intervalo_de_autoguardado(app: &mut AplicacionMapaMental, ui: &mut egui::Ui, idioma: Idioma) {
    ui.label(
        RichText::new(Texto::VerAutoguardado.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );

    let minutos_actuales = app.persistencia_mut().autoguardado.minutos();
    for minutos in crate::autoguardado::MINUTOS_DE_AUTOGUARDADO_OFRECIDOS {
        // El cero apaga el autoguardado, y el uno lleva el minuto en singular: «Cada 1
        // minutos» delata que el rótulo se compuso sin mirar lo que decía.
        let etiqueta = match minutos {
            0 => Texto::AutoguardadoDesactivado.en(idioma).to_string(),
            1 => Texto::AutoguardadoCadaMinuto.en(idioma).to_string(),
            varios => {
                Texto::AutoguardadoCadaNMinutos
                    .en(idioma)
                    .replacen("{}", &varios.to_string(), 1)
            }
        };

        if ui
            .selectable_label(minutos == minutos_actuales, etiqueta)
            .clicked()
        {
            app.fijar_minutos_de_autoguardado(minutos);
            ui.close();
        }
    }
}

/// Contenido del menú «Ayuda».
///
/// Galletas de ayuda, guías del panel lateral y datos de la compilación.
///
/// Recibe el `Ui` que abre `ui.menu_button`, así que dibuja solo lo que va dentro del
/// desplegable, no el botón que lo despliega.
///
/// # Parámetros
/// - `app`: estado de la aplicación, sobre el que actúan las entradas del menú.
/// - `ui`: el `Ui` del desplegable ya abierto.
pub(crate) fn menu_ayuda(app: &mut AplicacionMapaMental, ui: &mut egui::Ui) {
    let idioma = app.idioma();

    // «Acerca de» es donde se busca esta información por costumbre, así
    // que va aquí además de en la barra de estado.
    if ui
        .button(Texto::AyudaAcercaDe.en(idioma))
        .on_hover_text(Texto::AyudaAcercaDeAyuda.en(idioma))
        .clicked()
    {
        app.presentacion_mut().ventanas().modal_acerca_de = true;
        ui.close();
    }

    ui.separator();

    let mut cookies_toggle = app.presentacion_mut().ventanas().galletas_de_ayuda;
    if ui
        .checkbox(
            &mut cookies_toggle,
            Texto::AyudaGalletasPrincipiantes.en(idioma),
        )
        .changed()
    {
        app.presentacion_mut().ventanas().galletas_de_ayuda = cookies_toggle;
    }
    ui.separator();

    ui.label(
        RichText::new(Texto::AyudaGuiasRapidas.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    entradas_de_ayuda(app, ui, idioma, &GUIAS_RAPIDAS);

    ui.separator();
    ui.label(
        RichText::new(Texto::AyudaIntegracionesConIas.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    entradas_de_ayuda(app, ui, idioma, &INTEGRACIONES_CON_IAS);

    ui.separator();
    if ui
        .button(Texto::AyudaVerTodosLosAtajos.en(idioma))
        .clicked()
    {
        app.presentacion_mut().ventanas().modal_atajos = true;
        ui.close();
    }
}

/// Las guías rápidas que ofrece el menú «Ayuda», en el orden en que se muestran.
///
/// # Por qué es una tabla y no una lista de bloques `if`
///
/// Eran once bloques idénticos salvo por el par (rótulo, tema). Repetir el cuerpo once veces
/// tiene dos costes que se pagan tarde: añadir una guía era copiar y pegar —con lo fácil que
/// es dejarse el `ui.close()` o repetir el tema de la de al lado—, y cualquier corrección al
/// comportamiento de una entrada había que aplicarla once veces o quedaba a medias, que es el
/// patrón que más disgustos ha dado en este proyecto.
///
/// El orden de la tabla es el orden de la interfaz: se lee de arriba abajo igual que el menú.
const GUIAS_RAPIDAS: [(Texto, TemaDeAyuda); 7] = [
    (Texto::AyudaPrimerosPasos, TemaDeAyuda::PrimerosPasos),
    (
        Texto::AyudaAtajosYCreacion,
        TemaDeAyuda::AtajosYCreacionNodos,
    ),
    (Texto::AyudaVisionDelCreador, TemaDeAyuda::VisionDelCreador),
    (
        Texto::AyudaMapeoDeArchivos,
        TemaDeAyuda::MapeoArchivosCodigo,
    ),
    (
        Texto::AyudaControlHumano,
        TemaDeAyuda::ControlHumanoCorrecciones,
    ),
    (
        Texto::AyudaExportarMarkdown,
        TemaDeAyuda::ExportarMarkdownIA,
    ),
    (Texto::AyudaImportarDesdeIa, TemaDeAyuda::ImportarDesdeIA),
];

/// Las guías de integración con cada modelo de IA, en el orden en que se muestran.
///
/// Van en su propia sección del menú, y por tanto en su propia tabla, porque responden a otra
/// pregunta: las de [`GUIAS_RAPIDAS`] explican cómo se usa MMCelt, y estas cómo se conecta con
/// un agente concreto.
const INTEGRACIONES_CON_IAS: [(Texto, TemaDeAyuda); 4] = [
    (Texto::AyudaServidorMcp, TemaDeAyuda::ClaudeMCPIntegracion),
    (Texto::AyudaClaudeCode, TemaDeAyuda::ClaudeCodeTerminal),
    (Texto::AyudaChatGpt, TemaDeAyuda::ChatGPTCustomGPT),
    (Texto::AyudaGemini, TemaDeAyuda::GeminiGems),
];

/// Pinta una lista de entradas del menú que abren un tema de la ayuda detallada.
///
/// Cada entrada abre su tema en el panel lateral y cierra el menú, que es lo que se esperaba
/// de las once copias que había antes.
///
/// # Parámetros
/// - `app`: estado de la aplicación; ahí se anota qué tema de ayuda queda abierto.
/// - `ui`: el `Ui` del desplegable ya abierto.
/// - `idioma`: el del usuario, para los rótulos.
/// - `entradas`: los pares (rótulo, tema) a pintar, en orden.
fn entradas_de_ayuda(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: Idioma,
    entradas: &[(Texto, TemaDeAyuda)],
) {
    for (rotulo, tema) in entradas {
        if ui.button(rotulo.en(idioma)).clicked() {
            app.abrir_ayuda(*tema);
            ui.close();
        }
    }
}
