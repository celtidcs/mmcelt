# 🔗 Conexiones Cruzadas y Dependencias

Sirven para decir algo que la estructura de ramas **no puede expresar**: que dos partes del
proyecto, colgadas de sitios distintos, dependen la una de la otra.

### 🌳 Por qué hacen falta

Un mapa mental es un árbol: cada idea cuelga de otra. Eso funciona hasta que te encuentras con
que «el sistema de pagos» necesita «el registro de usuarios», y resulta que están en dos ramas
que no se tocan.

Puedes moverlos para que queden juntos, pero entonces el mapa deja de reflejar cómo piensas y
pasa a reflejar una limitación de la herramienta. Las conexiones cruzadas evitan eso: **dejas
cada idea donde tiene sentido y trazas la relación aparte.**

### 🏷️ Los cinco tipos, y cuándo usar cada uno

El tipo que elijas **cambia lo que la IA entiende**, así que conviene no elegir al azar.

- **➡️ Dependencia (Requiere):** A no puede funcionar sin B. Es la más común.
  *Ejemplo: «Enviar factura» requiere «Datos fiscales del cliente».*
- **⛔ Bloquea a:** B no puede avanzar mientras A no se resuelva. Es más fuerte que una
  dependencia: describe algo **detenido ahora mismo**.
  *Ejemplo: «Decidir la base de datos» bloquea «Diseñar el modelo de datos».*
- **🔀 Alternativa a:** son dos caminos **excluyentes**; elegir uno descarta el otro.
  *Ejemplo: «Aplicación de escritorio» es alternativa a «Aplicación web».*
- **✨ Sinergia con:** no se necesitan, pero juntos valen más que por separado.
  *Ejemplo: «Sistema de etiquetas» y «Buscador».*
- **💡 Inspirado por:** una referencia. Copias una idea o un patrón de otro sitio del mapa.
  *Ejemplo: «Panel de administración» inspirado por «Panel de usuario».*

### 🛠️ Cómo se crea

1. Selecciona el nodo de origen: el que **necesita**, **bloquea** o **se inspira**.
2. Pulsa **`🔗 Conexión`** en el inspector de la derecha.
   También está en **`✏️ Edición`** → **`🔗 Crear Conexión Cruzada / Dependencia...`**.
3. Elige el nodo destino, el tipo, y escribe **por qué** existe esa relación.

**El orden importa.** «A requiere B» y «B requiere A» son cosas distintas, y la IA lo lee tal
cual. Si te equivocas, borra la conexión y créala al revés.

**Y el motivo importa más de lo que parece.** Una conexión sin explicación le dice a la IA que hay
una relación, pero no qué hacer con ella. Con el motivo escrito, puede razonar sobre ella.

### ✋ Cuándo NO usarlas

Es igual de importante, porque un mapa lleno de flechas cruzadas no se entiende:

- **Si la relación es «una cosa es parte de la otra»**, eso no es una conexión cruzada: es un
  hijo. Cuélgalo donde corresponde.
- **Si todo depende de todo**, no traces treinta conexiones. Suele ser señal de que falta un
  nodo que reúna esa idea común.
- **Si la relación es obvia** —dos tareas del mismo bloque que evidentemente van seguidas—, no
  ganas nada dibujándola.

Una regla práctica: **traza la conexión si te ha costado darte cuenta de que existía.** Esas son
las que la IA no puede deducir sola, y las que se olvidan en las reuniones.

### 👀 Dónde se ven después

Las conexiones que salen o llegan a un nodo aparecen en el inspector de la derecha, en
**`🔗 Conexiones Cruzadas:`**, y desde ahí se pueden borrar. Esa sección **solo aparece si el nodo
tiene alguna**, así que si no la ves, es que ese nodo no tiene ninguna.

En el lienzo se dibujan como curvas, con un estilo distinto según el tipo. Y en el documento que
se le manda a la IA viajan en una sección propia, la matriz de dependencias.
