# Conectar MMCelt con tu inteligencia artificial

MMCelt funciona solo, sin conectarse a nada. Pero su razón de ser es el ida y vuelta con
un modelo de IA: tú dibujas el plano y la IA lo programa, o la IA propone y tú corriges el
rumbo sobre el mapa.

Esta carpeta reúne todo lo necesario para montar ese ida y vuelta, con cualquiera de las
plataformas.

## Por dónde empezar

Si es la primera vez, lee el **[manual de control con IA](manual-de-control-ia.md)**. No
explica menús: explica los dos flujos de trabajo —ser el arquitecto o ser el supervisor— y
cómo mantener alineado un proyecto largo sin perder el control. Lo demás son instrucciones
de configuración que solo tienen sentido si entiendes para qué.

## Guía por plataforma

| Tu IA | Guía | Qué consigues |
|---|---|---|
| Claude Desktop, Claude Code | [`claude-mcp.md`](claude-mcp.md) | Conexión **automática** por el servidor MCP: el agente lee y escribe tus mapas sin copiar y pegar |
| ChatGPT, Custom GPTs, Cursor, Codex | [`chatgpt.md`](chatgpt.md) | Un GPT personalizado que entiende el formato de MMCelt |
| Gemini, Gems, Google Antigravity | [`gemini.md`](gemini.md) | Un Gem que aprovecha el contexto largo para mapas grandes y skill nativo |

El servidor MCP integrado en el ejecutable permite comunicación **bidireccional directa** con agentes compatibles (Claude Desktop, Claude Code, Google Antigravity, Cursor, Windsurf y Codex CLI). Con las demás plataformas web, el puente es la importación/exportación estructurada de documentos Markdown y JSON.

## Plantillas para copiar y pegar

En [`plantillas/`](plantillas/) están los textos que hay que pegar en la casilla de
instrucciones al crear un asistente personalizado:

| Archivo | Dónde se pega |
|---|---|
| [`plantillas/custom-gpt.txt`](plantillas/custom-gpt.txt) | En «Instructions», al crear un Custom GPT |
| [`plantillas/gem.txt`](plantillas/gem.txt) | En las instrucciones de un Gem de Google |

Para los agentes que admiten *skills* —Claude Code y Antigravity— hay una definición completa en
[`../../skills/mmcelt-mindmap/SKILL.md`](../../skills/mmcelt-mindmap/SKILL.md), que va en
la carpeta de skills del agente en lugar de pegarse a mano.

## Conexión automática desde la aplicación

Si solo quieres que funcione, la aplicación lo hace sola: menú
**`🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...`**. Detecta los agentes
instalados en el equipo y se registra en ellos.
