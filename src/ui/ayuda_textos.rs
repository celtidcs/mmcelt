//! # Textos de la ayuda interna (`ui/ayuda_textos.rs`)
//!
//! **Este archivo se genera automáticamente a partir de las guías de ayuda. No se edita a mano.**
//!
//! Las guías son documentos largos —unos 143 KB entre los seis idiomas— y viven en
//! `assets/ayuda/<idioma>/<Tema>.md`, no dentro del fuente. Metidas aquí dejarían el
//! módulo ilegible para editar y para revisar en un diff; en archivos se leen y se
//! comparan idioma a idioma.
//!
//! `include_str!` las incrusta **en tiempo de compilación**, así que el binario sigue
//! siendo autosuficiente: no hay que encontrar ningún archivo al arrancar. Importa
//! porque el servidor MCP corre sin ventana y sin directorio de trabajo garantizado.
//!
//! Si falta un archivo, **no compila**. Es la misma garantía que da el `match`
//! exhaustivo de `textos.rs`: no se puede olvidar una traducción a medias.
//!
//! ## Por qué `guia_de`, `titulo_de` y `resumen_de` son largas a propósito
//!
//! Miden 418, 234 y 154 líneas, por encima del límite del estándar 2 de `CLAUDE.md`. Son
//! el mismo caso que [`crate::textos`]: tablas de datos, no lógica. Su complejidad real es
//! 1, porque un `match` de mapeo no decide nada; lo que crece es el número de temas de
//! ayuda —hoy 24— multiplicado por los seis idiomas.
//!
//! Y como allí, la forma actual la vigila el compilador: añadir un tema a
//! [`crate::ui::help_system::TemaDeAyuda`] sin escribir su guía deja el `match` incompleto
//! y `cargo build` falla diciendo cuál falta. Trocearlas obligaría a devolver `Option` y a
//! encadenar, y con eso un tema sin texto compilaría y se descubriría en la pantalla de
//! alguien.

use crate::textos::Idioma;
use crate::ui::help_system::TemaDeAyuda;

/// Rótulo corto del tema, en el idioma pedido.
///
/// Es lo que se ve en el selector del panel de ayuda, que es estrecho: por eso el
/// título y el encabezado `# ` de la guía son cadenas distintas y no tienen por qué
/// coincidir palabra por palabra.
pub fn titulo_de(tema: TemaDeAyuda, idioma: Idioma) -> &'static str {
    match (tema, idioma) {
        (TemaDeAyuda::PrimerosPasos, Idioma::Espanol) => "🏁 Primeros Pasos en MMCelt",
        (TemaDeAyuda::PrimerosPasos, Idioma::Ingles) => "🏁 First Steps in MMCelt",
        (TemaDeAyuda::PrimerosPasos, Idioma::Frances) => "🏁 Premiers pas dans MMCelt",
        (TemaDeAyuda::PrimerosPasos, Idioma::Aleman) => "🏁 Erste Schritte in MMCelt",
        (TemaDeAyuda::PrimerosPasos, Idioma::Ruso) => "🏁 Первые шаги в MMCelt",
        (TemaDeAyuda::PrimerosPasos, Idioma::ChinoSimplificado) => "🏁 MMCelt 入门指引",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Espanol) => {
            "⌨️ Creación Rápida y Atajos de Teclado"
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Ingles) => {
            "⌨️ Quick Creation & Keyboard Shortcuts"
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Frances) => {
            "⌨️ Création rapide et raccourcis clavier"
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Aleman) => {
            "⌨️ Schnelle Erstellung und Tastenkombinationen"
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Ruso) => {
            "⌨️ Быстрое создание и горячие клавиши"
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::ChinoSimplificado) => "⌨️ 快速创建与常用快捷键",
        (TemaDeAyuda::VisionDelCreador, Idioma::Espanol) => {
            "🧭 Proyecto e instrucciones para la IA"
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::Ingles) => "🧭 Project and instructions for AI",
        (TemaDeAyuda::VisionDelCreador, Idioma::Frances) => "🧭 Projet et instructions pour l’IA",
        (TemaDeAyuda::VisionDelCreador, Idioma::Aleman) => "🧭 Projekt und Anweisungen für die KI",
        (TemaDeAyuda::VisionDelCreador, Idioma::Ruso) => "🧭 Проект и инструкции для ИИ",
        (TemaDeAyuda::VisionDelCreador, Idioma::ChinoSimplificado) => "🧭 项目与 AI 指令",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Espanol) => {
            "🏛️ Roles Arquitectónicos de los Nodos"
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Ingles) => "🏛️ Architectural Roles of Nodes",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Frances) => "🏛️ Rôles architecturaux des nœuds",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Aleman) => {
            "🏛️ Architektonische Rollen der Knoten"
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Ruso) => "🏛️ Архитектурные роли узлов",
        (TemaDeAyuda::RolesYArquitectura, Idioma::ChinoSimplificado) => "🏛️ 节点的架构角色分类",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Espanol) => "📊 Estados del Ciclo de Vida del Nodo",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Ingles) => "📊 Node Lifecycle States",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Frances) => "📊 États du cycle de vie du nœud",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Aleman) => "📊 Status im Lebenszyklus des Knotens",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Ruso) => "📊 Статусы жизненного цикла узла",
        (TemaDeAyuda::EstadosYProgreso, Idioma::ChinoSimplificado) => "📊 节点生命周期状态",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Espanol) => {
            "📁 Mapeo de Archivos de Código (file_path)"
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Ingles) => "📁 Code File Mapping (file_path)",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Frances) => {
            "📁 Mappage des fichiers de code (file_path)"
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Aleman) => {
            "📁 Zuordnung von Codedateien (file_path)"
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Ruso) => "📁 Привязка файлов кода (file_path)",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::ChinoSimplificado) => {
            "📁 代码文件映射 (file_path)"
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Espanol) => {
            "🛑 Control Humano y Directivas de Corrección"
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Ingles) => {
            "🛑 User Control & Correction Directives"
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Frances) => {
            "🛑 Contrôle humain et directives de correction"
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Aleman) => {
            "🛑 Benutzerkontrolle und Korrekturanweisungen"
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Ruso) => {
            "🛑 Контроль пользователя и указания по исправлению"
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::ChinoSimplificado) => {
            "🛑 用户把控与修正指令"
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Espanol) => "🤖 El Recorrido Completo con la IA",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Ingles) => "🤖 The Whole Journey with the AI",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Frances) => "🤖 Le parcours complet avec l'IA",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Aleman) => "🤖 Der gesamte Weg mit der KI",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Ruso) => "🤖 Весь путь работы с ИИ",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::ChinoSimplificado) => "🤖 与 AI 协作的完整流程",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Espanol) => {
            "🤖 Exportación de Markdown y Previsualización para la IA"
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Ingles) => "🤖 Markdown Export & AI Preview",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Frances) => {
            "🤖 Exportation Markdown et prévisualisation IA"
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Aleman) => "🤖 Markdown-Export und KI-Vorschau",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Ruso) => {
            "🤖 Экспорт в Markdown и предпросмотр для ИИ"
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::ChinoSimplificado) => {
            "🤖 Markdown 导出与 AI 预览"
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Espanol) => "📥 Importación Bidireccional desde IA",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Ingles) => "📥 Two-Way Import from AI",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Frances) => {
            "📥 Importation bidirectionnelle depuis l'IA"
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Aleman) => "📥 Bidirektionaler Import aus der KI",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Ruso) => "📥 Двусторонний импорт из ИИ",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::ChinoSimplificado) => "📥 来自 AI 的双向导入",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Espanol) => "📤 «Enviar a...» y Vigilancia",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Ingles) => "📤 'Send to...' & Live Watch",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Frances) => "📤 « Envoyer à... » et surveillance",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Aleman) => "📤 „Senden an...“ und Überwachung",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Ruso) => "📤 «Отправить в...» и наблюдение",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::ChinoSimplificado) => "📤 “发送到...”与实时监听",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Espanol) => {
            "🔗 Relaciones Cruzadas y Dependencias"
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Ingles) => "🔗 Cross-Relations & Dependencies",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Frances) => {
            "🔗 Relations croisées et dépendances"
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Aleman) => {
            "🔗 Querverbindungen und Abhängigkeiten"
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Ruso) => "🔗 Перекрёстные связи и зависимости",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::ChinoSimplificado) => "🔗 交叉关联与依赖矩阵",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Espanol) => "📐 Modos de Disposición Espacial",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Ingles) => "📐 Spatial Layout Modes",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Frances) => "📐 Modes de disposition spatiale",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Aleman) => "📐 Räumliche Layout-Modi",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Ruso) => "📐 Режимы пространственной компоновки",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::ChinoSimplificado) => "📐 空间排版模式",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Espanol) => {
            "✨ Plantillas de Arquitectura de Software"
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Ingles) => "✨ Software Architecture Templates",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Frances) => {
            "✨ Modèles d'architecture logicielle"
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Aleman) => {
            "✨ Vorlagen für Softwarearchitektur"
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Ruso) => "✨ Шаблоны архитектуры ПО",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::ChinoSimplificado) => "✨ 软件架构脚手架模版",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Espanol) => "🔍 Escáner Automático de Código",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Ingles) => "🔍 Automatic Codebase Scanner",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Frances) => "🔍 Scanner automatique de code",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Aleman) => "🔍 Automatischer Code-Scanner",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Ruso) => "🔍 Автоматический сканер кода",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::ChinoSimplificado) => "🔍 本地代码库自动扫描",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Espanol) => "🎨 Qué Significan los Colores",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Ingles) => "🎨 What the Colors Mean",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Frances) => "🎨 Signification des couleurs",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Aleman) => "🎨 Bedeutung der Farben",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Ruso) => "🎨 Что означают цвета",
        (TemaDeAyuda::ColoresDelMapa, Idioma::ChinoSimplificado) => "🎨 颜色标识的设计规范",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Espanol) => "📚 Ejemplos Incluidos",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Ingles) => "📚 Included Examples",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Frances) => "📚 Exemples inclus",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Aleman) => "📚 Enthaltene Beispiele",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Ruso) => "📚 Включенные примеры",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::ChinoSimplificado) => "📚 内置完整案例",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Espanol) => "💾 Guardado y Recuperación",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Ingles) => "💾 Saving & Recovery",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Frances) => {
            "💾 Enregistrement et récupération"
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Aleman) => {
            "💾 Speichern und Wiederherstellung"
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Ruso) => "💾 Сохранение и восстановление",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::ChinoSimplificado) => "💾 存盘与灾难恢复机制",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Espanol) => "🔍 Tamaño de la Interfaz",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Ingles) => "🔍 Interface Scale",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Frances) => "🔍 Échelle de l'interface",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Aleman) => "🔍 Skalierung der Benutzeroberfläche",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Ruso) => "🔍 Масштаб интерфейса",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::ChinoSimplificado) => "🔍 界面缩放比例调节",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Espanol) => "ℹ️ Versión y Compilación",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Ingles) => "ℹ️ Version & Build",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Frances) => "ℹ️ Version et compilation",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Aleman) => "ℹ️ Version und Build",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Ruso) => "ℹ️ Версия и сборка",
        (TemaDeAyuda::VersionYCompilacion, Idioma::ChinoSimplificado) => "ℹ️ 版本信息与构建详情",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Espanol) => "🟣 Servidor MCP y agentes de IA",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Ingles) => "🟣 MCP Server & AI Agents",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Frances) => "🟣 Serveur MCP et agents IA",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Aleman) => "🟣 MCP-Server und KI-Agenten",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Ruso) => "🟣 Сервер MCP и агенты ИИ",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::ChinoSimplificado) => {
            "🟣 MCP 服务器与 AI 智能体"
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Espanol) => {
            "🟣 Integración con Claude Code (CLI)"
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Ingles) => {
            "🟣 Integration with Claude Code (CLI)"
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Frances) => {
            "🟣 Intégration avec Claude Code (CLI)"
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Aleman) => "🟣 Integration mit Claude Code (CLI)",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Ruso) => "🟣 Интеграция с Claude Code (CLI)",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::ChinoSimplificado) => {
            "🟣 与 Claude Code 集成 (CLI)"
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Espanol) => {
            "🟢 Integración con Codex CLI y ChatGPT"
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Ingles) => {
            "🟢 Integration with Codex CLI & ChatGPT"
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Frances) => {
            "🟢 Intégration avec Codex CLI et ChatGPT"
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Aleman) => {
            "🟢 Integration mit Codex CLI und ChatGPT"
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Ruso) => "🟢 Интеграция с Codex CLI и ChatGPT",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::ChinoSimplificado) => {
            "🟢 与 Codex CLI 及 ChatGPT 集成"
        }
        (TemaDeAyuda::GeminiGems, Idioma::Espanol) => "🔵 Integración con Gemini CLI",
        (TemaDeAyuda::GeminiGems, Idioma::Ingles) => "🔵 Integration with Gemini CLI",
        (TemaDeAyuda::GeminiGems, Idioma::Frances) => "🔵 Intégration avec Gemini CLI",
        (TemaDeAyuda::GeminiGems, Idioma::Aleman) => "🔵 Integration mit Gemini CLI",
        (TemaDeAyuda::GeminiGems, Idioma::Ruso) => "🔵 Интеграция с Gemini CLI",
        (TemaDeAyuda::GeminiGems, Idioma::ChinoSimplificado) => "🔵 与 Gemini CLI 集成",
    }
}

/// Resumen breve del tema, en el idioma pedido.
///
/// Es el texto de la «galleta» emergente, la que lleva el botón `+info`.
pub fn resumen_de(tema: TemaDeAyuda, idioma: Idioma) -> &'static str {
    match (tema, idioma) {
        (TemaDeAyuda::PrimerosPasos, Idioma::Espanol) => "Haz clic central o derecho y arrastra para moverte. Usa la rueda del ratón para zoom.",
        (TemaDeAyuda::PrimerosPasos, Idioma::Ingles) => "Middle-click or right-click and drag to pan. Use the mouse wheel to zoom.",
        (TemaDeAyuda::PrimerosPasos, Idioma::Frances) => "Cliquez avec le bouton central ou droit et faites glisser pour vous déplacer. Utilisez la molette pour zoomer.",
        (TemaDeAyuda::PrimerosPasos, Idioma::Aleman) => "Klicken Sie mit der mittleren oder rechten Maustaste und ziehen Sie. Nutzen Sie das Mausrad zum Zoomen.",
        (TemaDeAyuda::PrimerosPasos, Idioma::Ruso) => "Нажмите среднюю или правую кнопку мыши и перетащите для перемещения. Колесико мыши — зум.",
        (TemaDeAyuda::PrimerosPasos, Idioma::ChinoSimplificado) => "按住鼠标中键或右键拖拽以平移画布。滚动滚轮可缩放视图。",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Espanol) => "Pulsa [Tab] para crear un hijo, [Enter] para un hermano y [Espacio] para editar.",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Ingles) => "Press [Tab] to create a child node, [Enter] for a sibling, and [Space] to edit text.",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Frances) => "Appuyez sur [Tab] pour créer un enfant, [Entrée] pour un frère et [Espace] pour éditer.",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Aleman) => "Drücken Sie [Tab] für einen Kindknoten, [Eingabe] für einen Geschwisterknoten und [Leertaste] zum Bearbeiten.",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Ruso) => "Нажмите [Tab] для дочернего узла, [Enter] для соседнего и [Пробел] для редактирования.",
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::ChinoSimplificado) => "按 [Tab] 创建子节点，按 [Enter] 创建同级节点，按 [空格键] 编辑文本。",
        (TemaDeAyuda::VisionDelCreador, Idioma::Espanol) => "Define qué pretendes conseguir con el proyecto para que la IA entienda el propósito real.",
        (TemaDeAyuda::VisionDelCreador, Idioma::Ingles) => "Define project goals so the AI thoroughly understands the authentic real-world purpose.",
        (TemaDeAyuda::VisionDelCreador, Idioma::Frances) => "Définissez ce que vous souhaitez accomplir pour que l'IA comprenne le but réel du projet.",
        (TemaDeAyuda::VisionDelCreador, Idioma::Aleman) => "Definieren Sie Ihre Projektziele, damit die KI den eigentlichen Zweck und Kontext versteht.",
        (TemaDeAyuda::VisionDelCreador, Idioma::Ruso) => "Определите цели проекта, чтобы ИИ понимал подлинный замысел и назначение.",
        (TemaDeAyuda::VisionDelCreador, Idioma::ChinoSimplificado) => "确立项目目标，使 AI 能够深刻领会真实世界的业务意图与开发初衷。",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Espanol) => "Asigna si el nodo es un pilar, subtema, duda o tarea para estructurar la jerarquía.",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Ingles) => "Assign whether a node is a pillar, subtopic, question, or task to structure hierarchy.",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Frances) => "Indiquez si le nœud est un pilier, un sous-thème, une question ou une tâche pour structurer la hiérarchie.",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Aleman) => "Legen Sie fest, ob ein Knoten eine Säule, ein Untermodul, eine Frage oder eine Aufgabe ist.",
        (TemaDeAyuda::RolesYArquitectura, Idioma::Ruso) => "Укажите, является ли узел модулем, подтемой, вопросом или задачей для построения структуры.",
        (TemaDeAyuda::RolesYArquitectura, Idioma::ChinoSimplificado) => "设定节点为支柱、子模块、疑问或任务，精确构建架构层级。",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Espanol) => "Marca si está en idea, en progreso, completado o si es una duda crítica a resolver.",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Ingles) => "Track whether a node is an idea, in progress, completed, or a critical blocker to resolve.",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Frances) => "Indiquez s'il s'agit d'une idée, d'un travail en cours, terminé ou d'un blocage critique à résoudre.",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Aleman) => "Kennzeichnen Sie Ideen, laufende Aufgaben, fertige Module oder kritische Blockaden.",
        (TemaDeAyuda::EstadosYProgreso, Idioma::Ruso) => "Отмечайте идеи, активную разработку, завершенные этапы или критические блокеры.",
        (TemaDeAyuda::EstadosYProgreso, Idioma::ChinoSimplificado) => "清晰标记构想、调研中、开发中、已完成或待攻坚的阻塞问题。",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Espanol) => "Vincula el nodo a una ruta de código real (ej. src/auth/jwt.rs) para que la IA programe ahí.",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Ingles) => "Link a node to a real codebase path (e.g. src/auth/jwt.rs) so AI writes code there.",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Frances) => "Associez le nœud à un chemin de code réel (ex. src/auth/jwt.rs) pour que l'IA y programme directement.",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Aleman) => "Verknüpfen Sie Knoten mit echten Pfaden (z. B. src/auth/jwt.rs), damit die KI dort programmiert.",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Ruso) => "Свяжите узел с реальным путем в коде (напр., src/auth/jwt.rs), чтобы ИИ программировал там.",
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::ChinoSimplificado) => "将节点与真实代码路径关联（如 src/auth/jwt.rs），以便 AI 准确落实现实代码。",
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Espanol) => "Veta caminos erróneos y exige correcciones a la IA para reajustar el rumbo del proyecto.",
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Ingles) => "Veto flawed paths and require explicit AI corrections to realign project direction.",
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Frances) => "Écartez les pistes erronées et exigez des rectifications à l'IA pour réajuster le cap du projet.",
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Aleman) => "Verwerfen Sie Fehlentwicklungen und fordern Sie KI-Korrekturen zur Neuausrichtung des Projekts.",
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Ruso) => "Блокируйте ошибочные пути и требуйте исправлений от ИИ для корректировки курса проекта.",
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::ChinoSimplificado) => "否决错误方案并向 AI 下达强制修正要求，精准调控项目研发方向。",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Espanol) => "El viaje entero: qué sale de tu equipo, qué decides tú y qué puede hacer la IA al volver.",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Ingles) => "The whole journey: what leaves your computer, what you decide, and what the AI may do back.",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Frances) => "Le voyage entier : ce qui sort de votre poste, ce que vous décidez et ce que l'IA peut faire.",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Aleman) => "Der ganze Weg: was Ihren Rechner verlässt, was Sie entscheiden und was die KI zurück darf.",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Ruso) => "Весь путь: что покидает ваш компьютер, что решаете вы и что ИИ вправе сделать в ответ.",
        (TemaDeAyuda::RecorridoConLaIa, Idioma::ChinoSimplificado) => "完整流程：哪些内容离开你的电脑、哪些由你决定，以及 AI 返回时能做什么。",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Espanol) => "Genera un documento Markdown con ingeniería de prompts y diagrama Mermaid listo para LLMs.",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Ingles) => "Generate a structured Markdown document with prompt engineering and Mermaid diagram for LLMs.",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Frances) => "Générez un document Markdown structuré avec ingénierie de prompts et diagramme Mermaid pour LLM.",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Aleman) => "Erzeugen Sie strukturiertes Markdown mit Prompt-Engineering und Mermaid-Diagramm für LLMs.",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Ruso) => "Генерируйте структурированный Markdown с промпт-инжинирингом и Mermaid-диаграммой для LLM.",
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::ChinoSimplificado) => "生成包含提示词工程规范与 Mermaid 拓扑图的结构化文档，专为大模型优化。",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Espanol) => "Pega respuestas de ChatGPT/Claude y MMCelt creará el mapa mental automáticamente.",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Ingles) => "Paste ChatGPT/Claude responses and MMCelt builds the mind map automatically.",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Frances) => "Collez les réponses de ChatGPT/Claude et MMCelt construira la carte heuristique automatiquement.",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Aleman) => "Fügen Sie ChatGPT-/Claude-Antworten ein und MMCelt baut die Mindmap automatisch auf.",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Ruso) => "Вставьте ответы ChatGPT/Claude, и MMCelt автоматически построит интеллект-карту.",
        (TemaDeAyuda::ImportarDesdeIA, Idioma::ChinoSimplificado) => "粘贴 ChatGPT/Claude 的回复，MMCelt 将自动构建完整的思维导图。",
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Espanol) => {
            "Guarda el mapa, exporta el documento para la IA y activa la recarga automática en bucle cerrado."
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Ingles) => {
            "Saves the map, exports the AI document, and enables closed-loop automatic live reload."
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Frances) => {
            "Enregistre la carte, exporte le document pour l'IA et active le rechargement automatique en boucle."
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Aleman) => {
            "Speichert die Mindmap, exportiert das KI-Dokument und aktiviert das automatische Neuladen."
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Ruso) => {
            "Сохраняет карту, экспортирует документ для ИИ и включает автоматическое обновление в замкнутом цикле."
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::ChinoSimplificado) => {
            "保存思维导图，导出 AI 协同文档并启用闭环自动热重载同步。"
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Espanol) => "Traza flechas de dependencia o bloqueo entre ramas distintas del proyecto.",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Ingles) => "Draw dependency or blocking arrows across different project branches.",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Frances) => "Tracez des flèches de dépendance ou de blocage entre différentes branches du projet.",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Aleman) => "Ziehen Sie Abhängigkeits- oder Blockadepfeile zwischen verschiedenen Projektzweigen.",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Ruso) => "Проводите стрелки зависимостей или блокировок между различными ветвями проекта.",
        (TemaDeAyuda::ConexionesCruzadas, Idioma::ChinoSimplificado) => "在不同项目分支之间建立依赖或阻塞连线，精准建模系统关系。",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Espanol) => "Organiza tus nodos automáticamente en árbol horizontal, radial o libre.",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Ingles) => "Automatically organize your nodes in balanced horizontal tree, radial, or free mode.",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Frances) => "Organisez vos nœuds automatiquement en arbre horizontal équilibré, radial ou libre.",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Aleman) => "Ordnen Sie Knoten automatisch als ausbalancierten horizontalen Baum, radial oder frei an.",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Ruso) => "Автоматически упорядочивайте узлы в виде сбалансированного дерева, радиально или свободно.",
        (TemaDeAyuda::ModosDeDisposicion, Idioma::ChinoSimplificado) => "一键将节点自动整理为均衡横向树、放射状或自由排版布局。",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Espanol) => "Carga estructuras probadas como Clean Architecture o Fullstack con un clic.",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Ingles) => "Load proven project structures like Clean Architecture or Fullstack in one click.",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Frances) => "Chargez des structures éprouvées telles que Clean Architecture ou Fullstack en un clic.",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Aleman) => "Laden Sie bewährte Projektstrukturen wie Clean Architecture oder Fullstack mit einem Klick.",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Ruso) => "Загружайте проверенные структуры, такие как Clean Architecture или Fullstack, в один клик.",
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::ChinoSimplificado) => "一键加载业界验证的最佳实践架构，如整洁架构或全栈应用模版。",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Espanol) => "Selecciona una carpeta y MMCelt mapeará tus archivos y módulos automáticamente.",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Ingles) => "Select a folder and MMCelt maps your files and modules automatically.",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Frances) => "Sélectionnez un dossier et MMCelt cartographiera vos fichiers et modules automatiquement.",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Aleman) => "Wählen Sie einen Ordner und MMCelt erfasst Dateien und Module automatisch in einer Mindmap.",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Ruso) => "Выберите папку, и MMCelt мгновенно преобразует структуру файлов в интеллект-карту.",
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::ChinoSimplificado) => "选定本地代码文件夹，MMCelt 将自动解析目录并构建思维导图。",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Espanol) => "Cada rama tiene su color y su descendencia lo hereda: sirve para seguir un hilo.",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Ingles) => "Each branch gets a distinct color inherited by descendants: follow any thread easily.",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Frances) => "Chaque branche possède sa propre couleur héritée par sa descendance : suivez chaque fil aisément.",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Aleman) => "Jeder Zweig erhält eine eigene Farbe, die vererbt wird: Verfolgen Sie Stränge mühelos.",
        (TemaDeAyuda::ColoresDelMapa, Idioma::Ruso) => "Каждая ветвь получает свой цвет, наследуемый потомками: легко отслеживать контекст.",
        (TemaDeAyuda::ColoresDelMapa, Idioma::ChinoSimplificado) => "每个主干分支拥有独立专属配色并由子孙继承，便于脉络追踪。",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Espanol) => "Dos mapas terminados, con notas y dudas, para ver cómo se anota uno de verdad.",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Ingles) => "Two complete maps with notes and questions showing how real projects are annotated.",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Frances) => "Deux cartes complètes, avec notes et questions, montrant comment documenter un projet réel.",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Aleman) => "Zwei fertige Mindmaps mit Notizen und Fragen zeigen, wie echte Projekte strukturiert werden.",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Ruso) => "Две готовые карты с примечаниями и вопросами показывают оформление реальных проектов.",
        (TemaDeAyuda::EjemplosIncluidos, Idioma::ChinoSimplificado) => "两份包含详尽备注与设问的完整导图，直观展现专业导图的标注规范。",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Espanol) => "Se guarda una copia cada dos minutos por si el programa se cierra sin avisar.",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Ingles) => "An automatic backup is saved every two minutes in case the program closes unexpectedly.",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Frances) => "Une sauvegarde automatique est effectuée toutes les deux minutes en cas de fermeture imprévue.",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Aleman) => "Alle zwei Minuten wird automatisch ein Backup erstellt, falls das Programm unerwartet schließt.",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Ruso) => "Резервная копия сохраняется каждые две минуты на случай непредвиденного закрытия.",
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::ChinoSimplificado) => "系统每两分钟自动保存一次快照副本，以防软件异常关闭导致数据丢失。",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Espanol) => "¿Se ve pequeño en un televisor? Ctrl + agranda todo, y se recuerda al reiniciar.",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Ingles) => "UI looks small on a TV? Ctrl + enlarges everything, and settings are remembered on restart.",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Frances) => "L'application semble petite sur un téléviseur ? Ctrl + agrandit tout, et le réglage est mémorisé.",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Aleman) => "Ansicht zu klein auf dem TV? Strg + vergrößert alles, und die Einstellung bleibt gespeichert.",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Ruso) => "Шрифт кажется мелким на ТВ? Ctrl + увеличивает всё, настройка сохраняется.",
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::ChinoSimplificado) => "在大屏幕或电视上看字体偏小？Ctrl + 可放大整体界面，设置会自动记忆。",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Espanol) => "Saber qué versión tienes abierta y desde qué archivo se está ejecutando.",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Ingles) => "Check active application version, commit hash, and running executable path.",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Frances) => "Connaître la version ouverte, le commit et le chemin du binaire en cours d'exécution.",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Aleman) => "Prüfen Sie aktive Version, Commit-Hash und den Pfad der aktuell ausgeführten Binärdatei.",
        (TemaDeAyuda::VersionYCompilacion, Idioma::Ruso) => "Узнайте активную версию программы, коммит и путь к запущенному исполняемому файлу.",
        (TemaDeAyuda::VersionYCompilacion, Idioma::ChinoSimplificado) => "查看当前运行的版本号、Git Commit 哈希以及执行文件的确切本地路径。",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Espanol) => "El servidor MCP permite que tus agentes de IA lean y actualicen mapas en disco.",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Ingles) => "The MCP server allows your AI agents to read and update maps on disk.",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Frances) => "Le serveur MCP permet à vos agents IA de lire et mettre à jour les cartes sur le disque.",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Aleman) => "Der MCP-Server ermöglicht es Ihren KI-Agenten, Maps auf dem Datenträger zu lesen und zu aktualisieren.",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Ruso) => "Сервер MCP позволяет агентам ИИ читать и обновлять интеллект-карты на диске.",
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::ChinoSimplificado) => "MCP 服务器允许您的 AI 智能体直接读取和更新本地磁盘上的导图。",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Espanol) => "Usa MMCelt como plano de arquitectura mientras Claude Code programa en tu terminal.",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Ingles) => "Use MMCelt as an architecture blueprint while Claude Code programs in your terminal.",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Frances) => "Utilisez MMCelt comme plan d'architecture pendant que Claude Code programme dans votre terminal.",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Aleman) => "Nutzen Sie MMCelt als Architekturplan, während Claude Code im Terminal entwickelt.",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Ruso) => "Используйте MMCelt как архитектурный план, пока Claude Code пишет код в терминале.",
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::ChinoSimplificado) => "当 Claude Code 在终端中编写代码时，将 MMCelt 作为可视化架构蓝图。",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Espanol) => "Aprovecha la IA de OpenAI en la terminal con Codex CLI o en el navegador con ChatGPT.",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Ingles) => "Leverage OpenAI's AI in the terminal with Codex CLI or in the browser with ChatGPT.",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Frances) => "Profitez de l'IA d'OpenAI dans le terminal avec Codex CLI ou dans le navigateur avec ChatGPT.",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Aleman) => "Nutzen Sie die KI von OpenAI im Terminal mit Codex CLI oder im Browser mit ChatGPT.",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Ruso) => "Используйте ИИ OpenAI в терминале с Codex CLI или в браузере с ChatGPT.",
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::ChinoSimplificado) => "在终端中使用 Codex CLI 或在浏览器中使用 ChatGPT 畅享 OpenAI 的 AI 能力。",
        (TemaDeAyuda::GeminiGems, Idioma::Espanol) => "Conecta Gemini CLI con MMCelt o usa una clave de API gratuita de Google AI Studio.",
        (TemaDeAyuda::GeminiGems, Idioma::Ingles) => "Connect Gemini CLI with MMCelt or use a free Google AI Studio API key.",
        (TemaDeAyuda::GeminiGems, Idioma::Frances) => "Connectez Gemini CLI avec MMCelt ou utilisez une clé d'API gratuite Google AI Studio.",
        (TemaDeAyuda::GeminiGems, Idioma::Aleman) => "Verbinden Sie Gemini CLI mit MMCelt oder nutzen Sie einen kostenlosen Google AI Studio API-Schlüssel.",
        (TemaDeAyuda::GeminiGems, Idioma::Ruso) => "Подключите Gemini CLI к MMCelt или используйте бесплатный ключ API Google AI Studio.",
        (TemaDeAyuda::GeminiGems, Idioma::ChinoSimplificado) => "将 Gemini CLI 连接至 MMCelt，或使用 Google AI Studio 免费 API 密钥。",
    }
}

/// Documento completo del tema, en Markdown y en el idioma pedido.
///
/// Sale del archivo `assets/ayuda/<idioma>/<Tema>.md`, incrustado en compilación.
pub fn guia_de(tema: TemaDeAyuda, idioma: Idioma) -> &'static str {
    match (tema, idioma) {
        (TemaDeAyuda::PrimerosPasos, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/PrimerosPasos.md")
        }
        (TemaDeAyuda::PrimerosPasos, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/PrimerosPasos.md")
        }
        (TemaDeAyuda::PrimerosPasos, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/PrimerosPasos.md")
        }
        (TemaDeAyuda::PrimerosPasos, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/PrimerosPasos.md")
        }
        (TemaDeAyuda::PrimerosPasos, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/PrimerosPasos.md")
        }
        (TemaDeAyuda::PrimerosPasos, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/PrimerosPasos.md")
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/AtajosYCreacionNodos.md")
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/AtajosYCreacionNodos.md")
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/AtajosYCreacionNodos.md")
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/AtajosYCreacionNodos.md")
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/AtajosYCreacionNodos.md")
        }
        (TemaDeAyuda::AtajosYCreacionNodos, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/AtajosYCreacionNodos.md")
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/VisionDelCreador.md")
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/VisionDelCreador.md")
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/VisionDelCreador.md")
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/VisionDelCreador.md")
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/VisionDelCreador.md")
        }
        (TemaDeAyuda::VisionDelCreador, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/VisionDelCreador.md")
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/RolesYArquitectura.md")
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/RolesYArquitectura.md")
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/RolesYArquitectura.md")
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/RolesYArquitectura.md")
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/RolesYArquitectura.md")
        }
        (TemaDeAyuda::RolesYArquitectura, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/RolesYArquitectura.md")
        }
        (TemaDeAyuda::EstadosYProgreso, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/EstadosYProgreso.md")
        }
        (TemaDeAyuda::EstadosYProgreso, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/EstadosYProgreso.md")
        }
        (TemaDeAyuda::EstadosYProgreso, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/EstadosYProgreso.md")
        }
        (TemaDeAyuda::EstadosYProgreso, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/EstadosYProgreso.md")
        }
        (TemaDeAyuda::EstadosYProgreso, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/EstadosYProgreso.md")
        }
        (TemaDeAyuda::EstadosYProgreso, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/EstadosYProgreso.md")
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/MapeoArchivosCodigo.md")
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/MapeoArchivosCodigo.md")
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/MapeoArchivosCodigo.md")
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/MapeoArchivosCodigo.md")
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/MapeoArchivosCodigo.md")
        }
        (TemaDeAyuda::MapeoArchivosCodigo, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/MapeoArchivosCodigo.md")
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ControlHumanoCorrecciones.md")
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ControlHumanoCorrecciones.md")
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ControlHumanoCorrecciones.md")
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ControlHumanoCorrecciones.md")
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ControlHumanoCorrecciones.md")
        }
        (TemaDeAyuda::ControlHumanoCorrecciones, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ControlHumanoCorrecciones.md")
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/RecorridoConLaIa.md")
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/RecorridoConLaIa.md")
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/RecorridoConLaIa.md")
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/RecorridoConLaIa.md")
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/RecorridoConLaIa.md")
        }
        (TemaDeAyuda::RecorridoConLaIa, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/RecorridoConLaIa.md")
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ExportarMarkdownIA.md")
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ExportarMarkdownIA.md")
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ExportarMarkdownIA.md")
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ExportarMarkdownIA.md")
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ExportarMarkdownIA.md")
        }
        (TemaDeAyuda::ExportarMarkdownIA, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ExportarMarkdownIA.md")
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ImportarDesdeIA.md")
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ImportarDesdeIA.md")
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ImportarDesdeIA.md")
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ImportarDesdeIA.md")
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ImportarDesdeIA.md")
        }
        (TemaDeAyuda::ImportarDesdeIA, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ImportarDesdeIA.md")
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/EnviarAYVigilancia.md")
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/EnviarAYVigilancia.md")
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/EnviarAYVigilancia.md")
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/EnviarAYVigilancia.md")
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/EnviarAYVigilancia.md")
        }
        (TemaDeAyuda::EnviarAYVigilancia, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/EnviarAYVigilancia.md")
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ConexionesCruzadas.md")
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ConexionesCruzadas.md")
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ConexionesCruzadas.md")
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ConexionesCruzadas.md")
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ConexionesCruzadas.md")
        }
        (TemaDeAyuda::ConexionesCruzadas, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ConexionesCruzadas.md")
        }
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ModosDeDisposicion.md")
        }
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ModosDeDisposicion.md")
        }
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ModosDeDisposicion.md")
        }
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ModosDeDisposicion.md")
        }
        (TemaDeAyuda::ModosDeDisposicion, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ModosDeDisposicion.md")
        }
        (TemaDeAyuda::ModosDeDisposicion, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ModosDeDisposicion.md")
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/PlantillasDeSoftware.md")
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/PlantillasDeSoftware.md")
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/PlantillasDeSoftware.md")
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/PlantillasDeSoftware.md")
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/PlantillasDeSoftware.md")
        }
        (TemaDeAyuda::PlantillasDeSoftware, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/PlantillasDeSoftware.md")
        }
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/EscanerDeRepositorios.md")
        }
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/EscanerDeRepositorios.md")
        }
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/EscanerDeRepositorios.md")
        }
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/EscanerDeRepositorios.md")
        }
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/EscanerDeRepositorios.md")
        }
        (TemaDeAyuda::EscanerDeRepositorios, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/EscanerDeRepositorios.md")
        }
        (TemaDeAyuda::ColoresDelMapa, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ColoresDelMapa.md")
        }
        (TemaDeAyuda::ColoresDelMapa, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ColoresDelMapa.md")
        }
        (TemaDeAyuda::ColoresDelMapa, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ColoresDelMapa.md")
        }
        (TemaDeAyuda::ColoresDelMapa, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ColoresDelMapa.md")
        }
        (TemaDeAyuda::ColoresDelMapa, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ColoresDelMapa.md")
        }
        (TemaDeAyuda::ColoresDelMapa, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ColoresDelMapa.md")
        }
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/EjemplosIncluidos.md")
        }
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/EjemplosIncluidos.md")
        }
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/EjemplosIncluidos.md")
        }
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/EjemplosIncluidos.md")
        }
        (TemaDeAyuda::EjemplosIncluidos, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/EjemplosIncluidos.md")
        }
        (TemaDeAyuda::EjemplosIncluidos, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/EjemplosIncluidos.md")
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/GuardadoYRecuperacion.md")
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/GuardadoYRecuperacion.md")
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/GuardadoYRecuperacion.md")
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/GuardadoYRecuperacion.md")
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/GuardadoYRecuperacion.md")
        }
        (TemaDeAyuda::GuardadoYRecuperacion, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/GuardadoYRecuperacion.md")
        }
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/TamanoDeLaInterfaz.md")
        }
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/TamanoDeLaInterfaz.md")
        }
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/TamanoDeLaInterfaz.md")
        }
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/TamanoDeLaInterfaz.md")
        }
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/TamanoDeLaInterfaz.md")
        }
        (TemaDeAyuda::TamanoDeLaInterfaz, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/TamanoDeLaInterfaz.md")
        }
        (TemaDeAyuda::VersionYCompilacion, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/VersionYCompilacion.md")
        }
        (TemaDeAyuda::VersionYCompilacion, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/VersionYCompilacion.md")
        }
        (TemaDeAyuda::VersionYCompilacion, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/VersionYCompilacion.md")
        }
        (TemaDeAyuda::VersionYCompilacion, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/VersionYCompilacion.md")
        }
        (TemaDeAyuda::VersionYCompilacion, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/VersionYCompilacion.md")
        }
        (TemaDeAyuda::VersionYCompilacion, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/VersionYCompilacion.md")
        }
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ClaudeMCPIntegracion.md")
        }
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ClaudeMCPIntegracion.md")
        }
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ClaudeMCPIntegracion.md")
        }
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ClaudeMCPIntegracion.md")
        }
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ClaudeMCPIntegracion.md")
        }
        (TemaDeAyuda::ClaudeMCPIntegracion, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ClaudeMCPIntegracion.md")
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ClaudeCodeTerminal.md")
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ClaudeCodeTerminal.md")
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ClaudeCodeTerminal.md")
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ClaudeCodeTerminal.md")
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ClaudeCodeTerminal.md")
        }
        (TemaDeAyuda::ClaudeCodeTerminal, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ClaudeCodeTerminal.md")
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/ChatGPTCustomGPT.md")
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/ChatGPTCustomGPT.md")
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/ChatGPTCustomGPT.md")
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/ChatGPTCustomGPT.md")
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/ChatGPTCustomGPT.md")
        }
        (TemaDeAyuda::ChatGPTCustomGPT, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/ChatGPTCustomGPT.md")
        }
        (TemaDeAyuda::GeminiGems, Idioma::Espanol) => {
            include_str!("../../assets/ayuda/es/GeminiGems.md")
        }
        (TemaDeAyuda::GeminiGems, Idioma::Ingles) => {
            include_str!("../../assets/ayuda/en/GeminiGems.md")
        }
        (TemaDeAyuda::GeminiGems, Idioma::Frances) => {
            include_str!("../../assets/ayuda/fr/GeminiGems.md")
        }
        (TemaDeAyuda::GeminiGems, Idioma::Aleman) => {
            include_str!("../../assets/ayuda/de/GeminiGems.md")
        }
        (TemaDeAyuda::GeminiGems, Idioma::Ruso) => {
            include_str!("../../assets/ayuda/ru/GeminiGems.md")
        }
        (TemaDeAyuda::GeminiGems, Idioma::ChinoSimplificado) => {
            include_str!("../../assets/ayuda/zh/GeminiGems.md")
        }
    }
}
