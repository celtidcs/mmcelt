//! # Módulo de Temas Visuales (`theme.rs`)
//!
//! Define las tres paletas de la aplicación y las traduce al estilo que `egui` aplica a
//! toda la interfaz.
//!
//! - **Oscuro**: azul pizarra, pensado para sesiones largas con luz baja.
//! - **Claro**: papel cálido; el fondo es marfil en lugar de blanco, y los nodos son
//!   blancos, de modo que flotan sobre el lienzo como fichas sobre una mesa.
//! - **Alto contraste**: negro absoluto y colores saturados para máxima legibilidad.
//!
//! ## Una paleta, toda la ventana
//!
//! El punto importante de este módulo es [`ThemeConfig::visuals`]. El lienzo se pinta a
//! mano con los colores de [`ThemeConfig`], pero la barra de menús, el panel lateral, la
//! barra de estado, los botones, los desplegables y los cuadros de diálogo los pinta
//! `egui` con su propio estilo.
//!
//! Mientras nadie le dio un estilo propio, `egui` usaba el suyo por omisión, que es
//! oscuro. El tema oscuro parecía correcto por casualidad, y el claro se veía partido: un
//! lienzo claro rodeado de barras grises oscuras. [`ThemeConfig::visuals`] cierra ese
//! hueco traduciendo la paleta al estilo de `egui`, y la aplicación lo instala al arrancar
//! y cada vez que se cambia de tema.
//!
//! ## Los colores son opacos a propósito
//!
//! Ningún color de la paleta lleva transparencia, salvo la sombra. La transparencia se
//! aplica, si hace falta, en el punto donde se dibuja. El motivo está explicado en
//! [`ThemeConfig::linea_de_la_reticula`] y lo respalda una prueba automática.
//!
//! ## Organización del módulo
//!
//! 1. Las constantes de forma (redondeos, grosores y sombras), para que ningún número
//!    suelto aparezca dentro de la lógica.
//! 2. [`AppThemeMode`], el modo elegido por el usuario.
//! 3. [`ThemeConfig`], la paleta, con una función constructora por tema.
//! 4. La traducción al estilo de `egui`, repartida en piezas pequeñas que [`ThemeConfig::visuals`]
//!    compone.

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{Color32, CornerRadius, Shadow, Stroke, Visuals};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constantes de forma
// ---------------------------------------------------------------------------

/// Radio de redondeo de botones, campos y demás controles, en puntos.
const REDONDEO_CONTROL: u8 = 5;

/// Radio de redondeo de ventanas y cuadros de diálogo, en puntos.
const REDONDEO_VENTANA: u8 = 8;

/// Radio de redondeo de los menús desplegables, en puntos.
const REDONDEO_MENU: u8 = 6;

/// Grosor del borde de un control en reposo, en puntos.
const GROSOR_BORDE: f32 = 1.0;

/// Grosor del borde de un control pulsado o con el foco del teclado, en puntos.
///
/// Es mayor que [`GROSOR_BORDE`] para que el foco se distinga por su forma y no solo por
/// su color, que es lo que exige un uso accesible.
const GROSOR_BORDE_FOCO: f32 = 2.0;

/// Cuánto crece un control cuando el puntero se posa sobre él, en puntos.
///
/// Es una respuesta que se percibe aunque el usuario no distinga bien los cambios de color.
const EXPANSION_AL_SENALAR: f32 = 1.0;

/// Expansión de un control que no está siendo señalado: ninguna.
const SIN_EXPANSION: f32 = 0.0;

/// Opacidad del resaltado del texto seleccionado, sobre el color de énfasis.
///
/// Por debajo de la unidad para que el texto siga leyéndose a través del resaltado.
const OPACIDAD_SELECCION_TEXTO: f32 = 0.45;

/// Desplazamiento vertical de la sombra de una ventana, en puntos.
const DESPLAZAMIENTO_SOMBRA_VENTANA: [i8; 2] = [0, 4];

/// Anchura del difuminado de la sombra de una ventana, en puntos.
const DIFUMINADO_SOMBRA_VENTANA: u8 = 16;

/// Desplazamiento vertical de la sombra de un menú desplegable, en puntos.
///
/// Menor que el de una ventana: un menú se percibe como una capa más pegada a su origen.
const DESPLAZAMIENTO_SOMBRA_MENU: [i8; 2] = [0, 2];

/// Anchura del difuminado de la sombra de un menú desplegable, en puntos.
const DIFUMINADO_SOMBRA_MENU: u8 = 10;

/// Ensanchamiento de la sombra más allá de la figura que la proyecta: ninguno.
const SIN_ENSANCHAMIENTO_DE_SOMBRA: u8 = 0;

// ---------------------------------------------------------------------------
// Modo de tema
// ---------------------------------------------------------------------------

/// Modos de tema visual soportados por la aplicación.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppThemeMode {
    #[default]
    Dark,
    Light,
    HighContrast,
}

impl AppThemeMode {
    /// Todos los modos, en el orden en que se ofrecen al usuario.
    ///
    /// Tenerlos en un solo sitio evita que el menú de la barra de herramientas y las
    /// pruebas mantengan cada uno su propia lista y se desincronicen al añadir un tema.
    pub const TODOS: [AppThemeMode; 3] = [
        AppThemeMode::Dark,
        AppThemeMode::Light,
        AppThemeMode::HighContrast,
    ];

    /// Nombre visible en el selector de tema de la barra de herramientas.
    ///
    /// # Parámetros
    /// - `idioma`: el idioma elegido por el usuario.
    ///
    /// # Devuelve
    /// El nombre del tema con su emoji correspondiente, en el idioma solicitado.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (AppThemeMode::Dark, Idioma::Espanol) => "🌙 Tema Oscuro",
            (AppThemeMode::Light, Idioma::Espanol) => "☀️ Tema Claro",
            (AppThemeMode::HighContrast, Idioma::Espanol) => "👁️ Alto Contraste",

            (AppThemeMode::Dark, Idioma::Ingles) => "🌙 Dark Theme",
            (AppThemeMode::Light, Idioma::Ingles) => "☀️ Light Theme",
            (AppThemeMode::HighContrast, Idioma::Ingles) => "👁️ High Contrast",

            (AppThemeMode::Dark, Idioma::Frances) => "🌙 Thème Sombre",
            (AppThemeMode::Light, Idioma::Frances) => "☀️ Thème Clair",
            (AppThemeMode::HighContrast, Idioma::Frances) => "👁️ Contraste élevé",

            (AppThemeMode::Dark, Idioma::Aleman) => "🌙 Dunkles Thema",
            (AppThemeMode::Light, Idioma::Aleman) => "☀️ Helles Thema",
            (AppThemeMode::HighContrast, Idioma::Aleman) => "👁️ Hoher Kontrast",

            (AppThemeMode::Dark, Idioma::Ruso) => "🌙 Тёмная тема",
            (AppThemeMode::Light, Idioma::Ruso) => "☀️ Светлая тема",
            (AppThemeMode::HighContrast, Idioma::Ruso) => "👁️ Высокий контраст",

            (AppThemeMode::Dark, Idioma::ChinoSimplificado) => "🌙 深色主题",
            (AppThemeMode::Light, Idioma::ChinoSimplificado) => "☀️ 浅色主题",
            (AppThemeMode::HighContrast, Idioma::ChinoSimplificado) => "👁️ 高对比度",
        }
    }

    /// Indica si el tema es de fondo oscuro con texto claro.
    ///
    /// `egui` usa este dato para decidir detalles menores, como el tono de las sombras.
    ///
    /// # Devuelve
    /// `true` para los temas oscuro y de alto contraste; `false` para el claro.
    pub fn es_oscuro(&self) -> bool {
        !matches!(self, AppThemeMode::Light)
    }
}

// ---------------------------------------------------------------------------
// Paleta
// ---------------------------------------------------------------------------

/// Configuración cromática completa de colores para el lienzo y paneles.
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    /// Variante de tema de la que se ha derivado toda la paleta.
    pub modo: AppThemeMode,

    /// Color de fondo del lienzo.
    pub fondo_del_lienzo: Color32,

    /// Color de las líneas de la retícula de fondo.
    ///
    /// Es un color **opaco**, calculado para quedar a medio camino entre el fondo del
    /// lienzo y el texto: se ve, pero no compite con los nodos.
    ///
    /// # Por qué opaco y no semitransparente
    ///
    /// Antes se definía con `Color32::from_rgba_premultiplied(255, 255, 255, 12)`,
    /// buscando un blanco muy tenue. El resultado era el contrario: **una línea blanca
    /// brillante** que destacaba más que los propios nodos.
    ///
    /// El motivo es que ese constructor espera los componentes **ya multiplicados por la
    /// transparencia**. Al pasarle 255 con transparencia 12, la composición queda en
    /// `255 + fondo × 0,95`, que satura a blanco puro. Para un blanco al 5 % habría que
    /// haber escrito `(12, 12, 12, 12)`, o bien usar `from_rgba_unmultiplied`.
    ///
    /// Se optó por un color opaco: el resultado es idéntico sobre un fondo conocido y no
    /// depende de entender la premultiplicación para ajustarlo. Una prueba automática
    /// impide que vuelva a colarse un color semitransparente en la paleta.
    pub linea_de_la_reticula: Color32,

    /// Fondo de un nodo corriente.
    pub fondo_del_nodo: Color32,

    /// Fondo del nodo raíz, un escalón por encima del resto para reconocerlo de un vistazo.
    pub fondo_de_la_raiz: Color32,

    /// Borde de un nodo en reposo.
    pub borde_del_nodo: Color32,

    /// Borde de un nodo seleccionado.
    pub borde_del_nodo_seleccionado: Color32,

    /// Color del texto principal: títulos de nodo y contenido.
    pub texto_principal: Color32,

    /// Color del texto de apoyo: notas, descripciones y controles en reposo.
    pub texto_secundario: Color32,

    /// Color del texto de menor importancia: recuentos, marcas de tiempo y pistas.
    pub texto_atenuado: Color32,

    /// Color de las conexiones que cruzan ramas distintas.
    pub linea_de_conexion_cruzada: Color32,

    /// Fondo de las etiquetas que acompañan a un nodo.
    pub fondo_de_la_insignia: Color32,

    /// Fondo de la barra de menús, el panel lateral y la barra de estado.
    pub fondo_del_panel: Color32,

    /// Fondo de los controles interactivos en reposo: botones, campos y desplegables.
    pub superficie: Color32,

    /// Fondo de un control con el puntero encima.
    pub superficie_con_cursor: Color32,

    /// Fondo de un control pulsado o con un menú abierto.
    pub superficie_activa: Color32,

    /// Color de las líneas divisorias y de los bordes de los controles.
    pub separador: Color32,

    /// Color de énfasis: selección de texto, foco del teclado y elementos activos.
    pub acento: Color32,

    /// Color de lo que se dibuja **encima** de un color de rama.
    ///
    /// Existe porque las ramas invierten su claridad de un tema a otro: en el oscuro son
    /// colores vivos sobre fondo apagado, y en el claro son colores profundos sobre fondo
    /// marfil. Un blanco fijo se lee sobre las ramas del tema claro y desaparece sobre las
    /// del oscuro; con el negro pasa justo lo contrario.
    pub texto_sobre_la_rama: Color32,

    /// Color de lo que ha salido bien: confirmaciones y acciones que completan algo.
    ///
    /// Forma parte de la paleta, y no de cada pantalla, para que cambie con el tema. Los
    /// verdes, ámbares y rojos vivos que se ven bien sobre un fondo oscuro quedan
    /// ilegibles sobre el papel cálido del tema claro.
    pub exito: Color32,

    /// Color de las advertencias: avisos que no impiden seguir.
    pub aviso: Color32,

    /// Color de lo destructivo o erróneo: eliminar, errores y correcciones exigidas.
    pub peligro: Color32,

    /// Color de la sombra de ventanas y menús.
    ///
    /// Es el **único** color de la paleta con transparencia, porque una sombra sin
    /// transparencia no es una sombra.
    pub sombra: Color32,

    /// Colores asignados cíclicamente a las ramas principales del mapa.
    pub colores_de_las_ramas: Vec<Color32>,
}

impl ThemeConfig {
    /// Construye la paleta completa correspondiente al modo indicado.
    ///
    /// # Parámetros
    /// - `mode`: modo claro, oscuro o de alto contraste.
    ///
    /// # Devuelve
    /// La configuración de colores lista para usar en toda la interfaz.
    pub fn new(mode: AppThemeMode) -> Self {
        match mode {
            AppThemeMode::Dark => Self::paleta_oscura(),
            AppThemeMode::Light => Self::paleta_clara(),
            AppThemeMode::HighContrast => Self::paleta_alto_contraste(),
        }
    }

    /// Paleta del tema oscuro: azul pizarra.
    ///
    /// Las tres superficies —lienzo, panel y nodo— se escalonan de oscuro a claro, de modo
    /// que la profundidad se lee sin necesidad de bordes marcados: el panel se eleva sobre
    /// el mapa y el nodo se eleva sobre el panel.
    ///
    /// # Devuelve
    /// La paleta del modo [`AppThemeMode::Dark`].
    fn paleta_oscura() -> Self {
        Self {
            modo: AppThemeMode::Dark,
            fondo_del_lienzo: Color32::from_rgb(18, 22, 31),
            linea_de_la_reticula: Color32::from_rgb(42, 51, 70),
            fondo_del_nodo: Color32::from_rgb(35, 43, 60),
            fondo_de_la_raiz: Color32::from_rgb(46, 58, 82),
            borde_del_nodo: Color32::from_rgb(51, 61, 82),
            borde_del_nodo_seleccionado: Color32::from_rgb(96, 165, 250),
            texto_principal: Color32::from_rgb(238, 242, 248),
            texto_secundario: Color32::from_rgb(180, 192, 212),
            texto_atenuado: Color32::from_rgb(147, 157, 175),
            linea_de_conexion_cruzada: Color32::from_rgb(245, 158, 11),
            fondo_de_la_insignia: Color32::from_rgb(43, 52, 72),
            fondo_del_panel: Color32::from_rgb(29, 36, 52),
            superficie: Color32::from_rgb(38, 47, 66),
            superficie_con_cursor: Color32::from_rgb(48, 59, 82),
            superficie_activa: Color32::from_rgb(58, 71, 99),
            separador: Color32::from_rgb(51, 61, 82),
            acento: Color32::from_rgb(96, 165, 250),
            // Las ramas del tema oscuro son claras: encima va el color del lienzo.
            texto_sobre_la_rama: Color32::from_rgb(18, 22, 31),
            exito: Color32::from_rgb(52, 211, 153),
            aviso: Color32::from_rgb(251, 191, 36),
            peligro: Color32::from_rgb(252, 160, 160),
            sombra: Color32::from_black_alpha(96),
            colores_de_las_ramas: vec![
                Color32::from_rgb(59, 130, 246), // Azul
                Color32::from_rgb(16, 185, 129), // Esmeralda
                Color32::from_rgb(245, 158, 11), // Ámbar
                Color32::from_rgb(168, 85, 247), // Púrpura
                Color32::from_rgb(236, 72, 153), // Rosa
                Color32::from_rgb(45, 212, 191), // Turquesa
                Color32::from_rgb(249, 115, 22), // Naranja
                Color32::from_rgb(99, 102, 241), // Índigo
            ],
        }
    }

    /// Paleta del tema claro: papel cálido.
    ///
    /// El lienzo es marfil y no blanco. El blanco puro a pantalla completa cansa la vista
    /// y, sobre todo, deja a los nodos —que también son blancos— sin nada sobre lo que
    /// destacar. Con el fondo un punto por debajo, los nodos se leen como fichas apoyadas
    /// sobre una mesa, y la tinta gris cálida evita la dureza del negro sobre papel.
    ///
    /// # Devuelve
    /// La paleta del modo [`AppThemeMode::Light`].
    fn paleta_clara() -> Self {
        Self {
            modo: AppThemeMode::Light,
            fondo_del_lienzo: Color32::from_rgb(242, 239, 233),
            linea_de_la_reticula: Color32::from_rgb(224, 219, 209),
            fondo_del_nodo: Color32::from_rgb(255, 255, 255),
            fondo_de_la_raiz: Color32::from_rgb(245, 233, 208),
            borde_del_nodo: Color32::from_rgb(216, 209, 196),
            borde_del_nodo_seleccionado: Color32::from_rgb(194, 65, 12),
            texto_principal: Color32::from_rgb(42, 39, 33),
            texto_secundario: Color32::from_rgb(74, 68, 59),
            texto_atenuado: Color32::from_rgb(100, 94, 84),
            linea_de_conexion_cruzada: Color32::from_rgb(154, 92, 5),
            fondo_de_la_insignia: Color32::from_rgb(237, 231, 219),
            fondo_del_panel: Color32::from_rgb(229, 223, 211),
            superficie: Color32::from_rgb(243, 239, 231),
            superficie_con_cursor: Color32::from_rgb(255, 255, 255),
            superficie_activa: Color32::from_rgb(224, 216, 201),
            separador: Color32::from_rgb(216, 209, 196),
            acento: Color32::from_rgb(194, 65, 12),
            // Las ramas del tema claro son profundas: encima va el blanco de los nodos.
            texto_sobre_la_rama: Color32::from_rgb(255, 255, 255),
            exito: Color32::from_rgb(6, 95, 70),
            aviso: Color32::from_rgb(146, 64, 14),
            peligro: Color32::from_rgb(185, 28, 28),
            sombra: Color32::from_black_alpha(28),
            colores_de_las_ramas: vec![
                Color32::from_rgb(29, 78, 216),  // Azul
                Color32::from_rgb(4, 120, 87),   // Esmeralda
                Color32::from_rgb(180, 83, 9),   // Ámbar
                Color32::from_rgb(126, 34, 206), // Púrpura
                Color32::from_rgb(190, 24, 93),  // Rosa
                Color32::from_rgb(15, 118, 110), // Turquesa
                Color32::from_rgb(154, 52, 18),  // Naranja
                Color32::from_rgb(67, 56, 202),  // Índigo
            ],
        }
    }

    /// Paleta del tema de alto contraste: negro absoluto y colores saturados.
    ///
    /// Aquí la prioridad no es el descanso visual sino la legibilidad: los bordes son
    /// blancos y sólidos, y la retícula se mantiene deliberadamente marcada.
    ///
    /// # Devuelve
    /// La paleta del modo [`AppThemeMode::HighContrast`].
    fn paleta_alto_contraste() -> Self {
        Self {
            modo: AppThemeMode::HighContrast,
            fondo_del_lienzo: Color32::from_rgb(0, 0, 0),
            linea_de_la_reticula: Color32::from_rgb(105, 105, 105),
            fondo_del_nodo: Color32::from_rgb(20, 20, 20),
            fondo_de_la_raiz: Color32::from_rgb(38, 38, 38),
            borde_del_nodo: Color32::from_rgb(255, 255, 255),
            borde_del_nodo_seleccionado: Color32::from_rgb(255, 255, 0),
            texto_principal: Color32::from_rgb(255, 255, 255),
            texto_secundario: Color32::from_rgb(220, 220, 220),
            texto_atenuado: Color32::from_rgb(170, 170, 170),
            linea_de_conexion_cruzada: Color32::from_rgb(255, 255, 0),
            fondo_de_la_insignia: Color32::from_rgb(35, 35, 35),
            fondo_del_panel: Color32::from_rgb(26, 26, 26),
            superficie: Color32::from_rgb(26, 26, 26),
            superficie_con_cursor: Color32::from_rgb(51, 51, 51),
            superficie_activa: Color32::from_rgb(74, 74, 74),
            separador: Color32::from_rgb(255, 255, 255),
            acento: Color32::from_rgb(255, 255, 0),
            texto_sobre_la_rama: Color32::from_rgb(0, 0, 0),
            exito: Color32::from_rgb(0, 255, 0),
            aviso: Color32::from_rgb(255, 255, 0),
            peligro: Color32::from_rgb(255, 170, 170),
            sombra: Color32::from_black_alpha(160),
            colores_de_las_ramas: vec![
                Color32::from_rgb(0, 255, 255),   // Cyan
                Color32::from_rgb(0, 255, 0),     // Verde brillante
                Color32::from_rgb(255, 255, 0),   // Amarillo
                Color32::from_rgb(255, 128, 0),   // Naranja
                Color32::from_rgb(255, 0, 255),   // Magenta
                Color32::from_rgb(128, 255, 255), // Cyan claro
                Color32::from_rgb(255, 128, 128), // Rosa claro
                Color32::from_rgb(128, 255, 128), // Verde claro
            ],
        }
    }

    /// Devuelve el color asignado a una rama principal.
    ///
    /// Los colores se reparten cíclicamente, de modo que cualquier índice es válido por
    /// grande que sea: la rama número N reutiliza el color N módulo el total disponible.
    ///
    /// # Parámetros
    /// - `branch_index`: posición de la rama entre los hijos de la raíz.
    ///
    /// # Devuelve
    /// El color correspondiente a esa rama.
    pub fn color_de_la_rama(&self, branch_index: usize) -> Color32 {
        let len = self.colores_de_las_ramas.len();
        self.colores_de_las_ramas[branch_index % len]
    }

    // -----------------------------------------------------------------------
    // Traducción al estilo de egui
    // -----------------------------------------------------------------------

    /// Traduce la paleta al estilo con el que `egui` pinta el resto de la ventana.
    ///
    /// El lienzo se dibuja a mano con los campos de [`ThemeConfig`], pero todo lo demás
    /// —barra de menús, panel lateral, barra de estado, botones, desplegables, campos de
    /// texto y cuadros de diálogo— lo pinta `egui`. Sin este método, `egui` usa su estilo
    /// por omisión, que es oscuro, y el tema claro queda partido en dos.
    ///
    /// Se limita a componer las piezas que construyen los métodos auxiliares de esta
    /// sección.
    ///
    /// # Devuelve
    /// Los `Visuals` listos para instalar con [`ThemeConfig::instalar_en`].
    pub fn visuals(&self) -> Visuals {
        let mut visuals = if self.modo.es_oscuro() {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        visuals.widgets = self.estilo_de_los_controles();
        visuals.selection = self.estilo_de_la_seleccion();
        self.aplicar_superficies(&mut visuals);
        self.aplicar_sombras(&mut visuals);

        visuals
    }

    /// Construye el estilo de un control en uno de sus estados.
    ///
    /// Reúne en un solo sitio la forma común a todos los controles —el redondeo y la
    /// estructura— para que cada estado solo tenga que declarar en qué se diferencia.
    ///
    /// # Parámetros
    /// - `fondo`: color de relleno del control.
    /// - `texto`: trazo con el que se dibuja el texto y los iconos del control.
    /// - `borde`: trazo del contorno.
    /// - `expansion`: cuánto crece el control respecto a su tamaño en reposo.
    ///
    /// # Devuelve
    /// El estilo de ese estado concreto.
    fn estilo_control(
        fondo: Color32,
        texto: Stroke,
        borde: Stroke,
        expansion: f32,
    ) -> WidgetVisuals {
        WidgetVisuals {
            bg_fill: fondo,
            // `bg_fill` cubre los controles que siempre tienen relleno (casillas,
            // interruptores) y `weak_bg_fill` los que solo lo tienen a veces (botones).
            // La paleta no distingue entre ambos casos: un botón y una casilla del mismo
            // estado deben verse igual.
            weak_bg_fill: fondo,
            bg_stroke: borde,
            corner_radius: CornerRadius::same(REDONDEO_CONTROL),
            fg_stroke: texto,
            expansion,
        }
    }

    /// Construye el estilo de los controles en sus cinco estados.
    ///
    /// # Jerarquía del texto
    ///
    /// Los controles en reposo usan el texto secundario y pasan al principal en cuanto se
    /// señalan o se pulsan. Así el ojo distingue lo que está activo sin recurrir a más
    /// color, y ambos estados superan el contraste mínimo exigido.
    ///
    /// # Devuelve
    /// El juego completo de estados que espera `egui`.
    fn estilo_de_los_controles(&self) -> Widgets {
        let borde_apagado = Stroke::new(GROSOR_BORDE, self.separador);
        let borde_enfasis = Stroke::new(GROSOR_BORDE, self.acento);
        let borde_enfasis_grueso = Stroke::new(GROSOR_BORDE_FOCO, self.acento);
        let texto_principal = Stroke::new(GROSOR_BORDE, self.texto_principal);
        let texto_apagado = Stroke::new(GROSOR_BORDE, self.texto_secundario);

        Widgets {
            // Lo no interactivo abarca las etiquetas y el fondo de las ventanas: es el
            // color con el que se lee la mayor parte del texto de la aplicación.
            noninteractive: Self::estilo_control(
                self.fondo_del_panel,
                texto_principal,
                borde_apagado,
                SIN_EXPANSION,
            ),
            inactive: Self::estilo_control(
                self.superficie,
                texto_apagado,
                borde_apagado,
                SIN_EXPANSION,
            ),
            hovered: Self::estilo_control(
                self.superficie_con_cursor,
                texto_principal,
                borde_enfasis,
                EXPANSION_AL_SENALAR,
            ),
            active: Self::estilo_control(
                self.superficie_activa,
                Stroke::new(GROSOR_BORDE_FOCO, self.texto_principal),
                borde_enfasis_grueso,
                EXPANSION_AL_SENALAR,
            ),
            open: Self::estilo_control(
                self.superficie_activa,
                texto_principal,
                borde_enfasis,
                SIN_EXPANSION,
            ),
        }
    }

    /// Construye el estilo del texto seleccionado.
    ///
    /// # Devuelve
    /// El resaltado y el trazo con que `egui` marca una selección.
    fn estilo_de_la_seleccion(&self) -> Selection {
        Selection {
            bg_fill: self.acento.gamma_multiply(OPACIDAD_SELECCION_TEXTO),
            stroke: Stroke::new(GROSOR_BORDE, self.texto_principal),
        }
    }

    /// Asigna los fondos de paneles, ventanas y campos de texto.
    ///
    /// # Parámetros
    /// - `visuals`: estilo en construcción, que se modifica en el sitio.
    fn aplicar_superficies(&self, visuals: &mut Visuals) {
        visuals.panel_fill = self.fondo_del_panel;
        visuals.window_fill = self.fondo_del_panel;
        visuals.window_stroke = Stroke::new(GROSOR_BORDE, self.separador);
        visuals.window_corner_radius = CornerRadius::same(REDONDEO_VENTANA);
        visuals.menu_corner_radius = CornerRadius::same(REDONDEO_MENU);

        // Los campos de texto y las barras de desplazamiento necesitan un fondo que se
        // distinga del panel; se toma el del lienzo, que es la superficie más profunda.
        visuals.extreme_bg_color = self.fondo_del_lienzo;
        visuals.faint_bg_color = self.fondo_de_la_insignia;
        visuals.code_bg_color = self.fondo_de_la_insignia;

        visuals.hyperlink_color = self.acento;

        // `egui` reserva estos dos para sus propios mensajes de aviso y error. Traerlos a
        // la paleta cierra el ultimo hueco por el que se colaba su estilo de fabrica: en
        // el tema claro eran un rojo y un naranja puros que no llegaban al minimo legible
        // sobre el papel calido.
        visuals.warn_fg_color = self.aviso;
        visuals.error_fg_color = self.peligro;
    }

    /// Construye una sombra de la paleta con la geometría indicada.
    ///
    /// # Parámetros
    /// - `desplazamiento`: cuánto se mueve la sombra respecto a la figura, en puntos.
    /// - `difuminado`: anchura de la penumbra, en puntos.
    ///
    /// # Devuelve
    /// La sombra lista para asignar a un elemento del estilo.
    fn sombra(&self, desplazamiento: [i8; 2], difuminado: u8) -> Shadow {
        Shadow {
            offset: desplazamiento,
            blur: difuminado,
            spread: SIN_ENSANCHAMIENTO_DE_SOMBRA,
            color: self.sombra,
        }
    }

    /// Asigna las sombras de ventanas y menús desplegables.
    ///
    /// # Parámetros
    /// - `visuals`: estilo en construcción, que se modifica en el sitio.
    fn aplicar_sombras(&self, visuals: &mut Visuals) {
        visuals.window_shadow =
            self.sombra(DESPLAZAMIENTO_SOMBRA_VENTANA, DIFUMINADO_SOMBRA_VENTANA);
        visuals.popup_shadow = self.sombra(DESPLAZAMIENTO_SOMBRA_MENU, DIFUMINADO_SOMBRA_MENU);
    }

    /// Instala el estilo del tema en el contexto de `egui`.
    ///
    /// # Por qué no basta con `Context::set_visuals`
    ///
    /// `egui` guarda **dos** estilos, uno claro y otro oscuro, y en cada fotograma usa el
    /// que corresponde a `Context::tema()`. `set_visuals` escribe solo en el que esté
    /// activo *en ese momento*, y eso abre dos agujeros:
    ///
    /// 1. **Al arrancar**, todavía no se ha ejecutado ningún fotograma, así que `egui` no
    ///    sabe aún en qué modo está el sistema operativo y responde `Dark` por defecto. El
    ///    estilo se guardaría en el hueco oscuro; si Windows está en modo claro, en el
    ///    primer fotograma `egui` cambiaría al hueco claro, que conserva su estilo de
    ///    fábrica. El resultado sería exactamente el defecto que se venía a corregir, pero
    ///    al revés: el lienzo de MMCelt rodeado de barras claras ajenas.
    /// 2. **En marcha**, si el usuario cambia el modo de Windows, `egui` saltaría al otro
    ///    hueco y toda la ventana volvería a su estilo de fábrica.
    ///
    /// Escribiendo el mismo estilo en los dos huecos, el tema de la aplicación manda
    /// siempre, que es lo que corresponde: MMCelt tiene su propio selector de tema y no
    /// debe cambiar de aspecto porque lo haga el sistema.
    ///
    /// # Parámetros
    /// - `ctx`: contexto de `egui` sobre el que se instala el estilo.
    pub fn instalar_en(&self, ctx: &egui::Context) {
        let visuals = self.visuals();
        ctx.set_visuals_of(egui::Theme::Dark, visuals.clone());
        ctx.set_visuals_of(egui::Theme::Light, visuals);
    }
}

// ---------------------------------------------------------------------------
// Pruebas de legibilidad de las paletas
// ---------------------------------------------------------------------------

/// Comprobaciones automáticas de contraste sobre las paletas de color.
///
/// Un tema puede ser bonito y seguir siendo ilegible. Estas pruebas fijan un suelo
/// objetivo —el que define la norma de accesibilidad WCAG 2.1— para que ningún ajuste
/// estético posterior deje texto que no se lee sobre su propio fondo.
#[cfg(test)]
mod tests {
    use super::*;

    /// Relación de contraste mínima exigida a texto normal (WCAG 2.1, nivel AA).
    const CONTRASTE_MINIMO_TEXTO: f32 = 4.5;

    /// Relación de contraste mínima exigida a elementos gráficos y texto grande
    /// (WCAG 2.1, nivel AA para componentes no textuales).
    const CONTRASTE_MINIMO_GRAFICO: f32 = 3.0;

    /// Convierte un componente de color de 0-255 a su valor lineal.
    ///
    /// Los valores RGB que se escriben en el código están codificados con la curva sRGB,
    /// que no es proporcional a la luz emitida. Para medir contraste hay que deshacer esa
    /// curva primero; si no, dos colores que sobre el papel parecen separados pueden estar
    /// mucho más cerca de lo que aparentan.
    ///
    /// # Parámetros
    /// - `componente`: valor del canal en el rango 0-255.
    ///
    /// # Devuelve
    /// El valor lineal del canal, entre 0.0 y 1.0.
    fn a_lineal(componente: u8) -> f32 {
        let normalizado = componente as f32 / 255.0;
        if normalizado <= 0.03928 {
            normalizado / 12.92
        } else {
            ((normalizado + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Calcula la luminancia relativa de un color según la fórmula de la WCAG.
    ///
    /// # Parámetros
    /// - `color`: color a medir.
    ///
    /// # Devuelve
    /// La luminancia entre 0.0 (negro) y 1.0 (blanco).
    fn luminancia(color: Color32) -> f32 {
        0.2126 * a_lineal(color.r()) + 0.7152 * a_lineal(color.g()) + 0.0722 * a_lineal(color.b())
    }

    /// Calcula la relación de contraste entre dos colores.
    ///
    /// El resultado va de 1.0 (colores idénticos) a 21.0 (negro puro contra blanco puro).
    ///
    /// # Parámetros
    /// - `primero`: uno de los dos colores.
    /// - `segundo`: el otro color.
    ///
    /// # Devuelve
    /// La relación de contraste, siempre mayor o igual que 1.0.
    fn contraste(primero: Color32, segundo: Color32) -> f32 {
        let (a, b) = (luminancia(primero), luminancia(segundo));
        let (claro, oscuro) = if a > b { (a, b) } else { (b, a) };
        (claro + 0.05) / (oscuro + 0.05)
    }

    /// Exige que un par de colores alcance el contraste indicado, informando de cuál falla.
    ///
    /// # Parámetros
    /// - `frente`: color del elemento que debe verse.
    /// - `fondo`: color sobre el que se dibuja.
    /// - `minimo`: relación de contraste exigida.
    /// - `descripcion`: texto que identifica el par en el mensaje de error.
    fn exigir_contraste(frente: Color32, fondo: Color32, minimo: f32, descripcion: &str) {
        let medido = contraste(frente, fondo);
        assert!(
            medido >= minimo,
            "{descripcion}: contraste {medido:.2}, se exige {minimo:.1}"
        );
    }

    #[test]
    /// El texto principal supera el contraste de la WCAG sobre cualquier fondo donde aparece.
    ///
    /// Se comprueba contra las tres superficies —lienzo, panel y nodo— y en los tres temas.
    /// Basta con que falle una combinación para que haya una pantalla ilegible.
    fn el_texto_principal_se_lee_sobre_todas_las_superficies() {
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let superficies = [
                ("lienzo", tema.fondo_del_lienzo),
                ("panel", tema.fondo_del_panel),
                ("nodo", tema.fondo_del_nodo),
                ("nodo raiz", tema.fondo_de_la_raiz),
                ("etiqueta", tema.fondo_de_la_insignia),
                ("control", tema.superficie),
                ("control senalado", tema.superficie_con_cursor),
                ("control pulsado", tema.superficie_activa),
            ];
            for (nombre, fondo) in superficies {
                exigir_contraste(
                    tema.texto_principal,
                    fondo,
                    CONTRASTE_MINIMO_TEXTO,
                    &format!("{modo:?} / texto principal sobre {nombre}"),
                );
            }
        }
    }

    #[test]
    /// El texto de apoyo se lee sobre las superficies en las que se usa.
    ///
    /// No se le exige sobre el lienzo porque ahí no se dibuja: el fondo del mapa solo lleva
    /// nodos y conexiones.
    fn el_texto_secundario_se_lee_sobre_nodos_y_paneles() {
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let superficies = [
                ("panel", tema.fondo_del_panel),
                ("nodo", tema.fondo_del_nodo),
                ("control", tema.superficie),
                ("lienzo", tema.fondo_del_lienzo),
                ("etiqueta", tema.fondo_de_la_insignia),
            ];
            for (nombre, fondo) in superficies {
                exigir_contraste(
                    tema.texto_secundario,
                    fondo,
                    CONTRASTE_MINIMO_TEXTO,
                    &format!("{modo:?} / texto secundario sobre {nombre}"),
                );
            }
        }
    }

    #[test]
    /// Al texto atenuado se le exige contraste de **texto**, no de elemento gráfico.
    ///
    /// No es decoración: es la barra de estado, los encabezados de los menús y los recuentos
    /// de las etiquetas. Antes se comprobaba solo contra el nodo, que en el tema claro es
    /// blanco puro y resulta ser la única superficie donde apenas se usa.
    fn el_texto_atenuado_sigue_siendo_texto() {
        // El texto atenuado no es decoración: es la barra de estado, los encabezados de
        // los menús y los recuentos de las etiquetas. Se le exige el contraste del texto
        // sobre todas las superficies donde aparece, no el de un elemento gráfico.
        //
        // Antes se comprobaba solo contra el nodo, que en el tema claro es blanco puro y
        // resulta ser la única superficie donde el texto atenuado apenas se usa. Sobre el
        // panel, que es donde de verdad se lee, medía 2,89:1.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let superficies = [
                ("panel", tema.fondo_del_panel),
                ("control", tema.superficie),
                ("nodo", tema.fondo_del_nodo),
                ("etiqueta", tema.fondo_de_la_insignia),
                ("lienzo", tema.fondo_del_lienzo),
            ];
            for (nombre, fondo) in superficies {
                exigir_contraste(
                    tema.texto_atenuado,
                    fondo,
                    CONTRASTE_MINIMO_TEXTO,
                    &format!("{modo:?} / texto atenuado sobre {nombre}"),
                );
            }
        }
    }

    #[test]
    /// Los tres niveles de texto se distinguen entre sí sobre el panel.
    ///
    /// Si al subir el contraste del atenuado se acerca demasiado al de apoyo, la jerarquía
    /// deja de existir y el diseño pierde la información que transmitía.
    fn el_texto_mantiene_su_jerarquia() {
        // Los tres niveles de texto tienen que distinguirse entre sí sobre el panel. Si
        // al subir el contraste del atenuado se acerca demasiado al de apoyo, la jerarquía
        // deja de existir y el diseño pierde la información que transmitía.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let principal = contraste(tema.texto_principal, tema.fondo_del_panel);
            let apoyo = contraste(tema.texto_secundario, tema.fondo_del_panel);
            let atenuado = contraste(tema.texto_atenuado, tema.fondo_del_panel);
            assert!(
                principal > apoyo && apoyo > atenuado,
                "{modo:?}: la jerarquía del texto está desordenada \
                 (principal {principal:.1}, apoyo {apoyo:.1}, atenuado {atenuado:.1})"
            );
        }
    }

    #[test]
    /// Ninguno de los colores de rama se confunde con el fondo del mapa.
    ///
    /// Son los que dan a cada rama su identidad visual: uno que se pierda contra el lienzo
    /// deja esa rama sin distinguir de las demás.
    fn los_colores_de_rama_se_distinguen_del_lienzo() {
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            for (indice, color) in tema.colores_de_las_ramas.iter().enumerate() {
                exigir_contraste(
                    *color,
                    tema.fondo_del_lienzo,
                    CONTRASTE_MINIMO_GRAFICO,
                    &format!("{modo:?} / rama {indice} sobre lienzo"),
                );
            }
        }
    }

    #[test]
    /// La retícula del fondo se ve, pero no destaca.
    ///
    /// Tiene un mínimo y un máximo a propósito: por debajo del mínimo es invisible y no sirve
    /// de referencia; por encima del máximo compite con los nodos y ensucia el mapa.
    fn la_reticula_se_intuye_sin_competir_con_los_nodos() {
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let medido = contraste(tema.linea_de_la_reticula, tema.fondo_del_lienzo);
            assert!(
                medido > 1.05,
                "{modo:?}: la reticula es invisible (contraste {medido:.2})"
            );
            // El limite superior es lo que fallo en su dia con el blanco premultiplicado:
            // una reticula que grita mas que el contenido.
            let limite = if modo == AppThemeMode::HighContrast {
                6.0
            } else {
                2.2
            };
            assert!(
                medido <= limite,
                "{modo:?}: la reticula compite con los nodos (contraste {medido:.2}, maximo {limite:.1})"
            );
        }
    }

    #[test]
    /// Al color de la conexión cruzada se le exige el umbral de texto, no el de gráfico.
    ///
    /// La etiqueta de la conexión se dibuja como texto sobre el lienzo con este mismo color,
    /// así que tiene que superar 4,5:1 y no el 3:1 que bastaría para una línea.
    fn la_conexion_cruzada_es_visible_sobre_el_lienzo() {
        // La etiqueta de la conexión cruzada se dibuja como texto sobre el lienzo con este
        // mismo color, por lo que debe superar el umbral WCAG de texto (4,5:1).
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            exigir_contraste(
                tema.linea_de_conexion_cruzada,
                tema.fondo_del_lienzo,
                CONTRASTE_MINIMO_TEXTO,
                &format!("{modo:?} / conexion cruzada sobre lienzo"),
            );
        }
    }

    #[test]
    /// El borde del nodo seleccionado se distingue del propio nodo.
    ///
    /// Es lo único que indica qué nodo está seleccionado: si no se ve, el usuario no sabe
    /// sobre qué van a actuar Supr, Tab o el inspector.
    fn el_borde_de_seleccion_destaca_sobre_el_nodo() {
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            exigir_contraste(
                tema.borde_del_nodo_seleccionado,
                tema.fondo_del_nodo,
                CONTRASTE_MINIMO_GRAFICO,
                &format!("{modo:?} / borde de seleccion sobre nodo"),
            );
        }
    }

    #[test]
    /// Lienzo, panel y nodo se distinguen entre sí.
    ///
    /// Si dos coinciden, la interfaz se aplana y el panel deja de leerse como una capa
    /// separada del mapa.
    fn las_superficies_se_escalonan_en_profundidad() {
        // Lienzo, panel y nodo deben distinguirse entre si: si dos coinciden, la interfaz
        // se aplana y el panel deja de leerse como una capa separada del mapa.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            for (nombre, primero, segundo) in [
                (
                    "lienzo y panel",
                    tema.fondo_del_lienzo,
                    tema.fondo_del_panel,
                ),
                ("lienzo y nodo", tema.fondo_del_lienzo, tema.fondo_del_nodo),
                (
                    "nodo y nodo raiz",
                    tema.fondo_del_nodo,
                    tema.fondo_de_la_raiz,
                ),
            ] {
                let medido = contraste(primero, segundo);
                assert!(
                    medido >= 1.12,
                    "{modo:?}: {nombre} son casi el mismo color (contraste {medido:.2})"
                );
            }
        }
    }

    #[test]
    /// Ningún color de la paleta lleva transparencia, salvo la sombra.
    ///
    /// Un color con alfa obliga a razonar sobre la premultiplicación de canales cada vez que
    /// se ajusta, y ya provocó un fallo visible en este proyecto. La paleta se define opaca;
    /// la transparencia, si hace falta, se aplica en el punto donde se dibuja.
    fn ningun_color_de_la_paleta_es_semitransparente() {
        // Un color con transparencia dentro de la paleta obliga a razonar sobre la
        // premultiplicacion de canales cada vez que se ajusta, y ya ha provocado un fallo
        // visible en este proyecto. La paleta se define opaca; la transparencia, si hace
        // falta, se aplica en el punto de dibujo.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let mut colores = vec![
                ("canvas_bg", tema.fondo_del_lienzo),
                ("grid_line", tema.linea_de_la_reticula),
                ("node_bg", tema.fondo_del_nodo),
                ("node_root_bg", tema.fondo_de_la_raiz),
                ("node_border", tema.borde_del_nodo),
                ("node_border_selected", tema.borde_del_nodo_seleccionado),
                ("text_primary", tema.texto_principal),
                ("text_secondary", tema.texto_secundario),
                ("text_muted", tema.texto_atenuado),
                ("cross_connection_line", tema.linea_de_conexion_cruzada),
                ("badge_bg", tema.fondo_de_la_insignia),
                ("panel_bg", tema.fondo_del_panel),
                ("surface", tema.superficie),
                ("surface_hover", tema.superficie_con_cursor),
                ("surface_active", tema.superficie_activa),
                ("separator", tema.separador),
                ("accent", tema.acento),
                ("text_on_branch", tema.texto_sobre_la_rama),
                ("success", tema.exito),
                ("warning", tema.aviso),
                ("danger", tema.peligro),
            ];
            for color in &tema.colores_de_las_ramas {
                colores.push(("color_de_la_rama", *color));
            }
            for (nombre, color) in colores {
                assert_eq!(color.a(), 255, "{modo:?}: {nombre} no es opaco");
            }
        }
    }

    #[test]
    /// Los botones y desplegables se leen en reposo, señalados y pulsados.
    ///
    /// Usan el texto secundario en reposo y el principal en los otros dos estados: los tres
    /// tienen que ser legibles, no solo el que se ve al abrir la ventana.
    fn el_texto_de_los_controles_se_lee_en_todos_sus_estados() {
        // Los botones y desplegables usan el texto secundario en reposo y el principal
        // cuando se señalan o se pulsan. Los tres estados tienen que ser legibles.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            exigir_contraste(
                tema.texto_secundario,
                tema.superficie,
                CONTRASTE_MINIMO_TEXTO,
                &format!("{modo:?} / control en reposo"),
            );
            for (nombre, fondo) in [
                ("control señalado", tema.superficie_con_cursor),
                ("control pulsado", tema.superficie_activa),
            ] {
                exigir_contraste(
                    tema.texto_principal,
                    fondo,
                    CONTRASTE_MINIMO_TEXTO,
                    &format!("{modo:?} / {nombre}"),
                );
            }
        }
    }

    #[test]
    /// El color de énfasis se distingue de todo aquello sobre lo que se dibuja.
    ///
    /// Marca el foco del teclado; si no se distingue del control que rodea, se pierde la única
    /// pista de dónde está el cursor al navegar sin ratón.
    fn el_color_de_enfasis_se_ve_sobre_las_superficies_que_lo_usan() {
        // El énfasis marca el foco del teclado; si no se distingue del control que rodea,
        // se pierde la única pista de dónde está el cursor al navegar sin ratón.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            for (nombre, fondo) in [
                ("control", tema.superficie),
                ("panel", tema.fondo_del_panel),
                ("control señalado", tema.superficie_con_cursor),
            ] {
                exigir_contraste(
                    tema.acento,
                    fondo,
                    CONTRASTE_MINIMO_GRAFICO,
                    &format!("{modo:?} / énfasis sobre {nombre}"),
                );
            }
        }
    }

    #[test]
    /// Lo que se pinta encima del color de una rama se distingue de ella.
    ///
    /// El botón de añadir hijo pinta un «+» sobre un círculo del color de la rama. Estaba en
    /// blanco fijo, y sobre el marfil del tema claro medía 1,23:1: era invisible hasta que el
    /// puntero se posaba encima.
    fn lo_que_se_dibuja_sobre_una_rama_se_ve() {
        // El boton de anadir hijo pinta un signo «+» sobre un circulo del color de la
        // rama. Estaba en blanco fijo, y sobre el marfil del tema claro medía 1,23:1: era
        // invisible hasta que el puntero se posaba encima.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            for (indice, rama) in tema.colores_de_las_ramas.iter().enumerate() {
                exigir_contraste(
                    tema.texto_sobre_la_rama,
                    *rama,
                    CONTRASTE_MINIMO_GRAFICO,
                    &format!("{modo:?} / simbolo sobre la rama {indice}"),
                );
            }
            // El mismo simbolo se dibuja sobre la etiqueta cuando no esta señalado.
            exigir_contraste(
                tema.texto_principal,
                tema.fondo_de_la_insignia,
                CONTRASTE_MINIMO_TEXTO,
                &format!("{modo:?} / simbolo sobre la etiqueta"),
            );
        }
    }

    #[test]
    /// La sombra es la única excepción a la regla de la paleta opaca.
    ///
    /// Y tiene que serlo: una sombra sin transparencia no es una sombra, es un recuadro gris.
    /// La prueba fija esa excepción para que no se cuelen otras detrás.
    fn la_sombra_es_el_unico_color_translucido() {
        // Complementa a `ningun_color_de_la_paleta_es_semitransparente`: la sombra es la
        // excepción deliberada, y una sombra opaca sería un rectángulo negro.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let alfa = tema.sombra.a();
            assert!(
                alfa > 0 && alfa < 255,
                "{modo:?}: la sombra debe ser translúcida (alfa {alfa})"
            );
        }
    }

    #[test]
    /// El estilo que recibe `egui` sale de la paleta, y no de valores escritos aparte.
    ///
    /// Si alguno se fijara a mano, cambiar de tema dejaría ese elemento con el color del tema
    /// anterior, que es el defecto más difícil de ver de todos: solo aparece al cambiar.
    fn el_estilo_de_egui_se_construye_a_partir_de_la_paleta() {
        // Esta es la prueba que vigila el fallo que motivó el cambio: que la interfaz
        // vuelva a pintarse con el estilo por omisión de egui en lugar de con el tema.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let visuals = tema.visuals();

            assert_eq!(
                visuals.panel_fill, tema.fondo_del_panel,
                "{modo:?}: el fondo de los paneles no sale de la paleta"
            );
            assert_eq!(
                visuals.window_fill, tema.fondo_del_panel,
                "{modo:?}: el fondo de las ventanas no sale de la paleta"
            );
            assert_eq!(
                visuals.widgets.inactive.bg_fill, tema.superficie,
                "{modo:?}: los botones no salen de la paleta"
            );
            assert_eq!(
                visuals.widgets.hovered.bg_fill, tema.superficie_con_cursor,
                "{modo:?}: el estado señalado no sale de la paleta"
            );
            assert_eq!(
                visuals.hyperlink_color, tema.acento,
                "{modo:?}: los enlaces no usan el color de énfasis"
            );
            assert_eq!(
                visuals.extreme_bg_color, tema.fondo_del_lienzo,
                "{modo:?}: los campos de texto no salen de la paleta"
            );
            assert_eq!(
                visuals.dark_mode,
                modo.es_oscuro(),
                "{modo:?}: egui no sabe si el tema es oscuro o claro"
            );
        }
    }

    #[test]
    /// El tema claro se declara claro ante `egui`, y el oscuro, oscuro.
    ///
    /// `egui` decide por su cuenta algunos detalles —sombras, resaltados— según esa bandera.
    /// Con ella mal puesta, la paleta es correcta y aun así la interfaz se ve rara.
    fn el_tema_claro_no_le_dice_a_egui_que_es_oscuro() {
        // Comprobación explícita del síntoma original: el usuario elegía el tema claro y
        // la mitad de la ventana seguía oscura.
        let claro = ThemeConfig::new(AppThemeMode::Light);
        let visuals = claro.visuals();
        assert!(!visuals.dark_mode);
        assert_eq!(visuals.panel_fill, claro.fondo_del_panel);
        assert!(
            luminancia(visuals.panel_fill) > 0.5,
            "el panel del tema claro es oscuro (luminancia {:.2})",
            luminancia(visuals.panel_fill)
        );
    }

    #[test]
    /// Los colores de significado —éxito, aviso, peligro— se leen como texto.
    ///
    /// No son adornos: llevan mensajes que el usuario tiene que poder leer, así que se les
    /// exige el umbral de texto sobre las superficies donde aparecen.
    fn los_colores_semanticos_se_leen_como_texto() {
        // Aciertos, avisos y errores se dibujan como texto de etiquetas y botones sobre
        // los paneles, así que se les exige el contraste del texto y no el de un gráfico.
        // Es la comprobación que faltaba: los verdes y ámbares vivos del tema oscuro
        // quedaban en 2,2:1 sobre el papel cálido del tema claro.
        for modo in AppThemeMode::TODOS {
            let tema = ThemeConfig::new(modo);
            let semanticos = [
                ("acierto", tema.exito),
                ("aviso", tema.aviso),
                ("peligro", tema.peligro),
            ];
            // Los estados señalado y pulsado son imprescindibles aquí: estos tres colores
            // se usan sobre todo en botones, que son justo los controles que se señalan y
            // se pulsan. El peligro llegaba a 2,27:1 en alto contraste al hacer clic.
            let superficies = [
                ("panel", tema.fondo_del_panel),
                ("control", tema.superficie),
                ("control senalado", tema.superficie_con_cursor),
                ("control pulsado", tema.superficie_activa),
                ("nodo", tema.fondo_del_nodo),
            ];
            for (nombre_color, color) in semanticos {
                for (nombre_fondo, fondo) in superficies {
                    exigir_contraste(
                        color,
                        fondo,
                        CONTRASTE_MINIMO_TEXTO,
                        &format!("{modo:?} / {nombre_color} sobre {nombre_fondo}"),
                    );
                }
            }
        }
    }
}
