//! Aislamiento de instancias del servidor MCP, sin entorno global mutable.

/// Las pruebas MCP no cambian el entorno del proceso que comparte toda la batería.
#[test]
fn las_pruebas_mcp_no_modifican_el_entorno_global() {
    let auxiliares = include_str!("audit_checks.rs");
    for escritura in ["std::env::set_var(", "std::env::remove_var("] {
        assert!(
            !auxiliares.contains(escritura),
            "entorno global compartido: {escritura}"
        );
    }
}

/// Dos instancias atienden a la vez sin mezclar raíces ni permitir acceso a la otra.
#[test]
fn dos_instancias_mcp_conservan_sus_raices_en_paralelo() {
    let primero = super::aislar_espacio_mcp("paralelo_uno");
    let segundo = super::aislar_espacio_mcp("paralelo_dos");
    let raices = [primero.carpeta.clone(), segundo.carpeta.clone()];
    std::thread::scope(|hilos| {
        let trabajos: Vec<_> = raices
            .iter()
            .enumerate()
            .map(|(indice, raiz)| {
                let exterior = &raices[1 - indice];
                hilos.spawn(move || {
                    let contexto = crate::mcp_server::EspacioMcp::nuevo(Some(raiz.clone()));
                    let saludo = crate::mcp_server::procesar_peticion(
                        &contexto,
                        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
                    )
                    .expect("saludo");
                    let saludo: serde_json::Value = serde_json::from_str(&saludo).expect("JSON");
                    let instrucciones = saludo["result"]["instructions"]
                        .as_str()
                        .expect("instrucciones");
                    assert!(instrucciones
                        .contains(&raiz.canonicalize().expect("raíz").display().to_string()));
                    let peticion = serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/call",
                    "params":{"name":"mmcelt_read_mindmap","arguments":{
                        "file_path": exterior.join("ajeno.mmcelt").display().to_string()}}});
                    let respuesta =
                        crate::mcp_server::procesar_peticion(&contexto, &peticion.to_string())
                            .expect("respuesta");
                    assert!(
                        respuesta.contains("security"),
                        "no puede leer fuera de su raíz: {respuesta}"
                    );
                })
            })
            .collect();
        for trabajo in trabajos {
            trabajo.join().expect("instancia aislada");
        }
    });
}
