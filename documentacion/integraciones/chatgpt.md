ChatGPT y Codex: qué puede conectarse y cómo se usa

OpenAI ofrece varias formas de conversar con un modelo, pero no todas pueden abrir los archivos de tu
ordenador. Esta guía separa tres casos que se parecen por el nombre y se comportan de forma distinta:
ChatGPT web, Codex CLI y la aplicación Codex. MMCelt nunca necesita una clave de API ni contrata
consumo por su cuenta. La sesión y sus límites siguen perteneciendo a la herramienta oficial en la
que ya hayas iniciado sesión.

Elige el recorrido que corresponde a tu pantalla

No necesitas abrir Codex CLI antes de usar «Enviar a...»: MMCelt lo abre después de que revises y
confirmes el prompt. La aplicación Codex sí la abres tú, porque MMCelt no controla sus tareas ni
decide en qué conversación debe entrar. En ambos casos conviene haber iniciado sesión previamente en
el cliente oficial; MMCelt no gestiona esa cuenta.

En una conversación normal de ChatGPT web, el mapa llega como archivo adjunto o como texto copiado y
la conversación sigue en el propio chat. Esa conversación web no puede usar el servidor MCP local de
MMCelt, porque no lee el archivo local ~/.codex/config.toml.

Con Codex CLI en una terminal, «Enviar a...» abre codex con el expediente de la sesión. Sí puede usar
el servidor MCP local, después de conectar MMCelt y reiniciar Codex CLI, y la conversación sigue en
la terminal que abre MMCelt.

Con la aplicación Codex en el mismo equipo, abres o continúas una tarea local y le indicas que lea el
expediente. Puede usar el servidor MCP local si ese host admite MCP local y ha cargado la
configuración compartida. La conversación sigue en la tarea de la aplicación y, cuando proceda, en su
terminal integrada.

La documentación oficial de MCP de OpenAI, en https://learn.chatgpt.com/docs/extend/mcp?surface=cli,
confirma dos límites importantes: los clientes locales de Codex comparten la configuración MCP del
mismo host, mientras ChatGPT web no lee esa configuración local. Por eso MMCelt no promete que un chat
abierto en el navegador pueda escribir directamente en tu mapa.

ChatGPT web: recorrido con copia manual

Es el método universal. Funciona aunque no tengas una consola instalada y no modifica ninguna
configuración.

Para enviar un mapa a la conversación, primero guárdalo en MMCelt. Después pulsa Ctrl+E o abre
«Inteligencia Artificial → Exportar Archivo .md para IA...». Adjunta el archivo _AI.md a la
conversación o copia su contenido, y escribe el objetivo que quieres conseguir. El documento ya
incluye la estructura, las notas, las decisiones humanas y el contrato de devolución.

Para traer una respuesta al mapa, usa «Inteligencia Artificial → Copiar Prompt Maestro para IA...» y
pégalo en la conversación junto con el material que quieras convertir. Pide que la respuesta incluya
únicamente el bloque JSON compatible con MMCelt. Por último abre «Inteligencia Artificial → Importar
desde IA (ChatGPT, Claude, Gemini)...», pega la respuesta y confírmala.

Una conversación web ordinaria no puede escribir por sí sola en el archivo .mmcelt de tu disco. Que
ChatGPT permita adjuntar o descargar archivos no equivale a tener acceso al servidor MCP local. Los
plugins remotos de un espacio ChatGPT Work son otra arquitectura y no convierten automáticamente a
MMCelt, que es un servidor STDIO local, en una herramienta del navegador.

Codex CLI: mapa, MCP y conversación en la misma sesión

Este es el recorrido más directo con OpenAI cuando quieres trabajar desde una consola.

La preparación se hace una sola vez por carpeta autorizada. Abre el mapa que vas a utilizar y entra
en «Inteligencia Artificial → Conectar MMCelt con mis IAs...». Comprueba la carpeta que se
autorizará: debe ser la raíz del proyecto, no una carpeta general de documentos. Pulsa «Conectar» o
«Actualizar la carpeta» en la fila de Codex, y después cierra y vuelve a abrir Codex CLI para que lea
su configuración.

MMCelt añade una entrada mcp_servers.mmcelt al archivo config.toml de la cuenta de Codex, que es
~/.codex/config.toml para la cuenta por defecto, o el de la carpeta que señale CODEX_HOME o la de
cualquier otra cuenta que aparezca en su propia ficha. Conserva el resto del
archivo y deja una copia antes de modificarlo. La entrada ejecuta el mismo binario con la opción
--mcp-server y define la variable MMCELT_WORKSPACE, que es el límite real de lectura y escritura.

Cada vez que envíes trabajo, elige «Inteligencia Artificial → Enviar a... → Codex CLI». Revisa la
vista previa, que no es un prompt editable sino el texto exacto que saldrá: verás el contrato fijo de
MMCelt, las reglas comunes del proyecto, el contexto generado desde el mapa y el encargo concreto de
esta sesión. Si quieres cambiar las reglas o el encargo, cancela y hazlo en la ventana «Proyecto e
instrucciones para la IA»; nada sale mientras no confirmes. Cuando esté listo, pulsa «Iniciar en consola».

MMCelt guarda entonces el mapa, genera el _AI.md, conserva el prompt exacto en
.mmcelt/sesiones/<sesión>/inicio.md, abre Codex CLI en la carpeta del proyecto y empieza a vigilar el
mapa. Puedes seguir escribiendo instrucciones en esa consola sin volver a preparar el mapa. El agente
puede consultar las correcciones nuevas mediante MCP y devolver avances con mmcelt_sync_ai_progress;
cuando el archivo cambia de forma válida y no hay trabajo humano sin guardar, MMCelt lo recarga.

La consola y MCP son cosas distintas. La consola lleva tu conversación; MCP proporciona herramientas
para leer, validar y actualizar el mapa. Juntos permiten conversar y ver el resultado estructurado,
pero cada capacidad se comprueba por separado en la ventana «Enviar a...».

Aplicación Codex: misma configuración, tarea distinta

La aplicación Codex puede compartir la configuración MCP con Codex CLI cuando ambos trabajan en el
mismo host. Eso permite que una tarea local vea las herramientas de MMCelt después de conectar y
reiniciar el cliente. Sin embargo, MMCelt no fuerza una tarea existente ni escribe mensajes dentro de
una conversación que ya tengas abierta.

El recorrido seguro es este. Conecta Codex desde MMCelt y reinicia la aplicación Codex. En «Enviar
a...», prepara la sesión para Codex; si MMCelt detecta la CLI, puede abrir su consola, pero preparar
una sesión no crea por sí solo una tarea en otra aplicación. En tu tarea local de Codex, pide que lea
.mmcelt/sesiones/<sesión>/inicio.md o el _AI.md que MMCelt acaba de generar. Comprueba en Codex que
el servidor mmcelt está disponible antes de pedir una modificación, y continúa hablando en la tarea.
El agente devuelve el trabajo mediante las mismas herramientas MCP que usa la CLI.

Si esa instalación o ese host no expone MCP local, utiliza el recorrido de copia manual. Es una
limitación de la conexión, no un fallo del mapa.

Qué debe hacer el agente al comenzar

Independientemente de la interfaz de OpenAI, el orden correcto es el mismo. Primero consulta
mmcelt_workspace_info para conocer la raíz autorizada y las rutas reales. Después lee el mapa con
mmcelt_read_mindmap y, antes de un cambio sensible, lee las decisiones humanas con
mmcelt_get_human_feedback. Entonces trabaja sobre el código o el documento solicitado, devuelve el
avance con mmcelt_sync_ai_progress y vuelve a leer el mapa si la persona ha corregido el rumbo. En
su última entrega pasa work_finished con valor true: mientras no lo hace, MMCelt mantiene el mapa
bloqueado para la persona.

El agente no debe editar el JSON del mapa a mano. Las herramientas MCP preservan los identificadores,
impiden salir de la carpeta autorizada, rechazan títulos ambiguos y evitan que el agente se atribuya
una aprobación humana.

Si algo no funciona

Si Codex no muestra las herramientas, cierra el cliente por completo, vuelve a abrirlo y comprueba la
fila de Codex en «Conectar MMCelt con mis IAs...». Si MMCelt cambió de ubicación, pulsa «Conectar»
en esa fila para volver a registrar la ruta del ejecutable.

Si la consola se abre pero el mapa no cambia, recuerda que hablar en la consola no obliga al modelo a
llamar a MCP. Pídele que consulte mmcelt_workspace_info y que devuelva el avance con
mmcelt_sync_ai_progress.

Si MMCelt avisa de cambios humanos sin guardar, guarda o descarta conscientemente tus cambios y repite
la lectura. MMCelt no sustituye silenciosamente tu copia en memoria.

Si quieres evitar cualquier API de pago, inicia sesión únicamente en los clientes oficiales que ya
incluya tu cuenta. MMCelt no solicita ni almacena claves de API, no elige el modelo y no cambia la
facturación. Antes de usarlo en serio, comprueba los límites de la cuenta en el propio cliente.
