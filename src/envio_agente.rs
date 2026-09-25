//! Caso de uso completo para entregar un mapa confirmado a un agente de IA.
//!
//! La interfaz solo reúne la intención del usuario y representa el resultado. El orden de las
//! operaciones, sus cortes por error y la diferencia entre consola y MCP viven aquí para poder
//! comprobarse sin `egui`.

use crate::conectores::IdAgente;
use crate::error::{AppError, AppResult};
use crate::lanzador_agentes::{
    EjecutableConfirmado, EjecutorDeConsola, ResultadoLanzamiento as ProcesoIniciado,
};
use crate::model::Proyecto;
use crate::proyecto_trabajo::ContextoProyecto;
use crate::sesiones_agentes::{
    ConfirmacionSesion, EstadoLanzamiento, PromptSesionCompuesto, SesionPreparada,
};
use crate::textos::Idioma;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender, TrySendError};
use std::thread::JoinHandle;

/// Datos ya aprobados en la vista previa que necesita el recorrido de envío.
#[derive(Clone)]
pub struct SolicitudEnvio {
    /// Proyecto canónico dentro del cual deben quedar mapa y expediente.
    pub contexto: ContextoProyecto,
    /// Instantánea inmutable que el usuario acaba de revisar.
    pub confirmacion: ConfirmacionSesion,
    /// Consola exacta mostrada en pantalla, si el producto dispone de ella.
    pub ejecutable: Option<EjecutableConfirmado>,
    /// Indica que el agente puede recibir y devolver el mapa aunque no haya consola.
    pub conectado_por_mcp: bool,
}

/// Resultado del paso de guardado que precede a cualquier entrega.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultadoPreparacion {
    /// El mapa quedó guardado en la ruta indicada y puede exportarse.
    Preparado { ruta_mapa: PathBuf },
}

/// Resultado completo que la interfaz traduce a vigilancia y estado visual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultadoLanzamiento {
    /// El expediente está listo para un cliente que se comunica únicamente mediante MCP.
    PreparadoMcp { sesion: SesionPreparada },
    /// El sistema confirmó la creación del proceso de consola.
    Iniciado { sesion: SesionPreparada, pid: u32 },
}

/// Estado visible de la confirmación de «Enviar a…».
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EstadoEnvio {
    /// No hay ninguna preparación en curso ni un resultado reciente que mostrar.
    #[default]
    Inactivo,
    /// El mapa, el Markdown y el expediente se están escribiendo fuera de la interfaz.
    Preparando,
    /// La preparación terminó y el agente recibió el expediente.
    Preparado,
    /// Algún paso falló; el borrador visible se conserva para reintentar.
    Fallido,
}

/// Copia autosuficiente de todo lo que el trabajador necesita para preparar un envío.
pub struct TrabajoEnvio {
    /// Solicitud confirmada que conserva agente, capacidades y prompt aprobado.
    pub solicitud: SolicitudEnvio,
    /// Instantánea del mapa que debe persistirse sin volver a tocar la interfaz.
    pub proyecto: Proyecto,
    /// Destino ya elegido para guardar y vigilar el mapa.
    pub ruta_mapa: PathBuf,
    /// Idioma en el que se exportará el documento para el agente.
    pub idioma: Idioma,
}

/// Resultado que vuelve del trabajador al estado gráfico.
pub struct ResultadoTrabajoEnvio {
    /// Ruta del mapa al que pertenece el resultado, usada para descartar respuestas obsoletas.
    pub ruta_mapa: PathBuf,
    /// Permite registrar correctamente un guardado aunque falle un paso posterior.
    pub mapa_guardado: bool,
    /// Huella calculada en el trabajador para que la interfaz no tenga que releer el mapa.
    pub huella_guardada: Option<u64>,
    /// Sesión preparada o fallo tipado del primer paso que no pudo completarse.
    pub resultado: AppResult<ResultadoLanzamiento>,
}

/// Ejecuta como máximo un envío y deja otro en espera, sin crear un hilo por clic.
pub struct EnvioEnSegundoPlano {
    solicitudes: Option<SyncSender<TrabajoEnvio>>,
    resultados: Receiver<ResultadoTrabajoEnvio>,
    trabajador: Option<JoinHandle<()>>,
}

impl EnvioEnSegundoPlano {
    /// Arranca el trabajador real y lo vincula al repintado de la aplicación.
    pub fn nuevo(ctx: egui::Context) -> Self {
        Self::con_funcion_interna(ctx, ejecutar_trabajo)
    }

    /// Intenta iniciar una preparación sin bloquear a quien dibuja el fotograma.
    pub fn solicitar(&self, trabajo: TrabajoEnvio) -> bool {
        let Some(solicitudes) = &self.solicitudes else {
            return false;
        };
        match solicitudes.try_send(trabajo) {
            Ok(()) => true,
            Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => false,
        }
    }

    /// Recoge el resultado, si ya está disponible, sin esperar.
    pub fn intentar_recibir(&self) -> Option<ResultadoTrabajoEnvio> {
        self.resultados.try_recv().ok()
    }

    fn con_funcion_interna<F>(ctx: egui::Context, ejecutar: F) -> Self
    where
        F: Fn(TrabajoEnvio) -> ResultadoTrabajoEnvio + Send + 'static,
    {
        let (solicitudes_tx, solicitudes_rx) = sync_channel::<TrabajoEnvio>(1);
        let (resultados_tx, resultados_rx) = channel::<ResultadoTrabajoEnvio>();
        let trabajador = std::thread::Builder::new()
            .name("mmcelt-envio".to_string())
            .spawn(move || {
                while let Ok(trabajo) = solicitudes_rx.recv() {
                    let resultado = ejecutar(trabajo);
                    if resultados_tx.send(resultado).is_err() {
                        break;
                    }
                    ctx.request_repaint();
                }
            })
            .expect("no se pudo crear el trabajador de envío");
        Self {
            solicitudes: Some(solicitudes_tx),
            resultados: resultados_rx,
            trabajador: Some(trabajador),
        }
    }

    #[cfg(test)]
    fn con_funcion<F>(ctx: egui::Context, ejecutar: F) -> Self
    where
        F: Fn(TrabajoEnvio) -> ResultadoTrabajoEnvio + Send + 'static,
    {
        Self::con_funcion_interna(ctx, ejecutar)
    }
}

impl Drop for EnvioEnSegundoPlano {
    fn drop(&mut self) {
        self.solicitudes.take();
        if let Some(trabajador) = self.trabajador.take() {
            let _ = trabajador.join();
        }
    }
}

/// Persistencia local poseída por el trabajador, sin referencias al estado de `egui`.
struct PersistenciaEnvioLocal {
    proyecto: Proyecto,
    ruta_mapa: PathBuf,
    idioma: Idioma,
    mapa_guardado: bool,
}

impl PersistenciaEnvio for PersistenciaEnvioLocal {
    fn guardar(&mut self) -> AppResult<ResultadoPreparacion> {
        crate::storage::guardar_proyecto_en_archivo(&self.proyecto, &self.ruta_mapa)?;
        self.mapa_guardado = true;
        Ok(ResultadoPreparacion::Preparado {
            ruta_mapa: self.ruta_mapa.clone(),
        })
    }

    fn exportar(&mut self, ruta_mapa: &Path) -> AppResult<()> {
        let nombre = ruta_mapa
            .file_stem()
            .and_then(|valor| valor.to_str())
            .unwrap_or("mapa");
        let ruta_markdown = ruta_mapa.with_file_name(format!("{nombre}_AI.md"));
        crate::storage::exportar_proyecto_a_markdown(&self.proyecto, &ruta_markdown, self.idioma)
    }
}

fn ejecutar_trabajo(trabajo: TrabajoEnvio) -> ResultadoTrabajoEnvio {
    let ruta_mapa = trabajo.ruta_mapa.clone();
    match trabajo.solicitud.contexto.contiene(&ruta_mapa) {
        Ok(true) => {}
        Ok(false) => {
            return ResultadoTrabajoEnvio {
                ruta_mapa,
                mapa_guardado: false,
                huella_guardada: None,
                resultado: Err(AppError::RutaFueraDelProyecto),
            };
        }
        Err(error) => {
            return ResultadoTrabajoEnvio {
                ruta_mapa,
                mapa_guardado: false,
                huella_guardada: None,
                resultado: Err(error),
            };
        }
    }
    let mut persistencia = PersistenciaEnvioLocal {
        proyecto: trabajo.proyecto,
        ruta_mapa: trabajo.ruta_mapa,
        idioma: trabajo.idioma,
        mapa_guardado: false,
    };
    let ejecutor = crate::lanzador_agentes::ConsolaDelSistema;
    let lanzador = LanzadorConEjecutor::nuevo(&ejecutor);
    let sesiones = RepositorioSesionesLocal;
    let resultado =
        EnvioAAgente::nuevo(&lanzador, &mut persistencia, &sesiones).ejecutar(trabajo.solicitud);
    ResultadoTrabajoEnvio {
        ruta_mapa,
        mapa_guardado: persistencia.mapa_guardado,
        huella_guardada: persistencia
            .mapa_guardado
            .then(|| crate::proyectos::huella(&persistencia.ruta_mapa))
            .flatten(),
        resultado,
    }
}

/// Persistencia mínima que el caso de uso necesita para guardar y exportar el mapa.
pub trait PersistenciaEnvio {
    /// Guarda el mapa y devuelve la ruta que ya está preparada para los pasos siguientes.
    fn guardar(&mut self) -> AppResult<ResultadoPreparacion>;

    /// Escribe la representación Markdown junto al mapa ya guardado.
    fn exportar(&mut self, ruta_mapa: &Path) -> AppResult<()>;
}

/// Repositorio mínimo de expedientes y sus transiciones de estado.
pub trait RepositorioSesiones {
    /// Persiste el expediente con el prompt aprobado.
    fn preparar(
        &self,
        contexto: &ContextoProyecto,
        ruta_mapa: &Path,
        prompt: &PromptSesionCompuesto,
    ) -> AppResult<SesionPreparada>;

    /// Persiste una transición sin alterar la sesión en memoria antes de escribirla.
    fn actualizar(
        &self,
        contexto: &ContextoProyecto,
        sesion: &mut SesionPreparada,
        estado: EstadoLanzamiento,
    ) -> AppResult<()>;
}

/// Límite que convierte un expediente en un proceso, sin exponer detalles de consola al caso de uso.
pub trait LanzadorAgente {
    /// Abre la consola exacta usando programa y argumentos separados.
    fn lanzar(
        &self,
        agente: IdAgente,
        ejecutable: EjecutableConfirmado,
        contexto: &ContextoProyecto,
        sesion: &SesionPreparada,
    ) -> AppResult<ProcesoIniciado>;
}

/// Coordinador del recorrido guardar → exportar → expediente → lanzar.
pub struct EnvioAAgente<'a, E, P, S> {
    lanzador: &'a E,
    persistencia: &'a mut P,
    sesiones: &'a S,
}

impl<'a, E, P, S> EnvioAAgente<'a, E, P, S>
where
    E: LanzadorAgente,
    P: PersistenciaEnvio,
    S: RepositorioSesiones,
{
    /// Reúne dependencias pequeñas sin ocultarlas en un contenedor global.
    pub fn nuevo(lanzador: &'a E, persistencia: &'a mut P, sesiones: &'a S) -> Self {
        Self {
            lanzador,
            persistencia,
            sesiones,
        }
    }

    /// Ejecuta la entrega en orden y detiene los pasos posteriores ante el primer fallo.
    ///
    /// # Errores
    ///
    /// Devuelve el primer error al guardar o exportar el mapa, validar su pertenencia al proyecto,
    /// preparar el expediente, abrir la consola o persistir el estado del lanzamiento.
    ///
    /// # Efectos secundarios
    ///
    /// Guarda y exporta el mapa, crea el expediente de sesión y, cuando hay un ejecutable CLI,
    /// abre su consola. Si solo existe MCP, deja el expediente preparado sin crear un proceso.
    pub fn ejecutar(&mut self, solicitud: SolicitudEnvio) -> AppResult<ResultadoLanzamiento> {
        if solicitud.ejecutable.is_none() && !solicitud.conectado_por_mcp {
            return Err(AppError::lectura(
                "agente de IA",
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "la consola y la conexión MCP dejaron de estar disponibles",
                ),
            ));
        }
        let ResultadoPreparacion::Preparado { ruta_mapa } = self.persistencia.guardar()?;
        if !solicitud.contexto.contiene(&ruta_mapa)? {
            return Err(AppError::RutaFueraDelProyecto);
        }
        self.persistencia.exportar(&ruta_mapa)?;
        let mut sesion = self.sesiones.preparar(
            &solicitud.contexto,
            &ruta_mapa,
            &solicitud.confirmacion.prompt,
        )?;
        let Some(ejecutable) = solicitud.ejecutable else {
            return Ok(ResultadoLanzamiento::PreparadoMcp { sesion });
        };
        let proceso = match self.lanzador.lanzar(
            solicitud.confirmacion.prompt.agente(),
            ejecutable,
            &solicitud.contexto,
            &sesion,
        ) {
            Ok(proceso) => proceso,
            Err(error) => {
                // Registrar el intento fallido es secundario: si también falla, el error que
                // explica por qué no se abrió la consola sigue siendo el resultado principal.
                let _ = self.sesiones.actualizar(
                    &solicitud.contexto,
                    &mut sesion,
                    EstadoLanzamiento::Fallido,
                );
                return Err(error);
            }
        };
        self.sesiones.actualizar(
            &solicitud.contexto,
            &mut sesion,
            EstadoLanzamiento::Iniciado,
        )?;
        Ok(ResultadoLanzamiento::Iniciado {
            sesion,
            pid: proceso.pid,
        })
    }
}

/// Adaptador de los expedientes locales existentes.
pub struct RepositorioSesionesLocal;

impl RepositorioSesiones for RepositorioSesionesLocal {
    fn preparar(
        &self,
        contexto: &ContextoProyecto,
        ruta_mapa: &Path,
        prompt: &PromptSesionCompuesto,
    ) -> AppResult<SesionPreparada> {
        crate::sesiones_agentes::preparar_sesion(contexto, ruta_mapa, prompt)
    }

    fn actualizar(
        &self,
        contexto: &ContextoProyecto,
        sesion: &mut SesionPreparada,
        estado: EstadoLanzamiento,
    ) -> AppResult<()> {
        crate::sesiones_agentes::actualizar_estado(contexto, sesion, estado)
    }
}

/// Adaptador que conserva la orden estructurada y delega solo la creación del proceso.
pub struct LanzadorConEjecutor<'a, E> {
    ejecutor: &'a E,
}

impl<'a, E> LanzadorConEjecutor<'a, E> {
    /// Usa el ejecutor real o un sustituto explícito de pruebas.
    pub fn nuevo(ejecutor: &'a E) -> Self {
        Self { ejecutor }
    }
}

impl<E: EjecutorDeConsola> LanzadorAgente for LanzadorConEjecutor<'_, E> {
    fn lanzar(
        &self,
        agente: IdAgente,
        ejecutable: EjecutableConfirmado,
        contexto: &ContextoProyecto,
        sesion: &SesionPreparada,
    ) -> AppResult<ProcesoIniciado> {
        let orden = crate::lanzador_agentes::orden_para(agente, ejecutable, contexto, sesion)?;
        self.ejecutor.abrir(&orden)
    }
}

#[cfg(test)]
mod pruebas_segundo_plano {
    use super::*;
    use crate::model::Proyecto;
    use crate::proyecto_trabajo::IdProyecto;
    use crate::textos::Idioma;
    use std::sync::mpsc::sync_channel;
    use std::time::{Duration, Instant};

    fn trabajo_de_prueba(carpeta: &Path) -> TrabajoEnvio {
        TrabajoEnvio {
            solicitud: SolicitudEnvio {
                contexto: ContextoProyecto::nuevo(IdProyecto::nuevo(), "prueba", carpeta)
                    .expect("contexto de prueba"),
                confirmacion: crate::sesiones_agentes::ConfirmacionSesion::nueva(
                    crate::sesiones_agentes::componer_prompt_validado(
                        &crate::sesiones_agentes::BorradorSesion {
                            agente: IdAgente::CodexCli,
                            contrato_mmcelt: String::new(),
                            reglas_proyecto: String::new(),
                            contexto_mapa: String::new(),
                            encargo: String::new(),
                            fuentes_nativas: Vec::new(),
                        },
                    )
                    .expect("componer prompt de prueba"),
                    crate::model::RevisionProyecto::INICIAL,
                    "",
                    "",
                    &[],
                ),
                ejecutable: None,
                conectado_por_mcp: true,
            },
            proyecto: Proyecto::nuevo_vacio("Prueba"),
            ruta_mapa: carpeta.join("prueba.mmcelt"),
            idioma: Idioma::Espanol,
        }
    }

    #[test]
    fn preparar_un_envio_no_bloquea_el_fotograma() {
        let carpeta = std::env::temp_dir().join(format!("mmcelt-envio-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&carpeta).expect("crear carpeta de prueba");
        let trabajo = trabajo_de_prueba(&carpeta);
        let (iniciado_tx, iniciado_rx) = sync_channel(0);
        let (continuar_tx, continuar_rx) = sync_channel(0);
        let preparador =
            EnvioEnSegundoPlano::con_funcion(egui::Context::default(), move |trabajo| {
                iniciado_tx.send(()).expect("avisar del inicio");
                continuar_rx.recv().expect("autorizar el final");
                ResultadoTrabajoEnvio {
                    ruta_mapa: trabajo.ruta_mapa,
                    mapa_guardado: true,
                    huella_guardada: Some(7),
                    resultado: Ok(ResultadoLanzamiento::PreparadoMcp {
                        sesion: SesionPreparada {
                            carpeta: PathBuf::new(),
                            ruta_inicio: PathBuf::new(),
                            ruta_metadatos: PathBuf::new(),
                            metadatos: crate::sesiones_agentes::MetadatosSesion {
                                version_formato: 1,
                                id_sesion: uuid::Uuid::nil(),
                                fecha_utc: chrono::DateTime::UNIX_EPOCH,
                                agente: IdAgente::CodexCli,
                                mapa_relativo: PathBuf::from("prueba.mmcelt"),
                                fuentes_incorporadas: Vec::new(),
                                fuentes_nativas_detectadas: Vec::new(),
                                prompt_bytes: 0,
                                estado: EstadoLanzamiento::Preparado,
                            },
                        },
                    }),
                }
            });

        let comienzo = Instant::now();
        assert!(preparador.solicitar(trabajo));
        assert!(comienzo.elapsed() < Duration::from_millis(30));
        iniciado_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("el trabajo debe comenzar");
        assert!(preparador.intentar_recibir().is_none());
        continuar_tx.send(()).expect("terminar el trabajo");

        let limite = Instant::now() + Duration::from_secs(1);
        while preparador.intentar_recibir().is_none() {
            assert!(Instant::now() < limite, "el trabajo no terminó");
            std::thread::yield_now();
        }
        drop(preparador);
        std::fs::remove_dir_all(carpeta).expect("limpiar carpeta de prueba");
    }
}
