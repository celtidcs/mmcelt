//! # Módulo de Ventanas Modales y Diálogos (`ui/ai_modal.rs`)
//!
//! Gestiona las ventanas emergentes del sistema:
//! - Importación tolerante a fallos desde respuestas conversacionales de IA.
//! - Copia del Prompt Maestro para alimentar a ChatGPT / Claude / Gemini.
//! - Previsualización del documento Markdown generado con meta-prompting y Mermaid.
//! - Informe y directivas de corrección humana (*Human-in-the-loop*).
//! - Edición de visión del creador y objetivos del proyecto.
//! - Diálogo de creación de conexiones cruzadas y tabla completa de atajos de teclado.

use crate::ai_bridge::{importar_de_texto_de_ia, prompt_maestro_para_ia};
use crate::ai_export::{exportar_correcciones_del_usuario, exportar_markdown_para_ia};
use crate::aplicacion::AplicacionMapaMental;
use crate::model::TipoRelacion;
use crate::textos::Texto;
use crate::ui::help_system::TemaDeAyuda;
use egui::{RichText, ScrollArea};
use std::time::{Duration, Instant};

/// Tamaño mínimo que mantiene legible una vista previa sin ocupar toda la pantalla.
pub(crate) struct DimensionesVistaPrevia {
    /// Anchura mínima de la ventana, en puntos lógicos de `egui`.
    ancho_minimo: f32,
    /// Altura mínima de la ventana, en puntos lógicos de `egui`.
    alto_minimo: f32,
}

/// La importación necesita espacio para el texto entrante y para explicar sus errores.
const DIMENSIONES_IMPORTACION: DimensionesVistaPrevia = DimensionesVistaPrevia {
    ancho_minimo: 600.0,
    alto_minimo: 450.0,
};

/// El prompt maestro es más ancho porque se revisa como un documento de instrucciones.
const DIMENSIONES_PROMPT_MAESTRO: DimensionesVistaPrevia = DimensionesVistaPrevia {
    ancho_minimo: 650.0,
    alto_minimo: 480.0,
};

/// Qué ha decidido el usuario en el aviso de recuperación.
///
/// Son tres estados y no dos booleanos porque **restaurar y descartar se excluyen**, y con dos
/// banderas sueltas nada impedía que las dos valieran `true` a la vez. El `match` de quien la
/// recibe pasa a ser exhaustivo, y el compilador avisa si algún día se añade una tercera
/// salida.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecisionDeRecuperacion {
    /// El usuario aún no ha decidido: la ventana sigue abierta.
    Pendiente,
    /// Recuperar el trabajo de la sesión anterior.
    Restaurar,
    /// Tirar la copia y quedarse con lo que hay.
    Descartar,
}

/// Dibuja el aviso de recuperación de trabajo sin guardar.
///
/// Aparece al arrancar cuando la sesión anterior terminó de forma inesperada y quedó una
/// copia automática.
///
/// **No restaura por su cuenta.** El usuario pudo haber cerrado a propósito sin guardar,
/// y devolverle sin preguntar justo lo que descartó sería peor que no ofrecer nada. Por
/// eso la ventana no tiene botón de cerrar: exige decidir entre restaurar o descartar,
/// que son las dos únicas salidas sensatas.
///
/// # Parámetros
/// - `app`: estado de la aplicación.
/// - `ctx`: contexto de `egui` donde se dibuja.
pub fn dibujar_modal_recuperacion(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    let Some(recuperacion) = &app.persistencia_mut().recuperacion else {
        return;
    };

    let descripcion = recuperacion.descripcion();
    let total_nodos = recuperacion.proyecto.nodes.len();
    let titulo_mapa = recuperacion.proyecto.title.clone();

    let mut decision = DecisionDeRecuperacion::Pendiente;

    egui::Window::new(Texto::ModalSeEncontroTrabajoSin.en(idioma))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            decision = contenido_del_modal_recuperacion(
                app,
                ui,
                idioma,
                &descripcion,
                &titulo_mapa,
                total_nodos,
            );
        });

    // Las acciones se aplican fuera del cierre para no tener prestado `app` mientras se
    // dibuja: el compilador no permitiría modificarlo dentro.
    match decision {
        DecisionDeRecuperacion::Restaurar => app.aplicar_recuperacion_pendiente(),
        DecisionDeRecuperacion::Descartar => app.eliminar_recuperacion_pendiente(),
        DecisionDeRecuperacion::Pendiente => {}
    }
}

/// Dibuja la ventana «Acerca de» con la identificación de la compilación.
///
/// Responde a una pregunta muy concreta: **¿es este el ejecutable que creo que es?**
///
/// Con varias copias del programa por el disco —la de `target/release`, una portable en
/// una memoria USB, otra en una carpeta de trabajo aparte— todas se llaman igual y todas
/// declaran la misma versión. Lo que las distingue es la fecha de compilación, el commit
/// y la ruta desde la que se están ejecutando, y eso es lo que muestra esta ventana.
///
/// # Parámetros
/// - `app`: estado de la aplicación.
/// - `ctx`: contexto de `egui` donde se dibuja.
pub fn dibujar_modal_acerca_de(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_acerca_de {
        return;
    }

    let mut abierta = app.presentacion_mut().ventanas().modal_acerca_de;
    let mut copiar = false;

    egui::Window::new(Texto::ModalAcercaDeMmcelt.en(idioma))
        .open(&mut abierta)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            copiar = contenido_del_modal_acerca_de(app, ui, idioma);
        });

    if copiar {
        ctx.copy_text(crate::version::detalle_completo());
        app.establecer_estado(Texto::ModalDatosDeLaCompilacion.en(idioma));
    }

    if !abierta {
        app.presentacion_mut().ventanas().modal_acerca_de = false;
    }
}

/// Renderiza condicionalmente todas las ventanas modales activas de la aplicación.
///
/// Cada modal se dibuja solo si su bandera correspondiente está activa, de modo que basta
/// llamar a esta función una vez por fotograma sin comprobar nada antes.
///
/// # Parámetros
/// - `app`: el estado de la aplicación, del que se leen las banderas de cada modal y sobre
///   el que se aplican las acciones que el usuario confirme.
/// - `ctx`: el contexto de `egui` del fotograma en curso.
pub fn dibujar_modales(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    dibujar_modal_acerca_de(app, ctx);
    dibujar_modal_importar_de_ia(app, ctx);
    dibujar_modal_prompt_maestro(app, ctx);
    dibujar_modal_vista_previa_markdown(app, ctx);
    dibujar_modal_correcciones(app, ctx);
    crate::ui::proyecto_ia_modal::dibujar_modal_proyecto_ia(app, ctx);
    dibujar_modal_atajos(app, ctx);
    dibujar_modal_conexion_cruzada(app, ctx);
    dibujar_modal_error_apertura_proyecto_ia(app, ctx);
    dibujar_modal_mapa_en_raiz(app, ctx);
}

/// Dibuja el aviso modal centrado cuando falla la apertura de «Proyecto e instrucciones».
///
/// Informa de forma visible en el centro de la pantalla (no solo en la barra de estado)
/// de por qué falló la carga de la carpeta de proyecto, muestra la ruta concreta afectada
/// y ofrece un botón para ir directamente a «Conectar con mis IAs» o cerrar el aviso.
fn dibujar_modal_error_apertura_proyecto_ia(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let Some(aviso) = app.presentacion().aviso_error_proyecto_ia.clone() else {
        return;
    };

    let idioma = app.idioma();
    let mut abierta = true;

    egui::Window::new(Texto::ModalErrorAperturaProyectoTitulo.en(idioma))
        .open(&mut abierta)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .min_width(520.0)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.label(RichText::new(Texto::ProyectoIaErrorAlAbrir.en(idioma)).strong());
            ui.add_space(8.0);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📁").size(16.0));
                    ui.label(
                        RichText::new(crate::ui::limpiar_ruta_para_interfaz(&aviso.ruta))
                            .monospace()
                            .strong(),
                    );
                });
                let texto_motivo = aviso.motivo.en(idioma);
                if !texto_motivo.is_empty() {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("({})", texto_motivo))
                            .small()
                            .color(app.presentacion().tema().aviso),
                    );
                }
            });

            ui.add_space(10.0);
            ui.label(RichText::new(
                Texto::ProyectoIaErrorAccionRecomendada.en(idioma),
            ));
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new(Texto::BotonIrAConexiones.en(idioma)).strong())
                    .clicked()
                {
                    app.presentacion_mut().ventanas().modal_conexiones = true;
                    app.presentacion_mut().aviso_error_proyecto_ia = None;
                }
                if ui.button(Texto::BotonCerrar.en(idioma)).clicked() {
                    app.presentacion_mut().aviso_error_proyecto_ia = None;
                }
            });
            ui.add_space(4.0);
        });

    if !abierta {
        app.presentacion_mut().aviso_error_proyecto_ia = None;
    }
}

/// Acciones posibles en el diálogo modal de mapa mental en la raíz de una unidad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccionModalMapaEnRaiz {
    Ninguna,
    MoverMapa,
    CrearNuevo,
    Declinar,
}

/// Dibuja el diálogo modal centrado cuando un mapa se abre directamente en la raíz de una unidad o volumen.
///
/// Explica al usuario por qué la raíz de un volumen no puede usarse como carpeta de proyecto
/// y ofrece tres opciones guiadas:
/// 1. Mover el mapa actual a una carpeta de proyecto específica.
/// 2. Crear un nuevo mapa dentro de una carpeta seleccionada.
/// 3. Mantener el mapa en la raíz (sin funciones de proyecto de IA).
pub(crate) fn dibujar_modal_mapa_en_raiz(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let Some(aviso) = app.presentacion().aviso_mapa_en_raiz.clone() else {
        return;
    };

    let idioma = app.idioma();
    let mut abierta = true;
    let mut accion = AccionModalMapaEnRaiz::Ninguna;

    egui::Window::new(Texto::ModalMapaEnRaizTitulo.en(idioma))
        .open(&mut abierta)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .min_width(540.0)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.label(RichText::new(Texto::ModalMapaEnRaizExplicacion.en(idioma)));
            ui.add_space(8.0);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📁").size(16.0));
                    ui.label(
                        RichText::new(crate::ui::limpiar_ruta_para_interfaz(&aviso.ruta_mapa))
                            .monospace()
                            .strong(),
                    );
                });
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(10.0);

            // Opción 1: Mover mapa a carpeta de proyecto
            if ui
                .button(RichText::new(Texto::BotonMoverMapaACarpeta.en(idioma)).strong())
                .clicked()
            {
                accion = AccionModalMapaEnRaiz::MoverMapa;
            }

            ui.add_space(6.0);

            // Opción 2: Crear nuevo mapa en carpeta
            if ui
                .button(Texto::BotonCrearNuevoEnCarpeta.en(idioma))
                .clicked()
            {
                accion = AccionModalMapaEnRaiz::CrearNuevo;
            }

            ui.add_space(6.0);

            // Opción 3: Mantener en raíz sin IA
            if ui
                .button(Texto::BotonMantenerEnRaizSinIa.en(idioma))
                .clicked()
            {
                accion = AccionModalMapaEnRaiz::Declinar;
            }

            ui.add_space(4.0);
        });

    if !abierta {
        accion = AccionModalMapaEnRaiz::Declinar;
    }

    match accion {
        AccionModalMapaEnRaiz::Ninguna => {}
        AccionModalMapaEnRaiz::MoverMapa => {
            crate::traslado::pedir_y_trasladar_mapa_en_raiz(app, &aviso.ruta_mapa);
        }
        AccionModalMapaEnRaiz::CrearNuevo => {
            crate::traslado::pedir_y_crear_nuevo_en_carpeta(app);
        }
        AccionModalMapaEnRaiz::Declinar => {
            app.presentacion_mut().aviso_mapa_en_raiz = None;
        }
    }
}

/// Dibuja la ventana de importación de mapas mentales desde una respuesta de IA.
///
/// Ofrece un área de texto donde pegar la respuesta del modelo y muestra el error de
/// interpretación si la importación no prospera.
fn dibujar_modal_importar_de_ia(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_importar {
        return;
    }

    let mut open = app.presentacion_mut().ventanas().modal_importar;
    egui::Window::new(Texto::ModalImportarMapaMentalDesde.en(idioma))
        .open(&mut open)
        .min_width(DIMENSIONES_IMPORTACION.ancho_minimo)
        .min_height(DIMENSIONES_IMPORTACION.alto_minimo)
        .show(ctx, |ui| {
            contenido_del_modal_importar(app, ui, idioma);
        });
    // Solo se cierra desde aquí si lo cerró la ✖ del marco de la ventana.
    //
    // Antes se asignaba sin condición, y esa línea pisaba lo que los botones de dentro
    // acababan de escribir: `open` solo lo pone a falso `egui` cuando el usuario usa la ✖,
    // así que **todos los botones de cerrar del programa volvían a abrir la ventana** en la
    // misma pasada. Cancelar, Cerrar, Entendido y Guardar Metadatos no hacían nada visible.
    if !open {
        app.presentacion_mut().ventanas().modal_importar = false;
    }
}

/// Dibuja la ventana con el prompt maestro listo para copiar.
///
/// Es el texto que el usuario pega en ChatGPT, Claude o Gemini para que le devuelvan un
/// mapa mental con el formato que MMCelt entiende.
fn dibujar_modal_prompt_maestro(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_prompt_maestro {
        return;
    }

    let mut open = app.presentacion_mut().ventanas().modal_prompt_maestro;
    egui::Window::new(Texto::ModalInstruccionMaestraParaModelos.en(idioma))
        .open(&mut open)
        .min_width(DIMENSIONES_PROMPT_MAESTRO.ancho_minimo)
        .min_height(DIMENSIONES_PROMPT_MAESTRO.alto_minimo)
        .show(ctx, |ui| {
            ui.label(RichText::new(Texto::ModalUsaEstaInstruccionPara.en(idioma)).strong());
            ui.add_space(8.0);

            let master_prompt = prompt_maestro_para_ia();

            ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut master_prompt.to_string())
                        .desired_rows(14)
                        .desired_width(f32::INFINITY)
                        .interactive(false),
                );
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new(Texto::ModalCopiarPromptAlPortapapeles.en(idioma)).strong(),
                    )
                    .clicked()
                {
                    ctx.copy_text(master_prompt.to_string());
                    app.establecer_estado(Texto::ModalPromptMaestroCopiadoAl.en(idioma));
                }

                if ui.button(Texto::BotonCerrar.en(app.idioma())).clicked() {
                    app.presentacion_mut().ventanas().modal_prompt_maestro = false;
                }
            });
        });
    // Igual que arriba: la ✖ cierra, y los botones de dentro ya se han encargado.
    if !open {
        app.presentacion_mut().ventanas().modal_prompt_maestro = false;
    }
}

/// Cada cuánto se comprueba si el mapa ha cambiado, con un documento en pantalla.
///
/// Averiguarlo cuesta un recorrido del árbol entero —2,65 ms con veinte mil nodos— y se
/// hacía **en cada fotograma** mientras la ventana estuviera abierta, que es todo el rato
/// que el usuario dedica a leer el documento: sesenta recorridos por segundo para responder
/// casi siempre que no ha cambiado nada.
///
/// Un cuarto de segundo no lo nota nadie leyendo, y baja el coste a cuatro comprobaciones
/// por segundo en lugar de sesenta.
pub(crate) const INTERVALO_ENTRE_COMPROBACIONES: Duration = Duration::from_millis(250);

/// Documento generado a partir del mapa que un modal está mostrando.
///
/// Lo guarda la aplicación entre fotogramas: componer el documento recorre el árbol y arma
/// el diagrama, y la ventana se queda abierta mientras el usuario lee.
pub struct DocumentoEnPantalla {
    /// Huella del mapa con el que se generó, para saber si sigue valiendo.
    huella: u64,
    /// El documento ya compuesto.
    pub(crate) texto: String,
    /// Cuándo se comprobó por última vez si el mapa había cambiado.
    ultima_comprobacion: Instant,
}

/// Saca del estado el documento del mapa, rehaciéndolo solo si hace falta.
///
/// Se **saca** en lugar de copiarse porque el cuadro de texto que lo muestra exige poder
/// escribir en lo que enseña, aunque sea de solo lectura, y clonar el documento entero dos
/// veces por fotograma era el otro coste de estas ventanas. Quien lo saca tiene que
/// devolverlo al estado antes de terminar.
///
/// # Parámetros
/// - `proyecto`: el mapa del que sale el documento.
/// - `almacen`: donde la aplicación guarda el documento entre fotogramas.
/// - `componer`: la función que genera el documento a partir del mapa. Se toma como `impl Fn`
///   y no como puntero porque desde que el documento va en el idioma del usuario, quien llama
///   pasa un cierre que lleva capturado ese idioma.
/// - `ahora`: el instante en curso. Lo recibe en vez de leerlo del reloj para que el
///   espaciado entre comprobaciones se pueda comprobar sin esperar en una prueba.
///
/// # Devuelve
/// El documento al día, ya fuera del estado.
pub(crate) fn tomar_documento_al_dia(
    proyecto: &crate::model::Proyecto,
    almacen: &mut Option<DocumentoEnPantalla>,
    componer: impl Fn(&crate::model::Proyecto) -> String,
    ahora: Instant,
) -> DocumentoEnPantalla {
    if let Some(guardado) = almacen.take() {
        // Todavía no toca mirar si el mapa ha cambiado: se devuelve tal cual.
        if ahora.duration_since(guardado.ultima_comprobacion) < INTERVALO_ENTRE_COMPROBACIONES {
            return guardado;
        }

        let huella = crate::autoguardado::calcular_huella(proyecto);
        if huella == guardado.huella {
            return DocumentoEnPantalla {
                ultima_comprobacion: ahora,
                ..guardado
            };
        }
    }

    DocumentoEnPantalla {
        huella: crate::autoguardado::calcular_huella(proyecto),
        texto: componer(proyecto),
        ultima_comprobacion: ahora,
    }
}

/// Dibuja la vista previa del documento Markdown que se entregará a la IA.
///
/// Permite revisarlo antes de exportarlo, copiarlo al portapapeles o guardarlo en un
/// archivo.
fn dibujar_modal_vista_previa_markdown(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_vista_previa {
        return;
    }

    let mut open = app.presentacion_mut().ventanas().modal_vista_previa;

    // El documento se rehace solo si el mapa ha cambiado, y hasta eso se comprueba unas
    // pocas veces por segundo en vez de en cada fotograma. Sale del estado sin copiarse, y
    // se devuelve al final de la función.
    let mut documento = {
        let (mapa, presentacion) = app.mapa_y_presentacion_mut();
        tomar_documento_al_dia(
            mapa.proyecto(),
            presentacion.vista_previa_para_ia(),
            |proyecto| exportar_markdown_para_ia(proyecto, idioma),
            Instant::now(),
        )
    };

    egui::Window::new(Texto::ModalVistaPreviaDelArchivo.en(idioma))
        .open(&mut open)
        .min_width(700.0)
        .min_height(500.0)
        .show(ctx, |ui| {
            contenido_de_la_vista_previa(app, ui, ctx, &mut documento, idioma);
        });
    // Igual que arriba: la ✖ cierra, y los botones de dentro ya se han encargado.
    if !open {
        app.presentacion_mut().ventanas().modal_vista_previa = false;
    }

    // El documento vuelve al estado: si no, se recompondría entero en cada fotograma, que
    // es justo lo que este mecanismo existe para evitar.
    *app.presentacion_mut().vista_previa_para_ia() = Some(documento);
}

/// Dibuja la ventana de directivas de corrección para la IA.
///
/// Reúne los nodos que el usuario ha marcado como incorrectos o descartados y genera el
/// prompt que reconduce el trabajo del modelo.
fn dibujar_modal_correcciones(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_correcciones {
        return;
    }

    let mut open = app.presentacion_mut().ventanas().modal_correcciones;

    // Igual que la vista previa, y por el mismo mecanismo.
    let mut documento = {
        let (mapa, presentacion) = app.mapa_y_presentacion_mut();
        tomar_documento_al_dia(
            mapa.proyecto(),
            presentacion.vista_previa_de_correcciones(),
            |proyecto| exportar_correcciones_del_usuario(proyecto, idioma),
            Instant::now(),
        )
    };

    egui::Window::new(Texto::ModalDirectivasDeCorreccionY.en(idioma))
        .open(&mut open)
        .min_width(700.0)
        .min_height(500.0)
        .show(ctx, |ui| {
            contenido_del_modal_correcciones(app, ui, ctx, &mut documento, idioma);
        });
    // Igual que arriba: la ✖ cierra, y los botones de dentro ya se han encargado.
    if !open {
        app.presentacion_mut().ventanas().modal_correcciones = false;
    }

    // Y el documento vuelve al estado, por el mismo motivo que en la vista previa.
    *app.presentacion_mut().vista_previa_de_correcciones() = Some(documento);
}

/// Dibuja la ventana con la tabla completa de atajos de teclado.
fn dibujar_modal_atajos(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_atajos {
        return;
    }

    let mut open = app.presentacion_mut().ventanas().modal_atajos;
    egui::Window::new(Texto::ModalAtajosDeTecladoY.en(idioma))
        .open(&mut open)
        .min_width(500.0)
        .show(ctx, |ui| {
            ui.heading(Texto::ModalAtajosDeCreacionY.en(idioma));
            ui.add_space(6.0);

            egui::Grid::new("shortcuts_grid")
                .striped(true)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    for (tecla, que_hace) in &ATAJOS_DE_TECLADO {
                        ui.label(RichText::new(tecla.rotulo(idioma)).strong());
                        ui.label(que_hace.en(idioma));
                        ui.end_row();
                    }
                });

            ui.add_space(10.0);
            ui.label(
                RichText::new(Texto::ModalVesLaAplicacionPequena.en(idioma))
                    .small()
                    .italics()
                    .color(app.presentacion().tema().texto_secundario),
            );

            ui.add_space(14.0);
            ui.heading(Texto::ModalInteroperabilidadConIa.en(idioma));
            ui.label(Texto::ModalUsaElBotonExportar.en(idioma));
            ui.label(Texto::ModalUsaPromptParaIa.en(idioma));
            ui.label(Texto::ModalUsaImportarDesdeIa.en(idioma));

            ui.add_space(10.0);
            if ui.button(Texto::BotonEntendido.en(app.idioma())).clicked() {
                app.presentacion_mut().ventanas().modal_atajos = false;
            }
        });
    // Igual que arriba: la ✖ cierra, y los botones de dentro ya se han encargado.
    if !open {
        app.presentacion_mut().ventanas().modal_atajos = false;
    }
}

/// Dibuja la ventana de creación de conexiones cruzadas entre dos nodos.
///
/// Permite elegir el nodo de destino, el tipo de relación y la etiqueta que explica el
/// motivo del vínculo.
fn dibujar_modal_conexion_cruzada(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_conexion_cruzada {
        return;
    }

    let mut open = app.presentacion_mut().ventanas().modal_conexion_cruzada;
    let Some(id_del_origen) = app.lienzo_mut().editar_conexion().origen else {
        // Sin origen no hay nada que crear. Ocurre si la ventana se abre desde un sitio que
        // no lo anotó antes; el menú de edición sí lo hace.
        app.presentacion_mut().ventanas().modal_conexion_cruzada = false;
        return;
    };

    let titulo_del_origen = app
        .mapa_mut()
        .proyecto()
        .nodes
        .get(&id_del_origen)
        .map(|n| n.title.clone())
        .unwrap_or_default();

    egui::Window::new(Texto::ModalCrearRelacionCruzadaTransversal.en(idioma))
        .open(&mut open)
        .min_width(450.0)
        .show(ctx, |ui| {
            ui.label(
                RichText::new(Texto::ModalDesdeElNodo.en(idioma).replacen(
                    "{}",
                    &titulo_del_origen,
                    1,
                ))
                .strong(),
            );
            ui.add_space(6.0);

            selector_del_nodo_de_destino(app, ui, idioma, id_del_origen);

            ui.add_space(6.0);
            selector_del_tipo_de_relacion(app, ui, idioma);

            ui.add_space(6.0);
            ui.label(Texto::ModalMotivoExplicacionDelVinculo.en(idioma));
            ui.text_edit_singleline(&mut app.lienzo_mut().editar_conexion().motivo);

            ui.add_space(10.0);
            botones_de_la_conexion_cruzada(app, ui, idioma, id_del_origen);
        });
    // Igual que arriba: la ✖ cierra, y los botones de dentro ya se han encargado. Cerrar por
    // ahí también olvida lo que hubiera a medias, para que el destino y el motivo de una
    // relación abandonada no reaparezcan pre-rellenados en la siguiente.
    if !open {
        olvidar_la_conexion_en_curso(app);
        app.presentacion_mut().ventanas().modal_conexion_cruzada = false;
    }
}

/// El desplegable con el que se elige a qué nodo apunta la conexión cruzada.
///
/// Los destinos se ordenan **por título**, y no en el orden en que están guardados. Los nodos
/// viven en un mapa ordenado por identificador, y los identificadores se generan al azar: la
/// lista salía sin ningún orden reconocible —ni de creación, ni alfabético, ni por ramas— y
/// con la raíz en medio. Sobre un mapa del escáner de repositorios, con cientos de nodos,
/// encontrar el destino era inviable. Es orden de presentación: no toca ningún contrato.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el destino elegido se anota en la conexión en curso.
/// - `ui`: el `Ui` de la ventana.
/// - `idioma`: el del usuario, para los rótulos.
/// - `id_del_origen`: el nodo del que sale la conexión, que se excluye de la lista para que
///   nadie pueda enlazarlo consigo mismo.
fn selector_del_nodo_de_destino(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
    id_del_origen: uuid::Uuid,
) {
    ui.label(Texto::ModalSeleccionarNodoDestino.en(idioma));
    egui::ComboBox::from_id_salt("cross_link_target_combo")
        .selected_text(
            app.lienzo_mut()
                .conexion()
                .destino
                .and_then(|id| app.mapa_mut().proyecto_para_editar().nodes.get(&id))
                .map(|n| n.title.as_str())
                .unwrap_or(Texto::ModalEligeNodoDeDestino.en(idioma)),
        )
        .show_ui(ui, |ui| {
            let mut destinos: Vec<(uuid::Uuid, String)> = app
                .mapa()
                .proyecto()
                .nodes
                .iter()
                .filter(|(&id, _)| id != id_del_origen)
                .map(|(&id, nodo)| (id, nodo.title.clone()))
                .collect();
            destinos.sort_by(|(_, uno), (_, otro)| {
                uno.to_lowercase()
                    .cmp(&otro.to_lowercase())
                    .then_with(|| uno.cmp(otro))
            });

            for (id, titulo) in destinos {
                ui.selectable_value(
                    &mut app.lienzo_mut().editar_conexion().destino,
                    Some(id),
                    titulo,
                );
            }
        });
}

/// El desplegable con el que se elige qué clase de relación es.
///
/// Cada tipo lleva su símbolo delante, que es el mismo que se dibuja después junto a la curva
/// en el lienzo y el que aparece en la exportación para la IA.
///
/// # Parámetros
/// - `app`: estado de la aplicación; el tipo elegido se anota en la conexión en curso.
/// - `ui`: el `Ui` de la ventana.
/// - `idioma`: el del usuario, para los nombres de los tipos.
fn selector_del_tipo_de_relacion(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
) {
    ui.label(Texto::ModalTipoDeRelacion.en(idioma));
    app.dibujar_galleta_de_ayuda(ui, TemaDeAyuda::ConexionesCruzadas);
    egui::ComboBox::from_id_salt("cross_link_type_combo")
        .selected_text(format!(
            "{} {}",
            app.lienzo_mut().editar_conexion().tipo.simbolo(),
            app.lienzo_mut()
                .editar_conexion()
                .tipo
                .nombre_para_interfaz(idioma)
        ))
        .show_ui(ui, |ui| {
            for tipo in TipoRelacion::TODOS {
                ui.selectable_value(
                    &mut app.lienzo_mut().editar_conexion().tipo,
                    tipo,
                    format!("{} {}", tipo.simbolo(), tipo.nombre_para_interfaz(idioma)),
                );
            }
        });
}

/// Dibuja el selector real del tipo de relación dentro del arnés de interfaz.
#[cfg(test)]
pub(crate) fn dibujar_selector_tipo_relacion_para_prueba(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
) {
    selector_del_tipo_de_relacion(app, ui, app.idioma());
}

/// Los botones de crear y cancelar de la ventana de conexión cruzada.
///
/// # Qué se comprueba antes de dar el parte de éxito
///
/// Dos cosas que antes no se miraban, y las dos hacían que el programa dijera que había hecho
/// algo sin haberlo hecho:
///
/// - **Que haya un destino elegido.** Es el estado normal al abrir la ventana, y pulsar
///   «Crear» sin elegir no producía ni relación, ni aviso, ni cierre.
/// - **Que la relación llegue a crearse.** `anadir_conexion_cruzada` devuelve `None` cuando el
///   origen y el destino son el mismo nodo, o cuando alguno ya no existe. Ese valor se
///   descartaba y se escribía «Relación cruzada añadida» igualmente. Bastaba con dejar elegido
///   un destino, cancelar, borrar ese nodo y volver a entrar.
///
/// # Parámetros
/// - `app`: estado de la aplicación; aquí se añade la relación al mapa y se cierra la ventana.
/// - `ui`: el `Ui` de la ventana.
/// - `idioma`: el del usuario, para los rótulos y los avisos.
/// - `id_del_origen`: el nodo del que sale la conexión.
fn botones_de_la_conexion_cruzada(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
    id_del_origen: uuid::Uuid,
) {
    ui.horizontal(|ui| {
        if ui
            .button(RichText::new(Texto::ModalCrearRelacion.en(idioma)).strong())
            .clicked()
        {
            let Some(id_del_destino) = app.lienzo_mut().editar_conexion().destino else {
                app.establecer_estado(Texto::AvisoEligeUnDestino.en(app.idioma()));
                return;
            };

            let motivo = app.lienzo().conexion().motivo.clone();
            let tipo = app.lienzo().conexion().tipo;
            let creada = app
                .mapa_mut()
                .proyecto_para_editar()
                .anadir_conexion_cruzada(id_del_origen, id_del_destino, motivo, tipo)
                .is_some();

            if creada {
                let nombre_de_la_relacion = tipo.nombre_para_interfaz(idioma);
                let aviso = Texto::ModalNombredelarelacion
                    .en(idioma)
                    .replacen("{}", Texto::AvisoRelacionAnadida.en(idioma), 1)
                    .replace("{nombre_de_la_relacion}", nombre_de_la_relacion);
                olvidar_la_conexion_en_curso(app);
                app.presentacion_mut().ventanas().modal_conexion_cruzada = false;
                app.establecer_estado(aviso);
            } else {
                app.establecer_estado(Texto::AvisoRelacionNoCreada.en(app.idioma()));
                app.lienzo_mut().editar_conexion().destino = None;
            }
        }

        if ui.button(Texto::BotonCancelar.en(app.idioma())).clicked() {
            olvidar_la_conexion_en_curso(app);
            app.presentacion_mut().ventanas().modal_conexion_cruzada = false;
        }
    });
}

/// Olvida la conexión cruzada que se estaba componiendo.
///
/// Se llama al terminarla y también al abandonarla. Antes, cancelar solo soltaba el nodo de
/// origen: el destino elegido, el motivo escrito y el tipo seleccionado se quedaban puestos, y
/// aparecían ya rellenos la próxima vez que se abría la ventana. La explicación escrita para
/// una relación que no llegó a existir acababa adjuntada, en silencio, a otra distinta.
///
/// # Parámetros
/// - `app`: estado de la aplicación, del que se limpian los cuatro campos.
fn olvidar_la_conexion_en_curso(app: &mut AplicacionMapaMental) {
    app.lienzo_mut().editar_conexion().origen = None;
    app.lienzo_mut().editar_conexion().destino = None;
    app.lienzo_mut().editar_conexion().motivo.clear();
    app.lienzo_mut().editar_conexion().tipo = TipoRelacion::default();
}

/// Cómo se nombra una tecla en la tabla de atajos.
///
/// Hacen falta tres formas porque no todas las teclas se dicen igual en los seis idiomas.
/// `Ctrl + S` es `Ctrl + S` en todas partes; «Supr» y «Espacio» no.
enum TeclaDelAtajo {
    /// El nombre es el mismo en cualquier idioma: `Tab`, `Enter`, `Ctrl + S`.
    Literal(&'static str),
    /// El nombre se traduce, porque cambia de un idioma a otro.
    Traducible(Texto),
    /// Un nombre traducible seguido de una alternativa que no se traduce, como
    /// «Supr / Backspace».
    TraducibleMas(Texto, &'static str),
}

impl TeclaDelAtajo {
    /// Compone el rótulo de la tecla en el idioma del usuario.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    ///
    /// # Devuelve
    /// El texto listo para pintar en la primera columna de la tabla.
    fn rotulo(&self, idioma: crate::textos::Idioma) -> String {
        match self {
            TeclaDelAtajo::Literal(texto) => (*texto).to_string(),
            TeclaDelAtajo::Traducible(texto) => texto.en(idioma).to_string(),
            TeclaDelAtajo::TraducibleMas(texto, sufijo) => {
                format!("{} / {sufijo}", texto.en(idioma))
            }
        }
    }
}

/// Los atajos de teclado que se muestran en la ventana «Atajos», en el orden en que aparecen.
///
/// # Por qué es una tabla
///
/// Eran catorce bloques de tres líneas idénticos salvo por el par (tecla, descripción), con
/// la fila del zoom repetida a mano en medio. Añadir un atajo era copiar tres líneas y
/// acordarse del `ui.end_row()`; olvidarlo descuadra la rejilla entera y no lo dice nadie.
///
/// El orden es el de la interfaz: primero lo que se usa construyendo el mapa, después el
/// ratón, y al final los `Ctrl` de archivo, deshacer y tamaño.
const ATAJOS_DE_TECLADO: [(TeclaDelAtajo, Texto); 14] = [
    (TeclaDelAtajo::Literal("Tab"), Texto::ModalAnadirNodoHijoAl),
    (
        TeclaDelAtajo::Literal("Enter"),
        Texto::ModalAnadirNodoHermanoMismo,
    ),
    (
        TeclaDelAtajo::TraducibleMas(Texto::TeclaSuprimir, "Backspace"),
        Texto::ModalEliminarElNodoSeleccionado,
    ),
    (
        TeclaDelAtajo::TraducibleMas(Texto::TeclaEspacio, "F2"),
        Texto::ModalEditarTextoDelNodo,
    ),
    (
        TeclaDelAtajo::Traducible(Texto::ModalArrastrarConClicCentral),
        Texto::ModalMoversePorElLienzo,
    ),
    (
        TeclaDelAtajo::Traducible(Texto::ModalRuedaDelRaton),
        Texto::ModalZoomInZoomOut,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + N"),
        Texto::ModalEmpezarUnMapaNuevo,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + S"),
        Texto::ModalGuardarMapaMentalMmcelt,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + E"),
        Texto::ModalExportarArchivoMarkdownMd,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + F"),
        Texto::ModalCentrarLaVistaEn,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + Z"),
        Texto::ModalDeshacerElUltimoCambio,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + Y"),
        Texto::ModalRehacerLoDeshecho,
    ),
    // El zoom del lienzo (rueda del ratón) y el tamaño de la interfaz son cosas distintas, y
    // confundirlos es fácil: el primero acerca el mapa, el segundo agranda letras, botones y
    // paneles.
    (
        TeclaDelAtajo::Literal("Ctrl + / Ctrl -"),
        Texto::ModalAgrandarOReducirToda,
    ),
    (
        TeclaDelAtajo::Literal("Ctrl + 0"),
        Texto::ModalDevolverLaInterfazA,
    ),
];

/// El contenido de la ventana «Acerca de»: qué compilación es esta.
///
/// Identifica la versión y la fecha de compilación, que es lo que se le pide a un usuario
/// cuando informa de un fallo, y recoge el compromiso de licencia.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde salen el tema y los datos de la compilación.
/// - `ui`: el `Ui` de la ventana ya abierta.
/// - `idioma`: el del usuario, para los rótulos.
fn contenido_del_modal_acerca_de(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
) -> bool {
    // Se devuelve en vez de copiar aquí mismo porque el portapapeles se pide al
    // `Context`, y dentro del closure de la ventana no se tiene: `egui` lo presta como
    // `Ui`. El llamador, que sí lo tiene, hace la copia.
    let mut copiar = false;
    ui.add_space(6.0);

    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(Texto::ModalMmcelt.en(idioma))
                .size(22.0)
                .strong()
                .color(app.presentacion().tema().colores_de_las_ramas[0]),
        );
        ui.label(
            RichText::new(Texto::ModalMapasMentalesParaDirigir.en(idioma))
                .color(app.presentacion().tema().texto_secundario),
        );
    });

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    tabla_de_datos_de_la_compilacion(app, ui, idioma);
    ruta_del_ejecutable_en_uso(ui, idioma);

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui
            .button(Texto::ModalCopiarEstosDatos.en(idioma))
            .on_hover_text(Texto::ModalUtilParaIncluirlosAl.en(idioma))
            .clicked()
        {
            copiar = true;
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(Texto::BotonCerrar.en(app.idioma())).clicked() {
                app.presentacion_mut().ventanas().modal_acerca_de = false;
            }
        });
    });

    ui.add_space(4.0);
    ui.label(
        RichText::new(Texto::ModalTambienPuedesComprobarloSin.en(idioma))
            .small()
            .italics()
            .color(app.presentacion().tema().texto_atenuado),
    );

    copiar
}

/// El contenido de la ventana «Importar desde IA»: el cuadro donde se pega el mapa.
///
/// # Parámetros
/// - `app`: estado de la aplicación; aquí acaba el mapa importado y el error si lo hay.
/// - `ui`: el `Ui` de la ventana ya abierta.
/// - `idioma`: el del usuario, para los rótulos.
fn contenido_del_modal_importar(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
) {
    ui.label(RichText::new(Texto::ModalPegaAContinuacionEl.en(idioma)).strong());
    ui.label(
        RichText::new(Texto::ModalMmceltInterpretaraAutomaticamenteLa.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    ui.add_space(6.0);

    // El aviso va antes del cuadro de texto, no junto al botón: quien llega hasta
    // aquí ya trae la respuesta de la IA copiada y va a pegarla sin releer nada.
    ui.label(
        RichText::new(Texto::ModalEstoSustituyeElMapa.en(idioma))
            .small()
            .color(app.presentacion().tema().aviso),
    );
    ui.add_space(8.0);

    ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
        ui.add(
            egui::TextEdit::multiline(app.presentacion_mut().texto_para_importar())
                .desired_rows(12)
                .desired_width(f32::INFINITY)
                .hint_text(Texto::ModalPegaAquiElJson.en(idioma)),
        );
    });

    if let Some(err) = &app.presentacion_mut().error_de_importacion() {
        ui.add_space(4.0);
        ui.label(RichText::new(format!("❌ {}", err)).color(app.presentacion().tema().peligro));
    }

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if ui
            .button(
                RichText::new(Texto::ModalSustituirElMapaPor.en(idioma))
                    .strong()
                    .color(app.presentacion().tema().exito),
            )
            .clicked()
        {
            match importar_de_texto_de_ia(app.presentacion_mut().texto_para_importar()) {
                Ok(proyecto_importado) => {
                    // El mapa abierto se pierde al importar, así que pasa por el
                    // camino común, que deja copia de recuperación del anterior. Este
                    // fue el primer sitio donde se resolvió —un mapa con horas de
                    // trabajo desaparecía sin más al pegar la respuesta de un
                    // modelo—, y durante un tiempo fue el único.
                    let aviso = Texto::AvisoMapaImportado.en(app.idioma());
                    app.sustituir_el_mapa_abierto(proyecto_importado, None, aviso);
                    app.presentacion_mut().texto_para_importar().clear();
                    *app.presentacion_mut().error_de_importacion() = None;
                    app.presentacion_mut().ventanas().modal_importar = false;
                }
                Err(e) => {
                    // La capa de interfaz es el único punto donde un error se
                    // convierte en texto para el usuario; antes se registra
                    // con su contexto para poder depurarlo después.
                    crate::error::registrar(&e, Texto::ModalImportacionDesdeIa.en(idioma));
                    *app.presentacion_mut().error_de_importacion() =
                        Some(e.mensaje_usuario().en(idioma));
                }
            }
        }

        // Aquí había un botón «Pegar del Portapapeles» que no funcionaba nunca:
        // buscaba un evento de pegado entre los del fotograma, y ese evento solo
        // existe en el fotograma en que el usuario pulsa Ctrl+V, no cuando pulsa un
        // botón. El usuario copiaba la respuesta del modelo, pulsaba el botón que se
        // llamaba exactamente como lo que quería hacer, y el cuadro seguía vacío sin
        // ningún aviso.
        //
        // `egui` no permite leer el portapapeles —solo escribir en él—, así que no
        // hay forma de que ese botón funcione. En su lugar, se dice dónde pegar.
        ui.label(
            RichText::new(Texto::ModalPegaAquiConCtrl.en(idioma))
                .small()
                .color(app.presentacion().tema().texto_secundario),
        );

        if ui.button(Texto::BotonCancelar.en(app.idioma())).clicked() {
            app.presentacion_mut().ventanas().modal_importar = false;
        }
    });
}

/// El contenido de la ventana de correcciones que se le exigen a la IA.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde sale el documento de correcciones.
/// - `ui`: el `Ui` de la ventana ya abierta.
/// - `idioma`: el del usuario, para los rótulos.
fn contenido_del_modal_correcciones(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    documento: &mut DocumentoEnPantalla,
    idioma: crate::textos::Idioma,
) {
    ui.label(RichText::new(Texto::ModalUsaEsteDocumentoPara.en(idioma)).strong());
    ui.label(
        RichText::new(Texto::ModalResaltaAutomaticamenteLosNodos.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
    ui.add_space(8.0);

    ScrollArea::vertical().max_height(340.0).show(ui, |ui| {
        ui.add(
            // Igual que la vista previa: se muestra para copiarlo, y lo que se
            // copia se regenera del mapa.
            egui::TextEdit::multiline(&mut documento.texto)
                .desired_rows(16)
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .interactive(false),
        );
    });

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if ui
            .button(
                RichText::new(Texto::ModalCopiarDirectivasDeCorreccion.en(idioma))
                    .strong()
                    .color(app.presentacion().tema().peligro),
            )
            .clicked()
        {
            ctx.copy_text(documento.texto.clone());
            app.establecer_estado(Texto::ModalDirectivasDeCorreccionCopiadas.en(idioma));
        }

        if ui
            .button(Texto::ModalGuardarComoArchivo.en(idioma))
            .clicked()
        {
            if let Some(path) = crate::ui::dialogos::pedir_archivo_para_exportar(
                &format!(
                    "{}_Correcciones_IA.md",
                    app.mapa_mut().proyecto_para_editar().title
                ),
                idioma,
            ) {
                // El resultado de la escritura se descartaba y, aun así, se
                // anunciaba «Guardado en …». Si el disco estaba lleno o la carpeta
                // era de solo lectura, el usuario se quedaba convencido de que sus
                // correcciones estaban a salvo y no había nada.
                // Por la puerta de la capa de persistencia, y no con `fs::write`:
                // era la única escritura del programa que truncaba el destino antes
                // de escribir, así que un corte a media faena dejaba las correcciones
                // del usuario en un archivo vacío. Se libraba de la regla porque la
                // prueba que la vigila no miraba este archivo; ahora sí lo mira.
                match app.guardar_documento_auxiliar(&path, documento.texto.as_bytes()) {
                    Ok(()) => app.establecer_estado(Texto::ModalGuardadoEn.en(idioma).replacen(
                        "{}",
                        &path.display().to_string(),
                        1,
                    )),
                    Err(e) => app.reportar_error(&e, Texto::ModalGuardarLasDirectivasDe.en(idioma)),
                }
            }
        }

        if ui.button(Texto::BotonCerrar.en(app.idioma())).clicked() {
            app.presentacion_mut().ventanas().modal_correcciones = false;
        }
    });
}

/// El contenido de la vista previa del Markdown que se le entrega a la IA.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde sale el documento ya compuesto.
/// - `ui`: el `Ui` de la ventana ya abierta.
/// - `idioma`: el del usuario, para los rótulos.
fn contenido_de_la_vista_previa(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    documento: &mut DocumentoEnPantalla,
    idioma: crate::textos::Idioma,
) {
    ui.label(RichText::new(Texto::ModalEsteEsElDocumento.en(idioma)).strong());
    ui.add_space(8.0);

    ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
        ui.add(
            // Vista previa, no editor: lo que se exporta se regenera del mapa, así
            // que cualquier retoque hecho aquí se perdería sin avisar.
            egui::TextEdit::multiline(&mut documento.texto)
                .desired_rows(16)
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .interactive(false),
        );
    });

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if ui
            .button(Texto::ModalCopiarTodoElMarkdown.en(idioma))
            .clicked()
        {
            ctx.copy_text(documento.texto.clone());
            app.establecer_estado(Texto::ModalMarkdownCopiadoAlPortapapeles.en(idioma));
        }

        if ui
            .button(
                RichText::new(Texto::ModalGuardarComoArchivoMd.en(idioma))
                    .strong()
                    .color(app.presentacion().tema().exito),
            )
            .clicked()
        {
            if let Some(path) = crate::ui::dialogos::pedir_archivo_para_exportar(
                &format!("{}_AI.md", app.mapa_mut().proyecto_para_editar().title),
                idioma,
            ) {
                match app.exportar_mapa_para_agente(&path, idioma) {
                    Ok(()) => {
                        app.establecer_estado(Texto::ModalGuardadoEn.en(idioma).replacen(
                            "{}",
                            &path.display().to_string(),
                            1,
                        ));
                    }
                    Err(e) => app.reportar_error(&e, Texto::ModalExportarElMarkdownPara.en(idioma)),
                }
            }
        }

        if ui.button(Texto::BotonCerrar.en(app.idioma())).clicked() {
            app.presentacion_mut().ventanas().modal_vista_previa = false;
        }
    });
}

/// El contenido del aviso de recuperación: qué copia se ha encontrado y qué hacer con ella.
///
/// # Parámetros
/// - `app`: estado de la aplicación, de donde sale la copia pendiente.
/// - `ui`: el `Ui` de la ventana ya abierta.
/// - `idioma`: el del usuario, para los rótulos.
fn contenido_del_modal_recuperacion(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
    descripcion: &str,
    titulo_mapa: &str,
    total_nodos: usize,
) -> DecisionDeRecuperacion {
    let mut decision = DecisionDeRecuperacion::Pendiente;
    ui.add_space(4.0);
    ui.label(
        RichText::new(Texto::ModalLaSesionAnteriorTermino.en(idioma))
            .strong()
            .size(15.0),
    );
    ui.add_space(8.0);

    ui.label(
        Texto::ModalHayUnaCopiaAutomatica
            .en(idioma)
            .replace("{descripcion}", descripcion),
    );
    ui.label(
        RichText::new(
            Texto::ModalContieneTitulomapaConTotalnodos
                .en(idioma)
                .replace("{titulo_mapa}", titulo_mapa)
                .replace("{total_nodos}", &total_nodos.to_string()),
        )
        .color(app.presentacion().tema().texto_secundario),
    );

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui
            .button(
                RichText::new(Texto::ModalRecuperarEseTrabajo.en(idioma))
                    .strong()
                    .color(app.presentacion().tema().exito),
            )
            .on_hover_text(Texto::ModalCargaLaCopiaAutomatica.en(idioma))
            .clicked()
        {
            decision = DecisionDeRecuperacion::Restaurar;
        }

        if ui
            .button(Texto::ModalDescartarlaYEmpezarDe.en(idioma))
            .on_hover_text(Texto::ModalBorraLaCopiaAutomatica.en(idioma))
            .clicked()
        {
            decision = DecisionDeRecuperacion::Descartar;
        }
    });

    ui.add_space(4.0);
    ui.label(
        RichText::new(Texto::ModalLaCopiaSeGuarda.en(idioma))
            .small()
            .italics()
            .color(app.presentacion().tema().texto_atenuado),
    );

    decision
}

/// La tabla de la ventana «Acerca de»: versión, fecha, commit y rama.
///
/// Son los cuatro datos que se le piden a un usuario cuando informa de un fallo, y por eso van
/// en monoespaciada: se copian tal cual.
///
/// Debajo, y solo si procede, el aviso de que el binario se compiló con cambios sin guardar.
/// Un ejecutable así no corresponde a ningún commit concreto, y conviene que se vea aquí antes
/// de que alguien intente reproducir un fallo sobre el commit que dice la tabla.
///
/// # Parámetros
/// - `app`: estado de la aplicación, del que sale el color del aviso.
/// - `ui`: el `Ui` de la ventana.
/// - `idioma`: el del usuario, para los rótulos.
fn tabla_de_datos_de_la_compilacion(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
) {
    egui::Grid::new("datos_de_compilacion")
        .num_columns(2)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            ui.label(RichText::new(Texto::ModalVersion.en(idioma)).strong());
            ui.label(RichText::new(crate::version::VERSION).monospace());
            ui.end_row();

            ui.label(RichText::new(Texto::ModalCompilado.en(idioma)).strong());
            ui.label(RichText::new(crate::version::FECHA_COMPILACION).monospace());
            ui.end_row();

            ui.label(RichText::new(Texto::ModalCommit.en(idioma)).strong());
            ui.label(RichText::new(crate::version::COMMIT).monospace());
            ui.end_row();

            ui.label(RichText::new(Texto::ModalRama.en(idioma)).strong());
            ui.label(RichText::new(crate::version::RAMA).monospace());
            ui.end_row();
        });

    if crate::version::es_arbol_sucio() {
        ui.add_space(8.0);
        ui.label(
            RichText::new(Texto::ModalSeCompiloConCambios.en(idioma))
                .small()
                .color(app.presentacion().tema().aviso),
        );
    }
}

/// Dice qué ejecutable es exactamente el que está abierto.
///
/// Es lo que resuelve la duda cuando hay varias copias por el disco —la de `target`, una
/// portable, otra carpeta de trabajo—: la versión y el commit pueden coincidir, la ruta no.
///
/// El cuadro es de **solo lectura**: se muestra para poder copiarlo, no para editarlo. Sin
/// decirlo, aceptaba escritura y la tiraba al acabar el fotograma —el búfer es una copia
/// temporal—, así que el usuario veía aparecer y desaparecer cada carácter que tecleaba.
///
/// # Parámetros
/// - `ui`: el `Ui` de la ventana.
/// - `idioma`: el del usuario, para el rótulo y para el texto de cuando no se puede saber.
fn ruta_del_ejecutable_en_uso(ui: &mut egui::Ui, idioma: crate::textos::Idioma) {
    ui.add_space(10.0);
    ui.label(RichText::new(Texto::ModalEjecutableEnUso.en(idioma)).strong());

    let ruta = std::env::current_exe()
        .map(|r| r.display().to_string())
        .unwrap_or_else(|_| Texto::ModalNoSePudoDeterminar.en(idioma).to_string());

    ui.add(
        egui::TextEdit::singleline(&mut ruta.clone())
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Monospace)
            .interactive(false),
    );
}
