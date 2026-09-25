# 🔵 Integración con Gemini CLI

Gemini es la inteligencia artificial desarrollada por Google, y **Gemini CLI** es su herramienta oficial para la consola de comandos. MMCelt te permite conectar directamente con Gemini CLI para analizar, expandir y enriquecer tus mapas mentales.

### 💻 Uso con Gemini CLI en la terminal

1. Ve a `🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...` y activa la casilla de Gemini CLI.
2. Abre tu mapa y selecciona `🤖 Inteligencia Artificial` → `📤 Enviar a...` → `Gemini CLI`.
3. Revisa la vista previa del documento y del encargo que recibirá la IA.
4. Pulsa **Iniciar en consola**. MMCelt abrirá automáticamente la terminal con el entorno preparado y comenzará a vigilar los cambios para incorporar los nodos que Gemini cree.

### 💡 Trucos y Consejos: Clave de API gratuita y compatibilidad

Si al ejecutar Gemini CLI la consola te pide autenticación o encuentras problemas con el inicio de sesión del navegador, el método más rápido, directo y fiable es utilizar una **clave de API gratuita** de Google AI Studio:

1. Entra en Google AI Studio (`https://aistudio.google.com/app/apikey`) con tu cuenta de Google y pulsa en **Create API Key** (Crear clave de API). Es gratuita.
2. Copia la clave generada.
3. Configura la variable de entorno en tu sistema antes de lanzar la consola o en tu perfil de usuario:
   - **En Windows (PowerShell):**
     ```powershell
     $env:GEMINI_API_KEY="tu_clave_aqui"
     ```
   - **En Linux o macOS (Bash/Zsh):**
     ```bash
     export GEMINI_API_KEY="tu_clave_aqui"
     ```
4. Con esta variable activa, Gemini CLI funcionará de forma inmediata sin solicitar inicios de sesión adicionales en el navegador.
5. Puedes consultar la documentación oficial de Gemini CLI en su repositorio: `https://github.com/google-gemini/gemini-cli`.

**ℹ️ Estado actual de compatibilidad de cuentas en Gemini CLI:**
Confirmado: desde el 18 de junio de 2026, Google retiró el inicio de sesión interactivo («Sign in with Google») de Gemini CLI para cuentas personales, incluidas las gratuitas, Google AI Pro y Ultra.
- **13 de septiembre de 2026:** primera prueba real, con el mensaje literal: *«This client is no longer supported for Gemini Code Assist for individuals. To continue using Gemini, please migrate to the Antigravity suite of products»*.
- **Repetido después con varias versiones anteriores de Gemini CLI**, para descartar que fuera un problema de la versión instalada: el resultado fue el mismo con todas.
- **Confirmado con una clave de API gratuita de Google AI Studio**: la conexión funcionó de principio a fin, creando nodos y enlaces comprobados en disco, sin ninguna restricción de cuenta personal.

La vía de la clave de API explicada arriba **es la única confirmada** para cuentas personales: el inicio de sesión interactivo ya no está disponible para ellas.

### 🌐 Gemini en el navegador web

Si prefieres no usar la terminal:
1. Exporta tu mapa con `📁 Archivo` → `🤖 Exportar Markdown para IA (.md)` (o pulsa `🤖 Inteligencia Artificial` → `👁️ Previsualizar Markdown de IA (.md)...`).
2. Entra en la web de Gemini en tu navegador, pega el texto y formula tu consulta.
3. Copia la respuesta generada por Gemini y vuelve a MMCelt: pulsa `🤖 Inteligencia Artificial` → `📥 Importar desde IA (ChatGPT, Claude, Gemini)...` para incorporar las nuevas ramas a tu mapa.
