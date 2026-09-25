# 📥 Importación Bidireccional desde IA

Te permite traer a MMCelt cualquier proyecto, conversación o especificación generada por ChatGPT, Claude o Gemini.

### 🔄 Cómo utilizarlo:
1. En MMCelt ve a **`🤖 Inteligencia Artificial` > `📋 Copiar Prompt Maestro para IA...`**.
2. Pega esa instrucción en tu chat con la IA junto con tus ideas, código o documentos.
3. La IA te devolverá un bloque estructurado en formato JSON.
4. En MMCelt ve a **`🤖 Inteligencia Artificial` > `📥 Importar desde IA (ChatGPT, Claude, Gemini)...`**.
5. Pega el bloque devuelto y pulsa **`✨ Sustituir el mapa por el de la IA`**.
6. MMCelt reconstruye el mapa completo con sus notas, etiquetas, estados, rutas de código y disposición automática.

### ⚠️ Importar sustituye, no añade
El mapa que tengas abierto **se pierde**: lo que llega de la IA ocupa su lugar, no se mezcla con lo que había.

Antes de sustituirlo, MMCelt guarda una copia de seguridad del mapa anterior. Si te das cuenta de que no querías importar, cierra el programa y vuelve a abrirlo: al arrancar te ofrecerá recuperar ese mapa.

Aun así, la costumbre sana es **guardar con `Ctrl+S` antes de importar**. La copia de seguridad es una red, no un sustituto del guardado.

### 🎨 Lo que no viaja en el JSON
La IA devuelve la estructura y el contenido, no la presentación. Al importar se pierden las posiciones que hubieras ajustado a mano y las ramas que tuvieras plegadas: el mapa se recoloca con la disposición automática.

Lo que sí viaja completo es lo que importa para trabajar: títulos, notas, rutas de archivo, etiquetas, roles, estados, prioridades, conexiones cruzadas y las correcciones que hayas exigido.
