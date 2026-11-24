Antigravity CLI y Gemini web: dos recorridos distintos

MMCelt puede trabajar con los modelos de Google por varios clientes, pero no los abre ni los controla
a todos de la misma manera. Antigravity CLI (agy) es una consola interactiva oficial de Google;
Gemini web es una conversación en el navegador. Compartir la familia de modelos no hace que compartan
conversación, permisos ni herramientas.

Qué conecta MMCelt hoy

El programa se conecta con tres agentes oficiales de consola: Claude Code, Codex CLI y Antigravity
CLI (agy). Google ha sustituido Gemini CLI por Antigravity CLI. Lo que esta
guía cuenta sobre el entorno gráfico antiguo de Antigravity se conserva únicamente como referencia
histórica de cómo funcionaba esa plataforma, que ya no se conecta desde MMCelt. El motivo está en el
apartado «Conectarlo con tu IA» del README.md de la raíz del repositorio.

Con Antigravity CLI (agy), «Enviar a...» abre la consola interactiva con el expediente; usa el MCP
local después de conectar y reiniciar, y tú sigues escribiendo en la terminal que abre MMCelt. Con
Gemini web o un Gem, adjuntas o pegas el archivo _AI.md; un chat web ordinario no usa el MCP local y la
conversación sigue en el navegador.

Acceso con cuenta normal de Google

A diferencia de clientes anteriores que requerían una clave de API o cuentas autorizadas especiales,
Antigravity CLI (agy) admite el inicio de sesión interactivo con una cuenta personal estándar de
Google. No hace falta configurar claves de API en Google AI Studio ni utilizar cuentas de pago. MMCelt
se integra con Antigravity CLI preparando el envío, redactando el expediente y vigilando la actividad
en el mapa.

Preparación común para los clientes locales

Abre el mapa y guárdalo dentro de la carpeta del proyecto. Entra en «Inteligencia Artificial →
Conectar MMCelt con mis IAs...» y comprueba que la carpeta autorizada sea la raíz del proyecto.
Conecta Antigravity CLI (agy) y después cierra por completo la consola y vuelve a abrirla, porque la
configuración MCP se carga al arrancar.

MMCelt registra su propio ejecutable con la opción --mcp-server en el archivo mcp_config.json de
Antigravity y limita el acceso mediante la variable MMCELT_WORKSPACE. La documentación oficial de
Google Antigravity contempla dos ubicaciones para este registro: la configuración global en la
carpeta de configuración del usuario, que es la gestionada por MMCelt, y la configuración local
por espacio de trabajo en la carpeta .agents del proyecto. MMCelt configura la ubicación global
para asegurar la disponibilidad en todas las sesiones y no pide claves de API ni elige el modelo:
utiliza la sesión autenticada con tu cuenta de Google.

Con Antigravity CLI, MMCelt solo trabaja
con la cuenta por defecto. Claude Code y Codex CLI permiten tener varias cuentas, cada una en su
carpeta, y elegirla con una variable de entorno; agy no documenta nada parecido y siempre lee su
configuración de la misma carpeta. Por eso en la ventana de conexiones no se pueden añadir cuentas
de agy a mano: no se usarían ni al conectar ni al abrir la consola.

Antigravity CLI: MMCelt abre la consola

No hace falta abrirla antes. Elige «Enviar a... → Antigravity CLI (agy)», revisa el prompt y pulsa
«Iniciar en consola». MMCelt guarda el mapa, genera el archivo _AI.md, escribe el expediente en
.mmcelt/sesiones/<sesión>/inicio.md, abre Antigravity CLI con la opción -i en la carpeta del proyecto y
activa la vigilancia. A partir de ahí sigues escribiendo en esa terminal.

Pide al agente que empiece por mmcelt_workspace_info, lea el mapa y consulte las correcciones humanas
antes de modificarlo. Para devolver avances debe usar mmcelt_sync_ai_progress.

Gemini web y Gems: copia manual

Guarda el mapa, expórtalo con Ctrl+E y adjunta el _AI.md a la conversación. Gemini aprovecha bien un
contexto largo, así que puedes adjuntar también archivos de código o documentación junto al mapa.
Para traer una respuesta, pide el JSON compatible con MMCelt y usa «Importar desde IA (ChatGPT,
Claude, Gemini)...». Subir un archivo al chat no equivale a conceder acceso al servidor MCP local.

Si quieres un asistente permanente que hable siempre el formato de MMCelt, crea un Gem y pega en sus
instrucciones el contenido del archivo gem.txt de la carpeta documentacion/integraciones/plantillas,
o el de skills/mmcelt-mindmap/SKILL.md.

Si no aparece el mapa actualizado

Si no ves las herramientas, reinicia el cliente y revisa la carpeta registrada en MMCelt. Si la
consola ve MCP pero no escribe el mapa, pide expresamente que use mmcelt_sync_ai_progress. Si MMCelt
conserva cambios humanos sin guardar, no recargará por encima de ellos: guarda o descarta esos
cambios de forma consciente.
