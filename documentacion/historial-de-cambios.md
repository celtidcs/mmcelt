# Historial de cambios

Los cambios de cada versión, de la más reciente a la más antigua. Sigue el criterio de
[Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y el
[versionado semántico](https://semver.org/lang/es/).

**Mientras el primer número sea 0**, el programa está en desarrollo inicial: el formato del
archivo y el protocolo MCP pueden cambiar entre versiones. El **1.0.0** llegará cuando ambos
se declaren estables y el programa se haya probado también en macOS.

## [0.13.0] — 2026-10-08

**Menú al soltar un nodo encima de otro.** Arrastrar un nodo y soltarlo con su centro sobre otra
tarjeta ofrece «Hacer hijo», «Hacer hermano» (lo cuelga del mismo padre, justo detrás del de debajo),
«Conectar con enlace», «Mover aquí sin tapar» y «Cancelar». Lo que el árbol no admite aparece
deshabilitado y explica por qué al pasar el ratón: «Hacer hijo» si ya es su hijo, «Hacer hermano» si
el de debajo es la raíz o si ya son hermanos. «Conectar con enlace» no crea una segunda línea entre
padre e hijo ni repite una conexión que ya existe. `Esc`, un clic fuera o «Cancelar» devuelven el
nodo a su sitio, y todo se deshace con `Ctrl+Z`.

**Menú contextual con clic derecho.** Un clic derecho sin arrastrar sobre una tarjeta la selecciona
y abre «Acciones del nodo»: añadir hijo o hermano, conexión cruzada, editar el título, eliminar, y
submenús de estado, prioridad, control humano y rol. Arrastrar con el botón derecho sigue moviendo
la vista.

**El rol, a la vista, y cada icono explicado.** Cada tarjeta muestra abajo a la derecha el icono de
su rol. Al pasar el ratón por el icono de estado, prioridad, control humano, rol o notas aparece qué
significa, con los nombres del inspector («Prioridad: ⚡ Alta»). Todas las tarjetas reservan la fila
inferior de iconos, así que miden lo mismo tengan notas o no. Los títulos de los mapas de ejemplo
pierden el emoji, que parecía un icono.

**`Ctrl+F` busca, `Inicio` centra e `Insert` crea un hijo.** `Ctrl+F` lleva el cursor al buscador con
lo que hubiera escrito ya seleccionado, y sigue valiendo mientras se escribe en otro campo. Centrar
la vista pasa a la tecla `Inicio`, que dentro de un campo de texto lleva el cursor al principio de la
línea sin mover el mapa. `Insert` añade un hijo, como `Tab`. Los nombres de las teclas salen
traducidos en cada idioma.

**Aviso de versión nueva.** Al arrancar, MMCelt pregunta a GitHub por la última versión publicada y,
si es más nueva, lo indica en la barra superior con un enlace a su página. No descarga ni instala
nada. La consulta va en un hilo aparte y solo por HTTPS verificado; si falla, no molesta. Se
desactiva en `🎨 Ver y Diseño`, y desactivada no hay ningún tráfico de red. Limitaciones conocidas en
[defectos conocidos](defectos-conocidos.md).

**El mapa solo responde a lo que se hace sobre él.** Elegir un valor en un desplegable del
inspector cuya lista caía encima de un nodo, o con un nodo oculto detrás del panel, seleccionaba ese
nodo y el valor iba a parar al nuevo. Ahora el mapa solo atiende el ratón dentro de su zona, sin nada
encima, y solo las pulsaciones que empezaron en él.

**Correcciones.** En chino, el texto emergente de los iconos ya no sale con dos signos de dos puntos.
Los textos emergentes ya no salen estrechos, con tres letras por línea, después de haber mostrado
uno corto.

**Para quien compila.** La versión mínima de Rust es la **1.95**, la que exigen `egui` y `eframe`
0.36; la documentación decía 1.85, y con ella los scripts de preparación daban el visto bueno y la
compilación fallaba después. `preparar-entorno.ps1` ya se ejecuta en Windows PowerShell 5.1. Los
flujos de integración continua y de publicación piden solo los permisos que necesitan.

## [0.12.0] — 2026-09-25

**Indicadores de prioridad y supervisión humana, homogéneos en todo el nodo.** El lienzo
dibuja siempre el icono de la prioridad (🔽 Baja, 🔷 Media, ⚡ Alta, 🔥 Crítica) y el del
estado de revisión humana (⏳ Pendiente, 🤖 Generado por IA, 🛡️ Aprobado, ⚠️ Requiere
corrección), y los selectores del inspector muestran el mismo icono junto al nombre: lo que
se ve en el nodo coincide con lo que se elige en el menú, en los seis idiomas.

**Buscador por estado y prioridad.** La búsqueda combina texto, estado y prioridad a la vez,
permite ordenar por prioridad y limita a cincuenta resultados después de ordenarlos, para no
descartar primero los nodos críticos.

**Formatos abiertos no propietarios de intercambio: OPML y FreeMind/Freeplane `.mm`.**
Exportación e importación completas. La jerarquía, el título y las notas usan los campos
nativos de cada formato; estado, prioridad, rol, revisión, comentario, etiquetas, ruta,
posición, plegado y conexiones cruzadas viajan como JSON estructurado en las notas, y se
recuperan exactos al reimportar en MMCelt. Un archivo ajeno, sin esos metadatos, recibe
valores por defecto, un aviso explícito en los seis idiomas y se distribuye automáticamente
en el lienzo para no amontonar los nodos en un único punto.

## [0.11.0] — 2026-09-23

**El espacio de trabajo sigue siempre a la carpeta del mapa abierto**, y un mapa suelto en la
raíz de un disco se abre en modo de solo lectura guiado en vez de arriesgar acceso a la
unidad entera. «Guardar como» en otra carpeta ofrece dos salidas explícitas: guardar una
copia sin tocar la sesión activa, o guardar y trasladar el trabajo a la carpeta nueva.

**MMCelt se conecta con tres agentes de consola: Claude Code, Codex CLI y Gemini CLI.** Se
retiran del catálogo los clientes gráficos que no se habían podido verificar de principio a
fin; el servidor MCP integrado sigue siendo el mismo para cualquier cliente compatible.

**Errores localizados de verdad.** Los mensajes de error y sus motivos se generan a partir de
tipos cerrados y se traducen en los seis idiomas, en vez de construirse como texto libre en
castellano.

**Ayuda y documentación revisadas por completo**: se corrigieron rótulos, rutas de menú y
recorridos que ya no coincidían con la interfaz, se tradujeron por completo las guías que se
habían quedado recortadas en algún idioma, y se amplió la ayuda para cubrir funciones que no
se explicaban en ninguna parte.

**Instrucciones de los agentes de IA en un único núcleo compartido**, para que las tres
integraciones (Claude, Codex, Gemini) partan de las mismas reglas comunes con sus
particularidades propias por encima.

## [0.10.0] — 2026-09-01

**«Enviar a…» muestra el prompt completo antes de mandarlo nada.** La vista previa separa lo
de solo lectura (el contrato de MMCelt y el contexto del mapa) de lo editable en el momento
(las reglas comunes del proyecto y el encargo de la sesión), con la precedencia escrita
dentro del propio documento.

**MMCelt abre la consola oficial del agente** —Claude Code, Codex CLI o Gemini CLI— con el
mapa guardado, el documento exportado y la vigilancia activada, en ese orden. Cada envío deja
un expediente reproducible en `.mmcelt/sesiones/`, con el prompt exacto que se aprobó.

**El texto ya no pasa por el portapapeles ni por la línea de órdenes.** El prompt viaja en el
expediente; la consola solo recibe una referencia a su ubicación.

## [0.9.0] — 2026-08-27 a [0.9.9] — 2026-09-01

**Deshacer y rehacer** (`Ctrl+Z` / `Ctrl+Y`), con cincuenta pasos de historial. **Búsqueda de
nodos** por título o etiqueta, sin tocar el texto de las notas. **Detección del idioma del
sistema** en el primer arranque, con respaldo a castellano cuando el programa no lo habla.
**Identidad portable por carpeta**, para que MMCelt reconozca un proyecto aunque se mueva de
sitio. **Intervalo de autoguardado configurable**, en vez de fijo en el código.

## [0.8.0] — 2026-08-26

Cambio del motor gráfico y separación de la lógica de negocio en casos de uso propios, de
cara a la integración con agentes de IA que llegaría en las versiones siguientes.

## [0.7.0] — 2026-08-26

Primera versión con historial de cambios. El lienzo, el modelo de nodos, el guardado en
`.mmcelt` y la exportación del mapa a Markdown para una IA ya existían en esta versión.
