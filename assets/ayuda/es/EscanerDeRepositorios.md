# 🔍 Escáner de Carpetas de Código

Si ya tienes un proyecto de programación en tu ordenador o una carpeta con código fuente, no necesitas crear tu mapa mental nodo a nodo desde cero. El **escáner de código** de MMCelt lee la estructura de tu proyecto y genera automáticamente un mapa visual interactivo en cuestión de segundos.

### 🛠️ Cómo usar el escáner y sacarle provecho:

**🧠 ¿Para qué sirve escanear tu código?**
Al convertir una carpeta de archivos en un mapa mental consigues:
- **Una vista de pájaro de tu arquitectura:** Entender rápidamente cómo se organizan tus módulos, librerías y componentes sin perderte entre decenas de subcarpetas.
- **Vínculos directos a tus archivos:** Cada caja del mapa queda asociada a su ruta real en disco (`file_path`), lo que permite que tanto tú como las IAs sepáis con exactitud qué archivo físico corresponde a cada concepto.
- **El punto de partida ideal para trabajar con IA:** Puedes pedirle a cualquier modelo de inteligencia artificial que analice la estructura obtenida, detecte código duplicado, proponga refactorizaciones o identifique dependencias circulares.

**Pasos para usar el escáner:**
1. En la barra superior, abre el menú **`📁 Archivo`** → **`🔍 Escanear Carpeta de Código...`**.
2. Aparecerá un diálogo del sistema para que elijas la carpeta raíz de tu repositorio o proyecto de software.
3. MMCelt explorará el árbol de carpetas con total seguridad:
   - **Filtra el ruido técnico:** Descarta automáticamente carpetas pesadas de compilación o dependencias que no aportan valor conceptual (como `node_modules/`, `target/`, `dist/`, `.git/`, entornos virtuales de Python, etc.).
   - **Crea la jerarquía:** Coloca la carpeta principal en el centro y ramifica los módulos, paquetes y archivos de código clave.
4. Una vez generado el mapa, puedes reorganizar los nodos a tu gusto, cambiar colores, añadir notas explicativas o marcar áreas pendientes de revisión.

**🤖 Siguiente paso con la IA:**
Una vez escaneado tu código:
- Usa `🤖 Inteligencia Artificial` → `👁️ Previsualizar Markdown de IA (.md)...` para ver el resumen arquitectónico.
- Envía el proyecto a tu agente de consola preferido (`📤 Enviar a...`) o expórtalo para consultar a un chat web. ¡La IA sabrá exactamente dónde vive cada archivo!
