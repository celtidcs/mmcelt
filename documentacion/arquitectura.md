Arquitectura de MMCelt

Este documento describe cómo está organizado el código, por qué se tomaron las decisiones técnicas
relevantes y qué invariantes deben respetarse al modificarlo. Está pensado para quien descargue el
proyecto y quiera entenderlo, corregirlo o ampliarlo.

Visión general

MMCelt es un único ejecutable escrito en Rust que puede funcionar de dos maneras. Abierto sin
argumentos, es la aplicación de escritorio de mapas mentales, dibujada con egui y eframe. Abierto
con el argumento --mcp-server, el mismo binario se convierte en un servidor del Model Context
Protocol que habla JSON-RPC 2.0 por la entrada y la salida estándar y ofrece seis herramientas a los
agentes de IA: mmcelt_workspace_info, mmcelt_create_mindmap, mmcelt_read_mindmap,
mmcelt_export_ai_markdown, mmcelt_get_human_feedback y mmcelt_sync_ai_progress.

Dentro de la aplicación de escritorio, la capa de presentación (src/ui) recoge la entrada del
usuario y se apoya en el cálculo de posiciones (src/layout.rs) y en el modelo de datos con su
validación (src/model). El modelo se guarda y se carga a través de la persistencia (src/storage.rs),
y la traducción entre el mapa y el texto para IA vive en src/ai_bridge.rs y src/ai_export.rs. Todas
estas capas propagan sus fallos con el tipo común de src/error.rs.

La aplicación y el servidor MCP no se comunican directamente: se entienden a través de los archivos
.mmcelt del disco. El servidor escribe un archivo y la aplicación lo lee, o al revés. Esto los
mantiene desacoplados, pero implica que el formato del archivo es el contrato entre ambos y los dos
deben respetarlo.

Cada archivo declara schema_version: 1 por separado de generated_by, que identifica la aplicación y
la versión que lo escribió. La ausencia de ambos campos identifica un archivo anterior a su
introducción: serde aporta los valores vigentes al abrirlo, de modo que el archivo se actualiza sin
perder compatibilidad en el siguiente guardado.

Estructura de carpetas

En la raíz del repositorio, build.rs captura la versión, la fecha y el commit al compilar e incrusta
el icono en el ejecutable de Windows, y Cargo.toml declara las dependencias y el perfil de
publicación.

La carpeta src contiene la aplicación. Su punto de entrada es main.rs, que elige entre abrir la
interfaz o arrancar el servidor MCP. Los módulos principales son estos.

El modelo de datos está en src/model, con mod.rs como fachada. Dentro, tipos.rs define las
estructuras del mapa; validacion.rs, las invariantes estructurales y los límites; jerarquia.rs, las
operaciones de hacer hijo o hermano sin crear ciclos; conexiones.rs, qué conexión cruzada admite la
interfaz; clasificacion.rs, el estado, la prioridad, el control humano y el rol de cada nodo;
filtro_de_busqueda.rs, qué se pide al buscador; y metadatos_externos.rs, los metadatos de MMCelt que
se incrustan en los formatos externos. Las plantillas y los ejemplos localizados de mapas de partida
están en plantillas.rs, y escaner_repositorio.rs convierte la estructura acotada de una carpeta en un
mapa.

La persistencia la forman storage.rs, con la lectura y la escritura de archivos; respaldo.rs, la
única forma de copiar un archivo antes de sobrescribirlo; proyectos.rs, los casos de uso que separan
la interfaz del disco; servicios_aplicacion.rs, que compone esos servicios para la interfaz;
guardado_como.rs, la decisión sobre el destino de «Guardar como»; traslado.rs, la reubicación de
mapas; autoguardado.rs, la copia periódica y la recuperación tras un cierre inesperado;
cierre_con_cambios.rs, la decisión de preguntar al cerrar con cambios sin guardar y lo que pasa
tras cada respuesta; y carga_en_segundo_plano.rs, que lee sin detener la interfaz los mapas modificados por otro proceso.

Los formatos de intercambio son ai_bridge.rs, que importa respuestas de IA; ai_export.rs, que
exporta el Markdown para IA; exportacion_opml.rs y exportacion_freemind.rs, que convierten a OPML y
al .mm de FreeMind y Freeplane; y sus módulos de orquestación exportacion_opml_comando.rs y
exportacion_freemind_comando.rs, que enlazan el diálogo de archivo, la conversión y el estado.

El servidor MCP está en src/mcp_server. Su fachada es mod.rs; protocolo.rs es la frontera entre el
texto JSON-RPC que llega por la entrada estándar y el resto del servidor; catalogo.rs guarda los
nombres, descripciones y esquemas públicos de las herramientas; argumentos.rs define los contratos
de entrada; actualizaciones.rs convierte los cambios pedidos en operaciones sobre el modelo;
operaciones.rs ejecuta cada herramienta; seguridad.rs delimita el espacio de trabajo y las
escrituras; y error.rs define el error único de las operaciones MCP.

La relación con los agentes de IA la llevan conectores.rs, que detecta los agentes instalados y
registra MMCelt en ellos, apoyado en la biblioteca deteccion-agentes, que encuentra las cuentas de
cada uno; proyecto_trabajo.rs, que da una identidad estable y un límite seguro a la
carpeta de un proyecto; configuracion_agente_proyecto.rs, que conserva el idioma del documento y el
consentimiento de cada fuente de instrucciones; sesiones_agentes.rs, que compone el prompt neutral y
persiste el expediente de cada envío; lanzador_agentes.rs, que traduce una sesión en una orden de
consola y abre la terminal; envio_agente.rs, el caso de uso completo de entregar un mapa a un
agente; envio_agente_comando.rs, su orquestación fuera del objeto de la aplicación;
devolucion_agentes.rs, el recibo de vuelta que deja el servidor MCP; vigilante.rs, que avisa cuando
un agente cambia el mapa en disco; vigilancia_mapa.rs, que decide qué hacer con cada cambio; y
bloqueo_por_agente.rs, que bloquea el mapa desde que se confirma un envío hasta que el agente
declara terminado su trabajo o la persona recupera el control.

El resto de módulos de src son aplicacion.rs, con el estado raíz de la aplicación;
persistencia_mapa_comando.rs, con el ciclo de guardado del mapa activo; historial.rs, que implementa
deshacer y rehacer mediante instantáneas del mapa; busqueda.rs, que encuentra nodos por título o
etiqueta; preferencias.rs, con los ajustes que se conservan entre sesiones; version.rs, con la
identificación de la compilación; aviso_de_version.rs, el único módulo que habla con la biblioteca
aviso-version-github, que lanza en un hilo de trabajo la comprobación de versión nueva y registra sus fallos; consola_windows.rs, que permite imprimir --version sin mostrar una
consola al abrir la interfaz; theme.rs, con las paletas de color y el estilo de toda la ventana;
textos.rs, con todo lo que la aplicación le dice al usuario en sus seis idiomas; error.rs, con el
manejo centralizado de errores; audit_checks.rs, con las pruebas de regresión y las barreras
estructurales; y arnes_interfaz.rs, que ejecuta la interfaz en las pruebas sin abrir una ventana.

La capa de presentación está en src/ui. El ciclo de vida de eframe y la carga de las fuentes
embebidas están en src/aplicacion.rs. canvas.rs dibuja el lienzo 2D infinito; sidebar.rs, el inspector de
propiedades; toolbar.rs, la barra de herramientas; ai_modal.rs, las ventanas modales, incluida
«Acerca de»; conexiones_modal.rs, la ventana de conexión con agentes; proyecto_ia_modal.rs, la
ventana de proyecto e instrucciones; sesion_agente_modal.rs, la vista previa supervisada de una
sesión de agente; dialogos.rs, los diálogos nativos del sistema; dialogo_de_cierre.rs, la ventana
que pregunta si se guarda antes de cerrar; estado_version_nueva.rs, la
comprobación de versión de cada arranque y su aviso; suelta_de_nodo.rs, el menú que aparece al
soltar un nodo encima de otro; menu_contextual_nodo.rs, el menú del clic derecho sobre un nodo;
help_system.rs, el sistema de ayuda contextual y sus temas; ayuda_textos.rs, las piezas de la ayuda
por idioma, que incrusta las guías; y estado_agentes.rs y estado_persistencia.rs, los grupos de
estado de la comunicación con agentes y del archivo abierto.

La carpeta assets contiene ayuda, con las guías de la ayuda en Markdown, una carpeta por idioma (de,
en, es, fr, ru y zh) y una guía por tema; fuentes, con los subconjuntos embebidos de Noto Sans SC para
los caracteres chinos, japoneses y coreanos y de Noto Emoji y Noto Sans Math para los símbolos; icono, con el icono en varios formatos y tamaños (svg,
ico, png y rgba); y capturas, con las capturas de pantalla de la documentación. La carpeta skills
contiene la habilidad mmcelt-mindmap para Claude Code y Antigravity, y la carpeta documentacion
contiene esta documentación, con las guías de integración por plataforma en
documentacion/integraciones y las de entorno en documentacion/infraestructura.

Capas y responsabilidades

La capa de presentación, src/ui, dibuja, captura la entrada y muestra los errores; no debe contener
lógica de negocio ni tocar el disco directamente. La capa de dominio, src/model, mantiene la
estructura del mapa, sus invariantes y sus plantillas; no debe saber nada de egui ni de rutas de
archivo. La capa de geometría, src/layout.rs, calcula posiciones y no modifica el contenido de los
nodos. La persistencia, src/storage.rs, lee y escribe archivos y no abre ventanas ni interactúa con
el usuario. La traducción para IA, en src/ai_bridge.rs y src/ai_export.rs, convierte entre el modelo
y el texto para IA y no toca el disco. Los errores, en src/error.rs, se definen y se registran ahí,
pero los mensajes visibles los redacta la interfaz. Los textos, en src/textos.rs, guardan cada rótulo
y aviso con su traducción, y nunca contienen claves de protocolo o de archivo, que son contrato y no
texto para leer.

La regla de dependencia va hacia dentro: la interfaz conoce el modelo, pero el modelo no conoce la
interfaz. La única excepción es error.rs, del que dependen todas las capas por ser transversal.

Por qué los diálogos están en ui/dialogos.rs y no en storage.rs

Escribir archivos y abrir diálogos nativos no son la misma responsabilidad. Un diálogo no persiste
nada: abre una ventana modal y pregunta al usuario por una ruta, lo que es interacción y no acceso a
datos. Mantenerlos separados deja storage.rs como lógica pura sobre rutas y contenidos, que por eso
se puede probar sin abrir ninguna ventana, como hacen las pruebas de audit_checks.rs.

Por qué storage.rs no llama a std::fs

Es la aplicación de la inversión de dependencias al único sitio del programa donde de verdad hace
falta. storage.rs decide qué se guarda, cómo se serializa y cuándo un archivo es válido, que es
política; abrir un archivo y escribir bytes es un detalle. Si la política llamara directamente a
std::fs, cualquier prueba de la escritura atómica o de la carga con validación tendría que crear
archivos de verdad en una carpeta temporal, lo que obliga a serializar las pruebas y deja basura en
el disco si una falla a medias.

Por eso storage.rs depende de un contrato, SistemaDeArchivos, con cuatro operaciones: leer, leer con
límite, escribir de forma atómica y crear una carpeta. La implementación real, DiscoLocal, es la que
usa el programa en marcha, y las funciones guardar_proyecto_en_archivo, cargar_proyecto_de_archivo y
escribir_de_forma_atomica son envoltorios suyos que el resto de módulos usa sin conocer el contrato.

No es una interfaz decorativa. La prueba
el_mapa_va_y_vuelve_por_un_sistema_de_archivos_que_no_es_el_disco pone otra implementación, que
guarda en memoria, escribe un mapa, lo recupera y comprueba que no se ha creado ningún archivo y que
la validación estructural sigue en el camino de carga.

La escritura real crea el temporal con exclusividad y lo identifica mediante el PID más un contador
atómico del proceso. Dos hilos pueden guardar el mismo destino a la vez sin compartir temporal:
ambas operaciones terminan y el renombrado deja una de las dos versiones completa, nunca una mezcla.
Si queda un temporal antiguo, la creación exclusiva avanza a otro nombre en vez de reutilizarlo.

La interfaz no toca el disco

Todo acceso a disco que necesita la interfaz pasa por storage, que expone leer_bytes,
huella_del_archivo, crear_carpeta y escribir_de_forma_atomica. En particular, ninguna escritura usa
fs::write, que vacía el destino antes de escribir: todas las escrituras del programa son atómicas.

Cómo está organizado el estado de la aplicación

AplicacionMapaMental, en src/aplicacion.rs, es lo único que sobrevive de un fotograma al siguiente.
egui dibuja en modo inmediato y reconstruye la interfaz entera entre treinta y sesenta veces por
segundo, así que nada de lo que se pinta puede guardar estado por su cuenta. El módulo es hermano de
ui, no descendiente suyo: dibujar una pantalla no concede acceso privilegiado a las invariantes de la
aplicación.

La raíz tiene seis campos privados, cada uno agrupado por una razón de cambio. EstadoDelMapa reúne
el documento, el nodo seleccionado y el historial, porque son la unidad sobre la que operan las
acciones de edición, deshacer y rehacer. EstadoDelLienzo reúne la cámara, el índice espacial y el
título y la conexión que se están editando, que son gestos o geometría transitorios y no forman parte
del documento. EstadoDePresentacion reúne ventanas, tema, preferencias, búsqueda y borradores
visibles, que solo existen para reconstruir la presentación entre fotogramas. EstadoAgentes reúne la
vista previa de sesión, la fase del envío, el trabajador, la detección, la carpeta autorizada y el
bloqueo del mapa, que
cambian juntos como parte del recorrido «Enviar a…». EstadoPersistencia reúne la ruta, el
autoguardado, la recuperación, el vigilante, el cargador, la huella y el conflicto externo, que
forman el ciclo de vida del archivo activo. ServiciosAplicacion contiene los contratos ya compuestos
para abrir, guardar y exportar: la interfaz pide una operación, pero no elige el adaptador que la
ejecuta.

Los agregados del mapa, del lienzo y de la presentación tienen todos sus campos privados, y ui solo
recibe consultas u operaciones con nombre. La selección comprueba que el nodo exista, el historial no
se presta para escribir y el tema solo se consulta. El compilador lo hace cumplir: escribir la
selección desde ui/canvas.rs se rechaza con el error E0616, fabricar el agregado con E0451 y
sustituirlo con E0277. Otros datos transitorios, como el borrador de una ventana o el gesto de
arrastre, se prestan de forma mutable porque egui necesita editarlos directamente; esa parte es una
frontera de organización, no una garantía completa de invariante. EstadoAgentes y
EstadoPersistencia viven dentro de ui y tienen campos pub(crate).

Hay dos detalles que no son evidentes. El primero es que las banderas de VentanasAbiertas conservan
el prefijo modal_, y no es redundancia: varias pruebas de audit_checks.rs analizan el texto del
fuente buscando esos nombres, porque no hay forma de preguntar por los campos declarados sin macros,
y una de ellas impide que un modal abierto deje pasar la tecla Supr hasta el mapa de detrás. Un
renombrado que las dejara sin encontrar nada no las rompería, sino que las dejaría pasando siempre.
El segundo es que dos implementaciones de Default están escritas a mano y explican por qué: las
galletas de ayuda arrancan encendidas, y el zoom empieza en 1.0 y no en el 0.0 que daría f32, que no
sería un mapa pequeño sino un lienzo en blanco y una división por cero en las conversiones de
coordenadas.

Decisión central: validar en la frontera

Es la decisión de arquitectura más importante del proyecto.

El problema

Todo el código que recorre el mapa (layout, eliminar_nodo, profundidad_del_nodo y la capa de dibujo)
asume tres cosas: que el mapa es un árbol de verdad, sin ciclos; que el nodo raíz existe; y que todo
identificador referenciado apunta a un nodo real.

Los datos entran por tres puertas, todas ajenas al control del programa.
storage::cargar_proyecto_de_archivo recibe un archivo del disco, que puede estar corrupto, editado a
mano o llegado por correo o por una carpeta sincronizada en la nube. ai_bridge::importar_de_texto_de_ia
recibe texto pegado desde un modelo de IA, es decir, de un tercero. model::desde_escaneo_de_carpeta
recibe el sistema de archivos, que puede tener enlaces circulares.

Cuando una de esas suposiciones falla, el resultado no es un error, sino la congelación de la
aplicación por un bucle infinito o su muerte por desbordamiento de pila. Con panic = "abort" en el
perfil de publicación, morir significa cerrarse de golpe sin guardar nada.

La solución

Proyecto::validar_estructura() comprueba las diez garantías que el resto del código da por supuestas
y se invoca en las tres fronteras. Un mapa que no la supere no entra. Entre esas garantías están los
dos extremos de cada conexión cruzada: el lienzo conserva comprobaciones defensivas al dibujar, pero
una conexión colgante se rechaza antes de que distintos consumidores, como el lienzo y la exportación
a Mermaid, puedan interpretar de forma diferente el mismo archivo.

En storage.rs, la carga deserializa el contenido con serde_json::from_str, convierte cualquier fallo
en AppError::formato con el contexto «al interpretar el mapa mental» y, antes de devolver el
proyecto, llama a validar_estructura(), porque nada de lo que hay debajo tolera un árbol inválido.

La validación es iterativa, no recursiva: usa una pila explícita en el montículo. Si fuera
recursiva, moriría por desbordamiento al intentar validar precisamente los archivos que pretende
rechazar. Además, los errores identifican el nodo concreto: FalloEstructural incluye el
identificador del nodo que incumple la garantía, de modo que el usuario o una IA puedan repararlo.

La invariante que debe mantenerse es esta: toda ruta nueva por la que un Proyecto entre en el
programa desde el exterior debe llamar a validar_estructura() antes de devolverlo.

Aun así, los recorridos de layout.rs conservan su propia defensa, con marca de visitados y límite de
profundidad, porque el usuario puede modificar el mapa después de cargarlo y la disposición se
recalcula sobre ese estado ya editado.

Tolerancia a lo que escriben los modelos de IA

Las cinco enumeraciones del modelo (EstadoNodo, PrioridadNodo, RolNodo, TipoRelacion y
EstadoRevision) se deserializan a través de String, con el atributo #[serde(from = "String", into =
"String")].

Los archivos .mmcelt los puede generar un modelo de IA a través del servidor MCP, y un modelo
escribe con frecuencia «In Progress» donde el formato espera «EnProgreso». Con enumeraciones cerradas
de serde, un solo valor desconocido en un solo nodo haría fracasar la deserialización del archivo
entero, y el error no diría qué nodo lo causa. Por eso cada enumeración tiene un desde_texto() que
acepta sinónimos en español e inglés y degrada a la variante por defecto ante lo desconocido, de modo
que el resto del mapa se conserva. La serialización sigue emitiendo el nombre canónico, así que el
formato en disco no cambia y los archivos antiguos siguen siendo válidos.

El atributo #[serde(other)] no sirve aquí: solo funciona en enumeraciones etiquetadas interna o
adyacentemente, no en las que se serializan como una cadena simple.

Existe una única tabla de equivalencias por enumeración. interpretar_estado y sus compañeras, en
ai_bridge.rs, delegan en EstadoNodo::desde_texto y sus equivalentes, de modo que la carga de
archivos, el importador de IA y el servidor MCP comparten las mismas tablas.

Manejo de errores

Todas las capas internas devuelven AppResult<T>, es decir, Result<T, AppError>. El modelo, la
persistencia y el importador propagan el AppError con el operador ? hasta la interfaz, donde
AplicacionMapaMental::reportar_error lo registra en mmcelt-errores.log y lo muestra en la barra de
estado. Ese es el único punto donde un error se convierte en texto visible; las capas internas se
limitan a propagarlo.

AppError distingue la causa en variantes separadas, entre ellas Lectura, Escritura, Formato,
EstructuraInvalida, ImportacionIa, CarpetaProyectoInvalida, IdentidadProyectoDistinta,
RutaFueraDelProyecto, EntradaDemasiadoGrande, EntradaInvalida, ConfirmacionCaducada, Vigilancia y
ComprobacionDeVersion. Así quien recibe el error puede distinguir, por ejemplo, un archivo
inexistente de un JSON corrupto.

Cada mensaje incluye qué puede hacer el usuario. Por ejemplo: «El mapa mental está dañado y no se
puede abrir: el mapa contiene un ciclo: se llega dos veces al nodo 7f3a…. Se ha impedido su carga
para no bloquear la aplicación.»

El registro en mmcelt-errores.log se hace en la medida de lo posible: si no se puede escribir, se
omite sin propagar el fallo. Registrar un error nunca debe provocar otro.

Recorridos iterativos

layout.rs, el modelo y ui/canvas.rs recorren el árbol sin recursión, con pila explícita y marca de
visitados. Afecta a seis funciones: calcular_altura_de_la_rama, disponer_rama, arcos_requeridos,
recoger_descendientes, profundidad_del_nodo y dibujar_conexiones_de_la_rama. La razón es directa: una
versión recursiva mataría el proceso ante un mapa con un ciclo o con una rama muy larga. El mismo
criterio sigue busqueda.rs.

calcular_altura_de_la_rama es la más interesante. Necesita medir los hijos antes que el padre,
porque la altura de un nodo depende de la suma de las de su descendencia, lo que es un recorrido en
postorden. La versión iterativa lo consigue apilando cada nodo dos veces, con una marca que indica
si sus hijos ya se encolaron. En la primera visita, el nodo se vuelve a apilar con la marca activada
y, encima, se apilan sus hijos; en la segunda visita, todos los hijos tienen ya su altura registrada
y se puede calcular la del padre.

Seguridad del servidor MCP

El servidor MCP es el componente de mayor privilegio efectivo del sistema: concede a un agente de IA
autónomo la capacidad de escribir archivos en el disco del usuario. Aquí no hace falta un atacante
para que haya daño: basta una ruta alucinada por el modelo para destruir un archivo del usuario.

La primera restricción es que la carpeta es obligatoria. Sin la variable MMCELT_WORKSPACE definida,
espacio_de_trabajo() devuelve error y no se opera con ningún archivo. Si la ausencia de la variable
hiciera que el espacio pasara a ser la carpeta desde la que se arrancó el proceso, la ausencia de
configuración se convertiría en ausencia de límite, que es lo contrario de lo que la ventana de
conexión le promete al usuario.

La segunda es que el espacio de trabajo está acotado. validar_ruta() encadena seis comprobaciones,
cada una en su propia función: ruta vacía; presencia de .. mirando las dos barras, porque en Linux
la invertida no separa; dos puntos dentro de un nombre, que en NTFS abren flujos alternativos;
nombres que acaban en punto o espacio, ingobernables en Windows; contención dentro de la raíz una vez
resuelta con Path::canonicalize; y extensión admitida para ese uso.

La tercera es que las extensiones dependen de la operación. Un mapa se escribe como .mmcelt o .json,
y un documento como .md. No hay una lista común, porque con ella el exportador de Markdown podría
escribir sobre un mapa y convertirlo en texto, rodeando la protección de los mapas que no ha escrito
la IA. Al leer no se exige extensión, porque el agente puede querer abrir un archivo con cualquier
nombre.

La cuarta es que no hay sobrescritura silenciosa. Antes de pisar un archivo, se copia con marca de
tiempo, para que las sucesivas copias no se pisen entre sí: con un nombre fijo, la segunda escritura
del agente machacaría la copia del trabajo del usuario. La copia vive en respaldo.rs, un único sitio
que usan tanto el servidor como el registro de agentes. Hay una sola excepción,
mmcelt_sync_ai_progress sobre un mapa que solo ha escrito la IA, que se sincroniza continuamente y
llenaría la carpeta; sobre un mapa con trabajo de la persona sí deja copia. Está declarada en el
código, en PoliticaDeCopia, y no se puede pedir desde fuera. Ningún argumento de las herramientas
permite al agente eximirse de dejar copia, porque quien llama es el propio modelo.

La quinta es que lo que no ha escrito la IA no se pisa. Crear un mapa encima de un archivo cuyos
nodos no sean todos trabajo de la IA se rechaza. Sin esto, la protección de la sincronización, que
no revive descartados ni firma aprobaciones, se rodearía con solo llamar a la herramienta de al lado.
Se mira el estado de revisión de cada nodo, y no si hay correcciones, porque un mapa escrito a mano
en la aplicación no tiene ninguna marca de supervisión y es justo el caso que hay que proteger.

Tres detalles de la implementación no son evidentes. Path::starts_with compara por componentes y no
por texto, así que no sufre el fallo clásico de la comparación textual, en la que /datos_privados
empezaría por /datos. El filtro de .. no es redundante con canonicalize, porque este último solo
resuelve rutas que ya existen, y al crear un mapa lo normal es que todavía no exista. Y la carpeta
interna .mmcelt de cada proyecto es un límite propio: ninguna ruta suministrada a una herramienta MCP
puede entrar en ella, aunque esté dentro del espacio de trabajo; solo los recorridos internos que
crean identidad, sesiones y recibos escriben ahí.

El ejecutable como servidor MCP

mmcelt --mcp-server convierte el propio binario en servidor del Model Context Protocol. La aplicación
y el servidor son el mismo archivo, distinguidos por un argumento. El saludo declara como versión
del servidor la versión real de MMCelt y negocia la versión MCP 2024-11-05. Como solo se implementa
esa versión, una petición con otra desconocida recibe la única compatible, en vez de que el servidor
finja entender un contrato futuro.

El servidor vive dentro del ejecutable por dos motivos. El primero es la portabilidad: un servidor
escrito aparte en otro lenguaje exigiría tener instalado su intérprete, y quien recibe MMCelt como
programa portable se quedaría sin conexión. El segundo es que al reutilizar model, storage y
ai_export, el servidor no puede divergir de lo que hace la aplicación: comparte la misma validación
del modelo y la misma exportación, sin copias que mantener sincronizadas.

Las pruebas del servidor ejercitan procesar_peticion directamente, sin lanzar ningún proceso: se le
pasa una petición JSON-RPC y se comprueba la respuesta, que es todo lo que hace el bucle principal.
Son más rápidas que arrancar el servidor y no dependen de nada externo.

Una sola implementación de cada cosa

ai_export.rs es el único sitio donde se genera el documento Markdown para IA, y lo usan tanto la
aplicación como el servidor MCP. Las secciones del documento exportado se numeran del 1 al 9, y esos
números no deben cambiar, porque los prompts sugeridos y las guías se apoyan en ellos.

El formato .mmcelt

Es JSON con los nodos en un mapa plano indexado por UUID; la jerarquía se expresa mediante las
listas children y los campos parent_id. En el nivel superior hay un id, un title, los textos
creator_vision y project_goals, el root_id con el identificador de la raíz, el objeto nodes, la lista
connections y el layout_mode, por ejemplo HorizontalTree. Cada entrada de nodes, indexada por su
UUID, repite su id y lleva parent_id (nulo en la raíz), la lista children, el title, el status (por
ejemplo Idea), la priority (por ejemplo Media), el role (por ejemplo IdeaCentral), el review_status
(por ejemplo SinMarcar), la posición pos como par de números y el indicador collapsed. Un nodo que la IA
acaba de crear y el usuario todavía no ha abierto lleva además nuevo_de_la_ia a true; cuando vale false
no se escribe.

Por qué un mapa plano y no anidado

Permite que las conexiones cruzadas, que relacionan nodos de ramas distintas, referencien cualquier
nodo por su identificador, algo imposible en una estructura anidada.

Tiene una consecuencia de seguridad que conviene recordar: el límite de anidamiento de serde_json,
de 128 niveles, no protege este formato. Un documento de profundidad sintáctica 3 puede describir un
árbol lógico de cientos de miles de niveles, y por eso PROFUNDIDAD_MAXIMA debe comprobarse
explícitamente en la validación.

Campos con valor por defecto

file_path, review_status y correction_feedback llevan #[serde(default)], así que los archivos creados
antes de que existieran siguen cargándose. Por el mismo motivo, quitar un campo no rompe los
archivos ya guardados: serde descarta lo que no reconoce.

Elecciones que conviene conocer

El perfil de publicación usa panic = "abort". Reduce el tamaño del binario y mejora el rendimiento,
pero no hay desenrollado de pila: un pánico cierra el proceso de golpe, sin destructores. Se mantiene
deliberadamente, porque lo peligroso no es la opción sino que haya pánicos alcanzables desde un
archivo de entrada, y la validación en la frontera los elimina. Al añadir código conviene
recordarlo: aquí un unwrap() sobre datos externos no produce un mensaje de error, sino la pérdida del
trabajo del usuario.

La interfaz usa egui en modo inmediato: se reconstruye entera en cada fotograma desde el estado
actual, sin árbol de componentes ni estado duplicado en la vista. La consecuencia práctica es que las
funciones de dibujo se ejecutan decenas de veces por segundo y que un nodo puede desaparecer entre
dos fotogramas, así que toda consulta al mapa debe contemplar la ausencia con if let Some(...) y
nunca indexar directamente.

La aplicación no habla con ningún modelo de IA: no maneja claves de API ni envía el mapa a ningún
servicio. Es una decisión de diseño que elimina de raíz la filtración de credenciales, la inyección
en peticiones y la exfiltración de datos. Su única conexión de red es la comprobación opcional de
versión nueva, descrita más abajo, y los únicos procesos que lanza son las consolas de agente que el
usuario confirma en la vista previa de una sesión. Si algún día se añade una integración directa con
un modelo, todo el análisis de seguridad del proyecto cambia y hay que rehacerlo.

Autoguardado: qué es y dónde escribe

El módulo autoguardado.rs guarda cada dos minutos, o con el intervalo que haya elegido el usuario,
una copia del proyecto si ha cambiado, y al arrancar ofrece recuperarla si la sesión anterior terminó
mal.

Autoguardar no es guardar: es una red de seguridad, y de ahí salen sus dos reglas. La primera es que
no interrumpe: un diálogo modal cada dos minutos sería insufrible, así que el autoguardado no puede
preguntar dónde escribir y necesita una ubicación conocida de antemano. La segunda es que no escribe
donde sorprenda: nunca toca el archivo del usuario ni deja artefactos junto a él.

La copia va a la carpeta de datos del usuario y no a la carpeta de la aplicación porque, si el
binario está en Program Files o en /usr/local/bin, el proceso no tiene permiso de escritura allí y
el autoguardado fallaría en silencio justo cuando más falta hace; además, nadie busca sus datos en la
carpeta del programa. En Windows la ubicación es %APPDATA%\MMCelt\recuperacion; en macOS,
~/Library/Application Support/MMCelt/recuperacion; y en Linux, $XDG_DATA_HOME/mmcelt/recuperacion o,
si esa variable no existe, ~/.local/share/mmcelt/recuperacion. La variable de entorno MMCELT_DATOS
permite reubicarlo, lo que sirve para instalaciones portables y para aislar las pruebas.

Para no reescribir la copia cuando nada ha cambiado, se compara una huella del contenido. Entran en
ella el título, los metadatos y el autor del proyecto; el título, las notas, las etiquetas, el
estado, la prioridad y el rol de cada nodo; y la estructura de children y las conexiones. Quedan
fuera las posiciones de los nodos, el campo updated_at, el nivel de zoom y el desplazamiento. Las
posiciones cambian con solo recalcular la disposición y updated_at cambia sola; si entraran, la
huella diferiría siempre y el autoguardado escribiría en cada intervalo aunque el usuario solo
estuviera leyendo.

Un detalle fácil de pasar por alto: egui solo repinta cuando hay actividad. Sin
ctx.request_repaint_after(…), una ventana inactiva dejaría de recibir fotogramas y el temporizador
no se dispararía, justo en el caso en que el autoguardado más falta hace, con el usuario ausente.

La recuperación no se aplica sola. Al arrancar se busca la copia, pero solo se ofrece: el usuario
pudo haber cerrado a propósito sin guardar, y devolverle sin preguntar lo que descartó sería peor que
no ofrecer nada. Una copia que no supere validar_estructura se descarta en silencio, porque restaurar
un mapa con un ciclo reintroduciría el problema que el resto del programa rechaza.

Proyecto::carpeta_proyecto_sugerida() deduce la carpeta del proyecto de código buscando el prefijo
común de las rutas file_path de los nodos, que llevan los mapas del escáner de repositorios y los que
genera una IA sobre un código concreto. Se usa solo para proponer el destino en el diálogo de
guardar, nunca para escribir sin preguntar, porque hacerlo podría colar archivos dentro de un
repositorio ajeno. Solo se sugiere si la carpeta existe de verdad, ya que una IA puede inventarse
rutas, y si el prefijo común tiene más de un componente, porque la raíz de una unidad o del sistema
no es la carpeta de ningún proyecto. En el flujo MCP este problema no se plantea: el agente indica la
ruta explícitamente y el mapa se escribe en la carpeta del proyecto, acotada por MMCELT_WORKSPACE.

Escala de la interfaz y preferencias

El módulo preferencias.rs conserva entre sesiones los ajustes de presentación, entre ellos la escala
de la interfaz y el tema visual.

egui trae de serie el escalado global de la interfaz con Ctrl más, Ctrl menos y Ctrl cero, mediante
la opción Options::zoom_with_keyboard, activa por defecto. Lo que egui no hace es guardar el
zoom_factor entre sesiones, así que sin MMCelt el ajuste volvería a 1,0 en cada arranque; para quien
trabaja en un televisor, eso convierte una función útil en una molestia diaria.

AplicacionMapaMental::sincronizar_escala_interfaz se ejecuta al principio de cada fotograma y hace
algo distinto según el momento. En el primer fotograma aplica la escala guardada, lo que no puede
hacerse en el constructor porque el contexto aún no conoce el monitor. En el primer arranque propone
una escala según la resolución y marca la sugerencia como mostrada. En los fotogramas siguientes
detecta si el factor ya no coincide con el guardado, señal de que el usuario usó los atajos, y
persiste el valor nuevo. Este último punto es la clave: egui aplica los atajos por su cuenta al final
de cada fotograma, así que la aplicación no los intercepta, sino que observa el resultado. Aplicar
la escala guardada en cada fotograma pisaría los cambios del usuario.

Hay un límite en lo que se puede deducir. La resolución del monitor y la escala de pantalla del
sistema son observables; el tamaño físico en pulgadas no lo es de forma fiable; y la distancia de
visionado, que es lo que de verdad manda, no es observable en absoluto. Un televisor de 42 pulgadas
a tres metros y un monitor de 24 a medio metro presentan al sistema exactamente los mismos datos y
necesitan tamaños de letra muy distintos. Por eso escala_sugerida es deliberadamente conservadora:
solo corrige el caso inequívoco, una resolución muy alta con el sistema sin escalar, donde la
interfaz sale diminuta para cualquiera, y respeta el escalado que ya aplique el sistema operativo
para no acumular factores. Todo lo demás queda en manos del usuario, y se recuerda.

Unas preferencias ilegibles o con valores absurdos no impiden arrancar: se parte de los valores por
defecto y la escala se recorta al rango admitido. Sin ese recorte, un archivo editado a mano con una
escala de 500 dejaría la interfaz inutilizable e impediría llegar al menú para corregirlo. Guardar
las preferencias se hace en la medida de lo posible, porque no poder recordar una preferencia es una
molestia y no un error que merezca interrumpir al usuario.

Conexión con agentes de IA

conectores.rs detecta los agentes instalados y registra MMCelt en ellos, y ui/conexiones_modal.rs lo
presenta como una ventana con una ficha por cada cuenta de cada agente. La conexión se hace desde la propia aplicación y
no con guiones de terminal, porque quien recibe MMCelt de un conocido no va a abrir una consola, y
porque la detección ocurre al abrir la ventana, de modo que un agente instalado después aparece solo
sin tener que acordarse de repetir ningún paso.

Cada cuenta guarda su configuración en un sitio distinto. Claude Code la guarda en ~/.claude.json
para la cuenta por defecto, o en el .claude.json de la carpeta de la cuenta que señala
CLAUDE_CONFIG_DIR, en formato JSON. Codex CLI la guarda en el config.toml de la carpeta de la cuenta,
que por defecto es ~/.codex y si no la que señala CODEX_HOME, en formato TOML. Antigravity CLI la
guarda en ~/.gemini/config/mcp_config.json, en formato JSON, y solo tiene la cuenta por defecto. La
búsqueda de cuentas la hace la biblioteca deteccion-agentes, incluida en el repositorio en
bibliotecas/deteccion-agentes: mira la carpeta por defecto de cada herramienta, las carpetas de la
carpeta personal que se llaman como ella seguida de algo más y la que señale su variable, y añade
las que la persona haya dado de alta a mano. Un detalle cuesta tiempo si no se sabe: el .claude.json
de Claude Code guarda además el historial y las preferencias del usuario.

Codex CLI es el único que no usa JSON. Añadir una dependencia de TOML para una operación tan acotada,
que consiste en comprobar si una sección existe y, si no, añadirla al final, no compensa, así que se
trabaja sobre el texto y el resto del archivo no se toca.

Estos archivos pertenecen a otros programas y suelen contener ajustes que costó trabajo dejar bien.
Por eso, antes de modificarlos se hace una copia de seguridad con marca de tiempo; se conserva todo
lo demás, incluidos otros servidores MCP y el resto de claves; y un archivo ilegible se deja intacto
y se informa, en lugar de sobrescribirlo, porque podría ser la configuración completa del usuario,
dañada por otra causa y aún recuperable a mano.

Composición y consentimiento de las instrucciones

sesiones_agentes.rs es el único módulo que compone una sesión. Recibe cuatro bloques tipados
(contrato, reglas comunes, contexto y encargo), valida tamaños y caracteres de control, incorpora
solo las fuentes elegidas y devuelve un PromptSesionCompuesto inmutable con su huella SHA-256. La
ventana de proyecto, el modal de confirmación y el repositorio de sesiones consumen ese mismo valor:
ninguno vuelve a concatenar cadenas.

configuracion_agente_proyecto.rs conserva en .mmcelt/configuracion-agente.json el idioma del
documento y el consentimiento durable de cada fuente nativa de instrucciones. La aceptación guarda
nombre, contenido y huella; toda lectura vuelve a resolver la ruta mediante ContextoProyecto, por lo
que un enlace simbólico exterior no puede ganar confianza. La escritura usa el reemplazo atómico
común, y una configuración corrupta produce un error tipado.

ui/proyecto_ia_modal.rs solo dibuja borradores y devuelve acciones semánticas. Cambiar el idioma,
guardar reglas o aceptar una fuente se ejecuta después del callback de egui, fuera del préstamo
visual. El selector genera CambiarIdiomaDocumento y persiste solo esa preferencia de inmediato;
GuardarReglas es la única acción que escribe el perfil común. Ambos controles ocupan un pie con
altura reservada, fuera del contenido de las cuatro pestañas, que conserva su propio desplazamiento.
La misma separación se mantiene en ui/sesion_agente_modal.rs: el botón confirma una
ConfirmacionSesion con la revisión del mapa y las huellas externas, y antes de encolar el trabajo
aplicacion.rs relee esas entradas; cualquier diferencia falla cerrada con ConfirmacionCaducada.

Las dos ventanas conservan el contexto con reglas distintas y deliberadas. «Proyecto e
instrucciones» es un editor del proyecto activo: si se abre otro mapa mientras está visible,
reconstruye sus borradores para la nueva raíz, y todo guardado o consentimiento se resuelve desde la
misma raiz_proyecto que enseña en pantalla. La ventana de envío, en cambio, captura un
ContextoProyecto al prepararse y lo mantiene hasta confirmar o cancelar, porque representa una sesión
concreta ya compuesta y no un editor que deba seguir los cambios de mapa. Esta diferencia evita tanto
escribir o consentir en una carpeta distinta de la mostrada como alterar sin aviso una sesión que la
persona ya estaba revisando.

Sesión supervisada y consola neutral

sesiones_agentes.rs persiste el prompt aprobado en .mmcelt/sesiones/<sesión>/inicio.md, junto a
metadatos sin credenciales. lanzador_agentes.rs traduce únicamente Claude Code, Codex CLI y
Antigravity CLI (agy) a una orden con programa, argumentos y carpeta de trabajo separados. El prompt
completo no viaja en los argumentos del proceso: solo una referencia relativa al expediente.

La interfaz conserva un EditorSesionAgente en memoria, y elegir o cancelar no guarda, exporta ni
vigila nada. Al confirmar, la secuencia es guardar el mapa, exportar el _AI.md, persistir el
expediente, lanzar la CLI y empezar a vigilar. El ejecutor es inyectable para comprobar ese límite
sin abrir ventanas, y la prueba el_envio_es_un_caso_de_uso_independiente_de_la_interfaz sustituye
persistencia, expedientes y lanzador para comprobar el orden y el corte ante cada error.

El adaptador de OpenAI no confunde superficies. Codex CLI, la aplicación Codex y su extensión pueden
compartir ~/.codex/config.toml cuando se ejecutan en el mismo equipo, pero una conversación de
ChatGPT web no lee ese archivo local. Por eso el núcleo solo modela capacidades comprobables,
ejecutable y conectado_por_mcp, y no deduce acceso al disco a partir del nombre «ChatGPT» o «Codex».
La interfaz puede abrir la CLI y preparar un expediente, pero no inyecta mensajes en una tarea
gráfica ya abierta.

Un destino que solo se conecta por MCP sigue el mismo camino sin el paso de proceso: deja el
expediente en estado Preparado y activa la vigilancia. Si una carpeta carece todavía de identidad, la
vista previa usa un ContextoProyecto efímero sin escribir nada, y la identidad durable nace
únicamente al confirmar.

Identidad del proyecto y recibo de vuelta

proyecto_trabajo.rs separa la carpeta física de su identidad lógica. La conexión explícita crea
.mmcelt/configuracion.json mediante la persistencia atómica común; abrirla otra vez conserva el
UUID, y encontrarla bajo otra raíz produce CargaProyecto::Trasladado en vez de aceptar el cambio sin
avisar. ContextoProyecto es la única puerta para resolver rutas relativas y comprueba también los
enlaces existentes antes de autorizar una escritura.

devolucion_agentes.rs modela el contrato neutral ReciboDevolucion. El servidor MCP lo prepara antes
de guardar y lo publica después del mapa en .mmcelt/devolucion.json; la creación y la sincronización
comparten validación, formato y escritura atómica. Los proyectos sin configuración mantienen la
compatibilidad anterior y no generan recibo hasta una conexión explícita. El recibo es trazabilidad
durable para la persona y para el cliente MCP, no una credencial de autorización: la aplicación
observa y valida el mapa real, porque un recibo anterior podría seguir en disco después de una
edición distinta. ReciboDevolucion::validar comprueba que identidad, raíz y ruta sean coherentes
antes de guardar esa trazabilidad; que devuelva una ruta no concede permiso para escribir, no valida
el contenido del mapa y no dispara una recarga. Su único efecto en la aplicación es el campo
trabajo_terminado: cuando una entrega nueva sobre el mapa bloqueado lo trae a true, porque el agente
mandó work_finished, bloqueo_por_agente.rs anota la huella del mapa en disco y levanta el bloqueo en
cuanto se carga exactamente ese contenido.

El mapa que vuelve solo

El servidor MCP permite que un agente escriba el mapa, y vigilante.rs hace que la ventana abierta se
entere.

La vigilancia combina dos vías. La biblioteca notify usa el aviso del propio sistema operativo y un
hilo llama a ctx.request_repaint() cuando llega algo. Como algún aviso puede perderse, otro hilo
comprueba además cada dos segundos la fecha y el tamaño del archivo vigilado, una operación de
microsegundos que no despierta la interfaz si nada ha cambiado. Mientras el mapa está bloqueado, al
mismo ritmo se lee el recibo de entrega para saber si el agente ha terminado.

Se vigila la carpeta y no el archivo. Un guardado atómico escribe un temporal y lo renombra encima
del destino, así que quien vigile el archivo se pierde exactamente el evento que le importa.

Antes de tocar nada se aplican tres filtros, en este orden. Primero, si el cambio lo ha escrito la
propia aplicación: se compara una huella del contenido con la de la última escritura propia. Se
compara el contenido y no la marca de tiempo, porque la resolución del reloj del sistema de archivos
no es infinita y el agente puede escribir en el mismo instante que el autoguardado; sin este filtro,
el autoguardado dispararía su propia recarga cada dos minutos. Segundo, si el archivo está entero: si
no deserializa como JSON válido y no supera validar_estructura, se descarta en silencio y se espera
al siguiente aviso, porque un agente puede escribir en varios pasos y un error en pantalla por un
archivo a medias sería ruido. Tercero, si el usuario tiene trabajo sin guardar: en ese caso el mapa
en memoria no se toca, se avisa y decide la persona; si no lo tiene, se recarga, dejando antes una
copia de recuperación.

La lectura del archivo modificado la hace carga_en_segundo_plano.rs para no detener la interfaz, y la
orquestación vive en vigilancia_mapa.rs, que recibe únicamente EstadoPersistencia: inicia o detiene
el observador, solicita las lecturas en segundo plano y clasifica cada resultado como ignorado,
retenido por cambios locales o apto para sustituir el mapa. No recibe &mut AplicacionMapaMental. La
aplicación conserva solo el adaptador de presentación: guarda la recuperación antes de sustituir,
invalida el índice espacial, reinicia el historial, ajusta la selección y muestra el estado
correspondiente. Así una decisión de archivo no se mezcla con el objeto raíz y la clasificación se
puede probar sin conceder al módulo acceso a toda la interfaz.

Por qué no hay motor de fusión

No se fusiona nodo a nodo cuando el usuario y la IA tocan el mismo mapa, y la razón es de producto,
no técnica: el flujo es por turnos. Cuando se manda el mapa con «Enviar a…», el usuario está mirando
la terminal del modelo, que es donde le da las instrucciones, no editando el mapa, y el turno lo
garantiza el bloqueo de bloqueo_por_agente.rs, que impide editar hasta que el agente termina o la
persona recupera el control. Eso reduce la pieza más cara posible, un algoritmo de fusión que sería
el código más difícil de probar, a la comprobación de un booleano. Conviene tenerlo presente antes de intentar «mejorarlo».

Composición de los servicios de persistencia

proyectos.rs expone tres contratos pequeños: RepositorioMapas, PreparadorCarpetas y
RepositorioDocumentos. servicios_aplicacion.rs los compone una sola vez para la interfaz y es el
único punto de esa capa que selecciona RepositorioLocal. El trabajador de envío hace una segunda
composición deliberada, porque posee sus dependencias y las mueve a otro hilo; compartir el
contenedor de la interfaz introduciría acoplamiento y requisitos de sincronización sin aportar
comportamiento.

El módulo servicios_aplicacion es hijo privado de aplicacion, y su RepositorioLocal también es
privado. La interfaz no puede construirlo, invocar su fábrica ni crearle un alias: el compilador lo
rechaza con los errores E0603 o E0624. Esta barrera del compilador sustituye a cualquier búsqueda
textual, que podría esquivarse con un alias o con una pantalla nueva.

El trabajador en segundo plano llama a las mismas funciones seguras de storage que el adaptador
local. Hoy no duplica ninguna política, pero son dos puntos que habrá que cambiar juntos si el
guardado local adquiere una operación adicional.

La orquestación del envío a agentes de IA vive en envio_agente_comando.rs, que separa de
AplicacionMapaMental la preparación de borradores, el diálogo de selección de rutas, la confirmación
de envíos y la atención de las respuestas en segundo plano. Sus funciones reciben solo las
estructuras acotadas EstadoAgentes, EstadoPersistencia, Proyecto y ServiciosAplicacion, en lugar de
&mut AplicacionMapaMental. La aplicación conserva métodos de fachada para coordinar la interfaz
(preparar_sesion_agente, cancelar_sesion_agente, confirmar_sesion_agente y
exportar_markdown_dialogo) y despacha la reacción del trabajador asíncrono en cada fotograma
mediante procesar_eventos_de_envio. toolbar.rs y sesion_agente_modal.rs se comunican solo a través
de la aplicación, sin acoplarse directamente a envio_agente_comando.

El ciclo de persistencia del mapa activo vive en persistencia_mapa_comando.rs, submódulo privado de
aplicacion. Recibe EstadoPersistencia, Proyecto y, solo para el guardado, ServiciosAplicacion, y
concentra el autoguardado, la recuperación, el registro de la huella y la selección de la carpeta
sugerida sin conocer AplicacionMapaMental. La aplicación conserva los adaptadores de presentación
que abren diálogos, traducen resultados a avisos y aplican el proyecto recuperado.

Estos repartos siguen las responsabilidades, no el tamaño. Un límite de líneas se cumple partiendo
un archivo por la mitad aunque el corte no siga ninguna responsabilidad, así que lo que se cuida es
otra cosa: que cada archivo trate un solo asunto, que ningún tipo acumule métodos hasta hacerlo todo
y que ningún archivo mezcle disco, dibujo, dominio y agentes.

Por qué Windows no abre una consola vacía

El mismo ejecutable sirve como aplicación gráfica, como servidor MCP y para consultar --version. La
compilación distribuida declara el subsistema gráfico de Windows, de modo que al abrir MMCelt desde
el Explorador solo aparece la ventana de la aplicación. Las compilaciones de depuración conservan el
subsistema de consola para que los diagnósticos sigan visibles durante el desarrollo.

Antes de imprimir --version, consola_windows.rs intenta asociarse a la consola del proceso padre. Si
la salida ya es una tubería redirigida, Windows la conserva; si el programa se abrió desde un lugar
sin consola, el intento falla en silencio porque es un caso normal. El modo MCP no usa esta
adaptación: el cliente le entrega directamente sus tuberías de entrada y salida.

Para comprobarlo no basta con leer el código: hay que leer la cabecera PE del ejecutable de
publicación, cuyo campo Subsystem debe valer 2, capturar la salida real de --version y negociar una
sesión MCP completa con el mismo archivo.

El tema viste toda la ventana

ThemeConfig, en theme.rs, define la paleta del mapa, que el lienzo usa al dibujarse. Pero la barra de
menús, el panel lateral, la barra de estado, los botones, los desplegables, los campos de texto y los
cuadros de diálogo no los dibuja la aplicación, sino egui, con el estilo que guarda en su contexto.
Si ese estilo no se configura, egui usa el suyo por omisión, que es oscuro y fijo: el tema claro
saldría partido en dos, con un lienzo claro rodeado de barras oscuras, y el alto contraste perdería
su razón de ser en todo lo que no fuera el mapa.

ThemeConfig::visuals() traduce la paleta a egui::Visuals: fondos de panel y ventana, los cinco
estados de los controles, el color de selección, los bordes, los redondeos y las sombras. Se instala
en dos puntos y solo en esos dos. AplicacionMapaMental::new lo instala al arrancar, antes del primer
fotograma, porque el tema guardado en las preferencias tiene que estar puesto antes de dibujar nada.
AplicacionMapaMental::establecer_tema lo instala al elegir otro tema en el menú, porque cambiar la
paleta del lienzo sin reinstalar el estilo dejaría la ventana a medio pintar. Aplicarlo en cada
fotograma sería más simple de escribir, pero reconstruiría el estilo sesenta veces por segundo para
un dato que cambia cuando el usuario abre un menú; por eso establecer_tema recibe el &egui::Context.

Las paletas se comprueban solas. theme.rs incluye pruebas que calculan la relación de contraste de la
norma WCAG 2.1, con la luminancia relativa sobre canales linealizados, y exigen estos mínimos: 4,5 a
1 para el texto principal y de apoyo sobre cualquier superficie; 4,5 a 1 para el texto atenuado sobre
las cinco superficies donde se dibuja; 4,5 a 1 para los colores de acierto, aviso y peligro sobre los
cinco estados de un control; 3 a 1 para los colores de rama, líneas, bordes y símbolos sobre una
rama; 1,12 a 1 entre lienzo y panel y entre un nodo y el nodo raíz, para que se distingan; y entre
1,05 a 1 y 2,2 a 1 para la retícula contra el lienzo, para que se vea sin competir.

Al texto atenuado se le exige el umbral del texto y no el de un elemento gráfico, aunque el nombre
sugiera lo contrario, porque no es decoración: es la barra de estado, los encabezados de los menús y
los recuentos de las etiquetas. Una prueba aparte comprueba que los tres niveles de texto conservan
su jerarquía, para que subir el contraste del atenuado no acabe igualándolo con el de apoyo. El
límite superior de la retícula impide que un color premultiplicado mal construido la pinte de
blanco, y otra prueba comprueba que ningún color de la paleta lleve transparencia, salvo la sombra,
para que esa misma trampa de la premultiplicación no aparezca en otros colores como
connection_line.

El icono, en dos sitios a la vez

Windows saca el icono de un programa de dos fuentes, y hay que alimentar las dos. La primera es el
icono de la ventana, que el programa fija al arrancar y se ve en la barra de título, la barra de
tareas y el selector de Alt más Tab; main.rs lo pasa como IconData al constructor de la ventana. La
segunda es el recurso del ejecutable, incrustado al compilar, que se ve en el Explorador de archivos,
los accesos directos y el menú de inicio; build.rs incrusta mmcelt.ico con winresource. Poner solo el
primero deja el .exe con el icono blanco genérico, y poner solo el segundo funciona en Windows pero
no en otros sistemas.

Los píxeles van en crudo: main.rs incrusta mmcelt-64.rgba, que son los 16.384 bytes del icono sin
comprimir, y no un PNG. Descodificar un PNG exigiría añadir el paquete image y sus dependencias para
dibujar un icono, mientras que el archivo en crudo no necesita descodificador. El riesgo de guardar
píxeles sin cabecera es que nada dice cuánto miden, y lo cubre la prueba
el_icono_de_la_ventana_tiene_el_tamano_declarado, que comprueba que el archivo ocupa exactamente
ICONO_LADO × ICONO_LADO × 4 bytes y que no es del todo transparente.

winresource no puede tumbar la compilación: si falla al incrustar el recurso, build.rs avisa y sigue,
porque un ejecutable sin dibujo sigue siendo un ejecutable. Es el mismo criterio que build.rs aplica
a los datos de git.

Soporte tipográfico CJK y fuente embebida

egui incluye por omisión una fuente latina ligera que carece de glifos para ideogramas chinos,
japoneses o coreanos. En sistemas sin fuentes adecuadas o en instalaciones portables, esos textos se
verían como cuadros vacíos.

En lugar de incrustar una fuente china completa, que suele superar los 25 MB, MMCelt incluye un
subconjunto de Noto Sans SC, assets/fuentes/NotoSansSC-subconjunto.ttf, de alrededor de 1,06 MB. Cubre
el estándar GB2312 Nivel 1 completo, con 3.755 caracteres chinos simplificados de uso frecuente;
todos los caracteres adicionales que usan las traducciones de la interfaz, el inspector, los modales
y las guías de ayuda en chino; y los símbolos y flechas habituales de la interfaz.

En src/aplicacion.rs, en definiciones_de_fuentes, durante la inicialización de eframe, la fuente se
carga como datos estáticos en FUENTE_CJK_BYTES con include_bytes! y se registra en egui::FontDefinitions como fuente de respaldo en
las familias Proportional y Monospace. Así, los textos en alfabeto latino, cirílico o con símbolos
básicos siguen usando la tipografía estándar de egui, y cuando el motor encuentra un carácter CJK
recurre automáticamente a la fuente embebida, sin configuración del usuario. La prueba
todos_los_caracteres_cjk_del_programa_tienen_glifo_en_la_fuente_embebida inspecciona la tabla cmap de
la fuente en cada ejecución de las pruebas para garantizar que ningún carácter del programa queda sin
glifo.

Con el mismo criterio se añaden dos fuentes de símbolos. assets/fuentes/NotoEmoji-subconjunto.ttf es Noto Emoji en blanco y negro fijada al peso normal, con sus 1.489 símbolos, y assets/fuentes/NotoSansMath-subconjunto.ttf es un recorte de Noto Sans Math con los signos que no tiene ninguna otra fuente, como «→». Las tres fuentes de respaldo se registran en src/aplicacion.rs, en definiciones_de_fuentes, siempre detrás de las de egui y en este orden: china, símbolos y signos. Las tres tienen licencia SIL Open Font License 1.1, recogida en assets/fuentes/OFL.txt. La prueba todo_lo_que_se_muestra_tiene_dibujo_en_las_fuentes_del_programa lee la tabla cmap de todas ellas y falla si un texto del código o de la ayuda usa un carácter sin dibujo; no pregunta a egui porque su has_glyph da por ausente todo lo que cae en la fuente del recuadro de sustitución.

Puertas del compilador para código que deja de usarse

La raíz del binario trata dead_code, unused_imports, unused_variables y unused_mut como errores. En la
compilación de producción se declaran con forbid, de modo que un módulo interior no puede
silenciarlos con un #[allow(...)]. Esto impide conservar una función, variante, importación o
variable solo porque quizá vuelva a resultar útil.

El arnés que genera rustc --test introduce por su cuenta una excepción a dead_code, y Rust rechaza
combinar esa excepción con un forbid incondicional, por lo que durante las pruebas los mismos avisos
quedan en nivel deny. Una excepción local hace fallar cargo check --bin mmcelt con el error E0453.

Esta política no demuestra que una abstracción usada sea necesaria ni que una función tenga una
responsabilidad adecuada; eso se comprueba ejecutando el comportamiento que justifica el código.
Tampoco sustituye a Rustdoc ni a Clippy: una prueba estructural aparte cubre el único hueco conocido,
una línea vacía entre la documentación /// y el campo de una estructura.

Comprobación de versión nueva

Es la única conexión de red de la aplicación. Su lógica vive en la biblioteca
aviso-version-github, incluida en el repositorio en bibliotecas/aviso-version-github y declarada
por ruta en Cargo.toml, para que otros programas la reutilicen. La biblioteca lee
tag_name, compara versiones como números, valida el repositorio y las direcciones, compone el
enlace, define el rasgo ConsultaDeVersion y hace la consulta real con ConsultaHttps, mediante ureq y
rustls y con el proxy del sistema; sus pruebas no necesitan red. MMCelt la conecta
copiando la integración de referencia que trae la propia biblioteca en su carpeta examples. En MMCelt quedan dos piezas. aviso_de_version.rs
es el único módulo que llama a la biblioteca: guarda el nombre con que MMCelt se presenta en la
consulta y el repositorio de fábrica, prepara la consulta real, la lanza en un hilo de trabajo
llamado aviso-de-version y anota los fallos en el registro de errores. ui/estado_version_nueva.rs
la arranca una vez si la opción está activa y recoge el resultado sin esperar, pidiendo un repintado
mientras sigue en curso para que el aviso aparezca en cuanto llega. Las preferencias guardan la
opción y los ajustes, y toman sus valores de fábrica de aviso_de_version.rs.

La respuesta se trata como un dato ajeno. Solo se toma tag_name, y solo si tiene la forma X.Y.Z. El
enlace del aviso se compone con la raíz web y el repositorio de la configuración, y html_url se
ignora, para que una respuesta manipulada no pueda llevar al usuario a otro sitio.

Con la opción desactivada, la consulta no existe: arrancar_si_procede comprueba la opción antes de
llamar a la fábrica de la consulta, así que no llega a crearse nada capaz de salir a la red. Una
prueba lo vigila contando las llamadas.

Ningún fallo es un pánico. Con panic = "abort", un pánico en el hilo cerraría el programa entero, así
que cada fallo vuelve como FalloDeComprobacion, se registra como AppError::ComprobacionDeVersion con
severidad de aviso y no se muestra al usuario.

Es configurable sin recompilar. El repositorio, las raíces de la API y de la web, el tiempo de
espera y el límite de tamaño de la respuesta viven en preferencias.json, en ajustes_de_version, con
valores de fábrica documentados en preferencias.rs. Se validan en la frontera: solo se admite
https:// con un nombre de servidor, y un repositorio con la forma dueño/nombre.

Solo el arranque real consulta. nueva_con_contexto arranca la comprobación, y las construcciones de
prueba, como nueva_limpia, nunca salen a la red.

Pruebas y arnés de interfaz

Todo se ejecuta con cargo test, sin ninguna herramienta externa. El módulo tests de src/main.rs cubre
la jerarquía, el borrado en cascada, la exportación y la importación, además del tamaño del icono.
src/audit_checks.rs reúne las pruebas de regresión y las barreras estructurales: el servidor MCP, los
conectores, el autoguardado, la soberanía de los datos, el aislamiento del perfil, las preferencias,
la identificación de la compilación, la estructura de los módulos y la coherencia entre la ayuda y
las funcionalidades. src/arnes_interfaz.rs es un ejecutor de la interfaz sin ventana: renderiza
fotogramas reales de egui, mide la geometría de los paneles y simula clics de ratón con
hacer_clic_en_texto sin abrir nada en pantalla. El módulo de pruebas de src/theme.rs comprueba el
contraste y la legibilidad de las tres paletas según WCAG 2.1 y que el estilo de egui se construya a
partir del tema.

Cada prueba de audit_checks.rs documenta el defecto que impide, de modo que, si alguna falla, indica
exactamente qué se ha roto y por qué importa. Algunas leen el texto del código fuente o de la
documentación porque lo que vigilan no se puede preguntar en tiempo de ejecución; en ese caso
normalizan el fuente, quitando comentarios y juntando líneas, para que ni comentar un elemento ni un
salto de línea introducido por cargo fmt las dejen en verde por accidente.
