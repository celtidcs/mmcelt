#!/usr/bin/env bash
#
# preparar-entorno.sh — Configuración del entorno de desarrollo de MMCelt (Linux y macOS)
#
# Proyecto:   MMCelt — Mapas mentales para dirigir modelos de IA
# Requisitos: bash 4.0 o superior
#
# Qué hace:
#   1. Comprueba los requisitos del sistema (Rust y bibliotecas gráficas).
#   2. Ofrece instalar lo que falte.
#   3. Compila el proyecto.
#   4. Ejecuta las pruebas para verificar que todo funciona.
#   5. Explica cómo arrancar y cómo configurar el servidor MCP.
#
# Es idempotente: se puede ejecutar tantas veces como haga falta.
#
# Uso:
#   ./preparar-entorno.sh                  Instalación completa
#   ./preparar-entorno.sh --solo-verificar Solo comprueba, no instala ni compila
#
# NOTA SOBRE EL SOPORTE: la plataforma principal del proyecto es Windows. Linux y macOS
# deberían funcionar —nada en el código los excluye— pero no están verificados. Si
# encuentras algún problema en ellos, merece la pena documentarlo.

set -o pipefail

# --- Versiones mínimas exigidas -----------------------------------------------
readonly VERSION_MINIMA_RUST="1.85.0"

# --- Colores (se desactivan si la salida no es un terminal) -------------------
if [ -t 1 ]; then
    readonly ROJO=$'\033[0;31m'
    readonly VERDE=$'\033[0;32m'
    readonly AMARILLO=$'\033[0;33m'
    readonly CIAN=$'\033[0;36m'
    readonly GRIS=$'\033[0;90m'
    readonly SIN_COLOR=$'\033[0m'
else
    readonly ROJO='' VERDE='' AMARILLO='' CIAN='' GRIS='' SIN_COLOR=''
fi

# --- Estado acumulado ---------------------------------------------------------
problemas=()
avisos=()

# --- Opciones -----------------------------------------------------------------
solo_verificar=false

for argumento in "$@"; do
    case "$argumento" in
        --solo-verificar) solo_verificar=true ;;
        -h|--help)
            sed -n '3,25p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        *)
            echo "Opción desconocida: $argumento" >&2
            echo "Usa --help para ver las opciones disponibles." >&2
            exit 1
            ;;
    esac
done

# ==============================================================================
# Funciones auxiliares
# ==============================================================================

escribir_titulo() {
    echo
    echo "${CIAN}======================================================================${SIN_COLOR}"
    echo "${CIAN}  $1${SIN_COLOR}"
    echo "${CIAN}======================================================================${SIN_COLOR}"
}

escribir_paso()     { echo; echo "${AMARILLO}> $1${SIN_COLOR}"; }
escribir_correcto() { echo "  ${VERDE}[OK] $1${SIN_COLOR}"; }
escribir_fallo()    { echo "  ${ROJO}[X]  $1${SIN_COLOR}"; problemas+=("$1"); }
escribir_aviso()    { echo "  ${AMARILLO}[!]  $1${SIN_COLOR}"; avisos+=("$1"); }
escribir_detalle()  { echo "     ${GRIS}$1${SIN_COLOR}"; }

existe_comando() {
    # Indica si un ejecutable está disponible en el PATH.
    command -v "$1" >/dev/null 2>&1
}

version_mayor_o_igual() {
    # Compara dos versiones con formato X.Y.Z.
    #
    # Argumentos:
    #   $1: versión encontrada
    #   $2: versión mínima exigida
    #
    # Devuelve 0 (cierto en bash) si $1 >= $2.
    #
    # `sort -V` ordena por versión: si la menor de las dos es la mínima exigida,
    # entonces la encontrada la cumple.
    [ "$(printf '%s\n%s\n' "$2" "$1" | sort -V | head -n1)" = "$2" ]
}

obtener_version() {
    # Extrae el primer número de versión X.Y.Z de la salida de un comando.
    #
    # Argumentos:
    #   $1: nombre del ejecutable
    #
    # Escribe la versión en la salida estándar, o nada si no se encuentra.
    existe_comando "$1" || return 1
    "$1" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?' | head -n1
}

preguntar_si() {
    # Pregunta una confirmación al usuario.
    #
    # En un entorno no interactivo (por ejemplo una CI) responde que no, para no
    # quedarse bloqueado esperando una entrada que nunca llegará.
    [ -t 0 ] || return 1

    local respuesta
    read -r -p "  $1 [s/N] " respuesta
    [[ "$respuesta" =~ ^[sSyY] ]]
}

detectar_gestor_paquetes() {
    # Identifica el gestor de paquetes del sistema.
    if existe_comando apt-get;  then echo "apt"
    elif existe_comando dnf;    then echo "dnf"
    elif existe_comando pacman; then echo "pacman"
    elif existe_comando brew;   then echo "brew"
    else echo "desconocido"
    fi
}

# ==============================================================================
# 1. Comprobación de requisitos
# ==============================================================================

escribir_titulo "MMCelt — Configuración del entorno de desarrollo"

escribir_paso "Comprobando el sistema operativo"

sistema="$(uname -s)"
gestor="$(detectar_gestor_paquetes)"

case "$sistema" in
    Linux)
        escribir_correcto "Linux ($(uname -r)), gestor de paquetes: $gestor"
        escribir_detalle "Recuerda: la plataforma verificada del proyecto es Windows."
        ;;
    Darwin)
        escribir_correcto "macOS ($(uname -r)), gestor de paquetes: $gestor"
        escribir_detalle "Recuerda: la plataforma verificada del proyecto es Windows."
        ;;
    MINGW*|MSYS*|CYGWIN*)
        # Git Bash, MSYS2 o Cygwin sobre Windows. El script funciona, pero en Windows
        # es preferible preparar-entorno.ps1: detecta las Build Tools de C++ y sabe instalarlas.
        escribir_correcto "Windows con entorno tipo Unix ($sistema)"
        escribir_detalle "En Windows es preferible usar preparar-entorno.ps1, que además comprueba"
        escribir_detalle "las herramientas de compilación de C++ que Rust necesita."
        ;;
    *)
        escribir_aviso "Sistema no reconocido: $sistema. El script puede no funcionar."
        ;;
esac

# --- Rust ---------------------------------------------------------------------
escribir_paso "Comprobando Rust"

version_rust="$(obtener_version rustc || true)"

if [ -z "$version_rust" ]; then
    escribir_fallo "Rust no está instalado"

    if [ "$solo_verificar" = false ]; then
        echo
        echo "  Rust es imprescindible para compilar MMCelt."
        echo "  Se instala con rustup desde https://rustup.rs"
        echo

        if preguntar_si "¿Instalar Rust ahora?"; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

            # rustup modifica el PATH del intérprete de órdenes, pero la sesión actual
            # todavía no lo ve.
            # shellcheck disable=SC1090,SC1091
            [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

            version_rust="$(obtener_version rustc || true)"
            if [ -n "$version_rust" ]; then
                escribir_correcto "Rust $version_rust instalado"
                problemas=()
            fi
        else
            escribir_detalle "Instálalo manualmente y vuelve a ejecutar este script."
        fi
    fi
elif ! version_mayor_o_igual "$version_rust" "$VERSION_MINIMA_RUST"; then
    escribir_fallo "Rust $version_rust es anterior a la versión mínima $VERSION_MINIMA_RUST"

    if [ "$solo_verificar" = false ] && preguntar_si "¿Actualizar con 'rustup update stable'?"; then
        rustup update stable
        version_rust="$(obtener_version rustc || true)"
        escribir_correcto "Rust actualizado a $version_rust"
        problemas=()
    fi
else
    escribir_correcto "Rust $version_rust"
fi

# --- Bibliotecas del sistema (solo Linux) -------------------------------------
# En Linux, eframe necesita las bibliotecas de desarrollo del sistema de ventanas, y rfd
# necesita GTK 3 para los diálogos nativos de archivo. Sin ellas la compilación falla con
# errores de enlazado que despistan bastante.
if [ "$sistema" = "Linux" ]; then
    escribir_paso "Comprobando las bibliotecas gráficas del sistema"

    faltantes=()
    if existe_comando pkg-config; then
        for biblioteca in gtk+-3.0 xkbcommon; do
            pkg-config --exists "$biblioteca" 2>/dev/null || faltantes+=("$biblioteca")
        done
    else
        faltantes+=("pkg-config")
    fi

    if [ ${#faltantes[@]} -eq 0 ]; then
        escribir_correcto "Bibliotecas gráficas disponibles"
    else
        escribir_aviso "Faltan bibliotecas: ${faltantes[*]}"

        case "$gestor" in
            apt)
                comando_instalacion="sudo apt-get install -y build-essential pkg-config libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev"
                ;;
            dnf)
                comando_instalacion="sudo dnf install -y gcc-c++ pkgconf-pkg-config gtk3-devel libxkbcommon-devel"
                ;;
            pacman)
                comando_instalacion="sudo pacman -S --needed base-devel pkgconf gtk3 libxkbcommon"
                ;;
            *)
                comando_instalacion=""
                ;;
        esac

        if [ -n "$comando_instalacion" ]; then
            escribir_detalle "$comando_instalacion"
            if [ "$solo_verificar" = false ] && preguntar_si "¿Ejecutar ese comando ahora?"; then
                # shellcheck disable=SC2086
                if $comando_instalacion; then
                    escribir_correcto "Bibliotecas instaladas"
                    avisos=()
                else
                    escribir_fallo "La instalación de las bibliotecas falló"
                fi
            fi
        else
            escribir_detalle "Instala manualmente las bibliotecas de desarrollo de GTK3 y xkbcommon."
        fi
    fi
fi

# --- Componentes de Rust ------------------------------------------------------
if [ -n "$version_rust" ] && [ "$solo_verificar" = false ] && existe_comando rustup; then
    escribir_paso "Comprobando los componentes de Rust"

    componentes_instalados="$(rustup component list --installed 2>/dev/null || true)"

    for componente in clippy rustfmt; do
        if echo "$componentes_instalados" | grep -q "$componente"; then
            escribir_correcto "$componente ya instalado"
        else
            echo "  Instalando $componente..."
            if rustup component add "$componente"; then
                escribir_correcto "$componente instalado"
            else
                escribir_aviso "No se pudo instalar $componente"
            fi
        fi
    done
fi

# --- Git ----------------------------------------------------------------------
escribir_paso "Comprobando Git"

if existe_comando git; then
    escribir_correcto "Git $(obtener_version git)"
else
    escribir_aviso "Git no está instalado (no es necesario si ya tienes el código)"
fi

# ==============================================================================
# 2. Localizar la raíz del proyecto
# ==============================================================================

escribir_paso "Localizando la raíz del proyecto"

# El script vive en documentacion/infraestructura/, así que la raíz está dos niveles por
# encima. Se resuelve desde la ubicación del propio script para que funcione se ejecute
# desde donde se ejecute.
directorio_script="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
raiz_proyecto="$(cd "$directorio_script/../.." && pwd)"

if [ -f "$raiz_proyecto/Cargo.toml" ]; then
    escribir_correcto "Proyecto encontrado en $raiz_proyecto"

    # Detección de carpetas sincronizadas en la nube (Google Drive, OneDrive, Dropbox, etc.)
    if echo "$raiz_proyecto" | grep -qiE 'Google Drive|Mi unidad|My Drive|OneDrive|Dropbox|iCloud|Nextcloud'; then
        escribir_aviso "El proyecto está ubicado dentro de una carpeta sincronizada en la nube."
        escribir_detalle "Si compilas en Windows o montajes compartidos, asegúrate de redirigir target/ (.cargo/config.toml)."
    fi
else
    escribir_fallo "No se encontró Cargo.toml en $raiz_proyecto"
    escribir_detalle "Este script debe estar en documentacion/infraestructura/ dentro del repositorio."
fi

# ==============================================================================
# 3. Compilar y verificar
# ==============================================================================

if [ "$solo_verificar" = true ]; then
    echo
    echo "${GRIS}> Compilación omitida (--solo-verificar)${SIN_COLOR}"
elif [ ${#problemas[@]} -gt 0 ]; then
    echo
    echo "${AMARILLO}> Compilación omitida: hay requisitos sin cumplir${SIN_COLOR}"
else
    cd "$raiz_proyecto" || exit 1

    escribir_paso "Compilando el proyecto (la primera vez tarda varios minutos)"

    if cargo build --release; then
        escribir_correcto "Compilación completada"

        binario="$raiz_proyecto/target/release/mmcelt"
        if [ -f "$binario" ]; then
            tamano="$(du -h "$binario" | cut -f1)"
            escribir_detalle "Binario: $binario ($tamano)"
        fi

        # --- Pruebas de Rust ---
        escribir_paso "Ejecutando las pruebas de Rust"
        if cargo test; then
            escribir_correcto "Todas las pruebas de Rust pasan"
        else
            escribir_fallo "Hay pruebas de Rust que fallan"
        fi
    else
        escribir_fallo "La compilación falló"
        if echo "$raiz_proyecto" | grep -qiE 'Google Drive|Mi unidad|My Drive|OneDrive|Dropbox|iCloud|Nextcloud'; then
            escribir_detalle "Atención: en carpetas sincronizadas, comprueba que target/ no esté bloqueada por el sincronizador."
        fi
    fi

    # --- Linter ---
    escribir_paso "Ejecutando clippy"
    if cargo clippy --all-targets --all-features -- -D warnings; then
        escribir_correcto "clippy no reporta ningún aviso"
    else
        escribir_aviso "clippy reporta avisos; revísalos antes de commitear"
    fi

fi

# ==============================================================================
# 4. Resumen final
# ==============================================================================

escribir_titulo "Resumen"

if [ ${#problemas[@]} -eq 0 ]; then
    echo
    echo "  ${VERDE}Entorno listo.${SIN_COLOR}"
    echo
    echo "  Para arrancar la aplicación:"
    echo "      ${CIAN}cd \"$raiz_proyecto\"${SIN_COLOR}"
    echo "      ${CIAN}cargo run --release${SIN_COLOR}"
    echo
    echo "  Para conectar tus agentes de IA, lo más cómodo es hacerlo desde el propio"
    echo "  programa: menú «Inteligencia Artificial → Conectar MMCelt con mis IAs»."
    echo
    echo "  Y si prefieres configurarlo a mano, esta es la entrada:"
    echo

    # De dónde sale el binario de verdad. Con CARGO_TARGET_DIR definido —lo que la propia
    # documentación recomienda cuando el proyecto vive en una carpeta sincronizada con la
    # nube—, la carpeta «target» dentro del proyecto no existe, y esta configuración apuntaba
    # a un ejecutable inexistente: el agente no llegaba a arrancar el servidor.
    carpeta_target="${CARGO_TARGET_DIR:-$raiz_proyecto/target}"
    ruta_exe="$carpeta_target/release/mmcelt"

    if [ ! -f "$ruta_exe" ]; then
        escribir_aviso "No se encontró el ejecutable en $ruta_exe; compila con 'cargo build --release' antes de usar esta configuración"
    fi

    # La carpeta de trabajo se crea aquí. El servidor MCP rechaza **todas** las escrituras si
    # no existe —con un error de seguridad que además culpa a otra cosa—, así que proponer una
    # ruta sin crearla dejaba la integración inservible.
    espacio_trabajo="$HOME/MapasMentales"
    if [ ! -d "$espacio_trabajo" ]; then
        mkdir -p "$espacio_trabajo"
        escribir_correcto "Carpeta de trabajo de los agentes creada: $espacio_trabajo"
    fi
    cat <<FIN
${CIAN}      {
        "mcpServers": {
          "mmcelt": {
            "command": "$ruta_exe",
            "args": ["--mcp-server"],
            "env": { "MMCELT_WORKSPACE": "$espacio_trabajo" }
          }
        }
      }${SIN_COLOR}
FIN
    echo
    escribir_detalle "Documentación completa en documentacion/README.md"
else
    echo
    echo "  ${ROJO}Quedan ${#problemas[@]} problema(s) por resolver:${SIN_COLOR}"
    for problema in "${problemas[@]}"; do
        echo "    ${ROJO}- $problema${SIN_COLOR}"
    done
    echo
    escribir_detalle "Resuélvelos y vuelve a ejecutar este script."
fi

if [ ${#avisos[@]} -gt 0 ]; then
    echo
    echo "  ${AMARILLO}Avisos (no impiden trabajar):${SIN_COLOR}"
    for aviso in "${avisos[@]}"; do
        echo "    ${AMARILLO}- $aviso${SIN_COLOR}"
    done
fi

echo

# Código de salida: 0 si todo está correcto, 1 si queda algo por resolver.
[ ${#problemas[@]} -eq 0 ]
