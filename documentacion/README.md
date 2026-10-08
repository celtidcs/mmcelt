# MMCelt — Documentación

<img src="../assets/icono/mmcelt-128.png" alt="Icono de MMCelt" width="96" height="96">

**MMCelt** es una aplicación de escritorio para construir mapas mentales cuyo propósito
no es solo organizar ideas para uno mismo, sino **dirigir, acotar y supervisar el trabajo
de modelos y agentes de inteligencia artificial**.

## El problema que resuelve

Cuando se trabaja con una IA en proyectos de cierta envergadura, la conversación tiende a
desordenarse: el modelo pierde el hilo de la intención original, reabre caminos que ya se
habían descartado e inventa soluciones que contradicen decisiones previas.

MMCelt resuelve este problema transformando el proyecto en un **mapa visual estructurado con estados,
prioridades y dependencias**. El usuario define qué está completado, qué está en progreso, qué
dudas técnicas bloquean el camino y qué opciones han sido formalmente descartadas. A partir de ese
mapa, MMCelt exporta un documento Markdown jerárquico que la IA comprende de una sola vez, o se
comunica directamente con agentes autónomos mediante su protocolo MCP local.

El flujo es **bidireccional**: la persona diseña y supervisa; la IA analiza, propone y actualiza
el mapa con sus avances.

```
     ┌──────────────┐   exportar Markdown    ┌──────────────┐
     │              │ ─────────────────────► │              │
     │   MMCelt     │                        │  IA (Claude, │
     │  (escritorio)│                        │  ChatGPT,    │
     │              │ ◄───────────────────── │  Gemini…)    │
     └──────────────┘   importar JSON / OPML └──────────────┘
            ▲
            │  servidor MCP local (lectura y sincronización)
            ▼
     ┌──────────────┐
     │ Agente de IA │
     │  autónomo    │
     │  (CLI / MCP) │
     └──────────────┘
```

## Tecnologías

| Componente | Tecnología |
|---|---|
| Aplicación de escritorio | **Rust** 1.95+ con `egui` / `eframe` 0.36 (modo inmediato, aceleración por GPU con backend `glow`) |
| Persistencia nativa | JSON estructurado mediante `serde`, en archivos `.mmcelt` |
| Formatos de intercambio | Estándares abiertos no propietarios: **OPML** y **FreeMind / Freeplane (`.mm`)** |
| Servidor MCP integrado | `mmcelt --mcp-server`, protocolo JSON-RPC 2.0 (2024-11-05) sobre entrada/salida estándar con 6 herramientas |
| Diálogos del sistema | `rfd` (diálogos nativos de selección de archivos y carpetas) |
| Vigilancia de archivos | `notify` 8.0 (observador reactivo del sistema de archivos para sincronización en tiempo real) |
| Aviso de versión nueva | `ureq` 3.4 con `rustls` (una consulta HTTPS anónima a GitHub al arrancar, desactivable) |
| Tipografía internacional | Subconjunto embebido de **Noto Sans SC** para soporte CJK completo (chino simplificado) sin dependencias |

**Privacidad y soberanía de datos**: La aplicación no requiere conexión a internet para funcionar,
no aloja datos en la nube ni recopila telemetría. Su única petición de red es una consulta anónima
a GitHub al arrancar para avisar de versiones nuevas, que se desactiva desde el menú «🎨 Ver y
Diseño». Toda la comunicación con agentes de IA se realiza en
local (mediante la consola del sistema o el servidor MCP en `stdio`).

## Puesta en marcha rápida

```bash
git clone <url-del-repositorio> mmcelt
cd mmcelt
cargo run --release
```

Requiere Rust 1.95 o superior. **No hay dependencias externas pesadas**: el servidor MCP y todos los
activos de ayuda e idiomas van integrados dentro del propio ejecutable.

La guía completa está en [`infraestructura/guia-de-clonado.md`](infraestructura/guia-de-clonado.md), y
se incluyen scripts automatizados de preparación de entorno: [`preparar-entorno.ps1`](infraestructura/preparar-entorno.ps1) para Windows y [`preparar-entorno.sh`](infraestructura/preparar-entorno.sh) para Linux y macOS.

## Índice de la documentación

| Documento | Contenido |
|---|---|
| [`funcionalidades.md`](funcionalidades.md) | Catálogo exhaustivo de características: lienzo, nodos, estados, prioridades, conexiones cruzadas y formatos |
| [`manual-de-uso.md`](manual-de-uso.md) | Manual para el usuario: flujos de trabajo paso a paso, atajos de teclado y trabajo con agentes |
| [`arquitectura.md`](arquitectura.md) | Diseño del software, arquitectura de módulos, separación de responsabilidades y decisiones técnicas |
| [`historial-de-cambios.md`](historial-de-cambios.md) | Registro cronológico detallado de cambios y versiones publicadas |
| [`defectos-conocidos.md`](defectos-conocidos.md) | Registro de comportamientos anómalos conocidos y estado de investigación |
| [`integraciones/`](integraciones/) | Guías de integración y conexión con agentes (Claude Code, Codex CLI, Gemini CLI) |
| [`infraestructura/vision-general.md`](infraestructura/vision-general.md) | Mapa de componentes del entorno y dependencias del sistema |
| [`infraestructura/requisitos.md`](infraestructura/requisitos.md) | Requisitos mínimos y recomendados de hardware y compilación |
| [`infraestructura/servicios.md`](infraestructura/servicios.md) | Servicios externos, configuración de agentes y protocolo MCP |
| [`infraestructura/guia-de-clonado.md`](infraestructura/guia-de-clonado.md) | Instrucciones paso a paso para clonar y compilar en cualquier sistema |

## Integración con Agentes de Inteligencia Artificial

MMCelt admite dos modalidades principales de trabajo con IA:

1. **Flujo Asistido (Copiar / Pegar)**: Exportación de esquemas Markdown estructurados (`Ctrl + E`) y posterior importación de respuestas en JSON o formatos estándar OPML / `.mm`.
2. **Flujo Autónomo (Consola y MCP)**: Conexión con agentes de línea de comandos mediante el menú `🤖 Inteligencia Artificial → Conectar MMCelt con mis IAs`. Se soportan de forma oficial los agentes de terminal:
   - **Claude Code** (Anthropic)
   - **Codex CLI** (OpenAI)
   - **Gemini CLI** (Google, mediante clave de API gratuita de Google AI Studio)

## Estado del proyecto

- **Versión vigente**: **0.13.0**
- **Estado de verificación**:
  - Windows: **611 pruebas unitarias e integradas** pasadas al 100%.
  - Linux (`mmcelt-linux`): **610 pruebas** pasadas al 100% (una es exclusiva de Windows).
  - Auditoría de código: `cargo clippy --all-targets --all-features -- -D warnings` (0 advertencias).
  - Formato: `cargo fmt --check` (100% conforme).
  - Documentación: `cargo doc --all-features --no-deps` (0 advertencias).
  - Seguridad: `cargo audit` (0 vulnerabilidades reportadas sobre 441 dependencias).
  - Validación manual: la versión publicada se comprobó a mano, recorriendo cada novedad, antes de
    publicarla; los defectos que salieron se corrigieron antes de publicar.
