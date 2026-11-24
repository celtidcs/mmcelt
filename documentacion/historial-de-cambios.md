Historial de cambios de MMCelt

Este documento recoge los cambios de cada versión publicada de MMCelt, de la más reciente a la más antigua. Sigue el criterio de Keep a Changelog (https://keepachangelog.com/es-ES/1.1.0/) y el versionado semántico (https://semver.org/lang/es/).

El historial empieza en la 0.7.0, que es la primera versión que se publicó. Las versiones anteriores solo existieron durante el desarrollo y no llegaron a distribuirse; lo que se hizo entonces puede consultarse en los commits del repositorio.

Cómo se numeran las versiones. El último número sube cuando se corrige un defecto sin añadir nada nuevo, y en este proyecto sube con cada arreglo que se confirma, se publique o no; por eso hay números que nunca llegan a una descarga. El número del medio sube cuando aparece una funcionalidad nueva o cambia una pieza de fondo aunque no se vea, como el motor gráfico. El primer número subirá cuando se rompa la compatibilidad hacia fuera: que un mapa .mmcelt guardado siga abriéndose igual, que un agente ya conectado por MCP siga funcionando y que la configuración que MMCelt escribió en un agente siga sirviendo. Mientras el primer número sea 0, ese compromiso no está dado, porque el estándar considera las versiones 0.y.z desarrollo inicial. La 1.0.0 llegará cuando el formato y el protocolo se declaren estables y el programa se haya probado también en macOS. Una prueba del proyecto comprueba que la versión declarada en Cargo.toml tiene su entrada en este historial.

Versión 0.16.0 — 2026-11-24

Lo que se hace mientras el envío a un agente guarda el mapa ya no se da por guardado. Al enviar, MMCelt guarda el mapa en segundo plano tal como estaba en ese momento. Si mientras tanto se editaba un nodo o se abría uno nuevo de la IA, al terminar el guardado ese cambio contaba como guardado aunque el archivo no lo tuviera: al cerrar no se preguntaba y el cambio podía perderse. Ahora solo cuenta como guardado lo que de verdad está en el archivo.

El mapa queda bloqueado mientras un agente trabaja en él. Desde que se confirma el envío no se puede editar, aunque se puede ver, recorrer, buscar y exportar, y la barra inferior lo avisa. El agente marca su última entrega como trabajo terminado (work_finished en las herramientas mmcelt_sync_ai_progress y mmcelt_create_mindmap), y entonces MMCelt carga el mapa final y levanta el bloqueo. Si el agente falla o no avisa, el botón «Recuperar el control» lo quita. Si el envío falla antes de lanzar al agente, el bloqueo se quita solo.

MMCelt encuentra todas las cuentas de cada agente. Si un equipo tiene varias cuentas de Claude Code o de Codex CLI, cada una en su carpeta, MMCelt las busca sin conocer ningún nombre de antemano y muestra una ficha por cuenta; conectar una cuenta escribe en su propia configuración, y al enviar un mapa la consola se abre con esa cuenta. Antes solo veía la cuenta por defecto de cada herramienta, de modo que un agente abierto con otra cuenta no tenía cómo devolver el mapa. Las cuentas que estén en otro sitio se pueden añadir a mano, salvo las de Antigravity CLI, cuya consola no permite elegir cuenta. Si la carpeta elegida ya es una cuenta de esa herramienta, aunque esté escrita de otra forma, MMCelt no la añade y explica por qué. La búsqueda de cuentas usa la versión 0.2.1 de su biblioteca, deteccion-agentes, que aplica estas reglas por sí misma.

Las bibliotecas de base están al día. La interfaz pasa a egui y eframe 0.37 y los diálogos de archivo a rfd 0.18, que ya no depende de las bibliotecas sin mantenimiento que traía. Las fechas pasan de Chrono, que dejó de mantenerse, a Jiff; los mapas y las sesiones guardados con versiones anteriores se siguen abriendo igual. Quedan tres avisos de mantenimiento que llegan por la accesibilidad de Linux y se explican en los defectos conocidos.

El aviso de versión nueva pasa a la versión 0.2.4 de su biblioteca, aviso-version-github. La consulta a GitHub se presenta ahora como MMCelt, con su nombre y su versión en la cabecera User-Agent además de los de la biblioteca. El nombre del servidor que se puede configurar en las preferencias se valida parte por parte, así que nombres mal formados como a..b, que antes se aceptaban, ahora se rechazan y la comprobación no se hace. La comprobación sigue siendo una sola al arrancar, en segundo plano y desactivable en el menú «Ver y Diseño».

Guardar en Windows ya no falla de vez en cuando con «Acceso denegado». Cuando otro programa, normalmente el antivirus, tiene abierto un instante el archivo del mapa, MMCelt espera unos milisegundos y vuelve a intentarlo, hasta un límite de unos 200 ms, en lugar de avisar de que no ha podido guardar.

La descripción de cada agente en la ventana de conexiones y en el menú de la IA sale ahora en el idioma de la interfaz, y no siempre en español. Si falla el guardado de las preferencias al añadir o quitar una cuenta, el aviso lo dice con su propio título. Un recibo de entrega que el agente deja dañado ya no pasa desapercibido: queda anotado en el registro de errores de MMCelt, y el mapa sigue bloqueado hasta que el agente lo rehaga o se pulse «Recuperar el control».

Versión 0.15.0 — 2026-11-19

Lo que ha hecho la IA se ve de un vistazo. Los nodos que crea el usuario, incluidos los de las plantillas y los ejemplos, nacen en un estado nuevo, Sin marcar, que no lleva icono de revisión. Un nodo que la IA acaba de crear, ya sea con un agente por MCP o pegando su respuesta en «Importar de IA», se dibuja con un fondo y un borde propios hasta que el usuario lo selecciona; después recupera su aspecto normal y conserva el icono de Generado por IA. Si la IA cambia un nodo que ya existía y no era suyo, pasa a Pendiente de Revisión, y el reloj de ese estado se pinta en el color de aviso del tema.

Al cerrar la ventana con cambios sin guardar, MMCelt pregunta antes de cerrarse si se guarda el mapa, si se cierra sin guardar o si se cancela el cierre. Un mapa sin archivo, como un escaneo o una importación, también pregunta aunque no se haya tocado; solo se cierran sin preguntar el ejemplo del arranque y un mapa nuevo vacío. Si el mapa todavía no tiene archivo, Guardar abre «Guardar como», y si ahí se cancela, el programa sigue abierto.

Los cambios que hace un agente aparecen siempre en pantalla. Al enviar a un agente un mapa que aún no tenía archivo, MMCelt lo guardaba con el nombre elegido pero seguía vigilando el archivo anterior, de modo que lo que escribía el agente no llegaba a verse hasta volver a abrir el mapa. Ahora vigila el archivo recién guardado y, además de los avisos del sistema, comprueba cada dos segundos si el archivo ha cambiado, por si alguno de esos avisos no llega.

Los símbolos ya no salen como recuadros vacíos. Las fuentes que trae el programa no tenían muchos de los que usan los menús, los roles y la ayuda, como 🤖 en «Generado por IA», 🧭 o la flecha de las rutas de menú. Ahora el ejecutable incluye Noto Emoji en blanco y negro y un recorte de Noto Sans Math, así que esos símbolos, y los emojis que escriba la persona o la IA en un nodo, se ven igual en cualquier equipo.

Versión 0.14.0 — 2026-11-19

Los avisos de error permanecen visibles en la parte inferior hasta pulsar «Cerrar». Identifican la operación y el motivo. Cuando un envío rechaza una ruta por quedar fuera del proyecto, el aviso muestra el archivo solicitado y la carpeta autorizada.

Al soltar un nodo sobre un hermano, «Mover aquí sin tapar» lo coloca detrás del nodo de destino en el orden de los hermanos. El cambio se deshace en un paso con Ctrl+Z.

Las pruebas de conectores retiran sus carpetas temporales al terminar. Las del servidor MCP usan espacios de trabajo aislados y los flujos de integración y publicación fijan Ubuntu 24.04.

Las conexiones cruzadas directas entre un padre y su hijo se dibujan con curvatura propia separada de la línea de la jerarquía, manteniendo su etiqueta visible sin superposiciones.

Los ejecutables publicados ya no llevan dentro las rutas de la máquina que los compiló.

La batería de pruebas exige la creación real de enlaces simbólicos en las comprobaciones de seguridad y añade cobertura para el arrastre libre de nodos con ratón y su restauración con deshacer.

Arreglo de seguridad en las conexiones cifradas. Se actualizan las bibliotecas que cifran las conexiones con GitHub (rustls, rustls-webpki y webpki-roots) a versiones que corrigen seis avisos de seguridad publicados en noviembre de 2026: RUSTSEC-2026-0346, RUSTSEC-2026-0347, RUSTSEC-2026-0348, RUSTSEC-2026-0349, RUSTSEC-2026-0381 y RUSTSEC-2026-0382. MMCelt solo abre una conexión de red, la que comprueba si hay una versión nueva en GitHub, y es la que usa estas bibliotecas. No cambia nada de lo que se ve ni de cómo se usa.

El aviso de versión nueva funciona detrás del proxy de Windows y con GitHub Enterprise Server. En Windows, si no hay ninguna variable de entorno HTTPS_PROXY, HTTP_PROXY o ALL_PROXY, la consulta usa ahora el proxy activado en las opciones de internet del sistema, con sus excepciones; las variables, cuando existen, siguen mandando. Sigue sin interpretar archivos de configuración automática (PAC o WPAD) y sin autenticarse ante el proxy con NTLM o Kerberos. Además, la dirección de la API en las preferencias admite una ruta, como https://servidor/api/v3, que es donde GitHub Enterprise Server publica la suya. La ruta solo puede tener letras, números, puntos, guiones y guiones bajos, y se siguen rechazando el http sin cifrar, las credenciales, los puertos, las consultas, los fragmentos, los caracteres escapados y los retrocesos. Para que otros programas puedan reutilizarlo, el aviso de versión nueva pasa a una biblioteca propia, aviso-version-github, que viaja dentro del propio repositorio, en la carpeta bibliotecas/aviso-version-github. La consulta se identifica ante GitHub con el nombre de la biblioteca.

La ruta del programa de cada agente se puede elegir a mano. En la ventana «Conectar MMCelt con mis IAs...», cada agente enseña el programa que se va a abrir y si se ha detectado solo o se ha elegido a mano, y el botón «Cambiar ruta...» permite elegirlo cuando el agente está instalado en otra carpeta. La ruta elegida se guarda en las preferencias y manda sobre la detección; si deja de valer, se avisa y se usa la detección. Además, la detección ya no se limita al PATH: busca también en las carpetas donde dejan su programa los instaladores oficiales de Claude Code, Codex CLI y Antigravity CLI.

Versión 0.13.0 — 2026-10-08

Aviso de versión nueva en GitHub. Al arrancar, MMCelt pregunta a GitHub por la última versión publicada y, si es más nueva, lo indica en la barra superior con un enlace a su página. No descarga ni instala nada. La consulta va en un hilo aparte, solo por HTTPS verificando el certificado, y si falla no molesta: no aparece ningún aviso y el fallo queda en el registro. Se desactiva en el menú Ver y Diseño y, desactivada, no hay ningún tráfico de red. Es la primera dependencia de red del proyecto: ureq 3.4, solo con rustls. La ayuda en los seis idiomas, el catálogo de funcionalidades, el manual, la arquitectura y los documentos de infraestructura lo explican.

Ctrl+F busca un nodo y la tecla Inicio centra la vista. Ctrl+F lleva el cursor al buscador del panel lateral con lo que hubiera escrito ya seleccionado, como en cualquier programa, y sigue valiendo mientras se escribe en otro campo. Centrar la vista pasa a la tecla Inicio (Home, Origine o Pos1 según el idioma), que solo actúa cuando no se está escribiendo: dentro de un campo de texto lleva el cursor al principio de la línea sin mover el mapa. El menú, la ventana de atajos, las guías de ayuda en los seis idiomas, el catálogo de funcionalidades y el manual lo reflejan.

Insert añade un hijo, igual que Tab. El nombre de la tecla sale traducido en los menús, en la ventana de atajos y en la ayuda («Insertar», «Insert», «Inser», «Einfg»). Solo actúa cuando no se está escribiendo en un campo.

Menú contextual con clic derecho sobre un nodo. Un clic derecho sin arrastrar selecciona el nodo y abre «Acciones del nodo»: añadir hijo o hermano, conexión cruzada, editar título, eliminar, y submenús de estado, prioridad, control humano y rol. Estado, prioridad y control humano se cambian con operaciones del modelo que solo marcan el mapa como modificado si el valor cambia de verdad. Arrastrar con el botón derecho sigue moviendo la vista.

Menú al soltar un nodo encima de otro. Soltar un nodo arrastrado con su centro sobre otra tarjeta ofrece «Hacer hijo», «Hacer hermano», «Conectar con enlace», «Mover aquí sin tapar» y «Cancelar». «Hacer hijo» es una operación nueva del modelo que rechaza la raíz, el padre actual y los ciclos; «Mover aquí sin tapar» busca el hueco libre más cercano junto al destino. Cuando el nodo ya es hijo o hermano del destino, la opción correspondiente aparece deshabilitada y explica su motivo en los seis idiomas, de modo que arrastrar ya no sirve para reordenar hermanos. Esc, un clic fuera o «Cancelar» devuelven el nodo a su sitio, y con el menú abierto los atajos de estructura no actúan. «Conectar con enlace» y el cuadro de conexión cruzada no crean una segunda línea entre padre e hijo ni repiten una conexión que ya existe; los importadores siguen conservando lo que trae el archivo.

Textos emergentes e iconos en las tarjetas. Pasar el ratón por el icono de estado, prioridad, control humano, rol o notas muestra qué significa, con los mismos nombres que usa el inspector. Las zonas sensibles coinciden con las posiciones en que se pintan los iconos, y durante un arrastre no aparece nada. Cada tarjeta muestra el icono de su rol. La fila inferior de iconos se reserva en todas las tarjetas, que por eso miden lo mismo tengan notas o no, y el icono de rol ya no queda pegado debajo del de prioridad. Los textos emergentes ya no salen estrechos después de haber mostrado uno corto, y en chino ya no aparecen dos signos de dos puntos seguidos. Los títulos de los mapas de ejemplo pierden el emoji, que parecía un icono.

El mapa solo responde al ratón dentro de su zona. Elegir un valor en uno de los desplegables del inspector ya no cambia el nodo seleccionado cuando hay otro nodo detrás de la lista o del propio inspector: el mapa solo atiende las pulsaciones que empiezan en él y sin nada encima.

Requisitos y preparación del entorno. La versión mínima de Rust pasa de 1.85 a 1.95, que es la que exigen egui y eframe 0.36; con 1.85 los scripts de preparación daban el visto bueno y la compilación fallaba después. El script preparar-entorno.ps1 no llegaba a ejecutarse en Windows PowerShell 5.1, que lee los scripts UTF-8 sin BOM con la página de códigos ANSI; ahora lleva el BOM y una prueba lo comprueba. Las guías de integración avisan al principio de qué entornos ya no se conectan desde MMCelt, y el manual explica que Gemini CLI solo se puede usar con una clave de API de Google AI Studio.

Versión 0.12.0 — 2026-09-23

Indicadores de prioridad y supervisión humana en el nodo. El lienzo muestra de forma compacta solo los indicadores que requieren atención: prioridad Alta o Crítica y los estados especiales de supervisión humana. Los nodos ordinarios quedan limpios y la geometría reserva el margen necesario para no solapar el título. La prioridad Baja y Media y el estado de revisión pendiente llevan también su icono en el menú del inspector y en el nodo, igual que los estados del nodo, en los seis idiomas.

Buscador por estado y prioridad. La búsqueda permite combinar texto, estado y prioridad, ordenar por prioridad cuando se pide y limitar a cincuenta resultados después de ordenarlos, para no descartar primero los nodos críticos.

Intercambio en formatos abiertos: OPML y FreeMind o Freeplane (.mm). Se incorporan exportación e importación completas. La jerarquía, el título y las notas usan los campos nativos de cada formato; estado, prioridad, rol, revisión, comentario, etiquetas, ruta, posición, plegado y conexiones cruzadas viajan como JSON estructurado dentro de las notas, de modo que al reimportar en MMCelt se recupera toda la información. Los archivos de otras aplicaciones sin metadatos de MMCelt reciben valores por defecto, un aviso explícito en los seis idiomas y la disposición automática configurada, para que los nodos no queden amontonados; un archivo con metadatos propios conserva sus posiciones exactas. La documentación y la ayuda integrada explican campo por campo qué ve una aplicación externa y qué recupera MMCelt.

Reorganización interna de la aplicación. Tres responsabilidades salen de src/aplicacion.rs a módulos propios que reciben solo el estado y el dominio que necesitan, nunca la aplicación completa: la vigilancia del mapa abierto pasa a vigilancia_mapa.rs; la preparación, confirmación y seguimiento de los envíos a agentes de IA pasan a envio_agente_comando.rs, y la aplicación conserva solo métodos de fachada; y el autoguardado, la recuperación, la detección de cambios locales y la carpeta sugerida pasan a persistencia_mapa_comando.rs. Los diálogos y avisos siguen en la capa de presentación. Unas pruebas estructurales impiden que esas funciones vuelvan a la aplicación. No cambia nada de lo que se ve.

Versión 0.11.15 — 2026-09-23

La ayuda ya no enseña sus propias marcas de formato. Quien abría una guía leía asteriscos, acentos graves y signos de mayor en crudo en mitad de la frase. El panel de ayuda interpreta ahora la negrita, el código, las citas y la cursiva, y el texto sigue ajustándose al ancho del panel en los seis idiomas.

Las traducciones de la ayuda estaban recortadas. El castellano se había ampliado y los otros cinco idiomas se habían quedado con la versión corta, conservando los encabezados pero perdiendo contenido. Se han rehecho, y una comprobación impide que una guía traducida vuelva a tener menos contenido que la castellana.

MMCelt reconoce su propio documento en todos los idiomas. Para decidir si un archivo .md lo había escrito él, buscaba una frase en castellano, pero la cabecera del documento está traducida; en inglés, francés, alemán, ruso o chino, exportar y volver a exportar encima terminaba con el programa negándose a reconocer un documento suyo. Ahora reconoce su firma en los seis idiomas.

Las pruebas ya no dependen de la máquina en que se ejecutan. Parte de las comprobaciones leía las preferencias reales del equipo, de modo que el resultado podía cambiar según el idioma configurado. Ahora cada prueba fija su propio entorno.

Las plantillas de arquitectura, traducidas. El submenú se traducía y sus dos opciones no. Se traduce lo que es descripción y se conserva lo que es nombre propio, como «Clean Architecture (Hexagonal)», «Fullstack Web App» y «Claude Code». Una comprobación impide que vuelva a colarse un texto idéntico en los seis idiomas sin un motivo escrito al lado.

Versión 0.11.14 — 2026-09-19

Revisión completa de la ayuda y la documentación. La ayuda ofrecía conectar Claude Desktop, Antigravity, Cursor y Windsurf, que ya se habían retirado del producto, y el menú abría una guía titulada «Claude Desktop (MCP)». Los veinticuatro temas se han revisado uno a uno en los seis idiomas: la guía pasa a titularse «Servidor MCP y agentes de IA» y las guías de integración hablan de los tres agentes que MMCelt conecta, Claude Code, Codex CLI y Gemini CLI. Donde se cita literalmente un mensaje de error de Google, el nombre retirado se conserva para no falsear la cita.

Se explican tres funciones que no estaban documentadas: la vista previa del documento que se manda a la IA, las galletas de ayuda para principiantes, incluido cómo recuperarlas si se descartan, y qué hacer cuando no hay ninguna consola de IA instalada. La guía de primeros pasos explica ahora qué es un mapa mental antes de pedir nada.

El manual y el catálogo de funcionalidades citan los rótulos tal como aparecen en pantalla. Se añade una sección de trucos y consejos que explica cómo obtener una clave de API gratuita en Google AI Studio para usar Gemini CLI y resume, con su fecha y su origen, lo que se sabe del acceso con cuenta personal.

Nuevas pruebas impiden que un rótulo del programa ofrezca un producto retirado y que una ruta de menú citada en los manuales no exista en la interfaz.

Versión 0.11.13 — 2026-09-18

Motivos de error traducidos a los seis idiomas. El sistema de errores (src/error.rs) deja de producir frases en castellano. El motivo por el que se rechaza una carpeta de proyecto se modela con el enum cerrado MotivoCarpetaInvalida, y AppError::mensaje_usuario devuelve una clave tipada, ClaveError, con su contexto. Las traducciones a los seis idiomas (es, en, fr, de, ru, zh) se centralizan en src/textos.rs, de modo que nadie que use la aplicación en otro idioma reciba mensajes en castellano. Una prueba en src/audit_checks.rs impide que src/error.rs vuelva a contener texto para el usuario.

El mismo tratamiento alcanza a los errores al importar desde la IA y a los de entrada inválida, que se modelan con los enums FalloJsonIa, FalloEsquemaMarkdown, CampoEntrada y MotivoEntradaInvalida. Cuando el detalle procede de una biblioteca externa y no está traducido, como el mensaje de sintaxis de serde_json, que llega en inglés, se presenta dentro de una frase en el idioma de la persona, con el detalle técnico entre paréntesis.

Versión 0.11.12 — 2026-09-17

El proyecto es siempre la carpeta del mapa. Al abrir un mapa, el espacio de trabajo del proyecto adopta solo la carpeta donde está el archivo; ya no se conserva una carpeta superior cuando el mapa está en una subcarpeta, lo que podía exponer otros proyectos. Si el mapa está en la raíz de un volumen o en una carpeta no apta, el espacio de trabajo queda vacío y no se asocia a ningún proyecto.

La ventana «Conectar con mis IAs...» pierde el botón «Cambiar» y sincroniza la carpeta con el mapa activo al abrirse, para que el mapa abierto y la carpeta autorizada para los agentes no puedan divergir. Conectar se rechaza si la carpeta contiene otros proyectos .mmcelt en subcarpetas sin ser ella misma la carpeta del proyecto, y se pide conectar cada proyecto en su propia carpeta.

«Guardar como» con dos salidas explícitas. Guardar en la misma carpeta del proyecto activo guarda directamente. Guardar en otra carpeta abre un diálogo, en los seis idiomas, con dos opciones: «Guardar copia aquí», que guarda una copia y deja intactos la sesión y el proyecto originales, y «Guardar y trabajar en la nueva carpeta», que guarda en la nueva ubicación y traslada allí la sesión. La lógica vive en el módulo nuevo src/guardado_como.rs.

Las pruebas que vigilan el crecimiento del código dejan de medir el tamaño de los archivos y pasan a medir su estructura: cuántos tipos implementa cada módulo, cuántos métodos acumula el mayor de ellos y de cuántas partes del programa depende. Un límite de líneas se cumple partiendo un archivo por la mitad aunque el corte no siga ninguna responsabilidad; las medidas nuevas no se mueven troceando al azar. El tamaño se conserva como dato informativo. Este cambio no altera el programa.

Versión 0.11.11 — 2026-09-16

Mapas abiertos en la raíz de un volumen. Al abrir un mapa guardado directamente en la raíz de una unidad, MMCelt lo detecta en ese momento y muestra un diálogo, en los seis idiomas, con tres salidas: mover el mapa a una carpeta de proyecto, crear un mapa nuevo en una carpeta elegida o dejarlo en la raíz solo para consultarlo.

Si se elige solo consultarlo, el mapa queda en solo lectura: no se pueden añadir, borrar ni editar nodos, los atajos de teclado que modifican están desactivados y no hay guardado ni autoguardado en la raíz. Intentar guardar o abrir «Proyecto e instrucciones» vuelve a mostrar el diálogo en lugar de acceder a otros proyectos. La barra de herramientas muestra un indicador «Mover a una carpeta de proyecto» que abre el asistente de traslado en cualquier momento, y las opciones del menú que requieren edición aparecen deshabilitadas. Al mover el mapa a una carpeta válida o crear uno nuevo en ella, la edición, el guardado y la conexión de agentes vuelven a estar disponibles de inmediato.

Para desarrolladores: el traslado de archivos en disco sale de src/aplicacion.rs al módulo src/traslado.rs, que centraliza la validación de colisiones y el movimiento entre volúmenes. Se añaden pruebas que vigilan el tamaño y el número de métodos de los módulos de producción (marcas_de_tamano.txt), limitan la repetición de una misma decisión en varios sitios (marcas_de_duplicacion.txt) y prohíben textos de interfaz escritos directamente en el código fuera de los módulos que no dibujan.

Versión 0.11.9 — 2026-09-16

Apertura clara y segura de «Proyecto e instrucciones». Todo error de carpeta de proyecto inválida o inaccesible nombra la ruta exacta que falló, tanto en el registro técnico como en el mensaje para la persona; el tipo de error lo exige por construcción. Cuando la carpeta no se puede cargar, la ventana no se abre, no queda ningún editor a medias y no se crean carpetas ocultas en el disco: en su lugar aparece un diálogo, en los seis idiomas, que explica el motivo, muestra la ruta afectada y ofrece un botón para ir a «Conectar con mis IAs...».

Todos los puntos de entrada a «Proyecto e instrucciones» (el menú de la barra de herramientas, el botón del inspector, la ficha de proyecto del panel lateral y el enlace de la ventana de sesión de agente) pasan por una sola función, abrir_proyecto_ia_en_pestana. Las pruebas que lo comprueban dibujan la interfaz y pulsan los botones de verdad, recorren toda la capa de interfaz en busca de otras formas de abrir la ventana y exigen que las seis traducciones sean distintas entre sí.

Versión 0.11.8 — 2026-09-15

Cada texto con el formato que necesita su destinatario. Los nombres de las tres plantillas de encargo dejan de mostrarse en pantalla con las almohadillas de encabezado de Markdown, que solo tienen sentido dentro del documento _AI.md dirigido a la IA. El rótulo de pantalla y el encabezado del documento pasan a ser dos textos independientes en los seis idiomas. Se retiran las tres etiquetas que había bajo el desplegable de plantillas, que repetían lo que el propio desplegable ofrece y no se podían pulsar. Una prueba exige que los títulos de la interfaz no contengan marcas de Markdown y que los encabezados del documento exportado sí las conserven.

Versión 0.11.7 — 2026-09-14

La raíz de un disco ya no se convierte en carpeta de trabajo. Abrir un mapa suelto en la raíz de un volumen (la raíz de una unidad en Windows o / en Unix) ya no convierte esa raíz en espacio de trabajo ni la guarda en las preferencias. Una preferencia anterior que apunte a una raíz de volumen se descarta automáticamente, lo que evita el bloqueo «la ubicación es demasiado amplia».

En la ventana de envío a agentes, los campos de encargo y reglas son de solo lectura y no admiten foco ni edición; unas pruebas lo comprueban con clics y pulsaciones de teclado simulados.

Gemini CLI funciona de principio a fin con una clave de API gratuita, así que se retira la advertencia «MMCelt no ha podido verificar esta conexión de principio a fin» de la interfaz, la documentación y los manuales en los seis idiomas. Se conserva el aviso de que Google exige una cuenta autorizada.

Versión 0.11.6 — 2026-09-14

Solo los agentes de consola. MMCelt ofrece únicamente los tres agentes que funcionan por consola: Claude Code, Codex CLI y Gemini CLI. Se retiran Antigravity, Cursor, Windsurf y Claude Desktop, que nunca se habían probado de principio a fin. El servidor MCP no cambia, porque lo usan los propios agentes de consola. Quien tenga configurado uno de los retirados debe desconectarlo antes de actualizar, porque el botón que lo limpiaba desaparece con su fila.

La carpeta de trabajo vuelve a seguir al mapa abierto. Elegir una carpeta a mano la fijaba de forma permanente e invisible, y abrir después un mapa de otro sitio terminaba en «ruta fuera de la carpeta del proyecto» sin explicación. Ahora la carpeta elegida se respeta mientras el mapa siga dentro de ella, y se adopta la del mapa cuando queda fuera. La comparación normaliza los prefijos largos de Windows, de modo que la misma carpeta escrita de dos formas se reconoce como una sola.

La fila de Gemini CLI advierte, en los seis idiomas, de que Google exige una cuenta autorizada y ya no admite la cuenta personal estándar. La ayuda y el manual recogen el mismo aviso.

Un único encargo y unas únicas reglas. «Proyecto e instrucciones» y «Enviar a un agente» comparten el mismo encargo y las mismas reglas: lo que se redacta en la ventana de configuración se guarda al cerrarla en la carpeta del proyecto (.mmcelt/configuracion-agente.json e instrucciones-agente.md), y la ventana de envío solo confirma lo preparado, en modo de solo lectura. Si el encargo está vacío, la ventana de envío lo advierte y ofrece un botón que lleva a la pestaña de plantillas. Esa pestaña permite aplicar las tres plantillas de serie y también guardar y aplicar plantillas personalizadas del usuario, que se conservan en sus preferencias y no se incluyen en el documento exportado para la IA.

Para desarrolladores: añadir o quitar un agente obliga a revisar la orden de consola y el nombre del expediente, porque el compilador lo exige en lugar de dejar que el caso nuevo caiga en una rama genérica.

Versión 0.11.5 — 2026-09-13

Conectar o desconectar Codex CLI ya no estropea su config.toml cuando está escrito con subsecciones, que es una forma válida de la configuración oficial. MMCelt sustituye o retira su propia tabla junto con sus descendientes, conserva intactos los demás servidores, incluidos los que solo comparten el prefijo del nombre, y comprueba el resultado con un analizador TOML real.

Versión 0.11.4 — 2026-09-13

La carpeta de trabajo implícita sigue al mapa abierto o guardado, salvo que la persona haya elegido expresamente otra. Un registro MCP que apunta a otro ejecutable de MMCelt ya no se presenta como conexión válida del programa en curso, aunque los dos archivos tengan el mismo nombre.

«Proyecto e instrucciones» muestra siempre la carpeta efectiva, permite desplazarse en todas sus pestañas de contenido libre, oculta los prefijos largos de rutas de Windows y aclara cuándo una fuente nueva está pendiente de aceptación. Si se cambia de mapa con la ventana abierta, el editor se sincroniza con la nueva carpeta, y guardar reglas o aceptar fuentes usa siempre la carpeta que se muestra.

El idioma del documento y el botón «Guardar como reglas comunes» aparecen en un pie común visible desde las cuatro pestañas. Cambiar el idioma guarda al momento la preferencia del proyecto, sin guardar las reglas, y la elección se recupera al volver a abrir la ventana. El área de pestañas reserva el espacio del pie para que esos controles sigan al alcance en pantallas pequeñas.

Versión 0.11.3 — 2026-09-12

El botón de envío vuelve a estar al alcance. La vista final de la preparación del envío ocupaba más alto que la pantalla y dejaba fuera el botón «Iniciar en consola», y sus dos áreas desplazables se movían a la vez con la rueda. Ahora el espacio se reparte según la altura disponible, el pie de acciones queda siempre visible y cada área se desplaza por separado. Unas pruebas lo comprueban con la ventana real y pantallas de 720, 768 y 800 píxeles de alto.

El encargo elegido en «Plantillas de encargo» se conserva al cerrar la preparación y alimenta el borrador final del agente elegido. Cuando no hay archivos de instrucciones propios del agente, la pestaña Instrucciones nombra AGENTS.md, CLAUDE.md y GEMINI.md y muestra la carpeta exacta donde se puede crear uno. El selector de tipo de conexión ofrece un botón «+info» antes de crear la primera relación, que abre directamente la guía correspondiente.

Versión 0.11.2 — 2026-09-11

Instrucciones para la IA visibles y configurables. La antigua «Visión del Creador» se amplía como «Proyecto e instrucciones para la IA», con cuatro pestañas. La persona puede conservar las reglas comunes del proyecto, elegir por separado el idioma del documento, partir de una plantilla de encargo y leer el texto exacto antes de confirmar. El mismo resultado llega a la vista previa, al expediente inicio.md y al agente.

Los archivos AGENTS.md, CLAUDE.md y GEMINI.md solo se incorporan con consentimiento explícito. Su contenido y su huella SHA-256 quedan en la configuración del proyecto; cualquier cambio en el archivo retira el consentimiento y muestra la comparación. Justo antes de iniciar se vuelven a comprobar el mapa, las reglas, el encargo y las fuentes, y una diferencia anula la confirmación sin abrir ningún proceso. El contexto automático deja de contener tres encargos a la vez: las opciones siguen en la exportación manual, pero cada sesión usa solo uno. Se añade la dependencia sha2 0.10 para las huellas; no hay red, credenciales ni API de pago implicadas.

Un recorrido completo por el trabajo con la IA. La ayuda integrada incorpora el tema «Recorrido completo con la IA» en los seis idiomas. Explica en una sola secuencia qué prepara MMCelt, qué se entrega al agente, cómo se continúa por consola o por MCP, dónde queda la copia aprobada y cómo vuelve el trabajo al mapa sin sustituir las decisiones de la persona. Las guías «Visión del Creador» y «Conexiones Cruzadas», que estaban resumidas en cinco idiomas, tienen ahora el mismo alcance que en castellano. El selector y el título de la guía correspondiente se llaman igual que la ventana, «Proyecto e instrucciones para la IA», en los seis idiomas.

Correcciones en la ayuda y en el documento para la IA. Diez guías traducidas terminaban con un segundo título principal; ahora el contenido nuevo está dentro de su sección, y una prueba exige un solo título principal por guía y la misma estructura de encabezados que en castellano. Se corrigen seis referencias que no coincidían exactamente con la interfaz, como los puntos suspensivos del botón «Enviar a...». El documento que se entrega a la IA ya no coloca la sección 4.1 antes de la 4: las secciones siguen el orden del 1 al 9 en los seis idiomas, y las plantillas de correcciones y dudas remiten al número y al título que realmente se generan.

Versión 0.11.1 — 2026-09-09

La ayuda nombra lo que aparece en pantalla. Las indicaciones de la ayuda integrada y de los manuales se han cotejado con los rótulos que MMCelt muestra en sus seis idiomas. Había textos antiguos, rutas que enviaban al menú equivocado y rótulos que existían pero con otro significado, como los estados de revisión en alemán y chino, el escáner en francés y alemán, la reorganización y las plantillas en ruso o la importación desde IA en chino. También se corrigen los tres consejos del diálogo de atajos que nombraban versiones abreviadas de exportar, copiar el prompt e importar. Unas pruebas comparan ahora cada cita de los manuales y de la ayuda con los rótulos reales de la aplicación, conservando la puntuación.

Apertura limpia en Windows. Abrir el ejecutable desde el Explorador ya no crea una consola negra vacía detrás de la aplicación. Las compilaciones de depuración conservan su consola, y los modos de línea de órdenes siguen funcionando: mmcelt --version escribe en la terminal que lo inició y mmcelt --mcp-server mantiene la comunicación con el agente.

Las tres formas de hacer crecer un mapa, a la vista. El inspector tenía botón para añadir un hijo y un hermano, pero la conexión cruzada solo estaba en el menú Edición. Ahora tiene su propio botón, «Conexión», junto a los otros dos y en los seis idiomas. Además se corrige un defecto por el que la conexión cruzada no se podía crear desde el programa: el cuadro se cerraba en el mismo instante en que se abría. Una prueba exige que las tres operaciones tengan botón.

Seguridad del servidor MCP. El servidor rechaza mensajes JSON-RPC incompletos, identificadores de tipo incorrecto, parámetros desconocidos y mensajes o archivos que superen sus límites. Las rutas se comprueban contra la raíz canónica del proyecto antes de leer o escribir, y la carpeta de control .mmcelt queda fuera de cualquier ruta que entregue un agente. Las operaciones de sincronización usan tipos propios en lugar de interpretar valores libres a mitad de modificar el mapa, y la validación del mapa comprueba también que los dos extremos de cada conexión cruzada existan. Antes de abrir la consola se vuelve a confirmar la identidad del ejecutable del agente.

Una interfaz que no espera al disco. La carga de cambios externos y la preparación de «Enviar a...» se ejecutan fuera del hilo gráfico. Cada trabajo tiene un único hilo y colas de mensajes acotadas, de modo que un doble clic no duplica una sesión ni crea procesos sin seguimiento. Si la persona tiene cambios locales o el agente deja un JSON a medio escribir, el mapa en memoria se conserva. Cada escritura atómica usa un archivo temporal con nombre exclusivo, lo que evita un fallo cuando dos hilos escribían a la vez. Los mapas grandes usan una revisión incremental y un índice espacial del lienzo; con 10.000 nodos, el coste de cada fotograma mejora al menos un 25 %.

Arquitectura y documentación. La persistencia, la preparación de sesiones, el lanzamiento de consolas, las plantillas y el escaneo de carpetas se separan de la interfaz mediante casos de uso y adaptadores pequeños. RepositorioProyectos se divide en tres contratos pequeños y desaparece ServicioProyectos, que solo delegaba; la interfaz compone la persistencia una sola vez mediante ServiciosAplicacion, y los campos de agentes y persistencia se agrupan en EstadoAgentes y EstadoPersistencia. AplicacionMapaMental sale del módulo ui y sus agregados tienen campos privados, de modo que una pantalla no puede sustituir el estado. La compilación de producción prohíbe el código muerto con forbid, y la documentación Rustdoc es obligatoria para toda la API pública, con los enlaces rotos tratados como errores.

La guía de OpenAI distingue ChatGPT web, Codex CLI y la aplicación Codex, explica el prompt editable y aclara qué recorrido necesita copia manual y cuál puede devolver el mapa mediante MCP. La ayuda y los manuales distinguen, para los tres ecosistemas, el cliente de consola del cliente gráfico: cuáles abre MMCelt, cuáles debe abrir o reiniciar la persona y dónde continúa cada conversación. El recibo de devolución que deja el servidor MCP se documenta como constancia para la trazabilidad, no como autorización.

Versión 0.11.0 — 2026-09-06

Deshacer y rehacer. Ctrl+Z y Ctrl+Y, con sus dos entradas al principio del menú Edición. Hasta ahora un borrado en cascada, que se lleva toda la descendencia del nodo, era definitivo. El historial guarda cincuenta instantáneas del mapa y detecta los cambios comparando la huella del proyecto en cada fotograma, con la misma función que usa el autoguardado. Los cambios separados por menos de medio segundo se funden en un solo paso, de modo que un arrastre o una ráfaga de tecleo se deshacen de una vez.

Búsqueda de nodos por título o etiqueta, en lo alto del panel derecho. Al elegir un resultado, el nodo queda seleccionado y la vista va hasta él conservando el zoom. La búsqueda ignora mayúsculas y tildes y no mira las notas, a propósito, porque son párrafos largos y cualquier palabra común devolvería medio mapa. Un mapa con un ciclo entre nodos no cuelga el programa.

Intervalo de autoguardado configurable. Se elige en Ver y Diseño, Autoguardado, entre desactivado, cada minuto, cada dos, cada cinco o cada diez minutos, y se recuerda entre sesiones. Antes eran dos minutos fijos en el código. Un valor que no esté entre los ofrecidos, escrito a mano en el archivo de preferencias, se ignora.

El programa abre en el idioma del sistema la primera vez. Hasta ahora abría siempre en castellano. La detección solo actúa cuando no hay preferencias guardadas, así que quien ya usaba MMCelt no nota nada, y en cuanto alguien elige idioma en el menú manda su elección. En Windows se consulta GetUserDefaultLocaleName; en Linux y macOS, las variables LC_ALL, LC_MESSAGES y LANG. Un idioma que el programa no habla deja el castellano, y el chino tradicional no se sirve como simplificado.

La conexión cruzada se llama igual en toda la interfaz. El menú decía «Crear Conexión Cruzada» y el cuadro se titulaba «Crear Relación Cruzada Transversal». Quedan unificadas las piezas visibles del recorrido en los seis idiomas, además de la ayuda y el manual. El modelo de datos y la tabla del Markdown de intercambio no cambian, porque son el contrato con los agentes. Se corrigen también dos avisos en castellano con redacción poco clara.

Terminología de menú en los idiomas traducidos. «Rehacer» en ruso pasa de Вернуть a Повторить, el término que usan GNOME, KDE, Qt y Mozilla. Se contrastaron catorce términos de menú en los cuatro idiomas traducidos con esos cuatro corpus. Se mantienen a propósito tres elecciones que parecen erratas: Отмена para «Cancelar» en ruso, porque Отменить ya nombra «Deshacer»; Wiederholen para «Rehacer» en alemán, porque Wiederherstellen ya nombra recuperar el trabajo perdido; y 撤销 en chino, que convive con 撤消 y es igual de correcto. Una prueba fija esta terminología para que no cambie sin revisarla.

La configuración de los agentes se escribe de forma atómica. Al registrar o retirar MMCelt como servidor MCP en el .mcp.json de Claude o en el config.toml de Codex, donde el usuario puede tener otros servidores, el archivo se truncaba antes de escribirlo y un corte en ese momento lo dejaba sin configuración. Ahora se escribe al lado y se renombra encima, como el resto del programa, y se sigue haciendo una copia previa con marca de tiempo.

Para desarrolladores. Ninguna función de lógica pasa de cien líneas; se partieron los menús de la barra, los modales, validar_ruta, las herramientas del servidor MCP, los mapas de ejemplo, los dos lados del árbol horizontal y la barra de estado. Las tablas de traducción se dejan largas a propósito, porque así el compilador avisa de cualquier variante sin traducir. Se eliminan duplicaciones en el menú de ayuda, la tabla de atajos y el texto de ayuda del campo file_path. AplicacionMapaMental pasa de cuarenta y siete campos a veintiocho, agrupados en VentanasAbiertas, VistaDelLienzo, EdicionDelTitulo y ConexionEnCurso. La documentación tenía siete enlaces rotos por elementos renombrados; se corrigen y la integración continua ejecuta cargo doc --no-deps con -D warnings. La documentación de tres funciones de mcp_server.rs estaba unida a la función equivocada y vuelve a su sitio. Los comentarios que quedaban en inglés se traducen y los tamaños, grosores y radios sueltos del lienzo pasan a constantes documentadas. Una prueba comprueba que ninguna fila del inspector ensanche el panel a lo largo de varios fotogramas, y otra vigila que escanear el repositorio, guardar el mapa y componer el documento para la IA sigan costando milisegundos en el hilo de la interfaz.

Versión 0.10.0 — 2026-09-01

«Enviar a...» enseña el prompt antes de mandarlo. Elegir un agente ya no lanza nada: abre una vista previa con el texto completo repartido en cuatro bloques. El contrato MMCelt y el contexto del mapa se muestran de solo lectura; las reglas comunes del proyecto y el encargo de esta sesión se pueden editar ahí mismo. La precedencia va escrita dentro del propio prompt (correcciones humanas, contrato MMCelt, reglas del proyecto, encargo) para que el agente no tenga que deducirla. Cancelar no guarda el mapa, no exporta, no vigila y no deja expediente.

Un perfil de proyecto común a todos los agentes. Solo el botón «Guardar como reglas comunes» escribe .mmcelt/instrucciones-agente.md; editar el bloque durante una sesión no toca el archivo.

Los archivos de instrucciones de cada agente se enseñan, no se pegan. Si en la raíz del proyecto hay AGENTS.md, CLAUDE.md o GEMINI.md, la vista previa los enumera y permite leerlos, pero no los incorpora al prompt común: cada herramienta descubre el suyo, y copiarlos todos crearía reglas duplicadas y contradictorias.

MMCelt abre la consola oficial del agente. Con Claude Code, Codex CLI y Gemini CLI, confirmar la vista previa hace cinco pasos en orden fijo: guarda el mapa, exporta el documento _AI.md, escribe el expediente de la sesión, abre una consola nueva en la carpeta del proyecto y solo entonces empieza a vigilar el mapa. En Windows la ventana se crea directamente sobre el ejecutable del agente; en Linux se usa la primera terminal disponible de una lista cerrada (x-terminal-emulator, gnome-terminal, konsole, xfce4-terminal) y, si no hay ninguna, se avisa y no se lanza nada. Gemini CLI recibe además -i para quedarse en modo interactivo. Si la consola no llega a abrirse, MMCelt no dice «enviado», no empieza a vigilar, conserva la vista previa y marca el expediente como Fallido.

Cada confirmación deja un expediente reproducible en .mmcelt/sesiones. La carpeta se nombra con la fecha UTC, el identificador fijo del agente y un UUID, nunca con texto escrito por el usuario, y contiene inicio.md, con el prompt aprobado letra por letra, y sesion.json, con la versión de formato, el identificador, la fecha, el agente, la ruta relativa del mapa, las fuentes detectadas e incorporadas, el tamaño del prompt y el estado del lanzamiento (Preparado, Iniciado o Fallido). Para repetir una sesión basta con pedirle al agente que lea ese inicio.md, lo que sirve para reproducir un fallo o comparar cómo responden dos agentes al mismo encargo.

Antigravity y Gemini CLI, separados. Son dos productos distintos de Google y aparecen como destinos separados: Antigravity es cliente MCP y MMCelt no le abre consola, y Gemini CLI es la consola interactiva. Para Antigravity se guardan el mapa y el _AI.md, se crea el expediente en estado Preparado y se activa la vigilancia, con un botón «Preparar para MCP».

El portapapeles deja de ser el transporte del envío. El prompt viaja en el expediente y la consola solo recibe una referencia relativa a inicio.md: el texto completo no pasa por la línea de órdenes, ni por una variable de entorno, ni por un intérprete de shell. «Copiar Prompt Maestro» sigue existiendo para las plataformas de navegador.

El menú «Enviar a...» distingue dos capacidades. Cada agente se marca como disponible por consola y MCP, solo por consola, solo por MCP o por ninguna de las dos, porque estar conectado por MCP no significa que se le pueda abrir una consola, ni al revés. Si el MCP registrado apunta a otra carpeta, el envío se bloquea y se abre la reparación de conexiones.

Compatibilidad. Abrir la vista previa de un proyecto sin identidad propia compone un contexto temporal y no escribe nada; la identidad se crea al confirmar. En Windows la búsqueda del ejecutable respeta primero PATHEXT, para no elegir el script Unix sin extensión que npm instala junto al lanzador .cmd.

Seguridad. MMCelt no usa ninguna API de pago, no elige modelo ni decide la facturación: se apoya en la sesión que el usuario ya tenga abierta en claude, codex o gemini. El expediente no guarda contraseñas, claves, correos ni identificadores de cuenta. Todas las rutas del recorrido pasan por ContextoProyecto antes de leerse o escribirse, incluida la resolución de enlaces, y una ruta que salga de la raíz autorizada se rechaza. El expediente se escribe en una carpeta temporal y solo recibe su nombre definitivo cuando los dos archivos están cerrados.

Versión 0.9.9 — 2026-09-01

Cada carpeta conectada tiene una identidad propia que viaja con ella. La acción Conectar crea una sola vez .mmcelt/configuracion.json, con un UUID estable, el nombre del proyecto y la última carpeta confirmada. Si la carpeta se mueve, MMCelt conserva el UUID pero no adopta la nueva ubicación hasta que la persona vuelve a conectarla.

Las escrituras por MCP dejan un recibo. Crear o sincronizar un mapa en un proyecto identificado publica de forma atómica .mmcelt/devolucion.json, con un identificador de entrega, el proyecto, la ruta relativa del mapa, el agente remitente y la operación (Generado o Modificado). El contrato es el mismo para Claude Code, ChatGPT Codex, Gemini Antigravity y cualquier otro cliente MCP, y no incluye credenciales.

La identidad, la carpeta y la ruta del recibo se validan antes de publicarlo; una identidad ajena, una ruta absoluta, .. o un enlace que salga de la carpeta se rechazan. Las carpetas conectadas antes de esta versión siguen funcionando, pero no generan recibo hasta que se vuelven a conectar.

Versión 0.9.8 — 2026-08-31

Los mapas indican su formato y qué versión los escribió. Los archivos nuevos incluyen schema_version: 1 y generated_by con la aplicación y su versión. Los mapas anteriores siguen abriéndose y adquieren estos datos al volver a guardarse.

El Markdown para la IA explica el camino de vuelta. Su nueva sección 8 describe, en los seis idiomas, el orden de uso de las seis herramientas MCP, la identidad por UUID, la separación entre trabajo humano y generado por IA y todos los valores válidos de estados, roles, prioridades y relaciones.

Corregido: el saludo MCP usa la misma versión que la aplicación en lugar del antiguo 2.0.0 del servidor. La negociación confirma la versión de protocolo 2024-11-05 cuando el cliente la pide y, ante una versión desconocida, declara solo la que realmente implementa.

Versión 0.9.7 — 2026-08-31

Los títulos repetidos ya no hacen que el agente escriba sobre un nodo elegido al azar. Las actualizaciones, los padres y los dos extremos de una conexión pueden señalarse por UUID. Un título único funciona como antes; uno ambiguo rechaza el lote completo y devuelve todos los nodos que coinciden, con sus identificadores. Un UUID inválido, inexistente o que no concuerda con el título también se rechaza sin escribir nada parcialmente.

La lectura y la escritura de conexiones usan el mismo formato. Cada conexión conserva from y to con sus UUID y añade from_title y to_title. Si un archivo tiene un extremo huérfano, la lectura conserva el UUID y devuelve el título como null, sin tumbar el servidor. El catálogo MCP publica las alternativas id o title, parent_id o parent_title, y from_id o from_title con to_id o to_title.

Versión 0.9.6 — 2026-08-31

Un agente puede desarrollar el mapa existente de una persona sin sustituirlo. La herramienta mmcelt_sync_ai_progress admite el rol y las etiquetas de cada nodo y conexiones cruzadas, con el mismo contrato que la creación de mapas. Las conexiones se resuelven después de los nodos, así que pueden usar un nodo añadido en la misma llamada.

Seguridad: las etiquetas del agente se suman a las humanas en vez de borrarlas, los nodos nuevos nacen marcados como trabajo generado por IA, una modificación anula una aprobación humana anterior y no se crean conexiones a nodos inexistentes, a sí mismas, duplicadas o hacia caminos que la persona descartó. Los rechazos y los valores desconocidos se devuelven al agente en la respuesta.

Versión 0.9.5 — 2026-08-31

El agente puede descubrir su espacio de trabajo antes de operar. La nueva herramienta de solo lectura mmcelt_workspace_info devuelve la carpeta autorizada y una lista estable de los mapas .mmcelt accesibles. Falla de forma cerrada si no hay autorización, no sigue enlaces simbólicos y no modifica ningún archivo. El saludo MCP explica además el recorrido correcto y nombra la carpeta activa, sin inventarla cuando falta la configuración.

Corregido: la ventana de conexiones propone la carpeta del mapa abierto si la persona no ha fijado otra. Si un agente sigue registrado con una carpeta anterior, aparece como pendiente de actualizar y ofrece una reparación explícita; abrir la ventana no reescribe en silencio la configuración de otro programa.

Versión 0.9.4 — 2026-08-31

Una conexión antigua ya no se presenta como operativa. La ventana de conexiones exige que la entrada mmcelt apunte a un ejecutable que exista y que incluya el argumento --mcp-server, tanto en la configuración JSON como en el TOML de Codex. Si el programa se movió o la entrada quedó incompleta, vuelve a aparecer el botón Conectar para que MMCelt la repare.

Versión 0.9.3 — 2026-08-30

Guardar las correcciones de la IA ya no puede dejar el archivo vacío. Al pulsar «Guardar» en la ventana de correcciones, el programa vaciaba el archivo de destino antes de escribir, y un corte de luz, un cierre forzado o un disco lleno en ese momento dejaba el archivo vacío. Era la única escritura del programa que funcionaba así; ahora escribe al lado y renombra encima, de forma atómica, como todas las demás. La prueba que vigila las escrituras atómicas incluye ya la ventana de correcciones.

Cambios internos que no se ven al usarlo. Las funciones más largas se han partido: el lienzo pasó de una función de 615 líneas a catorce con nombre, el inspector del nodo de 365 líneas a nueve funciones y la sincronización del servidor MCP de 281 a cinco; también la guía de ayuda, el modal de conexiones y la validación del mapa. La interfaz ya no lee ni escribe archivos por su cuenta y pasa siempre por la capa de persistencia, que a su vez depende de una abstracción en lugar de std::fs, de modo que guardar y recuperar un mapa se puede probar sin tocar el disco. Una función con ocho parámetros los agrupa y deja de necesitar que se silencie el aviso del analizador.

Versión 0.9.2 — 2026-08-27

Un tema de ayuda para «Enviar a...» y la vigilancia, en los seis idiomas. Explica qué ocurre al enviar (se guarda el .mmcelt, se exporta el _AI.md y empieza la vigilancia), qué pasa con los cambios sin guardar, cuya recarga se detiene para no pisarlos, y cómo se corta la vigilancia. La ayuda tiene ya 23 temas.

Corregido: las etiquetas del mapa de bienvenida ya no mezclan idiomas. El mapa nacía con #tech y #objetivo fuera cual fuera el idioma; ahora se crean en el idioma elegido y, una vez creado, el mapa no se vuelve a tocar.

Para desarrolladores: la prueba que comprueba que cambiar de idioma no reescribe un mapa ya creado asignaba el idioma directamente y no ejecutaba la aplicación. Ahora abre el menú y pulsa la opción de chino con eventos de ratón simulados, mediante un ejecutor de interfaz sin ventana.

Versión 0.9.1 — 2026-08-27

El programa entero habla tu idioma, no solo los menús. Todo lo que crea el programa se crea en el idioma elegido: el mapa de bienvenida, los mapas nuevos, las dos plantillas de arquitectura, los dos ejemplos completos y el mapa que resulta de escanear una carpeta de código. Son 147 textos nuevos traducidos a los seis idiomas.

Lo que ya has escrito no se toca. Cambiar el idioma de la interfaz traduce los menús, los botones y la ayuda, pero no el mapa abierto: son datos del usuario y no hay forma fiable de distinguir un nodo de ejemplo que se dejó intacto de uno escrito a mano.

Corregido: el aviso de la ventana «Acerca de» sobre una compilación con cambios sin confirmar volvía a fallar fuera del castellano, porque comparaba una constante con un texto traducido; ahora la comprobación no depende del idioma. En inglés, el rótulo «Idioma / Language:» pasa a «Language:».

Una prueba recorre los seis idiomas creando el mapa inicial, las plantillas y los ejemplos, y comprueba que no queda texto en castellano donde no corresponde.

Versión 0.9.0 — 2026-08-27

El mapa vuelve solo. Hasta ahora, trabajar con un modelo obligaba a exportar el mapa, llevarlo a mano al modelo y traer la respuesta a mano para importarla. Ahora se manda desde el propio programa y el mapa cambia en pantalla mientras el modelo trabaja.

Menú «Enviar a...» en el menú Inteligencia Artificial, con los agentes instalados en el equipo. Al elegir uno, MMCelt guarda el mapa, exporta el documento para la IA, deja el prompt en el portapapeles y empieza a vigilar el archivo.

El mapa se recarga solo cuando el agente lo cambia. Avisa el propio sistema operativo, sin sondeos, así que en reposo no consume nada. Un indicador en la barra superior muestra que la vigilancia está activa, y el botón «Detener seguimiento» la corta en un clic.

Si el usuario y la IA tocan el mapa a la vez, manda el trabajo del usuario. Si no hay cambios sin guardar, el mapa se actualiza en vivo. Si los hay, el mapa en memoria no se toca: se avisa de que la IA cambió el archivo y decide el usuario. Antes de cualquier recarga se deja una copia de seguridad en el archivo de recuperación. El autoguardado no se dispara a sí mismo, porque MMCelt reconoce sus propias escrituras comparando el contenido, y un archivo a medio escribir se ignora hasta el siguiente aviso. No hace falta un motor de fusión nodo a nodo, porque el trabajo con el modelo es por turnos.

Versión 0.8.2 — 2026-08-26

El chino ya se puede leer. Estaba traducido desde la 0.6.0, pero se veía como cuadrados porque egui no trae glifos CJK. Ahora el programa incluye un subconjunto de Noto Sans SC (licencia SIL OFL 1.1, en assets/fuentes/OFL.txt) como respaldo al final de las familias de fuentes de egui, de modo que los otros cinco idiomas se dibujan exactamente igual. El subconjunto tiene 3.769 caracteres: los que usa la interfaz más el nivel 1 de GB2312, los 3.755 hanzi de uso corriente, para que también se lea el chino que escriba el usuario o devuelva un modelo. El ejecutable crece 1,06 MB, de 7.317.504 a 8.433.664 bytes.

Una prueba lee la fuente incluida y comprueba que cada carácter chino usado en el programa tiene su glifo; si se añade un texto en chino sin regenerar el subconjunto, la prueba falla e indica qué carácter falta.

Versión 0.8.1 — 2026-08-26

Los dos paneles laterales vuelven a verse enteros. El inspector de nodo se recortaba por la izquierda porque la fila «Archivo/Ruta» ensanchaba el panel 21 píxeles: el campo de texto pedía todo el ancho y el botón «+info» ya no cabía, así que el panel, anclado a la derecha, crecía hacia la izquierda y quedaba debajo del lienzo. Ahora se reserva el botón antes que el campo. El panel de ayuda detallada también se recortaba, porque las viñetas se dibujaban en un contenedor horizontal que en egui desactiva el ajuste de línea; ahora las líneas se ajustan, igual que el título y el selector de temas.

Una prueba recorre los 22 temas de ayuda en los seis idiomas y comprueba que ninguno ocupa más ancho del que el panel le da.

Versión 0.8.0 — 2026-08-26

eframe y egui suben a 0.36, con el motor de dibujo glow (OpenGL) en lugar de wgpu.

Cambios para desarrolladores. El trait eframe::App cambió: update ya no es miembro y ahora se implementa ui, que recibe un contenedor en lugar del contexto raíz. TopBottomPanel y SidePanel se unifican en egui::Panel, y sus medidas pasan de min_height y min_width a min_size, default_size y max_size. raw_scroll_delta se sustituye por smooth_scroll_delta y wants_keyboard_input pasa a egui_wants_keyboard_input. En el arnés de pruebas de interfaz, ctx.run pasa a ctx.run_ui.

Por qué glow. Con wgpu la compilación en Windows se detenía en wgpu-hal 30.0.1 por un conflicto de windows-core con gpu-allocator. Con glow, wgpu-hal deja de compilarse en todas las plataformas y el problema desaparece. Se conservan la accesibilidad (accesskit) y el soporte de Wayland y X11. Las dependencias bajan de 428 a 410.

Versión 0.7.0 — 2026-08-26

eframe y egui suben a 0.32, y con eso el proyecto se queda sin vulnerabilidades conocidas. Es la primera versión que se publica.

Se cierran las dos vulnerabilidades de quick-xml, RUSTSEC-2026-0194 y RUSTSEC-2026-0195, ambas de severidad 7,5 y de denegación de servicio al procesar XML. Llegaban por una cadena de dependencias que arrancaba en eframe, y zbus_xml 4.0 exigía la versión vulnerable, así que cargo update no podía resolverlo. Con eframe 0.32, quick-xml 0.30 desaparece de Cargo.lock y queda solo la 0.41, que está corregida. La integración continua deja de necesitar excepciones en cargo audit.

El cambio en el código propio fueron 44 sustituciones mecánicas en src/ui/toolbar.rs: 43 de ui.close_menu() por ui.close() y una de egui::menu::bar por egui::MenuBar::new().ui(.

La versión 0.36 de egui se dejó para más adelante por dos motivos: el trait eframe::App estaba reescrito y obligaba a rehacer el punto de entrada y el arnés de pruebas de interfaz (src/arnes_interfaz.rs), y la compilación en Windows se detenía en wgpu-hal 30.0.1 por un conflicto de windows-core entre wgpu-hal, que usa windows 0.62, y gpu-allocator, que usa la 0.58. Se incorporó en la 0.8.0.
