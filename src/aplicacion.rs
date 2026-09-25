//! # Estado y ciclo de vida de la aplicación
//!
//! Posee las invariantes que coordinan el mapa, la persistencia, los agentes y la presentación.
//! Vive junto a `ui`, no encima de ella, para que dibujar una pantalla no conceda acceso implícito
//! a los campos privados del estado raíz.

use crate::autoguardado::{ControlAutoguardado, Recuperacion};
use crate::error::AppError;
use crate::model::{FiltroDeBusqueda, ModoDisposicion, Proyecto, TipoRelacion};
use crate::preferencias::Preferencias;
use crate::textos::Texto;
use crate::theme::{AppThemeMode, ThemeConfig};
use crate::ui::{
    ai_modal, canvas, conexiones_modal, dialogos, estado_agentes, estado_persistencia, help_system,
    proyecto_ia_modal, sesion_agente_modal, sidebar, toolbar,
};
use help_system::TemaDeAyuda;
use std::path::PathBuf;
use uuid::Uuid;

#[cfg(test)]
#[path = "audit_checks.rs"]
mod audit_checks;

#[path = "servicios_aplicacion.rs"]
mod servicios_aplicacion;
pub(crate) use servicios_aplicacion::ServiciosAplicacion;

#[path = "persistencia_mapa_comando.rs"]
mod persistencia_mapa_comando;

/// Qué ventanas de la aplicación están abiertas ahora mismo.
///
/// # Por qué viven juntas
///
/// `AplicacionMapaMental` llegó a declarar cuarenta y siete campos en una sola lista, donde
/// el mapa del usuario, la cámara del lienzo, el autoguardado y estas once banderas estaban
/// al mismo nivel. Lo único que estas once tienen en común es que dicen qué hay abierto, así
/// que se agrupan aquí y el estado de la aplicación deja de nombrarlas una a una.
///
/// # Por qué son banderas y no un enum
///
/// Porque varias pueden estar abiertas a la vez: el panel de ayuda detallada convive con
/// cualquier modal, y las galletas de ayuda con todo. Un enum de «ventana activa» obligaría a
/// inventar variantes para cada combinación.
///
/// # Por qué conservan el prefijo `modal_`
///
/// No es redundancia con el nombre del campo que las contiene. Tres pruebas de
/// `audit_checks.rs` analizan el texto del fuente buscando estos nombres —entre otras, la que
/// impide que un modal abierto deje pasar la tecla Supr hasta el mapa de detrás—, y no hay
/// forma de preguntar por los campos declarados en tiempo de ejecución sin macros. Un
/// renombrado que las dejara sin encontrar nada no las rompería: las dejaría pasando siempre,
/// sin comprobar nada, que es peor.
/// Información contextual para el diálogo modal de error al abrir proyecto e instrucciones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvisoErrorProyectoIa {
    /// Ruta del proyecto que causó el fallo.
    pub ruta: std::path::PathBuf,
    /// Motivo detallado del error.
    pub motivo: crate::error::MotivoCarpetaInvalida,
}

/// Información contextual para el diálogo modal de aviso temprano cuando un mapa se abre en la raíz de una unidad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvisoMapaEnRaiz {
    /// Ruta del mapa mental abierto en la raíz.
    pub ruta_mapa: std::path::PathBuf,
}

pub struct VentanasAbiertas {
    /// Pegar un mapa que ha escrito un modelo de IA.
    pub modal_importar: bool,
    /// El texto maestro que se le entrega a la IA para que trabaje sobre el mapa.
    pub modal_prompt_maestro: bool,
    /// Vista previa del documento Markdown que se va a exportar.
    pub modal_vista_previa: bool,
    /// Las correcciones que el usuario le exige a la IA.
    pub modal_correcciones: bool,
    /// La lista de atajos de teclado.
    pub modal_atajos: bool,
    /// La visión y las metas del proyecto, que acompañan al mapa.
    pub modal_datos_del_proyecto: bool,
    /// Crear una conexión cruzada entre dos nodos.
    pub modal_conexion_cruzada: bool,
    /// Los agentes de IA detectados en el equipo y su estado de conexión.
    pub modal_conexiones: bool,
    /// Qué compilación es esta.
    pub modal_acerca_de: bool,

    /// Los avisos breves de ayuda repartidos por la interfaz.
    ///
    /// Es la única que arranca **encendida**: quien abre MMCelt por primera vez las necesita,
    /// y quien no las quiera las apaga una vez.
    pub galletas_de_ayuda: bool,
    /// El panel lateral de ayuda detallada.
    pub panel_de_ayuda: bool,
}

#[cfg(test)]
mod pruebas_del_espacio_de_trabajo {
    /// Abrir un mapa debe actualizar el espacio implícito que usarán las instrucciones y MCP.
    ///
    /// Reproduce P2-A y defecto del 2026-09-13: antes solo se recalculaba al abrir la ventana
    /// de conexiones o si la preferencia estaba vacía. Si el usuario había fijado otra carpeta
    /// en un proyecto anterior, abrir un mapa de un nuevo proyecto quedaba secuestrado por la
    /// preferencia desfasada. Ahora, si el mapa queda fuera de la carpeta preferida, el mapa manda
    /// y actualiza tanto el espacio de agentes como la preferencia persistida.
    #[test]
    fn abrir_un_mapa_actualiza_el_espacio_implicito_del_proyecto() {
        let (mut app, _ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
        let carpeta =
            std::env::temp_dir().join(format!("mmcelt_espacio_del_mapa_{}", uuid::Uuid::new_v4()));
        let mapa = carpeta.join("plan.mmcelt");
        // Caso 1: sin preferencia previa
        app.presentacion.preferencias.espacio_trabajo_ia.clear();

        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nuevo_vacio("Proyecto activo"),
            Some(mapa.clone()),
            "Mapa abierto",
        );

        assert_eq!(
            app.agentes.espacio_de_trabajo, carpeta,
            "las instrucciones deben usar el proyecto del mapa sin exigir abrir antes Conexiones"
        );
        assert_eq!(
            app.presentacion.preferencias.espacio_trabajo_ia,
            carpeta.display().to_string(),
            "la preferencia debe persistir la carpeta adoptada del mapa"
        );

        // Caso 2: con una preferencia ajena previa fijada (reproducción exacta de un fallo real reportado)
        let ajena = std::env::temp_dir().join(format!(
            "mmcelt_preferencia_antigua_{}",
            uuid::Uuid::new_v4()
        ));
        app.presentacion.preferencias.espacio_trabajo_ia = ajena.display().to_string();
        app.agentes.espacio_de_trabajo = ajena.clone();

        app.sustituir_el_mapa_abierto(
            crate::model::Proyecto::nuevo_vacio("Proyecto nuevo"),
            Some(mapa.clone()),
            "Mapa de otro proyecto",
        );

        assert_eq!(
            app.agentes.espacio_de_trabajo, carpeta,
            "abrir un mapa en otra carpeta debe actualizar el espacio y no quedar secuestrado por la anterior"
        );
        assert_eq!(
            app.presentacion.preferencias.espacio_trabajo_ia,
            carpeta.display().to_string(),
            "la preferencia persistida debe actualizarse con la nueva carpeta"
        );
    }
}

impl Default for VentanasAbiertas {
    /// Todo cerrado salvo las galletas de ayuda.
    ///
    /// No se deriva porque `galletas_de_ayuda` arranca en `true`, y un `derive` las apagaría
    /// sin que nadie lo notara hasta que un usuario nuevo se encontrara la interfaz muda.
    fn default() -> Self {
        Self {
            modal_importar: false,
            modal_prompt_maestro: false,
            modal_vista_previa: false,
            modal_correcciones: false,
            modal_atajos: false,
            modal_datos_del_proyecto: false,
            modal_conexion_cruzada: false,
            modal_conexiones: false,
            modal_acerca_de: false,
            galletas_de_ayuda: true,
            panel_de_ayuda: false,
        }
    }
}

/// Cómo se está mirando el mapa ahora mismo: la cámara del lienzo y el arrastre en curso.
///
/// # Por qué viven juntos
///
/// Los cinco cambian a la vez y por el mismo motivo —el usuario mueve la vista o arrastra un
/// nodo—, y ninguno dice nada sobre el mapa en sí: un mapa guardado y vuelto a abrir se ve
/// igual aunque estos cinco valgan otra cosa. Sueltos en `AplicacionMapaMental` quedaban
/// mezclados con el proyecto del usuario y con el autoguardado, que es justo lo contrario.
///
/// # Por qué el arrastre está aquí y no en el modelo
///
/// Porque un nodo a medio arrastrar no es un cambio del mapa todavía: es un gesto en curso.
/// Solo cuando el usuario suelta se escribe la posición en el nodo.
pub struct VistaDelLienzo {
    /// Desplazamiento de cámara (pan) en el plano X/Y.
    pub desplazamiento: egui::Vec2,
    /// Nivel de escala / zoom actual.
    pub zoom: f32,
    /// Tamaño del área de pantalla donde se dibuja el lienzo, del último fotograma.
    ///
    /// Lo anota el propio lienzo al dibujarse. Existe porque «Centrar vista» necesita saber
    /// cuánto sitio hay para decidir el zoom, y se ejecuta desde el menú o desde un atajo,
    /// fuera del dibujado, donde ese dato no está disponible. Arranca a cero: hasta el
    /// primer fotograma se conserva el tamaño natural.
    pub tamano_visible: egui::Vec2,
    /// Nodo que está siendo arrastrado manualmente en el lienzo.
    pub nodo_arrastrado: Option<Uuid>,
    /// Distancia entre el punto por el que se agarró el nodo y su centro.
    ///
    /// Sin guardarla, arrastrar colocaba el **centro** del nodo justo bajo el cursor, así que
    /// agarrarlo por un borde lo hacía saltar hasta media anchura de golpe. Como el arrastre
    /// empieza en el mismo instante en que se pulsa, ese salto ocurría también al dar un
    /// simple clic para seleccionar, y en el árbol horizontal el nodo se quedaba descolocado
    /// hasta la siguiente reorganización.
    pub agarre_del_arrastre: egui::Vec2,
}

impl Default for VistaDelLienzo {
    /// Vista centrada, a escala natural y sin nada arrastrándose.
    ///
    /// No se deriva: `zoom` tiene que arrancar en `1.0`, y el `Default` de `f32` es `0.0`.
    /// Un cero ahí no se ve como un mapa pequeño, se ve como un lienzo en blanco, y además
    /// divide por cero en las conversiones de coordenadas.
    fn default() -> Self {
        Self {
            desplazamiento: egui::Vec2::ZERO,
            zoom: 1.0,
            tamano_visible: egui::Vec2::ZERO,
            nodo_arrastrado: None,
            agarre_del_arrastre: egui::Vec2::ZERO,
        }
    }
}

/// El título que el usuario está escribiendo dentro del lienzo, si es que hay alguno.
///
/// # Por qué es un estado y no una variable local
///
/// `egui` dibuja en modo inmediato: el lienzo se construye entero en cada fotograma y
/// cualquier variable local muere al terminarlo. Lo que el usuario lleva tecleado tiene que
/// sobrevivir de un fotograma al siguiente, así que vive aquí.
///
/// # Por qué son tres campos y no uno
///
/// Porque el texto en curso no basta: hay que saber **sobre qué nodo** se está escribiendo
/// —para no volcarlo en otro— y si el editor todavía tiene que pedir el foco del teclado.
#[derive(Default)]
pub struct EdicionDelTitulo {
    /// ID del nodo en edición de texto activa dentro del lienzo.
    pub nodo: Option<Uuid>,

    /// Si el editor del lienzo todavía tiene que reclamar el foco del teclado.
    ///
    /// Se pide **una sola vez**, en el fotograma en que se abre la edición. Antes se pedía
    /// en todos, y eso tenía dos consecuencias feas: `lost_focus()` no volvía a ser cierto
    /// nunca —así que salir de la edición con un clic fuera tiraba lo escrito— y ningún otro
    /// cuadro de texto de la aplicación podía recibir teclado mientras hubiera un título en
    /// edición, porque el lienzo se dibuja después del panel lateral y le quitaba el foco en
    /// el acto. Lo tecleado en el inspector acababa en el título del nodo.
    pub foco_pendiente: bool,

    /// Búfer de texto temporal para la edición interactiva.
    pub texto: String,
}

/// La conexión cruzada que el usuario está creando, mientras la termina de definir.
///
/// Una conexión cruzada une dos nodos que no son padre e hijo, y crearla lleva varios pasos:
/// se elige el nodo de origen en el lienzo, después el de destino, y en el modal se escribe
/// el motivo y se elige el tipo de relación. Hasta que se confirma no existe nada en el mapa:
/// esto es solo lo que lleva reunido.
///
/// Mientras `origen` tenga algo, el lienzo está en modo de enlazado y el siguiente clic sobre
/// un nodo lo toma como destino en lugar de seleccionarlo.
#[derive(Default)]
pub struct ConexionEnCurso {
    /// Primer extremo elegido; mientras existe, el lienzo interpreta el siguiente clic como destino.
    pub origen: Option<Uuid>,

    /// Segundo extremo elegido antes de abrir la confirmación de la relación.
    pub destino: Option<Uuid>,

    /// Explicación que se mostrará sobre la línea de conexión.
    pub motivo: String,

    /// Tipo de relación elegido en el desplegable del modal de conexión cruzada.
    ///
    /// El desplegable no existía: la interfaz creaba siempre `Dependencia`, y las otras
    /// cuatro relaciones solo se podían conseguir importando de una IA o editando el archivo
    /// a mano, pese a estar documentadas y dibujarse en el diagrama de la exportación.
    pub tipo: TipoRelacion,
}

/// Documento activo, selección y memoria de cambios, que evolucionan como una sola unidad.
pub(crate) struct EstadoDelMapa {
    /// Proyecto activo de mapa mental.
    proyecto: Proyecto,
    /// ID del nodo actualmente seleccionado por el usuario.
    nodo_seleccionado: Option<Uuid>,
    /// Estados por los que ha pasado el mapa, para deshacer y rehacer.
    historial: crate::historial::HistorialDeCambios,
}

impl EstadoDelMapa {
    /// Consulta el documento activo sin abrir su sustitución completa a la presentación.
    pub(crate) fn proyecto(&self) -> &Proyecto {
        &self.proyecto
    }

    /// Permite editar el documento mediante las operaciones tipadas de [`Proyecto`].
    pub(crate) fn proyecto_para_editar(&mut self) -> &mut Proyecto {
        &mut self.proyecto
    }

    /// Devuelve el nodo seleccionado actualmente.
    pub(crate) fn nodo_seleccionado(&self) -> Option<Uuid> {
        self.nodo_seleccionado
    }

    /// Cambia la selección solo a un nodo existente, o la limpia.
    pub(crate) fn seleccionar_nodo(&mut self, nodo: Option<Uuid>) {
        self.nodo_seleccionado = nodo.filter(|id| self.proyecto.nodes.contains_key(id));
    }

    /// Indica si existe un estado anterior al que volver.
    pub(crate) fn puede_deshacer(&self) -> bool {
        self.historial.puede_deshacer()
    }

    /// Indica si existe un estado deshecho que se pueda recuperar.
    pub(crate) fn puede_rehacer(&self) -> bool {
        self.historial.puede_rehacer()
    }
}

/// Gestos y geometría transitorios del lienzo, que nunca se serializan como mapa.
pub(crate) struct EstadoDelLienzo {
    /// La conexión cruzada a medio crear. Ver [`ConexionEnCurso`].
    conexion: ConexionEnCurso,
    /// El título en edición dentro del lienzo. Ver [`EdicionDelTitulo`].
    edicion: EdicionDelTitulo,
    /// La cámara del lienzo y el arrastre en curso. Ver [`VistaDelLienzo`].
    vista: VistaDelLienzo,
    /// Celdas de tarjetas reutilizadas mientras el mapa no cambia.
    indice_espacial_del_lienzo: canvas::IndiceEspacialLienzo,
}

impl EstadoDelLienzo {
    /// Consulta la conexión que se está preparando.
    pub(crate) fn conexion(&self) -> &ConexionEnCurso {
        &self.conexion
    }

    /// Continúa la edición de la conexión en curso.
    pub(crate) fn editar_conexion(&mut self) -> &mut ConexionEnCurso {
        &mut self.conexion
    }

    /// Consulta la edición de título que está abierta.
    pub(crate) fn edicion(&self) -> &EdicionDelTitulo {
        &self.edicion
    }

    /// Continúa la edición del título actualmente abierto.
    pub(crate) fn editar_titulo(&mut self) -> &mut EdicionDelTitulo {
        &mut self.edicion
    }

    /// Consulta la cámara y el gesto de arrastre.
    pub(crate) fn vista(&self) -> &VistaDelLienzo {
        &self.vista
    }

    /// Actualiza la cámara o el gesto de arrastre del lienzo.
    pub(crate) fn actualizar_vista(&mut self) -> &mut VistaDelLienzo {
        &mut self.vista
    }

    /// Invalida el índice espacial después de mover una tarjeta.
    pub(crate) fn invalidar_indice_espacial(&mut self) {
        self.indice_espacial_del_lienzo.invalidar();
    }

    /// Presta el índice espacial para reconstruirlo o consultarlo durante el dibujo.
    pub(crate) fn indice_espacial(&mut self) -> &mut canvas::IndiceEspacialLienzo {
        &mut self.indice_espacial_del_lienzo
    }
}

/// Preferencias y búferes propios de la presentación inmediata de `egui`.
pub(crate) struct EstadoDePresentacion {
    /// Paleta aplicada al lienzo y a los controles de `egui`.
    tema: ThemeConfig,
    /// Qué ventanas están abiertas. Ver [`VentanasAbiertas`].
    ventanas: VentanasAbiertas,
    /// Guía extensa que ocupa el panel de ayuda, o ninguna si el panel está cerrado.
    guia_de_ayuda_activa: Option<TemaDeAyuda>,
    /// Borrador pegado en el modal de importación desde IA.
    ///
    /// Vive en la aplicación porque un modal inmediato se reconstruye en cada fotograma.
    texto_para_importar: String,
    /// Lo que el usuario lleva escrito en el campo «Añadir tag» del inspector.
    ///
    /// Vive aquí por lo mismo que los demás: en modo inmediato, el panel se dibuja de cero
    /// en cada fotograma y una variable local se descarta al terminarlo. Como local que era,
    /// la caja aparecía vacía siempre y la comprobación de «hay algo escrito» daba que no,
    /// así que **no se podía añadir ninguna etiqueta desde la interfaz**: las únicas que
    /// existían llegaban importadas, del escáner o de las plantillas.
    etiqueta_en_escritura: String,
    /// Último error del importador que el modal debe conservar visible.
    error_de_importacion: Option<String>,
    /// Documento para la IA ya generado, con la huella del mapa del que salió.
    ///
    /// Las ventanas de vista previa lo rehacían **entero** en cada fotograma —con su
    /// diagrama y su recorrido del árbol— mientras estuvieran abiertas, que es todo el rato
    /// que el usuario dedica a leerlo. Ahora se rehace solo cuando el mapa cambia, y hasta
    /// **averiguar** si ha cambiado se espacia: ver `ai_modal::DocumentoEnPantalla`.
    vista_previa_para_ia: Option<crate::ui::ai_modal::DocumentoEnPantalla>,
    /// Lo mismo para el documento de correcciones.
    vista_previa_de_correcciones: Option<crate::ui::ai_modal::DocumentoEnPantalla>,
    /// Borradores de la ventana unificada de proyecto e instrucciones para agentes.
    editor_proyecto_ia: Option<proyecto_ia_modal::EstadoEditorProyectoIa>,
    /// Diálogo modal visible cuando no se pudo abrir la ventana de proyecto e instrucciones.
    pub aviso_error_proyecto_ia: Option<AvisoErrorProyectoIa>,
    /// Diálogo modal visible cuando un mapa se abre directamente en la raíz de una unidad.
    pub aviso_mapa_en_raiz: Option<AvisoMapaEnRaiz>,
    /// Diálogo modal visible cuando se guarda como a una carpeta distinta a la del proyecto.
    pub aviso_guardar_como_otra_carpeta: Option<crate::guardado_como::AvisoGuardarComoOtraCarpeta>,
    /// Aviso temporal que muestra la barra inferior junto al instante en que se creó.
    mensaje_de_estado: Option<(String, std::time::Instant)>,
    /// Ajustes que se conservan entre sesiones: escala de la interfaz y tema.
    preferencias: Preferencias,
    /// Si la escala guardada ya se ha aplicado al contexto de `egui`.
    ///
    /// No puede aplicarse en el constructor porque el contexto todavía no conoce las
    /// características del monitor: hace falta esperar al primer fotograma.
    escala_aplicada: bool,
    /// Lo que se le está pidiendo ahora mismo al buscador del panel lateral.
    ///
    /// Vive aquí, y no dentro del panel, porque `egui` redibuja la interfaz entera en cada
    /// fotograma: una variable local se perdería entre uno y el siguiente y el campo se
    /// vaciaría solo mientras se teclea.
    ///
    /// Es **un solo valor** y no tres sueltos —texto, estado y prioridad— porque son una sola
    /// petición: separarlos no obligaría a nadie a mantenerlos coherentes.
    filtro_de_busqueda: FiltroDeBusqueda,
}

impl EstadoDePresentacion {
    /// Consulta la paleta activa.
    pub(crate) fn tema(&self) -> &ThemeConfig {
        &self.tema
    }
    /// Permite abrir, cerrar o alternar ventanas como una sola responsabilidad visual.
    pub(crate) fn ventanas(&mut self) -> &mut VentanasAbiertas {
        &mut self.ventanas
    }
    /// Cambia o consulta la guía mostrada en el panel de ayuda.
    pub(crate) fn guia_activa(&mut self) -> &mut Option<TemaDeAyuda> {
        &mut self.guia_de_ayuda_activa
    }
    /// Presta el borrador que el usuario está pegando para importarlo.
    pub(crate) fn texto_para_importar(&mut self) -> &mut String {
        &mut self.texto_para_importar
    }
    /// Presta el texto de la etiqueta que todavía no se ha confirmado.
    pub(crate) fn etiqueta_en_escritura(&mut self) -> &mut String {
        &mut self.etiqueta_en_escritura
    }
    /// Consulta o reemplaza el error visible del importador.
    pub(crate) fn error_de_importacion(&mut self) -> &mut Option<String> {
        &mut self.error_de_importacion
    }
    /// Consulta o renueva la vista previa destinada al agente.
    pub(crate) fn vista_previa_para_ia(
        &mut self,
    ) -> &mut Option<crate::ui::ai_modal::DocumentoEnPantalla> {
        &mut self.vista_previa_para_ia
    }
    /// Consulta o renueva la vista previa de correcciones.
    pub(crate) fn vista_previa_de_correcciones(
        &mut self,
    ) -> &mut Option<crate::ui::ai_modal::DocumentoEnPantalla> {
        &mut self.vista_previa_de_correcciones
    }
    /// Presta el editor de proyecto e instrucciones que debe sobrevivir entre fotogramas.
    pub(crate) fn editor_proyecto_ia(
        &mut self,
    ) -> &mut Option<proyecto_ia_modal::EstadoEditorProyectoIa> {
        &mut self.editor_proyecto_ia
    }
    /// Consulta y actualiza las preferencias persistentes desde sus controles.
    pub(crate) fn preferencias(&mut self) -> &mut Preferencias {
        &mut self.preferencias
    }
    /// Consulta las preferencias persistentes en modo solo lectura.
    pub(crate) fn preferencias_ref(&self) -> &Preferencias {
        &self.preferencias
    }
    /// Presta lo que se le está pidiendo al buscador mientras el usuario lo cambia.
    pub(crate) fn filtro_de_busqueda(&mut self) -> &mut FiltroDeBusqueda {
        &mut self.filtro_de_busqueda
    }
}

/// Estado global de la aplicación de escritorio MMCelt.
pub struct AplicacionMapaMental {
    /// Documento, selección e historial bajo una sola razón de cambio.
    mapa: EstadoDelMapa,
    /// Interacción transitoria con el lienzo.
    lienzo: EstadoDelLienzo,
    /// Búferes y preferencias de presentación.
    presentacion: EstadoDePresentacion,
    /// Servicios elegidos una sola vez en el punto de composición.
    servicios: servicios_aplicacion::ServiciosAplicacion,
    /// Sesión, detección y trabajador del recorrido «Enviar a…».
    agentes: estado_agentes::EstadoAgentes,
    /// Archivo, autoguardado, recuperación y vigilancia del mapa activo.
    persistencia: estado_persistencia::EstadoPersistencia,
}

/// Datos binarios de la fuente china Noto Sans SC recortada para MMCelt.
///
/// Contiene los 902 glifos CJK únicos que usa la interfaz hoy más los 3.755 hanzi de uso corriente
/// (GB2312 nivel 1), permitiendo mostrar chino en la interfaz, en la ayuda y en los nodos del usuario
/// sin alterar el aspecto de los textos en latín o cirílico (Ubuntu-Light).
pub const FUENTE_CJK_BYTES: &[u8] = include_bytes!("../assets/fuentes/NotoSansSC-subconjunto.ttf");

/// Nombre con el que se registra la fuente china dentro de `egui`.
///
/// Es una clave interna del mapa de fuentes, no un texto que vea nadie, pero tiene que ser
/// **el mismo** en el registro y en cada familia donde se añade: si se separan, `egui` guarda
/// la fuente con un nombre y la busca con otro, no encuentra nada y vuelven los cuadrados sin
/// que ningún error avise.
const NOMBRE_FUENTE_CJK: &str = "noto_sans_sc_subconjunto";

/// Instala en `egui` la fuente china como respaldo de las que ya trae.
///
/// Hay que llamarla antes del primer fotograma. `egui` no incluye ningún glifo CJK —su fuente
/// por omisión cubre latín y cirílico—, así que sin esto el chino se dibuja como cuadrados:
/// la traducción está, pero no hay con qué pintarla.
///
/// # Parámetros
/// - `ctx`: contexto de `egui` en el que se instalan las fuentes.
pub fn configurar_fuentes(ctx: &egui::Context) {
    let mut fuentes = egui::FontDefinitions::default();

    fuentes.font_data.insert(
        NOMBRE_FUENTE_CJK.to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(FUENTE_CJK_BYTES)),
    );

    // Va **al final** de cada familia, como respaldo: las de `egui` se consultan primero, así
    // que el latín y el cirílico se siguen dibujando igual que antes y solo los caracteres que
    // a ellas les faltan llegan hasta aquí.
    for familia in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        if let Some(lista) = fuentes.families.get_mut(&familia) {
            lista.push(NOMBRE_FUENTE_CJK.to_owned());
        }
    }

    ctx.set_fonts(fuentes);
}

impl AplicacionMapaMental {
    /// Ofrece acceso de lectura a los servicios desacoplados de la aplicación.
    #[cfg(test)]
    pub(crate) fn servicios(&self) -> &servicios_aplicacion::ServiciosAplicacion {
        &self.servicios
    }
    /// Consulta el agregado del mapa sin permitir modificarlo.
    pub(crate) fn mapa(&self) -> &EstadoDelMapa {
        &self.mapa
    }

    /// Ofrece a la presentación el agregado del mapa sin exponer los campos de la raíz.
    pub(crate) fn mapa_mut(&mut self) -> &mut EstadoDelMapa {
        &mut self.mapa
    }

    /// Ofrece a la presentación los gestos transitorios del lienzo como una sola capacidad.
    pub(crate) fn lienzo_mut(&mut self) -> &mut EstadoDelLienzo {
        &mut self.lienzo
    }

    /// Consulta los gestos transitorios del lienzo.
    pub(crate) fn lienzo(&self) -> &EstadoDelLienzo {
        &self.lienzo
    }

    /// Ofrece a la presentación únicamente sus preferencias y búferes inmediatos.
    pub(crate) fn presentacion_mut(&mut self) -> &mut EstadoDePresentacion {
        &mut self.presentacion
    }

    /// Consulta preferencias y búferes de presentación sin modificarlos.
    pub(crate) fn presentacion(&self) -> &EstadoDePresentacion {
        &self.presentacion
    }

    /// Divide préstamos de solo lectura del mapa y escritura de la presentación.
    pub(crate) fn mapa_y_presentacion_mut(
        &mut self,
    ) -> (&EstadoDelMapa, &mut EstadoDePresentacion) {
        (&self.mapa, &mut self.presentacion)
    }

    /// Divide los préstamos del mapa y del lienzo porque son agregados independientes.
    pub(crate) fn mapa_y_lienzo_mut(&mut self) -> (&EstadoDelMapa, &mut EstadoDelLienzo) {
        (&self.mapa, &mut self.lienzo)
    }

    /// Abre y valida un mapa mediante la persistencia ya elegida por la aplicación.
    pub(crate) fn abrir_mapa_desde(
        &self,
        ruta: &std::path::Path,
    ) -> crate::error::AppResult<Proyecto> {
        self.servicios.abrir(ruta)
    }

    /// Prepara una carpeta de trabajo sin revelar a la presentación el adaptador concreto.
    pub(crate) fn preparar_carpeta_de_trabajo(
        &self,
        ruta: &std::path::Path,
    ) -> crate::error::AppResult<()> {
        self.servicios.preparar_carpeta(ruta)
    }

    /// Guarda un documento auxiliar mediante la escritura segura de la aplicación.
    pub(crate) fn guardar_documento_auxiliar(
        &self,
        ruta: &std::path::Path,
        contenido: &[u8],
    ) -> crate::error::AppResult<()> {
        self.servicios.guardar_documento(ruta, contenido)
    }

    /// Exporta el mapa activo al documento Markdown que consume un agente.
    pub(crate) fn exportar_mapa_para_agente(
        &self,
        ruta: &std::path::Path,
        idioma: crate::textos::Idioma,
    ) -> crate::error::AppResult<()> {
        self.servicios.exportar(&self.mapa.proyecto, ruta, idioma)
    }

    /// Ofrece el estado cohesionado del recorrido «Enviar a…».
    pub(crate) fn agentes_mut(&mut self) -> &mut estado_agentes::EstadoAgentes {
        &mut self.agentes
    }

    /// Consulta el estado del recorrido «Enviar a…».
    pub(crate) fn agentes(&self) -> &estado_agentes::EstadoAgentes {
        &self.agentes
    }

    /// Ofrece el ciclo de vida cohesionado del archivo activo.
    pub(crate) fn persistencia_mut(&mut self) -> &mut estado_persistencia::EstadoPersistencia {
        &mut self.persistencia
    }

    /// Consulta el ciclo de vida del archivo activo.
    pub(crate) fn persistencia(&self) -> &estado_persistencia::EstadoPersistencia {
        &self.persistencia
    }

    /// Construye el estado inicial de la aplicación.
    ///
    /// Crea un proyecto de ejemplo con tres pilares de partida, para que el usuario vea un
    /// mapa con contenido nada mas abrir el programa en lugar de un lienzo vacío.
    ///
    /// # Parámetros
    /// - `cc`: contexto de creación de `eframe`, del que se toma el contexto de `egui`
    ///   para instalar el estilo del tema antes de dibujar el primer fotograma.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::nueva_con_contexto(&cc.egui_ctx)
    }

    /// Construye una aplicación con estado inicial limpio y aislado del disco.
    ///
    /// No lee preferencias ni busca archivos de recuperación del sistema del usuario,
    /// garantizando que las pruebas y los arneses de interfaz sean totalmente
    /// deterministas e independientes del entorno del host.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` en el que se instala el estilo del tema por defecto.
    #[cfg(test)]
    pub fn nueva_limpia(ctx: &egui::Context) -> Self {
        Self::nueva_limpia_en(ctx, None)
    }

    /// Construye una aplicación limpia que guarda sus copias **en la carpeta indicada**.
    ///
    /// Es lo que usa una prueba que sí quiere comprobar el autoguardado: recibe una carpeta
    /// temporal suya y no comparte nada con las demás, así que la batería puede seguir
    /// corriendo en paralelo sin cerrojos ni variables de entorno.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` en el que se instala el estilo del tema.
    /// - `carpeta_datos`: dónde escribir la copia de recuperación. Con `None` no se escribe
    ///   ninguna, que es lo que quiere la mayoría de las pruebas de interfaz.
    #[cfg(test)]
    pub fn nueva_limpia_en(ctx: &egui::Context, carpeta_datos: Option<PathBuf>) -> Self {
        Self::nueva_interna(
            ctx,
            Preferencias::default(),
            None,
            Texto::MensajeBienvenida
                .en(Preferencias::default().idioma)
                .to_string(),
            carpeta_datos,
        )
    }

    /// Construye el estado inicial a partir de un contexto de `egui` suelto.
    ///
    /// Carga las preferencias guardadas del usuario y evalúa si existe trabajo sin guardar
    /// de una sesión anterior para ofrecer su recuperación.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` en el que se instala el estilo del tema.
    pub fn nueva_con_contexto(ctx: &egui::Context) -> Self {
        let preferencias = match Preferencias::cargar() {
            Ok(preferencias) => preferencias,
            Err(error) => {
                crate::error::registrar(&error, "cargar preferencias al iniciar");
                Preferencias::primer_arranque()
            }
        };

        // Dónde guarda sus datos este usuario se resuelve **aquí y una sola vez**. A partir
        // de este punto la carpeta viaja dentro de la aplicación, en lugar de volver a
        // consultarse en cada escritura.
        let carpeta_datos = crate::autoguardado::directorio_datos();

        // Si la sesión anterior terminó de forma inesperada, aquí aparece su copia. No se
        // restaura sola: se ofrece, porque el usuario puede haber cerrado a propósito sin
        // guardar y restaurar sin preguntar le devolvería justo lo que descartó.
        let recuperacion = match carpeta_datos.as_deref() {
            Some(carpeta) => match crate::autoguardado::buscar_recuperacion_en(carpeta) {
                Ok(recuperacion) => recuperacion,
                Err(error) => {
                    crate::error::registrar(&error, "evaluar copia de recuperación al iniciar");
                    None
                }
            },
            None => None,
        };

        let mensaje_bienvenida = if recuperacion.is_some() {
            Texto::MensajeTrabajoPrevioEncontrado
                .en(preferencias.idioma)
                .to_string()
        } else {
            Texto::MensajeBienvenida.en(preferencias.idioma).to_string()
        };

        Self::nueva_interna(
            ctx,
            preferencias,
            recuperacion,
            mensaje_bienvenida,
            carpeta_datos,
        )
    }

    /// Construcción interna común de [`AplicacionMapaMental`].
    fn nueva_interna(
        ctx: &egui::Context,
        preferencias: Preferencias,
        recuperacion: Option<Recuperacion>,
        mensaje_bienvenida: String,
        carpeta_datos: Option<PathBuf>,
    ) -> Self {
        let tema = ThemeConfig::new(preferencias.tema);

        // El tema y las fuentes tienen que llegar a `egui` antes del primer fotograma. Si no se instala,
        // `egui` pinta menús, botones y paneles con su estilo por omisión, que es oscuro,
        // y el tema claro aparece partido: lienzo claro con barras oscuras alrededor.
        tema.instalar_en(ctx);
        configurar_fuentes(ctx);
        let proyecto = Proyecto::nuevo_ejemplo_inicial(preferencias.idioma);

        let selected = Some(proyecto.root_id);

        // Al arrancar no hay ningún mapa abierto todavía, así que la propuesta solo puede
        // salir de la preferencia guardada o de la carpeta de fábrica. En cuanto se abre la
        // ventana de conexión se recalcula teniendo en cuenta el mapa activo, que es cuando
        // el dato importa (ver `ui::conexiones_modal`).
        let espacio_trabajo_ia =
            crate::conectores::espacio_trabajo_propuesto(&preferencias.espacio_trabajo_ia, None);

        // El intervalo elegido en una sesión anterior se aplica ahora. Sin esto la
        // preferencia se guardaba y no servía de nada: el control arrancaba siempre con el
        // valor de fábrica.
        let mut autoguardado = ControlAutoguardado::nuevo();
        autoguardado.fijar_minutos(preferencias.intervalo_autoguardado_minutos);

        Self {
            mapa: EstadoDelMapa {
                proyecto,
                nodo_seleccionado: selected,
                historial: crate::historial::HistorialDeCambios::nuevo(),
            },
            lienzo: EstadoDelLienzo {
                conexion: ConexionEnCurso::default(),
                edicion: EdicionDelTitulo::default(),
                vista: VistaDelLienzo::default(),
                indice_espacial_del_lienzo: canvas::IndiceEspacialLienzo::default(),
            },
            presentacion: EstadoDePresentacion {
                tema,
                ventanas: VentanasAbiertas::default(),
                guia_de_ayuda_activa: None,
                texto_para_importar: String::new(),
                etiqueta_en_escritura: String::new(),
                error_de_importacion: None,
                vista_previa_para_ia: None,
                vista_previa_de_correcciones: None,
                editor_proyecto_ia: None,
                aviso_error_proyecto_ia: None,
                aviso_mapa_en_raiz: None,
                aviso_guardar_como_otra_carpeta: None,
                mensaje_de_estado: Some((mensaje_bienvenida, std::time::Instant::now())),
                preferencias,
                escala_aplicada: false,
                filtro_de_busqueda: FiltroDeBusqueda::default(),
            },
            servicios: servicios_aplicacion::ServiciosAplicacion::locales(),
            agentes: estado_agentes::EstadoAgentes::nuevo(ctx.clone(), espacio_trabajo_ia),
            persistencia: estado_persistencia::EstadoPersistencia::nuevo(
                ctx.clone(),
                autoguardado,
                carpeta_datos,
                recuperacion,
            ),
        }
    }

    /// Mantiene sincronizada la escala de la interfaz con las preferencias guardadas.
    ///
    /// Se ocupa de tres cosas, en este orden:
    ///
    /// 1. **Primer fotograma**: aplica la escala guardada. No puede hacerse antes porque
    ///    el contexto de `egui` todavía no conoce las características del monitor.
    /// 2. **Primer arranque**: si nunca se ha ajustado, propone una escala a partir de la
    ///    resolución del monitor.
    /// 3. **Después**: detecta si el usuario ha cambiado la escala con `Ctrl` `+` / `-` /
    ///    `0` —atajos que `egui` gestiona por su cuenta— y guarda el nuevo valor.
    ///
    /// Ese tercer punto es la razón de ser de todo esto: `egui` sabe escalar la interfaz,
    /// pero **no recuerda el ajuste entre sesiones**. Sin esta sincronización, quien
    /// trabaja en un televisor tendría que reajustar el tamaño en cada arranque.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` del que se lee y al que se aplica la escala.
    fn sincronizar_escala_interfaz(&mut self, ctx: &egui::Context) {
        if !self.presentacion.escala_aplicada {
            self.presentacion.escala_aplicada = true;

            if !self.presentacion.preferencias.sugerencia_mostrada {
                let (monitor, escala_sistema) = ctx.input(|i| {
                    (
                        i.viewport().monitor_size,
                        i.viewport().native_pixels_per_point,
                    )
                });

                let sugerida = crate::preferencias::escala_sugerida(monitor, escala_sistema);
                self.presentacion.preferencias.sugerencia_mostrada = true;

                if (sugerida - crate::preferencias::ESCALA_POR_DEFECTO).abs() > 0.01 {
                    self.presentacion.preferencias.escala_interfaz = sugerida;
                    let aviso = Texto::AvisoSugerenciaEscalaInicial
                        .en(self.idioma())
                        .replace("{:.0}", &format!("{:.0}", sugerida * 100.0));
                    self.establecer_estado(aviso);
                }

                if let Err(error) = self.guardar_preferencias() {
                    crate::error::registrar(&error, "guardar preferencias de escala");
                }
            }

            ctx.set_zoom_factor(self.presentacion.preferencias.escala_interfaz);
            return;
        }

        // `egui` aplica por su cuenta los atajos de zoom al final de cada fotograma. Si
        // el factor ya no coincide con el guardado, es que el usuario los ha usado.
        //
        // Qué hacer con esa diferencia lo decide `ajuste_de_escala`, que vive en el módulo
        // de preferencias y no necesita contexto gráfico: es donde está escrito el porqué
        // de cada caso, y es lo que permite probarlo.
        let Some(ajuste) = crate::preferencias::ajuste_de_escala(
            ctx.zoom_factor(),
            self.presentacion.preferencias.escala_interfaz,
        ) else {
            return;
        };

        if ajuste.forzar_zoom {
            ctx.set_zoom_factor(ajuste.escala);
        }

        if ajuste.guardar {
            self.presentacion.preferencias.escala_interfaz = ajuste.escala;
            if let Err(error) = self.guardar_preferencias() {
                crate::error::registrar(&error, "guardar preferencias de escala");
            }
            let aviso = Texto::AvisoTamanoInterfazAjustado
                .en(self.idioma())
                .replace("{:.0}", &format!("{:.0}", ajuste.escala * 100.0));
            self.establecer_estado(aviso);
        }
    }

    /// Cambia la escala de la interfaz y guarda la preferencia.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` al que se aplica la escala.
    /// - `escala`: factor deseado. Se limita al rango admitido.
    pub fn cambiar_escala_interfaz(&mut self, ctx: &egui::Context, escala: f32) {
        let escala = escala.clamp(
            crate::preferencias::ESCALA_MINIMA,
            crate::preferencias::ESCALA_MAXIMA,
        );

        self.presentacion.preferencias.escala_interfaz = escala;
        ctx.set_zoom_factor(escala);
        if let Err(error) = self.guardar_preferencias() {
            crate::error::registrar(&error, "guardar preferencias de escala");
        }

        let aviso = Texto::AvisoTamanoInterfazAjustado
            .en(self.idioma())
            .replace("{:.0}", &format!("{:.0}", escala * 100.0));
        self.establecer_estado(aviso);
    }

    /// Autoguarda el proyecto si ha pasado el intervalo y hay cambios.
    ///
    /// Se invoca en cada fotograma, así que en la inmensa mayoría de las llamadas se
    /// limita a comparar dos instantes y salir.
    ///
    /// Un fallo al autoguardar **no interrumpe al usuario con un error**: se anota en la
    /// barra de estado y se registra. El autoguardado es una comodidad, y avisar con
    /// alarma de que la red de seguridad no está disponible sería más molesto que útil,
    /// sobre todo porque el guardado normal sigue funcionando.
    fn procesar_autoguardado(&mut self) {
        let es_solo_lectura_por_raiz = self.es_solo_lectura_por_raiz();
        match persistencia_mapa_comando::autoguardar_si_toca(
            &mut self.persistencia,
            &self.mapa.proyecto,
            es_solo_lectura_por_raiz,
        ) {
            persistencia_mapa_comando::ResultadoAutoguardado::SinCambios => {}
            persistencia_mapa_comando::ResultadoAutoguardado::Guardado => {
                self.establecer_estado(Texto::AvisoAutoguardadoOk.en(self.idioma()));
            }
            persistencia_mapa_comando::ResultadoAutoguardado::Fallo(error) => {
                crate::error::registrar(&error, "autoguardado");
                self.establecer_estado(Texto::AvisoAutoguardadoError.en(self.idioma()));
            }
        }
    }

    /// Restaura el proyecto de la copia de recuperación pendiente.
    ///
    /// Adopta también la ruta del archivo original, de modo que un `Ctrl+S` posterior
    /// escriba donde el usuario esperaba y no le pregunte de nuevo.
    ///
    /// Va por la puerta común [`Self::sustituir_el_mapa_abierto`], y eso es lo importante:
    /// **restaurar tira el mapa que hubiera en pantalla**. Antes lo asignaba a mano, sin
    /// dejar copia de lo que pisaba, y el aviso de recuperación no es modal: quien arrancara,
    /// se pusiera a trabajar y luego pulsara «Restaurar» perdía lo hecho sin rastro. La
    /// puerta común escribe la copia del anterior, decide si había trabajo que salvar y lo
    /// cuenta en el aviso.
    pub fn aplicar_recuperacion_pendiente(&mut self) {
        let Some(recuperacion) =
            persistencia_mapa_comando::restaurar_recuperacion(&mut self.persistencia)
        else {
            return;
        };

        let descripcion = recuperacion.descripcion();

        // La copia en disco se puede pisar sin miedo: el mapa que traía ya está aquí, en
        // memoria, y lo que pasa a guardarse es el que se descarta, que es lo que hay que
        // poder recuperar a partir de ahora.
        self.sustituir_el_mapa_abierto(
            recuperacion.proyecto,
            recuperacion.ruta_original,
            &Texto::AvisoTrabajoRecuperado
                .en(self.idioma())
                .replace("{}", &descripcion),
        );
    }

    /// Guarda las preferencias de esta ejecución en su carpeta de datos.
    ///
    /// Único punto del programa que las escribe. Antes cada pantalla llamaba a
    /// `preferencias.guardar()`, que resolvía la carpeta por su cuenta desde una variable de
    /// entorno; una prueba de interfaz que simulara un clic en el menú de idioma acababa
    /// **escribiendo en el perfil real de quien ejecutaba la batería**. Pasó de verdad: una
    /// ejecución dejó el idioma del usuario en chino.
    ///
    /// Con la carpeta dentro de la aplicación, una prueba que no pide carpeta no escribe.
    ///
    /// # Devuelve
    /// `Ok(())` si se guardaron. El error se devuelve completo para que el punto de interfaz
    /// que solicitó el cambio decida cómo informar sin perder la causa.
    pub(crate) fn guardar_preferencias(&self) -> crate::error::AppResult<()> {
        match self.persistencia.carpeta_datos.as_deref() {
            Some(carpeta) => self.presentacion.preferencias.guardar_en(carpeta),
            None => Err(AppError::escritura(
                "la carpeta de datos del usuario",
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no se pudo determinar dónde guardar las preferencias",
                ),
            )),
        }
    }

    /// Escribe la copia de recuperación de esta ejecución.
    ///
    /// Único punto del programa que decide **dónde** va la copia: todo lo demás llama aquí.
    ///
    /// # Errores
    /// - [`AppError::Escritura`] si no hay carpeta de datos conocida o no se puede escribir.
    /// - [`AppError::Formato`] si el proyecto no se puede serializar.
    pub(crate) fn guardar_copia_de_recuperacion(&self) -> crate::error::AppResult<()> {
        persistencia_mapa_comando::escribir_copia_de_recuperacion(
            &self.persistencia,
            &self.mapa.proyecto,
        )
    }

    /// Descarta la copia de recuperación pendiente y la borra del disco.
    pub fn eliminar_recuperacion_pendiente(&mut self) {
        persistencia_mapa_comando::descartar_recuperacion(&mut self.persistencia);
        self.establecer_estado(Texto::AvisoRecuperacionDescartada.en(self.idioma()));
    }

    /// Registra que el proyecto acaba de guardarse en el archivo del usuario.
    ///
    /// Descarta la copia de recuperación: el trabajo ya está a salvo en su sitio, y
    /// conservarla solo serviría para ofrecer una restauración obsoleta en el próximo
    /// arranque.
    ///
    /// # Parámetros
    /// - `ruta`: archivo en el que se acaba de guardar.
    pub fn anotar_guardado(&mut self, ruta: &std::path::Path) {
        persistencia_mapa_comando::registrar_guardado(
            &mut self.persistencia,
            &self.mapa.proyecto,
            ruta,
        );
    }

    /// Registra un guardado cuya huella ya se calculó fuera del hilo gráfico.
    pub fn anotar_guardado_con_huella(&mut self, ruta: &std::path::Path, huella: Option<u64>) {
        persistencia_mapa_comando::registrar_guardado_con_huella(
            &mut self.persistencia,
            &self.mapa.proyecto,
            ruta,
            huella,
        );
    }

    /// Sincroniza el espacio de trabajo de los agentes con el archivo del mapa al abrirlo.
    pub fn sincronizar_espacio_de_trabajo_con_el_mapa(&mut self) {
        let propuesto = crate::conectores::espacio_trabajo_propuesto(
            &self.presentacion.preferencias.espacio_trabajo_ia,
            self.persistencia.ruta_actual.as_deref(),
        );
        // Se canoniza aquí, en el único sitio donde se calcula el espacio de trabajo (A-2): así
        // toda pantalla que lo lea después ve siempre la misma ruta resuelta, y una carpeta
        // alcanzada por una unión de directorio no se muestra distinta según qué ventana pinte.
        let nuevo = crate::conectores::canonizar_o_conservar(propuesto);
        self.agentes.espacio_de_trabajo = nuevo.clone();
        let pref = nuevo.display().to_string();
        if self.presentacion.preferencias.espacio_trabajo_ia != pref
            && crate::conectores::es_carpeta_valida_para_espacio_trabajo(&nuevo)
        {
            self.presentacion.preferencias.espacio_trabajo_ia = pref;
            let _ = self.guardar_preferencias();
        }
    }

    /// Comprueba si el usuario tiene cambios locales no guardados en el archivo actual.
    pub fn hay_cambios_locales_sin_guardar(&self) -> bool {
        persistencia_mapa_comando::tiene_cambios_locales_sin_guardar(
            &self.persistencia,
            &self.mapa.proyecto,
            self.lienzo.edicion.nodo.is_some(),
        )
    }

    /// Convierte las decisiones del módulo de vigilancia en cambios visibles de la interfaz.
    pub(crate) fn procesar_eventos_de_vigilancia(&mut self) {
        let hay_cambios_locales = self.hay_cambios_locales_sin_guardar();
        let acciones =
            crate::vigilancia_mapa::atender_vigilancia(&mut self.persistencia, hay_cambios_locales);
        for accion in acciones {
            self.presentar_accion_de_vigilancia(accion);
        }
    }

    /// Atiende el resultado del hilo de envío a agentes de IA y actualiza el estado de la aplicación.
    pub(crate) fn procesar_eventos_de_envio(&mut self, ctx: &egui::Context) {
        match crate::envio_agente_comando::atender_envio_en_segundo_plano(
            &mut self.agentes,
            &mut self.persistencia,
            ctx,
        ) {
            crate::envio_agente_comando::ReaccionEnvioSegundoPlano::Ninguna => {}
            crate::envio_agente_comando::ReaccionEnvioSegundoPlano::Exito {
                mapa_guardado,
                iniciada,
            } => {
                if let Some((ruta, huella)) = mapa_guardado {
                    self.anotar_guardado_con_huella(&ruta, huella);
                }
                if iniciada {
                    self.establecer_estado(Texto::AvisoSeguimientoIniciado.en(self.idioma()));
                }
                self.establecer_estado(Texto::IaVigilandoCambios.en(self.idioma()));
            }
            crate::envio_agente_comando::ReaccionEnvioSegundoPlano::Fallo {
                mapa_guardado,
                error,
            } => {
                if let Some((ruta, huella)) = mapa_guardado {
                    self.anotar_guardado_con_huella(&ruta, huella);
                }
                self.reportar_error(&error, Texto::SesionAgenteTitulo.en(self.idioma()));
            }
        }
    }

    /// Prepara la sesión con un agente de IA delegando en el comando correspondiente.
    pub(crate) fn preparar_sesion_agente(&mut self, agente: &crate::conectores::Agente) {
        let es_solo_lectura = self.es_solo_lectura_por_raiz();
        if es_solo_lectura {
            self.abrir_aviso_mapa_en_raiz();
        } else if crate::conectores::estado_de_la_conexion(agente, &self.agentes.espacio_de_trabajo)
            == crate::conectores::EstadoDeLaConexion::ConectadoAOtraCarpeta
        {
            self.presentacion.ventanas.modal_conexiones = true;
        }
        let idioma = self.idioma();
        let editor_ia = self.presentacion.editor_proyecto_ia().clone();
        let res = crate::envio_agente_comando::preparar_envio(
            &mut self.agentes,
            agente,
            &self.mapa.proyecto,
            idioma,
            es_solo_lectura,
            editor_ia.as_ref(),
        );
        if let Err(ref error) = res {
            let idioma = self.idioma();
            self.reportar_error(error, Texto::SesionAgenteTitulo.en(idioma));
        }
    }

    /// Cancela la sesión en preparación delegando en el comando correspondiente.
    pub(crate) fn cancelar_sesion_agente(&mut self) {
        crate::envio_agente_comando::cancelar_envio(&mut self.agentes);
    }

    /// Confirma e inicia el envío en segundo plano delegando en el comando correspondiente.
    pub(crate) fn confirmar_sesion_agente(
        &mut self,
        ctx: &egui::Context,
    ) -> crate::error::AppResult<bool> {
        let ruta_actual = self.persistencia.ruta_actual.as_deref();
        let carpeta_sugerida = self.sugerir_carpeta_para_guardar();
        let idioma = self.idioma();
        crate::envio_agente_comando::iniciar_envio_confirmado(
            &mut self.agentes,
            &self.mapa.proyecto,
            ruta_actual,
            carpeta_sugerida.as_deref(),
            idioma,
            ctx,
        )
    }

    /// Exporta el Markdown para IA mediante diálogo y presenta el resultado.
    pub(crate) fn exportar_markdown_dialogo(&mut self) {
        match crate::envio_agente_comando::exportar_markdown_para_ia(
            &self.mapa.proyecto,
            self.es_solo_lectura_por_raiz(),
            self.idioma(),
            &self.servicios,
        ) {
            crate::envio_agente_comando::ResultadoExportacionMarkdown::SoloLecturaPorRaiz => {
                self.abrir_aviso_mapa_en_raiz();
            }
            crate::envio_agente_comando::ResultadoExportacionMarkdown::Cancelado => {}
            crate::envio_agente_comando::ResultadoExportacionMarkdown::Exito(ruta) => {
                let aviso = Texto::ModalGuardadoEn
                    .en(self.idioma())
                    .replace("{}", &ruta.display().to_string());
                self.establecer_estado(aviso);
            }
            crate::envio_agente_comando::ResultadoExportacionMarkdown::Fallo(e) => {
                self.reportar_error(&e, Texto::ModalExportarElMarkdownPara.en(self.idioma()));
            }
        }
    }

    /// Detiene el seguimiento y presenta su confirmación sin exponer persistencia a la barra.
    pub(crate) fn detener_seguimiento_del_mapa(&mut self) {
        crate::vigilancia_mapa::detener_vigilancia(&mut self.persistencia);
        self.establecer_estado(crate::textos::Texto::AvisoSeguimientoDetenido.en(self.idioma()));
    }

    /// Presenta una decisión ya clasificada sin devolver al módulo el objeto raíz completo.
    fn presentar_accion_de_vigilancia(
        &mut self,
        accion: crate::vigilancia_mapa::AccionCargaExterna,
    ) {
        match accion {
            crate::vigilancia_mapa::AccionCargaExterna::Ignorar => {}
            crate::vigilancia_mapa::AccionCargaExterna::RetenerPorCambiosLocales => {
                self.establecer_estado(
                    crate::textos::Texto::AvisoCambioExternoConCambiosLocales.en(self.idioma()),
                );
            }
            crate::vigilancia_mapa::AccionCargaExterna::Sustituir(nuevo_proyecto) => {
                let _ = self.guardar_copia_de_recuperacion();
                self.mapa.proyecto = *nuevo_proyecto;
                self.lienzo.indice_espacial_del_lienzo.invalidar();
                self.mapa.historial.olvidar(&self.mapa.proyecto);
                if !self.mapa.proyecto.nodes.contains_key(
                    &self
                        .mapa
                        .nodo_seleccionado
                        .unwrap_or(self.mapa.proyecto.root_id),
                ) {
                    self.mapa.nodo_seleccionado = Some(self.mapa.proyecto.root_id);
                }
                self.establecer_estado(
                    crate::textos::Texto::AvisoMapaActualizadoPorIA.en(self.idioma()),
                );
            }
        }
    }

    /// Guarda el proyecto, pidiendo la ruta al usuario solo si hace falta.
    ///
    /// Reúne en un único sitio lo que antes estaban siendo tres copias del mismo bloque
    /// —el menú Guardar, el menú Guardar Como y el atajo `Ctrl+S`—, que es justo el tipo
    /// de duplicación que acaba divergiendo.
    ///
    /// Cuando hay que preguntar, el diálogo se abre en la carpeta sugerida por
    /// [`AplicacionMapaMental::sugerir_carpeta_para_guardar`]: la del archivo actual si ya se
    /// había guardado, o la del proyecto de código al que se refiere el mapa.
    ///
    /// # Parámetros
    /// - `pedir_ruta_siempre`: si es cierto, pregunta aunque el proyecto ya tenga
    ///   archivo. Es el comportamiento de «Guardar Como».
    ///
    /// # Devuelve
    /// `true` si el proyecto quedó guardado; `false` si el usuario canceló el diálogo o
    /// si la escritura falló (en cuyo caso el error ya se le ha comunicado).
    pub fn solicitar_guardado(&mut self, pedir_ruta_siempre: bool) -> bool {
        if !pedir_ruta_siempre && self.es_solo_lectura_por_raiz() {
            self.abrir_aviso_mapa_en_raiz();
            return false;
        }

        let destino = match (!pedir_ruta_siempre)
            .then(|| self.persistencia.ruta_actual.clone())
            .flatten()
        {
            Some(existente) => existente,
            None => {
                let carpeta = self.sugerir_carpeta_para_guardar();
                let nombre = format!("{}.mmcelt", self.mapa.proyecto.title);
                let Some(elegida) = dialogos::pedir_archivo_para_guardar(
                    &nombre,
                    carpeta.as_deref(),
                    self.idioma(),
                ) else {
                    return false;
                };
                elegida
            }
        };

        if pedir_ruta_siempre {
            self.presentacion.aviso_guardar_como_otra_carpeta =
                crate::guardado_como::evaluar_destino_guardado_como_aviso(self, &destino);
            if self.presentacion.aviso_guardar_como_otra_carpeta.is_some() {
                return true;
            }
        }

        match persistencia_mapa_comando::guardar(
            &self.servicios,
            &mut self.persistencia,
            &self.mapa.proyecto,
            &destino,
        ) {
            Ok(()) => {
                let txt = destino.display().to_string();
                self.establecer_estado(
                    Texto::ModalGuardadoEn.en(self.idioma()).replace("{}", &txt),
                );
                true
            }
            Err(e) => {
                self.reportar_error(&e, Texto::ContextoGuardarMapaMental.en(self.idioma()));
                false
            }
        }
    }

    /// Devuelve la carpeta que debe proponerse al guardar este mapa.
    ///
    /// Prioriza, en este orden:
    /// 1. La carpeta del archivo actual, si el mapa ya se había guardado.
    /// 2. La carpeta del proyecto de código al que se refiere el mapa, deducida de las
    ///    rutas de sus nodos. Es el caso de los mapas que genera una IA trabajando sobre
    ///    un código concreto, y de los que produce el escáner de repositorios.
    /// 3. Ninguna, y entonces el diálogo abre donde el sistema recuerde.
    pub fn sugerir_carpeta_para_guardar(&self) -> Option<std::path::PathBuf> {
        persistencia_mapa_comando::carpeta_sugerida_para_guardar(
            &self.persistencia,
            &self.mapa.proyecto,
        )
    }

    /// Abre el panel lateral derecho de ayuda detallada enfocado en un tema específico.
    ///
    /// # Parámetros
    /// - `topic`: el tema con el que se abre el panel. Si ya estaba abierto por otro tema,
    ///   se cambia al indicado en lugar de abrirse una segunda vez.
    pub fn abrir_ayuda(&mut self, topic: TemaDeAyuda) {
        self.presentacion.guia_de_ayuda_activa = Some(topic);
        self.presentacion.ventanas.panel_de_ayuda = true;
    }

    /// Muestra una «galleta» de ayuda compacta con su botón de «+info».
    ///
    /// La galleta es el primer nivel de la ayuda: una frase junto al control que explica
    /// para qué sirve. El botón abre el segundo nivel, la guía completa del panel lateral.
    ///
    /// # Parámetros
    /// - `ui`: el `Ui` donde se inserta la galleta, normalmente el del control que explica.
    /// - `topic`: el tema del que se toman el resumen y la guía a la que lleva el botón.
    pub fn dibujar_galleta_de_ayuda(&mut self, ui: &mut egui::Ui, topic: TemaDeAyuda) {
        if !self.presentacion.ventanas.galletas_de_ayuda {
            return;
        }

        ui.horizontal_wrapped(|ui| {
            ui.visuals_mut().override_text_color = Some(self.presentacion.tema.texto_secundario);
            ui.label(
                egui::RichText::new(format!("🍪 {}", topic.resumen_de_galleta(self.idioma())))
                    .small()
                    .italics(),
            );
            if ui
                .button(
                    egui::RichText::new(Texto::GalletaMasInfo.en(self.idioma()))
                        .small()
                        .strong()
                        .color(self.presentacion.tema.colores_de_las_ramas[0]),
                )
                .on_hover_text(Texto::GalletaMasInfoAyuda.en(self.idioma()))
                .clicked()
            {
                self.abrir_ayuda(topic);
            }
        });
    }

    /// Muestra un mensaje informativo en la barra de estado.
    ///
    /// El mensaje se desvanece solo pasados unos segundos.
    ///
    /// # Parámetros
    /// - `msg`: texto a mostrar, ya redactado para el usuario final.
    pub fn establecer_estado(&mut self, msg: impl Into<String>) {
        self.presentacion.mensaje_de_estado = Some((msg.into(), std::time::Instant::now()));
    }

    /// Comunica un error al usuario y lo deja registrado para su depuración.
    ///
    /// Este es **el único punto** en el que un [`AppError`] se convierte en texto
    /// visible. Las capas internas (modelo, persistencia, puente con la IA) se limitan
    /// a propagar el error hacia arriba; aquí se decide cómo se presenta.
    ///
    /// El mensaje se antepone con el emoji correspondiente a la gravedad del error,
    /// para que el usuario distinga de un vistazo un aviso menor de un archivo dañado.
    ///
    /// # Parámetros
    /// - `error`: el error a comunicar.
    /// - `contexto`: descripción breve de la operación durante la que se produjo, que
    ///   se escribe en el registro pero no se muestra al usuario.
    pub fn reportar_error(&mut self, error: &AppError, contexto: &str) {
        crate::error::registrar(error, contexto);
        let idioma = self.idioma();
        self.establecer_estado(format!(
            "{} {}",
            error.severidad().emoji(),
            error.mensaje_usuario().en(idioma)
        ));
    }

    /// Cambia cada cuántos minutos se autoguarda, y lo recuerda entre sesiones.
    ///
    /// # Parámetros
    /// - `minutos`: uno de `autoguardado::MINUTOS_DE_AUTOGUARDADO_OFRECIDOS`; el cero
    ///   desactiva el autoguardado.
    pub(crate) fn fijar_minutos_de_autoguardado(&mut self, minutos: u64) {
        self.persistencia.autoguardado.fijar_minutos(minutos);
        self.presentacion
            .preferencias
            .intervalo_autoguardado_minutos = minutos;
        if let Err(error) = self.guardar_preferencias() {
            crate::error::registrar(&error, "guardar intervalo de autoguardado");
        }
    }

    /// Cambia el tema visual y lo aplica a toda la ventana.
    ///
    /// Regenera la paleta del lienzo y reinstala el estilo de `egui`, que es lo que viste
    /// la barra de menús, el panel lateral, los botones y los cuadros de diálogo. Ambas
    /// cosas van juntas a propósito: cambiar solo una deja la ventana a medio pintar.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` al que se aplica el estilo nuevo.
    /// - `mode`: modo claro, oscuro o de alto contraste.
    pub fn establecer_tema(&mut self, ctx: &egui::Context, mode: AppThemeMode) {
        self.presentacion.tema = ThemeConfig::new(mode);
        self.presentacion.tema.instalar_en(ctx);

        // El tema se conserva entre sesiones junto al resto de preferencias.
        self.presentacion.preferencias.tema = mode;
        if let Err(error) = self.guardar_preferencias() {
            crate::error::registrar(&error, "guardar preferencia de tema");
        }
    }

    /// Lleva la vista al nodo indicado, sin cambiar el zoom.
    ///
    /// La conversión a pantalla del lienzo es `centro + (posición + desplazamiento) * zoom`,
    /// así que dejar un nodo en el centro es tan sencillo como poner el desplazamiento en su
    /// posición cambiada de signo.
    ///
    /// El zoom se respeta a propósito: quien busca un nodo quiere ir hasta él, no que le
    /// cambien también el nivel de detalle al que estaba mirando el mapa.
    ///
    /// # Parámetros
    /// - `id`: el nodo al que ir. Si no existe, la vista se queda donde está.
    pub(crate) fn centrar_en_el_nodo(&mut self, id: uuid::Uuid) {
        let Some(nodo) = self.mapa.proyecto.nodes.get(&id) else {
            return;
        };
        self.lienzo.vista.desplazamiento = -egui::Vec2::new(nodo.pos[0], nodo.pos[1]);
    }

    /// Encuadra el mapa entero en la ventana.
    ///
    /// Se invoca tras importar o cargar un mapa, cuando la posición previa de la cámara ya
    /// no tiene sentido para el contenido nuevo, y desde «Centrar vista» y `Ctrl` `F`.
    ///
    /// Antes ponía el desplazamiento a cero y el zoom a 1,0, sin mirar el mapa. Con la
    /// disposición radial, que reparte las hojas sobre una circunferencia y crece deprisa,
    /// eso dejaba en pantalla la tarjeta central y poco más, y no había ningún «ajustar a la
    /// ventana» al que recurrir: solo alejarse con la rueda hasta dar con el mapa.
    ///
    /// El cálculo está en [`crate::ui::canvas::encuadre_para_ver_el_mapa`], que no necesita
    /// contexto gráfico y por tanto se puede probar.
    pub fn centrar_en_la_raiz(&mut self) {
        let Some(caja) = crate::layout::caja_del_mapa(&self.mapa.proyecto) else {
            // Un mapa sin nodos visibles: no hay nada que encuadrar.
            self.lienzo.vista.desplazamiento = egui::Vec2::ZERO;
            self.lienzo.vista.zoom = 1.0;
            return;
        };

        let (zoom, desplazamiento) =
            crate::ui::canvas::encuadre_para_ver_el_mapa(caja, self.lienzo.vista.tamano_visible);

        self.lienzo.vista.zoom = zoom;
        self.lienzo.vista.desplazamiento = desplazamiento;
    }

    /// Añade un nodo hijo al nodo seleccionado y lo deja en edición.
    ///
    /// Si no hay ningún nodo seleccionado, cuelga de la raíz.
    ///
    /// Es lo que ocurre tras pulsar en el fondo del lienzo, que deselecciona: `Tab` sigue
    /// añadiendo, y lo añade al centro del mapa. La documentación decía que no hacía nada.
    pub fn anadir_hijo_al_seleccionado(&mut self) {
        if self.es_solo_lectura_por_raiz() {
            return;
        }
        let parent_id = self
            .mapa
            .nodo_seleccionado
            .unwrap_or(self.mapa.proyecto.root_id);
        let id_nuevo = self
            .mapa
            .proyecto
            .anadir_hijo(parent_id, Texto::NodoNuevaIdea.en(self.idioma()));
        if self.mapa.proyecto.layout_mode != ModoDisposicion::FreeDrag {
            crate::layout::aplicar_disposicion_automatica(&mut self.mapa.proyecto);
        }
        self.mapa.nodo_seleccionado = Some(id_nuevo);
        self.abrir_edicion_del_nodo(id_nuevo, Texto::NodoNuevaIdea.en(self.idioma()));
        self.establecer_estado(Texto::AvisoNodoHijoAnadido.en(self.idioma()));
    }

    /// Añade un nodo hermano del seleccionado y lo deja en edición.
    ///
    /// Si no hay ningún nodo seleccionado, cuelga de la raíz.
    ///
    /// Es lo que ocurre tras pulsar en el fondo del lienzo, que deselecciona: `Tab` sigue
    /// añadiendo, y lo añade al centro del mapa. La documentación decía que no hacía nada.
    pub fn anadir_hermano_al_seleccionado(&mut self) {
        if self.es_solo_lectura_por_raiz() {
            return;
        }
        let target_id = self
            .mapa
            .nodo_seleccionado
            .unwrap_or(self.mapa.proyecto.root_id);
        if let Some(id_nuevo) = self
            .mapa
            .proyecto
            .anadir_hermano(target_id, Texto::NodoNuevoHermano.en(self.idioma()))
        {
            if self.mapa.proyecto.layout_mode != ModoDisposicion::FreeDrag {
                crate::layout::aplicar_disposicion_automatica(&mut self.mapa.proyecto);
            }
            self.mapa.nodo_seleccionado = Some(id_nuevo);
            self.abrir_edicion_del_nodo(id_nuevo, Texto::NodoNuevoHermano.en(self.idioma()));
            self.establecer_estado(Texto::AvisoNodoHermanoAnadido.en(self.idioma()));
        }
    }

    /// Elimina el nodo seleccionado junto con toda su descendencia.
    ///
    /// La raíz no se elimina: en su lugar se vacía de hijos. Tras el borrado, la selección
    /// pasa al nodo raíz.
    pub fn eliminar_nodo_seleccionado(&mut self) {
        if self.es_solo_lectura_por_raiz() {
            return;
        }
        if let Some(id) = self.mapa.nodo_seleccionado {
            if id == self.mapa.proyecto.root_id {
                self.establecer_estado(Texto::AvisoNoEliminarRaiz.en(self.idioma()));
                return;
            }
            let parent_id = self.mapa.proyecto.nodes.get(&id).and_then(|n| n.parent_id);
            self.mapa.proyecto.eliminar_nodo(id);
            if self.mapa.proyecto.layout_mode != ModoDisposicion::FreeDrag {
                crate::layout::aplicar_disposicion_automatica(&mut self.mapa.proyecto);
            }
            self.mapa.nodo_seleccionado = parent_id.or(Some(self.mapa.proyecto.root_id));
            self.lienzo.edicion.nodo = None;
            self.establecer_estado(Texto::AvisoNodoEliminado.en(self.idioma()));
        }
    }
}

impl eframe::App for AplicacionMapaMental {
    /// Un fotograma entero de la aplicación.
    ///
    /// `egui` funciona en modo inmediato: esto no se llama una vez al arrancar, sino entre
    /// treinta y sesenta veces por segundo, y **toda** la interfaz se construye de cero en
    /// cada pasada. Por eso el orden importa y por eso nada de lo que se dibuja puede
    /// guardar estado por su cuenta: lo que tenga que sobrevivir al fotograma vive en
    /// [`AplicacionMapaMental`].
    ///
    /// El orden en el que ocurren las cosas aquí no es casual:
    ///
    /// 1. **Antes de dibujar nada**: la escala de la interfaz, el autoguardado y la
    ///    petición de repintado. Esta última no es opcional: `egui` solo repinta cuando hay
    ///    actividad, y sin ella una ventana inactiva dejaría de recibir fotogramas y el
    ///    autoguardado no llegaría a dispararse. Justo el caso en el que más falta hace,
    ///    con el usuario ausente.
    /// 2. **El aviso de recuperación, el primero de todo lo visible**: si quedó trabajo de
    ///    una sesión anterior, hay que decidir sobre él antes de seguir tocando nada.
    /// 3. **Atajos y vigilancia**, que pueden cambiar el mapa antes de que se pinte.
    /// 4. **Los paneles**, de fuera hacia dentro: barra, estado, inspector y por último el
    ///    lienzo, que ocupa lo que quede.
    /// 5. **El historial, al final**, cuando ya han pasado por el mapa el lienzo, el
    ///    inspector y los modales. Si algo cambió, anota el estado anterior.
    ///
    /// # Parámetros
    /// - `ui`: el `Ui` raíz del fotograma.
    /// - `_frame`: el marco de `eframe`. No se usa: la aplicación no necesita cambiar el
    ///   tamaño de la ventana ni cerrarla desde aquí.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Escala de la interfaz: aplica la preferencia guardada y recoge los cambios que
        // el usuario haya hecho con Ctrl + / Ctrl - / Ctrl 0.
        self.sincronizar_escala_interfaz(&ctx);

        // Autoguardado: comprueba si toca escribir la copia de seguridad. En la inmensa
        // mayoría de los fotogramas se limita a comparar dos instantes y salir.
        self.procesar_autoguardado();

        // `egui` solo repinta cuando hay actividad. Sin esta petición, una ventana
        // inactiva dejaría de recibir fotogramas y el autoguardado no llegaría a
        // dispararse: justo el caso en el que más falta hace, con el usuario ausente.
        ctx.request_repaint_after(self.persistencia.autoguardado.intervalo());

        // Aviso de recuperación, antes que nada: si hay trabajo de una sesión anterior,
        // el usuario debe decidir sobre él antes de seguir.
        ai_modal::dibujar_modal_recuperacion(self, &ctx);

        // Ventana de conexión con agentes de IA.
        conexiones_modal::dibujar_modal_conexiones(self, &ctx);
        self.procesar_eventos_de_envio(&ctx);
        sesion_agente_modal::dibujar_modal_sesion_agente(self, &ctx);

        // Atajos de teclado
        self.atender_atajos_de_teclado(&ctx);

        // Atiende eventos de vigilancia externa de archivos (.mmcelt modificado por IA)
        self.procesar_eventos_de_vigilancia();

        // Top Toolbar
        toolbar::dibujar_barra_de_herramientas(self, ui);

        self.dibujar_barra_de_estado(ui);

        // Inspector de la derecha
        sidebar::dibujar_panel_lateral(self, ui);

        // El lienzo infinito, en el centro
        canvas::dibujar_lienzo(self, ui);

        // Diálogos y ventanas emergentes
        ai_modal::dibujar_modales(self, &ctx);
        crate::guardado_como::dibujar_modal_guardar_como(self, &ctx);

        // El historial mira el mapa al final del fotograma, cuando ya han pasado por él el
        // lienzo, el inspector y los modales. Si algo ha cambiado, anota el estado anterior.
        // En la inmensa mayoría de los fotogramas se queda en comparar dos números.
        self.mapa.historial.observar_si_cambio(
            &self.mapa.proyecto,
            self.mapa.proyecto.revision(),
            std::time::Instant::now(),
        );
    }
}

/// Decide si los atajos que alteran la estructura del mapa pueden actuar.
///
/// Los que crean, borran o abren un título en edición (`Tab`, `Enter`, `Supr`, `Retroceso`,
/// `Espacio`, `F2`) solo valen con el teclado libre. Los de archivo y vista (`Ctrl+S`,
/// `Ctrl+E`, `Ctrl+F`) no pasan por aquí: siguen valiendo mientras se escribe.
///
/// Está aparte, y recibe tres `bool` en vez del contexto de `egui`, para poder comprobarla sin
/// levantar una ventana. La batería no ejercita la interfaz, y esta regla estuvo mal escrita
/// —con una `y` donde debía haber una `o`— sin que nada lo advirtiera: bastaba pulsar
/// `Retroceso` dentro de las notas de un nodo para borrarlo con toda su descendencia.
///
/// # Las tres condiciones cubren tres agujeros distintos
///
/// Las dos primeras miran si algo está recibiendo texto. La tercera no, y por eso hizo falta
/// añadirla: **una ventana sin ningún campo de escritura no pide el teclado**, así que con
/// «Ver Todos los Atajos», «Acerca de» o la de conexión con agentes abiertas, las dos primeras
/// decían que el teclado estaba libre y `Supr` llegaba hasta el mapa de detrás. El usuario
/// perdía el nodo que tuviera seleccionado, con toda su descendencia, mientras miraba una
/// ventana que no tiene nada que ver.
///
/// # Parámetros
/// - `quiere_teclado`: si algún campo de la interfaz tiene el foco de texto
///   (`egui::Context::wants_keyboard_input`).
/// - `hay_nodo_en_edicion`: si se está editando el título de un nodo en el lienzo. Cubre el
///   fotograma en que la edición acaba de empezar y el campo todavía no pide el teclado.
/// - `hay_ventana_abierta`: si hay cualquier ventana modal delante del mapa, tenga campos de
///   texto o no. Lo da [`AplicacionMapaMental::hay_alguna_ventana_abierta`].
///
/// # Retorno
/// `true` solo si el mapa está al frente y nadie está escribiendo.
pub(crate) fn atajos_de_estructura_permitidos(
    quiere_teclado: bool,
    hay_nodo_en_edicion: bool,
    hay_ventana_abierta: bool,
) -> bool {
    !quiere_teclado && !hay_nodo_en_edicion && !hay_ventana_abierta
}

impl AplicacionMapaMental {
    /// La barra inferior: a la izquierda lo que acaba de pasar, a la derecha qué y cuánto hay.
    ///
    /// # Qué manda cuando hay dos cosas que decir
    ///
    /// El aviso de cambio externo tiene prioridad sobre el mensaje de estado, y no es un
    /// detalle: significa que el archivo ha cambiado en el disco **y** que hay trabajo local
    /// sin guardar. Es lo único que se dice ahí que puede acabar en pérdida de trabajo, así
    /// que no puede quedar tapado por un «Mapa guardado» de hace tres segundos.
    ///
    /// # Parámetros
    /// - `ui`: el `Ui` en el que se ancla el panel inferior.
    fn dibujar_barra_de_estado(&mut self, ui: &mut egui::Ui) {
        /// Cuántos segundos se queda en pantalla un mensaje de estado antes de desaparecer.
        ///
        /// Suficiente para leerlo sin que se quede clavado: lo que cuenta son avisos de una
        /// línea, no texto que haya que estudiar.
        const SEGUNDOS_DEL_MENSAJE: u64 = 8;

        egui::Panel::bottom("bottom_status_bar")
            .min_size(24.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.persistencia.cambio_externo_pendiente {
                        ui.label(
                            egui::RichText::new(format!(
                                "⚠️ {}",
                                crate::textos::Texto::AvisoCambioExternoConCambiosLocales
                                    .en(self.idioma())
                            ))
                            .small()
                            .color(self.presentacion.tema.peligro),
                        );
                    } else if let Some((mensaje, momento)) = &self.presentacion.mensaje_de_estado {
                        if momento.elapsed().as_secs() < SEGUNDOS_DEL_MENSAJE {
                            ui.label(egui::RichText::new(format!("ℹ️ {}", mensaje)).small());
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.identificacion_y_metricas(ui);
                    });
                });
            });
    }

    /// El extremo derecho de la barra de estado: qué compilación es esta y qué tamaño tiene
    /// el mapa.
    ///
    /// La identificación de la compilación va **siempre** visible. Con varios ejecutables por
    /// el disco —el de `target`, una copia portable, otra carpeta de trabajo— saber de un
    /// vistazo cuál se ha abierto ahorra bastante desconcierto. Al pulsarla se abre «Acerca
    /// de», que da el detalle completo para copiarlo en un informe de fallo.
    ///
    /// # Parámetros
    /// - `ui`: el `Ui` de la barra, ya dispuesto de derecha a izquierda.
    fn identificacion_y_metricas(&mut self, ui: &mut egui::Ui) {
        let respuesta = ui
            .label(
                egui::RichText::new(crate::version::resumen_corto())
                    .small()
                    .color(self.presentacion.tema.texto_atenuado),
            )
            .on_hover_text(crate::version::detalle_completo());

        if respuesta.clicked() {
            self.presentacion.ventanas.modal_acerca_de = true;
        }

        ui.label(
            egui::RichText::new("|")
                .small()
                .color(self.presentacion.tema.texto_atenuado),
        );

        let plantilla_metricas = Texto::BarraMetricasEstado.en(self.idioma());
        let texto_metricas = plantilla_metricas
            .replacen("{}", &self.mapa.proyecto.nodes.len().to_string(), 1)
            .replacen("{}", &self.mapa.proyecto.connections.len().to_string(), 1)
            .replace("{:.0}", &format!("{:.0}", self.lienzo.vista.zoom * 100.0));
        ui.label(
            egui::RichText::new(texto_metricas)
                .small()
                .color(self.presentacion.tema.texto_atenuado),
        );
    }

    /// Vuelve al estado anterior del mapa.
    ///
    /// Deja antes copia de recuperación, como cualquier otra sustitución del mapa abierto:
    /// lo que se deshace vive en el historial, que es memoria, y un cierre inesperado se lo
    /// llevaría por delante.
    ///
    /// No guarda el archivo. Deshacer cambia lo que hay en pantalla; guardar sigue siendo
    /// una decisión aparte.
    pub(crate) fn deshacer(&mut self) {
        let _ = self.guardar_copia_de_recuperacion();
        // Si había un título a medio escribir, se cierra por su puerta, que es la que
        // decide qué pasa con lo tecleado. Cerrarlo a mano aquí dejaba esa decisión
        // duplicada en dos sitios, y este proyecto ya sabe cómo acaba eso.
        self.confirmar_edicion_del_titulo();

        let Some(anterior) = self.mapa.historial.deshacer(&self.mapa.proyecto) else {
            return;
        };
        self.mapa.proyecto = anterior;
        self.lienzo.indice_espacial_del_lienzo.invalidar();
        self.recolocar_la_seleccion_tras_moverse_por_el_historial();
    }

    /// Recupera el estado que se había deshecho.
    ///
    /// Simétrica de [`Self::deshacer`], con las mismas cautelas.
    pub(crate) fn rehacer(&mut self) {
        let _ = self.guardar_copia_de_recuperacion();
        self.confirmar_edicion_del_titulo();

        let Some(siguiente) = self.mapa.historial.rehacer(&self.mapa.proyecto) else {
            return;
        };
        self.mapa.proyecto = siguiente;
        self.lienzo.indice_espacial_del_lienzo.invalidar();
        self.recolocar_la_seleccion_tras_moverse_por_el_historial();
    }

    /// Deja la selección en un nodo que exista después de moverse por el historial.
    ///
    /// Deshacer puede devolver el mapa a un momento en que el nodo seleccionado todavía no
    /// existía, y rehacer a uno en que ya se había borrado. Con la selección apuntando a un
    /// nodo ausente, el inspector se queda en blanco y los atajos de estructura actúan sobre
    /// nada sin decir por qué. Se cae a la raíz, que existe siempre.
    ///
    /// La edición que hubiera en curso ya viene cerrada por `confirmar_edicion_del_titulo`,
    /// que es la única puerta de salida de la edición del lienzo.
    fn recolocar_la_seleccion_tras_moverse_por_el_historial(&mut self) {
        let sigue_existiendo = self
            .mapa
            .nodo_seleccionado
            .is_some_and(|id| self.mapa.proyecto.nodes.contains_key(&id));
        if !sigue_existiendo {
            self.mapa.nodo_seleccionado = Some(self.mapa.proyecto.root_id);
        }
    }

    /// Sustituye el mapa abierto por otro, dejando antes una copia de recuperación del actual.
    ///
    /// Siete entradas de menú tiraban el mapa abierto sin preguntar y sin copia: crear uno
    /// nuevo, los dos ejemplos, las dos plantillas, escanear una carpeta y abrir un archivo.
    /// El usuario que llevaba una hora anotando y entraba en «📚 Ejemplos» para ver cómo se
    /// anota un mapa de verdad —que es justo para lo que están los ejemplos— perdía la hora.
    ///
    /// El peligro estaba identificado y resuelto en un solo sitio, la importación desde IA,
    /// con un comentario que lo explicaba. Ahora todos pasan por aquí.
    ///
    /// # Parámetros
    /// - `nuevo`: el mapa que pasa a estar en pantalla.
    /// - `archivo`: la ruta con la que queda asociado, o `None` si aún no tiene ninguna.
    /// - `aviso`: qué contar al usuario. Se le añade la suerte que corrió la copia, salvo
    ///   que el mapa anterior estuviera vacío y no hubiera nada que perder.
    pub fn sustituir_el_mapa_abierto(
        &mut self,
        nuevo: Proyecto,
        archivo: Option<PathBuf>,
        aviso: &str,
    ) {
        // Un mapa recién creado tiene solo su raíz. Mencionar entonces la copia sería ruido:
        // no había nada que salvar.
        let habia_trabajo = self.mapa.proyecto.nodes.len() > 1;
        let copia_guardada = self.guardar_copia_de_recuperacion().is_ok();

        self.mapa.proyecto = nuevo;
        self.lienzo.indice_espacial_del_lienzo.invalidar();
        // Los pasos del mapa anterior no valen para este. Deshacer sobre ellos traería de
        // vuelta un mapa que no es el que está abierto, y lo pisaría.
        self.mapa.historial.olvidar(&self.mapa.proyecto);
        // La copia en disco es el mapa **anterior**, a propósito. Sin esto, el autoguardado
        // la pisaría con el mapa nuevo en la primera comprobación.
        self.persistencia
            .autoguardado
            .dar_por_copiado(&self.mapa.proyecto);
        self.mapa.nodo_seleccionado = Some(self.mapa.proyecto.root_id);
        self.persistencia.ruta_actual = archivo.clone();
        self.sincronizar_espacio_de_trabajo_con_el_mapa();
        self.persistencia.vigilante = None;
        self.persistencia.cambio_externo_pendiente = false;
        self.persistencia.ultima_huella_guardada =
            archivo.as_deref().and_then(crate::proyectos::huella);
        self.centrar_en_la_raiz();

        // Detección temprana: si el mapa abierto reside en la raíz de una unidad o volumen,
        // no puede servir de espacio de trabajo seguro para agentes de IA. Mostramos el diálogo
        // guiado inmediatamente para que el usuario pueda moverlo, crear uno nuevo o declinar.
        if let Some(ref ruta) = archivo {
            if crate::conectores::es_mapa_en_raiz_de_volumen(ruta) {
                self.presentacion.aviso_mapa_en_raiz = Some(AvisoMapaEnRaiz {
                    ruta_mapa: ruta.clone(),
                });
            } else {
                self.presentacion.aviso_mapa_en_raiz = None;
            }
        } else {
            self.presentacion.aviso_mapa_en_raiz = None;
        }

        let mensaje = match (habia_trabajo, copia_guardada) {
            (false, _) => aviso.to_string(),
            (true, true) => format!(
                "{aviso} {}",
                crate::textos::Texto::AvisoCopiaDelAnterior.en(self.idioma())
            ),
            (true, false) => format!(
                "{aviso} {}",
                crate::textos::Texto::AvisoSinCopiaDelAnterior.en(self.idioma())
            ),
        };
        self.establecer_estado(mensaje);
    }

    /// Vuelve a colocar el mapa si lo coloca el programa.
    ///
    /// Hay que llamarla después de cambiar **el contenido** de un nodo, no solo su
    /// estructura: el ancho y el alto de una tarjeta salen de su título, de si tiene notas y
    /// de cuántas etiquetas lleva, así que escribir en cualquiera de esos campos cambia el
    /// sitio que ocupa.
    ///
    /// Antes solo se recolocaba al añadir o borrar nodos. Desde que la separación entre la
    /// idea central y sus pilares depende del ancho de la raíz, renombrarla dejaba el mapa
    /// mal repartido hasta la siguiente reorganización: las tarjetas se montaban, y en la
    /// franja solapada el clic seleccionaba un nodo y el arrastre se llevaba otro.
    ///
    /// En «Posición Libre Manual» no hace nada, que es lo que se espera: ahí los coloca el
    /// usuario.
    /// Entra en la edición del título de un nodo, en el lienzo.
    ///
    /// Existe para que **haya una sola puerta**. Seis sitios abren la edición —el doble
    /// clic, el botón de hijo rápido del lienzo, añadir hijo, añadir hermano, `F2` y la
    /// entrada del menú—, y cada uno tenía que acordarse de poner dos campos; ahora son
    /// tres, con la marca del foco. La lección repetida de este proyecto es que una regla
    /// que hay que recordar en seis sitios se olvida en alguno.
    ///
    /// # Parámetros
    /// - `id`: el nodo cuyo título se va a editar.
    /// - `titulo`: el texto con el que arranca el editor.
    pub fn abrir_edicion_del_nodo(&mut self, id: Uuid, titulo: impl Into<String>) {
        if self.es_solo_lectura_por_raiz() {
            return;
        }
        self.lienzo.edicion.nodo = Some(id);
        self.lienzo.edicion.texto = titulo.into();
        self.lienzo.edicion.foco_pendiente = true;
    }

    /// Cierra la edición del título **conservando lo escrito**.
    ///
    /// La otra puerta, la de salida. Cerrar la edición haciendo clic en el fondo del lienzo
    /// —el gesto natural para dar por bueno lo escrito— descartaba el texto sin avisar: se
    /// ponía `nodo_en_edicion` a `None` y el búfer se quedaba huérfano.
    ///
    /// Si el nodo ya no existe, o el título no ha cambiado, no se toca el mapa ni se
    /// recoloca: escribir y dejarlo igual no debería mover nada.
    pub fn confirmar_edicion_del_titulo(&mut self) {
        let Some(id) = self.lienzo.edicion.nodo.take() else {
            return;
        };
        let titulo = std::mem::take(&mut self.lienzo.edicion.texto);
        self.lienzo.edicion.foco_pendiente = false;

        let ha_cambiado = match self.mapa.proyecto.nodes.get_mut(&id) {
            Some(nodo) if nodo.title != titulo => {
                nodo.title = titulo;
                // Lo ha escrito una persona: el mapa deja de ser trabajo exclusivo de la IA
                // y el servidor tiene que dejar copia antes de sobrescribirlo.
                nodo.marcar_editado_por_una_persona();
                true
            }
            _ => false,
        };

        // El título decide el ancho de la tarjeta, así que cambiarlo cambia el sitio que
        // ocupa y hay que repartir el mapa otra vez.
        if ha_cambiado {
            self.mapa.proyecto.marcar_modificado();
            self.recolocar_si_procede();
        }
    }

    /// Vuelve a repartir el mapa, salvo que el usuario haya pedido colocar los nodos a mano.
    ///
    /// Se llama después de cualquier cambio que altere el tamaño de una tarjeta o la forma
    /// del árbol: escribir un título, añadir un nodo, plegar una rama. El «si procede» es la
    /// parte que importa: en [`ModoDisposicion::FreeDrag`] el usuario ha colocado los nodos
    /// donde quiere, y recolocarlos le desharía el trabajo en cuanto tocara cualquier cosa.
    pub fn recolocar_si_procede(&mut self) {
        if self.mapa.proyecto.layout_mode != ModoDisposicion::FreeDrag {
            crate::layout::aplicar_disposicion_automatica(&mut self.mapa.proyecto);
        }
    }

    /// El idioma en el que la aplicación se dirige al usuario.
    ///
    /// Sale de sus preferencias, que es donde se guarda entre sesiones. Existe como atajo
    /// porque se consulta en casi todos los rótulos que se dibujan: `Texto::X.en(app.idioma())`.
    pub fn idioma(&self) -> crate::textos::Idioma {
        self.presentacion.preferencias.idioma
    }

    /// Dice si hay alguna ventana modal delante del mapa.
    ///
    /// # Para qué existe
    ///
    /// Para que los atajos que alteran la estructura del mapa —`Supr`, `Tab`, `Enter`— no lo
    /// alcancen mientras el usuario está mirando otra cosa. No basta con preguntarle a `egui`
    /// si alguien quiere el teclado: **una ventana sin campos de escritura no lo pide**, y con
    /// «Ver Todos los Atajos» abierta un `Supr` borraba el nodo seleccionado del mapa de
    /// detrás, con toda su descendencia y sin preguntar.
    ///
    /// # Hay que añadir aquí cada ventana nueva
    ///
    /// La lista está escrita a mano y es lo único que separa un modal nuevo de volver a abrir
    /// ese agujero. La prueba `ninguna_ventana_se_queda_fuera_de_la_guarda_de_atajos` recorre
    /// los campos `mostrar_modal_*` de la estructura y falla si alguno no se consulta aquí.
    ///
    /// # Devuelve
    /// `true` si hay al menos una ventana abierta, incluida la de recuperación.
    pub(crate) fn hay_alguna_ventana_abierta(&self) -> bool {
        self.presentacion.ventanas.modal_importar
            || self.presentacion.ventanas.modal_prompt_maestro
            || self.presentacion.ventanas.modal_vista_previa
            || self.presentacion.ventanas.modal_correcciones
            || self.presentacion.ventanas.modal_atajos
            || self.presentacion.ventanas.modal_datos_del_proyecto
            || self.presentacion.ventanas.modal_conexion_cruzada
            || self.presentacion.ventanas.modal_conexiones
            || self.agentes.editor_sesion.is_some()
            || self.presentacion.ventanas.modal_acerca_de
            || self.presentacion.aviso_mapa_en_raiz.is_some()
            || self.presentacion.aviso_error_proyecto_ia.is_some()
            // No es un `mostrar_modal_*`, pero se dibuja igual: es el aviso de que hay una
            // copia de recuperación de una sesión anterior esperando decisión.
            || self.persistencia.recuperacion.is_some()
    }

    /// Muestra el diálogo modal visible de error al abrir «Proyecto e instrucciones».
    pub fn mostrar_aviso_error_proyecto_ia(&mut self, error: &crate::error::AppError) {
        match error {
            crate::error::AppError::CarpetaProyectoInvalida { ruta, motivo } => {
                self.presentacion.aviso_error_proyecto_ia = Some(AvisoErrorProyectoIa {
                    ruta: ruta.clone(),
                    motivo: motivo.clone(),
                });
            }
            _ => {
                self.presentacion.aviso_error_proyecto_ia = Some(AvisoErrorProyectoIa {
                    ruta: self.agentes.espacio_de_trabajo.clone(),
                    motivo: crate::error::MotivoCarpetaInvalida::ErrorGeneral(Box::new(
                        error.mensaje_usuario(),
                    )),
                });
            }
        }
    }

    /// Abre la ventana de proyecto e instrucciones en la pestaña especificada.
    pub fn abrir_proyecto_ia_en_pestana(
        &mut self,
        pestana: crate::ui::proyecto_ia_modal::PestanaProyectoIa,
    ) {
        if self.es_solo_lectura_por_raiz() {
            self.presentacion.ventanas.modal_datos_del_proyecto = false;
            *self.presentacion.editor_proyecto_ia() = None;
            self.abrir_aviso_mapa_en_raiz();
            return;
        }

        if let Some(editor) = self.presentacion.editor_proyecto_ia() {
            editor.pestana = pestana;
            self.presentacion.ventanas.modal_datos_del_proyecto = true;
            self.presentacion.aviso_error_proyecto_ia = None;
        } else {
            match crate::ui::proyecto_ia_modal::cargar_editor(self) {
                Ok(mut editor) => {
                    editor.pestana = pestana;
                    *self.presentacion.editor_proyecto_ia() = Some(editor);
                    self.presentacion.ventanas.modal_datos_del_proyecto = true;
                    self.presentacion.aviso_error_proyecto_ia = None;
                }
                Err(error) => {
                    self.presentacion.ventanas.modal_datos_del_proyecto = false;
                    *self.presentacion.editor_proyecto_ia() = None;
                    crate::error::registrar(&error, "abrir proyecto e instrucciones para la IA");
                    let idioma = self.idioma();
                    self.establecer_estado(error.mensaje_usuario().en(idioma));
                    if let Some(ref ruta) = self.persistencia.ruta_actual {
                        if crate::conectores::es_mapa_en_raiz_de_volumen(ruta) {
                            self.presentacion.aviso_mapa_en_raiz = Some(AvisoMapaEnRaiz {
                                ruta_mapa: ruta.clone(),
                            });
                        } else {
                            self.mostrar_aviso_error_proyecto_ia(&error);
                        }
                    } else {
                        self.mostrar_aviso_error_proyecto_ia(&error);
                    }
                }
            }
        }
    }

    /// Procesa los atajos de teclado globales de la aplicación.
    ///
    /// Los atajos van en dos grupos, y la diferencia importa:
    ///
    /// - Los **de archivo y vista** (`Ctrl+S`, `Ctrl+E`, `Ctrl+F`) no alteran la estructura
    ///   del mapa ni roban el foco, así que siguen valiendo mientras se escribe.
    /// - Los **de estructura** (`Tab`, `Enter`, `Supr`, `Retroceso`, `Espacio`, `F2`) crean,
    ///   borran o abren un título en edición, y se ignoran en cuanto cualquier campo de
    ///   texto tiene el foco.
    ///
    /// La guarda pregunta dos cosas distintas a propósito. `wants_keyboard_input` cubre todos
    /// los campos de texto de la aplicación —el inspector, los modales, la ficha del
    /// proyecto—, y `nodo_en_edicion` cubre el fotograma en que se acaba de entrar en edición en
    /// el lienzo: el campo todavía no se ha dibujado, así que aún no pide el teclado.
    ///
    /// Antes se exigían **las dos a la vez**, con lo que ningún campo fuera del lienzo estaba
    /// protegido: pulsar `Retroceso` para corregir una letra en «Explicación y Contexto para
    /// la IA» borraba el nodo que se estaba anotando y toda su descendencia, sin preguntar.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` del que se leen los eventos de entrada.
    pub(crate) fn atender_atajos_de_teclado(&mut self, ctx: &egui::Context) {
        let permitir_estructura = atajos_de_estructura_permitidos(
            ctx.egui_wants_keyboard_input(),
            self.lienzo.edicion.nodo.is_some(),
            self.hay_alguna_ventana_abierta(),
        );

        // Lo que abre un diálogo del sistema se anota aquí y se ejecuta al salir del cierre.
        // Son llamadas bloqueantes, y hacerlas mientras `egui` mantiene tomado el cerrojo de
        // entrada es pedir un interbloqueo; como poco, la ventana se queda congelada todo el
        // rato que el usuario tarde en navegar por sus carpetas.
        let mut guardar_solicitado = false;
        let mut exportar_solicitado = false;
        let mut nuevo_mapa_solicitado = false;

        ctx.input(|i| {
            // Archivo y vista: valen siempre.
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                guardar_solicitado = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::E) {
                exportar_solicitado = true;
            }
            if i.modifiers.command && i.key_pressed(egui::Key::F) {
                self.centrar_en_la_raiz();
            }
            // Deshacer y rehacer valen aunque se esté escribiendo: son de archivo, no de
            // estructura, y quien acaba de equivocarse escribiendo es justo quien los busca.
            if !self.es_solo_lectura_por_raiz() {
                if i.modifiers.command && !i.modifiers.shift && i.key_pressed(egui::Key::Z) {
                    self.deshacer();
                }
                if i.modifiers.command
                    && (i.key_pressed(egui::Key::Y)
                        || (i.modifiers.shift && i.key_pressed(egui::Key::Z)))
                {
                    self.rehacer();
                }
            }

            if !permitir_estructura || self.es_solo_lectura_por_raiz() {
                return;
            }

            // Estructura del mapa: solo con el teclado libre.
            if i.modifiers.command && i.key_pressed(egui::Key::N) {
                nuevo_mapa_solicitado = true;
            }
            if i.key_pressed(egui::Key::Tab) && !i.modifiers.shift {
                self.anadir_hijo_al_seleccionado();
            }
            if i.key_pressed(egui::Key::Enter) && !i.modifiers.any() {
                self.anadir_hermano_al_seleccionado();
            }
            if i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace) {
                self.eliminar_nodo_seleccionado();
            }
            if i.key_pressed(egui::Key::Space) || i.key_pressed(egui::Key::F2) {
                self.abrir_el_titulo_en_edicion();
            }
        });

        if guardar_solicitado {
            self.solicitar_guardado(false);
        }
        if exportar_solicitado {
            self.exportar_markdown_dialogo();
        }
        if nuevo_mapa_solicitado {
            // Sale del cierre por lo mismo que los otros dos: deja copia de recuperación,
            // que escribe en disco.
            self.sustituir_el_mapa_abierto(
                Proyecto::nuevo_vacio(Texto::ArchivoNuevoProyectoDefecto.en(self.idioma())),
                None,
                crate::textos::Texto::AvisoProyectoNuevo.en(self.idioma()),
            );
        }
    }

    /// Pone el título del nodo seleccionado en edición, si hay alguno seleccionado.
    ///
    /// El texto de partida es el título actual: se edita sobre una copia y solo se traslada
    /// al nodo cuando el usuario confirma.
    ///
    /// # Sin nodo seleccionado
    ///
    /// `Espacio` y `F2` llegan aquí se haya elegido un nodo o no, porque son atajos globales.
    /// Sin nodo la acción **no hace nada, y eso está bien**: es inocua y no toca el mapa. Lo
    /// que no estaba bien es que además fuera **muda**, porque desde fuera no se distingue de
    /// que el atajo esté roto. Ahora se dice en la barra de estado.
    ///
    /// El aviso es el mismo `AvisoSeleccionaUnNodo` que ya usaban las otras acciones que
    /// necesitan un nodo elegido, para que el programa no invente una frase distinta cada vez
    /// que le falta lo mismo.
    pub(crate) fn abrir_el_titulo_en_edicion(&mut self) {
        if self.es_solo_lectura_por_raiz() {
            return;
        }
        let Some(id) = self.mapa.nodo_seleccionado else {
            self.establecer_estado(crate::textos::Texto::AvisoSeleccionaUnNodo.en(self.idioma()));
            return;
        };
        let Some(nodo) = self.mapa.proyecto.nodes.get(&id) else {
            self.establecer_estado(crate::textos::Texto::AvisoSeleccionaUnNodo.en(self.idioma()));
            return;
        };
        let titulo = nodo.title.clone();
        self.abrir_edicion_del_nodo(id, titulo);
    }

    /// Indica si el mapa activo se encuentra directamente en la raíz de una unidad o volumen.
    ///
    /// Por decisión de diseño (2026-09-16), los mapas en la raíz permanecen en modo de
    /// solo lectura para impedir accesos involuntarios a la unidad completa y comportamientos
    /// no deseados. La aplicación guía a la persona para mover el mapa a una carpeta dedicada.
    pub fn es_solo_lectura_por_raiz(&self) -> bool {
        self.persistencia
            .ruta_actual
            .as_ref()
            .is_some_and(|r| crate::conectores::es_mapa_en_raiz_de_volumen(r))
    }

    /// Abre el diálogo explicativo temprano si el mapa activo está en la raíz de una unidad o volumen.
    pub fn abrir_aviso_mapa_en_raiz(&mut self) {
        if let Some(ruta) = &self.persistencia.ruta_actual {
            if crate::conectores::es_mapa_en_raiz_de_volumen(ruta) {
                self.presentacion.aviso_mapa_en_raiz = Some(AvisoMapaEnRaiz {
                    ruta_mapa: ruta.clone(),
                });
            }
        }
    }
}
