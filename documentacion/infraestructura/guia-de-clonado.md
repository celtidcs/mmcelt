Guía de clonado: reproducir MMCelt desde cero

Esta guía explica cómo dejar el proyecto funcionando en un equipo nuevo, partiendo de que no hay nada
instalado. No hace falta conocer el proyecto para seguirla. Lleva unos 15 minutos, de los cuales unos
5 son de compilación.

Si prefieres automatizarlo, en esta misma carpeta hay dos guiones que hacen todo lo que describe la
guía: preparar-entorno.ps1 para Windows y preparar-entorno.sh para Linux y macOS. Aun así, conviene
leer el paso de verificación para saber qué comprobar al terminar.

Paso 1: instalar Git

Para comprobar si ya tienes Git:

git --version

Si no aparece una versión, instálalo. En Windows puedes descargarlo de https://git-scm.com/download/win
o usar la orden winget install Git.Git. En macOS sirve xcode-select --install o brew install git. En
Debian y Ubuntu, sudo apt install git. En Fedora, sudo dnf install git.

Paso 2: instalar Rust

Para comprobar qué versión de Rust tienes:

rustc --version

Necesitas la 1.95 o posterior, porque lo exigen egui y eframe 0.37. Si no la tienes, en Windows
descarga e instala rustup-init.exe desde https://win.rustup.rs/ y acepta la instalación por defecto,
que es la opción 1.

En Linux y macOS, para instalar rustup:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Y para cargar su entorno en la terminal actual:

source "$HOME/.cargo/env"

Si ya tenías Rust pero con una versión antigua, para actualizarlo:

rustup update stable

En Windows, Rust necesita además el enlazador de Microsoft. El instalador de rustup lo detecta y
ofrece instalarlo: acepta. Si lo omitiste, la compilación fallará con un error sobre link.exe. Se
resuelve instalando las Build Tools de Visual Studio, desde
https://visualstudio.microsoft.com/visual-cpp-build-tools/, con la carga de trabajo «Desarrollo para
el escritorio con C++».

En Linux hacen falta algunas bibliotecas del sistema. La biblioteca libgtk-3-dev es necesaria para
los diálogos nativos de archivo. En Debian y Ubuntu, para instalarlas todas:

sudo apt install build-essential pkg-config libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev

En Fedora:

sudo dnf install gcc-c++ pkgconf-pkg-config gtk3-devel libxkbcommon-devel

No hace falta instalar nada más. El servidor MCP que conecta MMCelt con los agentes de IA va dentro
del propio ejecutable, así que no necesitas Python, Node ni ningún otro entorno de ejecución.

Paso 3: obtener el código

Para descargar el repositorio público:

git clone https://github.com/celtidcs/mmcelt MMCelt

Y para entrar en la carpeta:

cd MMCelt

Paso 4: compilar

Para compilar en modo de publicación:

cargo build --release

La primera compilación descarga y construye las dependencias, y tarda entre 1 y 4 minutos según la
máquina; las siguientes son cuestión de segundos. Al terminar debe aparecer una línea que empieza por
Finished y menciona el perfil release. El binario queda en target/release/mmcelt.exe en Windows o en
target/release/mmcelt en Linux y macOS, y ocupa unos 12,58 MiB en Windows y 16,28 MiB en Linux, con
subconjuntos embebidos de tres fuentes, Noto Sans SC, Noto Emoji y Noto Sans Math.

Esto vale salvo que hayas movido la carpeta de compilación, y en algunos casos hay que moverla. Si el
proyecto vive dentro de una carpeta sincronizada con la nube, como las de Google Drive, OneDrive o
Dropbox, el enlazador de Windows falla con el error LNK1201 porque el sincronizador toca los archivos
mientras se escriben. La solución es fijar target-dir en el archivo .cargo/config.toml, fuera de esa
carpeta. Entonces el binario no está en target, sino donde apunte target-dir. Para saber dónde está
de verdad, sin adivinar, esta orden muestra la carpeta de compilación que usa cargo:

cargo metadata --format-version 1 --no-deps

En su salida, el campo target_directory indica la carpeta. Más sencillo aún: abre MMCelt y mira en
el menú «Ayuda → Acerca de MMCelt (versión y compilación)», que muestra la ruta del ejecutable que se está ejecutando.

Si la compilación falla

Si aparece el error linker 'link.exe' not found, faltan las Build Tools de C++; consulta el paso 2.

Si aparece failed to run custom build command for glutin, faltan bibliotecas del sistema en Linux;
consulta también el paso 2.

Si el mensaje dice que hace falta rustc 1.95 o posterior, Rust está desactualizado y se resuelve con
rustup update stable.

Si aparece LNK1104 diciendo que no se puede abrir el archivo, el ejecutable está en marcha o lo ha
bloqueado el antivirus. Cierra la aplicación. En Windows, para cerrar con PowerShell todas las copias
que sigan abiertas:

Get-Process -Name mmcelt* | Stop-Process -Force

Si aparece LNK1201, error al escribir en el archivo de programa, estás compilando dentro de una
carpeta sincronizada con la nube. Lo explica el apartado siguiente.

Si el proyecto está en Google Drive, OneDrive o Dropbox

Estos servicios copian los archivos mientras se escriben, y la compilación genera miles en pocos
segundos: el enlazador se encuentra con archivos bloqueados a medias. Falla unas veces sí y otras
no, lo que despista mucho.

El código fuente puede quedarse donde está; lo que hay que sacar de la carpeta sincronizada es la
carpeta de compilación. En Linux o macOS, para indicar a cargo una carpeta de compilación fuera de la
nube durante la sesión actual:

export CARGO_TARGET_DIR="/una/ruta/fuera/de/la/nube/target-mmcelt"

Y después compila como siempre:

cargo build --release

Para no tener que acordarte cada vez, y para que valga también al compilar desde el editor, crea el
archivo .cargo/config.toml en la raíz del proyecto con una sección build que contenga la clave
target-dir. Su contenido sería este, con la ruta que elijas:

[build]
target-dir = "C:/ruta/fuera/de/la/nube/target-mmcelt"

Añade ese archivo al .gitignore, porque la ruta solo existe en tu equipo.

Paso 5: verificar

Ejecuta las comprobaciones siguientes; todas deben pasar.

Para comprobar que todo compila sin ningún aviso:

cargo build --all-targets

Para comprobar que el linter no tiene nada que objetar:

cargo clippy --all-targets --all-features -- -D warnings

Para ejecutar las pruebas automatizadas:

cargo test --all-features

Para comprobar que el formato del código es el correcto:

cargo fmt --check

Para auditar las dependencias en busca de vulnerabilidades conocidas:

cargo audit

El resultado esperado es que cargo test termine con test result: ok y todas las pruebas en verde. La
batería incluye las pruebas del servidor MCP integrado, las del contraste de color y las de la
interfaz. Las pruebas del módulo audit_checks protegen contra defectos ya corregidos; si alguna
falla, su documentación explica qué defecto ha reaparecido. No lo ignores.

Para arrancar la aplicación:

cargo run --release

Debe abrirse una ventana de 1280 por 800 píxeles con un mapa mental de ejemplo en el idioma del
sistema.

Paso 6: conectar los agentes de IA (opcional)

Este paso solo hace falta si vas a usar MMCelt con Claude Code, Codex CLI o Antigravity CLI (agy).

La forma recomendada es hacerlo desde la aplicación, en el menú «Inteligencia Artificial → Conectar
MMCelt con mis IAs...». La ventana detecta los agentes instalados, permite elegir la carpeta de trabajo
(la variable MMCELT_WORKSPACE) y registra el servidor en cada agente con un solo clic. Hace copia de
seguridad antes de tocar nada y puede repetirse sin duplicar entradas. Después hay que reiniciar los
agentes, porque ninguno relee su configuración en caliente.

Si prefieres hacerlo a mano, crea primero una carpeta para tus mapas, por ejemplo
Documentos/MapasMentales. Después añade el servidor a la configuración de tu cliente MCP. En JSON,
la entrada tiene esta forma, sustituyendo las dos RUTA_ABSOLUTA por rutas reales:

{"mcpServers": {"mmcelt": {"command": "RUTA_ABSOLUTA/MMCelt/target/release/mmcelt.exe", "args": ["--mcp-server"], "env": {"MMCELT_WORKSPACE": "RUTA_ABSOLUTA/Documentos/MapasMentales"}}}}

Luego reinicia el cliente y comprueba que el agente ve las seis herramientas cuyo nombre empieza por
mmcelt_.

Ten en cuenta que la ruta del ejecutable no es siempre target/release, y copiar el ejemplo tal cual
es la forma más fácil de que el cliente no arranque. Si has movido la carpeta de compilación, cosa
obligada dentro de una carpeta sincronizada, el binario está en otro sitio y el cliente responde con
un error del tipo «The system cannot find the path specified». Lo más seguro es no escribir la ruta a
mano y usar la ventana de conexión del programa, que escribe la configuración por ti con la ruta del
ejecutable que está en marcha. La configuración manual es para cuando esa ventana no cubra tu
cliente.

Fija siempre MMCELT_WORKSPACE de forma explícita. Delimita dónde puede escribir el agente; sin ella
el servidor se niega a operar con archivos, y no usa en silencio la carpeta desde la que arrancó.
Las rutas de la configuración van con barras normales (/) incluso en Windows.

Para probar la función «Enviar a...» sin confundir MCP y consola, ten en cuenta lo siguiente. Con
Claude Code, Codex CLI o Antigravity CLI (agy), comprueba que MMCelt encuentra el ejecutable del agente,
en la carpeta de su instalador oficial, en .local/bin de la carpeta personal o en el PATH, o elígelo
a mano con «Cambiar ruta...», y que la cuenta ya está autenticada en la propia herramienta. Con un cliente que solo habla MCP, el
botón será «Preparar para MCP»: se crean el archivo _AI.md y el archivo inicio.md dentro de
.mmcelt/sesiones/, pero no se abre ninguna consola. MMCelt no necesita ni acepta claves de API para
este recorrido. El archivo .mmcelt/configuracion.json puede no existir en proyectos antiguos; abrir
la vista previa no lo crea, y confirmar el envío sí inicializa la identidad portable del proyecto.

Paso 7: comprobar el conjunto

Para hacer una prueba de extremo a extremo, hablándole al servidor MCP igual que haría un agente,
esta orden le pide la lista de herramientas:

echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/mmcelt --mcp-server

Debe responder con las seis herramientas cuyo nombre empieza por mmcelt_.

Para comprobar qué compilación tienes delante:

./target/release/mmcelt --version

Resumen

Con Git y Rust ya instalados, todo el proceso se reduce a descargar el código, entrar en su carpeta,
compilar, probar y arrancar:

git clone https://github.com/celtidcs/mmcelt MMCelt

cd MMCelt

cargo build --release

cargo test

cargo run --release

Desinstalar

No hay instalador ni registro que limpiar. Basta con borrar la carpeta del repositorio y quitar la
entrada mmcelt de la configuración de tu cliente MCP, si la añadiste. Tus archivos .mmcelt son
independientes y se conservan donde los hayas guardado.
