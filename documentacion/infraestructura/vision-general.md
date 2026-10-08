# Visión general de la infraestructura

Qué componentes existen, dónde se ejecuta cada uno y cómo se comunican.

---

## 1. Mapa de componentes

```
┌─────────────────────────── EQUIPO DEL USUARIO ────────────────────────────┐
│                                                                           │
│   ┌──────────────────────┐         ┌──────────────────────────────┐       │
│   │  MMCelt (mmcelt.exe) │         │  mmcelt.exe --mcp-server     │       │
│   │  Aplicación nativa   │         │  El mismo ejecutable, como    │      │
│   │  Rust + egui         │         │  servidor MCP (stdio)         │      │
│   └──────────┬───────────┘         └───────────────┬──────────────┘       │
│              │                                     │                      │
│              │        ┌────────────────────┐       │                      │
│              └───────►│  Archivos .mmcelt  │◄──────┘                      │
│                       │  (disco local)     │                              │
│                       └────────────────────┘                              │
│                                                                           │
│   ┌──────────────────────────────────────────────────────────────────┐    │
│   │  Agente de IA: Claude Code/Codex/Gemini CLI o cliente MCP        │    │
│   └──────────────────────────────────────────────────────────────────┘    │
└───────────────────────────────────────────────────────────────────────────┘
                                     ╎
                      ╎ (opcional, fuera de MMCelt)
                                     ╎
┌────────────────────────────────────▼──────────────────────────────────────┐
│  Plataformas de IA en la nube: ChatGPT, Claude, Gemini                    │
│  Se usan copiando y pegando en el navegador. MMCelt no habla con ellas.   │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Qué se ejecuta dónde

| Componente | Dónde | Cómo arranca | Persistencia |
|---|---|---|---|
| Aplicación de escritorio | Local, proceso nativo | El usuario, con doble clic o `cargo run` | Archivos `.mmcelt` |
| Servidor MCP | Local, proceso hijo | Lo lanza el agente de IA por stdio | Los mismos archivos |
| Consola oficial del agente | Local, proceso separado | Confirmación expresa de «Enviar a…» | Su propia sesión autenticada |
| Cliente solo MCP | Local | Independiente de MMCelt | Lee el expediente preparado desde su interfaz |
| Modelos de IA | Nube | Fuera del alcance del proyecto | — |

**No hay nada en la nube que pertenezca a MMCelt.** Ni servidor, ni base de datos, ni
cuenta de usuario, ni telemetría. Lo único externo que consulta es la página pública de
versiones de GitHub, al arrancar y si el usuario no lo ha desactivado.

---

## 3. Comunicación entre componentes

### Aplicación ↔ Servidor MCP

El canal entre ambos son los archivos `.mmcelt` en el disco: el servidor escribe y la
aplicación lee.

Con la función **«Enviar a…»** y el módulo `vigilante.rs` (basado en `notify` 8.0), la aplicación
vigila el archivo abierto: en cuanto el agente de IA escribe una actualización en disco vía
`mmcelt_sync_ai_progress`, la aplicación detecta el cambio y recarga el lienzo al instante. Si
el usuario tiene cambios locales sin guardar en la ventana, la recarga se detiene para proteger
su trabajo y le muestra un aviso.

### Agente de IA ↔ Servidor MCP

JSON-RPC 2.0 sobre entrada y salida estándar, según el Model Context Protocol. El agente
lanza el servidor como proceso hijo y le habla por las tuberías. No hay puertos de red ni
sockets.

### Usuario ↔ Plataformas de IA

MMCelt puede abrir las CLI oficiales `claude`, `codex` y `gemini` con una referencia al expediente
local; no proporciona credenciales ni usa API. Para Antigravity y otros clientes solo MCP prepara
los mismos archivos sin abrir consola. El retorno estructurado ocurre por el servidor MCP local.

---

## 4. Superficie de red

**Una petición saliente, desactivable, y nada más.**

| Comprobación | Resultado |
|---|---|
| Dependencias HTTP en `Cargo.toml` | `ureq` (solo `rustls`), para la comprobación de versión nueva |
| Peticiones en el código | Una: `GET` por HTTPS a `releases/latest` de GitHub al arrancar, si la opción está activa ([`servicios.md`](servicios.md#comprobación-de-versión-nueva)) |
| Puertos abiertos | Ninguno |
| Telemetría | Ninguna: la petición no lleva datos del usuario ni del mapa |
| Ejecución de procesos externos | Solo la CLI oficial elegida tras confirmación; argumentos separados y sin el prompt completo |

Es una decisión de diseño con consecuencias directas: no hay credenciales que filtrar ni datos
que exfiltrar. La única petición es anónima, solo lee un dato público, acepta solo HTTPS con
certificado verificado y trata la respuesta como no fiable.

La CLI puede conectarse por su cuenta al proveedor usando la sesión del usuario, pero esa red no
la abre ni la controla MMCelt. **Si algún día se añade una integración API directa**, este análisis
deja de ser válido y hay que rehacerlo por completo.

---

## 5. Variables de entorno

Solo hay una, y afecta únicamente al servidor MCP:

| Variable | Componente | Valor por defecto | Propósito |
|---|---|---|---|
| `MMCELT_WORKSPACE` | Servidor MCP | Obligatoria (sin valor por omisión) | Delimita la carpeta **dentro de la cual** el servidor puede leer y escribir. Si no está fijada, el servidor rechaza toda operación por seguridad |
| `MMCELT_DATOS` | Aplicación | El directorio de datos del sistema | Reubica la carpeta donde se guardan las copias de recuperación del autoguardado |

Toda ruta que envíe el agente se resuelve (neutralizando `..` y los enlaces simbólicos) y
se comprueba que quede dentro de esa raíz. Cualquier intento de salir se rechaza.

**Es obligatorio fijarla de forma explícita** (o configurarla automáticamente desde la ventana
de conexión de la aplicación).

```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/ruta/a/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": { "MMCELT_WORKSPACE": "C:/Users/tu-usuario/Documentos/MapasMentales" }
    }
  }
}
```

### `MMCELT_DATOS`: cuándo usarla

Por defecto no hace falta tocarla: la aplicación usa el directorio de datos que
corresponde a cada sistema (`%APPDATA%\MMCelt` en Windows). Tiene dos usos concretos:

- **Instalación portable**: llevar MMCelt en una memoria USB con sus datos al lado, en
  lugar de dejarlos en el equipo donde se ejecute.
- **Pruebas**: aislar el archivo de recuperación para que no interfiera con el del
  usuario.

Si la carpeta no se puede determinar ni crear, el autoguardado queda desactivado y la
aplicación sigue funcionando con normalidad: simplemente, sin red de seguridad.

---

## 6. Archivos que genera el sistema

| Archivo | Dónde | Quién lo crea | Se puede borrar |
|---|---|---|---|
| `*.mmcelt` | Donde elija el usuario | La aplicación o el servidor MCP | Es el trabajo del usuario |
| `*_AI.md` | Junto al `.mmcelt` | El servidor MCP o la exportación | Sí, se regenera |
| `*.bak` | Junto al original | El servidor MCP, antes de sobrescribir | Sí, es una copia de respaldo |
| `mmcelt-errores.log` | Junto al ejecutable | La aplicación, solo si hay errores | Sí |
| `sesion.autoguardado.json` | `%APPDATA%\MMCelt\recuperacion\` | La aplicación, cada 2 minutos si hay cambios (intervalo configurable) | Sí, pero se pierde el trabajo sin guardar de la última sesión |
| `preferencias.json` | `%APPDATA%\MMCelt\` | La aplicación, al cambiar la escala, el tema o el idioma | Sí, se vuelve a los valores por defecto |
| `target/` | Raíz del repositorio | `cargo` | Sí, se regenera al compilar |

---

## 7. Compilación y distribución
 
Se compila directamente con `cargo`. Actualmente **no existe una distribución oficial activa**:

```bash
cargo build --release      # binario en target/release/mmcelt.exe (8.678.400 bytes = 8,28 MiB en Windows)
```

El perfil de publicación aplica optimización máxima (`opt-level = 3`), LTO, una sola unidad de compilación,
eliminación de símbolos (`strip = true`) y `panic = "abort"`.

Los dos tamaños están medidos sobre los binarios de la publicación `v0.9.2`, no estimados:
**8,28 MiB (8.683.520 bytes) en Windows** y **13,23 MiB (13.877.536 bytes) en Linux**. La
diferencia entre ambos es habitual: el ejecutable de Linux conserva más metadatos de enlazado
aunque se le apliquen las mismas opciones.

Si alguien actualiza esta cifra, que la mire en los archivos publicados de la versión
correspondiente. La anterior —«unos 12 MB»— venía heredada de la `v0.8.1` y envejeció sin avisar
cuando la fuente CJK añadió 1,3 MB al binario.

### Publicación suspendida

El binario es totalmente autocontenido: incluye la tipografía CJK embebida y no requiere
instalador, ni tiempo de ejecución, ni bibliotecas adicionales en Windows.

---

## 8. Copias de seguridad

El proyecto no gestiona copias de seguridad remotas de los mapas del usuario, más allá de los
`.bak` automáticos con marca de tiempo que crea el servidor MCP antes de sobrescribir.

Al ser archivos JSON normales, la estrategia habitual funciona: guardarlos en una carpeta
sincronizada o en un repositorio git.
