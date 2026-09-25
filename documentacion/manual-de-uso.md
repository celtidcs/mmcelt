# Manual de uso de MMCelt

Guía práctica para usar la aplicación. Si buscas el catálogo de funcionalidades, está en
[`funcionalidades.md`](funcionalidades.md); si buscas cómo instalarla, en
[`infraestructura/guia-de-clonado.md`](infraestructura/guia-de-clonado.md).

---

## 1. La idea en una frase

MMCelt no es un programa de mapas mentales al que se le haya añadido inteligencia
artificial. Es una **herramienta de dirección**: el mapa existe para que puedas decirle a
un modelo qué quieres, qué has decidido y qué no debe volver a proponer.

Si lo usas solo para ordenar ideas, funciona. Pero le sacarás la mitad del partido.

---

## 1 bis. Mi primera vez con MMCelt — guía de principio a fin

Esta sección es para quien nunca ha hecho un mapa mental ni ha trabajado con una IA que edite sus
archivos. Va de cero a haber recibido el primer trabajo de un agente y haberle pedido cambios.
**No hace falta leer nada más antes.** Media hora larga, sin prisa.

Si algo no te sale, no es que lo estés haciendo mal: sáltalo y sigue. Casi todo se puede rellenar
después.

### Paso 1. Abre el programa y mira lo que hay

Al abrir verás un mapa de ejemplo: una idea central y tres ramas colgando. No es un tutorial que
haya que completar ni algo que debas conservar; es solo para que la pantalla no esté vacía.

Prueba a moverte antes de tocar nada:

- **Arrastra el fondo** para desplazarte por el lienzo.
- **Rueda del ratón** para acercar y alejar.
- **Clic en un nodo** para seleccionarlo. El panel de la derecha —el **inspector**— pasa a
  mostrar sus datos.

Si el texto se te queda pequeño, `Ctrl` `+` agranda letras, botones y paneles de golpe.

### Paso 2. Empieza tu propio mapa

`📁 Archivo` → `📄 Nuevo Mapa Mental`.

Ahora tienes una sola idea central. Haz clic sobre ella y escribe **de qué va tu proyecto**. Una
frase corta basta: «Tienda online de cerámica», «Reforma de la casa del pueblo», «Sistema de
reservas».

**Elige algo que conozcas bien.** Al final vas a tener que juzgar si lo que la IA responde vale
algo, y eso solo se puede hacer sobre un tema que domines.

### Paso 3. Levanta la estructura con el teclado

Aquí está el único truco que hay que aprender, y son dos teclas:

| Tecla | Qué hace |
|---|---|
| **`Tab`** | Crea un **hijo**: algo que cuelga de lo seleccionado |
| **`Enter`** | Crea un **hermano**: algo al mismo nivel |

El flujo es siempre igual: seleccionas un nodo, pulsas `Tab` o `Enter`, escribes el título, pulsas
`Enter` para confirmar, y sigues. **Se puede construir un mapa entero sin soltar el teclado.**

Haz tres o cuatro ramas principales con dos o tres hijos cada una. No busques que quede perfecto:
un mapa se reordena en cualquier momento.

> **Si las teclas no responden**, mira si tienes el cursor dentro de un campo de texto. Mientras
> escribes, `Tab` y `Enter` hacen lo propio de escribir. Haz clic en el fondo del lienzo y vuelve
> a intentarlo.

### Paso 4. Rellena los detalles del nodo y consulta las galletas

Cada nodo del mapa es mucho más que su título. Selecciona uno y mira el inspector de la derecha.
Además, para acompañarte al empezar, MMCelt muestra **galletas de ayuda** flotantes con consejos
útiles. Si en algún momento las descartas y deseas volver a verlas, puedes reactivarlas desde
`❓ Ayuda` → `🍪 Galletas de Ayuda para Principiantes`.

Esto es lo que puedes rellenar en el inspector, **de más útil a menos**:

| Campo | Para qué sirve | ¿Merece la pena el primer día? |
|---|---|---|
| **Notas** | Explicar a fondo esa idea: contexto, requisitos, dudas | **Sí, es el más importante** |
| **Estado** | En qué punto está: `💡 Idea`, `🔍 Investigando`, `⏳ En Progreso`, `❓ Duda / Bloqueo`, `✅ Completado`, `⛔ Descartado` | Sí, es un clic |
| **Etiquetas** | Agrupar por tema, cruzando ramas | Cuando el mapa crezca |
| **Prioridad** | Nivel de urgencia: `🔽 Baja`, `🔷 Media`, `⚡ Alta`, `🔥 Crítica` | Cuando haya que decidir |
| **Rol** | Qué papel juega en la arquitectura | Si el proyecto es de software |
| **Ruta de archivo** | Enlazar el nodo con un archivo de código real | Solo en proyectos de código |

**Dedica un momento a las notas de dos o tres nodos.** Es el campo que la IA lee con más atención,
y la diferencia entre un nodo titulado «Pagos» y ese mismo nodo con un párrafo explicando qué
pasarela quieres y por qué es enorme cuando llega la respuesta.

Escribe las notas como si se las explicaras a alguien que entra hoy en el proyecto. Es exactamente
lo que es la IA cada vez que abre tu mapa: alguien nuevo, sin memoria de ayer.

### Paso 5. Cruza dos ramas

Llega un momento en que dos ideas de ramas distintas se necesitan. Para eso están las conexiones
cruzadas:

1. Selecciona el nodo que **necesita** algo.
2. Pulsa **`🔗 Conexión`** en el inspector, junto a `➕ Hijo` y `➕ Hermano`.
3. Elige el nodo de destino, el tipo —`➡️ Dependencia (Requiere)` es el más habitual— y **escribe por qué**.

El motivo importa: sin él, la IA sabe que hay una relación pero no qué hacer con ella.

### Paso 6. Cuéntale a la IA qué pretendes

Este es **el paso que más cambia el resultado**, y el que más gente se salta.

`🤖 Inteligencia Artificial` → `🧭 Proyecto e instrucciones para la IA`. En la pestaña Proyecto hay cuatro campos:

- **Visión:** qué intentas construir y por qué. En prosa, dos o tres frases.
- **Objetivos:** qué resultados concretos buscas. Hitos, no deseos.
- **Contexto / público:** para quién es y dónde tiene que funcionar.

Sin esto, la IA ve tu estructura pero no tu intención, y rellena los huecos suponiendo. Un mapa
con estructura impecable y visión vacía consigue que la IA acierte con el «cómo» y falle con el
«para qué».

### Paso 7. Guarda

`Ctrl + S`. Elige una carpeta para el proyecto. A partir de aquí el programa guarda solo mientras
trabajas.

### Paso 8. Manda el mapa a un agente

`🤖 Inteligencia Artificial` → `📤 Enviar a...`, y elige uno de los agentes que MMCelt haya
encontrado instalados en tu equipo.

**Pulsarlo no manda nada todavía.** Se abre una vista previa con **todo** el texto que va a salir,
repartido en cuatro bloques:

| Bloque | ¿Puedes editarlo? | Qué es |
|---|---|---|
| **Contrato MMCelt** | No | Las reglas mínimas, iguales para todos |
| **Reglas comunes del proyecto** | **Sí** | Tu forma de trabajar, la que se repite siempre |
| **Contexto del mapa** | No | Lo genera tu mapa; para cambiarlo, cambia el mapa |
| **Encargo de esta sesión** | **Sí** | Qué quieres conseguir **ahora** |

**Léela entera al menos esta primera vez.** Es la única ocasión de ver con tus ojos todo lo que
sale de tu ordenador antes de que salga.

En **«Encargo de esta sesión»**, pide algo pequeño y concreto para empezar. Por ejemplo:

> «Revisa la rama de Pagos y propón qué me falta por decidir. No modifiques nada más.»

Cuando estés listo, el último botón arranca el agente: dirá `Iniciar en consola` o `Preparar para
MCP` según el destino.

### Paso 9. Mira volver el trabajo

El agente trabaja y devuelve el mapa modificado. **No tienes que importar nada a mano**: MMCelt
vigila el archivo y recoge los cambios.

Cuando lleguen, revisa qué ha cambiado. Y ten presente una cosa importante: **tu trabajo no se
pierde**. Si tú habías tocado el mapa a mano, el programa deja una copia de seguridad antes de
sobrescribir.

### Paso 10. Corrige lo que no te convenza

Aquí es donde MMCelt deja de ser un editor de mapas y pasa a ser una forma de dirigir a la IA.

Selecciona un nodo que no te guste y, en el inspector, marca su **estado de revisión**:

- **`🛡️ Aprobado por Usuario`** — esto está bien, no lo toques más.
- **`⚠️ Requiere Corrección`** — y **escribe qué hay que cambiar** en el campo de corrección.
- **`⛔ Descartado`** — esta vía no me interesa, no insistas.

Lo que escribas ahí **manda sobre todo lo demás**. Cuando dos instrucciones se contradicen, el
orden es: tus correcciones primero, después el contrato de MMCelt, después las reglas del
proyecto y, en último lugar, el encargo de la sesión.

### Paso 11. Devuélvele las correcciones

`🤖 Inteligencia Artificial` → `🛑 Enviar Correcciones y Directivas a la IA...`.

Esto genera un texto que contiene **solo** tus directivas, no el mapa entero. Es la forma de decir
«esto no, esto sí» sin volver a mandarlo todo ni repetir la conversación.

Y con eso se cierra el círculo: mapa → intención → agente → resultado → correcciones → agente.

### Ya está. Qué mirar después

Con esto ya sabes usar el programa. Cuando quieras profundizar:

- **Sección 3** para construir mapas más rápido.
- **Sección 5** si quieres que la IA edite tu mapa directamente, sin copiar y pegar.
- **Sección 8** para las conexiones cruzadas a fondo.
- **Sección 6** para dirigir a la IA cuando se desvía.

Y una recomendación que vale para siempre: **si algo de lo que devuelve la IA te chirría, hazle
caso a esa sensación.** Marca el nodo con `⚠️ Requiere Corrección` y explica por qué. Ese es
justamente el trabajo que ninguna máquina puede hacer por ti, y es la razón de que este programa
tenga un botón para ello.

---

## 2. Primer arranque

Al abrir la aplicación encontrarás un mapa de ejemplo con una idea central y tres
pilares. No es un tutorial que haya que completar: es un punto de partida que puedes
editar directamente.

**Lo primero que conviene hacer** es rellenar la visión del proyecto. Menú
`🤖 Inteligencia Artificial → 🧭 Proyecto e instrucciones para la IA`, o el botón «🧭 Editar Visión»
del panel lateral.

Son tres campos, y encabezan el documento que leerá la IA:

| Campo | Qué escribir |
|---|---|
| **Visión del creador** | Qué intentas construir y por qué. En prosa, sin telegrafiar |
| **Objetivos** | Resultados concretos y medibles |
| **Contexto / público** | Para quién es y en qué entorno se usará |

Es el paso que más diferencia produce en la calidad de lo que devuelve el modelo. Un mapa
con estructura impecable y visión vacía hace que la IA acierte con el «cómo» y falle con
el «para qué».

---

## 3. Construir el mapa

### Flujo básico

1. Selecciona un nodo con un clic.
2. `Tab` crea un hijo; `Enter` crea un hermano.
3. Escribe el título y pulsa `Enter`.
4. Repite.

Se puede construir un mapa entero sin soltar el teclado.

Los atajos que tocan el mapa —`Tab`, `Enter`, `Supr`, `Retroceso`, `Espacio`, `F2` y
`Ctrl + N`— solo actúan con el teclado libre. En cuanto se escribe en un campo de texto, esas
teclas hacen lo propio de escribir: `Retroceso` borra una letra, no el nodo.

`Ctrl + S`, `Ctrl + E` y `Ctrl + F` sí siguen valiendo mientras se escribe, porque no alteran
el mapa: guardan, exportan y centran la vista.

### Empezar de cero, cargar un ejemplo o abrir otro mapa

Todo lo que sustituye el mapa abierto —`Ctrl + N`, los ejemplos, las plantillas, escanear una
carpeta, abrir un archivo e importar de una IA— deja antes una **copia de recuperación** del
que había. Si te das cuenta de que no querías perderlo, cierra MMCelt y vuelve a abrirlo: el
programa te ofrecerá recuperarlo.

Esa copia es una red, no un sustituto de guardar: es única, así que la siguiente sustitución
la reemplaza, y en cuanto empieces a trabajar sobre el mapa nuevo pasará a copiarse ese. La
costumbre sana sigue siendo `Ctrl + S` antes de cambiar de mapa.

### Añadir profundidad

El título dice **qué** es un nodo. Lo que hace útil el mapa para una IA es el resto:

- **Notas**: el campo más importante. Es donde explicas el detalle técnico, las
  restricciones y lo que has considerado. Escribe frases completas, no palabras sueltas.
- **Ruta de archivo**: si el nodo corresponde a código real (`src/auth/jwt.rs`), indícalo.
  La IA sabrá entonces dónde debe operar.
- **Etiquetas**: para clasificar transversalmente (`backend`, `seguridad`, `urgente`).
- **Estado y prioridad**: dónde está cada cosa y qué es urgente. Ambos campos cuentan con iconos homogéneos en los selectores del inspector y se pintan directamente sobre el lienzo antes del título: Prioridad (`🔽 Baja`, `🔷 Media`, `⚡ Alta`, `🔥 Crítica`) y Estado (`💡 Idea`, `🔍 Investigando`, `⏳ En Progreso`, `❓ Duda / Bloqueo`, `✅ Completado`, `⛔ Descartado`).
- **Control humano**: supervisión del trabajo de la IA (`🤖 Generado por IA`, `⏳ Pendiente de Revisión`, `🛡️ Aprobado por Usuario`, `⚠️ Requiere Corrección`). La aprobación caduca de forma automática a `⏳ Pendiente de Revisión` si la IA modifica el contenido del nodo.

### Buscar y filtrar nodos

En el panel lateral dispones del buscador de nodos (`🔍 Buscar por título o etiqueta`):

- **Búsqueda por texto**: localiza términos en títulos o etiquetas.
- **Filtrado por estado y prioridad**: permite aislar rápidamente, por ejemplo, solo los nodos en `⏳ En Progreso` o con prioridad `🔥 Crítica`.
- **Ordenación**: puedes ver los resultados en el orden natural del mapa o clasificados por prioridad (de mayor a menor urgencia), generando una lista clara de trabajo pendiente.

### Marcar las dudas

Es la funcionalidad más infrautilizada, y una de las más útiles.

Cuando no sepas algo, **créalo como nodo** con estado `❓ Duda / Bloqueo` y rol
`❓ Hipótesis / Duda`. Por ejemplo: «¿PostgreSQL o SQLite?», con una nota que explique el
compromiso que estás evaluando.

Al exportar, esas dudas se agrupan en una sección propia del documento, con el
encabezado «PUNTOS DE DECISIÓN, DUDAS Y BLOQUEOS A RESOLVER». La IA las recibe como una
lista concreta de cosas que resolver, no como comentarios dispersos.

---

## 4. Flujo de trabajo A — Con copiar y pegar

El más sencillo. Funciona con cualquier modelo, sin instalar nada.

### 4.1 Del mapa a la IA

1. Puedes previsualizar el documento con `🤖 Inteligencia Artificial → 👁️ Previsualizar Markdown de IA (.md)...` para ver el texto antes de guardarlo.
2. Pulsa `Ctrl + E` o ve a `📁 Archivo → 🤖 Exportar Markdown para IA (.md)` (también en `🤖 Inteligencia Artificial → 💾 Exportar Archivo .md para IA...`) para guardar el archivo `.md`.
3. Elige dónde guardarlo (o cópialo directamente al portapapeles desde la ventana de previsualización).
4. Pégalo en ChatGPT, Claude o Gemini, y añade tu petición:

   > «Aquí tienes el mapa mental de mi proyecto. Analízalo y propón un plan de ejecución
   > resolviendo las dudas abiertas de la sección 5.»

### 4.2 De la IA al mapa

1. Menú `🤖 Inteligencia Artificial → 📋 Copiar Prompt Maestro para IA...`.
2. Pégalo en el modelo junto con lo que quieras convertir: una conversación, un documento,
   un fragmento de código.
3. El modelo devuelve un bloque JSON.
4. En MMCelt, `🤖 Inteligencia Artificial → 📥 Importar desde IA (ChatGPT, Claude, Gemini)...`, pega la respuesta y pulsa **Convertir**.

No hace falta que limpies la respuesta: el importador localiza el bloque JSON aunque
venga rodeado de texto conversacional.

**Si la importación falla**, el mensaje explica qué falló en cada intento de
interpretación. Lo más habitual es que el modelo devolviera prosa en lugar del JSON: pide
que genere **únicamente** el bloque.

### 4.3 Intercambio mediante formatos abiertos (OPML y FreeMind .mm)

Además de trabajar con Markdown y JSON nativo, MMCelt permite intercambiar mapas con cualquier herramienta externa de mapas mentales o editores de esquemas:

- **Exportar esquemas**:
  - `📁 Archivo → 🧭 Exportar a OPML (.opml)...`: Guarda el mapa como esquema jerárquico XML estándar.
  - `📁 Archivo → 🧭 Exportar a FreeMind (.mm)...`: Guarda el mapa en formato estándar FreeMind/Freeplane.
  En ambos casos, los datos propios de MMCelt (estado, prioridad, rol, supervisión humana, rutas y conexiones cruzadas) viajan codificados en JSON estructurado al pie de las notas de forma transparente.

- **Importar esquemas**:
  - `📁 Archivo → 🧭 Importar desde OPML (.opml)...`
  - `📁 Archivo → 🧭 Importar desde FreeMind (.mm)...`
  Si el mapa fue exportado previamente por MMCelt, se reconstruye de forma idéntica sin perder ningún dato ni conexión lateral.
  Si el mapa proviene de otra aplicación externa (sin metadatos MMCelt):
  1. Se asignan valores seguros por defecto (estado `💡 Idea`, prioridad `🔷 Media`, rol `📌 Subtema / Módulo` y raíz `🎯 Idea Central`).
  2. **Disposición visual automática**: MMCelt distribuye las ramas equilibradamente de inmediato para que los nodos no aparezcan solapados en el centro `[0, 0]`, dejándolo listo para leer y editar desde el primer instante.
  3. Se muestra un diálogo informativo transparente explicando los atributos adaptados.

---

## 5. Flujo de trabajo B — Con el servidor MCP

Más potente: el agente lee y escribe el mapa directamente, sin intervención manual.

La instalación está en
[`infraestructura/servicios.md`](infraestructura/servicios.md#servidor-mcp).

### Cómo se trabaja

1. Construyes el mapa en MMCelt y lo guardas dentro de tu espacio de trabajo.
2. Le dices a tu agente: *«Lee el mapa mental de `proyecto.mmcelt` y empieza por los
   nodos marcados como críticos.»*
3. El agente trabaja y va actualizando estados con `mmcelt_sync_ai_progress`. Con esa misma
   herramienta puede **ampliar tu mapa**: añadir nodos con su rol y sus etiquetas y trazar
   conexiones cruzadas entre ramas. No borra ni mueve nada de lo tuyo, y todo lo que añade
   queda marcado como generado por la IA para que lo revises.
4. **Ves el avance en pantalla, sin recargar nada** (ver el punto 5 bis).
5. Si se desvía, marcas nodos como `⚠️ Requiere Corrección` con tu instrucción, y le
   dices: *«Consulta `mmcelt_get_human_feedback` y corrige el rumbo.»*

El Markdown exportado incluye una sección final «Cómo devolver tu trabajo a este mapa». Allí el
agente recibe el recorrido completo de las seis herramientas, los valores exactos de estados,
roles, prioridades y relaciones y la regla de usar UUID cuando un título se repite. No necesita
deducir el contrato ni editar el `.mmcelt` directamente.

El bucle de corrección es lo que distingue este flujo. No tienes que reexplicar el
proyecto entero: marcas lo que está mal, en el mapa, y el agente lo recoge.

### La carpeta de trabajo

Es la única dentro de la cual el agente puede leer y escribir, y se elige en
`🤖 Inteligencia Artificial → 🔌 Conectar MMCelt con mis IAs...`.

**Mientras no elijas ninguna, MMCelt propone la del mapa que tengas abierto**, no una
carpeta fija: autorizar `Documentos/MapasMentales` cuando tu proyecto está en otro sitio le
da al agente permiso para trabajar donde no hay nada. Si eliges una a mano, manda esa.

El agente no tiene que adivinar nada: al conectarse, el servidor le dice en el saludo cuál
es la carpeta autorizada, y con `mmcelt_workspace_info` puede pedir la lista de mapas
`.mmcelt` que hay dentro, con las rutas exactas que después debe reenviar.

**Si cambias la carpeta, los agentes ya conectados no se enteran solos.** La carpeta va
escrita dentro del archivo de configuración de cada agente. Cuando dejan de coincidir, la
ventana deja de decir «conectado», te avisa de a qué carpeta apunta cada uno y te ofrece el
botón **Actualizar la carpeta**, que vuelve a registrarlo con la que estás viendo y deja
copia de seguridad. Nada se reescribe sin que lo pidas, y después hay que reiniciar el
agente.

---

## 5 bis. Enviar el mapa y verlo volver

Desde la **0.9.0** no hace falta llevar el mapa a mano al modelo ni traer su respuesta. Desde la
**0.10.0** además puedes **ver y corregir el prompt antes de que salga**.

### Prepararlo sin enviar nada

Abre `🤖 Inteligencia Artificial → 🧭 Proyecto e instrucciones para la IA`. La primera pestaña, **Proyecto**, es la
única que necesitas al empezar: contiene visión, objetivos, público y autor. Guardar esos datos no
obliga a configurar ningún agente.

El pie común de las cuatro pestañas permite elegir el **Idioma del documento para la IA**,
independiente del idioma de los menús. El cambio se guarda de inmediato para ese proyecto: no hace
falta pulsar `Guardar como reglas comunes` y, al cerrar y volver a abrir la ventana, se conserva la
elección. El contrato y las plantillas de MMCelt sí cambian de idioma; tu visión, tus reglas y tu
encargo permanecen como los escribiste. **Restaurar plantilla recomendada** propone unas reglas
prudentes, pero no las guarda hasta que pulses el botón de guardado, también situado en el pie
común y accesible desde cualquier pestaña.

También aparecen `AGENTS.md`, `CLAUDE.md` y `GEMINI.md` si existen. Una fuente nueva no se incorpora
por sorpresa. Tras leerla, **Aceptar esta versión** guarda localmente su contenido y su huella en
`.mmcelt/configuracion-agente.json`. Si cambia un solo byte, queda desmarcada y se muestran la
versión aceptada anterior y la actual. Si no existe ninguna, la pestaña muestra la carpeta exacta
donde buscará esos tres nombres: crea allí el archivo manualmente; MMCelt no lo crea ni lo modifica.
Esto no guarda credenciales ni modifica el archivo original.

**Plantillas de encargo** copia una sola ayuda al encargo editable; no acumula las tres órdenes.
**Vista completa** reúne contrato, reglas, contexto y ese único encargo mediante el mismo compositor
que usa la confirmación final. Cerrar esta ventana conserva el encargo preparado durante la
ejecución actual; al elegir un agente aparece ya en su borrador final.

### Cómo se manda

`🤖 Inteligencia Artificial → 📤 Enviar a...`, y eliges uno de los agentes que MMCelt encuentre
instalados en tu equipo. **Pulsarlo no manda nada todavía**: se abre una vista previa con el
prompt completo, repartido en cuatro bloques.

1. **Contrato MMCelt** (solo lectura): las reglas mínimas de integridad, las mismas para todos.
2. **Reglas comunes del proyecto** (puedes editarlo): tu perfil neutral.
3. **Contexto del mapa** (solo lectura): lo genera la exportación para IA; se cambia en el mapa.
4. **Encargo de esta sesión** (puedes editarlo): qué quieres conseguir ahora.

El propio prompt declara qué manda cuando dos instrucciones se contradicen: **correcciones
humanas del mapa > contrato MMCelt > reglas del proyecto > encargo de la sesión.**

Debajo tienes cuatro botones. `Restaurar esta sesión` deshace lo que hayas cambiado en los dos
bloques editables. `Guardar como reglas comunes` es una acción aparte, y es la **única** que
escribe tu perfil en `.mmcelt/instrucciones-agente.md`. `Cancelar` no guarda, no exporta, no
vigila y no deja rastro. El último botón dice `Iniciar en consola` si existe una CLI o `Preparar
para MCP` si el destino no tiene consola oficial. El pie permanece visible en pantallas pequeñas;
los bloques editables y la vista completa se desplazan de forma independiente por encima.

Si en la raíz del proyecto tienes `AGENTS.md`, `CLAUDE.md` o `GEMINI.md`, la vista previa los
enseña separados, indica si son nuevos, aceptados o modificados, y permite incluirlos o excluirlos
en esta sesión. Solo una versión aceptada sin cambios aparece preseleccionada.

Al pulsar `Iniciar en consola`, el programa hace cinco cosas seguidas, y cada una tiene que salir
bien antes de la siguiente:

1. **Guarda el mapa**, para que el modelo lea exactamente lo que tú estás viendo.
2. Exporta el documento Markdown para la IA, junto al mapa y con el sufijo `_AI.md`.
3. **Escribe el expediente** de la sesión en `.mmcelt/sesiones`.
4. **Abre la consola** del agente en la carpeta del proyecto.
5. **Empieza a vigilar el archivo del mapa.**

Si la consola no llega a abrirse, MMCelt **no dice «enviado»**, no empieza a vigilar y deja el
expediente marcado como fallido.

Justo antes de iniciar, MMCelt vuelve a leer el mapa, las reglas guardadas y las fuentes nativas.
Si algo cambió desde que se preparó la vista, la confirmación caduca: no se crea proceso ni un
expediente nuevo hasta que revises otra vez el texto. `inicio.md` conserva después exactamente los
mismos bytes que mostraba la vista completa, no una reconstrucción parecida.

Con un destino **solo MCP** —cuando un agente tiene configurado el servidor MCP pero MMCelt no
localiza su comando de consola en el PATH—, MMCelt guarda, exporta, escribe el expediente e
inicia la vigilancia, pero no ejecuta el paso 4. El expediente queda `Preparado` y se continúa
desde la interfaz del propio agente. La vista previa lo advierte antes de confirmar.

La identidad `.mmcelt/configuracion.json` se crea la primera vez que confirmas el proyecto desde
MMCelt. En una carpeta anterior, abrir y cancelar la vista previa no escribe nada. No edites la
identidad: permite reconocer el proyecto aunque cambie de nombre. Después de mover la carpeta,
vuelve a **Conectar** para confirmar la nueva ubicación.

A partir de ahí trabajas en la ventana del agente —que es donde le das las instrucciones— con
MMCelt a la vista al lado. **Cuando el modelo cambia el mapa, lo ves cambiar en pantalla.**

### Qué queda guardado de cada envío

Cada confirmación crea una carpeta dentro de `.mmcelt/sesiones`, con la fecha, el agente y un
identificador. Dentro hay dos archivos:

- `inicio.md`: el prompt exacto que aprobaste, letra por letra.
- `sesion.json`: los datos de la sesión —formato, identificador, fecha, agente, ruta del mapa,
  fuentes detectadas, tamaño del prompt y estado del lanzamiento—.

No se guardan contraseñas, claves, correos ni identificadores de cuenta. Es material tuyo y local.


### Agentes de consola admitidos (CLI)

MMCelt se integra exclusivamente con los tres agentes oficiales de consola: **Claude Code**,
**Codex CLI** y **Gemini CLI**. Al confirmar el envío («📤 Enviar a...»), MMCelt abre una nueva
ventana de terminal con la sesión preparada, el expediente redactado y la vigilancia activa.

> ⚠️ **Estado de compatibilidad y acceso en Gemini CLI:**
> Sobre el acceso interactivo con cuenta personal de Google conviven tres hechos documentados: el 13 de septiembre de 2026 la consola rechazó el acceso interactivo con cuentas personales estándar individuales; en la prueba humana de la versión 0.11.6 funcionó de principio a fin utilizando una **clave de API gratuita de Google AI Studio** (`https://aistudio.google.com/app/apikey`), creando nodos y conexiones comprobados en disco; y el 18 de septiembre de 2026 la documentación oficial de Gemini CLI (`geminicli.com`) indicó que las cuentas individuales sí tienen acceso.
> Como la compatibilidad interactiva puede variar según la versión del cliente instalada en cada equipo, el uso de la clave de API gratuita mediante `$env:GEMINI_API_KEY="tu_clave"` (PowerShell) o `export GEMINI_API_KEY="tu_clave"` (Bash/Zsh) es la vía comprobada y más directa para trabajar sin fricciones.

**Retirada de conectores gráficos:** Los clientes de entorno gráfico o editores sin consola
oficial (como Antigravity, Claude Desktop, Cursor o Windsurf) han sido retirados del catálogo
por decisión de diseño: el proyecto ofrece únicamente opciones verificadas y
sencillas de manejar, evitando configuraciones engorrosas.

**MMCelt no pide ni guarda credenciales.** Se apoya en la sesión que ya tengas autenticada en
`claude`, `codex` o `gemini`. No usa ninguna API de pago propia, no elige modelo y no decide facturación.

### Tabla de agentes y recorridos

| Ecosistema | Cliente | Quién lo abre | Qué haces después |
|---|---|---|---|
| Anthropic | **Claude Code** (consola) | MMCelt, al confirmar `📤 Enviar a... → Claude Code` | Continúas escribiendo en la consola nueva. |
| OpenAI | **Codex CLI** (consola) | MMCelt, al confirmar `📤 Enviar a... → Codex CLI` | Continúas escribiendo en la consola nueva. |
| Google | **Gemini CLI** (consola) | MMCelt, al confirmar `📤 Enviar a... → Gemini CLI` | Continúas escribiendo en la consola nueva (con sesión o clave de API). |
| Cualquiera | **Chat web ordinario** | Tú, en el navegador | Adjuntas o pegas el `_AI.md`; no tiene acceso al MCP local. |

No hace falta abrir antes una CLI: MMCelt la abre al confirmar el envío. Sí conviene tener
autenticado el cliente, porque MMCelt usa su sesión existente.

### En OpenAI: ChatGPT web y Codex CLI

No son dos nombres para la misma ventana. En una conversación ordinaria de **ChatGPT web** debes
adjuntar el `_AI.md` o usar la copia manual: el navegador no lee la configuración MCP local y no
puede escribir el mapa de tu disco. **Codex CLI** sí puede cargar el servidor MCP local y es el
destino que MMCelt abre en una consola al confirmar «📤 Enviar a...».

En la práctica, mira siempre dos cosas por separado: el icono de consola confirma si MMCelt puede
abrir una conversación nueva; el icono MCP confirma si el agente puede leer y devolver el mapa.
La explicación completa, con los tres recorridos, está en
[`integraciones/chatgpt.md`](integraciones/chatgpt.md).

El recorrido actual está comprobado en **Windows** —abre una ventana nueva y prioriza los
lanzadores declarados por `PATHEXT` frente a scripts Unix sin extensión—. En **Linux** el
adaptador usa una lista cerrada de terminales, pero la aceptación de esta versión sigue
pendiente. En **macOS** todavía **no** se afirma.

### Qué pasa si tú también estabas tocando el mapa

La regla es una y manda sobre todo lo demás: **tu trabajo no se pierde nunca.**

| Situación | Qué hace MMCelt |
|---|---|
| No tienes cambios sin guardar | Recarga el mapa en el momento. Es el caso normal. |
| **Tienes cambios sin guardar** | **No toca tu mapa.** Te avisa de que la IA cambió el archivo y decides tú. |
| El archivo llega a medio escribir | Lo ignora y espera al siguiente aviso, sin sacar errores. |
| MMCelt guarda por su cuenta (autoguardado) | Reconoce su propia escritura y no se dispara a sí mismo. |

Antes de cualquier recarga se deja una copia de seguridad en el archivo de recuperación.

### Cómo se corta

Mientras la vigilancia está activa, en la barra superior aparece un indicador **`🟢`** junto al
nombre del archivo. Para pararla: `🤖 Inteligencia Artificial → ⏹ Detener seguimiento`.

Merece la pena saber dónde está **antes** de necesitarlo: el día que un modelo se equivoque y
siga equivocándose, es el botón que quieres tener localizado. La vigilancia también termina de
forma limpia al crear un mapa nuevo con `Ctrl + N`, al abrir otro archivo desde el menú `Archivo`
y al cerrar la aplicación.

**La consola del agente es un proceso aparte**: se cierra en su propia ventana. Detener la
vigilancia no cierra la consola, y cerrar la consola no borra el expediente.

### Un consejo para el prompt

Pídele al modelo que **te avise cuando termine** de escribir sus cambios. Te lo dice en su
propio terminal y tú revisas en MMCelt cuando quieras; no hace falta que estés mirando el mapa
mientras trabaja.

Si el agente crea o sincroniza el mapa por MCP, puedes comprobar la entrega en
`.mmcelt/devolucion.json`: contiene el nombre del agente, la ruta relativa del mapa y si fue
generado o modificado. MMCelt lo sustituye con cada nueva entrega; no es el mapa ni una copia de
seguridad. Es una constancia para que tú o el propio agente podáis inspeccionarla. MMCelt detecta
los cambios reales del mapa mediante la vigilancia del archivo y no utiliza este recibo como
permiso automático para recargar: un recibo antiguo no demostraría quién escribió los bytes
actuales.

---

## 6. Dirigir a la IA cuando se desvía

Ocurre siempre en proyectos largos. El procedimiento:

### Marcar lo que está mal

1. Selecciona el nodo problemático.
2. En el inspector, estado de revisión → `⚠️ Requiere Corrección`.
3. Escribe la instrucción concreta en el campo de corrección. Sé específico:

   > «No uses MongoDB. Este proyecto necesita transacciones ACID y consultas relacionales.
   > Usa PostgreSQL.»

### Descartar caminos cerrados

Si una opción queda definitivamente fuera, ponla en estado `⛔ Descartado`. En el
documento exportado, esos nodos llevan una advertencia explícita para que el modelo no
insista.

Es preferible a borrarlos: dejas constancia de que se evaluó y se rechazó, lo que evita
que la IA lo vuelva a proponer como novedad.

### Enviar las correcciones

`🤖 Inteligencia Artificial → 🛑 Enviar Correcciones y Directivas a la IA...` genera un prompt que contiene **solo** las directivas
de rumbo, sin repetir el mapa entero. Está pensado para una IA que ya conoce el proyecto.

---

## 7. Partir de código existente

Si ya tienes un proyecto y quieres refactorizarlo con ayuda de una IA:

1. `📁 Archivo → 🔍 Escanear Carpeta de Código...`.
2. Elige la raíz del repositorio.

MMCelt recorre hasta tres niveles y crea un nodo por cada carpeta y archivo de código,
con su ruta rellenada. Omite `target`, `node_modules`, `dist`, `build` y las carpetas
ocultas.

A partir de ahí, anota tus intenciones sobre la estructura real: qué módulo te da
problemas, cuál quieres extraer, dónde falta cobertura de pruebas.

---

## 8. Conexiones cruzadas

Sirven para lo que la jerarquía no puede expresar: que dos ramas distintas dependen entre
sí.

1. Selecciona el nodo de origen.
2. Pulsa `🔗 Conexión` en el inspector de la derecha, junto a `➕ Hijo` y `➕ Hermano`.
   También está en `✏️ Edición` → `🔗 Crear Conexión Cruzada / Dependencia...`.
3. Elige el destino, el tipo de conexión y escribe **por qué** existe.

Hay cinco tipos, y la elección cambia lo que la IA entiende: `➡️ Dependencia (Requiere)`, `⛔ Bloquea a`,
`🔀 Alternativa a`, `✨ Sinergia con` e `💡 Inspirado por`.

Las conexiones de un nodo se ven en el inspector de la derecha, en «Conexiones Cruzadas», y
desde ahí se borran. Esa sección solo aparece si el nodo tiene alguna.

El motivo importa. «Dependencia» dice poco; «Los costes de hosting condicionan el
precio del plan gratuito» le da a la IA información que no estaba en ninguna otra parte.

---

## 9. Preguntas frecuentes

**Veo la aplicación muy pequeña en mi televisor o pantalla grande.**
Pulsa `Ctrl` `+` varias veces: agranda letras, botones y paneles de golpe. También está en
el menú `🎨 Ver y Diseño → Tamaño de la interfaz`, con el 200 % pensado justo para
televisores. **El ajuste se recuerda**, así que solo hay que hacerlo una vez.

No lo confundas con la rueda del ratón: esa acerca el *mapa*, no la interfaz. Si lo que
ves pequeño son los menús y los botones, es `Ctrl` `+`.

**¿Por qué no se ajusta solo al tamaño de mi pantalla?**
En parte lo hace: si tienes una pantalla 4K sin escalado del sistema, la primera vez se
agranda sola. Pero lo que de verdad determina el tamaño adecuado es **a qué distancia te
sientas**, y eso ningún programa puede saberlo: tu televisor de 42 pulgadas a tres metros
le presenta a Windows los mismos datos que un monitor de 24 pulgadas a medio metro. Por
eso se propone un punto de partida y se recuerda lo que elijas.

**¿Por qué las líneas del mapa tienen colores distintos?**
Cada rama principal tiene su color, y toda su descendencia lo hereda. Sirve para seguir un
hilo hasta el final aunque el mapa esté enredado. Las conexiones cruzadas van siempre en
ámbar, para distinguirlas de la jerarquía. El detalle completo está en
[`funcionalidades.md`](funcionalidades.md#3-qué-significan-los-colores).

**Aprobé un nodo y ahora aparece como pendiente. ¿Por qué?**
Porque la IA lo ha cambiado después. La aprobación caduca en cuanto el contenido del nodo
deja de ser el que aprobaste: sus notas, su estado, su prioridad o su ruta de archivo. Lo
que firmaste fue un texto, no el nodo para siempre.

Si la IA envía los mismos valores que ya había, no cuenta como cambio y la aprobación se
mantiene, de modo que una sincronización rutinaria no te obliga a revisar otra vez lo que ya
diste por bueno.

**Al importar la respuesta de la IA, ¿pierdo el mapa que tengo abierto?**
Sí. Importar **sustituye** el mapa, no lo añade al que ya tienes. Antes de hacerlo, MMCelt
guarda una copia de seguridad del mapa anterior: si te arrepientes, cierra el programa y
vuelve a abrirlo, y al arrancar te ofrecerá recuperarlo. Aun así, lo sensato es guardar con
`Ctrl+S` antes de pegar nada.

Tampoco viaja la presentación: las posiciones que hayas colocado a mano y las ramas que
tuvieras plegadas se pierden, porque la IA devuelve la estructura y el contenido, no la
disposición. El mapa se recoloca solo.

**¿Puedo cambiar el aspecto de la aplicación?**
Sí, en `🎨 Ver y Diseño → Tema Visual` hay tres: **oscuro** (el de partida, azul pizarra),
**claro** (papel cálido, con el fondo en marfil en lugar de blanco) y **alto contraste**
(negro con bordes blancos, para problemas de visión o pantallas con reflejos). El tema
cambia la ventana entera —lienzo, menús, paneles y diálogos— y **se recuerda al
reiniciar**.

**¿Puedo trabajar sin conexión a internet?**
Sí, siempre. La aplicación nunca se conecta a la red. Solo la necesitas para hablar con
el modelo, y eso ocurre en tu navegador o en tu agente, no en MMCelt.

**¿Dónde se guardan mis mapas?**
Donde tú elijas. No hay almacenamiento en la nube ni base de datos: son archivos `.mmcelt`
normales que puedes copiar, versionar o sincronizar como cualquier otro.

**Me dice que el archivo está dañado y no lo abre.**
Ha detectado que el mapa describe una estructura imposible (un ciclo, una raíz que no
existe, referencias a nodos ausentes). El mensaje indica el nodo concreto. Se rechaza a
propósito: cargarlo colgaría la aplicación. Si el archivo lo generó una IA, pídele que lo
regenere; si hay un `.bak` al lado, prueba con él.

**La IA devuelve JSON que no se importa.**
Suele ser porque añadió comentarios dentro del JSON o dejó una coma final, cosas que el
formato no admite. Pídele que lo genere de nuevo «como JSON estrictamente válido, sin
comentarios».

**¿Puedo tener nodos con el mismo título?**
Sí, sin salvedades. En un diagrama de decisiones es lo normal: dos **Sí** y dos **No**
colgando de preguntas distintas.

Una IA que pida un cambio nombrando solo el título ya no acaba en el nodo que no era: si
ese título lo llevan varios nodos, MMCelt **rechaza la petición entera sin escribir nada**
y le devuelve los candidatos con su identificador, para que repita la llamada señalando el
que quería. Cada nodo y cada conexión que lee llevan ya ese identificador.

**¿Cuántos nodos aguanta?**
El límite es de 100 000 nodos y 512 niveles de profundidad. Muy por encima de lo que
resulta legible: por encima de unos pocos cientos conviene dividir en varios mapas.

**¿Qué es el archivo `mmcelt-errores.log`?**
El registro de errores, junto al ejecutable. Solo aparece si ha ocurrido alguno. Es útil
para diagnosticar un problema; se puede borrar sin consecuencias.

**Al abrir me pregunta si quiero recuperar trabajo sin guardar. ¿Qué es eso?**
La aplicación guarda una copia de seguridad cada dos minutos. Si la sesión anterior
terminó sin que guardaras —un apagón, un cierre por error—, al volver te ofrece esa copia.
Te dice de cuándo es y qué mapa contiene, y decides tú. Si cerraste a propósito sin
guardar, descártala.

**¿Dónde se guardan esas copias automáticas?**
En la carpeta de datos de MMCelt (`%APPDATA%\MMCelt\recuperacion\` en Windows), nunca
junto a tus archivos: así no te aparecen archivos sueltos en tus proyectos. Solo se
conserva la última sesión, y se borra en cuanto guardas de verdad.

**¿El guardado automático sustituye a `Ctrl+S`?**
No, y es importante. Es una red de seguridad ante un cierre inesperado, no un lugar donde
tu trabajo esté guardado. Sigue guardando tú.

---

## 10. Recomendaciones

**Escribe las notas para alguien que no conoce el proyecto.** Ese es exactamente el punto
de partida del modelo en cada conversación nueva.

**Actualiza los estados mientras trabajas.** Un mapa donde todo sigue en «Idea» tres
semanas después no le dice nada a la IA. Los estados son la señal principal de por dónde
va el trabajo.

**Usa las dudas como lista de decisiones pendientes.** Es la sección del documento
exportado que más rendimiento da.

**Guarda a menudo.** `Ctrl + S`. Si el guardado falla, la aplicación te lo dirá: no
informa de éxito sin comprobarlo.

**Un mapa por proyecto, no por tarea.** El valor está en que la IA vea el conjunto.
