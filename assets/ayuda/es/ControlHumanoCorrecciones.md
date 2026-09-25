# 🛑 Control Humano y Directivas de Corrección (Human-in-the-Loop)

Convierte a MMCelt en tu **consola de supervisión y veto** frente a lo que la IA propone.

### 🛡️ El Flujo de Corrección:
1. La IA te presenta un mapa mental o un plan de trabajo.
2. Si detectas un error arquitectónico o una librería que no deseas:
   - Cambia el nodo a **`⛔ Descartado`** o **`⚠️ Requiere Corrección`**.
   - En el campo **"🛑 Corrección / Instrucción Exigida a la IA"**, escribe tu orden explícita (ej. *"No usar librerías externas, implementar con la librería estándar de Rust"*).
3. Abre el menú **`🤖 Inteligencia Artificial` > `🛑 Enviar Correcciones y Directivas a la IA...`**.
4. Pulsa **`📋 Copiar Directivas de Corrección`** y pégalo en tu chat con la IA (o deja que tu agente de consola, como Claude Code, Codex CLI o Gemini CLI, las consulte directamente mediante la herramienta MCP `mmcelt_get_human_feedback`).
5. La IA leerá las órdenes de veto obligatorio y reajustará su código de inmediato.

### ✅ Aprobar un nodo, y qué pasa después
Cuando das por bueno un nodo, ponle **`🛡️ Aprobado por Usuario`** en el inspector. Es tu firma sobre ese contenido. Tanto los controles de supervisión humana como los de prioridad y estado muestran iconos homogéneos en el menú y en el propio nodo en el lienzo.

**Esa firma caduca sola.** Si más tarde la IA cambia ese nodo —sus notas, su estado, su prioridad o su ruta de archivo—, el nodo vuelve automáticamente a **`⏳ Pendiente de Revisión`**.

El motivo es simple: aprobaste un texto concreto, no el nodo para siempre. Sin esta caducidad verías tu propio visto bueno sobre algo que no has leído, que es justo lo contrario de para lo que sirve este programa.

Repetir el mismo valor no cuenta como cambio, así que una sincronización rutinaria de la IA que no toca nada **no** te obliga a revisar de nuevo lo que ya diste por bueno.

Lo que escribiste en el campo de corrección **nunca se borra**, aunque la IA acate la orden: así puedes comprobar si de verdad hizo lo que le pediste.
