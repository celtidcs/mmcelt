MMCelt: documentación

MMCelt es una aplicación de escritorio para construir mapas mentales cuyo propósito no es solo
organizar ideas para uno mismo, sino dirigir, acotar y supervisar el trabajo de modelos y agentes de
inteligencia artificial. El icono de la aplicación está en assets/icono/mmcelt-128.png.

El problema que resuelve

Cuando se trabaja con una IA en proyectos de cierta envergadura, la conversación tiende a
desordenarse: el modelo pierde el hilo de la intención original, reabre caminos que ya se habían
descartado e inventa soluciones que contradicen decisiones previas.

MMCelt resuelve este problema transformando el proyecto en un mapa visual estructurado con estados,
prioridades y dependencias. El usuario define qué está completado, qué está en progreso, qué dudas
técnicas bloquean el camino y qué opciones se han descartado formalmente. A partir de ese mapa,
MMCelt exporta un documento Markdown jerárquico que la IA comprende de una sola vez, o se comunica
directamente con agentes autónomos mediante su servidor MCP local.

El flujo es bidireccional: la persona diseña y supervisa; la IA analiza, propone y actualiza el mapa
con sus avances. MMCelt exporta el Markdown hacia la IA (Claude, ChatGPT, Gemini u otra) y recibe de
vuelta su respuesta importando JSON, OPML o archivos de FreeMind. Además, un agente de IA autónomo
que trabaje en consola o por MCP puede leer y sincronizar el mapa a través del servidor MCP local.

Tecnologías

La aplicación de escritorio está escrita en Rust 1.95 o posterior con egui y eframe 0.37, una
interfaz de modo inmediato acelerada por GPU con el backend glow. Los mapas se guardan en archivos
.mmcelt, que son JSON estructurado serializado con serde. Para el intercambio con otros programas
usa estándares abiertos no propietarios: OPML y FreeMind o Freeplane (archivos .mm).

El servidor MCP integrado se arranca con mmcelt --mcp-server y habla el protocolo JSON-RPC 2.0 en
la versión de MCP 2024-11-05 sobre la entrada y la salida estándar, con seis herramientas. Los
diálogos nativos de selección de archivos y carpetas usan rfd. La vigilancia de archivos para la
sincronización en tiempo real usa notify 8.0. El aviso de versión nueva usa ureq 3.4 con rustls
para hacer una consulta HTTPS anónima a GitHub al arrancar, que se puede desactivar. Lleva embebidos
subconjuntos de tres fuentes, Noto Sans SC para el chino simplificado y Noto Emoji y Noto Sans Math
para los símbolos, sin dependencias externas.

Privacidad y control de los datos: la aplicación no necesita conexión a internet para funcionar, no
aloja datos en la nube ni recopila telemetría. Su única petición de red es una consulta anónima a
GitHub al arrancar para avisar de versiones nuevas, que se desactiva desde el menú «Ver y Diseño».
Toda la comunicación con agentes de IA se hace en local, mediante la consola del sistema o el
servidor MCP por la entrada y la salida estándar.

Puesta en marcha rápida

Para descargar el código del repositorio público:

git clone https://github.com/celtidcs/mmcelt mmcelt

Para entrar en la carpeta descargada:

cd mmcelt

Para compilar y arrancar la aplicación:

cargo run --release

Hace falta Rust 1.95 o posterior. No hay dependencias externas pesadas: el servidor MCP y todos los
recursos de ayuda e idiomas van integrados en el propio ejecutable.

La guía completa está en el documento guia-de-clonado.md de la carpeta infraestructura. En esa
misma carpeta hay dos guiones que preparan el entorno de forma automática: preparar-entorno.ps1 para
Windows y preparar-entorno.sh para Linux y macOS.

Índice de la documentación

El documento funcionalidades.md es el catálogo completo de características: lienzo, nodos, estados,
prioridades, conexiones cruzadas y formatos. El documento manual-de-uso.md es el manual para el
usuario, con los flujos de trabajo paso a paso, los atajos de teclado y el trabajo con agentes. El
documento arquitectura.md explica el diseño del software, los módulos, la separación de
responsabilidades y las decisiones técnicas. El documento historial-de-cambios.md recoge los cambios
de cada versión publicada, y defectos-conocidos.md los comportamientos anómalos conocidos y su
estado.

La carpeta integraciones reúne las guías de conexión con agentes: Claude Code, Codex CLI y
Antigravity CLI (agy). La carpeta infraestructura contiene vision-general.md, con el mapa de
componentes y cómo se comunican; requisitos.md, con los requisitos de compilación, sistema y hardware;
servicios.md, con los servicios externos, la configuración de los agentes y el protocolo MCP; y
guia-de-clonado.md, con las instrucciones paso a paso para descargar y compilar el proyecto en
cualquier sistema.

Integración con agentes de inteligencia artificial

MMCelt admite dos maneras de trabajar con una IA. En el flujo asistido, de copiar y pegar, se
exporta el esquema en Markdown con Ctrl+E y después se importa la respuesta en JSON o en los
formatos estándar OPML o .mm. En el flujo autónomo, por consola y MCP, se conectan agentes de línea
de órdenes desde el menú «Inteligencia Artificial → Conectar MMCelt con mis IAs...». Los agentes de
terminal admitidos son Claude Code, de Anthropic; Codex CLI, de OpenAI; y Antigravity CLI (agy), de
Google, que funciona con una cuenta normal de Google.

Estado del proyecto

La última versión publicada es la 0.16.0. Cada versión se publica después de pasar en Windows y en
Linux la batería completa de pruebas unitarias e integradas, el linter sin avisos (cargo clippy
--all-targets --all-features -- -D warnings), el formateador sin diferencias (cargo fmt --check), la
generación de la documentación sin avisos (cargo doc --all-features --no-deps) y la auditoría de
dependencias sin vulnerabilidades conocidas (cargo audit). Una de las pruebas solo existe en
Windows.
