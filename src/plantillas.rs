//! Plantillas y ejemplos localizados para crear mapas de partida.

use crate::model::{EstadoNodo, PrioridadNodo, Proyecto, RolNodo, TipoRelacion};
use uuid::Uuid;

impl Proyecto {
    /// Crea un proyecto a partir de la plantilla de Arquitectura Limpia.
    ///
    /// Genera las cuatro capas canónicas (dominio, casos de uso, infraestructura y
    /// presentación) con sus subtemas habituales, las rutas de código sugeridas y dos
    /// conexiones cruzadas que ilustran la regla de dependencia hacia el interior.
    ///
    /// # Parámetros
    /// - `title`: titulo del proyecto.
    ///
    /// # Devuelve
    /// El proyecto con la disposición automática ya aplicada.
    pub fn nueva_plantilla_arquitectura_limpia(idioma: crate::textos::Idioma) -> Self {
        use crate::textos::Texto;
        let mut proyecto = Self::nuevo_vacio(Texto::PlantillaLimpiaTituloDefecto.en(idioma));
        let root = proyecto.root_id;

        let domain = proyecto.anadir_hijo(root, Texto::PlantillaLimpiaCapaDominio.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&domain) {
            n.notes = Texto::PlantillaLimpiaNotaDominio.en(idioma).into();
            n.tags = vec!["domain".into(), "clean_arch".into()];
            n.file_path = Some("src/domain/".into());
        }
        let _d_ent = proyecto.anadir_hijo(domain, Texto::PlantillaLimpiaEntidades.en(idioma));
        let _d_val = proyecto.anadir_hijo(domain, Texto::PlantillaLimpiaReglasNegocio.en(idioma));

        let app = proyecto.anadir_hijo(root, Texto::PlantillaLimpiaCapaAplicacion.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&app) {
            n.notes = Texto::PlantillaLimpiaNotaAplicacion.en(idioma).into();
            n.tags = vec!["use_cases".into()];
            n.file_path = Some("src/application/".into());
        }
        let app_uc = proyecto.anadir_hijo(app, Texto::PlantillaLimpiaServiciosApp.en(idioma));
        let app_ports = proyecto.anadir_hijo(app, Texto::PlantillaLimpiaPuertosApp.en(idioma));

        let infra =
            proyecto.anadir_hijo(root, Texto::PlantillaLimpiaCapaInfraestructura.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&infra) {
            n.notes = Texto::PlantillaLimpiaNotaInfraestructura.en(idioma).into();
            n.tags = vec!["infra".into(), "database".into()];
            n.file_path = Some("src/infrastructure/".into());
        }
        let infra_db =
            proyecto.anadir_hijo(infra, Texto::PlantillaLimpiaRepositoriosSql.en(idioma));
        let _infra_ext =
            proyecto.anadir_hijo(infra, Texto::PlantillaLimpiaClientesExternos.en(idioma));

        let pres = proyecto.anadir_hijo(root, Texto::PlantillaLimpiaCapaPresentacion.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&pres) {
            n.notes = Texto::PlantillaLimpiaNotaPresentacion.en(idioma).into();
            n.tags = vec!["api".into(), "http".into()];
            n.file_path = Some("src/presentation/".into());
        }
        let pres_routes = proyecto.anadir_hijo(pres, Texto::PlantillaLimpiaRutasHttp.en(idioma));
        let _pres_mid = proyecto.anadir_hijo(pres, Texto::PlantillaLimpiaMiddlewares.en(idioma));

        proyecto.anadir_conexion_cruzada(
            infra_db,
            app_ports,
            Texto::PlantillaLimpiaRelacionRepo.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            pres_routes,
            app_uc,
            Texto::PlantillaLimpiaRelacionCasoUso.en(idioma),
            TipoRelacion::Dependencia,
        );

        crate::layout::aplicar_disposicion_automatica(&mut proyecto);
        proyecto
    }

    /// Crea un mapa de partida con la arquitectura habitual de una aplicación completa.
    ///
    /// Es una de las plantillas del menú «Archivo»: no describe ningún proyecto real, sino el
    /// esqueleto —frontend, backend, base de datos, despliegue— del que suele partir quien
    /// empieza de cero. El usuario lo renombra y lo poda.
    ///
    /// Todos los títulos salen del catálogo de [`crate::textos`], así que la plantilla nace en
    /// el idioma del usuario y no en castellano traducido después.
    ///
    /// # Parámetros
    /// - `idioma`: el que tenga elegido el usuario cuando pide la plantilla.
    ///
    /// # Devuelve
    /// Un proyecto nuevo, ya dispuesto, listo para editar.
    pub fn nueva_plantilla_fullstack(idioma: crate::textos::Idioma) -> Self {
        use crate::textos::Texto;
        let mut proyecto = Self::nuevo_vacio(Texto::PlantillaFullstackTituloDefecto.en(idioma));
        let root = proyecto.root_id;

        let front = proyecto.anadir_hijo(root, Texto::PlantillaFullstackFrontend.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&front) {
            n.tags = vec!["frontend".into(), "ui".into()];
            n.file_path = Some("frontend/src/".into());
        }
        proyecto.anadir_hijo(front, Texto::PlantillaFullstackComponentesUi.en(idioma));
        proyecto.anadir_hijo(front, Texto::PlantillaFullstackEstadoCache.en(idioma));
        let front_api = proyecto.anadir_hijo(front, Texto::PlantillaFullstackClienteApi.en(idioma));

        let back = proyecto.anadir_hijo(root, Texto::PlantillaFullstackBackend.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&back) {
            n.tags = vec!["backend".into(), "rust".into()];
            n.file_path = Some("backend/src/".into());
        }
        let back_routes = proyecto.anadir_hijo(back, Texto::PlantillaFullstackEndpoints.en(idioma));
        proyecto.anadir_hijo(back, Texto::PlantillaFullstackAutenticacion.en(idioma));

        let db = proyecto.anadir_hijo(root, Texto::PlantillaFullstackBaseDatos.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&db) {
            n.tags = vec!["database".into(), "sql".into()];
            n.file_path = Some("migrations/".into());
        }
        let db_tables =
            proyecto.anadir_hijo(db, Texto::PlantillaFullstackEsquemasTablas.en(idioma));
        proyecto.anadir_hijo(db, Texto::PlantillaFullstackMigraciones.en(idioma));

        let devops = proyecto.anadir_hijo(root, Texto::PlantillaFullstackDevOps.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&devops) {
            n.tags = vec!["docker".into(), "ci_cd".into()];
            n.file_path = Some(".github/workflows/".into());
        }
        proyecto.anadir_hijo(devops, Texto::PlantillaFullstackDockerfile.en(idioma));
        proyecto.anadir_hijo(devops, Texto::PlantillaFullstackCiCd.en(idioma));

        proyecto.anadir_conexion_cruzada(
            front_api,
            back_routes,
            Texto::PlantillaFullstackRelacionEndpoints.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            back_routes,
            db_tables,
            Texto::PlantillaFullstackRelacionDb.en(idioma),
            TipoRelacion::Dependencia,
        );

        crate::layout::aplicar_disposicion_automatica(&mut proyecto);
        proyecto
    }

    /// Crea el ejemplo técnico: un videojuego pequeño, con sus dudas abiertas.
    ///
    /// Sirve para enseñar de qué va MMCelt sin pedirle al usuario que invente nada. Es un
    /// proyecto de programación reconocible —lógica, interfaz, datos y pruebas— y lleva a
    /// propósito nodos marcados como duda y notas escritas, para que se vea qué se le puede
    /// contar a una IA y cómo se marca lo que aún no está decidido.
    ///
    /// # Parámetros
    /// - `idioma`: el del usuario. El ejemplo entero está traducido a los seis.
    ///
    /// # Devuelve
    /// El mapa de ejemplo, con su visión, sus metas y sus conexiones cruzadas.
    pub fn nuevo_ejemplo_juego_ahorcado(idioma: crate::textos::Idioma) -> Self {
        use crate::textos::Texto;
        let mut proyecto = Self::nuevo_vacio(Texto::EjemploAhorcadoRaizTitulo.en(idioma));
        proyecto.author = Texto::EjemploAhorcadoAutor.en(idioma).to_string();
        proyecto.creator_vision = Texto::EjemploAhorcadoVision.en(idioma).to_string();
        proyecto.project_goals = Texto::EjemploAhorcadoMetas.en(idioma).to_string();
        proyecto.target_audience_or_context = Texto::EjemploAhorcadoContexto.en(idioma).to_string();

        let raiz = proyecto.root_id;
        if let Some(n) = proyecto.nodes.get_mut(&raiz) {
            n.title = Texto::EjemploAhorcadoRaizTitulo.en(idioma).to_string();
            n.notes = Texto::EjemploAhorcadoRaizNotas.en(idioma).to_string();
        }

        let (logica, fin) = rama_de_la_logica_del_ahorcado(&mut proyecto, raiz, idioma);
        let dibujo = rama_de_la_interfaz_del_ahorcado(&mut proyecto, raiz, idioma);
        let (origen, dificultad) = rama_de_las_palabras_del_ahorcado(&mut proyecto, raiz, idioma);
        rama_de_las_estadisticas_del_ahorcado(&mut proyecto, raiz, idioma);
        let pruebas = rama_de_las_pruebas_del_ahorcado(&mut proyecto, raiz, idioma);

        // Conexiones cruzadas: dependencias que la jerarquía no puede expresar. Son la parte
        // del ejemplo que enseña para qué sirven, así que se trazan aquí, a la vista, y no
        // dentro de ninguna rama: unen nodos de ramas distintas, que es justamente su gracia.
        proyecto.anadir_conexion_cruzada(
            pruebas,
            logica,
            Texto::EjemploAhorcadoRelacionLogica.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            dibujo,
            fin,
            Texto::EjemploAhorcadoRelacionDibujos.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            dificultad,
            origen,
            Texto::EjemploAhorcadoRelacionDificultad.en(idioma),
            TipoRelacion::Sinergia,
        );

        crate::layout::aplicar_disposicion_automatica(&mut proyecto);
        proyecto
    }

    /// Crea el ejemplo no técnico: un negocio de comida a domicilio.
    ///
    /// Existe porque el otro ejemplo es de programación, y MMCelt no es solo para programar.
    /// Este recorre producto, permisos, costes y clientes, e incluye decisiones **ya
    /// descartadas**, que es la parte que más cuesta explicar: un mapa sirve tanto para
    /// recoger por dónde no se va a ir como por dónde sí.
    ///
    /// # Parámetros
    /// - `idioma`: el del usuario. El ejemplo entero está traducido a los seis.
    ///
    /// # Devuelve
    /// El mapa de ejemplo, con su visión, sus metas y sus conexiones cruzadas.
    pub fn nuevo_ejemplo_negocio_comida(idioma: crate::textos::Idioma) -> Self {
        use crate::textos::Texto;
        let mut proyecto = Self::nuevo_vacio(Texto::EjemploNegocioRaizTitulo.en(idioma));
        proyecto.author = Texto::EjemploAhorcadoAutor.en(idioma).to_string();
        proyecto.creator_vision = Texto::EjemploNegocioVision.en(idioma).to_string();
        proyecto.project_goals = Texto::EjemploNegocioMetas.en(idioma).to_string();
        proyecto.target_audience_or_context = Texto::EjemploNegocioContexto.en(idioma).to_string();

        let raiz = proyecto.root_id;
        if let Some(n) = proyecto.nodes.get_mut(&raiz) {
            n.title = Texto::EjemploNegocioRaizTitulo.en(idioma).to_string();
            n.notes = Texto::EjemploNegocioRaizNotas.en(idioma).to_string();
        }

        let raciones = rama_del_producto_del_negocio(&mut proyecto, raiz, idioma);
        let cocina = rama_legal_del_negocio(&mut proyecto, raiz, idioma);
        let (coste, precio) = rama_de_las_finanzas_del_negocio(&mut proyecto, raiz, idioma);
        let (boca, pedidos, plataformas) =
            rama_de_los_clientes_del_negocio(&mut proyecto, raiz, idioma);
        rama_de_las_operaciones_del_negocio(&mut proyecto, raiz, idioma);

        // Conexiones cruzadas
        proyecto.anadir_conexion_cruzada(
            cocina,
            coste,
            Texto::EjemploNegocioRelacionCocinaCoste.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            raciones,
            precio,
            Texto::EjemploNegocioRelacionRacionPrecio.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            precio,
            coste,
            Texto::EjemploNegocioRelacionPrecioCoste.en(idioma),
            TipoRelacion::Dependencia,
        );
        proyecto.anadir_conexion_cruzada(
            plataformas,
            precio,
            Texto::EjemploNegocioRelacionPlataformaMargen.en(idioma),
            TipoRelacion::Bloquea,
        );
        proyecto.anadir_conexion_cruzada(
            pedidos,
            boca,
            Texto::EjemploNegocioRelacionPedidosCanal.en(idioma),
            TipoRelacion::Sinergia,
        );

        crate::layout::aplicar_disposicion_automatica(&mut proyecto);
        proyecto
    }

    /// Crea el mapa mental inicial de bienvenida en el idioma indicado.
    pub fn nuevo_ejemplo_inicial(idioma: crate::textos::Idioma) -> Self {
        use crate::textos::Texto;
        let mut proyecto = Self::nuevo_vacio(Texto::MapaInicialNombreProyecto.en(idioma));
        proyecto.title = Texto::MapaInicialTitulo.en(idioma).to_string();

        let root_id = proyecto.root_id;
        if let Some(root) = proyecto.nodes.get_mut(&root_id) {
            root.title = Texto::MapaInicialIdeaCentral.en(idioma).to_string();
            root.notes = Texto::MapaInicialNotaIdeaCentral.en(idioma).to_string();
        }

        let p1 = proyecto.anadir_hijo(root_id, Texto::MapaInicialPilarArquitectura.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&p1) {
            n.tags = vec![Texto::MapaInicialTagTech.en(idioma).into(), "rust".into()];
            n.notes = Texto::MapaInicialNotaArquitectura.en(idioma).into();
        }

        let p2 = proyecto.anadir_hijo(root_id, Texto::MapaInicialPilarObjetivos.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&p2) {
            n.tags = vec![Texto::MapaInicialTagObjetivo.en(idioma).into()];
        }

        let p3 = proyecto.anadir_hijo(root_id, Texto::MapaInicialPilarDudas.en(idioma));
        if let Some(n) = proyecto.nodes.get_mut(&p3) {
            n.status = EstadoNodo::DudaBloqueo;
            n.role = RolNodo::HipotesisDuda;
            n.notes = Texto::MapaInicialNotaDudas.en(idioma).into();
        }

        crate::layout::aplicar_disposicion_automatica(&mut proyecto);
        proyecto
    }
}

/// La rama de la lógica del juego, en el ejemplo del ahorcado.
///
/// Es el núcleo del proyecto y por eso va marcada con prioridad alta y en progreso: enseña
/// cómo se ve una rama en la que ya se está trabajando.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// Los identificadores de la rama y del nodo «fin de partida», que las conexiones cruzadas
/// necesitan después.
fn rama_de_la_logica_del_ahorcado(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> (Uuid, Uuid) {
    use crate::textos::Texto;

    let logica = proyecto.anadir_hijo(raiz, Texto::EjemploAhorcadoLogica.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&logica) {
        n.notes = Texto::EjemploAhorcadoLogicaNotas.en(idioma).to_string();
        n.tags = vec!["nucleo".into()];
        n.status = EstadoNodo::EnProgreso;
        n.priority = PrioridadNodo::Alta;
    }

    let estado = proyecto.anadir_hijo(logica, Texto::EjemploAhorcadoEstado.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&estado) {
        n.notes = Texto::EjemploAhorcadoEstadoNotas.en(idioma).to_string();
        n.status = EstadoNodo::Completado;
    }

    let intento = proyecto.anadir_hijo(logica, Texto::EjemploAhorcadoIntento.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&intento) {
        n.notes = Texto::EjemploAhorcadoIntentoNotas.en(idioma).to_string();
        n.status = EstadoNodo::EnProgreso;
        n.role = RolNodo::AccionTarea;
    }

    let fin = proyecto.anadir_hijo(logica, Texto::EjemploAhorcadoFin.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&fin) {
        n.notes = Texto::EjemploAhorcadoFinNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::DudaBloqueo;
        n.priority = PrioridadNodo::Media;
    }

    (logica, fin)
}

/// La rama de la interfaz, en el ejemplo del ahorcado.
///
/// Lleva a propósito un nodo **descartado** —la versión gráfica— porque un mapa sirve tanto
/// para recoger por dónde no se va a ir como por dónde sí, y eso es difícil de explicar sin
/// enseñarlo.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// El identificador del nodo del dibujo, que se enlaza después con el fin de partida.
fn rama_de_la_interfaz_del_ahorcado(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> Uuid {
    use crate::textos::Texto;

    let interfaz = proyecto.anadir_hijo(raiz, Texto::EjemploAhorcadoInterfaz.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&interfaz) {
        n.notes = Texto::EjemploAhorcadoInterfazNotas.en(idioma).to_string();
        n.tags = vec!["ui".into()];
    }

    let dibujo = proyecto.anadir_hijo(interfaz, Texto::EjemploAhorcadoDibujo.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&dibujo) {
        n.notes = Texto::EjemploAhorcadoDibujoNotas.en(idioma).to_string();
        n.status = EstadoNodo::Idea;
    }

    let entrada = proyecto.anadir_hijo(interfaz, Texto::EjemploAhorcadoEntrada.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&entrada) {
        n.notes = Texto::EjemploAhorcadoEntradaNotas.en(idioma).to_string();
        n.status = EstadoNodo::Idea;
    }

    let grafica = proyecto.anadir_hijo(interfaz, Texto::EjemploAhorcadoGrafica.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&grafica) {
        n.notes = Texto::EjemploAhorcadoGraficaNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::Descartado;
    }

    dibujo
}

/// La rama de las palabras del juego, en el ejemplo del ahorcado.
///
/// Es donde vive la duda abierta del ejemplo: de dónde salen las palabras. Va marcada como
/// bloqueo y con prioridad alta porque es lo que enseña cómo se le señala a una IA que hay
/// algo sin decidir.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// Los identificadores del origen de las palabras y de la dificultad, que se enlazan entre sí.
fn rama_de_las_palabras_del_ahorcado(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> (Uuid, Uuid) {
    use crate::textos::Texto;

    let palabras = proyecto.anadir_hijo(raiz, Texto::EjemploAhorcadoPalabras.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&palabras) {
        n.notes = Texto::EjemploAhorcadoPalabrasNotas.en(idioma).to_string();
        n.tags = vec!["datos".into()];
    }

    let origen = proyecto.anadir_hijo(palabras, Texto::EjemploAhorcadoOrigen.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&origen) {
        n.notes = Texto::EjemploAhorcadoOrigenNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::DudaBloqueo;
        n.priority = PrioridadNodo::Alta;
    }

    let dificultad = proyecto.anadir_hijo(palabras, Texto::EjemploAhorcadoDificultad.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&dificultad) {
        n.notes = Texto::EjemploAhorcadoDificultadNotas.en(idioma).to_string();
        n.status = EstadoNodo::Idea;
        n.priority = PrioridadNodo::Baja;
    }

    (origen, dificultad)
}

/// La rama de las estadísticas, en el ejemplo del ahorcado.
///
/// Es la única que no participa en ninguna conexión cruzada, y por eso no devuelve nada: está
/// para que el mapa tenga también una rama que es solo una idea de futuro.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
fn rama_de_las_estadisticas_del_ahorcado(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) {
    use crate::textos::Texto;

    let estadisticas = proyecto.anadir_hijo(raiz, Texto::EjemploAhorcadoEstadisticas.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&estadisticas) {
        n.notes = Texto::EjemploAhorcadoEstadisticasNotas
            .en(idioma)
            .to_string();
        n.tags = vec!["datos".into()];
        n.status = EstadoNodo::Idea;
        n.priority = PrioridadNodo::Baja;
    }
}

/// La rama de las pruebas, en el ejemplo del ahorcado.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// El identificador de la rama, que se enlaza con la lógica del juego.
fn rama_de_las_pruebas_del_ahorcado(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> Uuid {
    use crate::textos::Texto;

    let pruebas = proyecto.anadir_hijo(raiz, Texto::EjemploAhorcadoPruebas.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&pruebas) {
        n.notes = Texto::EjemploAhorcadoPruebasNotas.en(idioma).to_string();
        n.tags = vec!["calidad".into()];
        n.status = EstadoNodo::EnProgreso;
        n.priority = PrioridadNodo::Alta;
    }

    pruebas
}

/// La rama del producto, en el ejemplo del negocio de comida.
///
/// Qué se vende: la carta, el tamaño de las raciones y los envases. El tamaño de la
/// ración es el que después condiciona el precio, y por eso se devuelve.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// Los identificadores que necesitan después las conexiones cruzadas.
fn rama_del_producto_del_negocio(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> Uuid {
    use crate::textos::Texto;

    let producto = proyecto.anadir_hijo(raiz, Texto::EjemploNegocioProducto.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&producto) {
        n.notes = Texto::EjemploNegocioProductoNotas.en(idioma).to_string();
        n.tags = vec!["producto".into()];
        n.status = EstadoNodo::EnProgreso;
        n.priority = PrioridadNodo::Alta;
    }

    let carta = proyecto.anadir_hijo(producto, Texto::EjemploNegocioCarta.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&carta) {
        n.notes = Texto::EjemploNegocioCartaNotas.en(idioma).to_string();
        n.status = EstadoNodo::Completado;
    }

    let raciones = proyecto.anadir_hijo(producto, Texto::EjemploNegocioRaciones.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&raciones) {
        n.notes = Texto::EjemploNegocioRacionesNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::DudaBloqueo;
        n.priority = PrioridadNodo::Alta;
    }

    let envase = proyecto.anadir_hijo(producto, Texto::EjemploNegocioEnvases.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&envase) {
        n.notes = Texto::EjemploNegocioEnvasesNotas.en(idioma).to_string();
        n.status = EstadoNodo::Investigando;
    }
    raciones
}

/// La rama de los permisos, en el ejemplo del negocio de comida.
///
/// Es la que enseña que un proyecto no técnico tiene obligaciones que no se pueden
/// saltar, y que algunas cuestan dinero: de ahí el enlace con el coste.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// Los identificadores que necesitan después las conexiones cruzadas.
fn rama_legal_del_negocio(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> Uuid {
    use crate::textos::Texto;

    let legal = proyecto.anadir_hijo(raiz, Texto::EjemploNegocioLegal.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&legal) {
        n.notes = Texto::EjemploNegocioLegalNotas.en(idioma).to_string();
        n.tags = vec!["legal".into()];
        n.status = EstadoNodo::EnProgreso;
        n.priority = PrioridadNodo::Alta;
    }

    let cocina = proyecto.anadir_hijo(legal, Texto::EjemploNegocioCocinaPropia.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&cocina) {
        n.notes = Texto::EjemploNegocioCocinaPropiaNotas
            .en(idioma)
            .to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::DudaBloqueo;
        n.priority = PrioridadNodo::Alta;
    }

    let carne = proyecto.anadir_hijo(legal, Texto::EjemploNegocioManipulador.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&carne) {
        n.notes = Texto::EjemploNegocioManipuladorNotas.en(idioma).to_string();
        n.role = RolNodo::AccionTarea;
        n.status = EstadoNodo::Idea;
        n.priority = PrioridadNodo::Alta;
    }

    let autonomo = proyecto.anadir_hijo(legal, Texto::EjemploNegocioAutonomo.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&autonomo) {
        n.notes = Texto::EjemploNegocioAutonomoNotas.en(idioma).to_string();
        n.role = RolNodo::AccionTarea;
        n.status = EstadoNodo::Idea;
    }
    cocina
}

/// La rama de los números, en el ejemplo del negocio de comida.
///
/// Coste por ración, precio de venta e inversión inicial. Los dos primeros participan en
/// tres de las cinco conexiones cruzadas del ejemplo: es donde todo lo demás acaba.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// Los identificadores que necesitan después las conexiones cruzadas.
fn rama_de_las_finanzas_del_negocio(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> (Uuid, Uuid) {
    use crate::textos::Texto;

    let finanzas = proyecto.anadir_hijo(raiz, Texto::EjemploNegocioFinanzas.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&finanzas) {
        n.notes = Texto::EjemploNegocioFinanzasNotas.en(idioma).to_string();
        n.tags = vec!["finanzas".into()];
        n.status = EstadoNodo::EnProgreso;
        n.priority = PrioridadNodo::Alta;
    }

    let coste = proyecto.anadir_hijo(finanzas, Texto::EjemploNegocioCosteRacion.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&coste) {
        n.notes = Texto::EjemploNegocioCosteRacionNotas.en(idioma).to_string();
        n.status = EstadoNodo::Investigando;
    }

    let precio = proyecto.anadir_hijo(finanzas, Texto::EjemploNegocioPrecioVenta.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&precio) {
        n.notes = Texto::EjemploNegocioPrecioVentaNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::Idea;
    }

    let inversion =
        proyecto.anadir_hijo(finanzas, Texto::EjemploNegocioInversionInicial.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&inversion) {
        n.notes = Texto::EjemploNegocioInversionInicialNotas
            .en(idioma)
            .to_string();
        n.status = EstadoNodo::Idea;
    }
    (coste, precio)
}

/// La rama de los clientes, en el ejemplo del negocio de comida.
///
/// Por dónde llegan los pedidos y por dónde se corre la voz. Incluye la decisión sobre
/// las plataformas de reparto, que es la que se enfrenta al margen.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
///
/// # Devuelve
/// Los identificadores que necesitan después las conexiones cruzadas.
fn rama_de_los_clientes_del_negocio(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) -> (Uuid, Uuid, Uuid) {
    use crate::textos::Texto;

    let clientes = proyecto.anadir_hijo(raiz, Texto::EjemploNegocioClientes.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&clientes) {
        n.notes = Texto::EjemploNegocioClientesNotas.en(idioma).to_string();
        n.tags = vec!["marketing".into()];
        n.status = EstadoNodo::Idea;
    }

    let boca = proyecto.anadir_hijo(clientes, Texto::EjemploNegocioCanalClientes.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&boca) {
        n.notes = Texto::EjemploNegocioCanalClientesNotas
            .en(idioma)
            .to_string();
        n.role = RolNodo::AccionTarea;
        n.status = EstadoNodo::Idea;
        n.priority = PrioridadNodo::Alta;
    }

    let pedidos = proyecto.anadir_hijo(clientes, Texto::EjemploNegocioPedidos.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&pedidos) {
        n.notes = Texto::EjemploNegocioPedidosNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::Idea;
    }

    let plataformas = proyecto.anadir_hijo(clientes, Texto::EjemploNegocioPlataformas.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&plataformas) {
        n.notes = Texto::EjemploNegocioPlataformasNotas.en(idioma).to_string();
        n.role = RolNodo::HipotesisDuda;
        n.status = EstadoNodo::Descartado;
    }
    (boca, pedidos, plataformas)
}

/// La rama del día a día, en el ejemplo del negocio de comida.
///
/// No participa en ninguna conexión cruzada: está para que el mapa tenga también la parte
/// que se ejecuta y no se discute.
///
/// # Parámetros
/// - `proyecto`: el mapa en construcción.
/// - `raiz`: el nodo del que cuelga la rama.
/// - `idioma`: el del usuario.
fn rama_de_las_operaciones_del_negocio(
    proyecto: &mut Proyecto,
    raiz: Uuid,
    idioma: crate::textos::Idioma,
) {
    use crate::textos::Texto;

    let operaciones = proyecto.anadir_hijo(raiz, Texto::EjemploNegocioOperaciones.en(idioma));
    if let Some(n) = proyecto.nodes.get_mut(&operaciones) {
        n.notes = Texto::EjemploNegocioOperacionesNotas.en(idioma).to_string();
        n.tags = vec!["operaciones".into()];
        n.status = EstadoNodo::Idea;
    }
}
