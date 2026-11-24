Funcionalidades de MMCelt

Este documento es el catálogo completo de lo que hace la aplicación, con el comportamiento esperado de cada funcionalidad y los casos en que resulta útil. Para una guía práctica paso a paso, consulta el documento manual-de-uso.md de la carpeta documentacion.


Lienzo infinito

El lienzo es un plano en dos dimensiones, sin límites, donde se distribuyen los nodos del mapa. La vista se desplaza arrastrando con el botón central o con el derecho del ratón. La rueda del ratón acerca y aleja, y el zoom toma como centro la posición del cursor. Un nodo se mueve arrastrándolo con el botón izquierdo.

La tecla Inicio, con el teclado libre, encuadra el mapa entero en la ventana.

El nivel de zoom está limitado entre 0,2 y 3,5 aumentos. La retícula de fondo adapta su espaciado para seguir siendo útil en todo ese rango.

Caso de uso: en mapas grandes se trabaja acercándose a la rama en curso y alejándose para recuperar la visión de conjunto.


Estructura del mapa

Nodos

Cada nodo representa un concepto y contiene varios campos. El título es el texto visible en el lienzo. Las notas son la especificación detallada dirigida a la IA y el campo que más peso tiene en el documento exportado. La ruta de archivo vincula el nodo con un archivo o carpeta real del proyecto, por ejemplo src/auth/jwt.rs. Las etiquetas sirven para la clasificación temática, con valores como backend, rust o db. El estado expresa la madurez del concepto. La prioridad puede ser Baja, Media, Alta o Crítica. El rol indica la función arquitectónica del nodo dentro del mapa. La revisión humana es el control de supervisión sobre lo que genera la IA. El color, por último, sustituye al color heredado de la rama.

Estados

Hay seis estados. Idea es un concepto incipiente, sin desarrollar. Investigando indica que está en análisis técnico o en estudio de viabilidad. En Progreso significa que está en desarrollo activo. Duda / Bloqueo es una incógnita crítica que exige una decisión. Completado es un hito validado. Descartado es una opción rechazada, y la IA recibe la instrucción de no insistir en ella.

Prioridades

Cada nodo puede ponderar su urgencia relativa dentro del proyecto. Los cuatro niveles tienen iconos homogéneos tanto en el selector del inspector como en el lienzo, donde se dibujan antes del título. Baja corresponde a una tarea menor, una mejora secundaria o una sugerencia opcional. Media es la prioridad estándar para el desarrollo ordinario. Alta señala un componente crítico o un hito clave que hay que resolver a corto plazo. Crítica indica una urgencia máxima o un bloqueo que detiene el avance global.

Roles

Hay seis roles. Idea Central es la raíz del mapa. Pilar Estratégico es el primer nivel, una capa arquitectónica o un módulo principal. Subtema / Módulo es un elemento que depende de un pilar. Hipótesis / Duda es una pregunta técnica o una decisión pendiente. Tarea / Acción es un trabajo concreto con un entregable medible. Recurso / Herramienta es una biblioteca, una API o una documentación.

Los roles se asignan solos según la profundidad, de modo que los hijos de la raíz son pilares y el resto son subtemas, y se pueden cambiar a mano.

Creación y borrado

Un nodo hijo se crea con Tab o Insertar, y un nodo hermano con Enter. El título se edita con Espacio, con F2 o con un doble clic. Supr o Retroceso eliminan el nodo y su descendencia.

El borrado es en cascada: elimina el nodo y toda su descendencia, y descarta las conexiones cruzadas en las que participen. La raíz no se elimina; si se pide borrarla, se vacía de hijos pero permanece.

Qué significa cada icono de la tarjeta

Al pasar el ratón por un icono de una tarjeta aparece junto al puntero su significado, con los mismos nombres que usa el inspector. El icono de estado está delante del título y su texto emergente es, por ejemplo, Estado: Idea. La prioridad aparece arriba a la derecha, solo cuando es Alta o Crítica, con un texto como Prioridad: Alta. El control humano también aparece arriba a la derecha, salvo en los estados ordinarios, con un texto como Control Humano: Pendiente de Revisión. El icono del rol está abajo a la derecha, a la izquierda del icono de notas, con un texto como Rol: Subtema / Módulo. El icono de notas, un bloc, aparece abajo a la derecha si el nodo tiene notas, y su texto invita a seleccionar el nodo para leerlas en el inspector.

Las zonas sensibles se calculan con las mismas posiciones con las que se pintan los iconos, así que no pueden desajustarse. Mientras se arrastra un nodo no aparece ningún texto.

Menú contextual Acciones del nodo

Un clic derecho sin arrastrar sobre una tarjeta la selecciona y abre en ese punto un menú con lo que se suele hacer con un nodo. Los rótulos y atajos son los mismos del menú «Edición» y del inspector, para que cada opción se reconozca esté donde esté. Añadir nodo hijo equivale a Tab o Insertar. Añadir nodo hermano equivale a Enter, y está deshabilitado en la raíz, que no tiene hermanos. Crear conexión cruzada abre el cuadro de conexión con este nodo como origen. Editar el título equivale a Espacio o F2. Eliminar el nodo equivale a Supr. Los submenús Estado, Prioridad, Control humano y Rol muestran todas las variantes con la actual marcada.

Cada opción actúa sobre el nodo del clic, aunque antes hubiera otro seleccionado. Arrastrar con el botón derecho sigue moviendo la vista y no abre el menú. Esc o un clic fuera lo cierran sin cambios, y mientras está abierto no actúan los atajos que cambian el mapa. Con el mapa en modo de solo lectura no se abre. Elegir en un submenú el valor que el nodo ya tenía no cuenta como cambio.

Soltar un nodo encima de otro

Al soltar un nodo sobre un hermano, «Mover aquí sin tapar» lo deja en el hueco libre junto a él y además lo pone detrás en el orden de los hijos del padre común. En los modos automáticos el mapa se recoloca según el nuevo orden. Todo se deshace en un solo paso con Ctrl+Z.

Arrastrar un nodo y soltarlo con su centro encima de otra tarjeta abre en ese punto un menú pequeño, porque el gesto admite varias intenciones.

«Hacer hijo» hace que el nodo, con toda su descendencia, pase a colgar del de debajo; en Posición Libre Manual se aparta lo justo para verse, y en los modos automáticos el mapa se recoloca. «Hacer hermano» lo cuelga del mismo padre que el de debajo, justo detrás de él, salvo si el de debajo es la raíz o si ya son hermanos. «Conectar con enlace» crea una conexión cruzada hacia el de debajo y devuelve el nodo arrastrado a su sitio; no se ofrece si ya son padre e hijo o si ya están conectados, porque sería otra línea encima de la que ya los une. «Mover aquí sin tapar» lo deja junto al de debajo, en el hueco libre más cercano a donde se soltó, sin solapar ninguna tarjeta; si el de debajo es su hermano, además lo pone detrás de él en el orden. «Cancelar» lo devuelve a donde estaba, igual que Esc o un clic fuera del menú.

«Hacer hijo» aparece deshabilitado, con la explicación al pasar el ratón, cuando el árbol no lo admite: si el arrastrado es la raíz, si el de debajo ya es su padre o si el de debajo está dentro de su propia descendencia, lo que crearía un bucle. Soltar en el vacío, o hacer un clic sin mover el nodo, no abre nada. Mientras el menú está abierto, los atajos que cambian el mapa no actúan, y cualquier opción se deshace con Ctrl+Z.

Caso de uso: reorganizar la jerarquía con el ratón, sin tener que borrar y volver a crear una rama para cambiarla de sitio.


Qué significan los colores

Los colores del lienzo no son decorativos: dicen a qué rama pertenece cada cosa.

Las líneas que unen los nodos

Cada rama principal, es decir, cada hijo directo de la idea central, recibe un color, y toda su descendencia lo hereda. La primera rama es azul, la segunda esmeralda, la tercera ámbar, la cuarta púrpura, la quinta rosa, la sexta turquesa, la séptima naranja y la octava índigo. Con más de ocho ramas la paleta se reutiliza, de modo que la novena vuelve al azul.

Eso permite seguir cualquier hilo hasta el final y saber de dónde viene, aunque el mapa esté enredado y las ramas se crucen por la pantalla.

Las conexiones cruzadas son distintas

Van siempre en ámbar, con un color propio que no depende de la rama. Así se distingue de un vistazo lo que es jerarquía de lo que es una dependencia entre ramas distintas.

En el lienzo todas se dibujan igual, con la misma curva y el mismo grosor. Lo que distingue una de otra es la etiqueta de texto que aparece en su punto medio: la que se escribió al crearla o, si se dejó vacía, el nombre del tipo de relación.

Donde sí cambian de forma es en el diagrama Mermaid del documento exportado, para que el modelo distinga el tipo de un vistazo. Dependencia (Requiere) se dibuja como una flecha gruesa, Inspirado por como una línea discontinua, Bloquea como una línea con aspas, Alternativa a como una flecha doble y Sinergia como una línea gruesa sin punta.

La barra vertical del borde izquierdo de cada nodo lleva el color de la rama de la que viene, el mismo que la línea que llega hasta él, así que se puede seguir cualquier hilo hasta el final y saber de dónde sale. La idea central usa el primer color de la paleta.


Conexiones cruzadas

Son relaciones entre nodos que no comparten rama, para expresar dependencias que la jerarquía no puede representar. Hay cinco tipos. Dependencia (Requiere) indica que el origen necesita que el destino esté disponible. Inspirado por indica que el origen toma el destino como referencia. Bloquea indica que el origen impide el progreso del destino. Alternativa a indica que son soluciones excluyentes entre sí. Sinergia indica que se potencian mutuamente.

Cada conexión lleva una etiqueta que explica el motivo. En el documento exportado aparecen como una matriz de dependencias.

Caso de uso: marcar que la elección de la base de datos bloquea el diseño de la API, de modo que la IA sepa que no debe proponer puntos de acceso antes de resolver esa duda.


Disposición automática

Hay tres modos. Árbol Balanceado reparte las ramas de primer nivel a izquierda y derecha de la raíz, y calcula las alturas para que ninguna se solape. Radial / Circular distribuye las ramas en abanico alrededor de la raíz; cada rama recibe el trozo de círculo proporcional al sitio que necesita, así que una rama gruesa no empuja a las demás hacia fuera. Posición Libre Manual no recoloca nada y respeta las posiciones que fije el usuario.

Los dos primeros modos recalculan las posiciones al añadir o eliminar nodos.


Exportación a Markdown para IA

Es la funcionalidad central del programa. Genera un documento estructurado con técnicas de ingeniería de prompts, pensado para que un modelo lo lea de una vez.

Para leer el texto antes de escribir ningún archivo, está la opción «Previsualizar Markdown de IA (.md)...» del menú «Inteligencia Artificial». Para exportarlo, la opción «Exportar Markdown para IA (.md)» del menú «Archivo», la opción «Exportar Archivo .md para IA...» del menú «Inteligencia Artificial» o el atajo Ctrl+E.

El documento empieza con unas meta-instrucciones para el modelo y sigue con nueve secciones numeradas, en este orden: la visión del creador, con qué se busca y por qué; un resumen ejecutivo con métricas y desglose por estado; la estructura jerárquica con notas, rutas de código y etiquetas; el control humano, con las correcciones exigidas y las ramas aprobadas; la matriz de dependencias con las conexiones cruzadas; los puntos de decisión, que agrupan y priorizan las dudas abiertas; un diagrama Mermaid del grafo completo; unos prompts de acción sugeridos, y las instrucciones para devolver el trabajo al mapa por el servidor MCP.

El orden importa. La visión y los objetivos van primero para que el modelo lea la intención antes que el detalle. Las dudas se agrupan en una sección propia, en lugar de quedar dispersas por el árbol, para que el modelo pueda abordarlas como una lista.

La vista previa permite revisar el documento antes de exportarlo y copiarlo directamente al portapapeles.


Importación desde IA

Es el camino inverso: convierte la respuesta de un modelo en un mapa mental. La aplicación incluye un prompt maestro listo para copiar, que indica al modelo el esquema JSON exacto que debe generar. Se pega la respuesta y MMCelt reconstruye el mapa.

Tolerante a fallos, pero no silencioso

El importador intenta dos interpretaciones. Primero lee la respuesta como un bloque JSON con el esquema del prompt maestro, y localiza ese bloque aunque venga envuelto en un bloque de código marcado como json o rodeado de texto conversacional. Si eso falla, la lee como un esquema Markdown de títulos y viñetas.

Acepta además sinónimos en los valores: un modelo que escriba In Progress en lugar de EnProgreso, o urgent en lugar de Critica, se interpreta correctamente.

Si ninguna interpretación funciona, lo dice. El mensaje explica por separado qué falló en cada intento, para que el usuario sepa qué corregir. No devuelve un mapa vacío fingiendo éxito.


Control humano

Es el mecanismo para dirigir a la IA cuando se desvía, lo que en inglés se llama human in the loop. Cada nodo tiene un estado de revisión. Sin marcar es el de los nodos que crea el propio usuario, incluidos los de las plantillas y los mapas de ejemplo: no hay nada que revisar y la tarjeta no lleva icono de revisión. Generado por IA indica que lo propuso un modelo y está pendiente de auditoría. Pendiente de Revisión indica que espera validación, casi siempre porque la IA cambió un nodo que ya existía. Aprobado por Usuario indica que se validó explícitamente. Requiere Corrección indica que el usuario exige cambios.

Los estados de revisión muestran sus iconos de forma homogénea, tanto en el selector desplegable del inspector como en el nodo dentro del lienzo. El icono de Pendiente de Revisión se pinta en el color de aviso del tema, para que destaque lo que queda por revisar.

Lo que la IA acaba de crear se resalta

Un nodo que la IA acaba de crear se dibuja con un fondo y un borde propios, en los tres temas, hasta que el usuario lo selecciona. Al seleccionarlo, con un clic, desde la búsqueda o con el clic derecho, recupera su aspecto normal y conserva el icono de Generado por IA. Abrir el mapa o recargarlo cuando la IA lo cambia no cuenta como abrirlo.

Quitar el resalte no es un cambio del mapa: no se puede deshacer y no impide que el mapa se siga recargando mientras la IA trabaja. Lo que el usuario ya abrió no vuelve a resaltarse aunque el archivo se recargue o se deshaga un paso. El resalte quitado se guarda en el archivo con el resto del mapa la próxima vez que se guarde; por eso, al cerrar el programa, cuenta como un cambio sin guardar y MMCelt pregunta si se guarda. Si se cierra sin guardar, al volver a abrir el mapa esos nodos aparecen resaltados de nuevo.

Lo que llega de la IA nace resaltado por cualquiera de sus tres puertas: el mapa que crea un agente con mmcelt_create_mindmap, los nodos que añade con mmcelt_sync_ai_progress y lo que se pega desde «Importar de IA», tanto en JSON como en Markdown. En lo importado, un nodo sin estado de revisión entra como Generado por IA; uno que se declara aprobado entra como Pendiente de Revisión, porque la aprobación solo la da una persona.

Los nodos marcados como Requiere Corrección admiten un texto con la instrucción concreta. Con ellos se genera un prompt de corrección que contiene solo las directivas de rumbo, sin repetir el mapa entero; está pensado para reconducir a una IA que ya conoce el proyecto.

Caso de uso: la IA propone MongoDB y se quiere PostgreSQL. Se marca el nodo como Requiere Corrección, se escribe el motivo y se envía el prompt de corrección.

La aprobación humana caduca

Marcar un nodo como Aprobado por Usuario es dar el visto bueno a ese contenido. Si después la IA lo cambia, ya sean las notas, el estado, la prioridad o la ruta de archivo, el nodo vuelve solo a Pendiente de Revisión. Lo mismo ocurre con un nodo Sin marcar o que Requiere Corrección: si la IA lo cambia, pasa a Pendiente de Revisión, para que el usuario vea qué ha tocado.

El motivo es que se aprueba un texto, no un nodo para siempre. Sin esta caducidad, el usuario vería su propio visto bueno sobre algo que no ha leído nunca, y la supervisión dejaría de significar nada.

Si la IA cambia las notas, el estado, la prioridad o la ruta, la aprobación caduca y el nodo vuelve a Pendiente de Revisión. Si la IA envía los mismos valores que ya había, la aprobación se conserva, porque no ha cambiado nada. Si la IA crea un nodo nuevo, nace como Generado por IA y resaltado, sin aprobación que perder. Si el nodo ya era Generado por IA, sigue igual: no es trabajo del usuario que haya que señalar.

La corrección exigida no se borra nunca, ni siquiera cuando la IA la acata, para que se pueda comprobar si hizo lo que se le pidió.

Un agente no puede declarar por su cuenta que algo está aprobado: todo lo que crea a través del servidor MCP nace marcado como generado por IA. Si pudiera aprobar su propio trabajo, la supervisión sería un adorno.


Ejemplos y plantillas

Ejemplos: mapas completos

Están en el menú «Archivo», en «Ejemplos (mapas completos)».

A diferencia de las plantillas, vienen terminados: con notas escritas, dudas abiertas, estados repartidos, opciones descartadas y conexiones cruzadas. Sirven para ver de un vistazo cómo queda un mapa que de verdad dirige a una IA, que es lo que cuesta imaginar partiendo de un lienzo vacío.

El ejemplo Juego del Ahorcado es un proyecto de programación pequeño, con lógica, interfaz, banco de palabras, estadísticas y pruebas. Es un juego que cabe entero en la cabeza, para que la atención vaya a cómo está anotado el mapa y no a entender el dominio. El ejemplo Negocio de comida a domicilio es deliberadamente no técnico: trata de producto, permisos, costes, clientes y logística, y muestra que la herramienta sirve igual para pensar un negocio que para diseñar software.

Al abrirlos conviene fijarse en tres cosas. Los nodos en Duda / Bloqueo son preguntas reales sin resolver; al exportar, la IA las recibe agrupadas como una lista de decisiones pendientes. Los nodos en Descartado dejan constancia de lo que ya se decidió no hacer, para que el modelo no lo vuelva a proponer. Las conexiones cruzadas expresan dependencias que la jerarquía no puede expresar: en el ejemplo del negocio, el nodo que pregunta si se puede vender desde la propia cocina bloquea el cálculo de costes, porque alquilar un obrador lo cambiaría todo.

Plantillas: estructuras para rellenar

Están en el menú «Archivo», en «Plantillas de Arquitectura». La plantilla Arquitectura Limpia contiene las cuatro capas (dominio, casos de uso, infraestructura y presentación) con sus subtemas, rutas sugeridas y dos conexiones que ilustran la regla de dependencia. La plantilla Fullstack contiene frontend, backend, base de datos y despliegue, con las conexiones del flujo de datos.


Escáner de repositorios

Convierte la estructura de carpetas de un proyecto real en un mapa mental. Recorre hasta tres niveles de profundidad, omite las carpetas ocultas y las de artefactos (target, node_modules, dist y build) y crea un nodo por cada carpeta y cada archivo de código reconocido, con su ruta ya rellenada.

Se limita a quince archivos de código por carpeta para que el mapa siga siendo legible. Los archivos que no son código, como imágenes, fuentes o datos, no se incluyen en el mapa ni cuentan para ese límite.

Caso de uso: partir de un código existente para pedir a la IA un plan de refactorización sobre su estructura real.


Persistencia

MMCelt trabaja con cinco formatos. El formato .mmcelt es el nativo, en JSON, y guarda el mapa completo. El formato .json es equivalente, con otra extensión. El formato .md es la exportación para IA y solo se escribe, no se lee. El formato .opml sirve para intercambiar esquemas; es un XML estándar no propietario y admite exportación e importación con metadatos incrustados. El formato .mm es el de los mapas mentales de FreeMind y Freeplane, también XML estándar no propietario, con exportación e importación con metadatos incrustados.

El atajo de guardado es Ctrl+S.

Al cerrar la ventana con cambios sin guardar, MMCelt pregunta antes de cerrarse, con tres botones. Guardar guarda el mapa y cierra; si el mapa todavía no tiene archivo, abre «Guardar como», y si ahí se cancela o el guardado falla, el programa no se cierra. No guardar cierra sin guardar. Cancelar anula el cierre y deja el programa abierto, sin guardar nada. Cuentan como cambios sin guardar las ediciones del mapa, un título a medio escribir y los nodos nuevos de la IA que se han abierto desde el último guardado. Un mapa que todavía no tiene archivo también pregunta aunque no se haya tocado, porque nada de lo que hay en pantalla está en disco: es el caso de un escaneo, una importación, una plantilla o un ejemplo abiertos desde el menú. Solo hay dos excepciones, en las que no hay nada que perder: el mapa de ejemplo con el que arranca el programa y un mapa nuevo con solo la idea central, mientras no se toquen. Si no hay nada de todo eso, el programa se cierra sin preguntar.

Formatos no propietarios de intercambio: OPML y FreeMind .mm

Por compromiso con los estándares abiertos, MMCelt solo admite formatos de intercambio no propietarios. OPML, siglas de Outline Processor Markup Language, es un estándar XML basado en esquemas jerárquicos, con elementos outline anidados, compatible con gestores de ideas, editores estructurados como Workflowy u OmniOutliner y lectores RSS. El formato .mm de FreeMind y Freeplane es el estándar XML de hecho para mapas mentales libres, compatible con FreeMind, Freeplane y los visores que lo admiten. Los formatos cerrados o propietarios, como .xmind o .mmap, se descartan deliberadamente.

Estudio comparativo: qué se gana y qué se pierde con cada formato

Los formatos clásicos como OPML o .mm se concibieron para árboles jerárquicos simples, de padre a hijos, con un título y una nota de texto. No tienen soporte nativo para los conceptos de arquitectura y supervisión de MMCelt: estado de madurez, prioridad, rol arquitectónico, supervisión humana, rutas a código fuente, etiquetas temáticas ni conexiones cruzadas laterales.

Para resolverlo sin perder información, MMCelt usa metadatos estructurados y transparentes. Lo que el formato admite de forma nativa viaja en sus campos estándar: el título, la jerarquía de ramas y las notas de texto libre del usuario se exportan en el atributo text y el atributo _note del elemento outline en OPML, y en el atributo TEXT del elemento node con un elemento richcontent de tipo NOTE en .mm. Lo que no cabe de forma nativa viaja como JSON estructurado: al final de la nota de cada nodo se añade una línea técnica que empieza por MMCELT_METADATOS_NODO_JSON y lleva los campos que el formato no puede representar. En el nodo raíz se añade además una línea que empieza por MMCELT_METADATOS_PROYECTO_JSON, con la visión del creador, los objetivos y las conexiones cruzadas.

A continuación se detalla, campo por campo, qué se conserva de forma nativa, qué viaja en el bloque estructurado y cómo se comporta al abrir el archivo en otra aplicación o al reimportarlo en MMCelt.

La jerarquía del mapa es nativa en .mmcelt (padre e hijos), en OPML (elementos outline anidados) y en .mm (elementos node anidados). Otra aplicación muestra la estructura de árbol completa, y al reimportar se reconstruye el árbol completo.

El título del nodo es nativo en .mmcelt (campo title), en OPML (atributo text o _text) y en .mm (atributo TEXT). En otra aplicación se ve como texto del nodo o del elemento, y al reimportar se recupera intacto.

Las notas de texto del usuario son nativas en .mmcelt (campo notes), en OPML (atributo _note o elemento notes) y en .mm (elemento richcontent de tipo NOTE). En otra aplicación se leen como la nota del nodo, y al reimportar se recupera la prosa limpia, sin las marcas de metadatos.

El estado de madurez, la prioridad y el rol arquitectónico son nativos en .mmcelt (campos status, priority y role) y viajan como JSON en la nota tanto en OPML como en .mm. En otra aplicación se leen como texto JSON al pie de la nota, y al reimportar se reconstruye cada valor con su tipo.

La supervisión y la corrección humana son nativas en .mmcelt (campos review_status y correction_feedback) y viajan como JSON en la nota en OPML y en .mm. En otra aplicación se leen como texto JSON al pie de la nota, y al reimportar se reconstruyen el estado y el texto de corrección.

La ruta de archivo de código es nativa en .mmcelt (campo file_path) y viaja como JSON en la nota en los otros dos formatos. En otra aplicación se lee como texto JSON, y al reimportar se recupera la vinculación con el disco.

Las etiquetas temáticas son nativas en .mmcelt (campo tags) y viajan como JSON en la nota en los otros dos formatos. En otra aplicación se leen como texto JSON, y al reimportar se recuperan las etiquetas.

La posición X e Y en el lienzo es nativa en .mmcelt (campo pos) y viaja como JSON en la nota en los otros dos formatos. La otra aplicación usa su propio algoritmo de colocación de ramas, y al reimportar se recuperan las coordenadas.

Las ramas plegadas son nativas en .mmcelt (campo collapsed); en OPML viajan como JSON en la nota, y en .mm viajan de forma nativa, con el atributo FOLDED a true, y también en el JSON. Otra aplicación respeta el plegado si lo admite, y al reimportar se recupera el estado de plegado.

Las conexiones cruzadas son nativas en .mmcelt (campo connections) y viajan como JSON en la nota de la raíz en OPML y en .mm. En otra aplicación solo se ven como texto de datos en la nota de la raíz, y al reimportar se reconstruyen todas.

La visión y los objetivos del proyecto son nativos en .mmcelt (campos creator_vision, project_goals y otros) y viajan como JSON en la nota de la raíz en OPML y en .mm. En otra aplicación se ven como texto al final de la nota de la raíz, y al reimportar se recuperan todos los campos del proyecto.

Balance: qué se gana y qué se pierde

Se gana, en primer lugar, interoperabilidad real: cualquier programa capaz de leer OPML o FreeMind puede abrir los mapas de MMCelt y aprovechar su estructura, sus títulos y sus notas. En segundo lugar, una ida y vuelta exacta: al exportar un mapa de MMCelt a OPML o a .mm y volver a importarlo en MMCelt se reconstruye toda la información, incluidos estados, prioridades, roles, rutas de código y conexiones cruzadas. En tercer lugar, transparencia: la información no queda encerrada en un formato binario ni propietario, y el JSON de las notas es estándar y legible por personas y por herramientas de automatización.

Al abrir el archivo en otra aplicación se pierden tres cosas. La primera es la semántica de la interfaz: la otra aplicación no reconoce los estados, las prioridades ni los roles como controles propios, y aparecen como texto JSON complementario en el panel de notas. La segunda es el dibujo de las conexiones cruzadas: las herramientas clásicas representan árboles estrictos, así que las relaciones laterales entre ramas no se dibujan como flechas; viajan a salvo en el JSON, pero solo MMCelt las pinta en su lienzo. La tercera es la disposición espacial libre: OPML no tiene coordenadas y .mm coloca los nodos en forma radial o bifurcada, de modo que las posiciones manuales de MMCelt no se reflejan en herramientas sin un lienzo libre.

Importación de archivos externos sin metadatos de MMCelt

Al importar un archivo .opml o .mm creado en otra aplicación, o que no contenga los bloques de metadatos de MMCelt, la jerarquía, los títulos y las notas se importan fielmente, y toda la prosa de la nota se conserva como notas del usuario.

Como el archivo ajeno no trae los campos propios de MMCelt, el programa asigna siempre los mismos valores por defecto: estado Idea; prioridad Media; rol Idea Central para la raíz y Subtema / Módulo para el resto de nodos, sin distinción por nivel; supervisión humana sin revisar, y ninguna ruta de archivo ni etiqueta.

Como tampoco trae coordenadas compatibles, MMCelt aplica automáticamente la disposición balanceada y despliega las ramas a izquierda y derecha del centro. Así los nodos no aparecen amontonados en el origen y el mapa queda legible y editable desde el primer momento.

Por último, MMCelt muestra un diálogo informativo en el idioma activo que explica que el archivo procede de una fuente externa y que se han establecido los atributos por defecto.

Comprobaciones al abrir

Un archivo se rechaza si describe un mapa imposible: con ciclos, sin nodo raíz, con referencias a nodos inexistentes, con nodos inalcanzables desde la raíz o con más de 512 niveles de profundidad o de 100 000 nodos. El mensaje de error identifica el nodo concreto que causa el problema.

No es una restricción arbitraria: sin ella, un archivo dañado podría congelar la aplicación o cerrarla de golpe. Los límites están muy por encima de cualquier mapa que una persona vaya a construir.

Guardado verificado

Si al guardar ocurre un error, como falta de permisos, disco lleno, archivo bloqueado o una carpeta de la nube sin sincronizar, la aplicación lo comunica. Nunca informa de un guardado correcto sin haberlo comprobado.

Guardado automático

Si el mapa ha cambiado, la aplicación escribe periódicamente una copia de seguridad, cada dos minutos de forma predeterminada. El intervalo se elige en el menú «Ver y Diseño», en «Autoguardado»: desactivado, cada minuto, cada dos, cada cinco o cada diez minutos. La elección se recuerda entre sesiones.

Los valores son una lista cerrada, y no un número libre, a propósito: un intervalo de dos segundos escribiría el mapa entero continuamente, y uno de ocho horas sería no tener autoguardado aparentando tenerlo. Un valor fuera de esa lista escrito a mano en el archivo de preferencias se ignora. Si el programa se cierra de forma inesperada, al volver a abrirlo se ofrece recuperar la copia.

El guardado automático no sustituye a guardar. Es una red de seguridad, y esa diferencia explica cómo se comporta. Guardar con Ctrl+S lo decides tú, escribe donde elijas, pregunta la primera vez y sirve para conservar el trabajo. El guardado automático lo decide el programa, escribe siempre en la carpeta de datos de MMCelt, no pregunta nunca y sirve para recuperar el trabajo tras un cierre inesperado.

La copia nunca se deja junto a tus archivos: va a la carpeta de datos de la aplicación, que en Windows es %APPDATA%\MMCelt\recuperacion\. Así no aparecen archivos sueltos en tus proyectos. Solo se conserva la última sesión, y al guardar de verdad la copia se descarta, porque el trabajo ya está en su sitio.

En los modos de disposición automática, mover un nodo no cuenta como cambio, porque el programa va a recolocarlo igualmente, así que un mapa que solo se está mirando no provoca escrituras. En Posición Libre Manual sí cuenta, porque ahí colocar los nodos es el trabajo del usuario.

Al arrancar tras un cierre inesperado aparece un aviso con la fecha de la copia y el mapa que contiene, con dos opciones: recuperarla o descartarla. No se restaura sola, porque podrías haber cerrado a propósito sin guardar, y devolverte lo que descartaste sería peor que no ofrecer nada.

Dónde propone guardar

Cuando hay que elegir carpeta, el diálogo se abre en el sitio más probable. Si el mapa ya se había guardado, propone la carpeta del archivo actual. Si no, y sus nodos llevan rutas de archivo, propone la carpeta del proyecto de código al que se refiere el mapa, que se deduce como la raíz común de esas rutas; es el caso de los mapas que genera el escáner de repositorios y de los que produce una IA trabajando sobre un código concreto. Si no hay nada mejor, propone la carpeta que recuerde el sistema.

Es una sugerencia, no una imposición: puedes ir a donde quieras. La aplicación nunca escribe por su cuenta en una carpeta deducida, porque eso podría colar archivos dentro de un repositorio sin que te dieras cuenta.

Si trabajas con el servidor MCP, este punto ya está resuelto por otra vía: el agente indica la ruta y el mapa se guarda directamente en la carpeta del proyecto.


Integración con agentes de IA y servidor MCP

MMCelt incluye un servidor que permite a agentes de consola como Claude Code, Codex CLI y Antigravity CLI (agy) leer y escribir mapas directamente, sin copiar y pegar.

Enviar a un agente y recibir el mapa de vuelta

En el menú «Inteligencia Artificial», la opción «Enviar a...» tiene un submenú con cada agente detectado en el equipo. El envío es supervisado: pulsar un agente no manda nada todavía.

La ventana «Proyecto e instrucciones para la IA» permite preparar el contenido antes de elegir un agente. La pestaña Proyecto mantiene los datos del mapa; la pestaña Instrucciones conserva el perfil portable; la pestaña Plantillas de encargo copia una sola petición, y la pestaña Vista completa muestra la composición exacta. El pie común mantiene visibles desde las cuatro pestañas el idioma del documento y la acción «Guardar como reglas comunes». El idioma del documento no depende del idioma de la interfaz, se guarda en cuanto cambia y reaparece al abrir de nuevo la ventana; los textos escritos por la persona nunca se traducen automáticamente.

Al pulsar un agente se ve una vista previa con el prompt completo, en cuatro bloques, y ninguno se puede editar en ella. El contrato de MMCelt lo genera la aplicación y contiene las reglas mínimas de integridad y de devolución por MCP. Las reglas comunes del proyecto salen de .mmcelt/instrucciones-agente.md, el mismo perfil para todos los agentes, y se escriben en la pestaña Instrucciones de la ventana «Proyecto e instrucciones para la IA». El contexto del mapa es la exportación para IA del mapa abierto y se cambia en el mapa. El encargo de esta sesión se escribe en esa misma ventana, a mano o a partir de una plantilla de la pestaña Plantillas de encargo. Si el encargo está vacío, la vista previa lo avisa, no deja confirmar y ofrece el botón «Configurar encargo...», que abre esa ventana en la pestaña del encargo.

La precedencia va escrita dentro del prompt: primero las correcciones humanas del mapa, después el contrato de MMCelt, después las reglas del proyecto y, por último, el encargo de la sesión.

Debajo de la vista previa solo hay dos botones: «Cancelar», que no guarda, no exporta, no vigila y no deja expediente, y el que confirma el envío. «Guardar como reglas comunes», en la ventana «Proyecto e instrucciones para la IA», es la única acción que escribe el perfil del proyecto en disco. Cambiar el idioma del documento sí guarda inmediatamente esa preferencia, pero no guarda ni altera el texto del perfil.

Instrucciones nativas con consentimiento duradero. Si en la raíz hay AGENTS.md, CLAUDE.md o GEMINI.md, la vista los enumera y deja leerlos. Una fuente nueva permanece desmarcada hasta que la persona acepta esa versión. Se guardan su contenido y su huella SHA-256 en .mmcelt/configuracion-agente.json; si el archivo cambia, vuelve a quedar desmarcado y se muestran la versión anterior y la actual. La selección final sigue siendo visible y ajustable en cada sesión. Si no existe ninguna fuente, la propia pestaña enumera los tres nombres admitidos y muestra la raíz exacta donde la persona puede crear una a mano; MMCelt no crea instrucciones por ella.

Un único compositor validado produce la vista, la confirmación y el archivo inicio.md. Rechaza caracteres de control invisibles, limita cada bloque editable a 65.536 bytes y el conjunto a 16 MiB. Una confirmación caduca si cambian el mapa, las reglas, el encargo o el contenido de una fuente antes del lanzamiento.

El botón final refleja la capacidad real: dice «Iniciar en consola» cuando existe una consola del agente y «Preparar para MCP» cuando el agente solo dispone del servidor. Con consola, sigue un orden fijo: guarda el mapa, para que el agente lea lo mismo que hay en pantalla; exporta el Markdown para la IA junto al mapa, con el sufijo _AI.md; escribe el expediente de la sesión en .mmcelt/sesiones; abre la consola del agente en la carpeta del proyecto, y activa la vigilancia del mapa. Si la consola no llega a abrirse, MMCelt no dice que el mapa se haya enviado, no empieza a vigilar y deja el expediente marcado como fallido.

Para un destino que solo tiene MCP, MMCelt guarda, exporta, escribe el expediente y activa la vigilancia, pero no abre ninguna consola: el expediente queda en estado Preparado, la vista indica que no se abrirá consola y la persona continúa desde la interfaz propia del agente.

Una carpeta creada con una versión anterior puede no tener todavía el archivo .mmcelt/configuracion.json. La vista previa usa entonces un límite provisional y no escribe esa identidad; solo se inicializa al confirmar.

El expediente de cada sesión está en .mmcelt/sesiones. Cada confirmación crea una carpeta nombrada con la fecha UTC, el identificador fijo del agente y un UUID, nunca con texto del usuario, que contiene dos archivos. El archivo inicio.md guarda el prompt exacto que se aprobó, letra por letra. El archivo sesion.json guarda la versión de formato, el identificador, la fecha UTC, el agente, la ruta relativa del mapa, las fuentes detectadas e incorporadas, el tamaño del prompt y el estado, que puede ser Preparado, Iniciado o Fallido. No se escriben contraseñas, claves, correos ni identificadores de cuenta. Para repetir una sesión basta con abrir la consola del agente en la carpeta y pedirle que lea ese inicio.md.

Después, cuando el agente cambia el mapa en disco, la ventana se actualiza sola, sin importar nada. Un indicador verde en la barra superior avisa de que la vigilancia está activa, y la opción «Detener seguimiento» la corta con un clic. La consola del agente es un proceso aparte: se cierra en su propia ventana, y detener la vigilancia no la cierra.

Mientras el agente trabaja, el mapa queda bloqueado. Desde que se confirma el envío no se puede editar, para que la persona y el agente no cambien el mismo archivo a la vez; se puede ver, recorrer, seleccionar, buscar y exportar, y la barra inferior lo avisa. Cualquier cambio del contenido que llegue a hacerse por cualquier vía se deshace al final del fotograma. El agente marca su última entrega como trabajo terminado (work_finished en mmcelt_sync_ai_progress y mmcelt_create_mindmap; el documento que recibe se lo pide), y MMCelt, que lee el recibo de entrega cada dos segundos mientras dura el bloqueo, carga el mapa final y lo levanta. El botón «Recuperar el control» lo quita si el agente falla o no avisa, y si el envío falla antes de lanzar al agente, el bloqueo se quita solo.

Si el usuario también estaba editando, la regla manda sobre todo lo demás: su trabajo no se pierde. Sin cambios locales pendientes de guardar, el mapa se recarga en el momento. Con cambios sin guardar, el mapa no se toca; se avisa y decide la persona. Un archivo a medio escribir se ignora y se espera al siguiente aviso, sin mostrar ningún error. Una escritura del propio MMCelt se reconoce por su contenido y no provoca una recarga. Antes de cualquier recarga se guarda una copia en el archivo de recuperación.

MMCelt abre la consola de Claude Code, Codex CLI y Antigravity CLI (agy). La vuelta, en cambio, funciona igual para cualquier agente: cualquiera que escriba el archivo por MCP provoca la recarga.

MCP y consola son dos capacidades separadas, y el menú lo indica con un icono por agente. Un destello significa que el agente tiene consola y MCP, de modo que puede conversar y devolver trabajo al mapa. Una pantalla significa que solo tiene consola: MMCelt encuentra su ejecutable, pero falta registrar el servidor MCP. Un eslabón significa que solo tiene MCP: puede devolver trabajo, pero MMCelt no puede abrirle una consola. Un enchufe significa que no tiene ninguna de las dos. Que un agente figure conectado por MCP no significa que se le pueda abrir una consola.

Las cuentas las pone la consola del agente, no MMCelt. La aplicación no pide, no guarda y no transforma credenciales: se apoya en la sesión que ya esté autenticada en claude, codex o agy. No usa ninguna API de pago, no elige modelo y no decide la facturación.

En cuanto a plataformas, el recorrido está comprobado en Windows, donde la detección respeta PATHEXT y no elige el guion Unix sin extensión que dejan algunas instalaciones de npm. El adaptador de Linux está implementado con una lista cerrada de terminales, pero todavía no está comprobado en esta versión. En macOS todavía no se garantiza.

Conexión en un clic

Está en el menú «Inteligencia Artificial», en «Conectar MMCelt con mis IAs...». La ventana detecta los agentes instalados en el equipo, muestra cuáles están conectados y permite conectarlos o desconectarlos con un botón. No hay que editar ningún archivo ni abrir una terminal.

Que un agente figure como conectado significa que la entrada no solo existe, sino que la ruta registrada sigue apuntando a un archivo real y contiene el argumento exacto --mcp-server. Si MMCelt se mueve, se renombra o se borra, la ventana vuelve a ofrecer Conectar para reparar la configuración obsoleta.

Reconoce tres agentes: Claude Code, el asistente de programación de Anthropic en la terminal; Codex CLI, el agente de programación de OpenAI en la terminal, y Antigravity CLI (agy), el agente oficial de Google en la terminal, que funciona con una cuenta normal de Google. Los clientes de entorno gráfico y los editores, como Claude Desktop, Antigravity IDE, Cursor o Windsurf, no forman parte del catálogo, para simplificar el flujo y trabajar exclusivamente con agentes oficiales de consola.

Un agente aparece si existe su carpeta de configuración, si existe su archivo de configuración o si se encuentra su programa. Esa tercera vía es la que hace visible una consola recién instalada que todavía no ha escrito nada en el disco. El programa se busca primero en la carpeta donde lo deja su instalador oficial, aunque esa carpeta no esté en el PATH: en Windows, %LOCALAPPDATA%\Programs\OpenAI\Codex\bin para Codex CLI y %LOCALAPPDATA%\agy\bin para Antigravity CLI. Después se busca en la carpeta .local/bin de la carpeta personal, que es donde lo deja el instalador de Claude Code en todos los sistemas y los de Codex CLI y Antigravity CLI en macOS y Linux. Por último se busca en el PATH.

Cada ficha enseña la ruta del programa que se va a usar y si se ha detectado solo o se ha elegido a mano. Si un agente está instalado en otra carpeta, el botón «📂 Cambiar ruta...» de su ficha abre el selector de archivos para elegir su programa; la ruta elegida se guarda en las preferencias y manda sobre la detección. El botón «Usar la detección» la quita. Si la ruta elegida deja de existir o no es un programa, la ficha lo avisa con la ruta y MMCelt usa el programa que encuentre por su cuenta.

Cada cuenta de un agente tiene su propia ficha. Una herramienta como Claude Code o Codex CLI puede tener varias cuentas en el mismo equipo, cada una en su carpeta de configuración, y elige cuál usar por una variable de entorno (CLAUDE_CONFIG_DIR o CODEX_HOME). MMCelt las busca sin conocer ningún nombre de antemano: la carpeta por defecto de cada herramienta, las carpetas de la carpeta personal que se llaman como ella seguida de algo más y contienen los archivos de una cuenta, y la que señale la variable. La ficha de una cuenta que no es la por defecto lleva el nombre de su carpeta. Conectar escribe en el archivo de esa cuenta, y al enviar un mapa la consola se abre con la variable apuntando a su carpeta; con la cuenta por defecto, la variable se quita. El botón «Añadir cuenta» da de alta a mano la carpeta de otra cuenta, que se guarda en las preferencias, y «Quitar cuenta» la retira de la lista sin borrar nada. Antigravity CLI (agy) es la excepción: solo se usa su cuenta por defecto y no admite añadir cuentas a mano. El motivo es que agy no documenta ninguna variable de entorno para elegir otra carpeta de configuración y siempre lee su registro de servidores de ~/.gemini/config/mcp_config.json. Una cuenta de agy añadida a mano aparecería en la lista, pero conectarla escribiría en un archivo que agy no lee y la consola se abriría igualmente con la cuenta por defecto, así que se ha preferido no ofrecerla antes que dar una opción que no funciona. Si Google documenta esa variable, se podrá tratar como las demás. La búsqueda está en la biblioteca bibliotecas/deteccion-agentes, para que otros programas puedan reutilizarla.

La detección se repite cada vez que se abre la ventana. Si instalas un agente nuevo más adelante, aparecerá sin reiniciar MMCelt ni recordar ningún paso.

En esa ventana también se elige la carpeta de trabajo, la única dentro de la cual los agentes podrán leer y escribir. Mientras no elijas ninguna, se propone la del mapa que tengas abierto y no una carpeta fija, porque autorizar Documentos/MapasMentales cuando tu proyecto está en otro sitio le daría al agente permiso para trabajar donde no hay nada, y cada ruta que probara chocaría contra el límite del espacio de trabajo. En cuanto eliges una carpeta a mano, manda esa: abrir un mapa en otro sitio no te la cambia.

Si cambias la carpeta, los agentes ya conectados no se actualizan solos, porque la carpeta va dentro del archivo de configuración de cada agente y cambiarla en MMCelt no la cambia allí. Cuando eso pasa, el agente deja de aparecer como conectado, se avisa de que su configuración apunta a otra carpeta, se muestra a cuál y aparece el botón «Actualizar la carpeta», que vuelve a registrarlo con la carpeta que estás viendo y deja una copia de seguridad. La configuración de otro programa nunca se reescribe sola.

Al pulsar Conectar, MMCelt identifica además la carpeta mediante el archivo .mmcelt/configuracion.json. Su UUID se crea una sola vez y viaja con la carpeta. Si la mueves, el programa detecta que la raíz cambió, y el gesto explícito de volver a conectarla confirma la ubicación nueva sin cambiar la identidad.

Cuando mmcelt_create_mindmap o mmcelt_sync_ai_progress escriben dentro de un proyecto ya identificado, dejan el archivo .mmcelt/devolucion.json. Ese recibo indica qué agente entregó qué mapa, si lo generó o lo modificó y a qué proyecto pertenece. Es neutral: no distingue proveedores ni contiene credenciales. Lo pueden consultar la persona y el agente, pero la aplicación no lo usa como autorización para aceptar el contenido del mapa: solo lo lee, mientras el mapa está bloqueado, para saber si el agente ha marcado su entrega como trabajo terminado. Un proyecto antiguo sigue funcionando sin ese archivo hasta que se vuelve a conectar.

Después de conectar hay que reiniciar el agente, porque ninguno vuelve a leer su configuración mientras está en marcha.

No necesita instalar nada más

Lo que se registra en el agente es el propio ejecutable de MMCelt, que lleva el servidor integrado y se lanza con el argumento --mcp-server, es decir, mmcelt.exe --mcp-server en Windows.

Esto importa si le pasas la aplicación a otra persona: le funcionará aunque no tenga ninguna otra herramienta instalada. Si puede ejecutar MMCelt, puede conectarlo con su agente.

Precauciones al modificar la configuración

Los archivos que se tocan pertenecen a otros programas, así que se hace una copia de seguridad con marca de tiempo antes de modificarlos, se conserva todo lo demás (los otros servidores MCP y el resto de ajustes) y, si un archivo tiene contenido ilegible, se deja intacto y se avisa del problema.

Herramientas del servidor MCP

El servidor ofrece seis herramientas. mmcelt_workspace_info declara la carpeta autorizada y lista los mapas que contiene. mmcelt_create_mindmap crea un mapa y su Markdown acompañante. mmcelt_read_mindmap lee la jerarquía, las notas, las dudas y las conexiones. mmcelt_get_human_feedback obtiene las correcciones exigidas por el usuario. mmcelt_sync_ai_progress desarrolla un mapa que ya existe: avance, rol, etiquetas y conexiones cruzadas. mmcelt_export_ai_markdown convierte un archivo .mmcelt en Markdown.

Caso de uso: Claude Code trabaja en el código y va marcando tareas como completadas en el mapa, mientras el usuario ve el avance en tiempo real en MMCelt y puede corregir el rumbo marcando nodos.

Desarrollar un mapa hecho por una persona. El agente puede ampliar un mapa del usuario sin sustituirlo. Añade nodos con su rol y sus etiquetas y traza conexiones cruzadas entre ramas distintas, que es lo que hace falta para dibujar un bucle. No borra ni mueve nada de lo que hay, sus etiquetas se suman a las del usuario en lugar de reemplazarlas, todo lo que añade queda marcado como Generado por IA, no puede marcar nada como aprobado y no vuelve a enlazar un camino que el usuario haya descartado. Los detalles del contrato están en el documento servicios.md de la carpeta documentacion/infraestructura.

Nodos con el mismo título. Los títulos se repiten, porque dos nodos Sí y dos nodos No son lo normal en un diagrama de decisiones, así que un nodo se señala por su identificador. La lectura devuelve el id de cada nodo, y cada conexión con sus dos extremos por identificador y por título. La escritura acepta los campos id, parent_id, from_id y to_id, y comprueba que el título que los acompañe corresponda al mismo nodo. Un título único sigue bastando; uno repetido hace que se rechace la llamada entera sin escribir nada y se enumeren los candidatos, en vez de elegir uno a ciegas.

Contrato autodescriptivo. Los mapas guardados declaran los campos schema_version y generated_by. Los archivos anteriores a esa cabecera siguen siendo compatibles y la reciben al siguiente guardado. El Markdown para IA publica además las seis herramientas MCP y los valores canónicos que debe devolver el modelo, de modo que no tiene que adivinar el formato ni editar directamente el archivo .mmcelt.

Límites de seguridad

El servidor solo escribe dentro del espacio de trabajo definido en la variable de entorno MMCELT_WORKSPACE, y conserva una copia antes de sobrescribir un archivo. La única excepción es mmcelt_sync_ai_progress sobre un mapa que solo ha escrito la IA, que se actualiza una y otra vez mientras avanza el trabajo y llenaría la carpeta de copias.

Cada operación tiene además su propia lista de extensiones: un mapa se escribe como .mmcelt o .json, y un documento como .md. Así el exportador de Markdown no puede escribir encima de un mapa y convertirlo en texto, lo que permitiría saltarse la protección que se describe a continuación.

No se puede crear un mapa nuevo encima de otro que ya contenga criterio humano, es decir, correcciones exigidas, descartes o aprobaciones. Sobre un borrador que escribió la propia IA sí se puede: lo que se protege es tu revisión, no la existencia del archivo.

Si esa variable no está definida, el servidor no opera con ningún archivo: responde con un error de seguridad y no hace nada. MMCelt la escribe en la configuración del agente al conectarlo desde «Conectar MMCelt con mis IAs...», en el menú «Inteligencia Artificial»; si falta, hay que volver a conectarlo desde ahí.

Cada copia lleva una marca de tiempo en el nombre, como proyecto.mmcelt.20260823-101530.bak, así que las sucesivas copias conviven en lugar de pisarse.

Es el componente con más privilegios del sistema, porque da a un agente autónomo la capacidad de escribir en el disco. No hace falta un atacante para que haya daño; basta con que el modelo se equivoque de ruta.


Búsqueda de nodos

El buscador está en lo alto del panel de la derecha, siempre visible. Se escribe parte de un título o de una etiqueta y debajo aparecen los nodos que coinciden. Al pulsar uno queda seleccionado, el inspector pasa a mostrarlo y la vista se desplaza hasta él sin cambiar el zoom.

La comparación ignora mayúsculas y tildes, así que escribir diseno encuentra Diseño: exigir la tilde exacta convertiría la búsqueda en un examen de ortografía.

Las notas quedan fuera a propósito. Son párrafos largos, y buscar dentro de ellos devolvería medio mapa para cualquier palabra común, que es lo mismo que no devolver nada útil. Los resultados se limitan a 50 y salen en el orden en que se leen en el mapa, de la raíz hacia abajo y rama por rama, que es como los reconoce quien está buscando.

El recorrido no es recursivo y lleva registro de los nodos visitados, de modo que un archivo con un ciclo entre nodos no puede dejar la búsqueda en un bucle infinito ni colgar el programa mientras alguien teclea.


Deshacer y rehacer

Ctrl+Z deshace y Ctrl+Y rehace; Ctrl+Mayús+Z también rehace, que es la forma habitual en macOS y en buena parte de Linux. Las dos acciones están al principio del menú «Edición» y aparecen desactivadas cuando no hay nada que deshacer o rehacer.

Se guardan cincuenta pasos. Cada paso es una copia del mapa entero, no una orden reversible: una copia no puede equivocarse al desandar el camino, y en un programa cuyo cometido es no perder el trabajo del usuario esa garantía vale más que la memoria que se ahorraría de otro modo.

Los cambios seguidos cuentan como uno. Arrastrar un nodo modifica el mapa en cada fotograma, y sin agruparlos un arrastre de dos segundos dejaría más de cien pasos. Lo que ocurre a menos de medio segundo del cambio anterior se funde con él, así que un arrastre completo, o una ráfaga de tecleo, se deshace de una vez.

Hay tres detalles que evitan sorpresas. Abrir otro mapa olvida el historial del anterior, porque deshacer traería de vuelta un mapa que no es el que está abierto y lo pisaría. Deshacer deja una copia de recuperación, igual que cualquier otra sustitución del mapa, porque lo que se deshace vive en memoria y un cierre inesperado se lo llevaría. Y si el nodo seleccionado no existe en el estado al que se vuelve, la selección pasa a la idea central, para que el inspector no se quede en blanco ni los atajos actúen sobre un nodo ausente.

Deshacer no toca el archivo en disco: cambia lo que hay en pantalla, y guardar sigue siendo una decisión aparte.


Idioma de la aplicación

El idioma se elige en el menú «Ver y Diseño», en «Idioma / Language:». Hay seis idiomas: español, inglés, francés, alemán, ruso y chino simplificado. La elección se recuerda entre sesiones, junto al tema y la escala.

La primera vez, el idioma lo pone el sistema

Solo en el primer arranque, cuando todavía no hay preferencias guardadas, MMCelt se abre en el idioma en que esté configurado el sistema operativo. En cuanto el usuario elige un idioma en el menú, manda su elección y el sistema deja de consultarse. A quien ya tiene preferencias guardadas no se le cambia nada.

La forma de averiguarlo depende del sistema. Windows no publica el idioma en ninguna variable de entorno, así que se le pregunta con la función GetUserDefaultLocaleName, que devuelve la misma etiqueta que muestra la configuración regional, por ejemplo es-ES. Linux y macOS lo indican en las variables POSIX, que se consultan por orden de precedencia: LC_ALL, LC_MESSAGES y LANG, con valores como es_ES.UTF-8.

Se admiten las dos notaciones, con su codificación y su modificador si los traen. Se compara la subetiqueta principal entera y nunca un prefijo, porque et (estonio) empieza igual que es y confundirlos abriría el programa en un idioma ajeno.

Conviene conocer dos decisiones. Si el sistema está en un idioma que MMCelt no habla, se usa el castellano, que es el idioma en que se escribió el programa; no se elige un sustituto por proximidad. Y el chino tradicional no se sirve como simplificado: MMCelt solo habla chino simplificado, y mostrárselo a alguien de Taiwán, Hong Kong o Macao no sería traducirle, sino darle un texto que no es el suyo, así que esos sistemas también reciben el castellano.

Está traducido todo lo que el usuario ve, sin excepciones: la barra de menús con sus ayudas emergentes y sus atajos, los avisos de la barra de estado, el inspector entero con los nombres de estados, prioridades y roles, las ventanas modales, los diálogos del sistema para abrir y guardar y los tipos de conexión cruzada. Esos textos están recogidos en src/textos.rs, con una traducción por idioma para cada uno; cada idioma es una lista exhaustiva, así que el compilador rechaza un texto nuevo que no tenga sus seis traducciones.

La ayuda interna también está traducida: sus 24 temas, cada uno con tres piezas (el rótulo del selector, el resumen de la galleta y la guía completa), suman 72 piezas por idioma. Las guías son documentos extensos y no van en src/textos.rs, sino en assets/ayuda, con una carpeta por idioma y un archivo por tema (144 archivos en total), y las incrusta src/ui/ayuda_textos.rs.

Soporte tipográfico para chino incluido en el programa

Para que el chino simplificado y los demás caracteres chinos, japoneses y coreanos se lean con nitidez en Windows, Linux y macOS sin depender de que el usuario tenga instaladas fuentes asiáticas, el ejecutable incluye un subconjunto optimizado de la fuente Noto Sans SC, en assets/fuentes/NotoSansSC-subconjunto.ttf, de unos 1,06 MB. La prueba automática todos_los_caracteres_cjk_del_programa_tienen_glifo_en_la_fuente_embebida comprueba sobre la tabla cmap de la fuente que cada carácter usado en los menús, los diálogos y las guías tiene su glifo, y que además está cubierto el nivel 1 del estándar GB2312 para las notas del usuario.

Lo mismo pasa con los símbolos. Las fuentes de egui no tienen muchos de los que usan los menús, los roles y la ayuda, como 🤖, 🧭, 🛑 o la flecha «→» de las rutas de menú, y se dibujaban como recuadros vacíos. El ejecutable incluye Noto Emoji en blanco y negro, con sus 1.489 símbolos, en assets/fuentes/NotoEmoji-subconjunto.ttf (unos 860 KB), y un recorte de Noto Sans Math con los tres signos que faltaban, en assets/fuentes/NotoSansMath-subconjunto.ttf. También se ven los emojis que escriba la persona o la IA en un nodo. La prueba todo_lo_que_se_muestra_tiene_dibujo_en_las_fuentes_del_programa lee la tabla cmap de todas las fuentes del programa y falla si algún texto del código o de la ayuda usa un carácter sin dibujo.

El documento que se entrega a la IA también va en el idioma del usuario

El Markdown que genera la exportación para IA, y el que el servidor MCP escribe en el disco en cada informe de progreso, sale en el idioma que tenga puesto la interfaz: las meta-instrucciones del principio, sus nueve encabezados de sección numerados, las instrucciones dirigidas al modelo, la matriz de relaciones, el bloque de control humano y los tres prompts de acción sugeridos.

El servidor MCP funciona sin ventana, porque lo lanza el agente de IA por su cuenta, así que lee el idioma del archivo de preferencias una sola vez por proceso.

Los datos del usuario no cambian con el idioma

Cambiar de idioma actualiza al instante todos los rótulos de la interfaz, el inspector, los menús y las plantillas de exportación a Markdown. Sin embargo, los mapas existentes del usuario nunca se modifican de forma retroactiva: el título, las notas y las etiquetas que el usuario escribió en sus nodos se conservan intactos.

Del mismo modo, el mapa de bienvenida y las plantillas de arquitectura se crean ya traducidos al idioma activo en el momento de su creación, incluidas sus etiquetas de ejemplo, que en español son tech y objetivo y en los demás idiomas tienen su propia traducción. Desde ese instante pasan a ser datos del usuario, con la misma garantía.

Lo que no cambia nunca es el contenido de los archivos: los nombres de los campos del formato .mmcelt, las claves del protocolo MCP y las de la configuración de los agentes. Son formato, no texto para leer, y traducirlos rompería los mapas guardados y la conexión con los agentes. Dos pruebas automáticas lo vigilan con los literales escritos a mano a propósito, para que un renombrado masivo no pueda cambiar a la vez el código y la prueba que lo comprueba.


Tamaño de la interfaz

Toda la interfaz, con sus letras, botones, paneles y márgenes, se puede escalar entre el 50 % y el 300 %. Ctrl más + la agranda, Ctrl más - la reduce y Ctrl más 0 la devuelve al tamaño normal. En el menú «Ver y Diseño», en Tamaño de la interfaz, se puede elegir entre cinco tamaños predefinidos. El ajuste se recuerda al reiniciar, junto con el tema visual.

No confundir con el zoom del mapa

Son cosas distintas y es fácil mezclarlas. La rueda del ratón acerca o aleja el mapa, y los textos de la interfaz no cambian. Ctrl más + y Ctrl más - agrandan o reducen la interfaz: letras, botones y paneles. Si lo que ves pequeño son los menús y los botones, lo que necesitas es lo segundo.

Tamaños predefinidos

El 80 % es compacto y deja más contenido en pantalla. El 100 % es el tamaño nativo del sistema. El 125 % está pensado para pantallas grandes o vista cansada. El 150 %, para pantallas 4K o distancia media. El 200 %, para televisores y pantallas que se ven desde lejos.

Qué se ajusta solo y qué no

En el primer arranque se propone una escala según la resolución del monitor: una pantalla 4K sin escalado del sistema se agranda automáticamente, porque ahí la interfaz sale diminuta para cualquiera.

Pero hay un límite: ningún programa puede saber a qué distancia te sientas. Un televisor de 42 pulgadas a tres metros le presenta al sistema operativo exactamente los mismos datos que un monitor de 24 pulgadas a medio metro, con la misma resolución y la misma escala, y sin embargo necesitan tamaños de letra muy distintos. Por eso la aplicación no intenta adivinarlo: propone un punto de partida razonable y recuerda lo que tú decidas. Lo ajustas una vez y ya está.


Saber qué versión estás usando

La versión aparece en el título de la ventana y en la barra de estado, junto con la fecha de compilación.

Para ver el detalle completo, abre el menú «Ayuda» y elige «Acerca de MMCelt (versión y compilación)», o pulsa la versión en la barra de estado. Se muestran la versión, la fecha, el commit, la rama y, lo más útil, la ruta del ejecutable que se está ejecutando. Sin abrir el programa, la orden mmcelt --version muestra la misma información en la terminal.

Por qué no basta el número de versión

El número de versión no cambia entre compilaciones. Dice qué versión pretende ser el programa, no qué binario concreto has abierto. Es la pregunta que surge cuando hay varias copias en el disco, como la de target/release, una portable en un USB y otra en una carpeta de trabajo: todas se llaman igual y todas dicen lo mismo. La fecha, el commit y la ruta sí las distinguen.

Si el binario se compiló con cambios sin confirmar en el repositorio, se avisa, porque en ese caso no corresponde exactamente a ningún commit.

Aviso de versión nueva

Al arrancar, MMCelt pregunta a GitHub cuál es la última versión publicada. Si es más nueva que la que se está ejecutando, la barra superior muestra el aviso «Nueva versión disponible:» con su número, como enlace a la página de esa versión. No descarga ni instala nada: abrir el enlace y actualizar es decisión del usuario.

Las versiones se comparan como números, de modo que la 0.12.0 es posterior a la 0.9.9, y solo se avisa de una estrictamente mayor. Una etiqueta que no tenga la forma X.Y.Z no provoca aviso. La consulta se hace en un hilo aparte, así que la ventana nunca espera por ella. Sin conexión, con el límite de peticiones de GitHub agotado o ante una respuesta extraña, no hay aviso ni error visible, y el fallo queda en el registro. La comprobación se desactiva en el menú «Ver y Diseño», con la opción «Comprobar al arrancar si hay una versión nueva», y la elección se recuerda. Desactivada, el programa no se conecta a internet al arrancar.

Caso de uso: quien tiene una copia portable o la instaló hace tiempo se entera de que hay una versión nueva sin tener que ir a mirar el repositorio.


Qué ocurre al importar un mapa de la IA

Importar sustituye el mapa abierto. No se mezcla con lo que hubiera: lo que llega del modelo ocupa su lugar.

Antes de sustituirlo se escribe una copia de recuperación del mapa anterior, en la misma carpeta que usa el guardado automático. Se ofrece al arrancar el programa, igual que la de un cierre inesperado, así que para volver atrás hay que cerrar y abrir MMCelt.

En el JSON de la IA viajan los títulos, las notas y la jerarquía; las rutas de archivo (campo file_path); las etiquetas, los roles, los estados y las prioridades; las conexiones cruzadas y su tipo, y el estado de revisión con las correcciones exigidas. Al importar se pierden las posiciones colocadas a mano y el plegado de los nodos.

La distinción es deliberada: el modelo devuelve contenido y estructura, no presentación. El mapa importado se recoloca con la disposición automática.


Temas visuales y ayuda

Hay tres temas, en el menú «Ver y Diseño», en Tema Visual, con paletas completas y colores por rama que se reparten de forma cíclica. El tema elegido se recuerda entre sesiones.

El tema Oscuro, que es el predeterminado, usa un azul pizarra; el lienzo, el panel y los nodos son tres tonos escalonados, de forma que las capas se separan sin bordes marcados, y es el adecuado para sesiones largas y luz baja. El tema Claro imita el papel cálido, con fondo marfil, nodos blancos que flotan sobre él y textos en gris tinta, y conviene con luz de día o pantallas muy brillantes. El tema Alto contraste usa negro, bordes blancos sólidos y colores saturados, para problemas de visión o reflejos en la pantalla.

El fondo del tema claro no es blanco a propósito. Con el lienzo blanco, los nodos, que también son blancos, no tendrían nada sobre lo que destacar, y una pantalla de blanco puro cansa antes la vista. El marfil deja que los nodos se lean como fichas apoyadas sobre una mesa.

El tema viste toda la ventana

La paleta no se queda en el lienzo: la barra de menús, el panel lateral, la barra de estado, los botones, los desplegables, los campos de texto y los cuadros de diálogo se pintan con los mismos colores. Los detalles técnicos están en el apartado El tema viste toda la ventana del documento arquitectura.md de la carpeta documentacion.

Contraste comprobado

Las tres paletas están sujetas a pruebas automáticas que miden el contraste según la norma de accesibilidad WCAG 2.1: el texto sobre cada superficie supera una relación de 4,5 a 1, y los colores de rama, las líneas y los bordes superan una relación de 3 a 1. Un ajuste estético que deje algo ilegible hace fallar las pruebas.

Sistema de ayuda

El sistema de ayuda tiene dos niveles: galletas informativas junto a los controles, con un botón +info, y un panel lateral con guías extensas sobre veinticuatro temas, desde los primeros pasos hasta la integración con cada plataforma de IA. Las galletas se pueden reactivar desde el menú «Ayuda», en «Galletas de Ayuda para Principiantes».


El icono

El icono es un mapa mental reducido a lo mínimo: el nodo raíz a la izquierda y dos ramas que se abren hacia la derecha. La de abajo termina en un nodo corriente; la de arriba no termina en un nodo, sino en un destello. Ese destello representa la inteligencia artificial, que es lo que separa a MMCelt de cualquier otro programa de mapas mentales.

Los colores salen de la paleta del tema oscuro. El fondo es el degradado del panel al lienzo, las dos ramas son la primera y la segunda del mapa, y el destello es el ámbar de las conexiones cruzadas.

Dónde aparece

En la barra de título y en la barra de tareas lo pone main.rs, con los píxeles de mmcelt-64.rgba. En el explorador de archivos y en los accesos directos lo pone build.rs, que incrusta mmcelt.ico como recurso del ejecutable. En el Dock y el Finder de macOS lo pone el paquete .app, con mmcelt.icns.

Son mecanismos distintos porque resuelven cosas distintas. El primero pertenece al programa en marcha y funciona en cualquier sistema. El segundo pertenece al archivo .exe y solo interviene al compilar en Windows. El tercero pertenece al paquete de macOS, que exige el formato propio de Apple y no acepta ni .ico ni .png.

Los archivos

Todos están en la carpeta assets/icono. El archivo mmcelt.svg es la fuente editable y el que manda. El archivo mmcelt.ico contiene los tamaños de 16, 24, 32, 48, 64, 128 y 256 píxeles, para el ejecutable. El archivo mmcelt.icns contiene 11 variantes de 16 a 1024 píxeles, para el paquete de macOS. Los archivos mmcelt-256.png, mmcelt-128.png y mmcelt-64.png sirven para documentación y tiendas. El archivo mmcelt-64.rgba contiene los píxeles en crudo que incrusta main.rs.

El archivo .rgba se guarda sin comprimir a propósito: descodificar un PNG obligaría a añadir una dependencia de imágenes al programa, y son solo 16 KB.

El icono de macOS y su guion propio

La herramienta con la que Apple construye un archivo .icns, iconutil, solo existe en macOS. Para no depender de ella y poder mantener el icono confirmado en el repositorio, se construye con el guion de Python documentacion/infraestructura/generar-icono-macos.py, que dibuja la geometría del SVG y la empaqueta en el formato de Apple. Se ejecuta desde la raíz del repositorio con la orden python documentacion/infraestructura/generar-icono-macos.py.

Los píxeles se dibujan de nuevo, no se amplían desde los PNG. El mayor de ellos mide 256 píxeles y macOS pide 1024; ampliarlo daría un icono borroso justo en el tamaño más visible, el de la ventana Obtener información.

La integración continua comprueba en cada cambio, en una máquina macOS real, que iconutil descompone el archivo sin errores, porque un archivo construido según una especificación puede ser correcto sobre el papel y no abrirse en el sistema.

Si cambias el diseño, edita el SVG y regenera el resto. El guion del archivo .icns lleva la geometría copiada del SVG en constantes con nombre, así que hay que actualizarla con él. Si el archivo .rgba deja de medir 64 por 64 píxeles, la prueba el_icono_de_la_ventana_tiene_el_tamano_declarado lo detecta.


Qué está comprobado en macOS y qué no

El soporte de macOS se mide en cada cambio en la integración continua, en una máquina macos-latest con Apple Silicon.

Está comprobado que el programa compila sin avisos, con cargo clippy --all-targets --all-features -- -D warnings, y que pasan todas las pruebas, con cargo test --all-features, incluidas las comprobaciones de enlaces propias de cada sistema. También está comprobado que la ventana se abre: se lanza el binario de publicación y se exige que el proceso siga vivo pasados veinte segundos, porque si eframe no consiguiera un contexto de OpenGL moriría en el acto. Se sube además una captura de la ventana como artefacto del flujo, para poder ver cómo se muestra. Y se comprueba el icono del paquete: iconutil descompone mmcelt.icns en sus 11 variantes y sips confirma que la mayor mide realmente 1024 por 1024 píxeles.

Que la ventana se abra confirma también que OpenGL sigue respondiendo en Apple Silicon, donde Apple lo mantiene solo como capa de compatibilidad ya declarada obsoleta.

Hay cosas que todavía no están comprobadas y que no conviene dar por hechas. La integración continua mira pero no toca: no hay clics, ni escritura, ni diálogos de archivo, ni atajos con Cmd, y nadie ha manejado todavía la ventana en un Mac. No hay paquete .app; lo que se ejecuta es el binario suelto, y sin el archivo Info.plist no está activada la opción NSHighResolutionCapable, así que no se sabe cómo se ve en una pantalla Retina. Y el aviso de Gatekeeper no está resuelto: sin certificado de Apple, macOS pone en cuarentena lo que se descarga y dice que está dañado. Se puede abrir con clic derecho y Abrir, o quitando el atributo con la orden xattr -d com.apple.quarantine.

Por eso las descargas publicadas siguen siendo solo de Windows y Linux.


Resumen de atajos

Tab o Insertar crean un nodo hijo. Enter crea un nodo hermano. Espacio o F2 editan el título, igual que un doble clic sobre el nodo. Supr o Retroceso eliminan el nodo y su descendencia. Ctrl+S guarda. Ctrl+E exporta el Markdown para IA. Ctrl+Z deshace, y Ctrl+Y o Ctrl+Mayús+Z rehacen.

La tecla Inicio encuadra el mapa entero en la ventana, solo con el teclado libre.

Ctrl+F lleva el cursor al buscador con su texto seleccionado.

Ctrl más + y Ctrl más - agrandan o reducen la interfaz, es decir, las letras y los botones, y Ctrl más 0 la devuelve al tamaño normal. La rueda del ratón acerca o aleja el mapa. Arrastrar con el botón central o el derecho desplaza la vista.
