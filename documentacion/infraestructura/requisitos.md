# Requisitos del sistema

Versiones exactas necesarias para compilar y ejecutar MMCelt.

---

## 1. Resumen

| Componente | Versión mínima | Verificado con | Obligatorio |
|---|---|---|---|
| **Rust** (`rustc` y `cargo`) | 1.85 | 1.85+ | ✅ Sí |
| **Git** | 2.30 | — | Para clonar el repositorio |

La versión mínima de Rust viene impuesta por `eframe`/`egui` 0.36 y por el uso de
`let ... else`, estabilizado en 1.65. El proyecto usa la **edición 2021**.

> No hay archivo `rust-toolchain.toml`: se compila con la cadena estable que tengas
> instalada.

---

## 2. Sistema operativo

| Sistema | Estado |
|---|---|
| **Windows 10 / 11** | ✅ Plataforma principal, verificada en local y en CI |
| **Linux (X11 / Wayland)** | ✅ Verificado y probado automáticamente en CI (`ubuntu-latest`) |
| **macOS** | ⚠️ Arquitectura compatible; soporte previsto |

`egui`, `eframe`, `notify` y `rfd` son multiplataforma, y la CI del proyecto ejecuta en
cada empujón la batería completa de pruebas, `clippy`, `cargo fmt` y `cargo audit` en Windows
y en Linux.

En Linux hacen falta además las bibliotecas de desarrollo del sistema de ventanas:

```bash
# Debian / Ubuntu
sudo apt install build-essential pkg-config libssl-dev libgtk-3-dev libxcb-render0-dev \
     libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev

# Fedora
sudo dnf install gcc-c++ pkgconf-pkg-config gtk3-devel libxkbcommon-devel
```

`libgtk-3-dev` es necesaria para los diálogos nativos de archivo (`rfd`).

---

## 3. Hardware

| Recurso | Mínimo | Notas |
|---|---|---|
| Memoria | 4 GB | La compilación es lo que más consume; la aplicación en marcha, poco |
| Disco | ~2 GB | El binario de publicación ocupa **8,28 MiB (8,68 MB) en Windows** (**13,23 MiB en Linux**, con la fuente CJK embebida); el resto es la carpeta `target/` |
| Gráficos | OpenGL 3.3 | `eframe` renderiza por GPU mediante `glow` |

En una máquina virtual sin aceleración gráfica puede hacer falta forzar el renderizado
por software (`LIBGL_ALWAYS_SOFTWARE=1` en Linux).

---

## 4. Dependencias de Rust

Todas se descargan solas al compilar. No hay que instalar nada a mano.

| Dependencia | Versión | Para qué |
|---|---|---|
| `eframe` | 0.36.0 | Marco de aplicación de escritorio y ventana nativa (backend `glow`) |
| `egui` | 0.36.0 | Interfaz gráfica en modo inmediato |
| `serde` | 1.0 (con `derive`) | Serialización del modelo |
| `serde_json` | 1.0 | Formato JSON de los archivos `.mmcelt` |
| `rfd` | 0.15 | Diálogos nativos de archivo del sistema |
| `uuid` | 1.10 (con `v4`, `serde`) | Identificadores únicos de nodos |
| `chrono` | 0.4 (con `serde`) | Marcas de tiempo en UTC |
| `notify` | 8.0 | Observador reactivo del sistema de archivos para vigilancia en bucle cerrado |
| `winresource` | 0.1 | **Solo al compilar en Windows.** Incrusta el icono como recurso del ejecutable. No forma parte del programa |

**No hay ninguna dependencia de red.** Ni `reqwest`, ni `hyper`, ni equivalentes: la
aplicación no se conecta a internet.

---

## 5. Otras dependencias

**Ninguna obligatoria.** El servidor MCP está integrado en el propio ejecutable, así que el proyecto
no necesita Python, Node ni ningún otro tiempo de ejecución.

Es lo que permite pasarle la aplicación a otra persona y que le funcione: si puede
ejecutar el binario, tiene todo lo necesario, incluida la conexión con sus agentes de IA.

Para que «Enviar a…» abra una consola hace falta tener instalada y autenticada al menos una CLI
oficial: Claude Code (`claude`), Codex CLI (`codex`) o Gemini CLI (`gemini`). Son componentes
opcionales y externos. Antigravity, Claude Desktop, Cursor y Windsurf pueden usar el recorrido
solo MCP sin que MMCelt les abra una consola.

El adaptador de consola de la 0.10.0 está comprobado en Windows. El código para terminales Linux
está implementado, pero la aceptación mecánica y el recorrido real de esta versión siguen
pendientes; macOS no se anuncia para este flujo.

---

## 6. Herramientas de desarrollo

Opcionales, pero recomendadas si vas a modificar el código:

| Herramienta | Instalación | Para qué |
|---|---|---|
| `clippy` | `rustup component add clippy` | Linter. **La CI exige cero avisos** |
| `rustfmt` | `rustup component add rustfmt` | Formato del código |
| `cargo-audit` | `cargo install cargo-audit --locked` | Vulnerabilidades conocidas en dependencias |

---

## 7. Comprobar el entorno

```bash
rustc --version     # >= 1.85
cargo --version
git --version
```

Y para verificar que todo compila y pasa:

```bash
cargo build --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Resultado esperado: sin avisos y todas las pruebas correctas. La suite incluye las del
servidor MCP integrado, así que no hace falta ninguna herramienta externa.

---

## 8. Dónde se puede compilar

Casi en cualquier sitio, con **una excepción que conviene conocer antes de empezar**: no
compiles dentro de una carpeta que sincronice un servicio en la nube —Google Drive, OneDrive,
Dropbox, iCloud—.

El motivo es que esos programas leen y copian los archivos mientras se están escribiendo. La
compilación genera y reemplaza miles de archivos en pocos segundos, así que el enlazador se
encuentra con archivos bloqueados a media escritura. En Windows el síntoma es:

```
error: linking with `link.exe` failed
LNK1201: error al escribir en el archivo de programa
```

Y no es determinista: unas veces compila y otras no, lo que hace perder bastante tiempo antes
de sospechar del sitio en lugar del código.

**El código fuente sí puede vivir en la carpeta sincronizada**; lo que hay que sacar fuera es
la carpeta de compilación. Se hace con la variable de entorno `CARGO_TARGET_DIR`:

```bash
export CARGO_TARGET_DIR="/una/ruta/fuera/de/la/nube/target-mmcelt"
cargo build
```

```powershell
$env:CARGO_TARGET_DIR = "C:\ruta\fuera\de\la\nube\target-mmcelt"
cargo build
```

Si vas a trabajar así a menudo, conviene fijarlo en un archivo `.cargo/config.toml` dentro
del proyecto, para que valga también cuando compiles desde el editor y no desde la terminal:

```toml
[build]
target-dir = "C:/ruta/fuera/de/la/nube/target-mmcelt"
```

Ese archivo lleva una ruta que solo existe en tu equipo, así que **no lo publiques**: añádelo
a `.gitignore`.

La carpeta de compilación ocupa entre 2 y 5 GB. Se puede borrar entera cuando haga falta
espacio: se regenera sola con el siguiente `cargo build`.

---

### Si creas un acceso a la carpeta de compilación, cuidado con las uniones de directorio

Es cómodo dejar un acceso a la carpeta de compilación en el escritorio. En Windows, lo habitual es
crearlo como **unión de directorio** (*junction*), porque los enlaces simbólicos de verdad exigen
permisos de administrador o el modo desarrollador activado.

> 🔴 **Una unión no es un acceso directo.** Se comporta como si fuera la carpeta de verdad:
> **si borras archivos desde dentro del enlace, borras los archivos reales**, y si mandas el icono
> a la papelera desde el explorador puedes llevarte la carpeta entera por delante.

Para quitar la unión **sin tocar** la carpeta original:

```powershell
Remove-Item "C:\ruta\del\enlace"    # sin -Recurse
```

```cmd
rmdir "C:\ruta\del\enlace"
```

No es grave si ocurre —la carpeta de compilación se rehace sola con el primer `cargo build`—, pero
son varios minutos de espera y un susto evitable.

## 9. Cuentas y accesos externos

**Ninguno es necesario.** MMCelt funciona por completo sin conexión y sin registrarse en
ningún servicio.

Las integraciones con IA usan las cuentas que ya tengas en ChatGPT, Claude o Gemini, pero
**la aplicación nunca ve esas credenciales**: el intercambio ocurre en tu navegador o en
tu agente. Ver [`servicios.md`](servicios.md).
