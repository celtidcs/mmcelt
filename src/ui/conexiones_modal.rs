//! # Ventana de Conexión con Agentes de IA (`ui/conexiones_modal.rs`)
//!
//! Presenta los agentes de IA instalados en el equipo y permite conectarlos con MMCelt
//! pulsando un botón, sin editar archivos ni abrir ninguna terminal.
//!
//! ## Cómo está pensada
//!
//! El destinatario de esta ventana no es necesariamente quien programó nada: puede ser
//! alguien que ha recibido MMCelt de un conocido y quiere usarlo con el agente que ya
//! tiene instalado. De ahí tres decisiones:
//!
//! 1. **La detección se rehace al abrir la ventana.** Si el usuario instala Codex la
//!    semana que viene, aparecerá sin tener que reiniciar ni recordar ningún paso.
//! 2. **Cada agente lleva una descripción de qué es.** Quien no sepa qué es Windsurf
//!    tampoco tiene por qué saberlo para usar esto.
//! 3. **Se avisa siempre de que hay que reiniciar el agente**, porque ninguno relee su
//!    configuración en caliente y, sin ese aviso, la conexión parece no haber funcionado.
//! 4. **La carpeta que se propone es la del trabajo en curso**, no una fija. Mientras el
//!    usuario no elija una, se propone la del mapa que tiene abierto: autorizar
//!    `Documentos/MapasMentales` cuando el proyecto está en otro sitio le da al agente
//!    permiso para trabajar donde no hay nada.
//!
//! ## Lo que la ventana dice tiene que ser lo que el agente obedece
//!
//! La carpeta de trabajo no vive en MMCelt: viaja **dentro** del archivo de configuración de
//! cada agente. Cambiarla aquí no la cambia allí, así que un agente registrado antes seguía
//! luciendo «Conectado» mientras autorizaba la carpeta anterior. Ahora esa discrepancia se
//! ve, se explica y se repara con un botón; lo que no se hace es reescribir sola la
//! configuración de otro programa.

use crate::aplicacion::AplicacionMapaMental;
use crate::conectores::{self, Agente, EstadoDeLaConexion};
use crate::textos::Texto;
use egui::RichText;

/// Lo que el usuario pide en la ventana de conexión y todavía está sin aplicar.
///
/// Se recoge mientras se dibuja y se aplica al cerrar el marco: no se puede modificar `app`
/// mientras se le está tomando prestado para pintar.
#[derive(Default)]
struct AccionesDelModalDeConexiones {
    /// Agente que hay que conectar, por su posición en la lista.
    conectar: Option<usize>,
    /// Agente que hay que desconectar, por su posición en la lista.
    desconectar: Option<usize>,
    /// Hay que volver a recorrer el disco buscando agentes.
    volver_a_buscar: bool,
}

/// Dibuja la ventana de conexión con agentes de IA.
///
/// # Parámetros
/// - `app`: estado de la aplicación.
/// - `ctx`: contexto de `egui` donde se dibuja.
pub fn dibujar_modal_conexiones(app: &mut AplicacionMapaMental, ctx: &egui::Context) {
    let idioma = app.idioma();
    if !app.presentacion_mut().ventanas().modal_conexiones {
        return;
    }

    app.sincronizar_espacio_de_trabajo_con_el_mapa();

    // La primera vez que se abre, se buscan los agentes. Se guarda el resultado para no
    // recorrer el disco en cada fotograma mientras la ventana está abierta.
    if app.agentes_mut().detectados.is_none() {
        app.agentes_mut().detectados = Some(conectores::detectar_agentes());
    }

    let mut abierta = app.presentacion_mut().ventanas().modal_conexiones;
    let mut acciones = AccionesDelModalDeConexiones::default();

    egui::Window::new(Texto::ModalConectarMmceltConMis.en(idioma))
        .open(&mut abierta)
        .min_width(620.0)
        .default_width(680.0)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.label(RichText::new(Texto::ModalRegistraMmceltEnTus.en(idioma)).size(14.0));

            ui.add_space(2.0);
            ui.label(
                RichText::new(Texto::ModalNoHaceFaltaInstalar.en(idioma))
                    .small()
                    .italics()
                    .color(app.presentacion().tema().texto_atenuado),
            );

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            dibujar_la_carpeta_de_trabajo(app, ui, idioma);
            dibujar_la_lista_de_agentes(app, ui, idioma, &mut acciones);
            dibujar_el_pie_de_la_ventana(app, ui, idioma, &mut acciones);
        });

    if !abierta {
        app.presentacion_mut().ventanas().modal_conexiones = false;
    }

    aplicar_las_acciones_del_modal(app, idioma, acciones);
}

/// Dibuja la carpeta de trabajo de los agentes en modo solo lectura.
///
/// # Parámetros
/// - `app`: el estado, del que salen la carpeta y los colores.
/// - `ui`: la interfaz de `egui`.
/// - `idioma`: el idioma en que se escriben los textos.
fn dibujar_la_carpeta_de_trabajo(
    app: &AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
) {
    // -----------------------------------------------------------------
    // Carpeta de trabajo
    // -----------------------------------------------------------------
    ui.label(RichText::new(Texto::ModalCarpetaDeTrabajo.en(idioma)).strong());
    ui.label(
        RichText::new(Texto::ModalLosAgentesSoloPodran.en(idioma))
            .small()
            .color(app.presentacion().tema().texto_secundario),
    );
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        let texto_espacio =
            conectores::ruta_de_trabajo_para_interfaz(&app.agentes().espacio_de_trabajo)
                .unwrap_or_else(|| Texto::ModalSinCarpetaDeProyecto.en(idioma).to_string());
        ui.label(
            RichText::new(texto_espacio)
                .monospace()
                .color(app.presentacion().tema().colores_de_las_ramas[0]),
        );
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);
}

/// Dibuja la lista de agentes detectados, o el aviso de que no hay ninguno.
///
/// # Parámetros
/// - `app`: el estado, del que sale la lista ya detectada.
/// - `ui`: la interfaz de `egui`.
/// - `idioma`: el idioma en que se escriben los textos.
/// - `acciones`: donde se anota a qué agente hay que conectarse o desconectarse.
fn dibujar_la_lista_de_agentes(
    app: &AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
    acciones: &mut AccionesDelModalDeConexiones,
) {
    // -----------------------------------------------------------------
    // Lista de agentes
    // -----------------------------------------------------------------
    let agentes = app.agentes().detectados.as_deref().unwrap_or(&[]);

    if agentes.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new(Texto::ModalNoSeEncontroNingun.en(idioma))
                    .strong()
                    .size(15.0),
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(Texto::ModalMmceltReconoceClaudeCode.en(idioma))
                    .color(app.presentacion().tema().texto_secundario),
            );
            ui.label(
                RichText::new(Texto::ModalInstalaAlgunoYPulsa.en(idioma))
                    .small()
                    .italics()
                    .color(app.presentacion().tema().texto_atenuado),
            );
            ui.add_space(10.0);
        });
    } else {
        ui.label(RichText::new(Texto::ModalAgentesEncontrados.en(idioma)).strong());
        ui.add_space(6.0);

        egui::ScrollArea::vertical()
            .max_height(280.0)
            .show(ui, |ui| {
                for (indice, agente) in agentes.iter().enumerate() {
                    dibujar_agente(
                        ui,
                        app,
                        agente,
                        indice,
                        &mut acciones.conectar,
                        &mut acciones.desconectar,
                    );
                    ui.add_space(6.0);
                }
            });
    }
}

/// Dibuja el pie de la ventana: volver a buscar, cerrar y el aviso final.
///
/// # Parámetros
/// - `app`: el estado; se recibe mutable porque el botón de cerrar actúa sobre él.
/// - `ui`: la interfaz de `egui`.
/// - `idioma`: el idioma en que se escriben los textos.
/// - `acciones`: donde se anota que hay que volver a buscar agentes.
fn dibujar_el_pie_de_la_ventana(
    app: &mut AplicacionMapaMental,
    ui: &mut egui::Ui,
    idioma: crate::textos::Idioma,
    acciones: &mut AccionesDelModalDeConexiones,
) {
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui
            .button(Texto::ModalVolverABuscar.en(idioma))
            .on_hover_text(Texto::ModalBuscaDeNuevoPor.en(idioma))
            .clicked()
        {
            acciones.volver_a_buscar = true;
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(Texto::BotonCerrar.en(app.idioma())).clicked() {
                app.presentacion_mut().ventanas().modal_conexiones = false;
            }
        });
    });

    ui.add_space(6.0);
    ui.label(
        RichText::new(Texto::ModalDespuesDeConectarO.en(idioma))
            .small()
            .color(app.presentacion().tema().aviso),
    );
}

/// Aplica lo que el usuario pidió mientras se dibujaba la ventana.
///
/// # Parámetros
/// - `app`: el estado sobre el que se aplican los cambios.
/// - `idioma`: el idioma en que se escriben los avisos.
/// - `acciones`: lo que se anotó al dibujar.
fn aplicar_las_acciones_del_modal(
    app: &mut AplicacionMapaMental,
    idioma: crate::textos::Idioma,
    acciones: AccionesDelModalDeConexiones,
) {
    if acciones.volver_a_buscar {
        app.agentes_mut().detectados = Some(conectores::detectar_agentes());
        app.establecer_estado(Texto::ModalBusquedaDeAgentesActualizada.en(idioma));
    }

    if let Some(indice) = acciones.conectar {
        aplicar_conexion(app, indice, true);
    } else if let Some(indice) = acciones.desconectar {
        aplicar_conexion(app, indice, false);
    }
}

/// Dibuja la ficha de un agente con su estado y sus botones.
///
/// El estado no sale de un `bool`, sino de comparar lo que el agente tiene **registrado en
/// su archivo** con la carpeta que esta ventana está mostrando: ver
/// [`conectores::estado_de_la_conexion`]. Un agente registrado con otra carpeta deja de
/// lucir «conectado», porque lo que obedecería no es lo que la ventana promete.
///
/// # Parámetros
/// - `ui`: la interfaz de `egui`.
/// - `app`: el estado, del que salen el idioma, los colores y la carpeta en uso.
/// - `agente`: el agente que se dibuja.
/// - `indice`: su posición en la lista detectada, que es como se anotan las acciones.
/// - `conectar`: donde se anota que hay que registrarlo con la carpeta actual.
/// - `desconectar`: donde se anota que hay que retirar el registro.
fn dibujar_agente(
    ui: &mut egui::Ui,
    app: &AplicacionMapaMental,
    agente: &Agente,
    indice: usize,
    conectar: &mut Option<usize>,
    desconectar: &mut Option<usize>,
) {
    let idioma = app.idioma();
    let estado = conectores::estado_de_la_conexion(agente, &app.agentes().espacio_de_trabajo);

    egui::Frame::group(ui.style())
        .fill(app.presentacion().tema().fondo_de_la_insignia)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ficha_del_agente(ui, app, agente, idioma, estado);

                botones_del_agente(ui, app, idioma, estado, indice, conectar, desconectar);
            });

            if agente.id == conectores::IdAgente::GeminiCli {
                ui.add_space(4.0);
                ui.add(
                    egui::Label::new(
                        RichText::new(Texto::ModalAdvertenciaGeminiCli.en(idioma))
                            .small()
                            .color(app.presentacion().tema().aviso),
                    )
                    .wrap(),
                );
            }
        });
}

/// Dibuja una ficha de agente dentro del arnés de interfaz.
///
/// La ventana flotante de `egui` no entrega sus formas al `Ui` raíz del arnés. Esta entrada
/// de prueba ejecuta exactamente la misma ficha que usa el modal, sin duplicar su lógica ni
/// convertir la comprobación en una inspección del código fuente.
#[cfg(test)]
pub(crate) fn dibujar_agente_para_prueba(
    app: &AplicacionMapaMental,
    agente: &Agente,
    ui: &mut egui::Ui,
) {
    let mut conectar = None;
    let mut desconectar = None;
    dibujar_agente(ui, app, agente, 0, &mut conectar, &mut desconectar);
}

/// Escribe qué carpeta autoriza de verdad la configuración de un agente descolocado.
///
/// Solo se pinta cuando hay discrepancia y la carpeta registrada se ha podido leer. Decir
/// «apunta a otra carpeta» sin decir a cuál obligaría a la persona a abrir el archivo del
/// agente para averiguarlo, que es justo lo que esta ventana existe para evitar.
///
/// # Parámetros
/// - `ui`: la interfaz de `egui`.
/// - `app`: el estado, del que salen los colores.
/// - `agente`: el agente cuya configuración se está describiendo.
/// - `idioma`: el idioma en que se escriben los textos.
/// - `estado`: en qué situación está respecto a la carpeta en uso.
fn dibujar_la_carpeta_registrada(
    ui: &mut egui::Ui,
    app: &AplicacionMapaMental,
    agente: &Agente,
    idioma: crate::textos::Idioma,
    estado: EstadoDeLaConexion,
) {
    if estado != EstadoDeLaConexion::ConectadoAOtraCarpeta {
        return;
    }

    let Some(registrada) = agente.espacio_registrado.as_deref() else {
        return;
    };

    ui.label(
        RichText::new(Texto::ModalApuntaALaCarpeta.en(idioma))
            .small()
            .color(app.presentacion().tema().aviso),
    );
    ui.label(
        RichText::new(registrada)
            .monospace()
            .small()
            .color(app.presentacion().tema().texto_atenuado),
    );
}

/// Ejecuta la conexión o desconexión y comunica el resultado.
///
/// # Parámetros
/// - `app`: estado de la aplicación.
/// - `indice`: posición del agente en la lista detectada.
/// - `conectar`: `true` para registrar, `false` para quitar el registro.
fn aplicar_conexion(app: &mut AplicacionMapaMental, indice: usize, conectar: bool) {
    let idioma = app.idioma();
    let Some(agentes) = &app.agentes().detectados else {
        return;
    };
    let Some(agente) = agentes.get(indice).cloned() else {
        return;
    };

    // La carpeta debe existir antes de registrarla: un agente que arranque apuntando a
    // una carpeta inexistente fallaría en su primera operación.
    if conectar {
        if let Err(e) =
            app.preparar_carpeta_de_trabajo(std::path::Path::new(&app.agentes().espacio_de_trabajo))
        {
            app.reportar_error(&e, Texto::ModalCrearLaCarpetaDe.en(idioma));
            return;
        }

        // El clic de conexión es la acción explícita que convierte la carpeta mostrada en un
        // proyecto identificable. Si la carpeta conserva una identidad después de moverse, este
        // mismo clic confirma la nueva ubicación sin generar otro UUID.
        let carga = match crate::proyecto_trabajo::RepositorioProyecto::abrir_o_inicializar(
            &app.agentes().espacio_de_trabajo,
        ) {
            Ok(carga) => carga,
            Err(error) => {
                app.reportar_error(&error, Texto::ModalConectarConUnAgente.en(idioma));
                return;
            }
        };
        if let crate::proyecto_trabajo::CargaProyecto::Trasladado {
            configuracion,
            raiz_detectada,
        } = carga
        {
            if let Err(error) = crate::proyecto_trabajo::RepositorioProyecto::confirmar_traslado(
                configuracion,
                &raiz_detectada,
            ) {
                app.reportar_error(&error, Texto::ModalConectarConUnAgente.en(idioma));
                return;
            }
        }
    }

    let espacio = app.agentes().espacio_de_trabajo.clone();

    let resultado = if conectar {
        conectores::conectar(&agente, &espacio)
    } else {
        conectores::desconectar(&agente)
    };

    match resultado {
        Ok(respaldo) => {
            let accion = if conectar {
                Texto::ModalConectadoCon.en(idioma)
            } else {
                Texto::ModalDesconectadoDe.en(idioma)
            };
            let mut mensaje = Texto::ModalMmceltAccionReinicialoPara
                .en(idioma)
                .replace("{accion}", accion)
                .replacen("{}", agente.nombre, 1);

            if let Some(ruta) = respaldo {
                mensaje.push_str(&Texto::ModalCopiaDeSeguridad.en(idioma).replacen(
                    "{}",
                    &ruta.display().to_string(),
                    1,
                ));
            }

            app.establecer_estado(mensaje);

            // Se rehace la detección para que el estado mostrado sea el real, y no una
            // suposición sobre lo que acaba de escribirse.
            app.agentes_mut().detectados = Some(conectores::detectar_agentes());
        }
        Err(e) => {
            let contexto = if conectar {
                Texto::ModalConectarConUnAgente.en(idioma)
            } else {
                Texto::ModalDesconectarDeUnAgente.en(idioma)
            };
            app.reportar_error(&e, contexto);
        }
    }
}

/// La ficha de un agente: su nombre, en qué estado está, qué es y qué archivo se le toca.
///
/// La ruta del archivo de configuración se muestra a propósito: quien quiera revisarla o
/// editarla a mano tiene que saber exactamente cuál es. Y si todavía no existe, se dice, para
/// que nadie vaya a buscarla y no la encuentre.
///
/// # Parámetros
/// - `ui`: el `Ui` de la fila del agente.
/// - `app`: estado de la aplicación, del que salen el tema y el espacio de trabajo.
/// - `agente`: el agente que se describe.
/// - `idioma`: el del usuario, para los rótulos.
/// - `estado`: en qué situación está su conexión con MMCelt.
fn ficha_del_agente(
    ui: &mut egui::Ui,
    app: &AplicacionMapaMental,
    agente: &Agente,
    idioma: crate::textos::Idioma,
    estado: EstadoDeLaConexion,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let (icono, color) = match estado {
                EstadoDeLaConexion::Conectado => ("✅", app.presentacion().tema().exito),
                EstadoDeLaConexion::ConectadoAOtraCarpeta => ("⚠", app.presentacion().tema().aviso),
                EstadoDeLaConexion::SinConectar => ("○", app.presentacion().tema().texto_atenuado),
            };

            ui.label(RichText::new(icono).color(color));
            ui.label(RichText::new(agente.nombre).strong().size(15.0));

            match estado {
                EstadoDeLaConexion::Conectado => {
                    ui.label(
                        RichText::new(Texto::ModalConectado.en(idioma))
                            .small()
                            .color(app.presentacion().tema().exito),
                    );
                }
                EstadoDeLaConexion::ConectadoAOtraCarpeta => {
                    ui.label(
                        RichText::new(Texto::ModalConectadoAOtraCarpeta.en(idioma))
                            .small()
                            .color(app.presentacion().tema().aviso),
                    );
                }
                EstadoDeLaConexion::SinConectar => {}
            }
        });

        ui.label(
            RichText::new(agente.descripcion)
                .small()
                .color(app.presentacion().tema().texto_secundario),
        );

        ui.label(
            RichText::new(agente.ruta_configuracion.display().to_string())
                .monospace()
                .small()
                .color(app.presentacion().tema().texto_atenuado),
        );

        dibujar_la_carpeta_registrada(ui, app, agente, idioma, estado);

        if !agente.tiene_configuracion() {
            ui.label(
                RichText::new(Texto::ModalElArchivoSeCreara.en(idioma))
                    .small()
                    .italics()
                    .color(app.presentacion().tema().texto_atenuado),
            );
        }
    });
}

/// Los botones de la derecha de un agente: conectar, desconectar o corregir la carpeta.
///
/// # Por qué la reparación es un botón
///
/// Cuando el agente está registrado pero apuntando a otra carpeta, MMCelt **no** lo arregla
/// solo al abrir la ventana. El archivo es de otro programa, y solo se toca cuando la persona
/// lo pide. Al pulsarlo se vuelve a registrar el agente con la carpeta que se está mostrando,
/// dejando copia de seguridad como cualquier otra conexión.
///
/// # Parámetros
/// - `ui`: el `Ui` de la fila del agente.
/// - `app`: estado de la aplicación, del que sale el tema.
/// - `idioma`: el del usuario, para los rótulos.
/// - `estado`: en qué situación está su conexión, que decide qué botones se ofrecen.
/// - `indice`: la posición del agente en la lista, que es lo que se comunica al llamador.
/// - `conectar`: dónde se anota que hay que conectar este agente, o volver a registrarlo.
/// - `desconectar`: dónde se anota que hay que retirarlo.
fn botones_del_agente(
    ui: &mut egui::Ui,
    app: &AplicacionMapaMental,
    idioma: crate::textos::Idioma,
    estado: EstadoDeLaConexion,
    indice: usize,
    conectar: &mut Option<usize>,
    desconectar: &mut Option<usize>,
) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if estado == EstadoDeLaConexion::SinConectar {
            if ui
                .button(
                    RichText::new(Texto::ModalConectar.en(idioma))
                        .strong()
                        .color(app.presentacion().tema().exito),
                )
                .on_hover_text(Texto::ModalRegistraMmceltEnEste.en(idioma))
                .clicked()
            {
                *conectar = Some(indice);
            }
            return;
        }

        if ui
            .button(Texto::ModalDesconectar.en(idioma))
            .on_hover_text(Texto::ModalQuitaMmceltDeEste.en(idioma))
            .clicked()
        {
            *desconectar = Some(indice);
        }

        if estado == EstadoDeLaConexion::ConectadoAOtraCarpeta
            && ui
                .button(
                    RichText::new(Texto::ModalActualizarLaCarpeta.en(idioma))
                        .strong()
                        .color(app.presentacion().tema().aviso),
                )
                .on_hover_text(Texto::ModalRegistraMmceltEnEste.en(idioma))
                .clicked()
        {
            *conectar = Some(indice);
        }
    });
}
