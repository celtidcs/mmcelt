# ChatGPT y Codex: qué puede conectarse y cómo se usa

OpenAI ofrece varias formas de conversar con un modelo, pero no todas pueden abrir los archivos de
tu ordenador. Esta guía separa tres casos que se parecen por el nombre y se comportan de forma
distinta: **ChatGPT web**, **Codex CLI** y la **aplicación Codex**. MMCelt nunca necesita una clave
de API ni contrata consumo por su cuenta. La sesión y sus límites siguen perteneciendo a la
herramienta oficial en la que ya hayas iniciado sesión.

## Elige el recorrido que corresponde a tu pantalla

No necesitas abrir Codex CLI antes de usar `Enviar a…`: MMCelt lo abre después de que revises y
confirmes el prompt. La aplicación Codex sí la abres tú, porque MMCelt no controla sus tareas ni
decide en qué conversación debe entrar. En ambos casos conviene haber iniciado sesión previamente
en el cliente oficial; MMCelt no gestiona esa cuenta.

| Dónde estás trabajando | Cómo recibe el mapa | ¿Puede usar el servidor MCP local de MMCelt? | Cómo conversas |
|---|---|---|---|
| ChatGPT web en una conversación normal | Archivo adjunto o texto copiado | No. Esa conversación web no lee el archivo local `~/.codex/config.toml` | En el propio chat |
| Codex CLI en una terminal | «Enviar a…» abre `codex` con el expediente de la sesión | Sí, después de conectar MMCelt y reiniciar Codex CLI | En la terminal que abre MMCelt |
| Aplicación Codex en el mismo equipo | Abres o continúas una tarea local y le indicas que lea el expediente | Sí, si ese host admite MCP local y ha cargado la configuración compartida | En la tarea de la aplicación y, cuando proceda, en su terminal integrada |

La [documentación oficial de MCP de OpenAI](https://learn.chatgpt.com/docs/extend/mcp?surface=cli)
confirma dos límites importantes: los clientes locales de Codex comparten la configuración MCP del
mismo host, mientras ChatGPT web no lee esa configuración local. Por eso MMCelt no promete que un
chat abierto en el navegador pueda escribir directamente en tu mapa.

## ChatGPT web: recorrido con copia manual

Es el método universal. Funciona aunque no tengas una consola instalada y no modifica ninguna
configuración.

### Enviar un mapa a la conversación

1. Guarda el mapa en MMCelt.
2. Pulsa `Ctrl+E` o abre `🤖 Inteligencia Artificial → Exportar Markdown para IA`.
3. Adjunta el archivo `_AI.md` a la conversación o copia su contenido.
4. Escribe el objetivo que quieres conseguir. El documento ya incluye la estructura, las notas,
   las decisiones humanas y el contrato de devolución.

### Traer una respuesta al mapa

1. Usa `🤖 Inteligencia Artificial → Copiar Prompt Maestro` y pégalo en la conversación junto con
   el material que quieras convertir.
2. Pide que la respuesta incluya únicamente el bloque JSON compatible con MMCelt.
3. Abre `🤖 Inteligencia Artificial → Importar desde IA`, pega la respuesta y pulsa **Convertir**.

Una conversación web ordinaria **no puede escribir** por sí sola en el `.mmcelt` de tu disco. Que
ChatGPT permita adjuntar o descargar archivos no equivale a tener acceso al servidor MCP local. Los
plugins remotos de un espacio ChatGPT Work son otra arquitectura y no convierten automáticamente a
MMCelt, que es un servidor STDIO local, en una herramienta del navegador.

## Codex CLI: mapa, MCP y conversación en la misma sesión

Este es el recorrido más directo con OpenAI cuando quieres trabajar desde una consola.

### Preparación, una sola vez por carpeta autorizada

1. Abre el mapa que vas a utilizar.
2. Entra en `🤖 Inteligencia Artificial → Conectar MMCelt con mis IAs`.
3. Comprueba la carpeta que se autorizará. Debe ser la raíz del proyecto, no una carpeta general de
   documentos.
4. Pulsa **Conectar** o **Actualizar la carpeta** en la fila de Codex.
5. Cierra y vuelve a abrir Codex CLI para que lea su configuración.

MMCelt añade una entrada `mcp_servers.mmcelt` a `~/.codex/config.toml`, conserva el resto del archivo
y deja una copia antes de modificarlo. La entrada ejecuta el mismo binario con `--mcp-server` y
define `MMCELT_WORKSPACE`, que es el límite real de lectura y escritura.

### Cada vez que envíes trabajo

1. Elige `🤖 Inteligencia Artificial → Enviar a… → Codex CLI`.
2. Revisa el **prompt editable**. Verás el contrato fijo de MMCelt, las reglas comunes del proyecto,
   el contexto generado desde el mapa y el encargo concreto de esta sesión.
3. Cambia las reglas o el encargo si lo necesitas. Nada sale mientras no confirmes.
4. Pulsa **Iniciar en consola**.

MMCelt guarda el mapa, genera `_AI.md`, conserva el prompt exacto en
`.mmcelt/sesiones/<sesión>/inicio.md`, abre Codex CLI en la carpeta del proyecto y empieza a vigilar
el mapa. Puedes seguir escribiendo instrucciones en esa consola sin volver a preparar el mapa. El
agente puede consultar las correcciones nuevas mediante MCP y devolver avances con
`mmcelt_sync_ai_progress`; cuando el archivo cambia de forma válida y no hay trabajo humano sin
guardar, MMCelt lo recarga.

La consola no es el canal MCP. La consola lleva tu conversación; MCP proporciona herramientas para
leer, validar y actualizar el mapa. Juntos permiten conversar y ver el resultado estructurado, pero
cada capacidad se comprueba por separado en la ventana «Enviar a…».

## Aplicación Codex: misma configuración, tarea distinta

La aplicación Codex puede compartir la configuración MCP con Codex CLI cuando ambos trabajan en el
mismo host. Eso permite que una tarea local vea las herramientas de MMCelt después de conectar y
reiniciar el cliente. Sin embargo, MMCelt no fuerza una tarea existente ni escribe mensajes dentro
de una conversación que ya tengas abierta.

El recorrido seguro es:

1. Conecta Codex desde MMCelt y reinicia la aplicación Codex.
2. En «Enviar a…», prepara la sesión para Codex. Si MMCelt detecta la CLI, puede abrir su consola;
   preparar una sesión no crea por sí solo una tarea en otra aplicación.
3. En tu tarea local de Codex, pide que lea `.mmcelt/sesiones/<sesión>/inicio.md` o el `_AI.md` que
   MMCelt acaba de generar.
4. Comprueba en Codex que el servidor `mmcelt` está disponible antes de pedir una modificación.
5. Continúa hablando en la tarea. El agente devuelve el trabajo mediante las mismas herramientas
   MCP que usa la CLI.

Si esa instalación o ese host no expone MCP local, utiliza el recorrido de copia manual. MMCelt lo
presenta como una limitación del canal, no como un fallo del mapa.

## Qué debe hacer el agente al comenzar

Independientemente de la interfaz de OpenAI, el orden correcto es el mismo:

1. Consultar `mmcelt_workspace_info` para conocer la raíz autorizada y las rutas reales.
2. Leer el mapa con `mmcelt_read_mindmap`.
3. Leer las decisiones humanas con `mmcelt_get_human_feedback` antes de un cambio sensible.
4. Trabajar sobre el código o el documento solicitado.
5. Devolver el avance con `mmcelt_sync_ai_progress` y volver a leer el mapa si la persona ha
   corregido el rumbo.

No debe editar el JSON del mapa a mano. Las herramientas MCP preservan los identificadores, impiden
salir de la carpeta autorizada, rechazan títulos ambiguos y evitan que el agente se atribuya una
aprobación humana.

## Si algo no funciona

**Codex no muestra las herramientas.** Cierra el cliente por completo, vuelve a abrirlo y comprueba
la fila de Codex en `Conectar MMCelt con mis IAs`. Si MMCelt cambió de ubicación, usa **Actualizar la
carpeta** para reparar la ruta del ejecutable.

**La consola se abre, pero el mapa no cambia.** Hablar en la consola no obliga al modelo a llamar a
MCP. Pídele que consulte `mmcelt_workspace_info` y que devuelva el avance con
`mmcelt_sync_ai_progress`.

**MMCelt avisa de cambios humanos sin guardar.** Guarda o descarta conscientemente tus cambios y
repite la lectura. MMCelt no sustituye silenciosamente tu copia en memoria.

**Quieres evitar cualquier API de pago.** Inicia sesión únicamente en los clientes oficiales que
ya incluya tu cuenta. MMCelt no solicita ni almacena claves de API, no elige el modelo y no cambia
la facturación. Antes de una prueba real, comprueba los límites de la cuenta en el propio cliente.
