MMCelt

MMCelt es una aplicación de escritorio para construir mapas mentales que sirven de plano de trabajo
y de control sobre lo que hace un modelo de inteligencia artificial. Está escrita en Rust, con una
interfaz nativa construida sobre egui y eframe.

Sobre Celtilander

Soy un usuario novel en el mundo de la programación y la IA. He ayudado a crear esta aplicación y
espero que a alguien le pueda servir para algo, sin más. Lo que no funciona no lo escondo: está
escrito en el documento defectos-conocidos.md de la carpeta documentacion. Si encuentras algo que no
esté ahí, cuéntamelo. Saludos a todo el mundo.

El problema que resuelve

En un proyecto largo con una IA, la conversación se convierte en un hilo interminable donde el
modelo pierde el rumbo: vuelve sobre decisiones que ya se habían descartado, inventa una
arquitectura distinta a la acordada y da por bueno lo que nadie ha revisado.

Un mapa mental no se desordena. Lo que hay decidido está a la vista, lo que está pendiente también,
y lo que se descartó sigue ahí escrito con el motivo. MMCelt convierte ese mapa en el documento que
se le pasa al modelo, y en el sitio donde se le corrige cuando se desvía.

Las dos direcciones

El mapa viaja en los dos sentidos, y el trabajo es distinto según quién empiece.

Cuando tú diseñas y la IA construye, dibujas los módulos, las dependencias entre ellos y las rutas
de los archivos que le corresponden a cada uno. Al exportar con Ctrl+E obtienes un documento
Markdown que el modelo interpreta como el plano que debe seguir, con un diagrama Mermaid incluido.

Cuando la IA propone y tú supervisas, el modelo genera el mapa con su propuesta y tú lo abres en
MMCelt. Marcas lo que no vale como Descartado o Requiere corrección, escribes en el nodo qué está mal
y por qué, y devuelves el mapa. Esa corrección viaja dentro del archivo: no es una sugerencia que el
modelo pueda pasar por alto en el siguiente mensaje.

Y el mapa puede volver solo. Con el menú Inteligencia Artificial, opción Enviar a..., preparas un
expediente visible para uno de los agentes detectados. Si el agente dispone de consola, MMCelt la
abre; si solo dispone de MCP, deja el trabajo preparado para que lo recojas desde la interfaz del
agente. Cuando el agente usa las herramientas MCP para devolver avances, ves el mapa cambiar en
pantalla sin exportarlo, pegarlo ni importarlo a mano.

Si tienes cambios sin guardar cuando el modelo devuelve el suyo, tu trabajo no se toca: se te avisa
y decides tú. Un indicador verde en la barra superior recuerda que la vigilancia está activa, junto
al botón Detener seguimiento, que la corta en un clic.

Mientras el agente trabaja, el mapa queda bloqueado para que tú y el agente no cambiéis el mismo
archivo a la vez: puedes verlo, recorrerlo, buscar y exportar, pero no editarlo. El agente avisa en
su última entrega de que ha terminado, y entonces MMCelt carga el mapa final y te devuelve el
control. Si el agente falla o no avisa, el botón Recuperar el control de la barra inferior quita el
bloqueo.

Cómo se ve

En la carpeta assets/capturas hay dos capturas de pantalla. La primera, mmcelt-mapa-e-inspector.png,
muestra el mapa a la izquierda y el Inspector de Nodo a la derecha, donde se define el rol de cada
concepto, su estado, la prioridad, la ruta del archivo de código que le corresponde y las notas que
leerá el modelo. Las líneas de colores con texto son las conexiones cruzadas: dependencias entre
ramas distintas, con el motivo escrito encima.

La segunda, mmcelt-ayuda-detallada.png, muestra la Ayuda Detallada, con 24 temas que explican cada
parte del programa sin salir de él. Está traducida, como el resto de la interfaz, a castellano,
inglés, francés, alemán, ruso y chino simplificado.

Qué hace

Es un solo archivo. El ejecutable ocupa 12,58 MiB (13,20 MB) en Windows y 16,28 MiB en Linux, y no
necesita instalador, ni WebView2, ni Node, ni Python. Se copia a un lápiz de memoria y funciona.

Ofrece un lienzo infinito: paneo con el botón central o derecho del ratón, zoom con la rueda centrado
en el cursor, y disposición automática en árbol horizontal, radial o libre.

Está pensado para código. Cada nodo puede llevar la ruta del archivo que representa, por ejemplo
src/auth/jwt.rs. El escáner recorre una carpeta de código y levanta solo el mapa de su estructura.
Hay plantillas de arquitectura limpia y de aplicación web completa para empezar con algo montado.

Lleva un servidor MCP dentro del ejecutable. Arrancado con la opción --mcp-server, el mismo archivo
habla JSON-RPC por la entrada estándar y deja que un agente lea y escriba mapas directamente en el
disco, dentro de la carpeta que se le indique y solo dentro de ella. No hay que instalar nada
aparte, y eso es lo que permite pasarle el programa a otra persona y que le funcione.

Se conecta con los agentes en un clic. El menú Inteligencia Artificial, opción Conectar MMCelt con
mis IAs, busca los agentes instalados en el equipo, los muestra y los configura, haciendo antes una
copia de seguridad de sus archivos de configuración. Si tienes varias cuentas de Claude Code o de
Codex CLI, cada una aparece en su propia ficha y se conecta por separado, y la consola se abre con la
cuenta que elijas.

Guarda automáticamente. Cada dos minutos, si el mapa ha cambiado, se guarda una copia en los datos
del usuario. Si el programa se cierra de forma inesperada, al volver a abrirlo ofrece recuperar ese
trabajo.

Permite el manejo directo sobre el lienzo. Con clic derecho sobre un nodo aparecen sus acciones:
crear hijo o hermano, conexión, título, eliminar, y los submenús de estado, prioridad, control
humano y rol. Soltar un nodo encima de otro ofrece hacerlo hijo o hermano, conectarlos o dejarlo al
lado; lo que el árbol no admite sale deshabilitado con su motivo. Cada icono de una tarjeta explica
qué significa al pasar el ratón por encima.

Avisa de las versiones nuevas. Al arrancar, una consulta anónima a GitHub avisa si hay una versión
más reciente, con el enlace para descargarla. No descarga nada y se desactiva en el menú Ver y
Diseño.

El tamaño de la interfaz es ajustable. Con Ctrl y la tecla más, Ctrl y la tecla menos, y Ctrl y la
tecla cero se agranda, reduce o restablece toda la interfaz, entre el 50 % y el 300 %, pensando en
monitores 4K y en televisores. El ajuste se recuerda entre sesiones.

Tiene tres temas: oscuro, claro y de alto contraste. Los colores están comprobados con pruebas
automáticas contra los mínimos de contraste de la norma WCAG 2.1, así que ningún texto queda por
debajo del umbral de legibilidad sobre su propio fondo.

Trae la ayuda dentro del programa. Junto a los controles hay explicaciones breves con un botón +info
que abre, en el panel lateral, la guía completa del asunto. Son veinticuatro guías, desde los
primeros pasos hasta el envío a agentes y la integración con cada plataforma.

Habla seis idiomas, sin cadenas a medias: castellano, inglés, francés, alemán, ruso y chino
simplificado, en el menú Ver y Diseño, opción Idioma / Language. No es solo la interfaz: van
traducidos los menús, el inspector, los diálogos, las veinticuatro guías de ayuda (144 archivos) y
el documento que se le entrega a la IA, que es el que más se nota. Incluye además subconjuntos
embebidos de tres fuentes, Noto Sans SC para los caracteres chinos y Noto Emoji y Noto Sans Math
para los símbolos, de modo que todo se ve igual en cualquier sistema operativo, sin dependencias
externas.

Lo que no se traduce es deliberado: los nombres de los campos del archivo .mmcelt, las claves del
protocolo MCP y las de la configuración de los agentes. Son formato, no texto para leer, y
traducirlos rompería los mapas guardados y la conexión con los agentes.

Atajos de teclado

Tabulador o Insert crean un nodo hijo del seleccionado, e Intro crea un nodo hermano al mismo nivel.
Suprimir o Retroceso borran el nodo y todo lo que cuelga de él. Espacio, F2 o un doble clic permiten
editar el título en el lienzo. Arrastrar con el botón central o derecho desplaza la vista, y la rueda
del ratón acerca y aleja el mapa. Ctrl+N crea un mapa nuevo, Ctrl+S guarda, Ctrl+E exporta el Markdown para la IA y Ctrl+F lleva
el cursor al buscador de nodos. Inicio centra la vista en el mapa. Ctrl+Z deshace y Ctrl+Y rehace.
Ctrl con más, menos o cero agranda, reduce o restablece la interfaz.

El documento que recibe la IA

Ctrl+E genera un Markdown pensado para que el modelo razone sobre el proyecto, no para que lo lea
una persona. Lleva, por este orden: las instrucciones de rol y contexto; la visión del creador y los
objetivos, que es donde se explica qué se busca de verdad; un recuento de nodos y estados; la
estructura jerárquica completa con las notas y las rutas de código; las conexiones cruzadas entre
ramas distantes; las dudas y los puntos de decisión pendientes, aparte y destacados; un diagrama
Mermaid; y varios prompts preparados para pedir un plan de acción o una auditoría.

Conectarlo con tu IA

MMCelt se conecta con tres agentes de consola: Claude Code, Codex CLI y Antigravity CLI (agy).

Antigravity, Cursor, Windsurf y Claude Desktop se retiraron en la versión 0.11.6. Funcionaban, pero
trabajar con ellos obligaba a un ir y venir entre ventanas y aplicaciones que no encaja con la
facilidad de uso que se busca: la idea de MMCelt es pulsar un botón y que el agente se ponga a
trabajar sobre el mapa, y con esos entornos hacían falta varios pasos manuales, abrir y cerrar
programas y reiniciarlos para que se enteraran de un cambio de carpeta. Además, el programa solo
ofrece las conexiones que se han comprobado de principio a fin. El servidor MCP sigue en el programa,
porque lo usan los propios agentes de consola, así que recuperar cualquiera de ellos en el futuro es
cuestión de volver a conectarlo y probarlo a fondo.

Si ya tenías conectado uno de esos cuatro entornos, desconéctalo desde una versión anterior antes de
actualizar: al desaparecer su fila desaparece también el botón que limpiaba su configuración, y el
registro quedaría huérfano en el archivo de ese programa.

Antigravity CLI (agy) es la consola oficial de Google y sustituye a Gemini CLI. Funciona con una
cuenta normal de Google, sin necesidad de claves de API ni de cuentas de pago. MMCelt se integra con
Antigravity CLI preparando el envío, redactando el expediente y vigilando la actividad en el mapa.

Las guías por plataforma están en la carpeta documentacion/integraciones. Conviene empezar por
manual-de-control-ia.md, que explica los dos flujos de trabajo de principio a fin. La guía
claude-mcp.md trata Claude Code, que es el agente integrado, y explica además Claude Desktop, que ya
no se conecta desde el programa. La guía chatgpt.md cubre ChatGPT web con copia manual y Codex CLI,
que es el agente integrado. La guía gemini.md cubre los Gems y los contextos largos, con Antigravity
CLI (agy) como agente integrado; lo que cuenta sobre el IDE antiguo de Antigravity queda como
referencia, no como conexión disponible. Estas guías describen también entornos que ya no se conectan
desde MMCelt, porque explican cómo funciona cada plataforma y porque su integración sigue abierta para
el futuro.

Compilar y ejecutar

Para compilar en modo de publicación, lo que deja el ejecutable en target/release/mmcelt.exe en
Windows o target/release/mmcelt en Linux:

cargo build --release

Para ejecutar la batería completa de pruebas:

cargo test

El ejecutable admite dos argumentos. Para ver la versión, la fecha de compilación, el commit y la
rama:

mmcelt --version

Para arrancarlo como servidor MCP en lugar de abrir la ventana:

mmcelt --mcp-server

La opción --version sirve para saber de qué compilación es un ejecutable sin abrirlo, que es lo
práctico cuando hay varias copias por el disco.

La guía completa para preparar un equipo desde cero está en el documento guia-de-clonado.md de la
carpeta documentacion/infraestructura.

Cómo contribuir

Antes de proponer un cambio, comprueba que pasan en Windows y en Linux la batería de pruebas con
cargo test, el linter con cargo clippy en modo -D warnings sin ningún aviso, el formateador con cargo
fmt --check sin diferencias, la documentación con cargo doc y la auditoría de dependencias con cargo
audit sin vulnerabilidades. Las órdenes exactas están en el documento requisitos.md de la carpeta
documentacion/infraestructura.

Versión y publicación

La última versión publicada es la 0.16.1, disponible en el repositorio público
https://github.com/celtidcs/mmcelt con ejecutables para Windows y Linux en su página de publicaciones
(Releases). Cada versión se publica después de pasar las pruebas automáticas en las dos plataformas.

Documentación

Todo lo demás está en la carpeta documentacion: el manual de uso en manual-de-uso.md, la lista
completa de funcionalidades en funcionalidades.md, la arquitectura con el porqué de cada decisión en
arquitectura.md, y la guía para reproducir el proyecto desde cero en otro equipo en
infraestructura/guia-de-clonado.md. El documento README.md de esa carpeta sirve de índice.

Licencia

MMCelt se distribuye bajo la licencia GPL-3.0 o posterior. El texto íntegro está en el archivo
LICENSE de la raíz del repositorio.

En resumen: puedes usar el programa para lo que quieras, estudiar cómo funciona, modificarlo y
repartirlo. La única condición es que, si repartes una versión modificada, publiques también su
código con esta misma licencia.

Se eligió una licencia copyleft y no una permisiva a propósito. Este programa existe para que una
persona mantenga el control sobre lo que hace una máquina; dejar que alguien lo cerrara y lo
vendiera sin devolver nada habría ido justo en contra de eso. Lo que se construya encima vuelve a
todos.

Se entrega sin ninguna garantía, como dice la licencia. Lo que se sabe que falla está en el
documento defectos-conocidos.md de la carpeta documentacion.
