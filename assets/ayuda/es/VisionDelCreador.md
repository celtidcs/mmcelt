# 🧭 Proyecto e instrucciones para la IA

Es lo que convierte tu mapa en algo que una IA puede entender de verdad.

### 🎯 ¿Por qué hace falta?

Un mapa mental, por sí solo, es un montón de cajas y flechas. Una IA puede leerlas, pero no sabe
**por qué** están ahí. Le falta lo que tú tienes en la cabeza y no has escrito en ningún nodo:
qué problema intentas resolver, por qué has ordenado las ramas así, y qué considerarías un buen
resultado.

Sin eso, la IA rellena huecos. Y rellenar huecos con suposiciones es exactamente lo que hace que
una respuesta suene muy bien y no sirva para nada.

### 📝 Cómo se rellena

1. Abre **`🤖 Inteligencia Artificial`** → **`🧭 Proyecto e instrucciones para la IA`** (también accesible desde el botón del panel lateral o tras editar en `✏️ Edición`).
2. Rellena los tres campos. Ninguno es obligatorio, pero cuanto más concretos, mejor:
   - **Visión:** qué estás intentando plasmar. En una o dos frases, en tus palabras.
   - **Objetivos:** qué resultados concretos buscas. «Hitos y entregables», no deseos.
   - **Contexto / Público:** quién lo va a usar, o en qué entorno tiene que funcionar.

**Un consejo:** escribe como si se lo explicaras a alguien que acaba de entrar en el proyecto.
Eso es exactamente lo que es la IA cada vez que abre tu mapa: alguien nuevo, sin memoria de la
conversación anterior.

### 📦 Qué se le manda a la IA, exactamente

Aquí es donde conviene no llevarse una idea equivocada. **No se mandan solo estos tres campos.**
La exportación manual conserva un documento explicativo con un preámbulo y nueve secciones:

1. **Preámbulo:** las instrucciones de partida para el modelo.
2. **Tu visión y tus objetivos:** lo que rellenas en esta ventana.
3. **Resumen y métricas:** cuántos nodos hay, cuántas ramas cuelgan de la idea central, cuántas
   conexiones, qué etiquetas usas y cuántos nodos hay en cada estado.
4. **La estructura completa** del mapa, con las notas de cada nodo, sus etiquetas y las rutas de
   archivo si las has puesto.
5. **Tus correcciones:** los nodos donde has exigido un cambio y los que has aprobado.
6. **La matriz de conexiones cruzadas** entre ramas.
7. **Las dudas y decisiones pendientes** que hayas marcado.
8. **Un diagrama** del mapa, en formato Mermaid.
9. **Prompts sugeridos:** peticiones ya redactadas para que las copies si quieres.
10. **Cómo devolverte el trabajo:** el contrato con las seis herramientas MCP, la regla de que
    ninguna IA puede declarar una aprobación tuya, y los valores exactos que debe usar.

Las piezas 5, 6 y 7 solo aparecen si tienes algo que poner en ellas. Las demás van siempre. Los
tres prompts sugeridos de la pieza 9 son opciones para ti: el agente no recibe las tres órdenes a
la vez.

Es decir: el mapa entero viaja, no un resumen. Si has escrito una nota larga en un nodo, la IA la
va a leer.

### 🧩 Las cuatro pestañas de la ventana

Para empezar solo necesitas **Proyecto**. Ahí siguen la visión, los objetivos, el público y el
autor. Las otras pestañas son opcionales y sirven cuando quieres controlar con más precisión una
sesión:

- **Instrucciones** guarda reglas comunes para Claude, Codex y Gemini. Puedes elegir el idioma del
  documento sin cambiar el idioma de la aplicación, restaurar una plantilla recomendada o escribir
  tus propias reglas. Lo que escribas se conserva literalmente: MMCelt no lo traduce a escondidas.
- **Plantillas de encargo** ofrece tres puntos de partida. Pulsar uno copia solo esa plantilla al
  encargo editable; nunca se envían las tres juntas. El encargo se conserva al cerrar la ventana y
  reaparece cuando eliges el agente.
- **Vista completa** enseña el resultado exacto de los cuatro bloques. El contrato y el contexto
  salen protegidos; las reglas y el encargo son los dos bloques que decides tú.

Si aparecen `AGENTS.md`, `CLAUDE.md` o `GEMINI.md`, se muestran como fuentes separadas. Una
**Fuente nueva** no se incorpora sola. Pulsa **Aceptar esta versión** solo después de leerla. Si el
archivo cambia más tarde, aparecerá como **Fuente modificada; requiere otra revisión**, quedará
desmarcado y podrás comparar la **Versión aceptada anterior** con la **Versión actual**.
Si no existe ninguno, la pestaña indica la carpeta exacta donde puedes crearlo manualmente.

### ✍️ Y puedes corregir lo que se le pide, antes de que salga

Esta parte pasa desapercibida y es de las más útiles del programa.

Cuando usas **`🤖 Inteligencia Artificial`** → **`📤 Enviar a...`** y eliges un agente, **no se manda
nada todavía**. Se abre una vista previa con el texto completo repartido en cuatro bloques:

| Bloque | ¿Se puede editar? | Qué es |
|---|---|---|
| **Contrato MMCelt** | No | Las reglas mínimas, iguales para todos los agentes |
| **Reglas comunes del proyecto** | **Sí** | Tu forma de trabajar, la que se repite en cada sesión |
| **Contexto del mapa** | No | Lo genera el mapa; para cambiarlo, cambia el mapa |
| **Encargo de esta sesión** | **Sí** | Qué quieres conseguir **ahora**, esta vez |

Si dos instrucciones se contradicen, el propio texto declara quién manda, de más fuerte a más
débil: **tus correcciones en el mapa**, el contrato de MMCelt, las reglas del proyecto y, por
último, el encargo de la sesión.

Debajo hay cuatro botones. `Restaurar esta sesión` deshace lo que hayas cambiado. `Guardar como
reglas comunes` es la **única** acción que guarda tu perfil de forma permanente, en
`.mmcelt/instrucciones-agente.md`. `Cancelar` no guarda nada ni deja rastro. Y el último arranca
el agente.

**Léela al menos una vez.** Si el mapa, las reglas o una fuente cambian después de abrir la
confirmación, MMCelt la da por caducada y obliga a revisar el contenido otra vez. Así el expediente
`inicio.md` y el agente reciben exactamente los mismos bytes que viste en pantalla.

### 🤔 Si nunca has trabajado así

No hace falta que rellenes los tres campos el primer día. Un mapa sin visión también se exporta.
Pero la diferencia entre una IA que te propone lo que ya sabías y una que te aporta algo suele
estar aquí, y no en el modelo que uses.

Empieza por la **Visión**, aunque sean dos líneas. Es el campo que más cambia la respuesta.
