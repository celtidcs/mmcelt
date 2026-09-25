//! Orquestación desacoplada del envío de contexto y encargos a agentes de IA.
//!
//! Este módulo aísla la preparación, confirmación y atención del ciclo de vida del envío
//! a agentes de IA, liberando a `AplicacionMapaMental` de la lógica de negocio y
//! manteniendo interfaces explícitas y comprobables sin dependencias monolíticas.

use std::path::{Path, PathBuf};

use crate::aplicacion::ServiciosAplicacion;
use crate::conectores::{estado_de_la_conexion, Agente, EstadoDeLaConexion};
use crate::envio_agente::{EstadoEnvio, SolicitudEnvio, TrabajoEnvio};
use crate::error::{AppError, AppResult, MotivoCarpetaInvalida};
use crate::model::Proyecto;
use crate::proyecto_trabajo::RepositorioProyecto;
use crate::sesiones_agentes::{
    cargar_reglas_comunes, detectar_fuentes_nativas, BorradorSesion, ConfirmacionSesion,
};
use crate::textos::Idioma;
use crate::ui::dialogos;
use crate::ui::estado_agentes::EstadoAgentes;
use crate::ui::proyecto_ia_modal::EstadoEditorProyectoIa;
use crate::ui::sesion_agente_modal::{self, EditorSesionAgente};

/// Prepara en memoria la vista previa de una sesión con un agente de IA sin abrir procesos externos.
///
/// Si el mapa activo se encuentra en la raíz de un volumen, o si el agente está vinculado a otra
/// carpeta, o si no dispone de CLI ni conexión MCP, devuelve el error específico correspondiente.
pub(crate) fn preparar_envio(
    agentes: &mut EstadoAgentes,
    agente: &Agente,
    proyecto: &Proyecto,
    idioma_defecto: Idioma,
    es_solo_lectura_por_raiz: bool,
    editor_ia: Option<&EstadoEditorProyectoIa>,
) -> AppResult<()> {
    if es_solo_lectura_por_raiz {
        return Err(AppError::CarpetaProyectoInvalida {
            ruta: agentes.espacio_de_trabajo.clone(),
            motivo: MotivoCarpetaInvalida::MapaEnRaizDebeTrasladarse,
        });
    }
    if estado_de_la_conexion(agente, &agentes.espacio_de_trabajo)
        == EstadoDeLaConexion::ConectadoAOtraCarpeta
    {
        return Err(AppError::CarpetaProyectoInvalida {
            ruta: agentes.espacio_de_trabajo.clone(),
            motivo: MotivoCarpetaInvalida::AgenteRegistradoParaOtraCarpeta,
        });
    }
    if agente.cli.is_none() && !agente.conectado {
        return Err(AppError::lectura(
            "agente de IA",
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no hay consola ni conexión MCP disponibles para el agente",
            ),
        ));
    }
    let contexto = RepositorioProyecto::contexto_para_vista_previa(&agentes.espacio_de_trabajo)?;
    let ejecutable_confirmado = agente
        .cli
        .as_ref()
        .map(|_| crate::lanzador_agentes::confirmar_ejecutable(agente, contexto.raiz()))
        .transpose()?;
    let (reglas_originales, encargo_original, idioma_documento, fuentes_nativas) =
        if let Some(ed) = editor_ia.filter(|ed| ed.raiz_proyecto == *contexto.raiz()) {
            (
                ed.reglas.clone(),
                ed.encargo.clone(),
                ed.idioma_documento,
                ed.fuentes.clone(),
            )
        } else {
            let reglas = cargar_reglas_comunes(&contexto)?;
            let fuentes = detectar_fuentes_nativas(&contexto)?;
            let configuracion_agente =
                crate::configuracion_agente_proyecto::cargar_configuracion(&contexto)?;
            let idioma = configuracion_agente
                .idioma_documento
                .unwrap_or(idioma_defecto);
            let encargo_memoria = agentes.encargo_preparado_para(contexto.raiz());
            let encargo = if !encargo_memoria.is_empty() {
                encargo_memoria
            } else {
                configuracion_agente.encargo.clone()
            };
            (reglas, encargo, idioma, fuentes)
        };
    let borrador = BorradorSesion {
        agente: agente.id,
        contrato_mmcelt: crate::ai_export::contrato_mmcelt_para_agente(idioma_documento),
        reglas_proyecto: reglas_originales.clone(),
        contexto_mapa: crate::ai_export::exportar_contexto_mapa_para_agente(
            proyecto,
            idioma_documento,
        ),
        encargo: encargo_original.clone(),
        fuentes_nativas,
    };
    agentes.editor_sesion = Some(EditorSesionAgente {
        agente: agente.clone(),
        borrador,
        contexto,
        revision_mapa: proyecto.revision(),
        idioma_documento,
        ejecutable_confirmado,
        reglas_originales,
    });
    Ok(())
}

/// Descarta una vista previa no confirmada sin crear expediente ni alterar el mapa.
pub(crate) fn cancelar_envio(agentes: &mut EstadoAgentes) {
    if agentes.envio != EstadoEnvio::Preparando {
        agentes.editor_sesion = None;
        agentes.envio = EstadoEnvio::Inactivo;
    }
}

/// Copia la intención ya revisada en el modal sin exponer el estado gráfico al caso de uso.
pub(crate) fn solicitud_de_envio(
    agentes: &EstadoAgentes,
    revision_actual_mapa: crate::model::RevisionProyecto,
) -> AppResult<Option<SolicitudEnvio>> {
    let Some(editor) = agentes.editor_sesion.as_ref() else {
        return Ok(None);
    };
    let prompt = sesion_agente_modal::componer_vista_previa(&editor.borrador)?;
    let confirmacion = ConfirmacionSesion::nueva(
        prompt,
        editor.revision_mapa,
        &editor.reglas_originales,
        &editor.borrador.encargo,
        &editor.borrador.fuentes_nativas,
    );
    let reglas_actuales = cargar_reglas_comunes(&editor.contexto)?;
    let fuentes_actuales = detectar_fuentes_nativas(&editor.contexto)?;
    confirmacion.validar(
        revision_actual_mapa,
        &reglas_actuales,
        &editor.borrador.encargo,
        &fuentes_actuales,
    )?;
    Ok(Some(SolicitudEnvio {
        contexto: editor.contexto.clone(),
        confirmacion,
        ejecutable: editor.ejecutable_confirmado.clone(),
        conectado_por_mcp: editor.agente.conectado,
    }))
}

/// Conserva la ruta actual del mapa o solicita mediante diálogo dónde guardar un mapa nuevo.
pub(crate) fn elegir_ruta_para_envio(
    ruta_actual: Option<&Path>,
    carpeta_sugerida: Option<&Path>,
    titulo_proyecto: &str,
    idioma: Idioma,
) -> Option<PathBuf> {
    if let Some(ruta) = ruta_actual {
        return Some(ruta.to_path_buf());
    }
    let nombre = format!("{titulo_proyecto}.mmcelt");
    dialogos::pedir_archivo_para_guardar(&nombre, carpeta_sugerida, idioma)
}

/// Empaqueta el mapa y la solicitud confirmada en una estructura para el hilo de trabajo.
pub(crate) fn construir_trabajo_de_envio(
    proyecto: &Proyecto,
    idioma: Idioma,
    solicitud: SolicitudEnvio,
    ruta_mapa: PathBuf,
) -> TrabajoEnvio {
    TrabajoEnvio {
        solicitud,
        proyecto: proyecto.clone(),
        ruta_mapa,
        idioma,
    }
}

/// Confirma el borrador, solicita la tarea al trabajador en segundo plano y pasa a fase `Preparando`.
///
/// Devuelve `false` si el envío ya está en curso, si no hay borrador o si el usuario canceló el diálogo de guardado.
pub(crate) fn iniciar_envio_confirmado(
    agentes: &mut EstadoAgentes,
    proyecto: &Proyecto,
    ruta_actual: Option<&Path>,
    carpeta_sugerida: Option<&Path>,
    idioma: Idioma,
    ctx: &egui::Context,
) -> AppResult<bool> {
    if agentes.envio == EstadoEnvio::Preparando {
        return Ok(false);
    }
    let Some(solicitud) = solicitud_de_envio(agentes, proyecto.revision())? else {
        return Ok(false);
    };
    let Some(ruta_mapa) =
        elegir_ruta_para_envio(ruta_actual, carpeta_sugerida, &proyecto.title, idioma)
    else {
        return Ok(false);
    };
    let trabajo = construir_trabajo_de_envio(proyecto, idioma, solicitud, ruta_mapa);
    if !agentes.trabajador.solicitar(trabajo) {
        return Ok(false);
    }
    agentes.envio = EstadoEnvio::Preparando;
    ctx.request_repaint();
    Ok(true)
}

/// Resultado clasificado de la atención a eventos del trabajador de envío.
pub(crate) enum ReaccionEnvioSegundoPlano {
    /// No había trabajos terminados pendientes en la cola del trabajador.
    Ninguna,
    /// El proceso concluyó con éxito y la vigilancia del mapa quedó iniciada.
    Exito {
        /// Ruta y huella si el mapa tuvo que ser guardado durante la preparación.
        mapa_guardado: Option<(PathBuf, Option<u64>)>,
        /// Indica si la vigilancia se activó por primera vez para este mapa.
        iniciada: bool,
    },
    /// Ocurrió un fallo al ejecutar o al iniciar la vigilancia posterior.
    Fallo {
        /// Ruta y huella si el mapa se llegó a guardar antes de fallar.
        mapa_guardado: Option<(PathBuf, Option<u64>)>,
        error: AppError,
    },
}

/// Aplica en la capa de agentes el resultado que dejó listo el hilo auxiliar de envío.
pub(crate) fn atender_envio_en_segundo_plano(
    agentes: &mut EstadoAgentes,
    persistencia: &mut crate::ui::estado_persistencia::EstadoPersistencia,
    ctx: &egui::Context,
) -> ReaccionEnvioSegundoPlano {
    let Some(trabajo) = agentes.trabajador.intentar_recibir() else {
        return ReaccionEnvioSegundoPlano::Ninguna;
    };
    let mapa_guardado = if trabajo.mapa_guardado {
        Some((trabajo.ruta_mapa, trabajo.huella_guardada))
    } else {
        None
    };
    match trabajo.resultado {
        Ok(_) => match crate::vigilancia_mapa::iniciar_vigilancia(persistencia, ctx) {
            Ok(iniciada) => {
                agentes.envio = EstadoEnvio::Preparado;
                agentes.editor_sesion = None;
                ReaccionEnvioSegundoPlano::Exito {
                    mapa_guardado,
                    iniciada,
                }
            }
            Err(error) => {
                agentes.envio = EstadoEnvio::Fallido;
                ReaccionEnvioSegundoPlano::Fallo {
                    mapa_guardado,
                    error,
                }
            }
        },
        Err(error) => {
            agentes.envio = EstadoEnvio::Fallido;
            ReaccionEnvioSegundoPlano::Fallo {
                mapa_guardado,
                error,
            }
        }
    }
}

/// Resultado clasificado de la exportación de Markdown para IA solicitada por la persona.
pub(crate) enum ResultadoExportacionMarkdown {
    /// El mapa activo está en la raíz de un volumen (modo solo lectura por seguridad).
    SoloLecturaPorRaiz,
    /// El diálogo de selección de archivo fue cancelado por el usuario.
    Cancelado,
    /// El archivo Markdown se exportó satisfactoriamente a la ruta indicada.
    Exito(PathBuf),
    /// Ocurrió un error de entrada/salida o serialización durante la exportación.
    Fallo(AppError),
}

/// Exporta el mapa mental activo a formato Markdown estructurado para IA.
pub(crate) fn exportar_markdown_para_ia(
    proyecto: &Proyecto,
    es_solo_lectura_por_raiz: bool,
    idioma: Idioma,
    servicios: &ServiciosAplicacion,
) -> ResultadoExportacionMarkdown {
    if es_solo_lectura_por_raiz {
        return ResultadoExportacionMarkdown::SoloLecturaPorRaiz;
    }
    let nombre_propuesto = format!("{}_AI.md", proyecto.title);
    let Some(ruta) = dialogos::pedir_archivo_para_exportar(&nombre_propuesto, idioma) else {
        return ResultadoExportacionMarkdown::Cancelado;
    };
    match servicios.exportar(proyecto, &ruta, idioma) {
        Ok(()) => ResultadoExportacionMarkdown::Exito(ruta),
        Err(e) => ResultadoExportacionMarkdown::Fallo(e),
    }
}

/// Adaptador de persistencia para flujos de envío con ejecutor inyectado en pruebas.
#[cfg(test)]
#[allow(clippy::type_complexity)]
pub(crate) struct PersistenciaEnvioAplicacion<'a> {
    guardar_fn: Box<dyn FnMut() -> Option<PathBuf> + 'a>,
    exportar_fn: Box<dyn FnMut(&Path) -> AppResult<()> + 'a>,
}

#[cfg(test)]
impl<'a> PersistenciaEnvioAplicacion<'a> {
    /// Construye un nuevo adaptador vinculando las operaciones de guardado y exportación.
    pub(crate) fn nuevo<G, E>(guardar: G, exportar: E) -> Self
    where
        G: FnMut() -> Option<PathBuf> + 'a,
        E: FnMut(&Path) -> AppResult<()> + 'a,
    {
        Self {
            guardar_fn: Box::new(guardar),
            exportar_fn: Box::new(exportar),
        }
    }
}

#[cfg(test)]
impl crate::envio_agente::PersistenciaEnvio for PersistenciaEnvioAplicacion<'_> {
    fn guardar(&mut self) -> AppResult<crate::envio_agente::ResultadoPreparacion> {
        let Some(ruta_mapa) = (self.guardar_fn)() else {
            return Err(AppError::lectura(
                "mapa de la sesión",
                std::io::Error::new(std::io::ErrorKind::Interrupted, "guardado cancelado"),
            ));
        };
        Ok(crate::envio_agente::ResultadoPreparacion::Preparado { ruta_mapa })
    }

    fn exportar(&mut self, ruta_mapa: &Path) -> AppResult<()> {
        let nombre = ruta_mapa
            .file_stem()
            .and_then(|valor| valor.to_str())
            .unwrap_or("mapa");
        let ruta_markdown = ruta_mapa.with_file_name(format!("{nombre}_AI.md"));
        (self.exportar_fn)(&ruta_markdown)
    }
}

/// Ejecuta la confirmación del envío con un ejecutor inyectado y activa la vigilancia.
#[cfg(test)]
pub(crate) fn iniciar_envio_confirmado_con<E, P>(
    agentes: &mut EstadoAgentes,
    persistencia: &mut crate::ui::estado_persistencia::EstadoPersistencia,
    revision_mapa: crate::model::RevisionProyecto,
    ctx: &egui::Context,
    ejecutor: &E,
    persistencia_envio: &mut P,
) -> AppResult<bool>
where
    E: crate::lanzador_agentes::EjecutorDeConsola,
    P: crate::envio_agente::PersistenciaEnvio,
{
    let Some(solicitud) = solicitud_de_envio(agentes, revision_mapa)? else {
        return Ok(false);
    };
    {
        let lanzador = crate::envio_agente::LanzadorConEjecutor::nuevo(ejecutor);
        let sesiones = crate::envio_agente::RepositorioSesionesLocal;
        crate::envio_agente::EnvioAAgente::nuevo(&lanzador, persistencia_envio, &sesiones)
            .ejecutar(solicitud)?;
    }
    crate::vigilancia_mapa::iniciar_vigilancia(persistencia, ctx)?;
    agentes.editor_sesion = None;
    Ok(true)
}
