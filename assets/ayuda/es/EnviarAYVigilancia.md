# 📤 «Enviar a...», consola del agente y vigilancia

«Enviar a...» prepara una sesión supervisada con un agente de programación y deja el bucle cerrado. Si el destino tiene consola oficial, la conversación ocurre ahí; si solo habla MCP, MMCelt deja el material preparado y el agente lo recoge desde su propia interfaz. En los dos casos el trabajo vuelve al mapa por el servidor MCP de MMCelt.

### 👁️ Elegir un agente no envía nada todavía
Al pulsar un agente se abre una **vista previa** con el prompt completo, dividido en cuatro bloques:

1. **Contrato MMCelt** (solo lectura): las reglas mínimas de integridad. Descubrir el espacio autorizado, leer las decisiones humanas antes de escribir, no tocar el `.mmcelt` a mano y devolver avances por MCP. Se muestra protegido para que no se borre sin querer.
2. **Reglas comunes del proyecto** (solo lectura): tu perfil neutral, el mismo para todos los agentes.
3. **Contexto del mapa** (solo lectura): lo genera la exportación para IA. Se cambia en el mapa, que es su origen.
4. **Encargo de esta sesión** (solo lectura): qué quieres conseguir ahora. Se conserva de una sesión a otra hasta que lo cambies.

Ningún bloque es editable en la vista previa. Las reglas y el encargo se escriben en `✏️ Edición` → `🧭 Proyecto e instrucciones para la IA`, pestañas **Instrucciones** y **Plantillas de encargo**. Esa ventana los guarda en el proyecto al cerrarla, y `Guardar como reglas comunes` guarda las reglas en `.mmcelt/instrucciones-agente.md` sin cerrarla. Si el encargo está vacío, la vista previa lo avisa, no deja confirmar y ofrece `Configurar encargo...`, que abre esa ventana en la pestaña del encargo. `Cancelar` no guarda, no exporta, no vigila y no deja expediente.

### ⚖️ Precedencia declarada en el propio prompt
Correcciones humanas del mapa **>** contrato MMCelt **>** reglas del proyecto **>** encargo de la sesión.

### 📚 Instrucciones nativas: solo entran si las aceptas
Si en la raíz del proyecto hay `AGENTS.md`, `CLAUDE.md` o `GEMINI.md`, la vista previa los enumera con su estado, pero **no los incorpora automáticamente**. Cada archivo se acepta por separado en la pestaña **Instrucciones** de esa misma ventana, con `Aceptar esta versión`, y desde entonces su contenido se añade a las reglas comunes del prompt. Si el archivo cambia después, la aceptación deja de valer: aparece como `Fuente modificada; requiere otra revisión` y no se incorpora hasta que vuelvas a aceptarlo.

### 🚀 Qué ocurre al confirmar
El botón dice lo que va a pasar de verdad: `Iniciar en consola` cuando MMCelt ha localizado la consola del agente, y `Preparar para MCP` cuando no la hay. En ese segundo caso la vista previa lo advierte antes, con un aviso destacado sobre el nombre del agente.

**Con consola**, el orden es fijo y cada paso tiene que salir bien antes del siguiente:

1. **Guarda el mapa** en su archivo `.mmcelt`.
2. **Exporta el documento para la IA** junto al mapa, con el sufijo `_AI.md`.
3. **Escribe el expediente** de la sesión.
4. **Abre la consola** del agente en la carpeta del proyecto.
5. **Activa la vigilancia** del mapa.

Si la consola no llega a abrirse, MMCelt **no dice «enviado»**, no empieza a vigilar y deja el expediente marcado como fallido.

**Sin consola** —un agente cuyo servidor MCP está registrado pero cuyo ejecutable MMCelt no localiza— se hace todo lo demás: guarda el mapa, exporta el `_AI.md`, escribe el expediente y activa la vigilancia. Lo único que no ocurre es el cuarto paso, y el expediente queda en estado **preparado** en vez de iniciado, porque MMCelt no ha arrancado nada. El agente recoge el trabajo desde su propia interfaz y lo devuelve por MCP como cualquier otro.

### 🗄️ Un proyecto de una versión anterior no se toca al mirarlo
Una carpeta que se usó con una versión anterior todavía no tiene `.mmcelt/configuracion.json`. Abrir la vista previa **no lo crea**: el prompt se compone en memoria y en el disco no se escribe nada. La identidad del proyecto se escribe al confirmar, que es el gesto explícito. Si cancelas, la carpeta queda exactamente como estaba.

### 🗂️ El expediente: `.mmcelt/sesiones`
Cada confirmación crea una carpeta con fecha, agente e identificador. Dentro hay dos archivos:

- `inicio.md`: el prompt exacto que se aprobó, letra por letra.
- `sesion.json`: versión de formato, identificador, fecha UTC, agente, ruta del mapa, fuentes detectadas e incorporadas, tamaño del prompt y estado (preparado, iniciado o fallido).

No se escriben contraseñas, claves, correos ni identificadores de cuenta. Es material local tuyo.

### 🔁 Repetir una sesión
Abre la consola del agente en la carpeta del proyecto y pídele que lea el `inicio.md` de esa carpeta. Recibirá lo mismo que la primera vez, sin depender del portapapeles ni de recordar qué escribiste. Sirve para reproducir un fallo y para comparar cómo responden dos agentes al mismo encargo.

### 🔑 Las cuentas las pone el CLI, no MMCelt
MMCelt no pide, no guarda y no transforma credenciales. Se apoya en la sesión que ya tienes autenticada en `claude`, `codex` o `agy`. No usa ninguna API de pago, no elige modelo y no decide facturación.

### 🔗 MCP y consola son dos capacidades separadas
El menú «Enviar a...» marca cada agente con un icono:

- ✨ **Consola y MCP**: puede conversar y devolver trabajo al mapa.
- 🖥 **Solo consola**: MMCelt encuentra su ejecutable, pero falta registrar MCP.
- 🔗 **Solo MCP**: puede devolver trabajo, pero MMCelt no puede abrirle una consola.
- 🔌 **Ninguna de las dos**.

Que un agente figure conectado por MCP **no significa** que se le pueda abrir una consola.

### 🌌 Agentes oficiales de consola (CLI)
MMCelt conserva exclusivamente los tres clientes oficiales de terminal: **Claude Code**, **Codex CLI** y **Antigravity CLI (agy)**. Los clientes gráficos y no oficiales han sido retirados para garantizar que solo se ofrecen opciones verificadas y sencillas de manejar.

### 💻 Dónde está comprobado y dónde solo está implementado
- **Windows**: es la plataforma con recorrido comprobado. Abre una ventana de consola nueva directamente sobre el ejecutable del agente. Para localizarlo manda `PATHEXT`: si una instalación de npm dejó un script de Unix sin extensión junto al lanzador `.cmd`, se elige el lanzador, que es el único que Windows sabe ejecutar.
- **Linux**: está implementado, pero **todavía no hay un recorrido real comprobado**. Usa la primera terminal conocida que encuentre, entre `x-terminal-emulator`, `gnome-terminal`, `konsole` y `xfce4-terminal`. Si no hay ninguna, avisa y no lanza nada.
- **macOS**: todavía **no** se afirma. El proyecto no dispone de un recorrido probado ahí, y prefiere no prometerlo.

### 🔒 El mapa queda bloqueado mientras el agente trabaja
Desde que confirmas el envío, el mapa no se puede editar: si tú y el agente cambiáis el mismo archivo a la vez, uno de los dos pierde su trabajo. Puedes verlo, recorrerlo, seleccionar nodos, buscar y exportar, y la barra inferior avisa de que un agente está trabajando. El bloqueo se levanta cuando el agente avisa de que ha terminado: lo marca en su última entrega (`work_finished: true`) y MMCelt carga ese mapa final. Si el agente falla o no avisa, el botón **Recuperar el control** de la barra quita el bloqueo. Si el envío falla antes de lanzar al agente, el bloqueo se quita solo.

### 🔄 Recarga automática en bucle cerrado
Cuando un agente actualiza el mapa en disco con `mmcelt_sync_ai_progress`, MMCelt detecta el cambio al momento y el lienzo se recarga solo, con los nodos, estados y prioridades nuevos.

### 🛡️ Protección de cambios locales sin guardar
Si estás editando el mapa y tienes cambios pendientes, la recarga automática se frena para no pisar tu trabajo. Vuelve a operar en cuanto guardas con `Ctrl + S`.

### 🛑 Cómo detener la vigilancia y la consola
La vigilancia termina de forma limpia al usar `Detener seguimiento` en el menú de inteligencia artificial, al crear un mapa nuevo con `Ctrl + N`, al abrir otro archivo desde el menú `Archivo` o al cerrar la aplicación.

La consola del agente es un **proceso aparte**: se cierra en su propia ventana. Detener la vigilancia no cierra la consola, y cerrar la consola no borra el expediente.
