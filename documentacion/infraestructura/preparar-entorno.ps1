# preparar-entorno.ps1 — Configuración del entorno de desarrollo de MMCelt (Windows)
#
# Proyecto:   MMCelt — Mapas mentales para dirigir modelos de IA
# Requisitos: Windows 10 o superior, PowerShell 5.1 o superior
#
# Qué hace:
#   1. Comprueba los requisitos del sistema (Rust y herramientas de C++).
#   2. Ofrece instalar lo que falte.
#   3. Compila el proyecto.
#   4. Ejecuta las pruebas para verificar que todo funciona.
#   5. Explica cómo arrancar y cómo configurar el servidor MCP.
#
# Es idempotente: se puede ejecutar tantas veces como haga falta. No modifica nada que ya
# esté correctamente instalado ni duplica configuraciones.
#
# Uso:
#   .\preparar-entorno.ps1                  Instalación completa
#   .\preparar-entorno.ps1 -SoloVerificar   Solo comprueba, no instala ni compila

[CmdletBinding()]
param(
    [switch]$SoloVerificar
)

# Cualquier error no controlado detiene el script: es preferible parar a dejar el entorno
# a medio configurar.
$ErrorActionPreference = "Stop"

# --- Versiones mínimas exigidas -------------------------------------------------
$VERSION_MINIMA_RUST = [version]"1.85.0"

# --- Estado acumulado -----------------------------------------------------------
$script:problemas = @()
$script:avisos = @()

# ================================================================================
# Funciones auxiliares
# ================================================================================

function Escribir-Titulo {
    param([string]$Texto)
    Write-Host ""
    Write-Host ("=" * 70) -ForegroundColor Cyan
    Write-Host "  $Texto" -ForegroundColor Cyan
    Write-Host ("=" * 70) -ForegroundColor Cyan
}

function Escribir-Paso {
    param([string]$Texto)
    Write-Host "`n> $Texto" -ForegroundColor Yellow
}

function Escribir-Correcto {
    param([string]$Texto)
    Write-Host "  [OK] $Texto" -ForegroundColor Green
}

function Escribir-Fallo {
    param([string]$Texto)
    Write-Host "  [X]  $Texto" -ForegroundColor Red
    $script:problemas += $Texto
}

function Escribir-Aviso {
    param([string]$Texto)
    Write-Host "  [!]  $Texto" -ForegroundColor DarkYellow
    $script:avisos += $Texto
}

function Existe-Comando {
    <#
    .SYNOPSIS
    Indica si un ejecutable está disponible en el PATH.
    #>
    param([string]$Nombre)
    $null -ne (Get-Command $Nombre -ErrorAction SilentlyContinue)
}

function Invocar-Nativo {
    <#
    .SYNOPSIS
    Ejecuta un programa externo y devuelve su código de salida.

    .DESCRIPTION
    Existe por un comportamiento incómodo de PowerShell 5.1: con
    `$ErrorActionPreference = "Stop"`, cualquier línea que un ejecutable escriba en la
    salida de error se convierte en una excepción terminante, **aunque el programa haya
    terminado correctamente**.

    Eso afecta de lleno a `cargo`, que escribe en la salida de error su progreso normal
    («Compiling…», «Finished…»). Sin esta función, una compilación correcta abortaba el
    script.

    Aquí se baja la preferencia a "Continue" solo durante la llamada y se decide el
    resultado por el código de salida, que es la señal fiable.

    .PARAMETER Programa
    Nombre o ruta del ejecutable.

    .PARAMETER Argumentos
    Lista de argumentos que se le pasan.

    .OUTPUTS
    El código de salida del programa (0 significa correcto).
    #>
    param(
        [Parameter(Mandatory = $true)][string]$Programa,
        [string[]]$Argumentos = @()
    )

    $preferencia_anterior = $ErrorActionPreference
    $ErrorActionPreference = "Continue"

    try {
        # `Out-Host` envía la salida del programa directamente a la consola. Sin él,
        # esa salida se mezclaría en el flujo de retorno de la función y quien llama
        # recibiría un array de líneas en lugar del código de salida.
        & $Programa @Argumentos | Out-Host
        return $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $preferencia_anterior
    }
}

function Obtener-Version {
    <#
    .SYNOPSIS
    Extrae el primer número de versión (X.Y.Z) de la salida de un comando.

    .DESCRIPTION
    Devuelve $null si el comando no existe o si su salida no contiene una versión
    reconocible, en lugar de lanzar una excepción.
    #>
    param(
        [string]$Comando,
        [string]$Argumentos = "--version"
    )

    if (-not (Existe-Comando $Comando)) { return $null }

    try {
        $salida = & $Comando $Argumentos 2>&1 | Out-String
        if ($salida -match "(\d+)\.(\d+)\.(\d+)") {
            return [version]"$($Matches[1]).$($Matches[2]).$($Matches[3])"
        }
    } catch {
        return $null
    }

    return $null
}

function Preguntar-Si {
    <#
    .SYNOPSIS
    Pregunta al usuario una confirmación. En modo no interactivo devuelve $false.
    #>
    param([string]$Pregunta)

    # Si no hay consola interactiva (por ejemplo en una CI), no se bloquea esperando.
    if (-not [Environment]::UserInteractive) { return $false }

    $respuesta = Read-Host "  $Pregunta [s/N]"
    return $respuesta -match '^[sSyY]'
}

# ================================================================================
# 1. Comprobación de requisitos
# ================================================================================

Escribir-Titulo "MMCelt — Configuración del entorno de desarrollo"

Escribir-Paso "Comprobando el sistema operativo"

$version_windows = [Environment]::OSVersion.Version
if ($version_windows.Major -ge 10) {
    Escribir-Correcto "Windows $($version_windows.Major).$($version_windows.Minor) (build $($version_windows.Build))"
} else {
    Escribir-Aviso "Windows $($version_windows.Major) es anterior a Windows 10; no se ha probado en esta version"
}

Write-Host "  PowerShell $($PSVersionTable.PSVersion)"

# --- Rust -----------------------------------------------------------------------
Escribir-Paso "Comprobando Rust"

$version_rust = Obtener-Version -Comando "rustc"

if ($null -eq $version_rust) {
    Escribir-Fallo "Rust no esta instalado"

    if (-not $SoloVerificar) {
        Write-Host ""
        Write-Host "  Rust es imprescindible para compilar MMCelt." -ForegroundColor White
        Write-Host "  El instalador se descarga de https://win.rustup.rs/" -ForegroundColor White
        Write-Host ""

        if (Preguntar-Si "Descargar e instalar Rust ahora?") {
            $instalador = Join-Path $env:TEMP "rustup-init.exe"
            Write-Host "  Descargando..." -ForegroundColor Gray
            Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $instalador

            Write-Host "  Ejecutando el instalador (acepta la opcion 1 por defecto)..." -ForegroundColor Gray
            & $instalador -y --default-toolchain stable
            Remove-Item $instalador -ErrorAction SilentlyContinue

            # rustup modifica el PATH, pero la sesión actual no lo ve todavía.
            $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

            $version_rust = Obtener-Version -Comando "rustc"
            if ($null -ne $version_rust) {
                Escribir-Correcto "Rust $version_rust instalado"
                $script:problemas = $script:problemas | Where-Object { $_ -ne "Rust no esta instalado" }
            }
        } else {
            Write-Host "  Instalalo manualmente desde https://rustup.rs y vuelve a ejecutar este script." -ForegroundColor Gray
        }
    }
} elseif ($version_rust -lt $VERSION_MINIMA_RUST) {
    Escribir-Fallo "Rust $version_rust es anterior a la version minima $VERSION_MINIMA_RUST"

    if (-not $SoloVerificar) {
        if (Preguntar-Si "Actualizar Rust con 'rustup update stable'?") {
            Invocar-Nativo -Programa "rustup" -Argumentos @("update", "stable") | Out-Null
            $version_rust = Obtener-Version -Comando "rustc"
            Escribir-Correcto "Rust actualizado a $version_rust"
            $script:problemas = @($script:problemas | Where-Object { $_ -notmatch "^Rust" })
        }
    }
} else {
    Escribir-Correcto "Rust $version_rust"
}

# --- Herramientas de compilación de C++ -----------------------------------------
# Rust necesita en Windows el enlazador de Microsoft. Sin el, la compilacion falla con
# un error sobre link.exe que despista bastante si no se conoce la causa.
Escribir-Paso "Comprobando las herramientas de compilacion de C++"

if (Existe-Comando "link") {
    Escribir-Correcto "Enlazador de Microsoft disponible"
} else {
    $rutas_vs = @(
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio",
        "${env:ProgramFiles}\Microsoft Visual Studio"
    )
    $encontrado = $rutas_vs | Where-Object { Test-Path $_ } | Select-Object -First 1

    if ($encontrado) {
        Escribir-Correcto "Visual Studio detectado en $encontrado"
    } else {
        Escribir-Aviso "No se detectaron las Build Tools de C++"
        Write-Host "     Si la compilacion falla con un error sobre 'link.exe', instalalas desde:" -ForegroundColor Gray
        Write-Host "     https://visualstudio.microsoft.com/visual-cpp-build-tools/" -ForegroundColor Gray
        Write-Host "     Marca la carga de trabajo 'Desarrollo para el escritorio con C++'." -ForegroundColor Gray
    }
}

# --- Componentes de Rust --------------------------------------------------------
if ($null -ne $version_rust -and -not $SoloVerificar) {
    Escribir-Paso "Comprobando los componentes de Rust"

    $preferencia = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $componentes = (rustup component list --installed | Out-String)
    $ErrorActionPreference = $preferencia

    foreach ($componente in @("clippy", "rustfmt")) {
        if ($componentes -match $componente) {
            Escribir-Correcto "$componente ya instalado"
        } else {
            Write-Host "  Instalando $componente..." -ForegroundColor Gray
            $codigo = Invocar-Nativo -Programa "rustup" -Argumentos @("component", "add", $componente)
            if ($codigo -eq 0) {
                Escribir-Correcto "$componente instalado"
            } else {
                Escribir-Aviso "No se pudo instalar $componente"
            }
        }
    }
}

# --- Git ------------------------------------------------------------------------
Escribir-Paso "Comprobando Git"

$version_git = Obtener-Version -Comando "git"
if ($null -eq $version_git) {
    Escribir-Aviso "Git no esta instalado (no es necesario si ya tienes el codigo)"
} else {
    Escribir-Correcto "Git $version_git"
}

# ================================================================================
# 2. Localizar la raíz del proyecto
# ================================================================================

Escribir-Paso "Localizando la raiz del proyecto"

# El script vive en documentacion/infraestructura/, así que la raíz está dos niveles
# por encima. Se resuelve a partir de la ubicación del propio script para que funcione
# se ejecute desde donde se ejecute.
$raiz_proyecto = Resolve-Path (Join-Path $PSScriptRoot "..\..")

if (Test-Path (Join-Path $raiz_proyecto "Cargo.toml")) {
    Escribir-Correcto "Proyecto encontrado en $raiz_proyecto"

    # Detección de carpetas sincronizadas en la nube (Google Drive, OneDrive, Dropbox, etc.)
    $patron_sincronizada = 'Google Drive|Mi unidad|My Drive|OneDrive|Dropbox|iCloud|Nextcloud'
    if ($raiz_proyecto.Path -match $patron_sincronizada) {
        Escribir-Aviso "El proyecto esta ubicado dentro de una carpeta sincronizada en la nube."
        Write-Host "     Advertencia: El enlazador de Windows (link.exe) puede fallar con LNK1201 si el directorio" -ForegroundColor Gray
        Write-Host "     'target' se sincroniza en tiempo real. MMCelt incluye .cargo/config.toml para redirigirlo." -ForegroundColor Gray
    }
} else {
    Escribir-Fallo "No se encontro Cargo.toml en $raiz_proyecto"
    Write-Host "     Este script debe estar en documentacion/infraestructura/ dentro del repositorio." -ForegroundColor Gray
}

# ================================================================================
# 3. Compilar y verificar
# ================================================================================

if ($SoloVerificar) {
    Write-Host "`n> Compilacion omitida (-SoloVerificar)" -ForegroundColor DarkGray
} elseif ($script:problemas.Count -gt 0) {
    Write-Host "`n> Compilacion omitida: hay requisitos sin cumplir" -ForegroundColor DarkYellow
} else {
    Push-Location $raiz_proyecto
    try {
        Escribir-Paso "Compilando el proyecto (la primera vez tarda varios minutos)"

        $codigo_compilacion = Invocar-Nativo -Programa "cargo" -Argumentos @("build", "--release")

        if ($codigo_compilacion -eq 0) {
            Escribir-Correcto "Compilacion completada"

            $binario = Join-Path $raiz_proyecto "target\release\mmcelt.exe"
            if (Test-Path $binario) {
                $tamano = [math]::Round((Get-Item $binario).Length / 1MB, 1)
                Write-Host "     Binario: $binario - $tamano MB" -ForegroundColor Gray
            }
        } else {
            Escribir-Fallo "La compilacion fallo con codigo $codigo_compilacion"
            if ($raiz_proyecto.Path -match 'Google Drive|Mi unidad|My Drive|OneDrive|Dropbox|iCloud|Nextcloud') {
                Write-Host "     Si el fallo menciona LNK1201 o bloqueo de archivos, comprueba que la carpeta 'target'" -ForegroundColor DarkYellow
                Write-Host "     este excluida de la sincronizacion o redirigida fuera de la nube en .cargo/config.toml." -ForegroundColor DarkYellow
            }
        }

        # --- Pruebas de Rust ---
        if ($codigo_compilacion -eq 0) {
            Escribir-Paso "Ejecutando las pruebas de Rust"

            $codigo_pruebas = Invocar-Nativo -Programa "cargo" -Argumentos @("test")
            if ($codigo_pruebas -eq 0) {
                Escribir-Correcto "Todas las pruebas de Rust pasan"
            } else {
                Escribir-Fallo "Hay pruebas de Rust que fallan"
            }
        }

        # --- Linter ---
        Escribir-Paso "Ejecutando clippy"

        $codigo_clippy = Invocar-Nativo -Programa "cargo" `
            -Argumentos @("clippy", "--all-targets", "--all-features", "--", "-D", "warnings")

        if ($codigo_clippy -eq 0) {
            Escribir-Correcto "clippy no reporta ningun aviso"
        } else {
            Escribir-Aviso "clippy reporta avisos; revisalos antes de commitear"
        }

    } finally {
        Pop-Location
    }
}

# ================================================================================
# 4. Resumen final
# ================================================================================

Escribir-Titulo "Resumen"

if ($script:problemas.Count -eq 0) {
    Write-Host ""
    Write-Host "  Entorno listo." -ForegroundColor Green
    Write-Host ""
    Write-Host "  Para arrancar la aplicacion:" -ForegroundColor White
    Write-Host "      cd `"$raiz_proyecto`"" -ForegroundColor Cyan
    Write-Host "      cargo run --release" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Para conectar tus agentes de IA, lo mas comodo es hacerlo desde el propio" -ForegroundColor White
    Write-Host "  programa: menu 'Inteligencia Artificial > Conectar MMCelt con mis IAs'." -ForegroundColor White
    Write-Host ""
    Write-Host "  Y si prefieres configurarlo a mano, esta es la entrada:" -ForegroundColor White
    Write-Host ""

    # De donde sale el binario de verdad. Si quien ejecuta esto tiene CARGO_TARGET_DIR
    # definido -que es lo que la propia documentacion recomienda cuando el proyecto vive en
    # una carpeta sincronizada con la nube- la carpeta "target" dentro del proyecto NO existe,
    # y esta configuracion apuntaba a un ejecutable inexistente: el agente no arrancaba.
    $carpeta_target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $raiz_proyecto "target" }
    $ruta_exe = (Join-Path $carpeta_target "release\mmcelt.exe").Replace([char]92, [char]47)

    if (-not (Test-Path $ruta_exe)) {
        Escribir-Aviso "No se encontro el ejecutable en $ruta_exe; compila con 'cargo build --release' antes de usar esta configuracion"
    }

    # La carpeta de trabajo se crea aqui. El servidor MCP rechaza TODAS las escrituras si no
    # existe -con un error de seguridad que ademas culpa a otra cosa-, asi que proponer una
    # ruta sin crearla dejaba la integracion inservible. Y se pide al sistema el nombre real
    # del directorio: en un Windows en espanol es "Documents", no "Documentos".
    $espacio_trabajo = Join-Path ([Environment]::GetFolderPath('MyDocuments')) "MapasMentales"
    if (-not (Test-Path $espacio_trabajo)) {
        New-Item -ItemType Directory -Path $espacio_trabajo -Force | Out-Null
        Escribir-Correcto "Carpeta de trabajo de los agentes creada: $espacio_trabajo"
    }
    $espacio_trabajo = $espacio_trabajo.Replace([char]92, [char]47)
    Write-Host @"
      {
        "mcpServers": {
          "mmcelt": {
            "command": "$ruta_exe",
            "args": ["--mcp-server"],
            "env": { "MMCELT_WORKSPACE": "$espacio_trabajo" }
          }
        }
      }
"@ -ForegroundColor Cyan

    Write-Host ""
    Write-Host "  Documentacion completa en documentacion/README.md" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "  Quedan $($script:problemas.Count) problema(s) por resolver:" -ForegroundColor Red
    foreach ($problema in $script:problemas) {
        Write-Host "    - $problema" -ForegroundColor Red
    }
    Write-Host ""
    Write-Host "  Resuelvelos y vuelve a ejecutar este script." -ForegroundColor Gray
}

if ($script:avisos.Count -gt 0) {
    Write-Host ""
    Write-Host "  Avisos (no impiden trabajar):" -ForegroundColor DarkYellow
    foreach ($aviso in $script:avisos) {
        Write-Host "    - $aviso" -ForegroundColor DarkYellow
    }
}

Write-Host ""

# Codigo de salida: 0 si todo esta correcto, 1 si queda algo por resolver.
if ($script:problemas.Count -gt 0) { exit 1 } else { exit 0 }
