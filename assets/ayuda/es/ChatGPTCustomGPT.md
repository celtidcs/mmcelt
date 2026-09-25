# 🟢 Integración con Codex CLI y ChatGPT

Tanto si utilizas la consola como si prefieres el navegador de internet, puedes aprovechar la inteligencia artificial de OpenAI con tus mapas mentales en MMCelt.

### 💻 Codex CLI (en la terminal)

Codex CLI es un agente de consola que se ejecuta directamente en tu ordenador. Es la forma más ágil y automática de trabajar:

1. Ve al menú `🤖 Inteligencia Artificial` → `📤 Enviar a...` → `Codex CLI`.
2. Verás una vista previa completa con el contenido de tu mapa y el encargo para la IA. Léela con calma.
3. Al pulsar **Iniciar en consola**, MMCelt abrirá automáticamente la terminal con el entorno preparado y comenzará a vigilar los cambios para incorporar los nodos que Codex cree.
4. Codex CLI leerá el mapa y las instrucciones a través del servidor MCP. A medida que analice tu proyecto, podrá crear ramas, proponer ideas y actualizar el mapa en directo.

> 💡 **Nota:** Para que Codex CLI pueda comunicarse con MMCelt, recuerda conectarlo previamente desde `🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...` y reiniciar la terminal si la tenías abierta.

### 🌐 ChatGPT en la web o un Custom GPT

Si no utilizas la consola técnica y prefieres chatear en el navegador web (con ChatGPT gratuito, Plus o un Custom GPT propio):

1. Exporta tu mapa con `📁 Archivo` → `🤖 Exportar Markdown para IA (.md)` (o examina el texto en pantalla mediante `🤖 Inteligencia Artificial` → `👁️ Previsualizar Markdown de IA (.md)...`).
2. Abre tu navegador, entra en ChatGPT y adjunta el archivo `.md` o pega el texto directamente en la conversación.
3. Pídele que amplíe el mapa, analice riesgos, busque alternativas o plantee nuevas tareas.
4. Cuando ChatGPT te responda (normalmente devolverá un bloque estructurado en JSON o Markdown), cópialo.
5. Vuelve a MMCelt y selecciona `🤖 Inteligencia Artificial` → `📥 Importar desde IA (ChatGPT, Claude, Gemini)...`. Pega el texto y pulsa importar: las nuevas ramas se integrarán en tu mapa conservando todo tu trabajo previo.

### 🔒 Diferencias clave y privacidad de tus datos

Es importante distinguir cómo interactúan ambos entornos con tus datos:
- **Codex CLI (local con MCP):** Se ejecuta en tu propia máquina y accede directamente a la carpeta de trabajo delimitada a través del protocolo MCP.
- **ChatGPT en el navegador web:** Se ejecuta en los servidores de OpenAI y no tiene acceso directo a tus archivos locales ni al servidor MCP. La interacción se realiza de forma totalmente manual y segura mediante la exportación e importación de texto Markdown o JSON.
