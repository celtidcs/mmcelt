Conectar MMCelt con tu inteligencia artificial

MMCelt funciona solo, sin conectarse a nada, pero su razón de ser es el ida y vuelta con un modelo
de inteligencia artificial: tú dibujas el plano y la IA lo programa, o la IA propone y tú corriges
el rumbo sobre el mapa. Esta carpeta reúne lo necesario para montar ese ida y vuelta con cada
plataforma.

Qué conecta MMCelt hoy

Desde la versión 0.11.6 el programa se conecta con tres agentes de consola: Claude Code, Codex CLI y
Antigravity CLI (agy). Claude Desktop, el IDE antiguo de Antigravity, Cursor y Windsurf ya no se
conectan desde MMCelt, porque obligaban a varios pasos manuales entre ventanas y a reiniciar programas,
y el programa solo ofrece las conexiones comprobadas de principio a fin. El motivo completo está en el
apartado «Conectarlo con tu IA» del README.md de la raíz del repositorio. Lo que estas guías cuentan de
esos entornos se conserva como referencia de cómo funciona cada plataforma, no como conexión disponible.

Por dónde empezar

Si es la primera vez, lee el manual de control con IA, que está en el documento
manual-de-control-ia.md de la carpeta documentacion/integraciones. No explica menús: explica los dos
flujos de trabajo, ser el arquitecto o ser el supervisor, y cómo mantener alineado un proyecto largo
sin perder el control. Lo demás son instrucciones de configuración que solo tienen sentido si
entiendes para qué sirven.

Guía por plataforma

Para Claude Code y Claude Desktop consulta el documento claude-mcp.md de esta misma carpeta: explica
la conexión automática por el servidor MCP, con la que el agente lee y escribe tus mapas sin copiar y
pegar. Para ChatGPT, los GPT personalizados y Codex consulta el documento chatgpt.md, que separa el
chat web, Codex CLI y la aplicación Codex. Para Gemini, los Gems y Antigravity CLI (agy) consulta
el documento gemini.md, que explica además cómo aprovechar el contexto largo para mapas grandes.

El servidor MCP va integrado en el propio ejecutable y permite una comunicación bidireccional directa
con los agentes compatibles. Con las plataformas web, el puente es la exportación e importación
estructurada de documentos Markdown y JSON.

Plantillas para copiar y pegar

En la carpeta documentacion/integraciones/plantillas están los textos que hay que pegar en la casilla
de instrucciones al crear un asistente personalizado. El archivo custom-gpt.txt se pega en el campo
Instructions al crear un Custom GPT, y el archivo gem.txt se pega en las instrucciones de un Gem de
Google.

Para los agentes que admiten skills hay una definición completa en el archivo
skills/mmcelt-mindmap/SKILL.md del repositorio, que se coloca en la carpeta de skills del agente en
lugar de pegarse a mano.

Conexión automática desde la aplicación

Si solo quieres que funcione, la aplicación lo hace sola desde el menú «Inteligencia Artificial →
Conectar MMCelt con mis IAs...». Detecta los agentes instalados en el equipo y se registra en ellos.
