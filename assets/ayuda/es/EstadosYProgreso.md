# 📊 Estados de Vida, Progreso y Prioridades

Un mapa mental en MMCelt no es solo un dibujo estático: es un panel vivo de seguimiento de tu proyecto. Cada nodo puede tener un estado de progreso y una prioridad que te ayudan a saber de un vistazo qué está listo, qué se está construyendo y qué dudas bloquean el camino.

### 🏷️ Los 6 Estados de un Nodo

Cuando seleccionas un nodo y miras el **Inspector lateral** (el panel situado a la derecha de la pantalla), puedes asignarle cualquiera de estos seis estados con un simple clic:

1. **💡 Idea:** Una propuesta preliminar o sugerencia que aún no se ha evaluado. Es el estado inicial perfecto para tormentas de ideas.
2. **🔍 Investigando:** Tarea o concepto en fase de exploración técnica, lectura de documentación o estudio de viabilidad antes de empezar a programar.
3. **⏳ En Progreso:** Trabajo activo que se está llevando a cabo en este momento.
4. **❓ Duda / Bloqueo:** Punto crítico donde el desarrollo está detenido porque falta tomar una decisión o resolver una incógnita técnica.
5. **✅ Completado:** Tarea o componente finalizado con éxito, probado y validado.
6. **⛔ Descartado:** Una opción que se estudió pero que se ha decidido no implementar. Dejarla en el mapa como descartada es muy valioso para no volver a tropezar con la misma idea más adelante.

**⚡ Prioridades (🔽 Baja, 🔷 Media, ⚡ Alta, 🔥 Crítica):**
Junto al estado, puedes marcar la urgencia de cada rama. Tanto el menú del Inspector como el propio nodo en el lienzo muestran estos iconos:
- **🔽 Baja y 🔷 Media:** Mejoras opcionales, tareas secundarias o trabajo habitual del día a día.
- **⚡ Alta:** Módulos centrales y componentes prioritarios que deben construirse cuanto antes.
- **🔥 Crítica:** Urgencias máximas o bloqueos que impiden avanzar al resto del proyecto.

### 🤖 ¿Cómo influyen los estados cuando trabajas con la IA?

Los estados de tus nodos no son solo colores para ti; la inteligencia artificial los lee e interpreta con mucho cuidado:
- **Prioridad a tus bloqueos:** Todos los nodos marcados como **`❓ Duda / Bloqueo`** se agrupan en una sección destacada del documento de encargo. La IA sabe que debe centrarse en resolver esas incógnitas antes de inventar cosas nuevas.
- **Respeto a lo descartado:** Si marcas una rama como **`⛔ Descartado`**, la IA entenderá que esa vía fue rechazada intencionadamente y no insistirá en proponértela.
- **Contexto de lo terminado:** Los nodos **`✅ Completado`** le indican a la IA qué partes de tu sistema ya existen y funcionan, para que construya sobre ellas sin duplicar esfuerzos.
- **Métricas automáticas:** En la cabecera de la exportación, MMCelt calcula un resumen global (porcentaje de avance, tareas completadas vs pendientes) para que el modelo sepa la fase exacta de madurez del proyecto.
