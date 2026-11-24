Manual de uso de MMCelt

Esta es la guía práctica para usar la aplicación. El catálogo completo de funcionalidades está en el documento funcionalidades.md de la carpeta documentacion, y las instrucciones para instalarla están en el documento guia-de-clonado.md de la carpeta documentacion/infraestructura.


La idea en una frase

MMCelt no es un programa de mapas mentales al que se le haya añadido inteligencia artificial. Es una herramienta de dirección: el mapa existe para que puedas decirle a un modelo qué quieres, qué has decidido y qué no debe volver a proponer.

Si lo usas solo para ordenar ideas, funciona. Pero le sacarás la mitad del partido.


Mi primera vez con MMCelt: guía de principio a fin

Esta parte es para quien nunca ha hecho un mapa mental ni ha trabajado con una IA que edite sus archivos. Va de cero a haber recibido el primer trabajo de un agente y haberle pedido cambios. No hace falta leer nada más antes. Lleva una media hora larga, sin prisa.

Si algo no te sale, no es que lo estés haciendo mal: sáltalo y sigue. Casi todo se puede rellenar después.

Paso 1. Abre el programa y mira lo que hay. Al abrir verás un mapa de ejemplo: una idea central y tres ramas colgando. No es un tutorial que haya que completar ni algo que debas conservar; está ahí para que la pantalla no esté vacía. Prueba a moverte antes de tocar nada. Arrastra el fondo para desplazarte por el lienzo, usa la rueda del ratón para acercar y alejar, y haz clic en un nodo para seleccionarlo; el panel de la derecha, que se llama inspector, pasa a mostrar sus datos. Si el texto se te queda pequeño, Ctrl más + agranda letras, botones y paneles de golpe.

Paso 2. Empieza tu propio mapa. Abre el menú «Archivo» y elige «Nuevo Mapa Mental». Ahora tienes una sola idea central. Haz doble clic sobre ella, o selecciónala y pulsa F2 o Espacio, y escribe de qué va tu proyecto; también puedes cambiar el título desde el inspector. Basta una frase corta, como «Tienda online de cerámica», «Reforma de la casa del pueblo» o «Sistema de reservas». Elige algo que conozcas bien: al final vas a tener que juzgar si lo que la IA responde vale algo, y eso solo se puede hacer sobre un tema que domines.

Paso 3. Levanta la estructura con el teclado. Aquí está el único truco que hay que aprender, y son dos teclas. La tecla Tab crea un hijo, es decir, algo que cuelga de lo seleccionado. La tecla Enter crea un hermano, algo al mismo nivel. El flujo es siempre igual: seleccionas un nodo, pulsas Tab o Enter, escribes el título, pulsas Enter para confirmar y sigues. Se puede construir un mapa entero sin soltar el teclado. Haz tres o cuatro ramas principales con dos o tres hijos cada una. No busques que quede perfecto: un mapa se reordena en cualquier momento. Si las teclas no responden, mira si tienes el cursor dentro de un campo de texto, porque mientras escribes Tab y Enter hacen lo propio de escribir. Haz clic en el fondo del lienzo y vuelve a intentarlo.

Paso 4. Rellena los detalles del nodo y consulta las galletas. Cada nodo del mapa es mucho más que su título. Selecciona uno y mira el inspector de la derecha. Para acompañarte al empezar, MMCelt muestra además galletas de ayuda flotantes con consejos útiles; si las descartas y quieres volver a verlas, puedes reactivarlas desde el menú «Ayuda», en «Galletas de Ayuda para Principiantes».

En el inspector puedes rellenar varios campos, que se presentan aquí de más útil a menos. Las notas sirven para explicar a fondo esa idea, con su contexto, sus requisitos y tus dudas, y son el campo más importante, el que merece la pena rellenar desde el primer día. El estado indica en qué punto está el nodo (Idea, Investigando, En Progreso, Duda / Bloqueo, Completado o Descartado) y se cambia con un clic, así que también conviene usarlo desde el principio. Las etiquetas agrupan nodos por tema cruzando ramas, y cobran sentido cuando el mapa crece. La prioridad marca el nivel de urgencia (Baja, Media, Alta o Crítica) y es útil cuando hay que decidir qué va primero. El rol indica qué papel juega el nodo en la arquitectura, sobre todo si el proyecto es de software. La ruta de archivo enlaza el nodo con un archivo de código real y solo tiene sentido en proyectos de código.

Dedica un momento a las notas de dos o tres nodos. Es el campo que la IA lee con más atención, y la diferencia entre un nodo titulado «Pagos» y ese mismo nodo con un párrafo que explique qué pasarela quieres y por qué es enorme cuando llega la respuesta. Escribe las notas como si se las explicaras a alguien que entra hoy en el proyecto, porque eso es exactamente la IA cada vez que abre tu mapa: alguien nuevo, sin memoria de ayer.

Paso 5. Cruza dos ramas. Llega un momento en que dos ideas de ramas distintas se necesitan, y para eso están las conexiones cruzadas. Selecciona el nodo que necesita algo, pulsa el botón «Conexión» del inspector, que está junto a «Hijo» y «Hermano», y elige el nodo de destino y el tipo de relación. Dependencia (Requiere) es el tipo más habitual. Escribe también por qué existe la conexión: sin el motivo, la IA sabe que hay una relación pero no qué hacer con ella.

Paso 6. Cuéntale a la IA qué pretendes. Este es el paso que más cambia el resultado y el que más gente se salta. Abre el menú «Edición» y elige «Proyecto e instrucciones para la IA». En la pestaña Proyecto hay varios campos. En la visión escribes qué intentas construir y por qué, en prosa, con dos o tres frases. En los objetivos pones qué resultados concretos buscas, hitos y no deseos. En el contexto o público indicas para quién es y dónde tiene que funcionar. Sin esto, la IA ve tu estructura pero no tu intención, y rellena los huecos suponiendo. Un mapa con una estructura impecable y la visión vacía consigue que la IA acierte con el cómo y falle con el para qué.

Paso 7. Guarda. Pulsa Ctrl+S y elige una carpeta para el proyecto. Cada vez que quieras conservar los cambios en ese archivo, vuelve a pulsar Ctrl+S: el archivo solo se actualiza cuando guardas. Mientras trabajas, el programa escribe además, por su cuenta, una copia de recuperación en su carpeta de datos, que te ofrece al abrir si el programa se cerró sin guardar; esa copia no sustituye al archivo, así que guarda antes de cerrar, copiar o compartir el mapa. Si cierras la ventana con cambios sin guardar, o con un mapa que todavía no tiene archivo, como un escaneo o una importación, MMCelt te pregunta antes: Guardar guarda y cierra, No guardar cierra sin guardar y Cancelar te deja seguir trabajando.

Paso 8. Manda el mapa a un agente. En el menú «Inteligencia Artificial», elige «Enviar a...» y después uno de los agentes que MMCelt haya encontrado instalados en tu equipo. Pulsarlo no manda nada todavía. Se abre una vista previa con todo el texto que va a salir, repartido en cuatro bloques. En la vista previa no se edita nada: está para que leas lo que va a salir. El primero es el contrato de MMCelt, con las reglas mínimas, iguales para todos. El segundo son las reglas comunes del proyecto, que recogen tu forma de trabajar, la que se repite siempre. El tercero es el contexto del mapa, que lo genera tu mapa; para cambiarlo, cambia el mapa. El cuarto es el encargo de esta sesión, donde dices qué quieres conseguir ahora. Las reglas y el encargo se escriben en la ventana del paso 6, «Proyecto e instrucciones para la IA», en las pestañas Instrucciones y Plantillas de encargo.

Lee la vista previa entera al menos esta primera vez. Es la única ocasión de ver con tus ojos todo lo que sale de tu ordenador antes de que salga. Para empezar, pide en el encargo algo pequeño y concreto, por ejemplo: revisa la rama de Pagos y propón qué me falta por decidir, sin modificar nada más. Si el encargo está vacío, la vista previa te lo avisa y te ofrece «Configurar encargo...», que abre esa ventana en la pestaña del encargo. Cuando estés listo, el último botón arranca el agente; dirá «Iniciar en consola» o «Preparar para MCP» según el destino.

Paso 9. Mira volver el trabajo. El agente trabaja y devuelve el mapa modificado. No tienes que importar nada a mano: MMCelt vigila el archivo y recoge los cambios. Mientras el agente trabaja, el mapa está bloqueado: puedes verlo, recorrerlo, buscar y exportar, pero no editarlo, y la barra inferior te lo recuerda. Cuando el agente marca su última entrega como terminada, MMCelt carga el mapa final y te devuelve el control. Si el agente falla o no avisa, pulsa «Recuperar el control» en la barra inferior. Después revisa qué ha cambiado.

Paso 10. Corrige lo que no te convenza. Aquí es donde MMCelt deja de ser un editor de mapas y pasa a ser una forma de dirigir a la IA. Selecciona un nodo que no te guste y, en el inspector, marca su estado de revisión. Aprobado por Usuario significa que está bien y que no se toque más. Requiere Corrección significa que hay que cambiarlo, y en ese caso escribes qué hay que cambiar en el campo de corrección. El estado Descartado indica que esa vía no te interesa y que la IA no debe insistir. Lo que escribas ahí manda sobre todo lo demás. Cuando dos instrucciones se contradicen, el orden es este: tus correcciones primero, después el contrato de MMCelt, después las reglas del proyecto y, en último lugar, el encargo de la sesión.

Paso 11. Devuélvele las correcciones. En el menú «Inteligencia Artificial», elige «Enviar Correcciones y Directivas a la IA...». Esto genera un texto que contiene solo tus directivas, no el mapa entero. Es la forma de decir esto no y esto sí sin volver a mandarlo todo ni repetir la conversación. Con eso se cierra el círculo: del mapa a la intención, de ahí al agente, del agente al resultado, del resultado a tus correcciones y de vuelta al agente.

Qué mirar después

Con esto ya sabes usar el programa. Cuando quieras profundizar, el apartado Construir el mapa enseña a hacerlo más rápido; el apartado Flujo de trabajo B, con el servidor MCP, explica cómo dejar que la IA edite tu mapa directamente, sin copiar y pegar; el apartado Conexiones cruzadas las trata a fondo, y el apartado Dirigir a la IA cuando se desvía explica cómo reconducirla.

Y una recomendación que vale para siempre: si algo de lo que devuelve la IA te chirría, hazle caso a esa sensación. Marca el nodo con Requiere Corrección y explica por qué. Ese es justamente el trabajo que ninguna máquina puede hacer por ti, y es la razón de que este programa tenga un botón para ello.


Primer arranque

Al abrir la aplicación encontrarás un mapa de ejemplo con una idea central y tres pilares. No es un tutorial que haya que completar: es un punto de partida que puedes editar directamente.

Lo primero que conviene hacer es rellenar la visión del proyecto, desde el menú «Edición», en «Proyecto e instrucciones para la IA», o con el botón «Editar Visión» del panel lateral. Son tres campos y encabezan el documento que leerá la IA. En la visión del creador escribes qué intentas construir y por qué, en prosa y sin telegrafiar. En los objetivos pones resultados concretos y medibles. En el contexto o público indicas para quién es y en qué entorno se usará.

Es el paso que más diferencia produce en la calidad de lo que devuelve el modelo. Un mapa con estructura impecable y visión vacía hace que la IA acierte con el cómo y falle con el para qué.


Construir el mapa

Flujo básico

Selecciona un nodo con un clic. Pulsa Tab, o Insertar, para crear un hijo, o Enter para crear un hermano. Escribe el título, pulsa Enter y repite. Se puede construir un mapa entero sin soltar el teclado.

Los atajos que tocan el mapa, que son Tab, Enter, Supr, Retroceso, Espacio, F2 y Ctrl+N, solo actúan con el teclado libre, igual que la tecla Inicio, que encuadra la vista. En cuanto se escribe en un campo de texto, esas teclas hacen lo propio de escribir: Retroceso borra una letra y no el nodo, e Inicio lleva el cursor al principio de la línea.

Ctrl+S, Ctrl+E y Ctrl+F sí siguen valiendo mientras se escribe, porque no alteran el mapa: guardan, exportan y llevan el cursor al buscador, respectivamente.

Empezar de cero, cargar un ejemplo o abrir otro mapa

Todo lo que sustituye el mapa abierto, ya sea Ctrl+N, los ejemplos, las plantillas, escanear una carpeta, abrir un archivo o importar de una IA, deja antes una copia de recuperación del que había. Si te das cuenta de que no querías perderlo, cierra MMCelt y vuelve a abrirlo: el programa te ofrecerá recuperarlo.

Esa copia es una red, no un sustituto de guardar. Es única, así que la siguiente sustitución la reemplaza, y en cuanto empieces a trabajar sobre el mapa nuevo pasará a copiarse ese. La costumbre sana sigue siendo pulsar Ctrl+S antes de cambiar de mapa.

Añadir profundidad

El título dice qué es un nodo. Lo que hace útil el mapa para una IA es el resto.

Las notas son el campo más importante. Es donde explicas el detalle técnico, las restricciones y lo que has considerado; escribe frases completas, no palabras sueltas. La ruta de archivo indica, si el nodo corresponde a código real, dónde está ese código (por ejemplo src/auth/jwt.rs), y así la IA sabe dónde debe operar. Las etiquetas sirven para clasificar de forma transversal, con palabras como backend, seguridad o urgente.

El estado y la prioridad dicen dónde está cada cosa y qué es urgente. Ambos campos tienen iconos homogéneos en los selectores del inspector y se pintan directamente sobre el lienzo, antes del título. Las prioridades son Baja, Media, Alta y Crítica; los estados son Idea, Investigando, En Progreso, Duda / Bloqueo, Completado y Descartado.

El control humano sirve para supervisar el trabajo de la IA, con cinco valores: Sin marcar, Generado por IA, Pendiente de Revisión, Aprobado por Usuario y Requiere Corrección. Los nodos que creas tú nacen Sin marcar y no llevan icono de revisión. Si la IA modifica un nodo que no era suyo, pasa a Pendiente de Revisión y su icono, el reloj, se pinta en el color de aviso para que lo veas; si estaba aprobado, la aprobación caduca.

Los nodos que la IA acaba de crear aparecen con un fondo y un borde de otro color. En cuanto seleccionas uno, recupera su aspecto normal y se queda con el icono de Generado por IA. Así, de un vistazo, el color te dice qué nodos son nuevos y el reloj qué nodos ya existentes ha cambiado la IA.

Qué significa cada icono

Si no recuerdas qué indica un icono de una tarjeta, pasa el ratón por encima. El icono que va delante del título es el estado; los de arriba a la derecha son la prioridad y el control humano; abajo a la derecha está el rol del nodo, y el icono del bloc de notas avisa de que el nodo tiene notas. El rol se cambia en el inspector o con el clic derecho, en el submenú Rol.

El clic derecho sobre un nodo

Haz clic derecho sobre cualquier tarjeta: queda seleccionada y aparece el menú «Acciones del nodo», con opciones para añadir un hijo o un hermano, crear una conexión, editar el título y eliminar, además de los submenús de estado, prioridad, control humano y rol. Es lo mismo que ofrecen el menú «Edición» y el inspector, pero sin ir a buscarlo. Para mover la vista, arrastra con el botón derecho como siempre: eso no abre el menú.

Cambiar una rama de sitio con el ratón

Arrastra el nodo y suéltalo encima del que quieres que sea su nuevo padre. Aparece un menú pequeño. Si eliges «Hacer hijo», la rama entera pasa a colgar de él; si eliges «Hacer hermano», cuelga del mismo padre que el nodo de debajo, justo detrás de él. Lo que el árbol no admite sale deshabilitado y explica por qué al pasar el ratón: «Hacer hijo» se deshabilita si ya es su hijo, y «Hacer hermano» si el de debajo es la raíz o si ya son hermanos. Si lo que querías era relacionarlos sin cambiar la jerarquía, elige «Conectar con enlace»; si solo querías dejarlo cerca, «Mover aquí sin tapar», que entre hermanos además lo pone detrás del de debajo en el orden. La opción «Cancelar», o la tecla Esc, lo devuelve a donde estaba, y cualquier opción se deshace con Ctrl+Z.

Buscar y filtrar nodos

En el panel lateral está el buscador de nodos, con el texto de ayuda «Buscar por título o etiqueta».

Con Ctrl+F el cursor salta al buscador desde cualquier sitio, con lo que hubiera escrito ya seleccionado para que puedas buscar otra cosa encima.

La búsqueda por texto localiza términos en los títulos o en las etiquetas. El filtrado por estado y prioridad permite aislar rápidamente, por ejemplo, solo los nodos En Progreso o los de prioridad Crítica. Los resultados se pueden ver en el orden natural del mapa o clasificados por prioridad, de mayor a menor urgencia, lo que da una lista clara de trabajo pendiente.

Marcar las dudas

Es la funcionalidad más infrautilizada y una de las más útiles. Cuando no sepas algo, créalo como nodo con el estado Duda / Bloqueo y el rol Hipótesis / Duda. Por ejemplo, un nodo titulado «¿PostgreSQL o SQLite?», con una nota que explique el compromiso que estás evaluando.

Al exportar, esas dudas se agrupan en una sección propia del documento, con el encabezado PUNTOS DE DECISIÓN, DUDAS Y BLOQUEOS A RESOLVER. La IA las recibe como una lista concreta de cosas que resolver, no como comentarios dispersos.


Flujo de trabajo A: con copiar y pegar

Es el más sencillo. Funciona con cualquier modelo, sin instalar nada.

Del mapa a la IA

Paso 1. Si quieres ver el texto antes de guardarlo, previsualiza el documento desde el menú «Inteligencia Artificial», en «Previsualizar Markdown de IA (.md)...».

Paso 2. Para guardar el archivo .md, pulsa Ctrl+E o ve al menú «Archivo» y elige «Exportar Markdown para IA (.md)». La misma acción está en el menú «Inteligencia Artificial», como «Exportar Archivo .md para IA...».

Paso 3. Elige dónde guardarlo, o cópialo directamente al portapapeles desde la ventana de previsualización.

Paso 4. Pégalo en ChatGPT, Claude o Gemini y añade tu petición, por ejemplo: aquí tienes el mapa mental de mi proyecto; analízalo y propón un plan de ejecución que resuelva las dudas abiertas de la sección de puntos de decisión.

De la IA al mapa

Paso 1. En el menú «Inteligencia Artificial», elige «Copiar Prompt Maestro para IA...».

Paso 2. Pégalo en el modelo junto con lo que quieras convertir: una conversación, un documento o un fragmento de código.

Paso 3. El modelo devuelve un bloque JSON.

Paso 4. En MMCelt, abre el menú «Inteligencia Artificial», elige «Importar desde IA (ChatGPT, Claude, Gemini)...», pega la respuesta y pulsa Convertir.

No hace falta que limpies la respuesta: el importador localiza el bloque JSON aunque venga rodeado de texto conversacional. Si la importación falla, el mensaje explica qué falló en cada intento de interpretación. Lo más habitual es que el modelo haya devuelto prosa en lugar del JSON; en ese caso, pídele que genere únicamente el bloque.

Intercambio mediante formatos abiertos: OPML y FreeMind .mm

Además de trabajar con Markdown y con su JSON nativo, MMCelt permite intercambiar mapas con cualquier herramienta externa de mapas mentales o editor de esquemas.

Para exportar, el menú «Archivo» ofrece «Exportar a OPML (.opml)...», que guarda el mapa como esquema jerárquico XML estándar, y «Exportar a FreeMind (.mm)...», que lo guarda en el formato estándar de FreeMind y Freeplane. En ambos casos, los datos propios de MMCelt (estado, prioridad, rol, supervisión humana, rutas y conexiones cruzadas) viajan codificados en JSON estructurado al pie de las notas.

Para importar, el mismo menú ofrece «Importar desde OPML (.opml)...» e «Importar desde FreeMind (.mm)...». Si el mapa lo exportó antes MMCelt, se reconstruye de forma idéntica, sin perder ningún dato ni ninguna conexión lateral. Si el mapa viene de otra aplicación y no trae metadatos de MMCelt, ocurren tres cosas. Primero, se asignan valores seguros por defecto: estado Idea, prioridad Media, rol Subtema / Módulo y, para la raíz, rol Idea Central. Segundo, MMCelt distribuye las ramas de forma equilibrada de inmediato, para que los nodos no aparezcan amontonados en el origen de coordenadas y el mapa quede listo para leer y editar desde el primer momento. Tercero, se muestra un diálogo informativo que explica qué atributos se han adaptado.


Flujo de trabajo B: con el servidor MCP

Es más potente: el agente lee y escribe el mapa directamente, sin intervención manual. La instalación se explica en el apartado del servidor MCP del documento servicios.md de la carpeta documentacion/infraestructura.

Cómo se trabaja

Paso 1. Construyes el mapa en MMCelt y lo guardas dentro de tu espacio de trabajo.

Paso 2. Le dices a tu agente algo como: lee el mapa mental de proyecto.mmcelt y empieza por los nodos marcados como críticos.

Paso 3. El agente trabaja y va actualizando estados con la herramienta mmcelt_sync_ai_progress. Con esa misma herramienta puede ampliar tu mapa: añadir nodos con su rol y sus etiquetas y trazar conexiones cruzadas entre ramas. No borra ni mueve nada de lo tuyo, y todo lo que añade queda marcado como generado por la IA para que lo revises.

Paso 4. Ves el avance en pantalla sin recargar nada, como se explica en el apartado Enviar el mapa y verlo volver.

Paso 5. Si se desvía, marcas nodos como Requiere Corrección con tu instrucción y le dices que consulte mmcelt_get_human_feedback y corrija el rumbo.

El Markdown exportado incluye una sección final titulada «Cómo devolver tu trabajo a este mapa». En ella el agente recibe el recorrido completo de las seis herramientas, los valores exactos de estados, roles, prioridades y relaciones, y la regla de usar el UUID cuando un título se repite. Así no necesita deducir el contrato ni editar el archivo .mmcelt directamente.

El bucle de corrección es lo que distingue este flujo. No tienes que volver a explicar el proyecto entero: marcas en el mapa lo que está mal y el agente lo recoge.

La carpeta de trabajo

Es la única carpeta dentro de la cual el agente puede leer y escribir, y se elige en el menú «Inteligencia Artificial», en «Conectar MMCelt con mis IAs...».

Mientras no elijas ninguna, MMCelt propone la del mapa que tengas abierto y no una carpeta fija, porque autorizar Documentos/MapasMentales cuando tu proyecto está en otro sitio le daría al agente permiso para trabajar donde no hay nada. Si eliges una a mano, manda esa.

El programa de cada agente

En la misma ventana, cada agente enseña la ruta del programa que MMCelt va a abrir y si lo ha encontrado solo o lo has elegido tú. MMCelt lo busca en la carpeta donde lo deja su instalador oficial, en la carpeta .local/bin de tu carpeta personal y en el PATH. Si lo instalaste en otro sitio, pulsa «📂 Cambiar ruta...» en su ficha y elige el programa; MMCelt lo recordará. Para volver a la búsqueda automática, pulsa «Usar la detección». Si el programa que elegiste se mueve o se borra, la ficha te lo dice y MMCelt vuelve a buscarlo por su cuenta.

Si usas varias cuentas de un mismo agente, por ejemplo dos de Claude Code, cada una sale en su propia ficha con el nombre de su carpeta. MMCelt las encuentra solo, tanto las que están en tu carpeta personal como la que señale CLAUDE_CONFIG_DIR o CODEX_HOME. Conecta las que quieras usar: cada una se conecta por separado. Al enviar un mapa, eliges la ficha de la cuenta con la que quieres trabajar y la consola se abre con ella. Si una cuenta está en otro sitio, pulsa «Añadir cuenta», elige el agente y su carpeta; «Quitar cuenta» la deja de mostrar sin borrar nada. Con Antigravity CLI (agy) solo se usa su cuenta por defecto y no se pueden añadir otras: agy no ofrece ninguna forma documentada de elegir otra carpeta de configuración y siempre lee la suya, así que una cuenta añadida a mano no se usaría ni al conectar ni al enviar el mapa.

El agente no tiene que adivinar nada: al conectarse, el servidor le dice en el saludo cuál es la carpeta autorizada, y con la herramienta mmcelt_workspace_info puede pedir la lista de mapas .mmcelt que hay dentro, con las rutas exactas que después debe reenviar.

Si cambias la carpeta, los agentes ya conectados no se enteran solos, porque la carpeta va escrita dentro del archivo de configuración de cada agente. Cuando dejan de coincidir, la ventana deja de decir que el agente está conectado, te avisa de a qué carpeta apunta cada uno y te ofrece el botón «Actualizar la carpeta», que vuelve a registrarlo con la carpeta que estás viendo y deja una copia de seguridad. Nada se reescribe sin que lo pidas, y después hay que reiniciar el agente.


Enviar el mapa y verlo volver

No hace falta llevar el mapa a mano al modelo ni traer su respuesta, y además puedes ver y corregir el prompt antes de que salga.

Prepararlo sin enviar nada

Abre el menú «Edición» y elige «Proyecto e instrucciones para la IA». La primera pestaña, Proyecto, es la única que necesitas al empezar: contiene la visión, los objetivos, el público y el autor. Guardar esos datos no obliga a configurar ningún agente.

El pie común de las cuatro pestañas permite elegir el idioma del documento para la IA, independiente del idioma de los menús. El cambio se guarda de inmediato para ese proyecto: no hace falta pulsar «Guardar como reglas comunes» y, al cerrar y volver a abrir la ventana, la elección se conserva. El contrato y las plantillas de MMCelt sí cambian de idioma; tu visión, tus reglas y tu encargo se quedan como los escribiste. El botón «Restaurar plantilla recomendada» propone unas reglas prudentes, pero no las guarda hasta que pulses el botón de guardado, que también está en el pie común y es accesible desde cualquier pestaña.

En esa ventana aparecen también los archivos AGENTS.md, CLAUDE.md y GEMINI.md si existen. Una fuente nueva no se incorpora por sorpresa. Después de leerla, el botón «Aceptar esta versión» guarda localmente su contenido y su huella en .mmcelt/configuracion-agente.json. Si cambia un solo byte, queda desmarcada y se muestran la versión aceptada anterior y la actual. Si no existe ninguna, la pestaña muestra la carpeta exacta donde buscará esos tres nombres; puedes crear allí el archivo a mano, porque MMCelt no lo crea ni lo modifica. Nada de esto guarda credenciales ni modifica el archivo original.

La pestaña Plantillas de encargo copia una sola ayuda al encargo editable; no acumula las tres órdenes. La pestaña Vista completa reúne el contrato, las reglas, el contexto y ese único encargo con el mismo compositor que usa la confirmación final. Cerrar esta ventana conserva el encargo preparado mientras el programa siga abierto, y al elegir un agente aparece ya en su borrador final.

Cómo se manda

En el menú «Inteligencia Artificial», elige «Enviar a...» y después uno de los agentes que MMCelt encuentre instalados en tu equipo. Pulsarlo no manda nada todavía: se abre una vista previa con el prompt completo, repartido en cuatro bloques. El contrato de MMCelt es de solo lectura y contiene las reglas mínimas de integridad, las mismas para todos. Las reglas comunes del proyecto son de solo lectura y son tu perfil neutral. El contexto del mapa es de solo lectura, lo genera la exportación para IA y se cambia en el mapa. El encargo de esta sesión también es de solo lectura y dice qué quieres conseguir ahora. Las reglas y el encargo se escriben en la ventana «Proyecto e instrucciones para la IA», descrita en el apartado anterior.

El propio prompt declara qué manda cuando dos instrucciones se contradicen: primero las correcciones humanas del mapa, después el contrato de MMCelt, después las reglas del proyecto y, por último, el encargo de la sesión.

Debajo hay dos botones. «Cancelar» no guarda, no exporta, no vigila y no deja rastro. El otro dice «Iniciar en consola» si existe una consola del agente, o «Preparar para MCP» si el destino no tiene consola oficial. Si el encargo está vacío, ese botón no se puede pulsar y la vista previa ofrece «Configurar encargo...». «Guardar como reglas comunes» no está aquí, sino en la ventana «Proyecto e instrucciones para la IA», y es la única acción que escribe tu perfil en .mmcelt/instrucciones-agente.md.

Si en la raíz del proyecto tienes AGENTS.md, CLAUDE.md o GEMINI.md, la vista previa los enseña por separado, indica si son nuevos, aceptados o modificados, y permite incluirlos o excluirlos en esta sesión. Solo una versión aceptada y sin cambios aparece preseleccionada.

Al pulsar «Iniciar en consola», el programa hace cinco cosas seguidas, y cada una tiene que salir bien antes de pasar a la siguiente. Primero guarda el mapa, para que el modelo lea exactamente lo que tú estás viendo. Después exporta el documento Markdown para la IA, junto al mapa y con el sufijo _AI.md. En tercer lugar escribe el expediente de la sesión en la carpeta .mmcelt/sesiones. En cuarto lugar abre la consola del agente en la carpeta del proyecto. Por último, empieza a vigilar el archivo del mapa. Si la consola no llega a abrirse, MMCelt no dice que el mapa se haya enviado, no empieza a vigilar y deja el expediente marcado como fallido.

Justo antes de iniciar, MMCelt vuelve a leer el mapa, las reglas guardadas y las fuentes nativas. Si algo cambió desde que se preparó la vista, la confirmación caduca: no se crea ningún proceso ni ningún expediente nuevo hasta que revises otra vez el texto. El archivo inicio.md conserva después exactamente los mismos bytes que mostraba la vista completa, no una reconstrucción parecida.

Con un destino que solo tiene MCP, es decir, cuando un agente tiene configurado el servidor MCP pero MMCelt no encuentra su programa de consola, MMCelt guarda, exporta, escribe el expediente e inicia la vigilancia, pero no abre ninguna consola. El expediente queda en estado Preparado y se continúa desde la interfaz del propio agente. La vista previa lo advierte antes de confirmar.

El archivo de identidad .mmcelt/configuracion.json se crea la primera vez que confirmas el proyecto desde MMCelt. En una carpeta creada con una versión anterior, abrir y cancelar la vista previa no escribe nada. No edites ese archivo de identidad: permite reconocer el proyecto aunque cambie de nombre. Si mueves la carpeta, vuelve a pulsar Conectar para confirmar la nueva ubicación.

A partir de ahí trabajas en la ventana del agente, que es donde le das las instrucciones, con MMCelt a la vista al lado. Cuando el modelo cambia el mapa, lo ves cambiar en pantalla.

Qué queda guardado de cada envío

Cada confirmación crea una carpeta dentro de .mmcelt/sesiones, con la fecha, el agente y un identificador. Dentro hay dos archivos. El archivo inicio.md contiene el prompt exacto que aprobaste, letra por letra. El archivo sesion.json contiene los datos de la sesión: formato, identificador, fecha, agente, ruta del mapa, fuentes detectadas, tamaño del prompt y estado del lanzamiento. No se guardan contraseñas, claves, correos ni identificadores de cuenta. Es material tuyo y local.

Agentes de consola admitidos

MMCelt se integra exclusivamente con tres agentes oficiales de consola: Claude Code, Codex CLI y Antigravity CLI (agy). Al confirmar el envío desde «Enviar a...», MMCelt abre una ventana nueva de terminal con la sesión preparada, el expediente redactado y la vigilancia activa.

Antigravity CLI (agy) sustituye a la consola anterior Gemini CLI. Funciona de forma nativa e interactiva con una cuenta personal estándar de Google, sin requerir claves de API de Google AI Studio ni cuentas de pago. MMCelt abre agy en la carpeta del proyecto con la opción -i y conecta con el servidor MCP a través del archivo mcp_config.json.

Los clientes de entorno gráfico y los editores sin consola oficial, como el IDE antiguo de Antigravity, Claude Desktop, Cursor o Windsurf, no forman parte del catálogo. Es una decisión de diseño: el programa ofrece únicamente opciones comprobadas y sencillas de manejar, sin configuraciones engorrosas.

MMCelt no pide ni guarda credenciales. Se apoya en la sesión que ya tengas autenticada en claude, codex o agy. No usa ninguna API de pago propia, no elige modelo y no decide la facturación.

Agentes y recorridos

Con Claude Code, de Anthropic, MMCelt abre la consola al confirmar «Enviar a...» y elegir «Claude Code», y tú continúas escribiendo en esa consola nueva. Con Codex CLI, de OpenAI, ocurre lo mismo al elegir «Codex CLI». Con Antigravity CLI (agy), de Google, también, al elegir «Antigravity CLI (agy)», y continúas en la consola interactiva con tu cuenta de Google. Con un chat web ordinario de cualquier proveedor, eres tú quien lo abre en el navegador y adjunta o pega el archivo _AI.md; ese chat no tiene acceso al servidor MCP local.

No hace falta abrir antes una consola: MMCelt la abre al confirmar el envío. Sí conviene tener autenticado el cliente, porque MMCelt usa su sesión existente.

En OpenAI: ChatGPT web y Codex CLI

No son dos nombres para la misma ventana. En una conversación ordinaria de ChatGPT web debes adjuntar el archivo _AI.md o usar la copia manual: el navegador no lee la configuración MCP local y no puede escribir el mapa de tu disco. Codex CLI sí puede cargar el servidor MCP local y es el destino que MMCelt abre en una consola al confirmar el envío.

En la práctica, mira siempre dos cosas por separado: el icono de consola confirma si MMCelt puede abrir una conversación nueva, y el icono MCP confirma si el agente puede leer y devolver el mapa. La explicación completa, con los tres recorridos, está en el documento chatgpt.md de la carpeta documentacion/integraciones.

El recorrido está comprobado en Windows, donde se abre una ventana nueva y se prefieren los lanzadores declarados en PATHEXT frente a los guiones Unix sin extensión. En Linux, el adaptador usa una lista cerrada de terminales, pero todavía no está comprobado en esta versión. En macOS todavía no se garantiza.

El mapa queda bloqueado mientras el agente trabaja

Desde que confirmas el envío, el mapa no se puede editar: si tú y el agente cambiarais el mismo archivo a la vez, uno de los dos perdería su trabajo. Puedes verlo, recorrerlo, seleccionar nodos, buscar y exportar, y la barra inferior avisa de que un agente está trabajando. El agente marca su última entrega como trabajo terminado; entonces MMCelt carga ese mapa final y levanta el bloqueo. Si el agente falla o no avisa nunca, el botón «Recuperar el control» de la barra inferior quita el bloqueo. Si el envío falla antes de lanzar al agente, el bloqueo se quita solo.

Qué pasa si recuperas el control y tocas el mapa

La regla es una y manda sobre todo lo demás: tu trabajo no se pierde nunca. Si no tienes cambios sin guardar, MMCelt recarga el mapa en el momento; es el caso normal. Si tienes cambios sin guardar, no toca tu mapa: te avisa de que la IA cambió el archivo y decides tú. Si el archivo llega a medio escribir, lo ignora y espera al siguiente aviso, sin mostrar errores. Cuando es MMCelt quien guarda por su cuenta con el autoguardado, reconoce su propia escritura y no se dispara a sí mismo. Antes de cualquier recarga se deja una copia de seguridad en el archivo de recuperación.

Cómo se corta

Mientras la vigilancia está activa, en la barra superior aparece un indicador verde junto al nombre del archivo. Para pararla, abre el menú «Inteligencia Artificial» y elige «Detener seguimiento».

Merece la pena saber dónde está antes de necesitarlo: el día que un modelo se equivoque y siga equivocándose, es el botón que quieres tener localizado. La vigilancia también termina de forma limpia al crear un mapa nuevo con Ctrl+N, al abrir otro archivo desde el menú «Archivo» y al cerrar la aplicación.

La consola del agente es un proceso aparte y se cierra en su propia ventana. Detener la vigilancia no cierra la consola, y cerrar la consola no borra el expediente.

Un consejo para el prompt

El documento que recibe el agente ya le pide que marque su última entrega como trabajo terminado, que es lo que desbloquea el mapa. Si ves que el agente ha acabado y el mapa sigue bloqueado, recuérdale que lo haga, o pulsa «Recuperar el control». No hace falta que estés mirando el mapa mientras trabaja.

Si el agente crea o sincroniza el mapa por MCP, puedes comprobar la entrega en .mmcelt/devolucion.json. Ese archivo contiene el nombre del agente, la ruta relativa del mapa y si el mapa fue generado o modificado. MMCelt lo sustituye con cada entrega nueva; no es el mapa ni una copia de seguridad, sino una constancia que tú o el propio agente podéis inspeccionar. MMCelt detecta los cambios reales del mapa vigilando el archivo y no usa este recibo como permiso automático para recargar, porque un recibo antiguo no demostraría quién escribió el contenido actual. Sí lo lee mientras el mapa está bloqueado, para saber si el agente ha marcado su entrega como trabajo terminado.


Dirigir a la IA cuando se desvía

En los proyectos largos ocurre siempre. Este es el procedimiento.

Marcar lo que está mal

Paso 1. Selecciona el nodo problemático.

Paso 2. En el inspector, cambia el estado de revisión a Requiere Corrección.

Paso 3. Escribe la instrucción concreta en el campo de corrección, y sé específico. Por ejemplo: no uses MongoDB; este proyecto necesita transacciones ACID y consultas relacionales, así que usa PostgreSQL.

Descartar caminos cerrados

Si una opción queda definitivamente fuera, ponle el estado Descartado. En el documento exportado, esos nodos llevan una advertencia explícita para que el modelo no insista. Es preferible a borrarlos: dejas constancia de que se evaluó y se rechazó, y así la IA no lo vuelve a proponer como novedad.

Enviar las correcciones

En el menú «Inteligencia Artificial», la opción «Enviar Correcciones y Directivas a la IA...» genera un prompt que contiene solo las directivas de rumbo, sin repetir el mapa entero. Está pensado para una IA que ya conoce el proyecto.


Partir de código existente

Si ya tienes un proyecto y quieres refactorizarlo con ayuda de una IA, abre el menú «Archivo», elige «Escanear Carpeta de Código...» y selecciona la raíz del repositorio.

MMCelt recorre hasta tres niveles y crea un nodo por cada carpeta y cada archivo de código, con su ruta ya rellenada. Omite las carpetas target, node_modules, dist y build y las carpetas ocultas.

A partir de ahí, anota tus intenciones sobre la estructura real: qué módulo te da problemas, cuál quieres extraer o dónde falta cobertura de pruebas.


Conexiones cruzadas

Sirven para lo que la jerarquía no puede expresar: que dos ramas distintas dependen entre sí.

Paso 1. Selecciona el nodo de origen.

Paso 2. Pulsa «Conexión» en el inspector de la derecha, junto a «Hijo» y «Hermano». La misma acción está en el menú «Edición», como «Crear Conexión Cruzada / Dependencia...».

Paso 3. Elige el destino y el tipo de conexión, y escribe por qué existe.

Hay cinco tipos, y la elección cambia lo que la IA entiende: Dependencia (Requiere), Bloquea a, Alternativa a, Sinergia con e Inspirado por.

Las conexiones de un nodo se ven en el inspector de la derecha, en la sección Conexiones Cruzadas, y desde ahí se borran. Esa sección solo aparece si el nodo tiene alguna.

El motivo importa. Decir solo que hay una dependencia aporta poco; explicar que los costes de alojamiento condicionan el precio del plan gratuito le da a la IA información que no estaba en ninguna otra parte.


Preguntas frecuentes

Veo la aplicación muy pequeña en mi televisor o en una pantalla grande. Pulsa Ctrl más + varias veces: agranda letras, botones y paneles de golpe. También está en el menú «Ver y Diseño», en Tamaño de la interfaz, donde el 200 % está pensado justo para televisores. El ajuste se recuerda, así que solo hay que hacerlo una vez. No lo confundas con la rueda del ratón: esa acerca el mapa, no la interfaz. Si lo que ves pequeño son los menús y los botones, la solución es Ctrl más +.

¿Por qué no se ajusta solo al tamaño de mi pantalla? En parte lo hace: si tienes una pantalla 4K sin escalado del sistema, la primera vez la interfaz se agranda sola. Pero lo que de verdad determina el tamaño adecuado es a qué distancia te sientas, y eso ningún programa puede saberlo: un televisor de 42 pulgadas a tres metros le presenta a Windows los mismos datos que un monitor de 24 pulgadas a medio metro. Por eso se propone un punto de partida y se recuerda lo que elijas.

¿Por qué las líneas del mapa tienen colores distintos? Cada rama principal tiene su color, y toda su descendencia lo hereda. Sirve para seguir un hilo hasta el final aunque el mapa esté enredado. Las conexiones cruzadas van siempre en ámbar, para distinguirlas de la jerarquía. El detalle completo está en el apartado Qué significan los colores del documento funcionalidades.md de la carpeta documentacion.

Aprobé un nodo y ahora aparece como pendiente. ¿Por qué? Porque la IA lo ha cambiado después. La aprobación caduca en cuanto el contenido del nodo deja de ser el que aprobaste: sus notas, su estado, su prioridad o su ruta de archivo. Lo que aprobaste fue un texto, no el nodo para siempre. Si la IA envía los mismos valores que ya había, no cuenta como cambio y la aprobación se mantiene, de modo que una sincronización rutinaria no te obliga a revisar otra vez lo que ya diste por bueno.

Al importar la respuesta de la IA, ¿pierdo el mapa que tengo abierto? Sí. Importar sustituye el mapa, no lo añade al que ya tienes. Antes de hacerlo, MMCelt guarda una copia de seguridad del mapa anterior: si te arrepientes, cierra el programa y vuelve a abrirlo, y al arrancar te ofrecerá recuperarlo. Aun así, lo sensato es guardar con Ctrl+S antes de pegar nada. Tampoco viaja la presentación: las posiciones que hayas colocado a mano y las ramas que tuvieras plegadas se pierden, porque la IA devuelve la estructura y el contenido, no la disposición. El mapa se recoloca solo.

¿Puedo cambiar el aspecto de la aplicación? Sí. En el menú «Ver y Diseño», en Tema Visual, hay tres temas: oscuro, que es el de partida, en azul pizarra; claro, de papel cálido, con el fondo en marfil en lugar de blanco; y alto contraste, negro con bordes blancos, para problemas de visión o pantallas con reflejos. El tema cambia la ventana entera, con el lienzo, los menús, los paneles y los diálogos, y se recuerda al reiniciar.

¿Puedo trabajar sin conexión a internet? Sí, siempre. Lo único que hace MMCelt en la red es preguntar a GitHub, al arrancar, si hay una versión nueva; sin conexión, simplemente no avisa. Esa comprobación se desactiva en el menú «Ver y Diseño», con la opción «Comprobar al arrancar si hay una versión nueva». Para hablar con el modelo sí necesitas conexión, pero eso ocurre en tu navegador o en tu agente, no en MMCelt.

¿Cómo sé si hay una versión nueva? Al arrancar, si hay una más nueva publicada en GitHub, la barra superior muestra el aviso «Nueva versión disponible:» con su número. Al pulsarlo se abre su página en el navegador; el programa no descarga ni instala nada.

¿Dónde se guardan mis mapas? Donde tú elijas. No hay almacenamiento en la nube ni base de datos: son archivos .mmcelt normales que puedes copiar, versionar o sincronizar como cualquier otro.

Me dice que el archivo está dañado y no lo abre. Ha detectado que el mapa describe una estructura imposible, como un ciclo, una raíz que no existe o referencias a nodos ausentes. El mensaje indica el nodo concreto. Se rechaza a propósito, porque cargarlo colgaría la aplicación. Si el archivo lo generó una IA, pídele que lo regenere; si hay un archivo .bak al lado, prueba con él.

La IA devuelve un JSON que no se importa. Suele ser porque añadió comentarios dentro del JSON o dejó una coma final, cosas que el formato no admite. Pídele que lo genere de nuevo como JSON estrictamente válido y sin comentarios.

¿Puedo tener nodos con el mismo título? Sí, sin salvedades. En un diagrama de decisiones es lo normal: dos nodos Sí y dos nodos No colgando de preguntas distintas. Una IA que pida un cambio nombrando solo el título no acaba en el nodo equivocado: si ese título lo llevan varios nodos, MMCelt rechaza la petición entera sin escribir nada y le devuelve los candidatos con su identificador, para que repita la llamada señalando el que quería. Cada nodo y cada conexión que la IA lee llevan ya ese identificador.

¿Cuántos nodos aguanta? El límite es de 100 000 nodos y 512 niveles de profundidad, muy por encima de lo que resulta legible. A partir de unos pocos cientos de nodos conviene dividir el trabajo en varios mapas.

¿Qué es el archivo mmcelt-errores.log? Es el registro de errores, y está junto al ejecutable. Solo aparece si ha ocurrido alguno. Es útil para diagnosticar un problema y se puede borrar sin consecuencias.

Al abrir me pregunta si quiero recuperar trabajo sin guardar. ¿Qué es eso? La aplicación guarda una copia de seguridad periódicamente, cada dos minutos de forma predeterminada. Si la sesión anterior terminó sin que guardaras, por un apagón o un cierre por error, al volver te ofrece esa copia. Te dice de cuándo es y qué mapa contiene, y decides tú. Si cerraste a propósito sin guardar, descártala.

¿Dónde se guardan esas copias automáticas? En la carpeta de datos de MMCelt, que en Windows es %APPDATA%\MMCelt\recuperacion\, y nunca junto a tus archivos, para que no aparezcan archivos sueltos en tus proyectos. Solo se conserva la última sesión, y la copia se borra en cuanto guardas de verdad.

¿El guardado automático sustituye a Ctrl+S? No, y es importante. Es una red de seguridad ante un cierre inesperado, no un lugar donde tu trabajo esté guardado. Sigue guardando tú.


Recomendaciones

Escribe las notas para alguien que no conoce el proyecto. Ese es exactamente el punto de partida del modelo en cada conversación nueva.

Actualiza los estados mientras trabajas. Un mapa en el que todo sigue en Idea tres semanas después no le dice nada a la IA. Los estados son la señal principal de por dónde va el trabajo.

Usa las dudas como lista de decisiones pendientes. Es la sección del documento exportado que más rendimiento da.

Guarda a menudo con Ctrl+S. Si el guardado falla, la aplicación te lo dirá: no informa de éxito sin comprobarlo.

Haz un mapa por proyecto, no por tarea. El valor está en que la IA vea el conjunto.
