Servicios externos e integraciones

Este documento explica qué se conecta con MMCelt, cómo se configura y qué credenciales hacen falta.

MMCelt no consume ninguna API

Conviene decirlo antes que nada, porque resulta poco intuitivo en una aplicación centrada en la
inteligencia artificial: MMCelt no se conecta a ninguna plataforma de IA. No tiene claves de API, no
guarda credenciales de ningún tipo y su única petición HTTP es la comprobación de versión nueva que
se describe más abajo, que va a GitHub y no a ninguna IA.

Las integraciones de este documento son plantillas y guías para que el usuario configure su propia
cuenta en cada plataforma. El intercambio de datos ocurre en el navegador del usuario o en su agente
local, nunca dentro de MMCelt. Ninguna de ellas guarda credenciales en MMCelt: ni el servidor MCP,
que es un proceso local; ni Claude Code, Codex CLI o Antigravity CLI (agy), que usan la configuración de
su propio cliente; ni ChatGPT con un GPT personalizado o Gemini con un Gem, que funcionan copiando y
pegando; ni la comprobación de versión nueva, que es una petición HTTPS anónima a GitHub al arrancar.

Comprobación de versión nueva

Al arrancar, MMCelt pregunta a GitHub cuál es la última versión publicada y, si es más nueva que la
que se está ejecutando, lo avisa en la barra superior con un enlace a su página. No descarga ni
instala nada.

La petición es un GET a https://api.github.com/repos/celtidcs/mmcelt/releases/latest, una por
arranque. Según la documentación de GitHub
(https://docs.github.com/en/rest/releases/releases#get-the-latest-release), esa dirección devuelve
«the most recent non-prerelease, non-draft release», es decir, la publicación más reciente que no sea
preliminar ni borrador. De la respuesta solo se usa el campo tag_name, y solo si tiene la forma
X.Y.Z, con una v delante opcional. El enlace del aviso se compone desde la configuración, nunca
desde la respuesta.

La petición lleva la cabecera User-Agent, que GitHub exige, con el nombre y la versión de MMCelt
seguidos de los de la biblioteca que hace la consulta, por ejemplo mmcelt/0.16.1
aviso-version-github/0.2.4, y la cabecera Accept con el valor application/vnd.github+json. No lleva autenticación; sin
autenticar, GitHub admite 60 peticiones por hora y dirección IP, según
https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api. El transporte es
solo HTTPS con certificado verificado, mediante ureq y rustls, y cualquier dirección http:// se
rechaza. La espera máxima es de 10 segundos y la respuesta no puede pasar de 1 MiB; todo ocurre en
un hilo aparte. Si falla, no aparece ningún aviso ni ventana de error, y el fallo queda anotado en
mmcelt-errores.log.

Para desactivarla, en el menú «Ver y Diseño» hay que desmarcar la opción «Comprobar al arrancar si
hay una versión nueva». Desactivada, no hay ningún tráfico de red.

El repositorio, las raíces de la API y de la web, el tiempo de espera y el límite de respuesta se
pueden cambiar en preferencias.json, en el bloque ajustes_de_version. Si falta el repositorio, se
usa el de MMCelt. Se validan antes de usarse: solo se aceptan direcciones https:// con un nombre de
servidor, sin credenciales ni puerto y, como mucho, con una ruta limpia, y un repositorio con la
forma dueño/nombre.

La consulta sale por el proxy del sistema. Primero mira las variables de entorno HTTPS_PROXY,
HTTP_PROXY y ALL_PROXY, con las excepciones de NO_PROXY; si no hay ninguna y MMCelt corre en
Windows, usa el proxy activado en las opciones de internet, con sus excepciones. No interpreta la
configuración automática por PAC o WPAD ni hace la autenticación integrada NTLM o Kerberos; detrás de
un proxy así no aparece el aviso, y se puede rodear definiendo HTTPS_PROXY.

Para GitHub Enterprise Server, cuya API vive en una dirección con ruta como https://SERVIDOR/api/v3,
basta con poner esa dirección en url_api y la de la web, https://SERVIDOR, en url_web. También
funcionan api.github.com, que es el valor de fábrica, y GitHub Enterprise Cloud
(api.SUBDOMINIO.ghe.com).

Servidor MCP

Es la única integración con componentes ejecutables, y permite a un agente de IA leer y escribir
mapas directamente.

Está integrado en el propio ejecutable y se arranca con mmcelt --mcp-server. Implementa el Model
Context Protocol, con JSON-RPC 2.0 sobre la entrada y la salida estándar, en la versión de MCP
2024-11-05; el saludo inicial no confirma versiones desconocidas. La versión del servidor es la misma
versión de MMCelt indicada en Cargo.toml. No abre puertos de red. No tiene autenticación: el agente
lanza el proceso con los permisos del usuario. No tiene dependencias, porque forma parte del
ejecutable.

Configuración desde la propia aplicación

Es la vía más sencilla y la recomendada, sobre todo si la aplicación se le pasa a otra persona,
porque no hace falta abrir ninguna terminal. Está en el menú «Inteligencia Artificial → Conectar
MMCelt con mis IAs...».

Lo que se escribe en la configuración de cada agente es el propio binario de MMCelt con la opción
--mcp-server. El ejecutable lleva el servidor integrado, así que la conexión funciona en un equipo
donde no haya ninguna otra herramienta instalada. Es lo que hace viable el uso portable: si el
usuario puede ejecutar MMCelt, puede conectarlo con su agente.

El gestor de conexiones detecta automáticamente los agentes de IA presentes en el sistema y muestra
su estado. La detección se rehace cada vez que se abre la ventana, así que un agente instalado
después aparece sin reiniciar nada. Antes de modificar la configuración de un agente crea una copia
de seguridad, permite conectar o desconectar cada agente con un solo clic, sin órdenes de terminal,
y permite elegir visualmente la carpeta de trabajo (la variable MMCELT_WORKSPACE).

Un agente solo se muestra como conectado si su entrada mmcelt apunta a un archivo ejecutable que
todavía existe y si sus argumentos incluyen exactamente --mcp-server. Una entrada antigua o
incompleta se muestra como desconectada, para que el botón de conectar pueda reescribirla
correctamente.

Los agentes que detecta y configura son tres, y de Claude Code y Codex CLI puede haber varias
cuentas, cada una en su propia ficha. Claude Code guarda su configuración en el archivo JSON
~/.claude.json para la cuenta por defecto, y en el .claude.json de la carpeta de cada otra cuenta,
como la que señala CLAUDE_CONFIG_DIR. Codex CLI la guarda en el archivo TOML config.toml de la
carpeta de la cuenta, que por defecto es ~/.codex y si no la que señala CODEX_HOME. Antigravity CLI
(agy) la guarda en el archivo JSON ~/.gemini/config/mcp_config.json y solo usa su cuenta por defecto.

La integración ofrece estas garantías. Hace una copia de seguridad con marca de tiempo de cada
archivo antes de tocarlo, porque son configuraciones de otras herramientas que pueden contener
ajustes que costó trabajo dejar bien. Conectar varias veces actualiza la entrada en vez de
duplicarla. Conserva lo que ya hubiera: los demás servidores MCP y el resto de ajustes del archivo se
mantienen intactos. Si un archivo contiene JSON no válido, lo deja intacto y avisa, en lugar de
sobrescribirlo. Y una entrada cuyo ejecutable desapareció o que no arranca en modo servidor no se
presenta como conectada, aunque su nombre siga escrito en el archivo.

Después de configurar hay que reiniciar los agentes, porque ninguno relee su configuración en
caliente. Para comprobar que funcionó, pide a tu agente algo como «Crea un mapa mental de prueba con
mmcelt_create_mindmap».

Consolas oficiales opcionales

MMCelt puede abrir las consolas claude, codex y agy cuando encuentra su programa: en la ruta que el
usuario haya elegido a mano en la ventana de conexiones o, si no eligió ninguna, en la carpeta de su
instalador oficial, en .local/bin de la carpeta personal o en el PATH. Usa la autenticación
que ya tenga cada una: no recibe claves, no llama a ninguna API y no decide el modelo ni la
facturación. En Windows se respetan las extensiones de PATHEXT antes que un archivo sin extensión;
en Linux se busca una terminal entre x-terminal-emulator, gnome-terminal, konsole y xfce4-terminal.

La consola y MCP son capacidades independientes. Una consola sin MCP permite conversar, pero no
cerrar el bucle estructurado. Un cliente que solo habla MCP es un destino en el que la acción
«Enviar a...» se convierte en «Preparar para MCP»: MMCelt guarda el mapa, exporta el archivo _AI.md,
crea el archivo inicio.md dentro de .mmcelt/sesiones/ en una subcarpeta propia de cada sesión y
activa la vigilancia, pero no abre ninguna consola. La persona abre o menciona ese inicio.md desde su
cliente, y el mapa vuelve igualmente por el servidor MCP.

Configuración manual

Si prefieres hacerlo a mano, o usas un cliente que la aplicación no configura, añade el servidor al
archivo de configuración de tu cliente MCP. En JSON, la entrada tiene esta forma:

{"mcpServers": {"mmcelt": {"command": "C:/ruta/absoluta/a/mmcelt.exe", "args": ["--mcp-server"], "env": {"MMCELT_WORKSPACE": "C:/ruta/a/tus/mapas"}}}}

Codex CLI es el único que no usa JSON, sino TOML, en el archivo ~/.codex/config.toml. Su entrada
tiene esta forma:

[mcp_servers.mmcelt]
command = "C:/ruta/a/mmcelt.exe"
args = ["--mcp-server"]
env = { MMCELT_WORKSPACE = "C:/ruta/a/tus/mapas" }

En Claude Code es mejor no editar archivos y usar su propia orden. Para registrar el servidor a
nivel de usuario, con la carpeta de trabajo indicada:

claude mcp add mmcelt -s user -e MMCELT_WORKSPACE=/ruta/a/tus/mapas -- /ruta/a/mmcelt --mcp-server

En PowerShell conviene pasar los argumentos como una matriz, con la forma & claude @argumentos,
porque el intérprete se come el doble guion si va suelto en la línea, y sin él la orden falla.

Por qué conviene fijar MMCELT_WORKSPACE

Esta variable delimita la carpeta dentro de la cual el servidor puede leer y escribir. Toda ruta que
envíe el agente se resuelve, neutralizando los .. y los enlaces simbólicos, y se comprueba que quede
dentro.

Sin ella, el servidor no opera con archivos. No recurre a la carpeta desde la que arrancó el proceso:
se niega y devuelve un error de seguridad que explica cómo repararlo. Un control que se abre del todo
cuando falta su configuración no es un control.

MMCelt la escribe por ti al conectar un agente desde «Inteligencia Artificial → Conectar MMCelt con
mis IAs...»; solo hace falta fijarla a mano si registras el servidor sin pasar por la aplicación.

Este es el componente con más privilegios del sistema: concede a un agente autónomo la capacidad de
escribir en tu disco. Aquí no hace falta un atacante para que haya daño; basta con que el modelo se
equivoque de ruta.

Lo que el servidor le cuenta al agente al conectarse

La respuesta a initialize incluye el campo instructions que reserva el protocolo. Ahí se le dice al
agente qué es MMCelt, cuál es la carpeta autorizada y que las rutas de file_path son relativas a
ella. Si no hay carpeta definida no se inventa ninguna: se le explica que el servidor rechazará
cualquier operación con archivos y cómo se repara. Sin ese campo, el agente empezaría a ciegas, se
inventaría rutas y cada intento chocaría contra un límite que nadie le había anunciado.

Herramientas expuestas

El servidor ofrece seis herramientas. mmcelt_workspace_info declara la carpeta autorizada y lista los
mapas .mmcelt que contiene, sin escribir nada. mmcelt_create_mindmap crea un mapa y su Markdown
acompañante. mmcelt_read_mindmap lee la jerarquía, las notas, las dudas y las conexiones, sin
escribir. mmcelt_get_human_feedback obtiene las correcciones que exige el usuario, sin escribir.
mmcelt_sync_ai_progress desarrolla un mapa existente: nodos, rol, etiquetas y conexiones cruzadas.
mmcelt_export_ai_markdown convierte un .mmcelt en Markdown y lo escribe.

mmcelt_workspace_info es la que hay que llamar primero: devuelve la carpeta ya resuelta y las rutas
relativas que el agente tiene que reenviar tal cual en file_path, de modo que no necesita adivinar
ninguna. Enumera solo archivos .mmcelt, porque la carpeta de trabajo suele ser la del proyecto, llena
de archivos .json que son del usuario. No sigue enlaces simbólicos, no entra en carpetas ocultas, no
abre ningún archivo, baja como mucho seis niveles y se detiene a los 500 mapas avisando con el campo
truncated. Sin carpeta definida se niega a operar, como el resto: es de solo lectura, y por eso mismo
es donde más tentaría hacer una excepción.

Identidad portable y recibo de devolución

La acción explícita de conectar una carpeta crea el archivo .mmcelt/configuracion.json. Su campo id
es un UUID no nulo y estable, y el campo ultima_raiz_confirmada permite detectar que la carpeta se ha
movido. Ante un traslado, el servidor se niega a operar hasta que MMCelt lo confirma mediante otra
conexión explícita.

Las herramientas de escritura aceptan el campo opcional agent_name, y mmcelt_create_mindmap y
mmcelt_sync_ai_progress aceptan además work_finished, que el agente pone a true solo en su última
entrega. En un proyecto identificado,
mmcelt_create_mindmap y mmcelt_sync_ai_progress devuelven el campo delivery_receipt y publican la
misma estructura en .mmcelt/devolucion.json, con los campos id_entrega, project_id, base_path,
map_path, modelo, operacion y trabajo_terminado. El campo map_path siempre es relativo y termina en .mmcelt; la
identidad, la raíz y la pertenencia se validan antes de la escritura atómica. En una carpeta antigua
sin configuración, las herramientas conservan el comportamiento compatible y responden con
delivery_receipt a null.

Ese archivo es un recibo de trazabilidad que pueden leer la persona y el agente. No sirve de permiso
para recargar el mapa: MMCelt vigila y valida el contenido del .mmcelt por separado, porque un recibo
puede quedarse en disco después de otra edición. Su único efecto es el campo trabajo_terminado:
mientras el mapa está bloqueado por un envío, MMCelt lee el recibo cada dos segundos y, cuando una
entrega nueva sobre ese mapa lo trae a true, levanta el bloqueo en cuanto carga el mapa final. La carpeta de control .mmcelt está reservada, y las
rutas que reciba una herramienta no pueden crear mapas ni documentos dentro de ella.

Desarrollar un mapa hecho por una persona

mmcelt_create_mindmap se niega a escribir sobre un mapa que contenga trabajo humano, y es lo correcto.
Para que un agente pueda enriquecer el mapa de otra persona sin crear un archivo nuevo ni editar el
.mmcelt por su cuenta, mmcelt_sync_ai_progress acepta, además de título, padre, estado, prioridad,
notas y ruta de código, tres campos más. El campo role, en cada elemento de node_updates, admite
IdeaCentral, PilarEstrategico, Subtema, HipotesisDuda, AccionTarea o RecursoHerramienta. El campo
tags, también en cada elemento de node_updates, añade etiquetas que se suman a las que ya tenga el
nodo. Y el campo cross_connections, junto a node_updates, sigue el mismo contrato que en
mmcelt_create_mindmap, con from_title, to_title, label y relation_type, más from_id y to_id. Hace
falta porque una jerarquía no sabe expresar un bucle, y los bucles se dibujan con conexiones
cruzadas.

Las conexiones se resuelven después de los nodos, de modo que pueden apuntar a uno creado en la misma
llamada, que es el caso normal: se añade un paso al flujo y se enlaza con lo que ya había.

Hay reglas que no cambian, porque son las que hacen auditable el bucle entero. No se borra ni se
mueve ningún nodo, y parent_title sigue valiendo solo para los nodos nuevos. Las etiquetas del agente
se añaden, y las del usuario no se sustituyen ni se duplican. Todo nodo nuevo nace con el estado
GeneradoPorIA y con la etiqueta ia_progress por delante. El agente no puede escribir la aprobación
humana, y cualquier cambio suyo sobre un nodo aprobado lo devuelve a PendienteRevision. Un role que
no esté en la lista se avisa en unrecognized_values, en vez de convertirse en silencio en Subtema.

Una conexión se rechaza, y se explica en connections_not_created, cuando nombra un nodo que no
existe, une un nodo consigo mismo, toca un nodo Descartado o repite una relación que ya está en el
mapa. Lo de los nodos descartados tiene su motivo: enlazar el flujo con un camino cancelado lo
devolvería al recorrido sin cambiarle el estado, y eso sería revivirlo por la puerta de atrás. La
respuesta añade connections_added con las conexiones que sí se crearon, porque un flujo a medio
dibujar no puede contestar un simple «success».

Cómo se señala un nodo

El agente nombra un nodo en cuatro sitios: el nodo de una actualización, su padre y los dos extremos
de una conexión. Un título no es único: en cualquier diagrama de decisiones es normal tener dos
hermanos que se llamen igual, como dos «Sí» o dos «No». Si el servidor eligiera uno de ellos, aunque
fuera de forma estable, estaría eligiendo a ciegas, porque el identificador es aleatorio y no guarda
relación con el nodo que se quería tocar. Por eso los nodos se pueden señalar por su UUID, y el
servidor se niega a adivinar.

El contrato es aditivo: un título único sigue valiendo exactamente igual que antes. Si llega solo el
identificador (id, parent_id, from_id o to_id), el servidor lo usa, y es la forma inequívoca. Si
llegan el identificador y el título, tienen que corresponder; si no, rechaza la llamada. Si llega
solo un título y es único en el mapa, lo usa. Si llega solo un título que llevan varios nodos,
rechaza la llamada y enumera los candidatos con su UUID. Si llega un identificador que no es un UUID,
o que no está en el mapa, rechaza la llamada. Y si llega solo un título que no está en el mapa, no es
un error: crea el nodo, o lo cuelga de la raíz avisando, según el campo.

Cuando llegan los dos no sobra ninguno: el identificador dice cuál es el nodo y el título dice cuál
creía el agente que era. Que no coincidan significa que su información está caducada, porque alguien
renombró el nodo o la leyó de otro mapa, y aplicar el cambio de todas formas lo pondría sobre un nodo
pensado para otro. Un id inexistente tampoco se trata como un título desconocido: crear un nodo nuevo
escondería el error del agente debajo de algo que nadie ha pedido.

Estos rechazos son transaccionales: se producen antes de escribir, así que el mapa queda intacto,
incluidas las demás actualizaciones y conexiones del mismo lote. Si una llamada trae diez cambios y
uno es ambiguo, guardar los otros nueve dejaría el mapa en un estado que el agente no ha pedido y no
puede deshacer.

Conviene no confundir los rechazos con los avisos. Un título ambiguo rechaza la operación, porque hay
varios sitios posibles y elegir uno sería inventar. En cambio, un título ausente, una conexión de un
nodo consigo mismo, un extremo descartado o una conexión duplicada son avisos no fatales, que se
devuelven en connections_not_created y parents_not_found. No saber dónde va algo no es lo mismo que
tener dos sitios y elegir uno a escondidas.

Leer y escribir hablan el mismo idioma

Cada conexión que devuelve mmcelt_read_mindmap trae los dos datos: from y to con los UUID, que se
mantienen para no romper a quien ya los leía, y from_title y to_title al lado. Así el agente no
tiene que construir a mano una tabla de equivalencias entre lo que lee y lo que puede escribir. Si un
archivo trae una conexión cuyo extremo ya no existe, el título sale como null y el UUID sale igual,
porque la lectura tiene que seguir siendo la operación que nunca falla por lo que encuentre dentro
del archivo; la validación estructural comprueba el árbol, no las conexiones.

Lo que publica tools/list

Lo que no aparece en el catálogo de herramientas no existe para un agente: un contrato ampliado que
no se anuncia no sirve, porque el agente seguirá mandando solo títulos y recibiendo rechazos que no
sabe cómo evitar. Por eso el esquema publica id y parent_id en node_updates, y from_id y to_id en
cross_connections. El primer objeto usa anyOf para exigir id o title; el segundo exige un origen y
un destino, cada uno mediante su propia alternativa entre UUID y título. Así el cliente puede validar
la regla sin exigir siempre title, from_title o to_title, lo que dejaría fuera justo las llamadas
capaces de distinguir títulos repetidos.

Restricciones de escritura

El servidor rechaza toda ruta fuera de MMCELT_WORKSPACE. Solo escribe archivos con las extensiones
.mmcelt, .json y .md. Al sobrescribir conserva una copia .bak, y no hay ningún argumento
para saltársela. La única excepción la decide el propio servidor: mmcelt_sync_ai_progress sobre un
mapa que solo ha escrito la IA no deja copia, porque se sincroniza continuamente y llenaría la
carpeta. Si faltan argumentos, devuelve un error que explica qué falta, en lugar de fallar de
forma opaca.

Comprobar que funciona

Las pruebas del servidor forman parte de la batería general, que se ejecuta con:

cargo test

Para probarlo a mano, esta orden le pide al servidor la lista de herramientas:

echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/mmcelt --mcp-server

Debe responder con las seis herramientas.

Claude Code

La guía detallada es el documento claude-mcp.md de la carpeta documentacion/integraciones. Claude
Code usa el servidor MCP descrito arriba y dispone además de una habilidad en la carpeta
skills/mmcelt-mindmap del repositorio. Las credenciales son las de tu cuenta de Anthropic, que
gestiona el propio cliente; MMCelt no las ve.

ChatGPT con un GPT personalizado

La guía detallada es el documento chatgpt.md de la carpeta documentacion/integraciones, y la
plantilla de instrucciones está en documentacion/integraciones/plantillas/custom-gpt.txt.

Se crea un GPT personalizado en chat.openai.com/gpts/editor y se pegan las instrucciones de la
plantilla. Ese GPT sabe entonces generar mapas en el formato de MMCelt sin necesidad de pegar el
prompt maestro cada vez. El flujo consiste en exportar el .md desde MMCelt, subirlo al GPT, recibir
el JSON e importarlo.

Las credenciales son las de tu cuenta de OpenAI, y hace falta ChatGPT Plus para crear GPT
personalizados. MMCelt no las ve.

Gemini con un Gem personalizado

La guía detallada es el documento gemini.md de la carpeta documentacion/integraciones, y la
plantilla de instrucciones está en documentacion/integraciones/plantillas/gem.txt.

Es el equivalente al caso anterior, con los Gems de Gemini. Su ventana de contexto amplia lo hace
adecuado para mapas muy grandes. Las credenciales son las de tu cuenta de Google; MMCelt no las ve.

Cualquier otro modelo

No hace falta configurar nada. La opción «Inteligencia Artificial → Copiar Prompt Maestro para IA...»
genera un texto autoexplicativo que funciona con cualquier modelo capaz de devolver JSON: DeepSeek,
Mistral, Llama o modelos locales.

Gestión de secretos

El proyecto no maneja ningún secreto, así que hoy no hay nada que proteger. Aun así, el .gitignore
excluye .env, *.key, *.pem, *.p12, *.pfx, secrets.json y credentials.json. No es porque existan,
sino para que el primer archivo de credenciales que aparezca no acabe en el repositorio por
descuido.

Si algún día se añade una integración directa con un modelo, la clave irá en una variable de entorno
o en un gestor de secretos del sistema, nunca en el código ni en un archivo versionado. Además, habrá
que revisar de nuevo todo el análisis de seguridad del proyecto, porque la ausencia de red es hoy la
base de varias garantías y dejaría de serlo. Y habrá que documentar la nueva integración en este
documento, en vision-general.md y en el documento arquitectura.md de la carpeta documentacion.
