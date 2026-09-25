# 🟣 Servidor MCP y agentes de IA

Un puente directo y seguro para que las inteligencias artificiales lean y creen mapas mentales en tu equipo.

### ⚡ Conexión sencilla en un clic

No necesitas editar archivos de configuración complicados. Puedes conectar tus agentes desde la propia aplicación:

1. Ve al menú `🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...`.
2. MMCelt buscará automáticamente qué asistentes de terminal tienes instalados en tu equipo (Claude Code, Codex CLI y Gemini CLI).
3. Pulsa el botón **Conectar** junto al asistente que quieras usar. ¡Y listo!

Si instalas un asistente nuevo más adelante, abre de nuevo esa ventana: la búsqueda se realiza cada vez que entras. El estado «Conectado» te garantiza que el asistente sabe dónde encontrar a MMCelt y cómo comunicarse con él mediante `--mcp-server`.

### ⚙️ Configuración manual y servidor embebido

Para configurar manualmente tu cliente MCP, añade esta entrada en su configuración JSON:
```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/ruta/a/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": { "MMCELT_WORKSPACE": "C:/ruta/a/tu/proyecto" }
    }
  }
}
```

El servidor MCP está **embebido dentro del propio ejecutable de MMCelt**: no requiere ningún entorno de ejecución externo ni dependencias adicionales.

### 🛡️ Seguridad total: tu carpeta de trabajo y copias de seguridad

Tu tranquilidad es lo primero. Cuando un asistente de IA trabaja con MMCelt, **solo tiene permiso para leer y escribir dentro de la carpeta de tu proyecto**. Esta zona delimitada se configura mediante la variable `MMCELT_WORKSPACE`.

Cualquier intento de la IA de acceder a carpetas ajenas de tu disco será rechazado al instante. Y por seguridad adicional:
- **Copias de seguridad automáticas:** Antes de que un agente modifique un archivo existente, MMCelt guarda una copia con la fecha y hora exactas (por ejemplo `proyecto.mmcelt.20260918-120000.bak`). Si algo no te gusta, tu trabajo anterior sigue intacto.
- **Respeto a tus ideas:** La IA puede sugerir nuevas ramas y conexiones, pero **nunca borrará ni moverá** nodos que hayas creado tú. Todo lo que la IA proponga quedará marcado visualmente como «Generado por IA» para que seas siempre tú quien decida si lo apruebas o lo corriges.

### 🔨 Las 6 herramientas oficiales del servidor MCP

El asistente dispone de seis herramientas oficiales diseñadas específicamente para colaborar contigo:

- `mmcelt_workspace_info`: Consulta qué carpeta tiene autorizada y qué mapas existen en ella. Es lo primero que revisa para no inventar rutas.
- `mmcelt_create_mindmap`: Crea mapas mentales nuevos en formato `.mmcelt`.
- `mmcelt_read_mindmap`: Lee la estructura de tu mapa, tus notas y las dudas pendientes.
- `mmcelt_sync_ai_progress`: Desarrolla el mapa añadiendo nodos con sus roles, prioridades y relaciones.
- `mmcelt_get_human_feedback`: Lee tus correcciones y directivas antes de tocar partes sensibles.
- `mmcelt_export_ai_markdown`: Convierte el mapa en un resumen estructurado en Markdown.

### 🌱 Si creaste el mapa tú mismo: respeto total a tu autoría

El agente puede ampliar tu mapa, pero nunca reemplazarlo. Añade nodos con sus roles y etiquetas, y traza conexiones entre ramas separadas.
Todo lo que añade nace marcado como generado por IA, no puede firmar nada como aprobado por ti, y no reconectará un camino que hayas marcado como descartado.

### 🏷️ Nodos con el mismo nombre

En un mapa de decisiones es natural repetir palabras comunes (como «Sí», «No» o «Pendiente») en varias ramas distintas. Si la IA solicita modificar un nodo indicando solo un nombre repetido, MMCelt rechaza la orden ambigua y le devuelve los identificadores únicos de cada coincidencia para que aclare exactamente cuál desea modificar. Así se evitan confusiones indeseadas.

### 🧭 ¿Y si no tienes asistentes instalados en la consola?

No te preocupes: MMCelt es totalmente útil aunque no tengas instalado ningún agente en la terminal. Puedes trabajar con servicios web como ChatGPT, Claude.ai o Gemini en tu navegador usando las opciones `🤖 Inteligencia Artificial` → `📋 Copiar Prompt Maestro para IA...` o `💾 Exportar Archivo .md para IA...`. Luego solo tienes que pegar la respuesta en `🤖 Inteligencia Artificial` → `📥 Importar desde IA (ChatGPT, Claude, Gemini)...` para transformar el texto en nodos visuales al instante.
