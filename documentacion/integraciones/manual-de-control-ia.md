Manual de supervisión y alineación bidireccional con IA: Claude, ChatGPT y Gemini

MMCelt es una cabina de control visual y un plano arquitectónico para proyectos desarrollados con
inteligencia artificial. Este manual explica los dos flujos de trabajo, cómo se usa la ayuda de la
aplicación y cómo se trabaja paso a paso con Claude, ChatGPT y Gemini.

Qué conecta MMCelt hoy

Desde la versión 0.11.6 el programa se conecta con tres agentes de consola: Claude Code, Codex CLI y
Antigravity CLI (agy). Lo que este manual cuenta de Claude Desktop, Cursor y el IDE antiguo de
Antigravity se conserva como referencia de cómo funciona cada plataforma: ya no se conectan desde
MMCelt. El motivo está en el apartado «Conectarlo con tu IA» del README.md de la raíz del repositorio.

La filosofía: bidireccionalidad real entre la persona y la IA

Cuando trabajas en proyectos complejos con modelos de inteligencia artificial, ya sea arquitectura de
software, investigación, diseño de producto o estrategia de negocio, el mayor reto no es escribir
texto o código, sino mantener la alineación conceptual y el control del proyecto.

MMCelt está diseñado para funcionar en las dos direcciones. Arriba está el usuario, como arquitecto o
supervisor: diseña la estructura, los módulos y las rutas de archivo, o bien supervisa el trabajo que
la IA ha generado. Abajo está la inteligencia artificial, como constructora o socia: Claude Code,
Codex, Gemini u otro agente lee tu mapa como especificación maestra, sincroniza su avance y acata las
correcciones en vivo. Entre ambos hay dos flujos posibles: en el primero tú eres el arquitecto,
diseñas el plano en MMCelt y la IA lo programa paso a paso; en el segundo tú eres el supervisor, la
IA genera el mapa y tú lo revisas y corriges los desvíos de rumbo.

Primer flujo: tú eres el arquitecto

En este modo tienes la idea clara y quieres que la IA programe exactamente lo que tienes en mente, sin
que tome decisiones por su cuenta ni invente nombres de archivo extraños.

Abres MMCelt y creas tu mapa mental con los atajos rápidos, Tab para ramas hijas y Enter para ramas
hermanas, o cargas una plantilla de arquitectura como Clean Architecture o Fullstack. Asignas a cada
nodo la ruta de su archivo de código, por ejemplo src/auth/jwt.rs o migrations/001_init.sql. En las
notas de cada nodo escribes las especificaciones técnicas: qué librerías usar, qué algoritmos o qué
restricciones. En el panel «Visión del Creador y Metas del Proyecto (Para la IA)» redactas el
propósito global del software.

Por último se lo entregas a la IA, sea Claude Code, ChatGPT, Gemini u otra. La IA recibe el archivo
.md o .mmcelt como un plano de obra, de modo que el código se escribe en orden, respetando tus
dependencias y sin decisiones arquitectónicas inventadas.

Segundo flujo: tú eres el supervisor

En este modo le pides a la IA que investigue o plantee la solución técnica a un problema grande.

La IA analiza tu proyecto y genera el mapa mental, por MCP o mediante un bloque JSON. Al abrirlo en
MMCelt tienes en segundos una vista panorámica de lo que la IA entiende, de cómo divide el trabajo y
de qué dudas técnicas tiene. Si eligió una base de datos incorrecta o un enfoque que no deseas,
marcas ese nodo como «Descartado» o «⚠️ Requiere Corrección» y escribes tu directiva en el campo de
corrección. Después eliges «🛑 Enviar Correcciones y Directivas a la IA...» y la IA recibe un informe
estricto de control humano que le prohíbe continuar por las vías descartadas y le obliga a reajustar
el código según tus indicaciones.

La interfaz y la ayuda para usuarios nuevos

Para que cualquier persona, principiante o desarrollador con experiencia, domine la herramienta desde
el primer minuto, MMCelt cuenta con una barra de menús de escritorio y con un sistema de ayuda
contextual.

La barra de menús tiene cinco entradas. En «Archivo» están crear un mapa nuevo, las plantillas de
arquitectura, el escáner automático de carpetas de código, abrir, guardar con Ctrl+S y exportar a
Markdown para IA con Ctrl+E. En «Edición» están añadir un hijo con Tab, añadir un hermano con Enter,
eliminar con Supr, editar con Espacio o F2, las conexiones cruzadas y la ficha de visión del
proyecto. En «Inteligencia Artificial» están importar mapas desde chats de IA, copiar el prompt
maestro, el panel de directivas de corrección humana, la vista previa del .md y la exportación. En
«Ver y Diseño» están los modos de disposición automática, que son árbol balanceado, radial y libre,
el centrado de la cámara con la tecla Inicio y los temas visuales oscuro, claro y de alto contraste.
En «Ayuda» están la activación de las galletas guiadas, las guías temáticas, los atajos de teclado y
el panel de ayuda completa.

Las galletas contextuales son avisos breves y discretos que aparecen arriba del lienzo y en el
inspector lateral, y se adaptan a lo que estás haciendo o al nodo que tengas seleccionado. Cada
galleta incluye un enlace «+info». Al pulsarlo no se abre una ventana emergente, sino un panel lateral
a la derecha con toda la información disponible: qué hace exactamente esa opción, qué posibilidades
tienes para configurarla, por dónde continuar y qué pasos se recomiendan, y ejemplos prácticos de
integración con IA y atajos de teclado relacionados.

Guía paso a paso para Claude

En el ecosistema de Anthropic puedes usar Claude Code, Claude Desktop o la web de Claude. Claude Code
y Claude Desktop admiten MCP, pero la aplicación gráfica la abres tú y la consola puede abrirla
MMCelt. La configuración de cada uno se explica en el documento claude-mcp.md de la carpeta
documentacion/integraciones.

Con Claude Desktop no hay que copiar y pegar el mapa: tú abres la aplicación y la conversación, y
Claude lee y devuelve los mapas mediante MCP. Una vez registrado el servidor, el flujo es el
siguiente. Primero le pides a Claude que inicie el proyecto, por ejemplo: «Claude, vamos a diseñar una
aplicación de análisis financiero en Rust. Analiza los requisitos que te he dado y usa la herramienta
mmcelt_create_mindmap para guardar el mapa mental en finanzas.mmcelt dentro de mi espacio de trabajo,
con los pilares principales y las dudas a resolver.» Después abres MMCelt, pulsas «Abrir Mapa (.mmcelt
/ .json)...», cargas finanzas.mmcelt y ves todas las ramas en el lienzo.

Si necesitas corregir a Claude, haces clic en el nodo que quieras cambiar, por ejemplo uno que diga
usar MongoDB, cambias su estado a «Descartado» o «Requiere Corrección» y escribes en el campo de
corrección algo como: descartamos las bases de datos NoSQL, usaremos PostgreSQL porque necesitamos
transacciones ACID estrictas. Guardas el archivo con Ctrl+S. Para devolverle el control, en tu
conversación le dices: «Claude, he revisado el mapa mental en MMCelt y he dejado correcciones. Usa
mmcelt_get_human_feedback y ajusta el plan de arquitectura.» Claude ejecutará la herramienta, leerá
tus notas y responderá confirmando que ha descartado MongoDB y reorientado el desarrollo hacia
PostgreSQL.

Si usas la versión web de Claude, en MMCelt pulsas «Copiar Prompt Maestro para IA...» y pegas el texto
en el chat junto con la descripción de tu proyecto. Claude te responderá con un bloque de código JSON.
En MMCelt abres «📥 Importar desde IA (ChatGPT, Claude, Gemini)...», pegas la respuesta de Claude y
pulsas «✨ Sustituir el mapa por el de la IA».

Guía paso a paso para ChatGPT

En un chat web de ChatGPT puedes trabajar por copia manual. Para un recorrido local con MCP utiliza
Codex CLI o la aplicación Codex, como se explica en el documento chatgpt.md de la carpeta
documentacion/integraciones. La CLI la abre MMCelt; la aplicación y su tarea las abres tú.

Si tienes ChatGPT Plus, Team o Enterprise, la forma recomendada es crear un GPT personalizado que
siempre hable el idioma de MMCelt. En ChatGPT entra en Explore GPTs y pulsa Create. En la pestaña
Configure escribe en Name «MMCelt - Arquitecto de Mapas Mentales», en Description «Asistente para
estructuración visual y supervisión de proyectos con MMCelt.» y en Instructions pega el texto completo
del archivo skills/mmcelt-mindmap/SKILL.md del repositorio. Guarda el GPT. A partir de entonces, cada
vez que lo abras y le hables de una idea, te generará los bloques listos para importar a MMCelt.

En cualquier chat normal de ChatGPT también puedes estructurar tu idea. En MMCelt pulsas «Copiar
Prompt Maestro para IA...» y en ChatGPT escribes algo como: «Actúa según estas directrices de MMCelt:
aquí pego el prompt maestro. Aquí tienes mi proyecto: aquí describo mi idea o pego mi código.»
ChatGPT generará el bloque estructurado, y en MMCelt eliges «Importar desde IA (ChatGPT, Claude,
Gemini)...», pegas la respuesta y tendrás tu mapa montado.

Si tras ver el mapa descubres que ChatGPT ha omitido un módulo o ha elegido una tecnología errónea,
modifica los nodos en MMCelt, añade tus notas de corrección o elimina lo que no sirva. Abre el menú
«Inteligencia Artificial», elige «Enviar Correcciones y Directivas a la IA...» y pulsa «Copiar
Directivas de Corrección». Pégalo en ChatGPT. El mensaje empieza con una cabecera de instrucción de
corrección y reajuste del proyecto que indica que has revisado el mapa en MMCelt y exiges cambios
obligatorios. ChatGPT acatará las correcciones, desechará los caminos vetados y continuará por donde
tú le has marcado.

Guía paso a paso para Gemini

En el ecosistema de Google puedes usar el chat web o Antigravity CLI (agy). El chat web usa copia
manual y MMCelt puede abrir Antigravity CLI (agy) de forma interactiva con MCP. Los detalles están en el
documento gemini.md de la carpeta documentacion/integraciones.

Para tener un asistente permanente, crea un Gem personalizado. En Gemini entra en el gestor de Gems y
pulsa New Gem, ponle un nombre como MMCelt Strategy Architect, pega en sus instrucciones el contenido
del archivo skills/mmcelt-mindmap/SKILL.md y pulsa Save. A partir de ahí el Gem responderá siempre en
formato MMCelt.

Para analizar proyectos grandes con el .md exportado, diseña en MMCelt la estructura inicial de tu
idea, escribe tu visión en el panel «Visión del Creador y Metas del Proyecto (Para la IA)» y pulsa
«Exportar Markdown para IA (.md)» o Ctrl+E. Se generará un archivo como MiProyecto_AI.md. Abre Gemini
o Google AI Studio y arrastra ese archivo al chat junto con tus archivos de código o documentación en
PDF. Una instrucción útil es: «Gemini, he adjuntado el mapa mental de mi proyecto. Lee con máxima
atención la Sección 1, mi visión como creador, y la Sección 6, puntos de decisión, dudas y bloqueos.
Elabora una propuesta técnica detallada que resuelva todas las dudas abiertas y devuélveme el mapa
mental actualizado en formato MMCelt para que pueda supervisarlo en mi aplicación.» Copias la
respuesta, la pegas en MMCelt con «Importar desde IA (ChatGPT, Claude, Gemini)...» y tendrás en
pantalla la evolución del proyecto.

Integración con entornos de desarrollo autónomos

Cuando pasas del chat a herramientas donde la IA escribe código, refactoriza repositorios y ejecuta
órdenes en la terminal de forma autónoma, MMCelt se vuelve aún más importante como salvaguarda y mapa
de ruta visual.

Claude Code

Claude Code es el agente de línea de órdenes de Anthropic que recorre tu código, crea ramas y ejecuta
pruebas de forma autónoma en la terminal.

Para vincularlo con el servidor MCP de MMCelt basta el menú «Inteligencia Artificial → Conectar
MMCelt con mis IAs...», que detecta Claude Code y lo configura. Si lo prefieres desde la terminal, se
registra con una sola orden, sustituyendo las dos rutas por las tuyas:

claude mcp add mmcelt -s user -e MMCELT_WORKSPACE="C:/ruta/hasta/tu/carpeta/de/mapas" -- "C:/ruta/hasta/mmcelt.exe" --mcp-server

Lo que se registra es el ejecutable de MMCelt, no un script suelto, así que quien reciba el programa
no necesita instalar nada más para conectarlo con su agente.

Antes de empezar a programar, inicia Claude Code con una instrucción como esta:

claude "Analiza el repositorio actual. Antes de tocar ningún archivo de código, genera un mapa mental con mmcelt_create_mindmap en ./arquitectura.mmcelt dividiendo las fases de refactorización y los puntos críticos a comprobar."

Abre MMCelt en paralelo y carga arquitectura.mmcelt para ver el plan completo. Si ves que Claude Code
planea modificar un módulo delicado que no debe tocar, marca ese nodo como «Descartado» o «Requiere
Corrección» en MMCelt, guarda con Ctrl+S y escribe en la terminal de Claude Code:

Consulta el mapa mental con mmcelt_get_human_feedback antes de continuar y acata las correcciones.

Claude Code leerá tus directivas, cancelará la modificación de ese módulo y redirigirá su plan.

Codex, Canvas y Cursor

En los entornos de desarrollo con modelos de OpenAI, como Canvas o extensiones de código como Cursor o
VS Code, el mapa funciona como plan de arquitectura vivo. Al comenzar una sesión de programación,
añade al inicio del contexto algo como: «Actúa como responsable técnico. Mantendremos la arquitectura
y las decisiones del proyecto sincronizadas con la estructura de MMCelt. Lee el archivo
proyecto_AI.md que adjunto como especificación maestra de requisitos y restricciones.» Cuando le
pidas una refactorización profunda, añade: «Genera el código necesario y, al terminar, entrégame el
bloque JSON de actualización para MMCelt reflejando los nuevos módulos creados, las pruebas
implementadas y los posibles bloqueos técnicos detectados.» Después pegas el bloque devuelto en
«Importar desde IA (ChatGPT, Claude, Gemini)...» y ves al instante si el código generado cubre todos
los requisitos o si ha dejado cabos sueltos. Para Codex CLI y la aplicación Codex, consulta el
documento chatgpt.md de la carpeta documentacion/integraciones.

Google Antigravity

Antigravity es el entorno de desarrollo agéntico de Google basado en Gemini, pensado para planificar
y ejecutar proyectos de software grandes con subagentes y herramientas integradas. Ya no se conecta
desde MMCelt, pero el recorrido se conserva como referencia.

El archivo skills/mmcelt-mindmap/SKILL.md sigue el formato estándar de Agent Skills compatible con
Antigravity. Para cargarlo puedes indicarle a Antigravity:

Carga el skill ubicado en ./skills/mmcelt-mindmap/SKILL.md y úsalo para documentar la arquitectura visual del proyecto.

Cuando Antigravity entra en su modo de planificación para crear el archivo implementation_plan.md,
pídele que genere a la vez el mapa visual, por ejemplo: «Antigravity, junto con el
implementation_plan.md, genera el archivo plan_arquitectura.mmcelt con la descomposición visual de
tareas y dependencias.» Mientras Antigravity crea archivos, ejecuta subagentes u órdenes en segundo
plano, tú mantienes abierto MMCelt para vigilar la visión global. Si al revisar el mapa detectas que
un subagente va a tomar una decisión no deseada, eliges «Enviar Correcciones y Directivas a la IA...»
y pegas el texto en Antigravity con una indicación como: «He modificado el mapa mental en MMCelt.
Acata las siguientes directivas de corrección obligatorias, que pego a continuación. Reajusta el
implementation_plan.md antes de continuar.» Antigravity reevaluará su plan de ejecución y seguirá con
las nuevas restricciones.

Resumen de acciones y atajos

Para que la IA te dé un mapa de lo que está haciendo, pulsa «Copiar Prompt Maestro para IA...» o usa
MCP; la IA genera el JSON estructurado o ejecuta mmcelt_create_mindmap. Para ver el mapa de la IA en
tu pantalla, elige «Importar desde IA (ChatGPT, Claude, Gemini)...» y pega la respuesta, o abre el
archivo .mmcelt. Para explicarle a la IA qué pretendes con el proyecto, edita el panel «Visión del
Creador y Metas del Proyecto (Para la IA)»; la IA lo lee en la sección 1 del .md. Para corregir a la
IA si tomó un mal camino, marca «Descartado» o «Requiere Corrección» y elige «Enviar Correcciones y
Directivas a la IA...»; después pega el texto en la IA o pídele que llame a mmcelt_get_human_feedback.
Para darle a la IA todo tu proyecto preparado, pulsa «Exportar Markdown para IA (.md)» o Ctrl+E y sube
el archivo .md a Claude, ChatGPT, Gemini o Claude Code. Para mandarlo y ver el mapa cambiar solo, usa
«Inteligencia Artificial → Enviar a...», revisa el prompt, confírmalo y deja trabajar al agente. Para
cortar el seguimiento de un modelo que se ha desbocado, usa «Inteligencia Artificial → Detener
seguimiento». Para navegar a máxima velocidad, usa Tab para crear un hijo, Enter para un hermano,
Supr para borrar y Espacio para editar.

El bucle cerrado: mandar y ver volver

Los flujos web anteriores tienen un paso manual en cada sentido: llevarle el documento al modelo y
traer su respuesta. Con «Enviar a...» y un cliente MCP, la devolución puede automatizarse; la
apertura depende del cliente.

Desde «Inteligencia Artificial → Enviar a...» eliges el agente. Antes de confirmar lees en la vista
previa todo lo que va a salir; las reglas comunes y el encargo se escriben antes, en la ventana
«Proyecto e instrucciones para la IA». MMCelt guarda el mapa, exporta el documento, conserva el
prompt exacto en .mmcelt/sesiones y empieza a vigilar el archivo. Si el destino tiene una CLI
compatible, también abre una consola nueva en la carpeta del proyecto.

La familia del modelo no es la aplicación que estás usando

La consola y la aplicación gráfica son clientes diferentes. Pueden utilizar modelos de la misma
familia, pero cada una mantiene su propia conversación y puede cargar herramientas o permisos
distintos. La conexión MCP permite que el cliente lea y devuelva el mapa; no permite que MMCelt
escriba dentro de una conversación gráfica ya abierta.

En la familia Claude, el cliente de consola es Claude Code y el gráfico es Claude Desktop: «Enviar
a...» abre Claude Code, mientras que Claude Desktop lo abres o reinicias tú y le adjuntas o pegas el
expediente. En la familia OpenAI, el cliente de consola es Codex CLI y el gráfico es la aplicación
Codex: «Enviar a...» abre Codex CLI, y en la aplicación Codex abres una tarea local y le indicas el
expediente. En la familia Gemini y Google, el cliente de consola oficial es Antigravity CLI (agy):
«Enviar a...» abre agy interactivamente. En la web, ChatGPT, Claude o Gemini en el navegador,
abres el chat y adjuntas el _AI.md; un chat web ordinario no lee el MCP local.

Después de registrar o actualizar MCP, cierra por completo el cliente afectado y vuelve a abrirlo.
Los tres clientes con apertura automática son Claude Code, Codex CLI y Antigravity CLI (agy). En los clientes
gráficos, «Preparar para MCP» significa exactamente eso: el mapa, el expediente y la vigilancia están
listos, pero MMCelt no ha abierto ni controlado la otra aplicación.

La vuelta puede ser automática en cualquier cliente local que haya cargado el servidor MCP y use
mmcelt_sync_ai_progress. En cambio, un chat web corriente necesita el recorrido manual de exportación
e importación.

Tu trabajo no se pierde

Mientras el agente trabaja, el mapa está bloqueado y no se puede editar; puedes verlo, recorrerlo,
buscar y exportar. El agente marca su última entrega como trabajo terminado y MMCelt levanta el
bloqueo al cargar el mapa final. Si el agente falla o no avisa, pulsa «Recuperar el control» en la
barra inferior.

Si recuperaste el control y estabas editando el mapa cuando llega el cambio del modelo, MMCelt no toca lo tuyo: te avisa y
decides tú. Antes de cualquier recarga deja una copia en el archivo de recuperación.

Un consejo que ahorra tiempo: si el modelo dice que ha terminado y el mapa sigue bloqueado,
recuérdale que haga su última entrega con work_finished a true, o recupera tú el control.
