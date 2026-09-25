# Guía de clonado: reproducir MMCelt desde cero

Pasos para dejar el proyecto funcionando en un equipo nuevo, partiendo de que no hay nada
instalado. No hace falta conocer el proyecto para seguirla.

**Tiempo estimado**: 15 minutos, de los cuales unos 5 son compilación.

> **Atajo**: si prefieres automatizarlo, ejecuta [`preparar-entorno.ps1`](preparar-entorno.ps1) en Windows o
> [`preparar-entorno.sh`](preparar-entorno.sh) en Linux y macOS. Hacen todo lo que describe esta guía. Aun así,
> conviene leer la sección 6 para saber qué comprobar al terminar.

---

## Paso 1 — Instalar Git

Comprueba si ya lo tienes:

```bash
git --version
```

Si no aparece una versión:

| Sistema | Instalación |
|---|---|
| Windows | Descargar de [git-scm.com](https://git-scm.com/download/win), o `winget install Git.Git` |
| macOS | `xcode-select --install`, o `brew install git` |
| Debian / Ubuntu | `sudo apt install git` |
| Fedora | `sudo dnf install git` |

---

## Paso 2 — Instalar Rust

```bash
rustc --version
```

Necesitas **1.85 o superior**. Si no lo tienes:

**Windows**: descarga e instala [rustup-init.exe](https://win.rustup.rs/). Acepta la
instalación por defecto (opción 1).

**Linux y macOS**:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Si ya tenías Rust pero con una versión antigua:

```bash
rustup update stable
```

### Windows: herramientas de compilación de C++

Rust necesita en Windows el enlazador de Microsoft. El instalador de `rustup` lo detecta
y ofrece instalarlo; **acepta**.

Si lo omitiste, la compilación fallará con un error sobre `link.exe`. Se resuelve
instalando las
[Build Tools de Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
con la carga de trabajo «Desarrollo para el escritorio con C++».

### Linux: bibliotecas del sistema

```bash
# Debian / Ubuntu
sudo apt install build-essential pkg-config libgtk-3-dev libxcb-render0-dev \
     libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev

# Fedora
sudo dnf install gcc-c++ pkgconf-pkg-config gtk3-devel libxkbcommon-devel
```

`libgtk-3-dev` hace falta para los diálogos nativos de archivo.

---

## Paso 3 — (nada más que instalar)

No hace falta nada más. El servidor MCP que conecta MMCelt con los agentes de IA va
**dentro del propio ejecutable**, así que no necesitas Python, Node ni ningún otro
tiempo de ejecución.

---

## Paso 4 — Obtener la copia local

Actualmente no existe un repositorio público desde el que clonar MMCelt. La fuente autorizada es
la copia local conservada por el responsable del proyecto. Copia esa carpeta completa —incluido
`.git` si necesitas conservar ramas e historial— al equipo nuevo y entra en ella:

```bash
cd MMCelt
```

Cuando se restaure un alojamiento remoto aprobado, este apartado volverá a incluir una orden de
clonado verificable. No uses ahora direcciones antiguas de GitHub: pueden no existir o pertenecer
a otro repositorio en el futuro.

---

## Paso 5 — Compilar

```bash
cargo build --release
```

La primera compilación descarga y construye las dependencias: tarda entre **1 y 4
minutos** según la máquina. Las siguientes son cuestión de segundos.

Resultado esperado:

```
    Finished `release` profile [optimized] target(s) in ...
```

El binario queda en `target/release/mmcelt.exe` (Windows) o `target/release/mmcelt`
(Linux y macOS), y ocupa unos 8,28 MiB (8,68 MB) en Windows (13,23 MiB en Linux, con la fuente CJK embebida).

> **Salvo que hayas movido la carpeta de compilación**, y en algunos casos hay que moverla.
> Si el proyecto vive dentro de una carpeta sincronizada —Google Drive, OneDrive, Dropbox—, el
> enlazador de Windows falla con `LNK1201` porque el sincronizador toca los archivos mientras
> se escriben, y la solución es fijar `target-dir` en `.cargo/config.toml` fuera de esa unidad.
> Entonces el binario **no está en `target/`**: está donde apunte `target-dir`.
>
> Para saber dónde está de verdad, sin adivinar:
>
> ```bash
> cargo metadata --format-version 1 --no-deps | grep -o '"target_directory":"[^"]*"'
> ```
>
> O más simple: abre MMCelt y mira **`❓ Ayuda → ℹ️ Acerca de MMCelt`**, que muestra la ruta
> del ejecutable que se está ejecutando.

### Si falla

| Error | Causa | Solución |
|---|---|---|
| `linker 'link.exe' not found` | Faltan las Build Tools de C++ | Ver el paso 2 |
| `failed to run custom build command for glutin` | Faltan bibliotecas del sistema (Linux) | Ver el paso 2 |
| `error: package requires rustc 1.85` | Rust desactualizado | `rustup update stable` |
| `LNK1104: no se puede abrir el archivo` | El ejecutable está en marcha, o bloqueado por el antivirus | Cierra la aplicación; en Windows, `Get-Process -Name mmcelt* \| Stop-Process -Force` |
| `LNK1201: error al escribir en el archivo de programa` | Estás compilando dentro de una carpeta sincronizada con la nube | Ver justo debajo |

### Si el proyecto está en Google Drive, OneDrive o Dropbox

Estos servicios copian los archivos mientras se escriben, y la compilación genera miles en
pocos segundos: el enlazador se encuentra con archivos bloqueados a medias. Falla unas veces
sí y otras no, lo que despista mucho.

El código fuente puede quedarse donde está; lo que hay que sacar de la carpeta sincronizada
es la **carpeta de compilación**:

```bash
export CARGO_TARGET_DIR="/una/ruta/fuera/de/la/nube/target-mmcelt"
cargo build --release
```

Para no tener que acordarte cada vez —y para que valga también al compilar desde el editor—,
crea `.cargo/config.toml` en la raíz del proyecto:

```toml
[build]
target-dir = "C:/ruta/fuera/de/la/nube/target-mmcelt"
```

Añádelo a `.gitignore`: la ruta solo existe en tu equipo.

---

## Paso 6 — Verificar

Ejecuta las comprobaciones de verificación mecánica. Todas deben pasar:

```bash
# 1. Compila sin ningún aviso
cargo build --all-targets

# 2. El linter no tiene nada que objetar
cargo clippy --all-targets --all-features -- -D warnings

# 3. Las pruebas automatizadas (201 pruebas)
cargo test --all-features

# 4. Formato de código impecable
cargo fmt --check

# 5. Seguridad y auditoría de dependencias
cargo audit
```

Resultado esperado: `cargo test` termina con `test result: ok` y 201 pruebas en verde.
La suite incluye las del servidor MCP integrado, el contraste de color y el arnés de interfaz.

> Si alguna prueba de `audit_checks` falla, su documentación indica qué defecto de la
> auditoría ha reaparecido. No lo ignores.

Y arranca la aplicación:

```bash
cargo run --release
```

Debe abrirse una ventana de 1280×800 con un mapa mental de ejemplo en el idioma seleccionado
por el sistema.

---

## Paso 7 — Conectar los agentes de IA (opcional)

Solo si vas a usar MMCelt con Claude Code, Antigravity, Codex, Cursor, Windsurf o Claude Desktop.

### Automático desde la aplicación (recomendado)

Abre la aplicación y ve al menú:
**`🤖 Inteligencia Artificial → 🔌 Conectar MMCelt con mis IAs`**

Detecta los agentes instalados, permite seleccionar la carpeta de trabajo (`MMCELT_WORKSPACE`)
y registra el servidor en cada uno en un solo clic. Hace copia de seguridad antes de tocar nada
y es idempotente.

**Después hay que reiniciar los agentes**: ninguno relee su configuración en caliente.

### Manual

1. Crea una carpeta para tus mapas, por ejemplo `Documentos/MapasMentales`.
2. Añade el servidor a la configuración de tu cliente MCP:

```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "RUTA_ABSOLUTA/MMCelt/target/release/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": { "MMCELT_WORKSPACE": "RUTA_ABSOLUTA/Documentos/MapasMentales" }
    }
  }
}
```

Sustituye las dos `RUTA_ABSOLUTA` por rutas reales.

> **La ruta del ejecutable no es siempre `target/release/`**, y copiar este ejemplo tal cual es
> la forma más fácil de que el cliente no arranque. Si has movido la carpeta de compilación
> —cosa obligada dentro de Google Drive u otra carpeta sincronizada—, el binario está en otro
> sitio y el cliente responde algo así:
>
> ```
> mmcelt: fork/exec .../MMCelt/target/release/mmcelt.exe:
> The system cannot find the path specified.
> ```
>
> **Lo más seguro es no escribir la ruta a mano**: usa
> **`🤖 Inteligencia Artificial → Conectar MMCelt con mis IAs`** dentro del programa. Detecta los
> agentes instalados, escribe la configuración por ti y pone **la ruta del ejecutable que está en
> marcha**, así que no puede equivocarse. Esta sección manual es para cuando esa ventana no cubra
> tu cliente.

3. Reinicia el cliente.
4. Comprueba que el agente ve las seis herramientas `mmcelt_*`.

### Probar «Enviar a…» sin confundir MCP y consola

- Para Claude Code, Codex CLI o Gemini CLI, comprueba que el ejecutable aparece en `PATH` y que la
  cuenta ya está autenticada en la propia herramienta.
- Para Antigravity u otro cliente solo MCP, el botón será **Preparar para MCP**: se crean el
  `_AI.md` y `.mmcelt/sesiones/.../inicio.md`, pero no se abre ninguna consola.
- MMCelt no necesita ni acepta claves API para este recorrido.
- La carpeta `.mmcelt/configuracion.json` puede no existir en proyectos antiguos. Abrir la vista
  previa no la crea; confirmar sí inicializa la identidad portable.

**Fija siempre `MMCELT_WORKSPACE` de forma explícita.** Delimita dónde puede escribir el
agente; sin ella el servidor falla cerrado y rechaza las operaciones con archivos. No usa en
silencio la carpeta desde la que arrancó.

Las rutas de la configuración van con barras normales (`/`) incluso en Windows.

---

## Paso 8 — Comprobar el conjunto

Una prueba de extremo a extremo, hablándole al servidor MCP igual que haría un agente:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/mmcelt --mcp-server
```

Debe responder con las seis herramientas `mmcelt_*`.

Y para comprobar qué compilación tienes delante:

```bash
./target/release/mmcelt --version
```

---

## Resumen

```bash
# Todo junto, suponiendo Git y Rust ya instalados
git clone <url> MMCelt && cd MMCelt
cargo build --release
cargo test
cargo run --release
```

---

## Desinstalar

No hay instalador ni registro que limpiar. Basta con:

1. Borrar la carpeta del repositorio.
2. Quitar la entrada `mmcelt` de la configuración de tu cliente MCP, si la añadiste.

Tus archivos `.mmcelt` son independientes y se conservan donde los hayas guardado.
