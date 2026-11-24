Visión general de la infraestructura

Este documento explica qué componentes forman MMCelt, dónde se ejecuta cada uno y cómo se comunican.

Mapa de componentes

Todo lo que pertenece a MMCelt se ejecuta en el equipo del usuario. Allí conviven tres piezas. La
primera es la aplicación nativa, el ejecutable mmcelt.exe en Windows o mmcelt en Linux, escrita en
Rust con egui. La segunda es ese mismo ejecutable arrancado con la opción --mcp-server, que funciona
como servidor MCP por la entrada y la salida estándar. La tercera son los archivos .mmcelt en el
disco local, que leen y escriben tanto la aplicación como el servidor MCP.

En el mismo equipo trabaja el agente de IA del usuario: Claude Code, Codex CLI, Antigravity CLI (agy)
o cualquier otro cliente MCP, que habla con el servidor MCP.

Fuera del equipo, y fuera de MMCelt, están las plataformas de IA en la nube, como ChatGPT, Claude o
Gemini. Su uso es opcional: se trabaja con ellas copiando y pegando en el navegador, y MMCelt no
habla nunca con ellas.

Qué se ejecuta dónde

La aplicación de escritorio es un proceso nativo local que el usuario arranca con un doble clic o con
cargo run, y guarda su trabajo en archivos .mmcelt.

El servidor MCP es un proceso hijo local que lanza el agente de IA por la entrada y la salida
estándar, y trabaja sobre esos mismos archivos.

La consola oficial del agente es un proceso local separado que solo se abre tras confirmar
expresamente la acción «Enviar a...», y usa su propia sesión autenticada.

Un cliente que solo habla MCP funciona de forma independiente de MMCelt y lee desde su propia
interfaz el expediente que MMCelt le ha preparado.

Los modelos de IA se ejecutan en la nube y quedan fuera del alcance del proyecto.

No hay nada en la nube que pertenezca a MMCelt: ni servidor, ni base de datos, ni cuenta de usuario,
ni telemetría. Lo único externo que consulta es la página pública de versiones de GitHub, al arrancar
y solo si el usuario no lo ha desactivado.

Comunicación entre componentes

La aplicación y el servidor MCP se comunican a través de los archivos .mmcelt del disco: el servidor
escribe y la aplicación lee. Con la función «Enviar a...» y el módulo vigilante.rs, basado en notify
8.0, la aplicación vigila el archivo abierto: en cuanto el agente de IA escribe una actualización en
disco mediante la herramienta mmcelt_sync_ai_progress, la aplicación detecta el cambio y recarga el
lienzo al instante. Desde que se confirma el envío, el mapa queda bloqueado en la aplicación para que
el usuario y el agente no lo cambien a la vez; el agente marca su última entrega con work_finished, la
aplicación lo lee en el recibo .mmcelt/devolucion.json, carga el mapa final y levanta el bloqueo, y
el usuario puede quitarlo antes con «Recuperar el control». Si el usuario tiene cambios sin guardar
en la ventana, la recarga se detiene para proteger su trabajo y se le muestra un aviso.

El agente de IA y el servidor MCP hablan JSON-RPC 2.0 por la entrada y la salida estándar, según el
Model Context Protocol. El agente lanza el servidor como proceso hijo y le habla por las tuberías. No
hay puertos de red ni sockets.

En cuanto a las plataformas de IA, MMCelt puede abrir las consolas oficiales claude, codex y agy
con una referencia al expediente local; no proporciona credenciales ni usa ninguna API. Para los
clientes que solo hablan MCP prepara los mismos archivos sin abrir consola. El retorno estructurado
llega siempre por el servidor MCP local.

Superficie de red

MMCelt hace una sola petición saliente, que se puede desactivar, y nada más. La única dependencia
HTTP de Cargo.toml es ureq, compilada solo con rustls, para comprobar si hay una versión nueva. En el
código hay una única petición: un GET por HTTPS a releases/latest de GitHub al arrancar, si la opción
está activa; los detalles están en el apartado de comprobación de versión nueva del documento
servicios.md de esta misma carpeta. No abre ningún puerto. No hay telemetría: la petición no lleva
datos del usuario ni del mapa. En cuanto a procesos externos, solo ejecuta la consola oficial que
elija el usuario, tras su confirmación, con los argumentos separados y sin el prompt completo.

Es una decisión de diseño con consecuencias directas: no hay credenciales que filtrar ni datos que
sacar del equipo. La única petición es anónima, solo lee un dato público, acepta solo HTTPS con
certificado verificado y trata la respuesta como no fiable.

La consola del agente puede conectarse por su cuenta con su proveedor usando la sesión del usuario,
pero esa conexión no la abre ni la controla MMCelt. Si algún día se añade una integración directa
con una API, este análisis deja de ser válido y hay que rehacerlo por completo.

Variables de entorno

MMCelt usa dos variables de entorno propias, MMCELT_WORKSPACE y MMCELT_DATOS, y respeta otras del sistema que se explican después.

MMCELT_WORKSPACE afecta al servidor MCP y es obligatoria, sin valor por omisión. Delimita la carpeta
dentro de la cual el servidor puede leer y escribir. Si no está fijada, el servidor rechaza toda
operación con archivos por seguridad. Toda ruta que envíe el agente se resuelve, neutralizando los
.. y los enlaces simbólicos, y se comprueba que quede dentro de esa raíz; cualquier intento de salir
se rechaza. Hay que fijarla de forma explícita, o dejar que la configure la ventana de conexión de
la aplicación. Una configuración típica de cliente MCP, en JSON, tiene esta forma:

{"mcpServers": {"mmcelt": {"command": "C:/ruta/a/mmcelt.exe", "args": ["--mcp-server"], "env": {"MMCELT_WORKSPACE": "C:/ruta/a/tus/mapas"}}}}

MMCELT_DATOS afecta a la aplicación y reubica la carpeta donde se guardan las copias de recuperación
del autoguardado. Por defecto no hace falta tocarla: la aplicación usa la carpeta de datos que
corresponde a cada sistema, que en Windows es %APPDATA%\MMCelt. Tiene dos usos concretos: una
instalación portable, para llevar MMCelt en una memoria USB con sus datos al lado en lugar de
dejarlos en el equipo donde se ejecute, y las pruebas, para aislar el archivo de recuperación y que
no interfiera con el del usuario. Si la carpeta no se puede determinar ni crear, el autoguardado
queda desactivado y la aplicación sigue funcionando con normalidad, solo que sin esa red de
seguridad.

CLAUDE_CONFIG_DIR y CODEX_HOME son las variables con las que Claude Code y Codex CLI eligen la
carpeta de una cuenta. MMCelt las lee para encontrar esa cuenta además de las de la carpeta personal,
y, al abrir la consola de una cuenta, fija la variable que la elige.

HTTPS_PROXY, HTTP_PROXY, ALL_PROXY y NO_PROXY, en mayúsculas o minúsculas, se respetan en la consulta
de versión nueva a GitHub; en Windows, si no están, se usa el proxy del sistema.

Archivos que genera el sistema

Los archivos .mmcelt se guardan donde elija el usuario; los crea la aplicación o el servidor MCP y
son el trabajo del usuario, así que no deben borrarse a la ligera.

Los archivos terminados en _AI.md aparecen junto al .mmcelt; los crea el servidor MCP o la
exportación y se pueden borrar, porque se regeneran.

Los archivos .bak aparecen junto al original; los crea el servidor MCP antes de sobrescribir y se
pueden borrar, porque son copias de respaldo.

El archivo mmcelt-errores.log aparece junto al ejecutable, solo si hay errores, y se puede borrar.

El archivo sesion.autoguardado.json está en %APPDATA%\MMCelt\recuperacion\; lo crea la aplicación
cada dos minutos si hay cambios, con un intervalo configurable. Se puede borrar, pero se pierde el
trabajo sin guardar de la última sesión.

El archivo preferencias.json está en %APPDATA%\MMCelt\; lo crea la aplicación al cambiar la escala,
el tema o el idioma. Si se borra, se vuelve a los valores por defecto.

La carpeta target, en la raíz del repositorio, la crea cargo al compilar. Se puede borrar y se
regenera en la siguiente compilación.

Compilación y distribución

Se compila directamente con cargo. Para compilar en modo de publicación, lo que deja el binario en
target/release/mmcelt.exe en Windows o target/release/mmcelt en Linux:

cargo build --release

El perfil de publicación aplica optimización máxima (opt-level = 3), optimización en tiempo de
enlazado (LTO), una sola unidad de compilación, eliminación de símbolos (strip = true) y panic =
"abort". El ejecutable ocupa 12,58 MiB en Windows y 16,28 MiB en Linux. La diferencia entre ambos es
habitual: el ejecutable de Linux conserva más metadatos de enlazado aunque se le apliquen las mismas
opciones. Una parte del tamaño corresponde a los subconjuntos embebidos de tres fuentes: Noto Sans SC para el
chino simplificado, y Noto Emoji y Noto Sans Math para los símbolos.

El binario es totalmente autocontenido: incluye la tipografía embebida y no necesita instalador, ni
entorno de ejecución, ni bibliotecas adicionales en Windows.

Las versiones publicadas, con ejecutables para Windows y Linux, están en la página de publicaciones
(Releases) del repositorio público https://github.com/celtidcs/mmcelt.

Copias de seguridad

El proyecto no gestiona copias de seguridad remotas de los mapas del usuario, más allá de los .bak
automáticos con marca de tiempo que crea el servidor MCP antes de sobrescribir. Como son archivos
JSON normales, la estrategia habitual funciona: guardarlos en una carpeta sincronizada o en un
repositorio git.
