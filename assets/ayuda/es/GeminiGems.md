# 🔵 Integración con Antigravity CLI (agy)

Google Antigravity es la plataforma avanzada de desarrollo de Google, y **Antigravity CLI (`agy`)** es su herramienta oficial para la consola de comandos que sustituye a la consola anterior Gemini CLI. MMCelt te permite conectar directamente con Antigravity CLI para analizar, expandir y enriquecer tus mapas mentales.

### 💻 Uso con Antigravity CLI (`agy`) en la terminal

1. Ve a `🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...` y pulsa en **Conectar** junto a Antigravity CLI (agy).
2. Abre tu mapa y selecciona `🤖 Inteligencia Artificial` → `📤 Enviar a...` → `Antigravity CLI (agy)`.
3. Revisa la vista previa del documento y del encargo que recibirá la IA.
4. Pulsa **Iniciar en consola**. MMCelt abrirá automáticamente la terminal ejecutando `agy -i` con el entorno preparado y comenzará a vigilar los cambios para incorporar los nodos que el agente cree mediante MCP.

### 💡 Autenticación con cuenta normal de Google

Antigravity CLI funciona de manera nativa con tu **cuenta de Google estándar**:
- No requiere claves de API de Google AI Studio ni variables de entorno adicionales.
- Si no has iniciado sesión previamente en `agy`, la herramienta abrirá el navegador para autenticarte con tu cuenta de Google normal.
- El servidor MCP de MMCelt queda registrado en `~/.gemini/config/mcp_config.json`, permitiendo que el agente lea y modifique el mapa respetando la carpeta del proyecto autorizada.

### 🌐 Gemini en el navegador web

Si prefieres no usar la terminal:
1. Exporta tu mapa con `📁 Archivo` → `🤖 Exportar Markdown para IA (.md)` (o pulsa `🤖 Inteligencia Artificial` → `👁️ Previsualizar Markdown de IA (.md)...`).
2. Entra en la web de Gemini en tu navegador, pega el texto y formula tu consulta.
3. Copia la respuesta generada por Gemini y vuelve a MMCelt: pulsa `🤖 Inteligencia Artificial` → `📥 Importar desde IA (ChatGPT, Claude, Gemini)...` para incorporar las nuevas ramas a tu mapa.
