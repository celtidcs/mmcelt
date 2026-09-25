# 🏛️ Roles Semánticos y Arquitectura del Mapa

Cuando un proyecto crece, un mapa mental lleno de cajas idénticas puede volverse confuso. Para evitarlo, MMCelt te permite asignar a cada nodo un **rol semántico o arquitectónico**.

Los roles definen la naturaleza de cada idea y su peso dentro del proyecto. De este modo, tanto tú como las inteligencias artificiales podéis distinguir al instante un pilar estratégico de una simple tarea puntual o de una duda abierta.

### 📋 Los 6 Roles Disponibles en el Inspector

Al seleccionar cualquier nodo del mapa, verás el desplegable **"Rol"** en el panel lateral derecho (Inspector). Puedes elegir entre seis funciones:

1. **🎯 Idea Central:**
   - Es el nodo raíz del mapa mental, el origen de todo el árbol.
   - Representa el producto, la empresa, la investigación o la aplicación que estás construyendo. En un mapa solo hay una idea central.

2. **🏛️ Pilar Estratégico:**
   - Son las grandes columnas maestras que sostienen el proyecto.
   - En software suelen representar capas principales (como Frontend, Backend, Base de Datos o Seguridad); en un negocio pueden ser Marketing, Ventas, Finanzas u Operaciones.

3. **📌 Subtema o Módulo:**
   - Componentes o secciones concretas que cuelgan de un pilar estratégico.
   - Por ejemplo, dentro del pilar *Backend*, los subtemas podrían ser *Servicio de Autenticación*, *Módulo de Pagos* o *Gestor de Notificaciones*.

4. **❓ Hipótesis / Duda:**
   - Preguntas abiertas, decisiones de diseño o experimentos cuya viabilidad técnica aún se desconoce.
   - Por ejemplo: *«¿Conviene usar WebSockets o Server-Sent Events para el chat?»*.

5. **⚡ Acción / Tarea:**
   - Pasos concretos, accionables y ejecutables con un entregable claro.
   - Por ejemplo: *«Diseñar la pantalla de inicio de sesión»* o *«Escribir pruebas unitarias para el cálculo de precios»*.

6. **🔧 Recurso / Herramienta:**
   - Librerías externas, dependencias, documentación de referencia, APIs de terceros o herramientas auxiliares necesarias para el proyecto.

### 💡 Cómo cambiar un rol y su impacto en el mapa y la IA

Para cambiar el rol de un nodo, selecciónalo y elige la opción deseada en el desplegable **"Rol"** del inspector lateral derecho.

**🎨 Distinción visual en el lienzo:**
Cada rol aporta una identidad clara a la caja del nodo dentro del lienzo gráfico:
- Iconos y marcos distintivos para identificarlos sin esfuerzo.
- Conexiones y líneas que reflejan la estructura orgánica del proyecto.

**🤖 ¿Por qué le importan los roles a la IA?**
Cuando exportas o envías tu mapa a una IA (como Claude Code, Codex CLI o Gemini CLI), el modelo no ve una lista plana de textos:
- **Entiende la arquitectura:** Sabe que un *Pilar Estratégico* requiere una visión global y no debe modificarse a la ligera.
- **Diferencia dudas de certezas:** Trata las *Hipótesis / Dudas* como preguntas que necesitan análisis técnico y alternativas.
- **Genera tareas coherentes:** Al pedirle que desarrolle un módulo, la IA propondrá *Acciones / Tareas* concretas y te sugerirá *Recursos / Herramientas* recomendadas para resolverlo.
