# 📐 Modos de Disposición Espacial

Para que tu mapa mental siempre se vea claro, limpio y legible, MMCelt cuenta con algoritmos que calculan la posición de cada nodo y evitan que las ramas se solapen.

Puedes elegir cómo organizar tu mapa en cualquier momento según tus preferencias visuales:

### 🎨 Modos Disponibles:

1. **Árbol Balanceado (Izq/Der):**
   - Sitúa la idea central en el origen del lienzo.
   - Distribuye las ramas principales de forma equilibrada a izquierda y derecha, repartiendo el peso visual equitativamente.
   - Calcula la altura de cada subrama de manera automática para que los textos nunca se pisen entre sí. Es el modo por defecto y el más cómodo para la mayoría de proyectos.

2. **Radial / Circular:**
   - Coloca la idea central en el punto medio y despliega las ramas en un abanico circular de 360 grados a su alrededor.
   - Es ideal para sesiones de tormenta de ideas (*brainstorming*) o mapas con muchos temas principales alrededor de un concepto único.

3. **Posición Libre Manual:**
   - Te permite mover cualquier nodo libremente arrastrándolo con el botón izquierdo del ratón a la posición exacta que desees.
   - En este modo el motor no fuerza la posición de los nodos, dándote control manual total para diagramar esquemas personalizados.

### ⚡ Cómo cambiar de modo y reorganizar

- En la barra superior, abre el menú **`🎨 Ver y Diseño`** para seleccionar el modo de disposición que prefieras.
- Si en algún momento deseas recalcular las posiciones y ordenar el mapa, pulsa en **`🎨 Ver y Diseño`** → **`🔄 Reorganizar los nodos`** (o utiliza la tecla rápida correspondiente). Todos los nodos volverán a alinearse con armonía.

### 🔀 Soltar un nodo encima de otro
Si arrastras un nodo y lo sueltas con su centro encima de otra tarjeta, aparece un pequeño menú en ese punto para que decidas qué querías hacer:

- **`➕ Hacer hijo`**: el nodo, con todas sus ramas, pasa a colgar del nodo de debajo. Aparece desactivado si no es posible: el de debajo ya es su padre, el que arrastras es la raíz, o el de debajo está dentro de sus propias ramas (se formaría un bucle).
- **`↔ Hacer hermano`**: el nodo pasa a colgar del mismo padre que el de debajo, justo detrás de él. No está disponible si el de debajo es la raíz o si ya son hermanos.
- **`🔗 Conectar con enlace`**: crea una conexión cruzada hacia el nodo de debajo y devuelve el que arrastraste a su sitio. No está disponible si ya son padre e hijo o ya están conectados: sería una segunda línea encima de la que ya los une.
- **`➡ Mover aquí sin tapar`**: lo deja junto al nodo de debajo, en el hueco libre más cercano, sin tapar ninguna tarjeta.
- **`↩ Cancelar`**: lo devuelve a donde estaba. `Esc` o un clic fuera del menú hacen lo mismo.

Mientras el menú está abierto, las teclas que cambian el mapa no actúan. Cualquiera de las opciones se puede deshacer con `Ctrl + Z`.
