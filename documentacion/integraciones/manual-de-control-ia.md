# 📖 Manual de Supervisión y Alineación Bidireccional con IA: Claude, ChatGPT y Gemini

> **MMCelt: Tu cabina de control visual y plano arquitectónico para proyectos desarrollados con Inteligencia Artificial.**

---

## 1. La Filosofía: Bidireccionalidad Real entre Humano e Inteligencia Artificial

Cuando trabajas en proyectos complejos con modelos de Inteligencia Artificial (arquitectura de software, investigación profunda, diseño de producto o estrategia de negocio), el mayor reto no es escribir texto o código, sino **mantener la alineación conceptual y el control absoluto del proyecto**.

**MMCelt está diseñado para funcionar en ambas direcciones de forma totalmente fluida:**

```
                  ┌────────────────────────────────────────────────────────┐
                  │ 👤 EL USUARIO (ARQUITECTO / SUPERVISOR)               │
                  │ - Diseña la estructura, módulos y rutas de archivo     │
                  │ - O bien supervisa el trabajo que la IA ha generado    │
                  └─────────────────────────┬──────────────────────────────┘
                                            │
        FLUJO 1: "Tú eres el Arquitecto"    │   FLUJO 2: "Tú eres el Supervisor"
        Diseñas el plano en MMCelt y        │   La IA genera el mapa, tú lo auditas
        la IA lo programa paso a paso       │   y corriges desvíos de rumbo
                                            │
                                            ▼
                  ┌────────────────────────────────────────────────────────┐
                  │ 🤖 LA INTELIGENCIA ARTIFICIAL (CONSTRUCTOR / SOCIO)   │
                  │ (Claude Code, Cursor, Codex, Gemini, Antigravity)      │
                  │ - Lee tu mapa como especificación maestra              │
                  │ - Sincroniza su avance y acata correcciones en vivo    │
                  └────────────────────────────────────────────────────────┘
```

---

### Flujo 1: Tú eres el Arquitecto (Diseñas en MMCelt ➔ La IA Programa)
En este modo, tú tienes la idea clara y quieres que la IA programe exactamente lo que tienes en mente, sin que tome decisiones por su cuenta ni invente nombres de archivos extraños:

1. **Abres MMCelt:** Creas tu mapa mental usando atajos rápidos (<kbd>Tab</kbd> para ramas hijas, <kbd>Enter</kbd> para ramas hermanas) o cargas una plantilla de arquitectura (*Clean Architecture*, *Fullstack*).
2. **Defines los módulos y sus archivos:** Asignas a cada nodo su ruta de archivo de código (ej. `src/auth/jwt.rs`, `migrations/001_init.sql`).
3. **Escribes las especificaciones técnicas:** En las notas de cada nodo defines qué librerías usar, algoritmos o restricciones.
4. **Fijas la visión del proyecto:** En el panel
   `🧭 Visión del Creador y Metas del Proyecto (Para la IA)` redactas el propósito global del
   software.
5. **Se lo entregas a la IA (Claude Code, ChatGPT, Gemini o Antigravity):** La IA recibe el archivo `.md` o `.mmcelt` como un **plano de construcción de obra civil**, garantizando que el código se escribe en orden, respetando tus dependencias y sin alucinaciones arquitectónicas.

---

### Flujo 2: Tú eres el Supervisor (La IA Plantea ➔ Tú Auditas y Corriges)
En este modo, le pides a la IA que investigue o plantee la solución técnica a un problema grande:

1. **La IA genera el mapa mental:** La IA analiza tu proyecto y genera el mapa mental visual (vía MCP o código JSON).
2. **Abres el mapa en MMCelt:** Tienes en segundos una radiografía panorámica de todo lo que la IA entiende, cómo divide el trabajo y qué dudas técnicas tiene.
3. **Detectas errores o caminos que no te gustan:** Si la IA eligió una base de datos incorrecta o un enfoque que no deseas, marcas ese nodo como **`⛔ Descartado`** o **`⚠️ Requiere Corrección`** y escribes tu directiva en el campo de corrección.
4. **Eliges `🛑 Enviar Correcciones y Directivas a la IA...`:** La IA recibe un informe estricto de control humano que le prohíbe continuar por las vías descartadas y le obliga a reajustar el código según tus órdenes exactas.

---

## 2. Interfaz Estándar y Sistema Pedagógico de Ayuda para Usuarios Noveles

Para garantizar que cualquier persona (sea principiante o desarrollador experimentado) domine la herramienta desde el primer minuto, MMCelt cuenta con dos elementos clave:

### A. Barra de Menús de Escritorio Estándar
Se han sustituido los botones dispersos por una estructura limpia y jerárquica:
- **`📁 Archivo`:** Crear nuevo mapa, plantillas arquitectónicas (*Clean Arch*, *Fullstack*), escáner automático de carpetas de código, abrir, guardar (`Ctrl+S`) y exportar a Markdown para IA (`Ctrl+E`).
- **`✏️ Edición`:** Añadir hijo (<kbd>Tab</kbd>), añadir hermano (<kbd>Enter</kbd>), eliminar (<kbd>Supr</kbd>), editar (<kbd>Espacio</kbd>/<kbd>F2</kbd>), conexiones cruzadas y ficha de visión del proyecto.
- **`🤖 Inteligencia Artificial`:** Importar mapas desde chats de IA, copiar el prompt maestro, panel de directivas de corrección humana, previsualizar `.md` y exportar.
- **`🎨 Ver y Diseño`:** Modos de auto-layout (Árbol balanceado, Radial, Libre), centrado de cámara (<kbd>Ctrl+F</kbd>) y temas visuales (Oscuro, Claro, Alto Contraste).
- **`❓ Ayuda`:** Activación de galletas guiadas, guías temáticas directas, atajos de teclado y panel de ayuda exhaustiva.

### B. Sistema de "Galletas" Guiadas y Ventana Lateral de Ayuda Detallada (`+info`)
- **🍪 Las Galletas Contextuales:** Son píldoras informativas discretas que aparecen automáticamente arriba del lienzo y en el inspector lateral, adaptándose a lo que estás haciendo o al nodo que tengas seleccionado.
- **El enlace interactivo `+info`:** Cada galleta incluye un botón `+info`. Al pulsarlo, **no te invade con ventanas emergentes molestas**, sino que **despliega un panel lateral a la derecha con toda la información exhaustiva posible**:
  - ¿Qué hace exactamente esa opción?
  - Qué opciones tienes para configurarla.
  - Por dónde continuar y qué pasos se recomiendan.
  - Ejemplos prácticos de integración con IAs y atajos de teclado relacionados.

---

## 🟣 3. Guía Paso a Paso para CLAUDE (Anthropic)

En el ecosistema de Anthropic puedes usar Claude Desktop o Claude Code. Ambos admiten MCP, pero
la aplicación gráfica la abres tú y la consola puede abrirla MMCelt.

### Modalidad A: Claude Desktop, con apertura manual y devolución MCP
En este modo no tienes que copiar y pegar el mapa. Tú abres la aplicación y la conversación;
Claude lee y devuelve los mapas mediante MCP.

#### Paso 1: Conectar MMCelt con Claude (se hace una sola vez)

El servidor MCP va dentro del propio ejecutable de MMCelt, así que no hay nada que
instalar aparte: ni Python, ni Node, ni ningún paquete.

**La vía corta.** Abre MMCelt y entra en el menú
**`🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...`**. La aplicación busca
los agentes que tengas instalados, te los muestra en una lista y los conecta al pulsar
**`🔌 Conectar`**. Hace una copia de seguridad del archivo de configuración antes de
tocarlo. Después, reinicia Claude Desktop.

**A mano, si prefieres ver lo que se escribe.** Abre
`%APPDATA%\Claude\claude_desktop_config.json` —o entra en Claude Desktop, en
**Settings → Developer → Edit Config**— y añade:

```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/ruta/hasta/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": {
        "MMCELT_WORKSPACE": "C:/ruta/hasta/tu/carpeta/de/mapas"
      }
    }
  }
}
```

Sustituye las dos rutas por las tuyas y escríbelas con barras normales (`/`), que es lo
que espera el formato JSON. `MMCELT_WORKSPACE` marca la carpeta dentro de la cual el
agente puede leer y escribir: fuera de ella no tiene acceso a nada.

Guarda y reinicia Claude Desktop. Sabrás que ha funcionado porque aparece un icono de
martillo 🔨 abajo a la derecha, con las herramientas de MMCelt dentro.

#### Paso 2: El flujo de trabajo real
1. **Pídele a Claude que inicie el proyecto:**
   > *"Claude, vamos a diseñar una aplicación de análisis financiero en Rust. Analiza los requisitos que te he dado y usa la herramienta `mmcelt_create_mindmap` para guardar el mapa mental en `finanzas.mmcelt` dentro de mi espacio de trabajo, con los pilares principales y dudas a resolver."*
2. **Supervisa en MMCelt:**
   * Abre la app MMCelt y pulsa **`📂 Abrir Mapa (.mmcelt / .json)...`** para cargar `finanzas.mmcelt`.
   * Verás todas las ramas en tu lienzo 2D infinito.
3. **Si necesitas corregir a Claude:**
   * Haz clic en el nodo que quieras cambiar (ej. *"Usar MongoDB"*).
   * Cambia su estado a **`⛔ Descartado`** o **`⚠️ Requiere Corrección`**.
   * En el campo **"🛑 Corrección / Instrucción Exigida a la IA"**, escribe: *"Descartamos bases de datos NoSQL. Usaremos PostgreSQL porque necesitamos transacciones ACID estrictas"*.
   * Guarda el archivo (<kbd>Ctrl</kbd>+<kbd>S</kbd>).
4. **Devuélvele el control a Claude:**
   * En tu conversación con Claude, dile simplemente:
     > *"Claude, he revisado el mapa mental en MMCelt y he dejado correcciones. Usa `mmcelt_get_human_feedback` y ajusta el plan de arquitectura."*
   * Claude ejecutará la herramienta, leerá tus notas y te responderá confirmando que ha descartado MongoDB y reorientado el desarrollo hacia PostgreSQL.

---

### Modalidad B: Uso en la Web (Claude.ai)
Si utilizas la versión web de Claude:
1. En MMCelt, pulsa el botón **`📋 Copiar Prompt Maestro para IA...`** y cópialo.
2. Pégalo en tu chat de Claude.ai junto con la descripción de tu proyecto.
3. Claude te responderá con un bloque de código JSON.
4. En MMCelt, abre **`📥 Importar desde IA (ChatGPT, Claude, Gemini)...`**, pega la respuesta de Claude y pulsa **`✨ Sustituir el mapa por el de la IA`**.

---

## 🟢 4. Guía Paso a Paso para CHATGPT (OpenAI)

En un chat web de ChatGPT puedes trabajar por copia manual. Para un recorrido local con MCP utiliza
Codex CLI o la aplicación Codex, según se explica en
[`chatgpt.md`](chatgpt.md). La CLI la abre MMCelt; la aplicación y su tarea las abres tú.

### Modalidad A: Crear tu propio "Custom GPT: Copiloto MMCelt" (Recomendada)
Si tienes ChatGPT Plus, Team o Enterprise, puedes crear un asistente permanente que siempre hable el idioma de MMCelt:

1. Entra en ChatGPT y haz clic en **Explore GPTs** > **+ Create**.
2. Ve a la pestaña **Configure**:
   * **Name:** `MMCelt - Arquitecto de Mapas Mentales`
   * **Description:** `Asistente para estructuración visual y supervisión de proyectos con MMCelt.`
   * **Instructions:** Abre el archivo [`skills/mmcelt-mindmap/SKILL.md`](../../skills/mmcelt-mindmap/SKILL.md) de tu proyecto, copia todo su texto y pégalo en este cuadro de instrucciones.
3. Guarda el GPT.

A partir de ahora, cada vez que abras ese GPT y le hables de una idea, te generará automáticamente los bloques listos para importar a MMCelt.

---

### Modalidad B: En cualquier Chat normal de ChatGPT
1. **Pedirle a ChatGPT que estructure tu idea:**
   * En MMCelt pulsa **`📋 Copiar Prompt Maestro para IA...`** y copia el texto maestro.
   * En ChatGPT escribe:
     > *"Actúa según estas directrices de MMCelt: [pega el prompt maestro]. Aquí tienes mi proyecto: [describe tu idea o pega tu código]."*
   * ChatGPT generará el bloque estructurado.
   * En MMCelt elige **`📥 Importar desde IA (ChatGPT, Claude, Gemini)...`**, pega la respuesta y tendrás tu mapa montado.

2. **Corregir a ChatGPT cuando se equivoque:**
   * Si tras ver el mapa mental en MMCelt descubres que ChatGPT ha omitido un módulo o ha elegido una tecnología errónea:
   * Modifica los nodos en MMCelt, añade tus notas de corrección o elimina lo que no sirva.
   * Abre `🤖 Inteligencia Artificial` y elige **`🛑 Enviar Correcciones y Directivas a la IA...`**.
   * Pulsa **`📋 Copiar Directivas de Corrección`**.
   * Pégalo en ChatGPT. El mensaje empezará con una cabecera estricta:
     > *"🛑 INSTRUCCIÓN DE CORRECCIÓN Y REAJUSTE DE PROYECTO PARA LA IA: He revisado el mapa mental en MMCelt y exijo los siguientes cambios obligatorios..."*
   * ChatGPT acatará las correcciones, desechará los caminos vetados y continuará el desarrollo exactamente por donde tú le has marcado.

---

## 🔵 5. Guía Paso a Paso para GEMINI (Google)

En el ecosistema de Google puedes usar el chat web, Gemini CLI o Antigravity. El chat web usa copia
manual, MMCelt puede abrir Gemini CLI y Antigravity se abre manualmente después de preparar MCP.

### Modalidad A: Crear un "Gem" Personalizado (Gemini Advanced)
1. En Gemini, entra en el gestor de **Gems** y pulsa **New Gem**.
2. **Nombre:** `MMCelt Strategy Architect`.
3. **Instrucciones:** Copia y pega el contenido de [`skills/mmcelt-mindmap/SKILL.md`](../../skills/mmcelt-mindmap/SKILL.md).
4. Pulsa **Save**. Ahora tu Gem responderá siempre en formato MMCelt.

---

### Modalidad B: Análisis de Proyectos Masivos mediante el `.md` Exportado
1. Diseña en MMCelt la estructura inicial de tu idea, escribe tu visión en el panel
   **`🧭 Visión del Creador y Metas del Proyecto (Para la IA)`** y pulsa
   **`🤖 Exportar Markdown para IA (.md)`** (o <kbd>Ctrl</kbd>+<kbd>E</kbd>).
2. Se generará un archivo como `MiProyecto_AI.md`.
3. Abre Gemini (o Google AI Studio) y **arrastra el archivo `.md` directamente al chat** junto con tus archivos de código o documentación PDF.
4. **Instrucción para Gemini:**
   > *"Gemini, he adjuntado el mapa mental de mi proyecto. Lee con máxima atención la Sección 1 (Mi visión como creador) y la Sección 5 (Dudas y puntos de decisión pendientes). Elabora una propuesta técnica detallada que resuelva todas las dudas abiertas y devuélveme el mapa mental actualizado en formato MMCelt para que pueda supervisarlo en mi aplicación."*
5. Copias la respuesta de Gemini, la pegas en MMCelt con **`📥 Importar desde IA (ChatGPT, Claude, Gemini)...`** y tendrás en tu pantalla la evolución del proyecto generada por Gemini.

---

---

## 6. Integración con Entornos de Desarrollo Autónomos y Agénticos

Cuando pasas del chat conversacional a herramientas donde la IA **escribe código, refactoriza repositorios y ejecuta terminales de forma autónoma**, MMCelt se vuelve aún más crucial como tu **salvaguarda y mapa de ruta visual**.

---

### 🟣 A. CLAUDE CODE (CLI de Anthropic para Terminal)

**Claude Code** es el agente de línea de comandos de Anthropic que navega por tu código, crea ramas y ejecuta tests de forma autónoma en la terminal.

#### 1. Vincular el Servidor MCP de MMCelt a Claude Code

Vale la misma vía corta que con Claude Desktop: el menú
**`🤖 Inteligencia Artificial` → `🔌 Conectar MMCelt con mis IAs...`** detecta también
Claude Code y lo configura.

Si lo prefieres desde la terminal, se registra con una sola orden:

```bash
claude mcp add mmcelt -s user -e MMCELT_WORKSPACE="C:/ruta/hasta/tu/carpeta/de/mapas" -- "C:/ruta/hasta/mmcelt.exe" --mcp-server
```

Lo que se registra es el ejecutable de MMCelt, no un script suelto: por eso quien reciba
el programa no necesita instalar nada más para conectarlo con su agente.

#### 2. Flujo de Trabajo en Terminal
1. **Antes de empezar a programar:**
   Inicia Claude Code y dale la siguiente instrucción:
   ```bash
   claude "Analiza el repositorio actual. Antes de tocar ningún archivo de código, genera un mapa mental con mmcelt_create_mindmap en ./arquitectura.mmcelt dividiendo las fases de refactorización y los puntos críticos a comprobar."
   ```
2. **Supervisión en tiempo real:**
   Abre MMCelt en paralelo. Cargas `arquitectura.mmcelt` y verás el plan completo de Claude Code.
3. **Pausar o Corregir a Claude Code:**
   Si ves que Claude Code planea modificar un módulo sensible que no debe tocar:
   - Marcas ese nodo como `⛔ Descartado` o `⚠️ Requiere Corrección` en MMCelt y guardas (<kbd>Ctrl</kbd>+<kbd>S</kbd>).
   - En la terminal de Claude Code escribes:
     ```text
     Consulta el mapa mental con mmcelt_get_human_feedback antes de continuar y acata las correcciones.
     ```
   - Claude Code leerá tus directivas, cancelará la modificación de ese módulo y redirigirá su plan de código.

---

### 🟢 B. OPENAI CODEX / CANVAS / CURSOR (Ecosistema OpenAI)

Para entornos orientados a desarrollo con modelos de OpenAI (GPT-4o, o1, Canvas y extensiones de código como Cursor o VS Code):

#### 1. El Mapa Mental como "Plan de Arquitectura Vivo"
1. **Inicialización:**
   Al comenzar una sesión de programación en Canvas o Cursor con ChatGPT, añade al inicio del contexto:
   > *"Actúa como Senior Tech Lead. Mantendremos la arquitectura y decisiones del proyecto sincronizadas con la estructura de MMCelt. Lee el archivo `proyecto_AI.md` que adjunto como especificación maestra de requisitos y restricciones."*
2. **Generación de Diffs y Verificación:**
   Cuando le pidas a la IA una refactorización profunda, añade:
   > *"Genera el código necesario y, al terminar, entrégame el bloque JSON de actualización para MMCelt reflejando los nuevos módulos creados, los tests implementados y los posibles bloqueos técnicos detectados."*
3. **Supervisión Continua:**
   Pegas el bloque devuelto en **`📥 Importar desde IA (ChatGPT, Claude, Gemini)...`** en MMCelt. Puedes ver al instante si el código generado cubre todos los requisitos o si ha dejado cabos sueltos.

---

### 🔵 C. GOOGLE ANTIGRAVITY (Plataforma Agéntica para Gemini)

**Google Antigravity** es el entorno de desarrollo agéntico avanzado impulsado por **Gemini**, diseñado para planificar y ejecutar proyectos de software de gran escala con subagentes y herramientas integradas.

#### 1. Instalación del Skill de MMCelt en Antigravity
El archivo [`skills/mmcelt-mindmap/SKILL.md`](../../skills/mmcelt-mindmap/SKILL.md) está diseñado con el formato estándar de **Agent Skills** compatible con Antigravity.

Puedes indicarle a Antigravity:
```text
Carga el skill ubicado en ./skills/mmcelt-mindmap/SKILL.md y úsalo para documentar la arquitectura visual del proyecto.
```

#### 2. El Ciclo de Planificación y Control en Antigravity
1. **Plan de Implementación Visual:**
   Cuando Antigravity entra en *Planning Mode* para crear el `implementation_plan.md`, pídele que genere simultáneamente el mapa visual:
   > *"Antigravity, junto con el implementation_plan.md, genera el archivo `plan_arquitectura.mmcelt` con la descomposición visual de tareas y dependencias."*
2. **Supervisión en Vivo mientras Antigravity ejecuta:**
   Mientras Antigravity crea archivos, ejecuta subagentes o corre comandos en segundo plano, tú mantienes abierto MMCelt en tu pantalla para monitorizar la visión global.
3. **Inyección de Feedback de Corrección:**
   Si al revisar el mapa en MMCelt detectas que un subagente va a tomar una decisión no deseada, eliges **`🛑 Enviar Correcciones y Directivas a la IA...`** y pegas el texto en el prompt de Antigravity:
   > *"He modificado el mapa mental en MMCelt. Acata las siguientes directivas de corrección obligatorias: [pegar directivas]. Reajusta el implementation_plan.md antes de continuar."*
   Antigravity reevaluará su plan de ejecución, actualizará el walkthrough y procederá con las nuevas restricciones.

---

## 7. Tabla Resumen de Acciones y Atajos

| Objetivo que deseas conseguir | Acción en MMCelt | Acción en la IA / CLI / IDE |
| :--- | :--- | :--- |
| **Que la IA te dé un mapa de lo que está haciendo** | Pulsa `📋 Copiar Prompt Maestro para IA...` o usa MCP | La IA genera el JSON estructurado o ejecuta `mmcelt_create_mindmap` |
| **Ver el mapa de la IA en tu pantalla** | Elige `📥 Importar desde IA (ChatGPT, Claude, Gemini)...` y pega (o abre el `.mmcelt`) | — |
| **Explicarle a la IA qué pretendes con el proyecto** | Edita el panel `🧭 Visión del Creador y Metas del Proyecto (Para la IA)` | La IA lo lee en la sección 1 del `.md` |
| **Corregir a la IA si tomó un mal camino** | Marca `⛔ Descartado` o `⚠️ Requiere Corrección` y elige `🛑 Enviar Correcciones y Directivas a la IA...` | Pega el texto en la IA o llama a `mmcelt_get_human_feedback` |
| **Darle a la IA todo tu proyecto masticado** | Pulsa `🤖 Exportar Markdown para IA (.md)` (<kbd>Ctrl</kbd>+<kbd>E</kbd>) | Sube el archivo `.md` a Claude, ChatGPT, Gemini o Claude Code |
| **Mandarlo y ver el mapa cambiar solo** | `🤖 Inteligencia Artificial → 📤 Enviar a...` | Pega el prompt, que ya está en el portapapeles, y deja trabajar al agente |
| **Cortar a un modelo que se ha desbocado** | `🤖 Inteligencia Artificial → ⏹ Detener seguimiento` | — |
| **Navegar a máxima velocidad** | <kbd>Tab</kbd> (Hijo), <kbd>Enter</kbd> (Hermano), <kbd>Supr</kbd> (Borrar), <kbd>Espacio</kbd> (Editar) | — |

---

## 8. El bucle cerrado: mandar y ver volver (desde la 0.9.0)

Los flujos web de arriba tienen un paso manual en cada sentido: llevarle el documento al modelo y
traer su respuesta. **Con «📤 Enviar a...» y un cliente MCP, la devolución puede automatizarse; la
apertura depende del cliente.**

`🤖 Inteligencia Artificial → 📤 Enviar a...` y eliges el agente. Antes de confirmar puedes leer y
modificar las reglas comunes y el encargo. MMCelt guarda el mapa, exporta el documento, conserva
el prompt exacto en `.mmcelt/sesiones` y **empieza a vigilar el archivo**. Si el destino tiene una
CLI compatible, también abre una consola nueva en la carpeta del proyecto.

### La familia del modelo no es la aplicación que estás usando

La consola y la aplicación gráfica son clientes diferentes. Pueden utilizar modelos de la misma
familia, pero cada una mantiene su propia conversación y puede cargar herramientas o permisos
distintos. La conexión MCP permite que el cliente lea y devuelva el mapa; no permite que MMCelt
escriba dentro de una conversación gráfica ya abierta.

| Familia | Cliente de consola | Cliente gráfico | Procedimiento de apertura |
|---|---|---|---|
| **Claude** | Claude Code | Claude Desktop | `📤 Enviar a...` abre Claude Code. Claude Desktop lo abres o reinicias tú y adjuntas o pegas el expediente. |
| **OpenAI** | Codex CLI | Aplicación Codex | `📤 Enviar a...` abre Codex CLI. En la aplicación Codex abres una tarea local y le indicas el expediente. |
| **Gemini** | Gemini CLI | Antigravity | `📤 Enviar a...` abre Gemini CLI. Para Antigravity, MMCelt prepara la sesión y tú abres el proyecto y continúas allí. |
| **Web** | — | ChatGPT, Claude.ai o Gemini en el navegador | Abres el chat y adjuntas el `_AI.md`; un chat web ordinario no lee el MCP local. |

Después de registrar o actualizar MCP, cierra por completo el cliente afectado y vuelve a abrirlo.
Los tres clientes con apertura automática son **Claude Code, Codex CLI y Gemini CLI**. En los
clientes gráficos, `Preparar para MCP` significa exactamente eso: el mapa, el expediente y la
vigilancia están listos, pero MMCelt no ha abierto ni controlado la otra aplicación.

La vuelta puede ser automática en cualquier cliente local que haya cargado el servidor MCP y use
`mmcelt_sync_ai_progress`. En cambio, un chat web corriente necesita el recorrido manual de
exportación e importación.

### Tu trabajo no se pierde

Si estabas editando el mapa cuando llega el cambio del modelo, **MMCelt no toca lo tuyo**: te
avisa y decides. Y antes de cualquier recarga deja una copia en el archivo de recuperación.

Un consejo que ahorra tiempo: **pídele al modelo que te avise cuando acabe** de escribir sus
cambios. Te lo dirá en su propio terminal y revisas en MMCelt cuando quieras.

---

*Manual para MMCelt v0.11.3 — Aplicación portable en Rust para la supervisión y el control humano de inteligencias artificiales.*

