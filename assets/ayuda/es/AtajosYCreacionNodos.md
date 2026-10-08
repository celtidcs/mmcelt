# ⌨️ Creación Rápida y Atajos de Teclado

MMCelt está diseñado para permitirte crear y editar ramas a la velocidad del pensamiento sin levantar las manos del teclado.

### ⚡ Tabla de Atajos Comerciales Estándar:
| Tecla | Acción Inmediata |
| :--- | :--- |
| **`Tab` / `Insertar`** | Añade un **nodo hijo** bajo el nodo que tengas seleccionado. |
| **`Enter`** | Añade un **nodo hermano** al mismo nivel jerárquico. |
| **`Espacio` / `F2`** | Abre el editor de texto en el propio lienzo sobre el nodo. |
| **`Supr` / `Retroceso`** | Elimina el nodo seleccionado y todas sus ramas hijas de forma segura. |
| **`Doble Clic`** | Entra en modo edición de texto del nodo. |
| **`Ctrl + N`** | Empieza un mapa nuevo. Del anterior queda copia de recuperación. |
| **`Ctrl + S`** | Guarda el mapa mental en disco (`.mmcelt`). |
| **`Ctrl + E`** | Exporta el archivo Markdown enriquecido para IA (`.md`). |
| **`Inicio`** | Centra la cámara en la idea central del proyecto. |
| **`Ctrl + F`** | Lleva el cursor al buscador del panel lateral, con lo escrito seleccionado para buscar otra cosa. |
| **`Ctrl + Z`** | Deshace el último cambio. Los cambios seguidos se deshacen de una vez. |
| **`Ctrl + Y`** | Rehace lo último que se deshizo. También vale `Ctrl + Mayús + Z`. |

### ✍️ Mientras escribes, los atajos que tocan el mapa callan
`Tab`, `Insertar`, `Enter`, `Supr`, `Retroceso`, `Espacio`, `F2`, `Inicio` y `Ctrl + N` solo actúan con el teclado
libre. En cuanto escribes en cualquier campo —las notas de un nodo, la visión del proyecto,
el cuadro donde pegas la respuesta de la IA—, esas teclas hacen lo que esperas de ellas al
escribir: `Retroceso` borra una letra, no el nodo, e `Inicio` lleva el cursor al principio
de la línea sin mover el mapa.

`Ctrl + S`, `Ctrl + E`, `Ctrl + F`, `Ctrl + Z` y `Ctrl + Y` sí siguen valiendo mientras escribes.

### 💡 Consejo Pro:
Selecciona el nodo central y pulsa `Tab` repetidamente para crear rápidamente 4 o 5 pilares estructurales.

### 🔍 Encontrar un nodo en un mapa grande

Arriba del panel de la derecha hay un buscador. Escribe parte de un **título** o de una
**etiqueta** y debajo aparecen los nodos que coinciden; al pulsar uno queda seleccionado y la
vista va hasta él **sin cambiar el zoom**, para que no pierdas el nivel de detalle al que
estabas mirando.

No distingue mayúsculas ni tildes: `diseno` encuentra «Diseño». Las **notas no se buscan**: son
párrafos largos, y cualquier palabra común devolvería medio mapa. La lista se corta en 50
resultados; con más aciertos que eso, lo que hace falta es afinar la búsqueda.

### ✏️ Clic derecho sobre un nodo: «Acciones del nodo»
Un clic derecho sobre una tarjeta la selecciona y abre el menú **Acciones del nodo** en ese punto, con lo mismo que el menú `✏️ Edición` y el inspector: añadir hijo (`Tab` / `Insertar`), añadir hermano (`Enter`, no disponible en la raíz), crear una conexión cruzada, editar el título (`Espacio` / `F2`), eliminar (`Supr`), y los submenús de estado, prioridad, control humano y rol, con el valor actual marcado.

Arrastrar con el botón derecho sigue moviendo la vista y no abre el menú. `Esc` o un clic fuera lo cierran sin cambiar nada. Con el mapa en solo lectura no se abre.
