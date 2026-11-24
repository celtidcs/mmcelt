//! Ejecución de CI sobre versiones explícitas de Linux.

/// Ejecutores declarados por trabajos y matrices, sin interpretar órdenes de los pasos.
///
/// Solo admite el subconjunto estructural usado por los flujos del proyecto: claves en
/// bloque, listas de escalares y entradas de include. Una forma no admitida falla cerrada.
fn ejecutores_del_flujo(yaml: &str) -> Result<Vec<String>, String> {
    let mut ruta: Vec<(usize, String)> = Vec::new();
    let mut ejecutores = Vec::new();
    let mut bloque_literal = None;
    for original in yaml.lines() {
        let linea = super::linea_yaml_sin_comentario(original);
        if linea.trim().is_empty() {
            continue;
        }
        let sangria = super::sangria_yaml(linea);
        if bloque_literal.is_some_and(|nivel| sangria > nivel) {
            continue;
        }
        bloque_literal = None;
        while ruta.last().is_some_and(|(nivel, _)| *nivel >= sangria) {
            ruta.pop();
        }
        let contenido = linea.trim_start().trim_start_matches("- ");
        let Some((clave, valor)) = contenido.split_once(':') else {
            continue;
        };
        let clave = clave.trim().trim_matches(['\'', '"']);
        let valor = valor.trim();
        if valor == "|" || valor == ">" || valor == "|-" || valor == ">-" {
            bloque_literal = Some(sangria);
        }
        let en_trabajos = ruta.iter().any(|(_, parte)| parte == "jobs");
        let en_matriz = ruta.iter().any(|(_, parte)| parte == "matrix");
        let es_ejecutor = en_trabajos && (clave == "runs-on" || (en_matriz && clave == "sistema"));
        if (clave == "jobs" || clave == "matrix") && !valor.is_empty() {
            return Err(format!("mapa en línea no admitido para {clave}"));
        }
        if es_ejecutor && !valor.starts_with("${{") {
            if valor.is_empty() || valor.starts_with('{') {
                return Err(format!("ejecutor no reconocido: {valor}"));
            }
            let valores = valor.trim_start_matches('[').trim_end_matches(']');
            for valor in valores.split(',') {
                ejecutores.push(valor.trim().trim_matches(['\'', '"']).to_string());
            }
        }
        if valor.is_empty() {
            ruta.push((sangria, clave.to_string()));
        }
    }
    if ejecutores.is_empty() {
        return Err("sin ejecutores reconocidos".into());
    }
    Ok(ejecutores)
}

/// Calibra bloques, listas, include y el texto que no es una declaración de ejecutor.
#[test]
fn la_lectura_de_ejecutores_reconoce_la_estructura_yaml() {
    let flujo = "jobs:\n  uno:\n    runs-on: ubuntu-24.04\n    steps:\n      - run: |\n          runs-on: ubuntu-latest\n  dos:\n    runs-on: ${{ matrix.sistema }}\n    strategy:\n      matrix:\n        sistema: [windows-latest, 'ubuntu-24.04']\n  tres:\n    strategy:\n      matrix:\n        include:\n          - sistema: ubuntu-24.04\n            archivo: ejemplo\n";
    assert_eq!(
        ejecutores_del_flujo(flujo).expect("flujo conocido"),
        [
            "ubuntu-24.04",
            "windows-latest",
            "ubuntu-24.04",
            "ubuntu-24.04"
        ]
    );
    assert!(ejecutores_del_flujo("jobs: {uno: {runs-on: ubuntu-latest}}").is_err());
}

/// La versión de Linux de cada trabajo es explícita y coincide en ambos flujos.
#[test]
fn los_flujos_fijan_la_version_de_ubuntu() {
    const EJECUTOR_LINUX: &str = "ubuntu-24.04";
    for (nombre, contenido) in [
        ("ci", include_str!("../.github/workflows/ci.yml")),
        (
            "publicacion",
            include_str!("../.github/workflows/publicacion.yml"),
        ),
    ] {
        let ejecutores =
            ejecutores_del_flujo(contenido).expect("estructura de ejecutores conocida");
        let linux: Vec<_> = ejecutores
            .iter()
            .filter(|valor| valor.starts_with("ubuntu-"))
            .collect();
        assert!(!linux.is_empty(), "{nombre}: sin trabajos Linux");
        assert!(
            linux.iter().all(|valor| valor.as_str() == EJECUTOR_LINUX),
            "{nombre}: se esperaba {EJECUTOR_LINUX}, ejecutores {linux:?}"
        );
    }
}

/// Pasos de un flujo que compilan en modo de publicación, cada uno con su texto completo.
///
/// Un paso empieza en una línea `- name:` y llega hasta el siguiente. No interpreta el YAML: solo
/// separa el texto, que basta para ver qué variables prepara cada paso antes de compilar.
fn pasos_que_compilan_para_publicar(yaml: &str) -> Vec<String> {
    let mut pasos: Vec<String> = Vec::new();
    for linea in yaml.lines() {
        if linea.trim_start().starts_with("- name:") {
            pasos.push(String::new());
        }
        if let Some(paso) = pasos.last_mut() {
            paso.push_str(linea);
            paso.push('\n');
        }
    }
    pasos.retain(|paso| paso.contains("cargo build --release"));
    pasos
}

/// Calibra la separación: un paso sin compilar no cuenta y uno con compilación sí.
#[test]
fn la_lectura_de_pasos_separa_cada_compilacion() {
    let flujo = "steps:\n  - name: uno\n    run: cargo test\n  - name: dos\n    run: |\n      export A=1\n      cargo build --release\n";
    let pasos = pasos_que_compilan_para_publicar(flujo);
    assert_eq!(pasos.len(), 1);
    assert!(pasos[0].contains("export A=1"));
}

/// El ejecutable publicado no lleva rutas de la máquina que lo compila.
///
/// Rust incrusta las rutas de compilación en los mensajes de error de las dependencias. Cada
/// paso de `publicacion.yml` que compila en modo de publicación debe cambiar por nombres neutros
/// la carpeta de cargo y la del repositorio con `--remap-path-prefix`, y pasarlo por
/// `CARGO_ENCODED_RUSTFLAGS`: con `RUSTFLAGS`, cargo parte los argumentos por los espacios y una
/// ruta con espacios rompe la compilación.
#[test]
fn la_publicacion_quita_las_rutas_de_compilacion() {
    // Uno por sistema de la matriz: Windows y Linux.
    const COMPILACIONES_DE_PUBLICACION: usize = 2;
    let pasos =
        pasos_que_compilan_para_publicar(include_str!("../.github/workflows/publicacion.yml"));
    assert_eq!(
        pasos.len(),
        COMPILACIONES_DE_PUBLICACION,
        "publicacion: se esperaban {COMPILACIONES_DE_PUBLICACION} pasos de compilación"
    );
    for paso in &pasos {
        let cambios = paso.matches("--remap-path-prefix=").count();
        // Los comentarios del flujo pueden nombrar RUSTFLAGS para explicar por qué no se usa.
        let usa_rustflags_simple = paso.lines().any(|linea| {
            let orden = linea.trim_start();
            !orden.starts_with('#')
                && orden.contains("RUSTFLAGS")
                && !orden.contains("CARGO_ENCODED_RUSTFLAGS")
        });
        assert!(
            !usa_rustflags_simple && paso.contains("CARGO_ENCODED_RUSTFLAGS"),
            "publicacion: los remapeos deben ir en CARGO_ENCODED_RUSTFLAGS, no en RUSTFLAGS:\n{paso}"
        );
        assert!(
            cambios >= 2 && paso.contains("=cargo") && paso.contains("=mmcelt"),
            "publicacion: un paso compila sin quitar las rutas de cargo y del repositorio:\n{paso}"
        );
    }
}
