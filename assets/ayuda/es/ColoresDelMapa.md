# 🎨 Qué Significan los Colores

Los colores **no son decorativos**: dicen a qué rama pertenece cada cosa.

### 🌈 Las líneas que unen los nodos
Cada rama principal —cada hijo directo de la idea central— recibe un color, y **toda su descendencia lo hereda**:

- 1.ª rama: Azul
- 2.ª rama: Esmeralda
- 3.ª rama: Ámbar
- 4.ª rama: Púrpura
- 5.ª rama: Rosa
- 6.ª rama: Turquesa
- 7.ª rama: Naranja
- 8.ª rama: Índigo

Eso te permite **seguir cualquier hilo hasta el final** y saber de dónde viene, aunque el mapa esté enredado y las ramas se crucen por la pantalla.

Con más de ocho ramas la paleta se reutiliza: la novena vuelve al azul.

### 👁️ En el tema de Alto Contraste los colores son otros
Si tienes puesto **👁️ Alto Contraste**, esa lista no coincide con lo que ves: la primera rama no es azul, es **cyan** —un cyan puro y saturado, sobre fondo negro y con borde blanco—, y las demás también cambian a colores saturados.

No es un fallo ni un descuido. El azul de la lista de arriba es un azul medio, y sobre fondo negro **no llega al contraste que exigen las pautas de accesibilidad WCAG**: quien puso ese tema lo puso precisamente porque lo necesita, así que ahí la paleta se elige por contraste medible y no por armonía visual. El cyan puro es de los colores que más contrastan contra el negro.

Lo que **no** cambia es la regla: cada rama sigue teniendo su color y su descendencia lo sigue heredando. Solo cambia qué color le toca a cada una.

### 🟠 Las conexiones cruzadas son distintas
Van siempre en **ámbar**, con un color propio que no depende de la rama. Así distingues de un vistazo lo que es jerarquía de lo que es una dependencia entre ramas distintas.

Todas se dibujan igual; lo que las diferencia es **la etiqueta de texto** de su punto medio: la que escribiste al crearla o, si la dejaste vacía, el nombre del tipo de conexión.

En el diagrama Mermaid del documento exportado sí cambian de forma, para que el modelo distinga el tipo de un vistazo.

### 🌍 El idioma de la aplicación
Menú **`🎨 Ver y Diseño` → `Idioma / Language`**. Están disponibles el español, el inglés, el francés, el alemán, el ruso y el chino simplificado, y la elección se recuerda al cerrar el programa.

El programa entero habla los seis: los menús, los avisos, el inspector, las ventanas, los diálogos, esta misma ayuda y el documento que se le entrega a la IA.

Lo que **nunca** cambia de idioma es lo que va dentro de tus archivos: los nombres de los campos del `.mmcelt`, lo que se le manda a la IA y lo que se guarda en la configuración de los agentes. Son formato, no texto para leer, y traducirlos rompería los mapas ya guardados.

### 🌗 Los tres temas
Menú **`🎨 Ver y Diseño` → `Tema Visual:`**. Los tres significan lo mismo —el azul sigue siendo la primera rama en todos— y cambian el fondo sobre el que se lee.

- **🌙 Tema Oscuro** (el de partida): azul pizarra. El lienzo, los paneles y los nodos son tres tonos distintos, de más oscuro a más claro, así que las capas se distinguen sin bordes marcados. Es el que menos cansa con luz baja.
- **☀️ Tema Claro**: papel cálido. El fondo es marfil y no blanco, a propósito: con el lienzo blanco, los nodos —que también son blancos— no tienen sobre qué destacar, y una pantalla de blanco puro cansa antes. Los textos van en gris tinta en vez de negro.
- **👁️ Alto Contraste**: fondo negro, bordes blancos sólidos y colores saturados. Pensado para problemas de visión o para trabajar con reflejos en la pantalla.

**El tema se recuerda al reiniciar**, junto con el tamaño de la interfaz.

Los colores de los tres temas están comprobados con pruebas automáticas contra el mínimo de contraste de la norma de accesibilidad WCAG: ningún texto queda por debajo del umbral de legibilidad sobre su propio fondo.
