# Gemini CLI, Antigravity y Gemini web: tres recorridos distintos

> **Qué conecta MMCelt hoy.** Desde la versión 0.11.6 el programa se conecta con tres agentes de
> consola: **Claude Code**, **Codex CLI** y **Gemini CLI**. Lo que esta guía cuenta de
> Antigravity se conserva como referencia de cómo funciona cada plataforma: **ya no se conectan desde
> MMCelt** (el motivo está en el [README](../../README.md#por-qué-ya-no-están-antigravity-cursor-windsurf-y-claude-desktop)).

MMCelt puede trabajar con los modelos de Google por varios clientes, pero no los abre ni los
controla a todos de la misma manera. **Gemini CLI** es una consola interactiva; **Antigravity** es
un entorno gráfico; **Gemini web** es una conversación en el navegador. Compartir la familia de
modelos no hace que compartan conversación, permisos ni herramientas.

| Cliente | Cómo recibe el trabajo | ¿Usa el MCP local? | Dónde escribes después |
|---|---|---|---|
| Gemini CLI | `Enviar a…` abre la consola con el expediente | Sí, después de conectar y reiniciar | En la consola abierta por MMCelt |
| Antigravity | MMCelt prepara el expediente; tú abres el proyecto | Sí, después de conectar y reiniciar | En la interfaz de Antigravity |
| Gemini web o un Gem | Adjuntas o pegas el `_AI.md` | No en un chat web ordinario | En el navegador |

## Preparación común para los clientes locales

1. Abre el mapa y guárdalo dentro de la carpeta del proyecto.
2. Entra en `🤖 Inteligencia Artificial → Conectar MMCelt con mis IAs`.
3. Comprueba que la carpeta autorizada sea la raíz del proyecto.
4. Conecta Gemini CLI o Antigravity, según el cliente que vayas a probar.
5. Cierra por completo ese cliente y vuelve a abrirlo. La configuración MCP se carga al arrancar.

MMCelt registra su propio ejecutable con `--mcp-server` y limita el acceso mediante
`MMCELT_WORKSPACE`. No pide claves de API ni elige el modelo: utiliza la sesión ya autenticada en
el cliente oficial.

## Gemini CLI: MMCelt abre la consola

No hace falta abrirla antes. Elige `Enviar a… → Gemini CLI`, revisa el prompt y pulsa **Iniciar en
consola**. MMCelt guarda el mapa, genera el `_AI.md`, escribe
`.mmcelt/sesiones/<sesión>/inicio.md`, abre Gemini CLI en la carpeta del proyecto y activa la
vigilancia. A partir de ahí sigues escribiendo en esa terminal.

Pide al agente que empiece por `mmcelt_workspace_info`, lea el mapa y consulte las correcciones
humanas antes de modificarlo. Para devolver avances debe usar `mmcelt_sync_ai_progress`.

## Antigravity: tú abres la aplicación

Antigravity no es Gemini CLI y MMCelt no le abre una consola. En `Enviar a… → Antigravity`, el
botón dice **Preparar para MCP**. Al confirmarlo, MMCelt guarda, exporta, crea el expediente y
activa la vigilancia, pero deja la sesión en estado `Preparado` porque todavía no ha arrancado el
otro programa.

Abre Antigravity, entra en el proyecto local y dile qué `inicio.md` debe leer. Comprueba allí que
el servidor `mmcelt` y sus herramientas estén disponibles. La conversación sigue en Antigravity;
los cambios regresan al mapa por MCP.

## Gemini web y Gems: copia manual

Guarda el mapa, expórtalo con `Ctrl+E` y adjunta el `_AI.md` a la conversación. Para traer una
respuesta, pide el JSON compatible con MMCelt y usa `Importar desde IA`. Subir un archivo al chat
no equivale a conceder acceso al servidor MCP local.

## Si no aparece el mapa actualizado

- Si no ves las herramientas, reinicia el cliente y revisa la carpeta registrada en MMCelt.
- Si la consola o Antigravity ven MCP pero no escriben el mapa, pide expresamente
  `mmcelt_sync_ai_progress`.
- Si MMCelt conserva cambios humanos sin guardar, no recargará por encima de ellos. Guarda o
  descarta esos cambios de forma consciente.

La prueba de Gemini CLI no sustituye la de Antigravity: son clientes diferentes y pueden arrancar
con permisos o herramientas distintos.
