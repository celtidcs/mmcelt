MMCelt con Claude: control y supervisión bidireccional entre la persona y la IA

MMCelt no es solo un visor de mapas mentales: es una consola visual de control, verificación y
alineación entre el usuario y los modelos de inteligencia artificial, como Claude, ChatGPT o Gemini.
Esta guía explica cómo se conecta con los clientes de Anthropic y qué herramientas pone a disposición
del agente.

Qué conecta MMCelt hoy

Desde la versión 0.11.6 el programa se conecta con tres agentes de consola: Claude Code, Codex CLI y
Antigravity CLI (agy). Lo que esta guía cuenta de Claude Desktop se conserva como referencia de cómo
funciona esa plataforma: ya no se conecta desde MMCelt. El motivo está en el apartado «Conectarlo con tu
IA» del README.md de la raíz del repositorio.

Claude Code y Claude Desktop no se abren de la misma manera

Ambos pertenecen al ecosistema de Anthropic, pero son clientes distintos. Claude Code es la consola:
después de conectarlo y reiniciarlo una vez, «Enviar a... → Claude Code» guarda el mapa, crea el
expediente, abre una terminal nueva en la carpeta del proyecto y activa la vigilancia. No tienes que
abrir Claude Code antes, pero sí debes haber iniciado sesión en su CLI.

Claude Desktop es una aplicación gráfica y MMCelt no controla sus conversaciones. Para usarla hay que
registrar el servidor MCP en su configuración a mano, como se explica más abajo, cerrarla por
completo y volver a abrirla para que cargue MCP. Después se prepara la sesión desde MMCelt, se abre
una conversación en Claude Desktop y se adjunta o se pega el archivo
.mmcelt/sesiones/<sesión>/inicio.md o el _AI.md generado. No hay que presuponer que Claude Desktop
puede abrir una ruta del proyecto solo porque tenga el MCP de MMCelt. La conversación continúa en
Claude Desktop y el mapa vuelve mediante MCP.

La consola lleva tus mensajes; MCP aporta las herramientas para leer y actualizar el mapa. Que una de
las dos capacidades funcione no demuestra la otra, y que funcione en Claude Code no garantiza que
funcione en un cliente gráfico.

Conectar Claude Code

El servidor MCP va dentro del propio ejecutable de MMCelt, así que no hay nada que instalar aparte:
ni Python, ni Node, ni ningún paquete.

La vía corta es el menú «Inteligencia Artificial → Conectar MMCelt con mis IAs...». La aplicación
busca los agentes instalados, los muestra en una lista y los conecta al pulsar «Conectar». Hace una
copia de seguridad del archivo de configuración antes de tocarlo. Después hay que cerrar y volver a
abrir el agente, porque ninguno relee su configuración mientras está en marcha. Si tienes varias
cuentas de Claude Code, cada una en su carpeta, MMCelt muestra una ficha por cuenta: se conecta cada
una por separado, y al enviar un mapa la consola se abre con la cuenta elegida. Una cuenta que esté
en otro sitio se añade con «Añadir cuenta».

Si prefieres hacerlo desde la terminal, se registra con una sola orden, sustituyendo las dos rutas
por las tuyas:

claude mcp add mmcelt -s user -e MMCELT_WORKSPACE="C:/ruta/hasta/tu/carpeta/de/mapas" -- "C:/ruta/hasta/mmcelt.exe" --mcp-server

Lo que se registra es el ejecutable de MMCelt, no un script suelto: por eso quien reciba el programa
no necesita instalar nada más para conectarlo con su agente.

Configurar Claude Desktop a mano

Abre el archivo %APPDATA%\Claude\claude_desktop_config.json, o entra en Claude Desktop en Settings,
Developer, Edit Config, y añade esta entrada:

{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/ruta/hasta/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": {
        "MMCELT_WORKSPACE": "C:/ruta/hasta/tu/carpeta/de/mapas"
      }
    }
  }
}

Sustituye las dos rutas por las tuyas y escríbelas con barras normales, que es lo que espera el
formato JSON. MMCELT_WORKSPACE marca la carpeta dentro de la cual el agente puede leer y escribir:
fuera de ella no tiene acceso a nada. Guarda y reinicia Claude Desktop. Sabrás que ha funcionado
porque aparece un icono de martillo abajo a la derecha, con las herramientas de MMCelt dentro.

El bucle cerrado de trabajo con la persona en el bucle

El ciclo de trabajo tiene seis pasos. La IA trabaja en un proyecto complejo, ya sea código,
arquitectura o ideas, y envía o sincroniza el mapa mental con MMCelt, mediante el archivo .mmcelt o
mediante MCP. El usuario ve el proyecto con visión panorámica en MMCelt. Supervisa, valida ramas y
detecta errores o desvíos. Modifica los nodos, descartando las vías erróneas y añadiendo notas de
corrección. MMCelt envía a la IA el informe de correcciones exigidas, en Markdown o por MCP. Por
último, la IA reajusta su rumbo, acata las correcciones y continúa.

Herramientas MCP para el control del proyecto

El servidor MCP va integrado en el propio ejecutable, que se arranca con mmcelt --mcp-server, e
incluye seis herramientas.

Con mmcelt_workspace_info la IA pregunta cuál es la carpeta autorizada y qué mapas hay dentro, en vez
de adivinar rutas. Es la primera que debe llamar, y las rutas que devuelve son las que hay que
reenviar tal cual.

Con mmcelt_create_mindmap la IA genera el mapa mental visual del proyecto en el que está trabajando
para que lo veas en tu pantalla.

Con mmcelt_read_mindmap la IA lee un mapa ya existente: su jerarquía, sus notas, sus dudas abiertas y
sus conexiones. Cada nodo y cada conexión vienen con su identificador y con su título, que es lo que
le permite escribir después sin ambigüedad.

Con mmcelt_sync_ai_progress la IA desarrolla un mapa que ya existe, también uno que hayas hecho tú:
actualiza estados, como EnProgreso, Completado o DudaBloqueo, y añade nodos con su rol y sus
etiquetas, además de conexiones cruzadas entre ramas distintas.

Mientras la IA trabaja, MMCelt mantiene el mapa bloqueado para que la persona no lo edite a la vez.
mmcelt_sync_ai_progress y mmcelt_create_mindmap aceptan el campo opcional work_finished: la IA lo
pone a true solo en su última entrega. MMCelt lo lee en el recibo .mmcelt/devolucion.json, carga el
mapa final y levanta el bloqueo. Las entregas sin ese campo, o con false, se cargan y el mapa sigue
bloqueado. Si la IA no avisa nunca, la persona puede quitar el bloqueo con «Recuperar el control».
Si el envío falla antes de lanzar al agente, el bloqueo se quita solo.

Con mmcelt_get_human_feedback la IA consulta si has modificado el mapa, validado ramas o exigido
correcciones, como los estados RequiereCorreccion o Descartado.

Con mmcelt_export_ai_markdown se exporta la especificación completa en Markdown, preparada para que
la lea un modelo.

Cómo la IA amplía un mapa tuyo sin sustituirlo

Si el mapa lo has hecho tú, mmcelt_create_mindmap se niega a tocarlo, porque rehacerlo encima
borraría tu trabajo. La vía legítima para ampliarlo es mmcelt_sync_ai_progress, y con ella el agente
puede añadir nodos completos, con rol y etiquetas, y trazar conexiones cruzadas entre ramas
distintas, que es lo que hace falta para dibujar un bucle o una dependencia que la jerarquía no
expresa.

Por esa vía el agente no puede borrar un nodo tuyo ni cambiarlo de sitio. Tampoco puede sustituir tus
etiquetas, porque las suyas se añaden a las tuyas, ni presentar su trabajo como tuyo, porque todo lo
que añade nace con el estado Generado por IA y resaltado, con otro color de fondo y de borde, hasta
que lo seleccionas. No puede marcar nada como aprobado por ti, ni conservar tu aprobación sobre un texto
que ha cambiado, ni volver a enlazar un camino que hayas marcado como Descartado. Si cambia un nodo
tuyo, este pasa a Pendiente de Revisión, para que veas qué ha tocado.

Si pide una conexión imposible, como un nodo que no existe, un nodo consigo mismo o una relación que
ya estaba, no se crea y el servidor le explica por qué, en lugar de responderle que todo ha ido bien.

Qué pasa cuando dos nodos se llaman igual

En un diagrama de decisiones lo normal es tener dos nodos Sí y dos nodos No colgando de preguntas
distintas. Puedes seguir repitiendo títulos.

Si una petición nombra solo el título y ese título lo llevan varios nodos, la llamada entera se
rechaza sin escribir nada y la respuesta enumera los candidatos con su identificador, para que la IA
repita la llamada señalando el que quería. El servidor nunca elige por su cuenta.

Para poder señalarlo, cada nodo y cada conexión que lee la IA traen su identificador.
mmcelt_read_mindmap devuelve el campo id de cada nodo, y cada conexión con los identificadores from y
to junto a los títulos from_title y to_title. mmcelt_sync_ai_progress acepta id y parent_id en cada
nodo, y from_id y to_id en cada conexión. Si llegan el identificador y el título a la vez, tienen que
corresponder al mismo nodo; si no, se rechaza, porque significa que la IA arrastra una copia vieja
del mapa.

El título por sí solo sigue funcionando siempre que sea único, que es el caso más común.

Un título que no existe no es lo mismo que uno ambiguo. Si se nombra un padre que no está en el mapa,
el nodo cuelga de la raíz y la IA recibe un aviso. Ambiguo significa que hay varios sitios donde
ponerlo, y en ese caso la operación se detiene.

Cómo corregir a la IA desde MMCelt

Abre el mapa en MMCelt. Si la IA sigue trabajando, el mapa está bloqueado: espera a que termine o
pulsa «Recuperar el control». Si la IA tomó una decisión incorrecta o un rumbo no deseado, cambia el estado
del nodo a Descartado o a Requiere Corrección y escribe en el campo de corrección exigida a la IA lo
que debe cambiar, por ejemplo: no usar SQLite, porque se necesita PostgreSQL por el volumen de
escrituras concurrentes. Si necesitas ayuda para redactar o estructurar correcciones, pulsa el botón
+info junto al apartado de control humano y se abrirá la guía detallada a la derecha.

Después ve al menú «Inteligencia Artificial → Enviar Correcciones y Directivas a la IA...» y pulsa
«Copiar Directivas de Corrección». Pega el texto en tu conversación con la IA, o pide al agente que
use mmcelt_get_human_feedback por MCP. La IA recibirá un informe que le indica que acate tus
correcciones y descarte las vías canceladas.

Asistencia para nuevos usuarios

Si estás empezando a usar MMCelt junto a Claude, activa las galletas de ayuda en el menú Ayuda. Te
guiarán paso a paso sobre cómo trabajar con los nodos, y podrás pulsar +info en cualquier momento
para desplegar la documentación completa en el panel lateral derecho.
