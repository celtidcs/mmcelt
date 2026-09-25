//! # Módulo de Sistema de Ayuda y Galletas Guiadas (`ui/help_system.rs`)
//!
//! Proporciona asistencia interactiva y pedagógica para usuarios noveles:
//! - **Galletas / Tooltips Contextuales**: Explicaciones breves con botón `+info`.
//! - **Panel de Ayuda Extensa a la Derecha**: Guías detalladas sobre qué hace cada opción,
//!   por dónde continuar, opciones disponibles y mejores prácticas.

use crate::textos::Idioma;
use crate::ui::ayuda_textos;

/// Tema estable que enlaza una ayuda breve de la interfaz con su guía extensa traducida.
///
/// Cada variante tiene título, resumen y documento en los seis idiomas. [`TemaDeAyuda::TODOS`]
/// fija el orden del selector y las pruebas impiden que una variante quede sin contenido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemaDeAyuda {
    /// Primer recorrido para quien abre MMCelt por primera vez.
    PrimerosPasos,
    /// Teclas y gestos con los que se crean y editan nodos.
    AtajosYCreacionNodos,
    /// Uso de la visión del creador como criterio general del mapa.
    VisionDelCreador,
    /// Significado de los roles arquitectónicos de cada nodo.
    RolesYArquitectura,
    /// Estados de trabajo y seguimiento del progreso.
    EstadosYProgreso,
    /// Asociación entre nodos y archivos del proyecto.
    MapeoArchivosCodigo,
    /// Decisiones, vetos y correcciones que pertenecen a la persona.
    ControlHumanoCorrecciones,
    /// Recorrido completo con la IA: qué sale, qué controla la persona y qué vuelve.
    ///
    /// Los temas siguientes explican cada pieza por separado. Este existe porque ninguno
    /// contaba el conjunto, y quien llega sin saber de mapas ni de IA necesita el hilo
    /// antes que el detalle.
    RecorridoConLaIa,
    /// Generación del documento Markdown que recibe un agente.
    ExportarMarkdownIA,
    /// Conversión de una respuesta de IA en un mapa validado.
    ImportarDesdeIA,
    /// Preparación de «Enviar a…» y vigilancia del mapa devuelto.
    EnviarAYVigilancia,
    /// Relaciones laterales entre ramas que no forman parte de la jerarquía.
    ConexionesCruzadas,
    /// Disposiciones automática, radial y libre del lienzo.
    ModosDeDisposicion,
    /// Mapas iniciales de arquitecturas de software.
    PlantillasDeSoftware,
    /// Creación de un mapa a partir de una carpeta de código.
    EscanerDeRepositorios,
    /// Registro del servidor MCP en los agentes compatibles.
    ClaudeMCPIntegracion,
    /// Recorrido de conversación con Claude Code en consola.
    ClaudeCodeTerminal,
    /// Diferencias entre ChatGPT web, Codex y un GPT personalizado.
    ChatGPTCustomGPT,
    /// Recorridos con Gemini, Gemini CLI y Antigravity.
    GeminiGems,
    /// Significado visual de colores, ramas y estados.
    ColoresDelMapa,
    /// Mapas de ejemplo incluidos y qué enseña cada uno.
    EjemplosIncluidos,
    /// Guardado normal, autoguardado y recuperación tras un cierre.
    GuardadoYRecuperacion,
    /// Escalado completo de la interfaz para distintos monitores.
    TamanoDeLaInterfaz,
    /// Identificación exacta de versión, commit y compilación.
    VersionYCompilacion,
}

impl TemaDeAyuda {
    /// Todos los temas de ayuda, en el orden en que se ofrecen al usuario.
    ///
    /// El orden va de lo básico a lo avanzado y deja las integraciones con cada plataforma
    /// de IA al final, que es donde se buscan cuando ya se conoce el programa.
    ///
    /// Tener la lista aquí, junto a la definición de los temas, evita el problema que
    /// tenía el selector del panel lateral: la enumeraba a mano, así que un tema nuevo
    /// existía pero no se podía elegir hasta que alguien se acordaba de añadirlo también
    /// allí.
    pub const TODOS: [TemaDeAyuda; 24] = [
        TemaDeAyuda::PrimerosPasos,
        TemaDeAyuda::AtajosYCreacionNodos,
        TemaDeAyuda::VisionDelCreador,
        TemaDeAyuda::RolesYArquitectura,
        TemaDeAyuda::EstadosYProgreso,
        TemaDeAyuda::MapeoArchivosCodigo,
        TemaDeAyuda::ControlHumanoCorrecciones,
        TemaDeAyuda::RecorridoConLaIa,
        TemaDeAyuda::ExportarMarkdownIA,
        TemaDeAyuda::ImportarDesdeIA,
        TemaDeAyuda::EnviarAYVigilancia,
        TemaDeAyuda::ConexionesCruzadas,
        TemaDeAyuda::ModosDeDisposicion,
        TemaDeAyuda::PlantillasDeSoftware,
        TemaDeAyuda::EscanerDeRepositorios,
        TemaDeAyuda::ColoresDelMapa,
        TemaDeAyuda::EjemplosIncluidos,
        TemaDeAyuda::GuardadoYRecuperacion,
        TemaDeAyuda::TamanoDeLaInterfaz,
        TemaDeAyuda::VersionYCompilacion,
        TemaDeAyuda::ClaudeMCPIntegracion,
        TemaDeAyuda::ClaudeCodeTerminal,
        TemaDeAyuda::ChatGPTCustomGPT,
        TemaDeAyuda::GeminiGems,
    ];

    /// Rótulo corto del tema, en el idioma que tenga puesto el usuario.
    ///
    /// Es lo que se ve en el selector del panel de ayuda.
    ///
    /// # Parámetros
    ///
    /// * `idioma` - Idioma en el que devolver el rótulo.
    ///
    /// # Devuelve
    ///
    /// El título traducido. Nunca está vacío: lo vigila una prueba.
    pub fn titulo(&self, idioma: Idioma) -> &'static str {
        ayuda_textos::titulo_de(*self, idioma)
    }

    /// Resumen breve del tema, para la «galleta» emergente con el botón `+info`.
    ///
    /// # Parámetros
    ///
    /// * `idioma` - Idioma en el que devolver el resumen.
    ///
    /// # Devuelve
    ///
    /// El resumen traducido, de una o dos frases.
    pub fn resumen_de_galleta(&self, idioma: Idioma) -> &'static str {
        ayuda_textos::resumen_de(*self, idioma)
    }

    /// Documento completo del tema, en Markdown, para el panel lateral de ayuda.
    ///
    /// Sale de `assets/ayuda/<idioma>/<Tema>.md`, incrustado en compilación.
    ///
    /// # Parámetros
    ///
    /// * `idioma` - Idioma en el que devolver la guía.
    ///
    /// # Devuelve
    ///
    /// El documento traducido, con su estructura de Markdown intacta.
    pub fn guia_completa(&self, idioma: Idioma) -> &'static str {
        ayuda_textos::guia_de(*self, idioma)
    }
}
