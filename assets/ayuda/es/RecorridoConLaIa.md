# 🤖 El recorrido completo con la IA

Los demás temas explican cada pieza por separado. Este cuenta **el viaje entero**: qué sale de tu
ordenador, por qué, qué decides tú y qué puede hacer la IA cuando responde.

Si solo vas a leer un tema sobre la IA, lee este.

### 🗺️ El recorrido en cuatro tiempos

**1. Tú preparas el mapa.** Nada de lo que hagas aquí se manda todavía. Construyes las ramas,
escribes notas, marcas estados y dudas, trazas conexiones cruzadas y rellenas la visión del
proyecto. Cuanto más de lo que tienes en la cabeza esté escrito, menos tendrá que suponer la IA.

**2. El programa compone un documento.** No manda tu archivo `.mmcelt`. Para una sesión automática
reúne cuatro bloques, siempre en el mismo orden: contrato MMCelt, reglas del proyecto, contexto del
mapa y un solo encargo. El mapa entero viaja dentro del contexto, no como un resumen.

**3. Tú lo lees y lo apruebas.** Este es el paso que casi nadie usa y el que más protege. Antes de
que salga nada, ves el texto completo y puedes cambiar dos de sus cuatro bloques.

**4. La IA devuelve trabajo al mapa.** No editando tu archivo, sino a través de seis herramientas
con reglas. Lo que añada nace marcado como generado por IA, y tú decides si lo apruebas.

### 📤 Las dos maneras de que salga

| | Cómo | Cuándo conviene |
|---|---|---|
| **A mano** | `📁 Archivo` → `🤖 Exportar Markdown para IA (.md)`, y subes el archivo al chat | Cualquier modelo, también los de navegador. No hace falta instalar nada |
| **Automática** | `🤖 Inteligencia Artificial` → `📤 Enviar a...` y eliges el agente | Tienes el agente instalado y quieres que el mapa se actualice solo |

La respuesta vuelve con `📥 Importar desde IA (ChatGPT, Claude, Gemini)...` si lo hiciste a mano,
o sola si usaste el envío automático.

### ✍️ Lo que decides tú, y dónde

- **Qué pretendes:** `🤖 Inteligencia Artificial` → `🧭 Proyecto e instrucciones para la IA` → pestaña **Proyecto**.
- **Qué está bien y qué no:** marca los nodos como `🛡️ Aprobado por Usuario` o
  `⚠️ Requiere Corrección`, y mándalas con
  `🤖 Inteligencia Artificial` → `🛑 Enviar Correcciones y Directivas a la IA...`.
- **Ver antes de mandar:** con `🤖 Inteligencia Artificial` → `👁️ Previsualizar Markdown de IA (.md)...`
  puedes leer en pantalla el texto exacto generado antes de enviarlo a una consola o copiarlo a un chat.
- **Qué se le pide esta vez:** en la vista previa del envío, el bloque «Encargo de esta sesión».
- **Cómo quieres que trabaje siempre:** en esa misma vista previa, el bloque de reglas comunes.

Antes del envío también puedes preparar todo desde **`🧭 Proyecto e instrucciones para la IA`**.
La pestaña **Instrucciones** permite elegir el **Idioma del documento para la IA**, guardar reglas
comunes y revisar fuentes nativas. La pestaña **Plantillas de encargo** copia una sola ayuda al
encargo, y **Vista completa** muestra la cadena exacta.

Una fuente nativa nunca entra por el mero hecho de existir. Primero aparece como **Fuente nueva**;
después de **Aceptar esta versión** queda preseleccionada mientras sus bytes no cambien. Si cambia,
se desmarca y enseña la versión anterior junto a la actual. Puedes decidir incluir o excluir una
fuente en la sesión final mediante su casilla.

Si dos instrucciones se contradicen, el propio documento declara el orden de mando, de más fuerte
a más débil: **tus correcciones en el mapa**, el contrato de MMCelt, las reglas del proyecto y,
en último lugar, el encargo de esta sesión.

### 🔙 Qué puede hacer la IA al volver, y qué no

**Puede** leer el mapa, leer tus correcciones, añadir nodos y devolver avances, y crear un mapa
nuevo. Son seis herramientas y ninguna más.

**No puede** editar tu archivo `.mmcelt` por su cuenta, salir de la carpeta de trabajo que le
autorizaste, ni **declarar que tú has aprobado algo**. Todo lo que aporte nace marcado como
generado por IA, y pasar eso a aprobado es una decisión tuya, siempre.

Además queda rastro: el texto exacto que aprobaste se guarda en `.mmcelt/sesiones`, y si un
cambio automático llega sobre un mapa que tú habías tocado, se guarda antes una copia `.bak`.

La confirmación no es una fotografía decorativa. Justo antes de iniciar, MMCelt vuelve a comprobar
el mapa, las reglas y los archivos fuente. Si alguno cambió, muestra que la confirmación ha
caducado, no abre el agente y te pide revisar de nuevo.

### ⚠️ Lo que esto no es

El documento le dice a la IA que el contenido de tu mapa son **datos, no órdenes**. Eso reduce
malentendidos, pero **no es una barrera de seguridad**: un título o una nota que venga de fuera
podría estar redactado como si fuese una instrucción.

Lo que de verdad protege es otra cosa: la carpeta de trabajo autorizada, el hecho de que las
herramientas sean seis y limitadas, y que seas tú quien mira la vista previa antes de que salga
nada. Por eso merece la pena leerla al menos una vez.

**💡 ¿Qué hacer si no tienes agentes instalados en la consola?**
Si al entrar en `🤖 Inteligencia Artificial` → `📤 Enviar a...` lees el mensaje «No se detectaron agentes instalados», no te preocupes: **no necesitas consolas técnicas ni agentes avanzados para usar la IA con MMCelt**.

Puedes trabajar cómodamente desde tu navegador habitual:
1. Usa `📁 Archivo` → `🤖 Exportar Markdown para IA (.md)` (o mira antes el texto con `🤖 Inteligencia Artificial` → `👁️ Previsualizar Markdown de IA (.md)...`).
2. Abre en tu navegador cualquier servicio de chat con IA (ChatGPT, Claude, Gemini...).
3. Pega el texto exportado y dale las instrucciones que desees para continuar el mapa.
4. Cuando la IA te conteste con un bloque en formato JSON o Markdown, cópialo.
5. En MMCelt, pulsa `🤖 Inteligencia Artificial` → `📥 Importar desde IA (ChatGPT, Claude, Gemini)...` y pega la respuesta. Tus nuevas ramas aparecerán en el mapa de inmediato.

Si en el futuro instalas un agente en la terminal de tu sistema (como Claude Code, Codex CLI o Gemini CLI), MMCelt lo reconocerá en cuanto esté en tu PATH.

### 📚 Dónde seguir, según lo que quieras

- **Escribir bien lo que pretendes:** el tema de la visión del creador.
- **Corregir a una IA que se desvía:** el del control humano y las correcciones.
- **Entender el documento que se genera:** el de la exportación de Markdown.
- **Traer a mano una respuesta pegada:** el de la importación desde IA.
- **Mandarlo y verlo volver solo:** el del envío y la vigilancia.
- **Conectar tu agente por MCP:** el tema de 🟣 Servidor MCP y agentes de IA.

Todos están en este mismo selector, en el orden en que se suelen necesitar.
