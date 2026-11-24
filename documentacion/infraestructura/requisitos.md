Requisitos del sistema

Este documento recoge las versiones exactas necesarias para compilar y ejecutar MMCelt.

Resumen

Rust, con rustc y cargo, es obligatorio en la versión 1.95 o posterior; el proyecto se ha comprobado
con la 1.96.0. Git 2.30 o posterior hace falta para clonar el repositorio.

La versión mínima de Rust la imponen eframe y egui 0.37, y también el uso de let ... else,
estabilizado en Rust 1.65. El proyecto usa la edición 2021 de Rust. No hay archivo
rust-toolchain.toml: se compila con la cadena estable que tengas instalada.

Sistema operativo

Windows 10 y Windows 11 son la plataforma principal, comprobada en local y en integración continua.
Linux, con X11 o Wayland, está comprobado y se prueba automáticamente en integración continua sobre
Ubuntu 24.04. En macOS la arquitectura es compatible y el soporte está previsto.

Las bibliotecas egui, eframe, notify y rfd son multiplataforma, y la integración continua del
proyecto ejecuta en cada envío al repositorio la batería completa de pruebas y clippy en Windows, en
Linux y en macOS. El formato con cargo fmt y la documentación con cargo doc se comprueban en el
trabajo de Windows, y cargo audit, en un trabajo de seguridad que se ejecuta en Linux.

En Linux hacen falta además las bibliotecas de desarrollo del sistema de ventanas. La biblioteca
libgtk-3-dev es necesaria para los diálogos nativos de archivo de rfd. En Debian y Ubuntu, para
instalarlas:

sudo apt install build-essential pkg-config libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev

En Fedora:

sudo dnf install gcc-c++ pkgconf-pkg-config gtk3-devel libxkbcommon-devel

Hardware

Hacen falta al menos 4 GB de memoria; la compilación es lo que más consume, y la aplicación en
marcha consume poco. En disco hacen falta unos 2 GB: el binario de publicación ocupa 12,58 MiB en
Windows y 16,28 MiB en Linux, con subconjuntos embebidos de tres fuentes, Noto Sans
SC, Noto Emoji y Noto Sans Math, y el resto es la carpeta
de compilación target. La tarjeta gráfica debe admitir OpenGL 3.3, porque eframe dibuja por GPU
mediante glow.

En una máquina virtual sin aceleración gráfica puede hacer falta forzar el dibujo por software. En
Linux se consigue fijando la variable de entorno LIBGL_ALWAYS_SOFTWARE=1 antes de arrancar.

Dependencias de Rust

Todas se descargan solas al compilar; no hay que instalar nada a mano.

eframe 0.37.0 es el marco de la aplicación de escritorio y de la ventana nativa, con el backend glow.
egui 0.37.0 es la interfaz gráfica de modo inmediato, con la opción monochrome_emoji_fonts para conservar sus emojis en blanco y negro como respaldo. serde 1.0, con la opción derive, serializa el
modelo, y serde_json 1.0 da el formato JSON de los archivos .mmcelt. rfd 0.18 abre los diálogos
nativos de archivo del sistema. uuid 1.10, con las opciones v4 y serde, genera los identificadores
únicos de los nodos. jiff 1.0, con la opción serde, da las marcas de tiempo en UTC y la hora local. notify 8.0
vigila el sistema de archivos para cerrar el bucle con los agentes. quick-xml 0.41.0, fijada a
esa versión exacta, lee y escribe los formatos de intercambio OPML y FreeMind. sha2 0.10 calcula
huellas SHA-256 con las que se sabe si una configuración o una sesión de agente ha cambiado.
aviso-version-github 0.2.4 es la biblioteca de la comprobación de versión nueva al arrancar, y
deteccion-agentes 0.2.1 la que encuentra las cuentas de los agentes de consola; ninguna viene de
crates.io, sino de su carpeta en bibliotecas/ del propio repositorio. Por último, winresource 0.1 solo se usa al compilar en Windows, para incrustar el icono como recurso del
ejecutable, y no forma parte del programa.

Hay una sola conexión de red, y sirve para una sola cosa. Al arrancar, si el usuario no lo ha
desactivado, MMCelt hace una única petición GET por HTTPS a la API de GitHub para saber cuál es la
última versión publicada; el detalle está en el apartado de comprobación de versión nueva del
documento servicios.md de esta misma carpeta. No descarga nada ni envía datos del usuario. La hace la
biblioteca aviso-version-github, con la misma licencia que MMCelt, GPL-3.0 o posterior, que a su vez
usa ureq 3.4 compilado sin sus opciones por defecto y solo con rustls, la implementación de TLS en
Rust, con las raíces de certificados de webpki-roots, sin gzip ni cookies. Respeta el proxy declarado
en las variables de entorno y, en Windows, el de las opciones de internet; para eso, solo en Windows,
ureq lleva además su opción win-system-proxy, que añade winreg. servicios.md explica sus límites. Las
licencias son MIT o Apache-2.0 para ureq; Apache-2.0, MIT o ISC para rustls; CDLA-Permissive-2.0
para webpki-roots; y MIT para winreg.

Otras dependencias

No hay ninguna obligatoria. El servidor MCP está integrado en el propio ejecutable, así que el
proyecto no necesita Python, Node ni ningún otro entorno de ejecución. Es lo que permite pasarle la
aplicación a otra persona y que le funcione: si puede ejecutar el binario, tiene todo lo necesario,
incluida la conexión con sus agentes de IA.

Para que la función «Enviar a...» abra una consola hace falta tener instalada y autenticada al menos
una de las consolas oficiales: Claude Code (la orden claude), Codex CLI (la orden codex) o
Antigravity CLI (la orden agy). Son componentes opcionales y externos. Los clientes que solo hablan
MCP pueden usar el recorrido sin consola, en el que MMCelt prepara el expediente pero no les abre
ninguna ventana.

El adaptador de consola está comprobado en Windows. El código para las terminales de Linux está
implementado, pero su recorrido real no está comprobado de principio a fin, y macOS no se anuncia
para este flujo.

Herramientas de desarrollo

Son opcionales, pero recomendadas si vas a modificar el código. clippy es el linter, y la
integración continua exige cero avisos; para instalarlo:

rustup component add clippy

rustfmt da formato al código; para instalarlo:

rustup component add rustfmt

cargo-audit busca vulnerabilidades conocidas en las dependencias; para instalarlo:

cargo install cargo-audit --locked

Comprobar el entorno

Para ver la versión de Rust, que debe ser la 1.95 o posterior:

rustc --version

Para ver la versión de cargo:

cargo --version

Para ver la versión de Git:

git --version

Y para comprobar que todo compila y pasa, primero la compilación de todos los objetivos:

cargo build --all-targets

Después el linter, que no debe dar ningún aviso:

cargo clippy --all-targets --all-features -- -D warnings

Después las pruebas:

cargo test

Luego el formato, que no debe mostrar diferencias:

cargo fmt --check

Luego la documentación de la API:

cargo doc --all-features --no-deps

Y por último la auditoría de dependencias, que no debe encontrar vulnerabilidades:

cargo audit

El resultado esperado es que no haya avisos y que todas las pruebas pasen. La batería incluye las
pruebas del servidor MCP integrado, así que no hace falta ninguna herramienta externa.

Dónde se puede compilar

Casi en cualquier sitio, con una excepción que conviene conocer antes de empezar: no compiles dentro
de una carpeta que sincronice un servicio en la nube, como Google Drive, OneDrive, Dropbox o iCloud.

El motivo es que esos programas leen y copian los archivos mientras se están escribiendo. La
compilación genera y reemplaza miles de archivos en pocos segundos, así que el enlazador se encuentra
con archivos bloqueados a media escritura. En Windows el síntoma es un error de enlazado con
link.exe que termina en LNK1201, error al escribir en el archivo de programa. Y no es determinista:
unas veces compila y otras no, lo que hace perder bastante tiempo antes de sospechar del sitio en
lugar del código.

El código fuente sí puede vivir en la carpeta sincronizada; lo que hay que sacar fuera es la carpeta
de compilación, y se hace con la variable de entorno CARGO_TARGET_DIR. En Linux o macOS, para fijarla
en la sesión actual:

export CARGO_TARGET_DIR="/una/ruta/fuera/de/la/nube/target-mmcelt"

En PowerShell:

$env:CARGO_TARGET_DIR = "C:\ruta\fuera\de\la\nube\target-mmcelt"

Después se compila como siempre:

cargo build

Si vas a trabajar así a menudo, conviene fijarlo en un archivo .cargo/config.toml dentro del
proyecto, para que valga también cuando compiles desde el editor y no desde la terminal. Su
contenido sería este, con la ruta que elijas:

[build]
target-dir = "C:/ruta/fuera/de/la/nube/target-mmcelt"

Ese archivo lleva una ruta que solo existe en tu equipo, así que no lo publiques: añádelo al
.gitignore.

La carpeta de compilación ocupa entre 2 y 5 GB. Se puede borrar entera cuando haga falta espacio,
porque se regenera sola con el siguiente cargo build.

Cuidado con las uniones de carpetas

Es cómodo dejar un acceso a la carpeta de compilación en el escritorio. En Windows lo habitual es
crearlo como unión de carpetas (junction), porque los enlaces simbólicos de verdad exigen permisos
de administrador o el modo de desarrollador activado.

Una unión no es un acceso directo. Se comporta como si fuera la carpeta de verdad: si borras archivos
desde dentro del enlace, borras los archivos reales, y si mandas el icono a la papelera desde el
explorador puedes llevarte la carpeta entera por delante.

Para quitar la unión sin tocar la carpeta original, en PowerShell y sin la opción -Recurse:

Remove-Item "C:\ruta\del\enlace"

O desde el símbolo del sistema:

rmdir "C:\ruta\del\enlace"

No es grave si ocurre, porque la carpeta de compilación se rehace sola con el primer cargo build,
pero son varios minutos de espera y un susto evitable.

Cuentas y accesos externos

No hace falta ninguno. MMCelt funciona por completo sin conexión y sin registrarse en ningún
servicio. Con conexión, consulta al arrancar la última versión publicada en GitHub, de forma anónima
y sin cuenta; se puede desactivar en el menú «Ver y Diseño».

Las integraciones con IA usan las cuentas que ya tengas en ChatGPT, Claude o Gemini, pero la
aplicación nunca ve esas credenciales: el intercambio ocurre en tu navegador o en tu agente. El
detalle está en el documento servicios.md de esta misma carpeta.
