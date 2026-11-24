//! Contrato real del ejecutable MCP: cada proceso recibe su entorno y su raíz autorizada.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// Directorio exclusivo de esta prueba, retirado al terminar todos los servidores.
struct CarpetaDePrueba(PathBuf);

impl CarpetaDePrueba {
    fn nueva() -> Self {
        let ruta =
            std::env::temp_dir().join(format!("mmcelt_contrato_mcp_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&ruta).expect("crear raíz de prueba");
        Self(ruta)
    }
}

impl Drop for CarpetaDePrueba {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.0) {
            eprintln!("no se pudo retirar {}: {error}", self.0.display());
        }
    }
}

/// Lanza el ejecutable que Cargo construye para esta prueba, con entorno exclusivo.
fn iniciar(raiz: Option<&Path>) -> Child {
    let mut orden = Command::new(env!("CARGO_BIN_EXE_mmcelt"));
    orden
        .arg("--mcp-server")
        .env("MMCELT_IDIOMA", "es")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match raiz {
        Some(raiz) => {
            orden.env("MMCELT_WORKSPACE", raiz);
        }
        None => {
            orden.env_remove("MMCELT_WORKSPACE");
        }
    }
    let mut proceso = orden.spawn().expect("arrancar servidor MCP real");
    let mut entrada = proceso.stdin.take().expect("entrada del servidor");
    writeln!(
        entrada,
        "{}",
        serde_json::json!({
            "jsonrpc":"2.0", "id":1, "method":"tools/call",
            "params":{"name":"mmcelt_workspace_info","arguments":{}}
        })
    )
    .expect("enviar petición");
    drop(entrada);
    proceso
}

/// Extrae el resultado tipado de la única petición y exige salida limpia del servidor.
fn respuesta(proceso: Child) -> serde_json::Value {
    let salida = proceso.wait_with_output().expect("terminar servidor");
    assert!(salida.status.success(), "estado: {:?}", salida.status);
    assert!(
        salida.stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&salida.stderr)
    );
    let sobre: serde_json::Value =
        serde_json::from_slice(&salida.stdout).expect("respuesta JSON-RPC");
    serde_json::from_str(
        sobre["result"]["content"][0]["text"]
            .as_str()
            .expect("contenido MCP"),
    )
    .expect("resultado JSON")
}

/// Tres procesos simultáneos no comparten raíz ni cambian el entorno del proceso de pruebas.
#[test]
fn el_ejecutable_respeta_el_entorno_exclusivo_de_cada_proceso() {
    let entorno_anterior = std::env::var_os("MMCELT_WORKSPACE");
    let a = CarpetaDePrueba::nueva();
    let b = CarpetaDePrueba::nueva();
    let proceso_a = iniciar(Some(&a.0));
    let proceso_b = iniciar(Some(&b.0));
    let proceso_sin_raiz = iniciar(None);
    let resultado_a = respuesta(proceso_a);
    let resultado_b = respuesta(proceso_b);
    let sin_raiz = respuesta(proceso_sin_raiz);
    assert_eq!(
        resultado_a["workspace"],
        a.0.canonicalize().expect("raíz A").display().to_string()
    );
    assert_eq!(
        resultado_b["workspace"],
        b.0.canonicalize().expect("raíz B").display().to_string()
    );
    assert_ne!(resultado_a["workspace"], resultado_b["workspace"]);
    assert_eq!(sin_raiz["error_type"], "security");
    assert_eq!(std::env::var_os("MMCELT_WORKSPACE"), entorno_anterior);
}
