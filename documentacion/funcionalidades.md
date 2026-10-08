# Funcionalidades de MMCelt

Catálogo completo de lo que hace la aplicación, con el comportamiento esperado de cada
funcionalidad y los casos en que resulta útil.

---

## 1. Lienzo infinito

Un plano 2D sin límites donde se distribuyen los nodos del mapa.

| Acción | Cómo |
|---|---|
| Desplazar la vista | Arrastrar con el botón central o el derecho |
| Acercar y alejar | Rueda del ratón (el zoom se centra en el cursor) |
| Mover un nodo | Arrastrarlo con el botón izquierdo |
| Encuadrar el mapa entero | `Inicio`, con el teclado libre |

El nivel de zoom está limitado entre 0,2× y 3,5×. La retícula de fondo adapta su
espaciado para seguir siendo útil en todo ese rango.

**Caso de uso**: en mapas grandes se trabaja acercándose a la rama en curso y alejándose
para recuperar la visión de conjunto.

---

## 2. Estructura del mapa

### Nodos

Cada nodo representa un concepto y contiene:

| Campo | Para qué sirve |
|---|---|
| **Título** | El texto visible en el lienzo |
| **Notas** | Especificación detallada dirigida a la IA. Es el campo que más peso tiene en el documento exportado |
| **Ruta de archivo** | Vincula el nodo con un archivo o carpeta real del proyecto (`src/auth/jwt.rs`) |
| **Etiquetas** | Clasificación temática (`backend`, `rust`, `db`) |
| **Estado** | Madurez del concepto (ver más abajo) |
| **Prioridad** | Baja, Media, Alta o Crítica |
| **Rol** | Función arquitectónica dentro del mapa |
| **Revisión humana** | Control de supervisión sobre lo que genera la IA |
| **Color** | Sobrescribe el color heredado de la rama |

### Estados

| Estado | Significado |
|---|---|
| 💡 Idea | Concepto incipiente, sin desarrollar |
| 🔍 Investigando | En análisis técnico o estudio de viabilidad |
| ⏳ En Progreso | En desarrollo activo |
| ❓ Duda / Bloqueo | Incógnita crítica que exige una decisión |
| ✅ Completado | Hito validado |
| ⛔ Descartado | Opción rechazada — **la IA recibe la instrucción de no insistir en ella** |

### Prioridades

Cada nodo puede ponderar su urgencia relativa en el proyecto. Los cuatro niveles cuentan con iconos homogéneos tanto en el selector del Inspector como dibujados sobre el lienzo antes del título:

| Prioridad | Significado |
|---|---|
| 🔽 Baja | Tarea menor, mejora secundaria o sugerencia opcional |
| 🔷 Media | Prioridad estándar para el desarrollo ordinario |
| ⚡ Alta | Componente crítico o hito clave a resolver a corto plazo |
| 🔥 Crítica | Urgencia máxima o bloqueo que detiene el avance global |

### Roles

| Rol | Uso |
|---|---|
| 🎯 Idea Central | La raíz del mapa |
| 🏛️ Pilar Estratégico | Primer nivel: capa arquitectónica o módulo principal |
| 📌 Subtema / Módulo | Elemento dependiente de un pilar |
| ❓ Hipótesis / Duda | Pregunta técnica o decisión pendiente |
| ⚡ Tarea / Acción | Trabajo concreto con entregable medible |
| 🔧 Recurso / Herramienta | Librería, API o documentación |

Los roles se asignan solos según la profundidad —los hijos de la raíz son pilares y el
resto subtemas— y se pueden cambiar a mano.

### Creación y borrado

| Acción | Atajo |
|---|---|
| Crear nodo hijo | `Tab` o `Insertar` |
| Crear nodo hermano | `Enter` |
| Editar el título | `Espacio` o `F2`, o doble clic |
| Eliminar nodo y su descendencia | `Supr` o `Retroceso` |

El borrado es **en cascada**: elimina el nodo y toda su descendencia, y descarta las
conexiones cruzadas en las que participen. La raíz no se elimina; si se solicita, se
vacía de hijos pero permanece.

### Qué significa cada icono de la tarjeta

Al pasar el ratón por un icono de una tarjeta aparece, junto al puntero, qué significa, con los
mismos nombres que usa el inspector:

| Icono | Dónde está | Texto emergente (ejemplo) |
|---|---|---|
| Emoji de estado | Delante del título | `Estado: 💡 Idea` |
| Prioridad | Arriba a la derecha (solo Alta y Crítica) | `Prioridad: ⚡ Alta` |
| Control humano | Arriba a la derecha (salvo los ordinarios) | `Control Humano: ⏳ Pendiente de Revisión` |
| Rol (🎯 🏛️ 📌 ❓ ⚡ 🔧) | Abajo a la derecha, a la izquierda del 📝 | `Rol: 📌 Subtema / Módulo` |
| 📝 | Abajo a la derecha, si hay notas | `📝 Tiene notas: selecciónalo para leerlas en el inspector` |

Las zonas sensibles se calculan con las mismas posiciones con las que se pintan los iconos, así
que no pueden separarse. Mientras se arrastra un nodo no aparece ningún texto.

### Menú contextual: «Acciones del nodo»

Un **clic derecho sin arrastrar** sobre una tarjeta la selecciona y abre, en ese punto, un menú
con lo que se suele hacer con un nodo. Los rótulos y atajos son los mismos del menú «✏️ Edición»
y del inspector, para que cada opción se reconozca esté donde esté:

| Opción | Equivale a |
|---|---|
| Añadir nodo hijo | `Tab` / `Insertar` |
| Añadir nodo hermano | `Enter` (deshabilitado en la raíz, que no tiene hermanos) |
| Crear conexión cruzada | El cuadro de conexión, con este nodo como origen |
| Editar el título | `Espacio` / `F2` |
| Eliminar el nodo | `Supr` |
| Estado, Prioridad, Control humano, Rol | Submenús con todas las variantes y la actual marcada |

Cada opción actúa sobre el nodo del clic, aunque antes hubiera otro seleccionado. Arrastrar con
el botón derecho sigue moviendo la vista y no abre el menú. `Esc` o un clic fuera lo cierran sin
cambios; mientras está abierto, los atajos que cambian el mapa no actúan. Con el mapa en solo
lectura no se abre. Elegir en un submenú el valor que ya tenía no cuenta como cambio.

### Soltar un nodo encima de otro

Arrastrar un nodo y soltarlo con su **centro** encima de otra tarjeta abre un mini-menú en
ese punto, porque el gesto admite varias intenciones:

| Opción | Qué hace |
|---|---|
| `➕ Hacer hijo` | El nodo, con toda su descendencia, pasa a colgar del de debajo. En «Posición Libre» se aparta lo justo para verse; en los modos automáticos, el mapa se recoloca |
| `↔ Hacer hermano` | Lo cuelga del mismo padre que el de debajo, justo detrás de él (no si el de debajo es la raíz ni si ya son hermanos) |
| `🔗 Conectar con enlace` | Crea una conexión cruzada hacia el de debajo y devuelve el nodo arrastrado a su sitio. No se ofrece si ya son padre e hijo o ya están conectados: sería otra línea encima de la que los une |
| `➡ Mover aquí sin tapar` | Lo deja junto al de debajo, en el hueco libre más cercano a donde se soltó, sin solapar ninguna tarjeta |
| `↩ Cancelar` | Lo devuelve a donde estaba; `Esc` o un clic fuera del menú hacen lo mismo |

«Hacer hijo» aparece deshabilitado, con la explicación al pasar el ratón, cuando el árbol no lo
admite: el arrastrado es la raíz, el de debajo ya es su padre, o el de debajo está dentro de su
propia descendencia, lo que crearía un bucle. Soltar en el vacío, o un simple clic sin mover el
nodo, no abre nada. Mientras el menú está abierto, los atajos que cambian el mapa no actúan, y
cualquier opción se deshace con `Ctrl + Z`.

**Caso de uso**: reorganizar la jerarquía con el ratón, sin tener que borrar y volver a crear
una rama para cambiarla de sitio.

---

## 3. Qué significan los colores

Los colores del lienzo **no son decorativos**: dicen a qué rama pertenece cada cosa.

### Las líneas que unen los nodos

Cada **rama principal** —cada hijo directo de la idea central— recibe un color, y **toda
su descendencia lo hereda**:

| Rama | Color |
|---|---|
| 1.ª | 🔵 Azul |
| 2.ª | 🟢 Esmeralda |
| 3.ª | 🟡 Ámbar |
| 4.ª | 🟣 Púrpura |
| 5.ª | 🌸 Rosa |
| 6.ª | 🩵 Turquesa |
| 7.ª | 🟠 Naranja |
| 8.ª | 🔷 Índigo |

Eso permite **seguir cualquier hilo hasta el final** y saber de dónde viene, aunque el
mapa esté enredado y las ramas se crucen por la pantalla.

Con más de ocho ramas la paleta se reutiliza: la novena vuelve al azul.

### Las conexiones cruzadas son distintas

Van siempre en **ámbar**, con un color propio que no depende de la rama. Es deliberado:
así distingues de un vistazo lo que es jerarquía de lo que es una dependencia entre ramas
distintas.

**Todas se dibujan igual en el lienzo**: misma curva y mismo grosor. Lo que distingue una
de otra es **la etiqueta de texto** que aparece en su punto medio: la que escribiste al
crearla o, si la dejaste vacía, el nombre del tipo de relación.

Donde sí cambian de forma es en el **diagrama Mermaid del documento exportado**, para que
el modelo distinga el tipo de un vistazo:

| Relación | Trazo en el diagrama exportado |
|---|---|
| ➡️ Dependencia (Requiere) | Flecha gruesa |
| 💡 Inspirado por | Línea discontinua |
| ⛔ Bloquea | Línea con aspas |
| 🔀 Alternativa a | Flecha doble |
| ✨ Sinergia | Línea gruesa sin punta |

> **Detalle que puede despistar**: la barrita vertical del borde izquierdo de cada nodo no
> usa el color de la rama, sino uno según su **profundidad** en el árbol. Así que dos nodos
> del mismo nivel comparten barra aunque estén en ramas distintas. Son dos codificaciones
> conviviendo; si te resulta confuso, es un cambio sencillo de unificar.

---

## 4. Conexiones cruzadas

Relaciones entre nodos que **no comparten rama**, para expresar dependencias que la
jerarquía no puede representar.

| Tipo | Significado |
|---|---|
| ➡️ Dependencia (Requiere) | El origen requiere que el destino esté disponible |
| 💡 Inspirado por | El origen toma el destino como referencia |
| ⛔ Bloquea | El origen impide el progreso del destino |
| 🔀 Alternativa a | Son soluciones excluyentes entre sí |
| ✨ Sinergia | Se potencian mutuamente |

Cada conexión lleva una etiqueta que explica el motivo. Se dibujan como curvas con un
estilo distinto según el tipo, y aparecen en el documento exportado como una matriz de
dependencias.

**Caso de uso**: marcar que «Elección de base de datos» bloquea «Diseño del API», de modo
que la IA sepa que no debe proponer endpoints antes de resolver esa duda.

---

## 5. Disposición automática

| Modo | Comportamiento |
|---|---|
| Árbol Balanceado (Izq/Der) | Reparte las ramas de primer nivel a izquierda y derecha de la raíz, calculando alturas para que ninguna se solape |
| ⭕ Radial / Circular | Distribuye las ramas en abanico alrededor de la raíz. Cada rama recibe el trozo de círculo proporcional al sitio que necesita, así que una rama gruesa no empuja a las demás hacia afuera |
| 🖐️ Posición Libre Manual | No recoloca nada: respeta las posiciones que fije el usuario |

Los dos primeros modos recalculan las posiciones al añadir o eliminar nodos.

---

## 6. Exportación a Markdown para IA

La funcionalidad central del programa. Genera un documento estructurado con técnicas de
ingeniería de prompts, pensado para que un modelo lo lea de una vez.

- **Previsualización:** con `🤖 Inteligencia Artificial → 👁️ Previsualizar Markdown de IA (.md)...` puedes leer el texto antes de escribir ningún archivo.
- **Exportación:** con `📁 Archivo → 🤖 Exportar Markdown para IA (.md)` o `🤖 Inteligencia Artificial → 💾 Exportar Archivo .md para IA...` (atajo `Ctrl + E`).

El documento contiene nueve secciones:

1. **Meta-instrucciones** para el modelo.
2. **Visión del creador**: qué se busca y por qué.
3. **Resumen ejecutivo**: métricas y desglose por estado.
4. **Estructura jerárquica** con notas, rutas de código y etiquetas.
5. **Control humano**: correcciones exigidas y ramas aprobadas.
6. **Matriz de dependencias**: las conexiones cruzadas.
7. **Puntos de decisión**: las dudas abiertas, agrupadas y priorizadas.
8. **Diagrama Mermaid** del grafo completo.
9. **Prompts de acción sugeridos**.

**Por qué importa el orden**: la visión y los objetivos van primero para que el modelo
lea la intención antes que el detalle. Las dudas se agrupan al final en una sección
propia, en lugar de quedar dispersas por el árbol, de modo que el modelo pueda abordarlas
como una lista.

Hay una vista previa que permite revisar el documento antes de exportarlo, y la opción de
copiarlo directamente al portapapeles.

---

## 7. Importación desde IA

El camino inverso: convierte la respuesta de un modelo en un mapa mental.

La aplicación incluye un **prompt maestro** listo para copiar, que indica al modelo el
esquema JSON exacto que debe generar. Se pega la respuesta y MMCelt reconstruye el mapa.

### Tolerante a fallos, pero no silencioso

El importador intenta dos interpretaciones:

1. Como bloque JSON con el esquema del prompt maestro. Localiza el bloque aunque venga
   envuelto en ```` ```json ```` o rodeado de texto conversacional.
2. Si eso falla, como esquema Markdown de títulos y viñetas.

Acepta además **sinónimos** en los valores: un modelo que escriba `"In Progress"` en
lugar de `"EnProgreso"`, o `"urgent"` en lugar de `"Critica"`, se interpreta
correctamente.

**Si ninguna interpretación funciona, lo dice.** El mensaje explica por separado qué
falló en cada intento, para que el usuario sepa qué corregir. No devuelve un mapa vacío
fingiendo éxito.

---

## 8. Control humano (*Human-in-the-loop*)

Mecanismo para dirigir a la IA cuando se desvía. Cada nodo tiene un estado de revisión:

| Estado | Significado |
|---|---|
| 🤖 Generado por IA | Propuesto por un modelo, pendiente de auditoría |
| ⏳ Pendiente de Revisión | A la espera de validación |
| 🛡️ Aprobado por Usuario | Validado explícitamente |
| ⚠️ Requiere Corrección | El usuario exige cambios |

Todos los estados de revisión humana muestran sus iconos de forma homogénea tanto en el selector desplegable del Inspector como en el nodo dentro del lienzo (incluido el indicador `⏳` para nodos pendientes).

Además, la supervisión cuenta con **caducidad automática**: si la IA modifica posteriormente cualquier dato de un nodo que ya había sido marcado como `🛡️ Aprobado por Usuario` (notas, estado, prioridad o ruta de archivo), el nodo vuelve automáticamente al estado `⏳ Pendiente de Revisión`. Esto evita que un cambio posterior de un modelo conserve un visto bueno humano que nunca existió.

Los nodos marcados como `Requiere Corrección` admiten un texto con la instrucción
concreta. Con ellos se genera un **prompt de corrección** que contiene solo las directivas
de rumbo, sin repetir el mapa entero: está pensado para reconducir a una IA que ya conoce
el proyecto.

**Caso de uso**: la IA propone MongoDB y se quiere PostgreSQL. Se marca el nodo como
`Requiere Corrección`, se escribe el motivo, y se envía el prompt de corrección.

---

## 9. Ejemplos y plantillas

### Ejemplos: mapas completos

Menú `📁 Archivo → 📚 Ejemplos (mapas completos)`.

A diferencia de las plantillas, **vienen terminados**: con notas escritas, dudas
abiertas, estados repartidos, opciones descartadas y conexiones cruzadas. Sirven para ver
de un vistazo cómo queda un mapa que de verdad dirige a una IA, que es lo que cuesta
imaginar partiendo de un lienzo vacío.

| Ejemplo | De qué trata |
|---|---|
| 🎮 **Juego del Ahorcado** | Un proyecto de programación pequeño: lógica, interfaz, banco de palabras, estadísticas y pruebas. Se eligió un juego que cabe entero en la cabeza, para que la atención vaya a **cómo está anotado el mapa** y no a entender el dominio |
| 🍲 **Negocio de comida a domicilio** | Deliberadamente **no técnico**: producto, permisos, costes, clientes y logística. Muestra que la herramienta sirve igual para pensar un negocio que para diseñar software |

Fíjate en tres cosas al abrirlos:

- Los nodos en **❓ Duda / Bloqueo** son preguntas reales sin resolver. Al exportar, la IA
  las recibe agrupadas como una lista de decisiones pendientes.
- Los nodos en **⛔ Descartado** dejan constancia de lo que ya se decidió *no* hacer, para
  que el modelo no lo vuelva a proponer.
- Las **conexiones cruzadas** expresan dependencias que la jerarquía no puede: en el
  ejemplo del negocio, «¿Puedo vender desde mi cocina?» *bloquea* el cálculo de costes,
  porque alquilar un obrador lo cambiaría todo.

### Plantillas: estructuras para rellenar

Menú `📁 Archivo → ✨ Plantillas de Arquitectura`.

| Plantilla | Contenido |
|---|---|
| **Arquitectura Limpia** | Las cuatro capas (dominio, casos de uso, infraestructura, presentación) con sus subtemas, rutas sugeridas y dos conexiones que ilustran la regla de dependencia |
| **Fullstack** | Frontend, backend, base de datos y despliegue, con las conexiones del flujo de datos |

---

## 10. Escáner de repositorios

Convierte la estructura de carpetas de un proyecto real en un mapa mental. Recorre hasta
**tres niveles** de profundidad, omite las carpetas ocultas y de artefactos (`target`,
`node_modules`, `dist`, `build`) y crea un nodo por cada carpeta y archivo de código
reconocido, con su ruta ya rellenada.

Se limita a quince **archivos de código** por carpeta para que el mapa siga siendo legible.
Los que no son código —imágenes, fuentes, datos— no se mapean ni cuentan para ese límite.

**Caso de uso**: partir de un código existente para pedir a la IA un plan de
refactorización sobre su estructura real.

---

## 11. Persistencia

| Formato | Uso |
|---|---|
| `.mmcelt` | Formato nativo (JSON). Guarda el mapa completo |
| `.json` | Equivalente, con otra extensión |
| `.md` | Exportación para IA (solo escritura) |
| `.opml` | Intercambio de esquemas (XML estándar no propietario). Exportación e importación con metadatos embebidos |
| `.mm` | Mapas mentales FreeMind / Freeplane (XML estándar no propietario). Exportación e importación con metadatos embebidos |

Atajo de guardado: `Ctrl + S`.

### Formatos no propietarios de intercambio (OPML y FreeMind .mm)

Por decisión estratégica y compromiso con los estándares abiertos, MMCelt soporta formatos de intercambio **estrictamente no propietarios**:
- **OPML (Outline Processor Markup Language)**: Estándar XML basado en esquemas jerárquicos (`<outline>`), compatible con gestores de ideas, editores estructurados (Workflowy, OmniOutliner, etc.) y lectores RSS.
- **FreeMind / Freeplane (`.mm`)**: Estándar XML de facto para mapas mentales libres, compatible con FreeMind, Freeplane y visores compatibles.

Se descartan deliberadamente formatos cerrados o propietarios (`.xmind`, `.mmap`).

#### Estudio comparativo: Qué se gana y qué se pierde por formato

Los formatos clásicos como OPML o `.mm` fueron concebidos exclusivamente para árboles jerárquicos simples (padre a hijos) con un título y una nota de texto. **No disponen de soporte nativo para los conceptos de arquitectura y supervisión de MMCelt**: estado de madurez, prioridad estratégica, rol arquitectónico, supervisión humana, rutas a código fuente, etiquetas temáticas, ni **conexiones cruzadas laterales**.

Para resolver esto sin perder ninguna información, MMCelt implementa una estrategia de **metadatos estructurados transparentes**:
1. **Lo nativo viaja en los campos del formato**: El título, la jerarquía de ramas y las notas de texto libre escritas por el usuario se exportan en las etiquetas estándar del formato (`<outline text="..." _note="...">` en OPML y `<node TEXT="...">` con `<richcontent TYPE="NOTE">` en `.mm`).
2. **Lo que no cabe nativamente viaja como JSON estructurado**: Al final de la nota de cada nodo se añade una línea técnica `MMCELT_METADATOS_NODO_JSON: {...}` con los campos que el formato no puede representar. En el nodo raíz se añade adicionalmente `MMCELT_METADATOS_PROYECTO_JSON: {...}` con la visión del creador, objetivos y las conexiones cruzadas.

La siguiente tabla resume exhaustivamente qué se conserva de forma nativa, qué viaja en el bloque estructurado y cómo se comporta al abrir en otra aplicación o reimportar en MMCelt:

| Característica / Campo en MMCelt | En formato nativo (.mmcelt) | En exportación OPML (.opml) | En exportación FreeMind (.mm) | Al abrir en otra aplicación (FreeMind, OmniOutliner, etc.) | Al reimportar en MMCelt |
|---|---|---|---|---|---|
| **Jerarquía del mapa (árbol)** | Nativo (padre / hijos) | Nativo (`<outline>` anidado) | Nativo (`<node>` anidado) | Se visualiza la estructura de árbol completa | Se reconstruye el árbol completo |
| **Título del nodo** | Nativo (`title`) | Nativo (`text` / `_text`) | Nativo (`TEXT`) | Visible como texto del nodo o ítem | Se recupera intacto |
| **Notas de texto del usuario** | Nativo (`notes`) | Nativo (`_note` / `<notes>`) | Nativo (`<richcontent TYPE="NOTE">`) | Legible como nota del nodo | Se recupera la prosa limpia (sin marcas) |
| **Estado de madurez** | Nativo (`status`) | JSON en la nota | JSON en la nota | Se lee como texto JSON al pie de la nota | Se reconstruye el enum tipado |
| **Prioridad estratégica** | Nativo (`priority`) | JSON en la nota | JSON en la nota | Se lee como texto JSON al pie de la nota | Se reconstruye el enum tipado |
| **Rol arquitectónico** | Nativo (`role`) | JSON en la nota | JSON en la nota | Se lee como texto JSON al pie de la nota | Se reconstruye el enum tipado |
| **Supervisión y corrección humana** | Nativo (`review_status`, `correction_feedback`) | JSON en la nota | JSON en la nota | Se lee como texto JSON al pie de la nota | Se reconstruye el estado y el texto de corrección |
| **Ruta de archivo de código** | Nativo (`file_path`) | JSON en la nota | JSON en la nota | Se lee como texto JSON al pie de la nota | Se recupera la vinculación a disco |
| **Etiquetas temáticas** | Nativo (`tags`) | JSON en la nota | JSON en la nota | Se lee como texto JSON al pie de la nota | Se recuperan las etiquetas |
| **Posición [X, Y] en lienzo 2D** | Nativo (`pos`) | JSON en la nota | JSON en la nota | La otra app usa su propio algoritmo de ramas | Se recuperan las coordenadas cartesianas |
| **Ramas plegadas** | Nativo (`collapsed`) | JSON en la nota | Nativo (`FOLDED="true"`) y JSON | Se respeta el plegado si la app lo soporta | Se recupera el estado de colapso |
| **Conexiones cruzadas (relaciones laterales)** | Nativo (`connections`) | JSON en la nota raíz | JSON en la nota raíz | Solo visible como texto de datos en la nota de la raíz | **Se reconstruyen todas las conexiones laterales** |
| **Visión y objetivos del proyecto** | Nativo (`creator_vision`, `project_goals`, etc.) | JSON en la nota raíz | JSON en la nota raíz | Visible como texto al final de la nota de la raíz | Se recuperan todos los campos del proyecto |

#### Resumen de balance: Qué se gana y qué se pierde

- **Qué se gana:**
  1. **Interoperabilidad real**: Cualquier programa capaz de leer OPML o FreeMind puede abrir los mapas de MMCelt y aprovechar la estructura conceptual, títulos y notas explicativas.
  2. **Ida y vuelta exacta (sin pérdidas)**: Al exportar un mapa de MMCelt a OPML o `.mm` y volver a importarlo en MMCelt, se reconstruye el 100% de la información (estados, prioridades, roles, rutas de código y conexiones cruzadas).
  3. **Transparencia técnica**: La información no queda secuestrada en un formato binario ni propietario; el JSON dentro de las notas es estándar y legible por humanos o herramientas de automatización.

- **Qué se pierde al abrir el archivo en otra aplicación:**
  1. **Semántica de interfaz**: La otra aplicación no reconoce los estados, prioridades o roles como controles de su propia interfaz gráfica; aparecen como texto JSON complementario en el panel de notas.
  2. **Visualización gráfica de conexiones cruzadas**: Las herramientas clásicas representan árboles estrictos. Las relaciones laterales entre ramas distintas no se dibujan como flechas visuales en la otra aplicación; viajan seguras en el JSON pero solo MMCelt las renderiza en su lienzo.
  3. **Disposición espacial libre**: OPML carece de coordenadas y `.mm` ubica los nodos de forma radial o bifurcada. Las posiciones 2D manuales de MMCelt no se reflejan visualmente en herramientas que carecen de lienzo cartesiano libre.

#### Importación de archivos externos sin metadatos de MMCelt

Al importar un archivo `.opml` o `.mm` creado en otra aplicación (o que no contenga los bloques `MMCELT_METADATOS_`):
1. **La jerarquía, títulos y notas se importan fielmente**: Toda la prosa de la nota se preserva como notas del usuario.
2. **Valores por defecto seguros**: Dado que el archivo ajeno carece de los campos específicos de MMCelt, el programa asigna de manera determinista:
   - Estado: `Idea` (💡).
   - Prioridad: `Media` (🔷).
   - Rol: `Idea Central` (🎯) para la raíz del mapa importado; `Subtema` (📌) para el resto de nodos, sin distinción por nivel.
   - Supervisión humana: `Sin Revisar`.
   - Sin ruta de archivo ni etiquetas.
3. **Disposición visual automática**: Al carecer de coordenadas cartesianas compatibles, MMCelt aplica automáticamente el algoritmo de disposición espacial balanceada para desplegar las ramas a izquierda y derecha del centro. De este modo, los nodos no aparecen amontonados en el origen `[0, 0]` y el mapa ajeno queda inmediatamente legible y editable desde el primer instante.
4. **Aviso explícito al usuario**: MMCelt muestra un diálogo informativo en el idioma activo explicando que el archivo importado proviene de una fuente externa y que se han establecido los atributos por defecto, garantizando transparencia total.

### Comprobaciones al abrir

Un archivo se rechaza si describe un mapa imposible: con ciclos, sin nodo raíz, con
referencias a nodos inexistentes, con nodos inalcanzables desde la raíz, o que supere los
límites de 512 niveles de profundidad o 100 000 nodos.

El mensaje de error **identifica el nodo concreto** que causa el problema.

Esto no es una restricción arbitraria: sin ella, un archivo dañado congelaba la
aplicación o la cerraba de golpe. Los límites están muy por encima de cualquier mapa que
una persona vaya a construir.

### Guardado verificado

Si al guardar ocurre un error —permisos, disco lleno, archivo bloqueado, carpeta de nube
sin sincronizar—, la aplicación **lo comunica**. Nunca informa de un guardado correcto
sin haberlo comprobado.

### Guardado automático

Cada dos minutos, si el mapa ha cambiado, la aplicación escribe una copia de seguridad.

**El intervalo se elige en `🎨 Ver y Diseño → 💾 Autoguardado`**: desactivado, cada minuto, cada
dos, cada cinco o cada diez. Se recuerda entre sesiones. Antes eran dos minutos fijos escritos
en el código y no había forma de cambiarlos ni de apagarlo sin recompilar el programa, aunque
el campo interno que lo apagaba ya existiera.

Los valores son cerrados y no un número libre a propósito: un intervalo de dos segundos
escribiría el mapa entero continuamente, y uno de ocho horas es no tener autoguardado mientras
se aparenta tenerlo. Un valor fuera de esa lista escrito a mano en el archivo de preferencias
se ignora.
Si el programa se cierra de forma inesperada, al volver a abrirlo se ofrece recuperarla.

**No sustituye a guardar.** Es una red de seguridad, y esa diferencia explica cómo se
comporta:

| | Guardar (`Ctrl+S`) | Guardado automático |
|---|---|---|
| Lo decide | Tú | El programa |
| Dónde escribe | Donde elijas | Siempre en la carpeta de datos de MMCelt |
| Pregunta | Sí, la primera vez | **Nunca** |
| Para qué sirve | Conservar el trabajo | Recuperarlo tras un cierre inesperado |

La copia **nunca se deja junto a tus archivos**: va a la carpeta de datos de la
aplicación (`%APPDATA%\MMCelt\recuperacion\` en Windows). Así no aparecen archivos
sueltos en tus proyectos.

Solo se conserva la última sesión. Al guardar de verdad, la copia se descarta: el trabajo
ya está en su sitio.

En los modos de disposición automática, mover un nodo no cuenta como cambio —el programa va
a recolocarlo igualmente—, así que un mapa que solo se está mirando no provoca escrituras. En
**Posición Libre Manual** sí cuenta: ahí colocar los nodos es el trabajo del usuario, y nada
más lo refleja.

**Al arrancar tras un cierre inesperado**, aparece un aviso con la fecha de la copia y el
mapa que contiene, y dos opciones: recuperarla o descartarla. No se restaura sola —
podrías haber cerrado a propósito sin guardar, y devolverte lo que descartaste sería peor
que no ofrecer nada.

### Dónde propone guardar

Cuando hay que elegir carpeta, el diálogo se abre en el sitio más probable:

1. La carpeta del archivo actual, si el mapa ya se había guardado.
2. **La carpeta del proyecto de código al que se refiere el mapa**, si sus nodos llevan
   rutas de archivo. Es el caso de los mapas que genera el escáner de repositorios y de
   los que produce una IA trabajando sobre un código concreto: se deduce la raíz común de
   esas rutas.
3. Donde el sistema recuerde, si no hay nada mejor.

Es una **sugerencia**, no una imposición: puedes navegar a donde quieras. La aplicación
nunca escribe por su cuenta en una carpeta deducida, porque eso podría colar archivos
dentro de un repositorio sin que te dieras cuenta.

> Si trabajas con el servidor MCP, este punto ya está resuelto por otra vía: el agente
> indica la ruta y el mapa se guarda directamente en la carpeta del proyecto.

---

## 12. Integración con agentes de IA (servidor MCP)

Un servidor que permite a Claude Code, Antigravity, Codex y otros agentes leer y escribir
mapas directamente, sin copiar y pegar.

### «📤 Enviar a...» y el mapa que vuelve solo

Desde la **0.9.0**, menú **`🤖 Inteligencia Artificial → 📤 Enviar a...`**, con un submenú por
cada agente detectado en el equipo. Desde la **0.10.0** el envío es **supervisado**: pulsar un
agente no manda nada todavía.

La ventana `🧭 Proyecto e instrucciones para la IA` permite preparar el contenido antes de elegir
un agente. **Proyecto** mantiene los datos del mapa; **Instrucciones** conserva el perfil portable;
**Plantillas de encargo** copia una sola petición; y **Vista completa** muestra la composición
exacta. El pie común mantiene visibles desde las cuatro pestañas el idioma del documento y la
acción `Guardar como reglas comunes`. El idioma no depende del idioma de la interfaz, se persiste
en cuanto cambia y reaparece al abrir de nuevo la ventana; los textos escritos por la persona nunca
se traducen automáticamente.

**Qué se ve al pulsarlo**: una **vista previa** con el prompt completo, en cuatro bloques.

| Bloque | Editable | De dónde sale |
|---|---|---|
| **Contrato MMCelt** | No | Lo genera la aplicación: reglas mínimas de integridad y devolución por MCP |
| **Reglas comunes del proyecto** | Sí | `.mmcelt/instrucciones-agente.md`, el mismo perfil para todos los agentes |
| **Contexto del mapa** | No | La exportación para IA del mapa abierto; se cambia en el mapa |
| **Encargo de esta sesión** | Sí | Lo escribe la persona o parte de una plantilla preparada antes de elegir agente |

La **precedencia va escrita dentro del prompt**: correcciones humanas del mapa **>** contrato
MMCelt **>** reglas del proyecto **>** encargo de la sesión.

`Restaurar esta sesión` devuelve reglas y encargo a como estaban. `Guardar como reglas comunes`
es una acción aparte y es la única que escribe el perfil del proyecto en disco. Cambiar el idioma
del documento sí guarda inmediatamente esa preferencia, pero no guarda ni altera el texto del
perfil. `Cancelar` no guarda, no exporta, no vigila y no deja expediente.

**Instrucciones nativas con consentimiento durable.** Si en la raíz hay `AGENTS.md`, `CLAUDE.md` o
`GEMINI.md`, la vista los enumera y deja leerlos. Una fuente nueva permanece desmarcada hasta que la
persona acepta esa versión. Se guardan contenido y SHA-256 en
`.mmcelt/configuracion-agente.json`; si el archivo cambia, vuelve a quedar desmarcado y se muestran
las versiones anterior y actual. La selección final sigue siendo visible y ajustable por sesión.
Si no existe ninguna fuente, la propia pestaña enumera los tres nombres admitidos y muestra la
raíz exacta donde la persona puede crear una manualmente; MMCelt no crea instrucciones por ella.

Un solo compositor validado produce la vista, la confirmación y `inicio.md`. Rechaza controles
invisibles, limita los bloques editables a 65.536 bytes y el conjunto a 16 MiB. Una confirmación
caduca si cambian mapa, reglas, encargo o contenido de una fuente antes del lanzamiento.

El botón refleja la capacidad real: **«Iniciar en consola»** cuando existe CLI y **«Preparar para
MCP»** cuando el agente solo dispone del servidor. Con consola sigue este orden fijo:

1. **Guarda el mapa**, para que el agente lea lo mismo que hay en pantalla.
2. **Exporta el Markdown para la IA** junto al mapa, con el sufijo `_AI.md`.
3. **Escribe el expediente** de la sesión en `.mmcelt/sesiones`.
4. **Abre la consola** del agente en la carpeta del proyecto.
5. **Activa la vigilancia** del mapa.

Si la consola no llega a abrirse, MMCelt **no dice «enviado»**, no empieza a vigilar y deja el
expediente marcado como fallido.

Para un destino **solo MCP**, como Antigravity, ejecuta los pasos 1, 2, 3 y 5 sin inventar el
cuarto: el expediente permanece `Preparado`, la vista indica que no se abrirá consola y la persona
continúa desde la interfaz propia del agente.

Una carpeta de una versión anterior puede no tener todavía `.mmcelt/configuracion.json`. La vista
previa usa un límite efímero y **no escribe esa identidad**; solo se inicializa al confirmar.

**El expediente: `.mmcelt/sesiones`.** Cada confirmación crea una carpeta nombrada con la fecha
UTC, el identificador fijo del agente y un UUID —nunca con texto del usuario—, con dos archivos:

- `inicio.md`: el prompt exacto que se aprobó, letra por letra.
- `sesion.json`: versión de formato, identificador, fecha UTC, agente, ruta relativa del mapa,
  fuentes detectadas e incorporadas, tamaño del prompt y estado (`Preparado`, `Iniciado` o
  `Fallido`).

No se escriben contraseñas, claves, correos ni identificadores de cuenta. Para **repetir una
sesión** basta con abrir la consola del agente en la carpeta y pedirle que lea ese `inicio.md`.

**Qué pasa después**: cuando el agente cambia el mapa en disco, **la ventana se actualiza
sola**. No hay que importar nada. Un indicador `🟢` en la barra superior avisa de que la
vigilancia está activa, y `⏹ Detener seguimiento` la corta en un clic. La consola del agente es
un **proceso aparte**: se cierra en su propia ventana, y detener la vigilancia no la cierra.

**Qué pasa si el usuario también estaba editando.** La regla manda sobre todo lo demás: su
trabajo no se pierde.

| Situación | Comportamiento |
|---|---|
| Sin cambios locales sin guardar | El mapa se recarga en el momento |
| **Con cambios sin guardar** | **El mapa no se toca**; se avisa y decide la persona |
| Archivo a medio escribir | Se ignora y se espera al siguiente aviso, sin error en pantalla |
| Escritura del propio MMCelt | Se reconoce por el contenido y no dispara recarga |

Antes de cualquier recarga se guarda una copia en el archivo de recuperación.

**Alcance real, que conviene no prometer de más**: MMCelt abre la consola de **Claude Code**,
**Codex CLI** y **Gemini CLI**. Con los editores y las aplicaciones de escritorio prepara el
documento y el expediente, pero el último paso lo da la persona. **La vuelta, en cambio,
funciona igual para todos**: cualquier agente que escriba el archivo por MCP dispara la
recarga.

**MCP y consola son dos capacidades separadas**, y el menú lo dice con un icono por agente:

| Icono | Significa |
|---|---|
| ✨ | Consola y MCP: puede conversar y devolver trabajo al mapa |
| 🖥 | Solo consola: MMCelt encuentra su ejecutable, pero falta registrar MCP |
| 🔗 | Solo MCP: puede devolver trabajo, pero MMCelt no puede abrirle una consola |
| 🔌 | Ninguna de las dos |

Que un agente figure conectado por MCP **no significa** que se le pueda abrir una consola.

**Antigravity no es Gemini CLI.** Son dos productos distintos de Google y aparecen como destinos
distintos: Antigravity es el entorno gráfico y sigue siendo cliente MCP; Gemini CLI es la consola
interactiva, y se abre como destino propio si su ejecutable está instalado.

**Las cuentas las pone el CLI, no MMCelt.** La aplicación no pide, no guarda y no transforma
credenciales: se apoya en la sesión que ya esté autenticada en `claude`, `codex` o `gemini`. No
usa ninguna API de pago, no elige modelo y no decide facturación.

**Plataformas**: el recorrido actual está comprobado en **Windows**; la detección respeta
`PATHEXT` y no elige el script Unix sin extensión que dejan algunas instalaciones npm. El
adaptador de **Linux** está implementado con una lista cerrada de terminales, pero su aceptación
de esta versión sigue pendiente. **macOS** todavía **no** se afirma.

### Conexión en un clic

Menú **`🤖 Inteligencia Artificial → Conectar MMCelt con mis IAs`**.

La ventana detecta los agentes instalados en el equipo, muestra cuáles están conectados y
permite conectarlos o desconectarlos con un botón. **No hay que editar ningún archivo ni
abrir una terminal.**

«Conectado» significa que la entrada no solo existe: la ruta registrada sigue apuntando a un
archivo real y contiene el argumento exacto `--mcp-server`. Si MMCelt se mueve, se renombra o se
borra, la ventana vuelve a ofrecer **Conectar** para reparar la configuración obsoleta.

Reconoce:

| Agente | Qué es |
|---|---|
| **Claude Code** | Asistente de programación de Anthropic en la terminal |
| **Codex CLI** | Agente de programación de OpenAI en la terminal |
| **Gemini CLI** | Agente de Google en la terminal (con soporte para clave de API) |

*(Los clientes de entorno gráfico o editores como Claude Desktop, Antigravity, Cursor o Windsurf fueron retirados del catálogo para simplificar el flujo y trabajar exclusivamente con agentes oficiales de consola).*

Un agente aparece si existe su carpeta de configuración, si existe su archivo de configuración
**o** si su ejecutable está en el `PATH`. Esa tercera vía es la que hace visible una consola recién
instalada que todavía no ha escrito nada en el disco.

La detección **se rehace cada vez que abres la ventana**. Si instalas un agente nuevo la
semana que viene, aparecerá sin necesidad de reiniciar MMCelt ni de recordar ningún paso.

También eliges ahí la **carpeta de trabajo**: la única dentro de la cual los agentes
podrán leer y escribir.

**Mientras no elijas ninguna, se propone la del mapa que tengas abierto**, no una carpeta
fija. Autorizar `Documentos/MapasMentales` cuando tu proyecto está en otro sitio le daba al
agente permiso para trabajar donde no hay nada, y cada ruta que probaba chocaba contra la
valla del espacio de trabajo. En cuanto eliges una carpeta a mano, manda esa: abrir un mapa
en otro sitio no te la cambia por debajo.

**Si cambias la carpeta, los agentes ya conectados no se actualizan solos.** La carpeta
viaja dentro del archivo de configuración de cada agente, así que cambiarla en MMCelt no la
cambia allí. Cuando eso pasa, el agente deja de lucir «conectado»: se avisa de que su
configuración apunta a otra carpeta, se muestra a cuál, y aparece el botón
**Actualizar la carpeta**, que vuelve a registrarlo con la que estás viendo y deja copia de
seguridad. La configuración de otro programa no se reescribe sola.

Al pulsar **Conectar**, MMCelt identifica además la carpeta mediante
`.mmcelt/configuracion.json`. El UUID se crea una sola vez y viaja con la carpeta. Si la mueves,
el programa detecta que la raíz cambió y ese mismo gesto explícito de volver a conectarla confirma
la ubicación nueva sin cambiar la identidad.

Cuando `mmcelt_create_mindmap` o `mmcelt_sync_ai_progress` escribe dentro de un proyecto ya
identificado, deja `.mmcelt/devolucion.json`. El recibo indica qué agente entregó qué mapa, si lo
generó o lo modificó y a qué proyecto pertenece. Es neutral: no distingue proveedores ni contiene
tokens. Lo pueden consultar la persona y el agente; no es una autorización que la aplicación use
para aceptar el contenido del mapa. Un proyecto antiguo sigue funcionando sin ese archivo hasta
que se conecta con la 0.9.9.

> Después de conectar hay que **reiniciar el agente**: ninguno relee su configuración
> mientras está en marcha.

### No necesita instalar nada más

Lo que se registra en el agente es **el propio ejecutable de MMCelt**, que lleva el
servidor integrado:

```
mmcelt.exe --mcp-server
```

Esto importa si le pasas la aplicación a otra persona: le funcionará aunque no tenga
ninguna otra herramienta instalada. Si puede ejecutar MMCelt, puede conectarlo con su
agente.

### Precauciones al modificar la configuración

Los archivos que se tocan pertenecen a otros programas, así que:

- Se hace **copia de seguridad** con marca de tiempo antes de modificarlos.
- Se **conserva todo lo demás**: los otros servidores MCP y el resto de ajustes.
- Un archivo con contenido ilegible **se deja intacto** y se avisa del problema.

| Herramienta | Función |
|---|---|
| `mmcelt_workspace_info` | Declara la carpeta autorizada y lista los mapas que contiene |
| `mmcelt_create_mindmap` | Crea un mapa y su Markdown acompañante |
| `mmcelt_read_mindmap` | Lee la jerarquía, notas, dudas y conexiones |
| `mmcelt_get_human_feedback` | Obtiene las correcciones exigidas por el usuario |
| `mmcelt_sync_ai_progress` | Desarrolla un mapa que ya existe: avance, rol, etiquetas y conexiones cruzadas |
| `mmcelt_export_ai_markdown` | Convierte un `.mmcelt` en Markdown |

**Caso de uso**: Claude Code trabaja en el código y va marcando tareas como completadas
en el mapa, mientras el usuario ve el avance en tiempo real en MMCelt y puede corregir el
rumbo marcando nodos.

**Desarrollar un mapa hecho por una persona**: el agente puede ampliar un mapa del usuario sin
sustituirlo. Añade nodos con su rol y sus etiquetas y traza conexiones cruzadas entre ramas
distintas, que es lo que hace falta para dibujar un bucle. No borra ni mueve nada de lo que hay,
sus etiquetas se suman a las del usuario en lugar de reemplazarlas, todo lo que añade queda
marcado como **Generado por IA**, no puede firmar nada como aprobado y no vuelve a enlazar un
camino que el usuario haya descartado. Los detalles del contrato están en
[Servicios](infraestructura/servicios.md).

**Nodos con el mismo título**: los títulos se repiten —dos «Sí» y dos «No» son lo normal en un
diagrama de decisiones—, así que un nodo se señala por su identificador. La lectura devuelve el
`id` de cada nodo y cada conexión con sus dos extremos por identificador **y** por título; la
escritura acepta `id`, `parent_id`, `from_id` y `to_id`, y comprueba que el título que los
acompañe corresponda al mismo nodo. Un título único sigue bastando; uno repetido rechaza la
llamada entera sin escribir nada y enumera los candidatos, en vez de elegir uno a ciegas.

**Contrato autodescriptivo**: los mapas guardados declaran `schema_version` y `generated_by`.
Los archivos anteriores a esa cabecera siguen siendo compatibles y la reciben al siguiente
guardado. El Markdown para IA publica además las seis herramientas MCP y los valores canónicos
que debe devolver, de modo que el modelo no tiene que adivinar el formato ni editar directamente
el `.mmcelt`.

### Búsqueda de nodos

En lo alto del panel de la derecha, siempre visible. Se escribe parte de un **título** o de una
**etiqueta** y debajo salen los nodos que coinciden; al pulsar uno queda seleccionado —con lo que
el inspector pasa a mostrarlo— y la vista va hasta él **sin cambiar el zoom**.

La comparación ignora mayúsculas y tildes, así que `diseno` encuentra «Diseño»: exigir la tilde
exacta convertiría la búsqueda en un examen de ortografía.

**Las notas quedan fuera a propósito.** Son párrafos largos y buscar dentro de ellos devolvería
medio mapa para cualquier palabra común, que es lo mismo que no devolver nada útil. Los
resultados se cortan en 50 y salen en el orden en que se leen en el mapa, de la raíz hacia abajo
y rama por rama, que es como los reconoce quien está buscando.

El recorrido no es recursivo y lleva registro de nodos visitados: un archivo con un ciclo entre
nodos —de los que la validación de entrada rechaza— haría que la búsqueda no terminara nunca, y
colgar el programa mientras alguien teclea es peor que no encontrar nada.

### Deshacer y rehacer

`Ctrl + Z` deshace y `Ctrl + Y` rehace; `Ctrl + Mayús + Z` también rehace, que es la forma
habitual en macOS y en buena parte de Linux. Las dos están al principio del menú `✏️ Edición`,
y se muestran apagadas cuando no hay nada que deshacer o rehacer.

Se guardan **cincuenta pasos**. Cada uno es una copia del mapa entero, no una orden invertible:
una copia no puede equivocarse al desandar el camino, y en un programa cuyo cometido es no
perder el trabajo del usuario esa garantía vale más que la memoria que ahorraría lo otro.

**Los cambios que van seguidos cuentan como uno.** Arrastrar un nodo modifica el mapa en cada
fotograma; sin agrupar, un arrastre de dos segundos dejaría más de cien pasos y deshacerlo
exigiría cien pulsaciones. Lo que ocurre a menos de medio segundo del cambio anterior se funde
con él, así que un arrastre completo —o una ráfaga de tecleo— se deshace de una vez.

Tres detalles que evitan sorpresas:

- **Abrir otro mapa olvida el historial del anterior.** Deshacer entonces traería de vuelta un
  mapa que no es el que está abierto, y lo pisaría.
- **Deshacer deja copia de recuperación**, igual que cualquier otra sustitución del mapa: lo
  que se deshace vive en memoria, y un cierre inesperado se lo llevaría.
- **Si el nodo seleccionado no existe en el estado al que se vuelve**, la selección pasa a la
  idea central. Con la selección apuntando a un nodo ausente, el inspector se queda en blanco
  y los atajos actúan sobre nada sin decir por qué.

Deshacer **no** toca el archivo en disco: cambia lo que hay en pantalla, y guardar sigue siendo
una decisión aparte.

### Idioma de la aplicación

Menú `🎨 Ver y Diseño → Idioma / Language`. **Seis idiomas**: español, inglés, francés, alemán,
ruso y chino simplificado. La elección se recuerda entre sesiones junto al tema y la escala.

#### La primera vez, el idioma lo pone el sistema

**Solo en el primer arranque**, cuando todavía no hay preferencias guardadas, MMCelt abre en el
idioma en que esté configurado el sistema operativo. Antes abría siempre en castellano: un Mac
en inglés mostraba el programa en español, y lo mismo ocurría en Windows y en Linux.

En cuanto el usuario elige un idioma en el menú, manda su elección y el sistema deja de
consultarse. Y a quien ya tenía preferencias guardadas **no se le cambia nada**: lo que veía
ayer es lo que sigue viendo hoy.

Cómo se averigua, que cambia según el sistema:

- **Windows** no publica el idioma en ninguna variable de entorno, así que se le pregunta con
  `GetUserDefaultLocaleName`, que devuelve la misma etiqueta que muestra la configuración
  regional (`es-ES`).
- **Linux y macOS** lo traen en las variables POSIX, que se miran por orden de precedencia:
  `LC_ALL`, `LC_MESSAGES` y `LANG` (`es_ES.UTF-8`).

Las dos notaciones se admiten, con su codificación y su modificador si los traen. Se compara el
subtag primario **entero** y nunca un prefijo, porque `et` (estonio) empieza igual que `es` y
confundirlos abriría el programa en un idioma ajeno.

Dos decisiones que conviene conocer:

- **Un idioma que MMCelt no habla deja el castellano**, que es el idioma en que se escribió el
  programa. No se elige un sustituto por proximidad.
- **El chino tradicional no se sirve como simplificado.** MMCelt solo habla simplificado; a
  alguien en Taiwán, Hong Kong o Macao mostrarle simplificado no sería traducirle, sería darle
  un texto que no es el suyo. Esos sistemas caen al respaldo.

**Está traducido todo lo que el usuario ve, sin excepciones**: la barra de menús con
sus ayudas emergentes y sus atajos, los avisos de la barra de estado, el inspector entero con los
nombres de estados, prioridades y roles, las ventanas modales, los diálogos del sistema para abrir
y guardar, y los tipos de conexión cruzada. Son **517 textos por idioma** (3.102 cadenas
en total), en `src/textos.rs`.

**La ayuda interna también.** Son sus 24 temas con las tres piezas de cada uno
—el rótulo del selector, el resumen de la galleta y la guía completa—: 72 piezas por idioma. Las
guías son documentos extensos y no van en `src/textos.rs`; viven en `assets/ayuda/<idioma>/<Tema>.md`
(144 archivos en total) y las incrusta `src/ui/ayuda_textos.rs`.

#### Soporte tipográfico CJK nativo y embebido

Para garantizar que el chino simplificado y los glifos CJK se lean con total nitidez en cualquier
sistema operativo (Windows, Linux y macOS) sin depender de si el usuario tiene instaladas fuentes
asiáticas en su sistema, el ejecutable incrusta un subconjunto optimizado de **Noto Sans SC**
(`assets/fuentes/NotoSansSC-subconjunto.ttf`, ~1,06 MB). Una prueba automatizada en la suite
(`todos_los_caracteres_cjk_del_programa_tienen_glifo_en_la_fuente_embebida`) comprueba sobre la
tabla `cmap` que cada carácter utilizado en los menús, diálogos y guías cuenta con su glifo
correspondiente, cubriendo además el estándar GB2312 nivel 1 para las notas del usuario.

#### El documento que se le entrega a la IA también va en el idioma del usuario

El Markdown que genera `🤖 Exportar Markdown para IA` —y el que el servidor MCP
escribe en el disco en cada informe de progreso— sale **en el idioma que tenga puesto la
interfaz**: sus nueve encabezados de sección, las instrucciones dirigidas al modelo, la matriz de
relaciones, el bloque de control humano y los tres prompts de acción sugeridos.

Antes salía siempre en castellano: las 72 cadenas que lo componen estaban escritas a mano dentro
de `src/ai_export.rs`. Quien ponía la interfaz en chino hablaba con su modelo en chino y recibía
un documento en español.

El servidor MCP corre sin ventana —lo lanza el agente de IA por su cuenta—, así que lee el idioma
del archivo de preferencias, una sola vez por proceso.

#### Soberanía de datos frente al cambio de idioma

Cambiar de idioma en la barra de herramientas actualiza al instante todos los rótulos de la
interfaz, el inspector, los menús y las plantillas de exportación a Markdown. Sin embargo, **los
mapas existentes del usuario nunca se mutan retroactivamente**. El título, las notas y las
etiquetas que el usuario escribió en sus nodos se conservan intactos.

Del mismo modo, el mapa de bienvenida (`nuevo_ejemplo_inicial`) y las plantillas de arquitectura
se instancian localizados en el idioma activo en el momento de su creación (incluyendo sus
etiquetas de ejemplo `#tech` y `#objetivo` / `#goal` / `#objectif` / `#ziel` / `#технологии` / `#цель` / `#技术` / `#目标`),
convirtiéndose desde ese instante en datos del usuario bajo la misma garantía de soberanía.

> **Sobre la procedencia de las traducciones.** Las de los cinco idiomas no castellanos se
> incorporaron por lotes, y ninguna entró sin pasar antes por comprobaciones automáticas: que
> estén todas las claves, que los emoji y los huecos coincidan en cantidad y **orden**, que la
> sintaxis Markdown y la sangría se conserven, y que las claves que hablan del mismo concepto no
> se contradigan entre ventanas. Esas comprobaciones forman parte de la batería de pruebas del
> proyecto, así que cualquiera puede volver a ejecutarlas: el resultado que se distribuye es
> `src/textos.rs` y las guías de `assets/ayuda/`.

Lo que no cambia nunca es el **contenido de los archivos**: los nombres de los campos del
`.mmcelt`, las claves del protocolo MCP y las de la configuración de los agentes. Son formato, no
texto para leer; traducirlos rompería los mapas guardados y la conexión con los agentes. Dos
pruebas lo vigilan con los literales copiados a mano a propósito, para que un renombrado masivo
no pueda voltear a la vez el código y la prueba que lo comprueba.

### Límites de seguridad

El servidor solo escribe **dentro del espacio de trabajo** definido en la variable
`MMCELT_WORKSPACE`, y **conserva una copia** antes de sobrescribir nada.

Cada operación tiene además su propia lista de extensiones: un mapa se escribe como `.mmcelt`
o `.json`, y un documento como `.md`. No es un detalle: mientras hubo una lista común para
todo, el exportador de Markdown podía escribir sobre un mapa y convertirlo en texto, y con eso
se rodeaba la protección de más abajo.

Y no se puede crear un mapa nuevo encima de otro que **ya contenga criterio humano** —
correcciones exigidas, descartes o aprobaciones—. Sobre un borrador que escribió la propia IA
sí: lo que se protege es tu revisión, no la existencia del archivo.

Si esa variable no está definida, el servidor **no opera con ningún archivo**: responde con un
error de seguridad y no hace nada. MMCelt la escribe en la configuración del agente al
conectarlo desde «🤖 Inteligencia Artificial → 🔌 Conectar MMCelt con mis IAs...»; si
falta, hay que volver a conectarlo desde ahí.

Cada copia lleva marca de tiempo —`proyecto.mmcelt.20260823-101530.bak`—, así que las
sucesivas conviven en lugar de pisarse.

Es el componente con más privilegio del sistema: da a un agente autónomo la capacidad de
escribir en el disco. Aquí no hace falta un atacante para que haya daño; basta una ruta
equivocada por el modelo.

---

## 13. Tamaño de la interfaz

Toda la interfaz —letras, botones, paneles y márgenes— se puede escalar entre el 50 % y
el 300 %.

| Cómo | Acción |
|---|---|
| `Ctrl` `+` | Agrandar |
| `Ctrl` `-` | Reducir |
| `Ctrl` `0` | Volver al tamaño normal |
| Menú `🎨 Ver y Diseño → Tamaño de la interfaz` | Elegir entre cinco tamaños predefinidos |

**El ajuste se recuerda al reiniciar**, junto con el tema visual.

### No confundir con el zoom del mapa

Son cosas distintas y es fácil mezclarlas:

| | Qué hace |
|---|---|
| **Rueda del ratón** | Acerca o aleja **el mapa**. Los textos de la interfaz no cambian |
| **`Ctrl` `+` / `-`** | Agranda **la interfaz**: letras, botones y paneles |

Si lo que ves pequeño son los menús y los botones, es el segundo.

### Tamaños predefinidos

| Escala | Para qué |
|---|---|
| 80 % | Compacto: más contenido en pantalla |
| 100 % | Tamaño nativo del sistema |
| 125 % | Pantallas grandes o vista cansada |
| 150 % | 4K o distancia media |
| 200 % | Televisores y pantallas vistas desde lejos |

### Qué se ajusta solo, y qué no

En el primer arranque se propone una escala según la resolución del monitor: una pantalla
4K sin escalado del sistema se agranda automáticamente, porque ahí la interfaz sale
diminuta para cualquiera.

**Pero hay un límite honesto**: ningún programa puede saber a qué distancia te sientas.
Un televisor de 42 pulgadas a tres metros le presenta al sistema operativo exactamente los
mismos datos que un monitor de 24 pulgadas a medio metro —misma resolución, misma
escala— y sin embargo necesitan tamaños de letra muy distintos.

Por eso la aplicación no intenta adivinarlo: propone un punto de partida razonable y
**recuerda lo que tú decidas**. Lo ajustas una vez y ya está.

---

## 14. Saber qué versión estás usando

La versión aparece en **el título de la ventana** y en la **barra de estado**, junto con la
fecha de compilación.

Para el detalle completo: `❓ Ayuda → ℹ️ Acerca de MMCelt`, o pulsando la versión en la
barra de estado. Muestra versión, fecha, commit, rama y —lo más útil— **la ruta del
ejecutable que se está ejecutando**.

Y sin abrir el programa:

```
mmcelt --version
```

### Por qué no basta el número de versión

El número de versión no cambia entre compilaciones. Dice qué versión *pretende* ser el
programa, no **qué binario concreto** has abierto.

Es la pregunta que surge cuando hay varias copias por el disco —la de `target/release`,
una portable en un USB, otra en una carpeta de trabajo—: todas se llaman igual y todas
dicen lo mismo. La fecha, el commit y la ruta sí las distinguen.

Si el binario se compiló con cambios sin guardar en el repositorio, se avisa: en ese caso
no corresponde exactamente a ningún commit.

### Aviso de versión nueva

Al arrancar, MMCelt pregunta a GitHub cuál es la última versión publicada. Si es más nueva que
la que se está ejecutando, la barra superior muestra `⬆ Nueva versión disponible:` con su número,
como enlace a la página de esa versión. **No descarga ni instala nada**: abrir el enlace y
actualizar es decisión del usuario.

- Las versiones se comparan como números (`0.12.0` es posterior a `0.9.9`), y solo se avisa de
  una estrictamente mayor. Una etiqueta que no sea `X.Y.Z` no provoca aviso.
- La consulta corre en un hilo aparte: la ventana nunca espera por ella. Sin conexión, con el
  límite de peticiones de GitHub agotado o ante una respuesta rara, no hay aviso ni error
  visible; el fallo queda en el registro.
- Se desactiva en `🎨 Ver y Diseño` → `Comprobar al arrancar si hay una versión nueva`, y la
  elección se recuerda. Desactivada, el programa no se conecta a internet al arrancar.

**Caso de uso**: quien tiene una copia portable o la instaló hace tiempo se entera de que hay
versión nueva sin tener que ir a mirar el repositorio.

---

## 13 bis. La aprobación humana caduca

Marcar un nodo como **Aprobado por Humano** es firmar ese contenido. Si después la IA lo
cambia —las notas, el estado, la prioridad o la ruta de archivo—, el nodo vuelve solo a
**Pendiente de revisión**.

La razón es que se aprueba un texto, no un nodo para siempre. Sin esta caducidad, el usuario
vería su propio visto bueno sobre algo que no ha leído nunca, y la supervisión dejaría de
significar nada.

| Situación | Qué pasa con la aprobación |
|---|---|
| La IA cambia las notas, el estado, la prioridad o la ruta | Caduca: vuelve a «Pendiente de revisión» |
| La IA envía los mismos valores que ya había | Se conserva: no ha cambiado nada |
| La IA crea un nodo nuevo | Nace como «Generado por IA», sin aprobación que perder |
| El nodo estaba en otro estado de revisión | No se toca: solo la aprobación afirma algo que el cambio invalida |

La corrección exigida **no se borra nunca**, ni siquiera cuando la IA la acata, para que se
pueda comprobar si hizo lo que se le pidió.

Un agente **no puede declarar por su cuenta que algo está aprobado**: todo lo que crea por
el servidor MCP nace marcado como generado por IA. Si pudiera firmar su propio trabajo, la
supervisión sería un adorno.

---

## 14 bis. Qué ocurre al importar un mapa de la IA

Importar **sustituye el mapa abierto**. No se mezcla con lo que hubiera: lo que llega del
modelo ocupa su lugar.

Antes de sustituirlo se escribe una copia de recuperación del mapa anterior, en la misma
carpeta que usa el guardado automático. Se ofrece al arrancar el programa, igual que la de
un cierre inesperado, así que para volver atrás hay que cerrar y abrir MMCelt.

| Qué viaja en el JSON de la IA | Qué se pierde al importar |
|---|---|
| Títulos, notas y jerarquía | Las posiciones colocadas a mano |
| Rutas de archivo (`file_path`) | Los nodos plegados |
| Etiquetas, roles, estados y prioridades | |
| Conexiones cruzadas y su tipo | |
| Estado de revisión y correcciones exigidas | |

La distinción es deliberada: el modelo devuelve **contenido y estructura**, no
presentación. El mapa importado se recoloca con la disposición automática.

---

## 15. Temas visuales y ayuda

Tres temas, en el menú `🎨 Ver y Diseño → Tema Visual`, con paletas completas y colores por
rama que se reparten cíclicamente. El tema elegido se recuerda entre sesiones.

| Tema | Aspecto | Cuándo usarlo |
|---|---|---|
| 🌙 **Oscuro** (por defecto) | Azul pizarra. Lienzo, panel y nodo son tres tonos escalonados, de forma que las capas se separan sin bordes marcados | Sesiones largas y luz baja |
| ☀️ **Claro** | Papel cálido: fondo marfil, nodos blancos que flotan sobre él y textos en gris tinta | Luz de día o pantallas muy brillantes |
| 👁️ **Alto contraste** | Negro, bordes blancos sólidos y colores saturados | Problemas de visión o reflejos en la pantalla |

**El fondo del tema claro no es blanco a propósito.** Con el lienzo blanco, los nodos
—que también son blancos— se quedan sin nada sobre lo que destacar, y una pantalla de
blanco puro cansa la vista antes. El marfil deja que los nodos se lean como fichas
apoyadas sobre una mesa.

### El tema viste toda la ventana

La paleta no se queda en el lienzo: la barra de menús, el panel lateral, la barra de
estado, los botones, los desplegables, los campos de texto y los cuadros de diálogo se
pintan con los mismos colores. Antes no era así, y el tema claro se veía partido: lienzo
claro rodeado de barras oscuras. Está explicado en
[`arquitectura.md`](arquitectura.md#16-el-tema-viste-toda-la-ventana).

### Contraste comprobado

Las tres paletas están sujetas a pruebas automáticas que miden el contraste según la
norma de accesibilidad WCAG 2.1: texto sobre cada superficie por encima de 4,5:1, y
colores de rama, líneas y bordes por encima de 3:1. Un ajuste estético que deje algo
ilegible rompe la compilación.

El sistema de ayuda tiene dos niveles: **galletas** informativas junto a los controles,
con un botón `+info`, y un **panel lateral** con guías extensas de veinticuatro temas (con galletas reactivables desde `❓ Ayuda → 🍪 Galletas de Ayuda para Principiantes`),
desde los primeros pasos hasta la integración con cada plataforma de IA.

---

## 16. El icono

Un mapa mental reducido a lo mínimo: el nodo raíz a la izquierda y dos ramas que se abren
hacia la derecha. La de abajo termina en un nodo corriente; **la de arriba no termina en
un nodo, sino en un destello**. Ahí está la inteligencia artificial, que es lo que separa
a MMCelt de cualquier otro programa de mapas mentales.

Los colores no son inventados: salen de la paleta del tema oscuro. El fondo es el
degradado del panel al lienzo, las dos ramas son la primera y la segunda del mapa, y el
destello es el ámbar de las conexiones cruzadas.

### Dónde aparece

| Sitio | Quién lo pone |
|---|---|
| Barra de título y barra de tareas | `main.rs`, con los píxeles de `mmcelt-64.rgba` |
| Explorador de archivos y accesos directos | `build.rs`, incrustando `mmcelt.ico` como recurso del ejecutable |
| Dock y Finder de macOS | El paquete `.app`, con `mmcelt.icns` |

Son mecanismos distintos porque resuelven cosas distintas: el primero es del programa en
marcha y funciona en cualquier sistema; el segundo es del archivo `.exe` y solo interviene
al compilar en Windows; el tercero es del paquete de macOS, que exige el formato propio de
Apple y no acepta ni `.ico` ni `.png`.

### Los archivos

```
assets/icono/
├── mmcelt.svg          Fuente editable. Manda esta.
├── mmcelt.ico          16, 24, 32, 48, 64, 128 y 256 px, para el ejecutable
├── mmcelt.icns         11 variantes de 16 a 1024 px, para el paquete de macOS
├── mmcelt-256.png      Para documentación y tiendas
├── mmcelt-128.png
├── mmcelt-64.png
└── mmcelt-64.rgba      Píxeles en crudo que incrusta `main.rs`
```

El `.rgba` se guarda sin comprimir a propósito: descodificar un PNG obligaría a añadir una
dependencia de imágenes al programa, y son solo 16 KB.

### El icono de macOS y por qué tiene su propio guion

`iconutil`, la herramienta con la que Apple construye un `.icns`, **solo existe en macOS**,
y este proyecto se desarrolla en Windows. Generarlo en la integración continua habría
dejado el icono fuera del repositorio, y aquí rige lo contrario: lo que se genera se
confirma, porque lo que no está en git no lo protege nadie.

Así que se construye con
[`documentacion/infraestructura/generar-icono-macos.py`](infraestructura/generar-icono-macos.py),
que rasteriza la geometría del SVG y la empaqueta en el formato de Apple:

```bash
python documentacion/infraestructura/generar-icono-macos.py
```

**Los píxeles se dibujan de nuevo, no se amplían desde los PNG.** El mayor que había mide
256 y macOS pide 1024: ampliarlo habría dado un icono borroso justo en el tamaño más
visible, el de la ventana «Obtener información».

Que macOS lo acepta no se supone: **la integración continua lo descompone con `iconutil`
en cada cambio**, en un ejecutor real de macOS. Un archivo construido contra una
especificación puede ser correcto sobre el papel y no abrirse en el sistema, y esa
diferencia solo se ve ejecutándolo donde importa.

**Si cambias el diseño**, edita el SVG y regenera el resto. El guion del `.icns` lleva la
geometría copiada del SVG en constantes con nombre, así que hay que actualizarla con él.
Si el `.rgba` deja de medir 64×64, la prueba `el_icono_de_la_ventana_tiene_el_tamano_declarado`
lo detecta.

---

## 17. Qué está comprobado en macOS, y qué no

MMCelt se desarrolla en Windows, así que conviene decir con precisión hasta dónde llega hoy el
soporte de macOS. Nada de lo que sigue es una suposición: todo se mide en cada cambio, en un
ejecutor `macos-latest` con **Apple Silicon**, dentro de la integración continua.

**Comprobado, y vigilado a partir de ahora:**

| Qué | Cómo se comprueba |
|---|---|
| Compila sin avisos | `cargo clippy --all-targets --all-features -- -D warnings` |
| Las 393 pruebas de Windows y las 392 de Linux pasan | `cargo test --all-features`, incluidas las comprobaciones de enlaces propias de cada sistema |
| **La ventana se abre** | Se lanza el binario de publicación y se exige que el proceso siga vivo pasados veinte segundos. Si `eframe` no consiguiera un contexto de OpenGL, moriría en el acto |
| **Cómo se ve** | Una captura de la ventana se sube como artefacto del flujo, para mirarla |
| El icono del paquete | `iconutil` descompone `mmcelt.icns` en sus 11 variantes y `sips` confirma que la mayor es de 1024×1024 real |

Que la ventana se abra confirma de paso algo que no era evidente: **OpenGL sigue respondiendo en
Apple Silicon**, donde Apple lo mantiene solo como capa de compatibilidad ya declarada obsoleta.
Si hubiera fallado, habría que haber cambiado el motor gráfico del programa.

**Lo que todavía NO está comprobado**, y conviene no darlo por hecho:

- **Nadie ha manejado la ventana en un Mac.** La integración continua mira, no toca: no hay
  clics, ni escritura, ni diálogos de archivo, ni atajos con Cmd. En este proyecto los dos peores
  defectos aparecieron con toda la batería en verde y los vio una persona mirando la pantalla.
- **No hay paquete `.app`.** Lo que se ejecuta es el binario suelto. Sin `Info.plist` no hay
  `NSHighResolutionCapable`, así que **no se sabe cómo se ve en una pantalla Retina**.
- **El aviso de Gatekeeper no está resuelto.** Sin certificado de Apple, macOS pone en cuarentena
  lo que se descarga y dice que «está dañado». Se abre con clic derecho → *Abrir*, o quitando el
  atributo con `xattr -d com.apple.quarantine`.

Por eso las descargas publicadas siguen siendo solo de Windows y Linux.

---

## Resumen de atajos

| Atajo | Acción |
|---|---|
| `Tab` / `Insertar` | Crear nodo hijo |
| `Enter` | Crear nodo hermano |
| `Espacio` / `F2` | Editar el título |
| `Supr` / `Retroceso` | Eliminar nodo y su descendencia |
| `Ctrl + S` | Guardar |
| `Ctrl + E` | Exportar Markdown para IA |
| `Inicio` | Encuadrar el mapa entero en la ventana (solo con el teclado libre) |
| `Ctrl + F` | Llevar el cursor al buscador con su texto seleccionado |
| `Ctrl +` / `Ctrl -` | Agrandar o reducir **la interfaz** (letras y botones) |
| `Ctrl + 0` | Interfaz a tamaño normal |
| Rueda del ratón | Acercar / alejar **el mapa** |
| Clic central o derecho | Desplazar la vista |
| Doble clic | Editar el nodo |
