# 📁 Mapeo de Archivos de Código (file_path)

Conecta las ideas y decisiones de tu mapa mental con los archivos reales de tu proyecto en el disco.

### 🛠️ Cómo vincular un archivo y para qué sirve:

Un mapa mental te ayuda a pensar en grande: puedes tener un nodo titulado «Autenticación de usuarios» y otro llamado «Base de datos». Sin embargo, tu ordenador y tus programas están formados por archivos concretos (como `src/auth.rs` o `config/database.json`).

El campo de archivo (`file_path`) es el puente que une la idea abstracta con el archivo real donde vive su código o su documentación. Así, el mapa no es solo un dibujo bonito: se convierte en un índice vivo de tu proyecto.

**Pasos para vincular un archivo:**
1. Haz clic sobre cualquier nodo de tu mapa para seleccionarlo.
2. Mira en el panel lateral derecho (el Inspector de nodos).
3. En el campo **📁 Archivo / Ruta:**, escribe la ruta del archivo relativa a la carpeta de tu proyecto. Por ejemplo:
   - `src/login.rs` para un archivo de código.
   - `documentacion/requisitos.md` para un texto explicativo.
   - `frontend/componentes/` para indicar toda una carpeta.

### 💡 Consejos prácticos y uso con la IA:

**🤖 ¿Cómo aprovecha esto la inteligencia artificial?**
Cuando trabajas con un asistente (como Claude Code, Codex CLI o Gemini CLI), o cuando exportas el resumen del proyecto en Markdown:
- La IA lee exactamente qué archivo corresponde a cada nodo.
- Sabe de antemano dónde debe aplicar sus cambios sin tener que buscar a ciegas en todo tu repositorio.
- Te ayuda a mantener una correspondencia limpia y ordenada entre la arquitectura conceptual y el código fuente.

**Consejos prácticos:**
- **Usa rutas relativas:** Escribe siempre las rutas a partir de la carpeta raíz de tu proyecto (por ejemplo `src/main.rs` en lugar de `C:\MisDocumentos\Proyecto\src\main.rs`). De este modo, si mueves tu proyecto o lo compartes con otra persona, todos los enlaces seguirán funcionando.
- **Rutas a carpetas:** Si un nodo agrupa varios archivos, puedes escribir la ruta terminada en barra (como `src/servicios/`) para indicar que representa a todo ese directorio.
