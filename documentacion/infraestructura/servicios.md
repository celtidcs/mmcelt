# Servicios externos e integraciones

Qué se conecta con MMCelt, cómo se configura y qué credenciales hacen falta.

---

## Resumen: MMCelt no consume ninguna API

Conviene decirlo antes que nada, porque es contraintuitivo en una aplicación centrada en
la inteligencia artificial:

> **MMCelt no se conecta a ninguna plataforma de IA.** No tiene claves de API, no guarda
> credenciales de ningún tipo y su única petición HTTP es la comprobación de versión nueva
> descrita más abajo, que va a GitHub y no a ninguna IA.

Las «integraciones» de este documento son **plantillas y guías** para que el usuario
configure su propia cuenta en cada plataforma. El intercambio de datos ocurre en el
navegador del usuario o en su agente local, nunca dentro de MMCelt.

| Integración | Tipo | ¿Credenciales en MMCelt? |
|---|---|---|
| Servidor MCP | Proceso local | No |
| Claude Desktop / Claude Code | Configuración del cliente | No |
| ChatGPT (GPT personalizado) | Copiar y pegar | No |
| Gemini (Gem) | Copiar y pegar | No |
| GitHub (comprobación de versión nueva) | Una petición HTTPS anónima al arrancar | No |

---

## Comprobación de versión nueva

Al arrancar, MMCelt pregunta a GitHub cuál es la última versión publicada y, si es más nueva que
la que se está ejecutando, lo avisa en la barra superior con un enlace a su página. **No descarga
ni instala nada.**

| Dato | Valor |
|---|---|
| Petición | `GET https://api.github.com/repos/celtidcs/mmcelt/releases/latest`, una por arranque |
| Qué devuelve | «the most recent non-prerelease, non-draft release» ([documentación de GitHub](https://docs.github.com/en/rest/releases/releases#get-the-latest-release)) |
| Qué se usa de la respuesta | Solo `tag_name`, y solo si es `X.Y.Z` (con `v` opcional). El enlace se compone desde la configuración, nunca desde la respuesta |
| Cabeceras | `User-Agent: mmcelt/<versión>` (GitHub la exige) y `Accept: application/vnd.github+json` |
| Autenticación | Ninguna. Sin autenticar, GitHub admite 60 peticiones por hora y dirección IP ([límites](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)) |
| Transporte | Solo HTTPS con certificado verificado (`ureq` + `rustls`); cualquier dirección `http://` se rechaza |
| Tiempo y tamaño | Espera máxima de 10 s y respuesta de 1 MiB como mucho, en un hilo aparte |
| Si falla | No hay aviso ni ventana de error; el fallo queda en `mmcelt-errores.log` |
| Cómo desactivarla | Menú «🎨 Ver y Diseño» → «Comprobar al arrancar si hay una versión nueva». Desactivada, no hay tráfico de red |

Repositorio, raíces de la API y de la web, tiempo de espera y límite de respuesta se pueden
cambiar en `preferencias.json`, en el bloque `ajustes_de_version`. Se validan antes de usarse:
solo `https://` con un nombre de servidor y un repositorio `dueño/nombre`.

**Limitaciones conocidas** (detalle en [`defectos-conocidos.md`](../defectos-conocidos.md)):

- **Proxy (D-2026-10-07-A).** Se respeta el proxy de las variables de entorno `HTTPS_PROXY`,
  `HTTP_PROXY`, `ALL_PROXY` y `NO_PROXY`, pero no el configurado en las opciones de internet de
  Windows, ni PAC/WPAD, ni la autenticación integrada NTLM/Kerberos. Detrás de un proxy así, no
  aparece el aviso. Rodeo: definir `HTTPS_PROXY`.
- **GitHub Enterprise Server (D-2026-10-07-B).** Su API vive en `https://SERVIDOR/api/v3`, con
  ruta, y `url_api` solo admite la raíz de un servidor: esa dirección se rechaza y no se consulta.
  Sí funcionan `api.github.com` y GitHub Enterprise Cloud (`api.SUBDOMINIO.ghe.com`).

---

## Servidor MCP

La única integración con componentes ejecutables. Permite a un agente de IA leer y
escribir mapas directamente.

| Dato | Valor |
|---|---|
| Ubicación | Integrado en el ejecutable (`mmcelt --mcp-server`) |
| Protocolo | Model Context Protocol — JSON-RPC 2.0 sobre stdio |
| Versión MCP implementada | `2024-11-05`; el saludo no confirma versiones desconocidas |
| Versión del servidor | La misma versión de MMCelt indicada en `Cargo.toml` |
| Transporte | Entrada y salida estándar (sin puertos de red) |
| Autenticación | Ninguna: el agente lanza el proceso con los permisos del usuario |
| Dependencias | Ninguna: forma parte del ejecutable |

### Configuración desde la propia aplicación (lo más sencillo)

Menú **`🤖 Inteligencia Artificial → Conectar MMCelt con mis IAs`**.

Detecta los agentes instalados, muestra su estado y los conecta con un botón. **La
detección se rehace cada vez que se abre la ventana**, así que un agente instalado después
aparece sin reiniciar nada.

Es la vía recomendada, sobre todo si la aplicación se le pasa a otra persona: no hace
falta abrir ninguna terminal.

### Qué se registra: el propio ejecutable

Lo que se escribe en la configuración de cada agente es el propio binario de MMCelt:

```
mmcelt.exe --mcp-server
```

El ejecutable **lleva el servidor integrado**, así que la conexión funciona en un equipo
donde no haya ninguna otra herramienta instalada. Es lo que hace viable el uso
portable: si el usuario puede ejecutar MMCelt, puede conectarlo con su agente.

### Configuración integrada desde la aplicación

Menú **`🤖 Inteligencia Artificial → 🔌 Conectar MMCelt con mis IAs`**.

La aplicación incluye un gestor nativo de conexiones que:
- Detecta automáticamente los agentes de IA presentes en el sistema.
- Crea copias de seguridad de las configuraciones de los agentes antes de modificarlas.
- Permite configurar o desvincular cada agente con un solo clic, sin comandos de terminal.
- Permite seleccionar visualmente el espacio de trabajo (`MMCELT_WORKSPACE`).

El estado **conectado** exige que la entrada `mmcelt` apunte a un archivo ejecutable que todavía
exista y que sus argumentos incluyan exactamente `--mcp-server`. Una entrada antigua o incompleta
se muestra como desconectada para que el botón **Conectar** pueda reescribirla correctamente.

Detecta y configura:

| Agente | Ubicación de configuración | Formato |
|---|---|---|
| **Claude Code** | `~/.claude.json` | JSON |
| **Claude Desktop** | `claude_desktop_config.json` (en AppData / Library) | JSON |
| **Google Antigravity** | `~/.gemini/antigravity/mcp_config.json` | JSON |
| **Cursor IDE** | `~/.cursor/mcp.json` | JSON |
| **Windsurf IDE** | `~/.codeium/windsurf/mcp_config.json` | JSON |
| **Codex CLI** | `~/.codex/config.toml` | TOML |

Garantías de la integración:

- **Copia de seguridad con marca de tiempo** de cada archivo antes de tocarlo. Son
  configuraciones de otras herramientas, que pueden contener ajustes que costó trabajo
  dejar bien.
- **Idempotente**: vincular varias veces actualiza la entrada en vez de duplicarla.
- **Conserva lo que ya hubiera**: los demás servidores MCP y el resto de ajustes del
  archivo se mantienen intactos.
- Si un archivo contiene JSON inválido, **lo deja intacto** y avisa, en lugar de
  sobrescribirlo.
- Una entrada cuyo ejecutable desapareció o que no arranca en modo servidor **no se presenta como
  conectada**, aunque su nombre siga escrito en el archivo.

**Después de configurar hay que reiniciar los agentes**: ninguno relee su configuración
en caliente.

Para comprobar que funcionó, pide a tu agente:

> «Crea un mapa mental de prueba con `mmcelt_create_mindmap`.»

### Detalle que conviene saber sobre Antigravity

Guarda su configuración MCP en `~/.gemini/antigravity/mcp_config.json`, **no** en
`~/.antigravity`, que es donde uno la buscaría. Además, el archivo suele existir vacío
(0 bytes) tras la instalación: eso es normal y el instalador lo maneja.

En «📤 Enviar a...» Antigravity es un destino **solo MCP**: MMCelt guarda el mapa, exporta `_AI.md`,
crea `.mmcelt/sesiones/<sesión>/inicio.md` y activa la vigilancia, pero no inventa una consola de
Gemini CLI. La persona abre o referencia `inicio.md` desde Antigravity.

### Consolas oficiales opcionales

MMCelt puede abrir `claude`, `codex` y `gemini` cuando están en `PATH`. Usa la autenticación que
ya tenga cada CLI: no recibe claves, no llama a APIs y no decide modelo ni facturación. En Windows
se respetan las extensiones de `PATHEXT` antes que un archivo sin extensión; en Linux se busca una
terminal entre `x-terminal-emulator`, `gnome-terminal`, `konsole` y `xfce4-terminal`.

La consola y MCP son capacidades independientes. Una CLI sin MCP permite conversar pero no cerrar
el bucle estructurado; un cliente solo MCP permite preparar el expediente y devolver el mapa sin
que MMCelt pueda abrir su interfaz.

### Configuración manual

Si prefieres hacerlo a mano, o usas un cliente no soportado, en el archivo de
configuración de tu cliente MCP:

```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/ruta/absoluta/a/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": {
        "MMCELT_WORKSPACE": "C:/Users/tu-usuario/Documentos/MapasMentales"
      }
    }
  }
}
```

Ubicación del archivo según el cliente:

| Cliente | Ruta | Formato |
|---|---|---|
| Claude Code | Mejor con `claude mcp add` que a mano | — |
| Claude Desktop (Windows) | `%APPDATA%\Claude\claude_desktop_config.json` | JSON |
| Claude Desktop (macOS) | `~/Library/Application Support/Claude/claude_desktop_config.json` | JSON |
| Antigravity | `~/.gemini/antigravity/mcp_config.json` | JSON |
| Cursor | `~/.cursor/mcp.json` o `.cursor/mcp.json` del proyecto | JSON |
| Codex CLI | `~/.codex/config.toml` | **TOML** |

**Codex es el único que no usa JSON.** Su entrada tiene esta forma:

```toml
[mcp_servers.mmcelt]
command = "C:/ruta/a/mmcelt.exe"
args = ["--mcp-server"]
env = { MMCELT_WORKSPACE = "C:/Users/tu-usuario/Documentos/MapasMentales" }
```

Y para **Claude Code**, en lugar de editar archivos:

```bash
claude mcp add mmcelt -s user \
  -e MMCELT_WORKSPACE=/ruta/a/tus/mapas \
  -- /ruta/a/mmcelt --mcp-server
```

> En PowerShell, pasa los argumentos como array (`& claude @argumentos`): el intérprete
> se come el `--` si va suelto en la línea, y sin él el comando falla.

### `MMCELT_WORKSPACE`: por qué conviene fijarla

Delimita la carpeta dentro de la cual el servidor puede leer y escribir. Toda ruta que
envíe el agente se resuelve —neutralizando `..` y los enlaces simbólicos— y se comprueba
que quede dentro.

**Sin ella, el servidor no opera con archivos.** No cae a la carpeta desde la que arrancó
el proceso: falla cerrado y devuelve un error de seguridad explicando cómo repararlo. Un
control que se abre del todo cuando falta su configuración no es un control.

MMCelt la escribe por ti al conectar un agente desde
`🤖 Inteligencia Artificial → 🔌 Conectar MMCelt con mis IAs...`; solo hace falta fijarla a
mano si registras el servidor sin pasar por la aplicación.

Este es el componente de mayor privilegio del sistema: concede a un agente autónomo la
capacidad de escribir en tu disco. Aquí no hace falta un atacante para que haya daño;
basta una ruta equivocada por el modelo.

### Lo que el servidor le cuenta al agente al conectarse

La respuesta a `initialize` incluye el campo `instructions` que reserva el protocolo. Ahí
se le dice al agente qué es MMCelt, **cuál es la carpeta autorizada** y que las rutas de
`file_path` son relativas a ella. Si no hay carpeta definida no se inventa ninguna: se le
explica que el servidor rechazará cualquier operación con archivos y cómo se repara.

Sin ese campo el agente empezaba a ciegas, se inventaba rutas y cada intento chocaba contra
una valla que nadie le había anunciado.

### Herramientas expuestas

| Herramienta | Función | Escribe |
|---|---|---|
| `mmcelt_workspace_info` | Declara la carpeta autorizada y lista los mapas `.mmcelt` que contiene | No |
| `mmcelt_create_mindmap` | Crea un mapa y su Markdown acompañante | Sí |
| `mmcelt_read_mindmap` | Lee jerarquía, notas, dudas y conexiones | No |
| `mmcelt_get_human_feedback` | Obtiene las correcciones exigidas por el usuario | No |
| `mmcelt_sync_ai_progress` | Desarrolla un mapa existente: nodos, rol, etiquetas y conexiones cruzadas | Sí |
| `mmcelt_export_ai_markdown` | Convierte un `.mmcelt` en Markdown | Sí |

`mmcelt_workspace_info` es la que hay que llamar primero: devuelve la carpeta ya resuelta y
las rutas relativas que el agente tiene que reenviar tal cual en `file_path`, de modo que no
necesita adivinar ninguna. Enumera **solo** archivos `.mmcelt` —el espacio suele ser la
carpeta del proyecto, llena de `.json` que son del usuario—, no sigue enlaces simbólicos, no
entra en carpetas ocultas, no abre ningún archivo, baja como mucho seis niveles y corta a
los 500 mapas avisando con `truncated`. Sin carpeta definida falla cerrado como el resto: es
de solo lectura, y por eso mismo es donde más tienta exceptuarla.

### Identidad portable y recibo de devolución

La acción explícita de conectar una carpeta crea `.mmcelt/configuracion.json`. Su `id` es un UUID
no nulo y estable; `ultima_raiz_confirmada` permite detectar que la carpeta fue movida. El servidor
falla cerrado ante un traslado hasta que MMCelt lo confirma mediante otra conexión explícita.

Las herramientas de escritura aceptan el campo opcional `agent_name`. En un proyecto identificado,
`mmcelt_create_mindmap` y `mmcelt_sync_ai_progress` devuelven `delivery_receipt` y publican la misma
estructura en `.mmcelt/devolucion.json`: `id_entrega`, `project_id`, `base_path`, `map_path`,
`modelo` y `operacion`. `map_path` siempre es relativa y termina en `.mmcelt`; identidad, raíz y
pertenencia se validan antes de la escritura atómica. En un espacio antiguo sin configuración, las
herramientas conservan el comportamiento compatible y responden `delivery_receipt: null`.

El archivo es un recibo de trazabilidad que pueden leer la persona y el agente. No actúa como
permiso para recargar el mapa: MMCelt vigila y valida el contenido del `.mmcelt` por separado, ya
que un recibo durable puede quedarse en disco después de otra edición. La carpeta de control
`.mmcelt` está reservada; las rutas recibidas de una herramienta no pueden crear mapas ni
documentos dentro de ella.

### Desarrollar un mapa hecho por una persona

`mmcelt_create_mindmap` se niega a escribir sobre un mapa que contenga trabajo humano, y hace
bien. Pero durante mucho tiempo esa era la única herramienta capaz de expresar rol, etiquetas y
conexiones cruzadas, así que un agente que quisiera **enriquecer** el mapa de alguien no tenía
por dónde: `mmcelt_sync_ai_progress` sí podía escribir en él, pero solo entendía título, padre,
estado, prioridad, notas y ruta de código. Una jerarquía no sabe expresar un bucle, y los bucles
se dibujan con conexiones cruzadas. El resultado era que el agente acababa entregando un archivo
nuevo en vez del mapa del usuario ampliado, o editando el `.mmcelt` por su cuenta. Una valla sin
puerta empuja fuera de la API justo a quien la respeta.

`mmcelt_sync_ai_progress` acepta ahora:

| Campo | Dónde va | Qué hace |
|---|---|---|
| `role` | en cada `node_updates` | `IdeaCentral`, `PilarEstrategico`, `Subtema`, `HipotesisDuda`, `AccionTarea` o `RecursoHerramienta` |
| `tags` | en cada `node_updates` | etiquetas que **se suman** a las que ya tenga el nodo |
| `cross_connections` | junto a `node_updates` | mismo contrato que en `mmcelt_create_mindmap` —`from_title`, `to_title`, `label` y `relation_type`—, más `from_id` y `to_id` |

Las conexiones se resuelven **después** de los nodos, de modo que pueden apuntar a uno creado en
la misma llamada, que es el caso normal: se añade un paso al flujo y se enlaza con lo que ya
había.

Lo que **no** cambia, porque es lo que hace auditable el bucle entero:

- No se borra ni se mueve ningún nodo. `parent_title` sigue valiendo solo para los nodos nuevos.
- Las etiquetas del agente se añaden; las del usuario no se sustituyen ni se duplican.
- Todo nodo nuevo nace `GeneradoPorIA`, con la etiqueta `ia_progress` por delante.
- El agente no puede escribir la aprobación humana, y cualquier cambio suyo sobre un nodo
  aprobado lo devuelve a `PendienteRevision`.
- Un `role` que no esté en la lista se avisa en `unrecognized_values` en vez de degradarse en
  silencio a `Subtema`, que es el hallazgo H6 aplicado al campo nuevo.

Una conexión se rechaza, y se explica en `connections_not_created`, cuando nombra un nodo que no
existe, une un nodo consigo mismo, toca un nodo `Descartado` —enlazar el flujo con un camino
cancelado lo devolvería al recorrido sin cambiarle el estado, que es revivirlo por la puerta de
al lado— o repite una relación que ya está en el mapa. La respuesta añade `connections_added`
con las que sí se crearon: un flujo a medio dibujar no puede contestar «success» a secas.

### Cómo se señala un nodo: identidad, no adivinanza

Es el hallazgo H4, y afecta a los cuatro sitios donde el agente nombra un nodo: el de una
actualización, su padre y los dos extremos de una conexión.

Los nodos se direccionaban **solo** por título, y el título no es único. El mapa de la prueba
real tenía dos hermanos «Sí» y dos «No» —lo normal en cualquier diagrama de decisiones—, así que
una actualización dirigida a «Sí» caía sobre uno de los dos y el agente recibía `success` sin
enterarse. Elegir «el de identificador menor» es estable entre ejecuciones, pero sigue siendo
elegir a ciegas: el UUID es aleatorio y no guarda relación con el nodo que se quería tocar.

El contrato es **aditivo**. Un título único sigue valiendo exactamente igual que antes; lo que se
añade es poder señalar el nodo por su UUID y, sobre todo, la negativa a adivinar:

| Cómo llega el nodo | Qué hace el servidor |
|---|---|
| Solo el identificador (`id`, `parent_id`, `from_id`, `to_id`) | Lo usa. Es la forma inequívoca |
| Identificador **y** título | Tienen que corresponder; si no, rechaza la llamada |
| Solo un título, único en el mapa | Lo usa, como siempre |
| Solo un título que llevan varios nodos | Rechaza la llamada y enumera los candidatos con su UUID |
| Un identificador que no es un UUID, o que no está en el mapa | Rechaza la llamada |
| Solo un título que no está en el mapa | No es un error: crea el nodo, o cuelga de la raíz avisando, según el campo |

Cuando llegan los dos no sobra ninguno: el identificador dice **cuál** es el nodo y el título dice
**cuál creía el agente que era**. Que discrepen significa que su tabla está caducada —alguien
renombró el nodo, o la leyó de otro mapa—, y aplicar el cambio de todas formas lo pondría sobre un
nodo pensado para otro. Un `id` inexistente tampoco se degrada a «título desconocido»: crear un
nodo nuevo escondería el error del agente debajo de algo que nadie ha pedido.

Los cuatro rechazos son **transaccionales**: se propagan antes de escribir, así que el mapa queda
intacto, incluidas las demás actualizaciones y conexiones del mismo lote. Si una llamada trae diez
cambios y uno es ambiguo, guardar los otros nueve dejaría el mapa en un estado que el agente no ha
pedido y no puede deshacer.

La distinción con los avisos de H3 es deliberada y conviene no confundirla: un título **ambiguo**
rechaza la operación, porque hay varios sitios y elegir uno sería inventar; un título **ausente**,
una autoconexión, un extremo descartado o una conexión duplicada siguen siendo avisos no fatales
en `connections_not_created` y `parents_not_found`. No saber dónde va algo no es lo mismo que
tener dos sitios y elegir uno a escondidas.

### Leer y escribir hablan el mismo idioma

Es el hallazgo H5, la otra mitad del mismo problema. `mmcelt_read_mindmap` devolvía las conexiones
con `from` y `to` como UUID y las dos herramientas de escritura las pedían por título, así que el
agente tenía que construirse a mano la tabla de equivalencias entre lo que leía y lo que podía
escribir —justo la tabla que H4 hace imposible de construir sin ambigüedad—.

Cada conexión de la lectura trae ahora las dos cosas: `from` y `to` con los UUID, que **no** se
sustituyen para no romper a quien ya los leía, y `from_title` y `to_title` añadidos al lado. Si un
archivo trae una arista cuyo extremo ya no existe —`validar_estructura` comprueba el árbol, no las
conexiones—, el título sale como `null` y el UUID sale igual: la lectura tiene que seguir siendo
la operación que nunca falla por lo que encuentre dentro del archivo.

### Lo que publica `tools/list`

La lección de H2 aplicada aquí: lo que no sale en el catálogo no existe para un agente. Un
contrato ampliado que no se anuncia deja el bloqueo donde estaba, porque el agente seguirá mandando
solo títulos y seguirá recibiendo rechazos que no sabe cómo evitar.

El esquema publica `id` y `parent_id` en `node_updates`, y `from_id` y `to_id` en
`cross_connections`. El primer objeto usa `anyOf` para exigir `id` o `title`; el segundo exige un
origen y un destino, cada uno mediante su propia alternativa entre UUID y título. Así el cliente
puede validar la regla sin exigir siempre `title`, `from_title` o `to_title`, que dejaría fuera
justo las llamadas capaces de resolver títulos repetidos.

### Restricciones de escritura

| Restricción | Comportamiento |
|---|---|
| Espacio de trabajo | Rechaza toda ruta fuera de `MMCELT_WORKSPACE` |
| Extensiones | Solo `.mmcelt`, `.json` y `.md` |
| Sobrescritura | Conserva siempre una copia `.bak`. No hay ningún argumento para saltársela |
| Argumentos | Devuelve un error explicando qué falta, en lugar de fallar de forma opaca |

### Comprobar que funciona

```bash
cargo test
```

Las pruebas del servidor forman parte de la suite. Para probarlo a mano:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/mmcelt --mcp-server
```

Debe responder con las seis herramientas.

---

## Claude Desktop y Claude Code

Guía detallada: [`../integraciones/claude-mcp.md`](../integraciones/claude-mcp.md)

Usan el servidor MCP descrito arriba. Claude Code dispone además de una habilidad en
`skills/mmcelt-mindmap/`.

**Credenciales**: las de tu cuenta de Anthropic, gestionadas por el cliente. MMCelt no
las ve.

---

## ChatGPT — GPT personalizado

Guía detallada: [`../integraciones/chatgpt.md`](../integraciones/chatgpt.md)

Plantilla de instrucciones: `documentacion/integraciones/plantillas/custom-gpt.txt`

Se crea un GPT personalizado en `chat.openai.com/gpts/editor` y se pegan las instrucciones
de la plantilla. Ese GPT sabe entonces generar mapas en el formato de MMCelt sin
necesidad de pegar el prompt maestro cada vez.

Flujo: exportar el `.md` desde MMCelt → subirlo al GPT → recibir el JSON → importarlo.

**Credenciales**: las de tu cuenta de OpenAI. Requiere ChatGPT Plus para crear GPT
personalizados. MMCelt no las ve.

---

## Gemini — Gem personalizado

Guía detallada: [`../integraciones/gemini.md`](../integraciones/gemini.md)

Plantilla de instrucciones: `documentacion/integraciones/plantillas/gem.txt`

Equivalente al anterior, con los Gems de Gemini. Su ventana de contexto amplia lo hace
adecuado para mapas muy grandes.

**Credenciales**: las de tu cuenta de Google. MMCelt no las ve.

---

## Cualquier otro modelo

No hace falta configurar nada. La ruta `🤖 Inteligencia Artificial` → `📋 Copiar Prompt Maestro para IA...` genera un texto
autoexplicativo que funciona con cualquier modelo capaz de devolver JSON: DeepSeek,
Mistral, Llama, modelos locales.

---

## Gestión de secretos

**El proyecto no maneja ningún secreto**, así que hoy no hay nada que proteger.

Aun así, el `.gitignore` excluye `.env`, `*.key`, `*.pem`, `*.p12`, `*.pfx`,
`secrets.json` y `credentials.json`. No es porque existan, sino para que el primer archivo
de credenciales que aparezca no acabe en el repositorio por descuido.

Si algún día se añade una integración directa con un modelo:

1. La clave va en una variable de entorno o en un gestor de secretos del sistema, **nunca
   en el código ni en un archivo versionado**.
2. Hay que revisar de nuevo todo el análisis de seguridad del proyecto: la ausencia de red
   es hoy la base de varias garantías, y dejaría de serlo.
3. Documentar la nueva integración en este archivo, en
   [`vision-general.md`](vision-general.md) y en [`../arquitectura.md`](../arquitectura.md).
