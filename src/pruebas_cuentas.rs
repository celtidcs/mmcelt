//! Cuentas de los agentes (0.16.0).
//!
//! MMCelt encuentra todas las cuentas de cada herramienta, registra el servidor en la de cada
//! una y abre su consola con ella. La búsqueda en sí se prueba en la biblioteca
//! `deteccion-agentes`; aquí se prueba lo que MMCelt hace con lo encontrado.

use super::carpeta_temporal_contexto;
use crate::conectores::{self, CuentaManual, IdAgente};
use deteccion_agentes::{Cuenta, Herramienta, OrigenDeLaCuenta};
use std::ffi::{OsStr, OsString};

/// Variable con la que Claude Code elige su cuenta.
const VARIABLE_CLAUDE: &str = "CLAUDE_CONFIG_DIR";

/// Cada cuenta es un agente propio y «Conectar» escribe en el archivo de esa cuenta,
/// sin tocar el de las demás.
#[test]
fn cada_cuenta_es_un_agente_que_registra_en_su_archivo() {
    let personal = carpeta_temporal_contexto("ag-cuentas");
    let espacio = personal.join("proyecto");
    std::fs::create_dir_all(&espacio).expect("crear el proyecto");
    for carpeta in [".claude", ".claude2"] {
        std::fs::create_dir_all(personal.join(carpeta)).expect("crear la cuenta");
        std::fs::write(personal.join(carpeta).join("settings.json"), "{}").expect("ajustes");
    }
    let por_defecto = personal.join(".claude.json");
    let segunda = personal.join(".claude2").join(".claude.json");
    std::fs::write(&por_defecto, "{}").expect("configuración por defecto");
    std::fs::write(&segunda, "{}").expect("configuración de la cuenta 2");

    let cuentas = deteccion_agentes::buscar_cuentas(&personal, |_| None);
    let entorno = conectores::EntornoDeBusqueda {
        personal: &personal,
        datos_locales: None,
        path: OsStr::new(""),
        pathext: OsStr::new(""),
        sistema: conectores::sistema_actual(),
    };
    let agentes = conectores::agentes_de_las_cuentas(&entorno, &Default::default(), &cuentas);
    let claude: Vec<_> = agentes
        .iter()
        .filter(|agente| agente.id == IdAgente::ClaudeCode)
        .collect();

    assert_eq!(claude.len(), 2, "una ficha por cuenta");
    assert_eq!(claude[0].cuenta.nombre_visible(), None);
    assert_eq!(
        claude[1].cuenta.nombre_visible().as_deref(),
        Some(".claude2")
    );
    assert_eq!(claude[1].ruta_configuracion, segunda);

    conectores::conectar(claude[1], &espacio).expect("conectar la cuenta 2");
    let leer = |ruta: &std::path::Path| std::fs::read_to_string(ruta).expect("leer");
    assert!(
        leer(&segunda).contains("mmcelt"),
        "la cuenta 2 queda conectada"
    );
    assert!(
        !leer(&por_defecto).contains("mmcelt"),
        "la cuenta por defecto no se toca"
    );
    std::fs::remove_dir_all(personal).expect("limpiar");
}

/// La consola de una cuenta se abre con la variable apuntando a su carpeta, y la de la
/// cuenta por defecto quitando la variable, por si MMCelt la hereda puesta.
#[test]
fn la_consola_se_abre_con_la_variable_de_su_cuenta() {
    let contenedor = carpeta_temporal_contexto("ag-consola");
    let raiz = contenedor.join("proyecto");
    std::fs::create_dir_all(&raiz).expect("crear el proyecto");
    let ejecutable = contenedor.join("bin").join("claude.exe");
    std::fs::create_dir_all(contenedor.join("bin")).expect("crear la carpeta del programa");
    std::fs::write(&ejecutable, "simulado").expect("crear el programa");
    let carpeta = contenedor.join(".claude2");

    for (origen, valor) in [
        (
            OrigenDeLaCuenta::Busqueda,
            Some(carpeta.clone().into_os_string()),
        ),
        (OrigenDeLaCuenta::PorDefecto, None),
    ] {
        let mut agente = super::agente_cli_para_vista_previa(ejecutable.clone());
        agente.cuenta = Cuenta {
            herramienta: Herramienta::ClaudeCode,
            carpeta: carpeta.clone(),
            origen,
        };
        let confirmado = crate::lanzador_agentes::confirmar_ejecutable(&agente, &raiz)
            .expect("confirmar el programa");
        let orden = crate::lanzador_agentes::OrdenCli {
            programa: ejecutable.clone(),
            argumentos: Vec::new(),
            directorio: raiz.clone(),
            ejecutable_confirmado: confirmado,
        };
        let mut comando = std::process::Command::new(&ejecutable);
        crate::lanzador_agentes::ajustar_la_cuenta(&orden, &mut comando);
        let entorno: Vec<(OsString, Option<OsString>)> = comando
            .get_envs()
            .map(|(clave, valor)| (clave.to_owned(), valor.map(OsStr::to_owned)))
            .collect();

        assert_eq!(
            entorno,
            vec![(OsString::from(VARIABLE_CLAUDE), valor)],
            "{origen:?}"
        );
    }
    std::fs::remove_dir_all(contenedor).expect("limpiar");
}

/// Unas preferencias de la 0.15, sin cuentas a mano, se siguen leyendo; y una cuenta
/// añadida a mano sobrevive a guardar y volver a leer.
#[test]
fn las_cuentas_a_mano_se_guardan_y_las_preferencias_antiguas_se_leen() {
    let carpeta = carpeta_temporal_contexto("ag-preferencias");
    crate::preferencias::Preferencias::default()
        .guardar_en(&carpeta)
        .expect("guardar");
    let archivo = std::fs::read_dir(&carpeta)
        .expect("listar")
        .flatten()
        .map(|entrada| entrada.path())
        .find(|ruta| {
            std::fs::read_to_string(ruta).is_ok_and(|texto| texto.contains("cuentas_de_agentes"))
        })
        .expect("archivo de preferencias");
    let mut json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&archivo).expect("leer")).expect("json");
    json.as_object_mut()
        .expect("objeto")
        .remove("cuentas_de_agentes");
    std::fs::write(&archivo, json.to_string()).expect("escribir como la 0.15");

    let mut antiguas =
        crate::preferencias::Preferencias::cargar_de(&carpeta).expect("leer las de la 0.15");
    assert!(antiguas.cuentas_de_agentes.is_empty());

    let manual = CuentaManual {
        agente: IdAgente::CodexCli,
        carpeta: carpeta.join("codex-trabajo"),
    };
    antiguas.cuentas_de_agentes.push(manual.clone());
    antiguas
        .guardar_en(&carpeta)
        .expect("guardar con la cuenta");
    let releidas = crate::preferencias::Preferencias::cargar_de(&carpeta).expect("releer");
    assert_eq!(releidas.cuentas_de_agentes, vec![manual]);
    std::fs::remove_dir_all(carpeta).expect("limpiar");
}

/// Añadir una cuenta a mano no la repite, y quitarla la borra sin tocar las demás.
#[test]
fn quitar_una_cuenta_a_mano_la_borra_y_deja_las_demas() {
    let base = std::path::Path::new("cuentas");
    let primera = CuentaManual {
        agente: IdAgente::ClaudeCode,
        carpeta: base.join("primera"),
    };
    let segunda = CuentaManual {
        agente: IdAgente::CodexCli,
        carpeta: base.join("segunda"),
    };
    let misma_carpeta_otro_agente = CuentaManual {
        agente: IdAgente::CodexCli,
        carpeta: primera.carpeta.clone(),
    };
    let mut cuentas = Vec::new();
    for cuenta in [&primera, &segunda, &primera, &misma_carpeta_otro_agente] {
        assert_eq!(
            conectores::anadir_cuenta_manual(&mut cuentas, cuenta.clone(), &[]),
            Ok(())
        );
    }
    assert_eq!(
        cuentas,
        vec![
            primera.clone(),
            segunda.clone(),
            misma_carpeta_otro_agente.clone()
        ]
    );

    conectores::quitar_cuenta_manual(&mut cuentas, IdAgente::ClaudeCode, &primera.carpeta);

    assert_eq!(cuentas, vec![segunda, misma_carpeta_otro_agente]);
}

/// El menú «Añadir cuenta» ofrece las herramientas que la biblioteca deja dar de alta a mano:
/// Claude Code y Codex CLI, no agy, cuya consola no usaría la carpeta añadida.
#[test]
fn el_menu_de_alta_solo_ofrece_las_herramientas_que_la_admiten() {
    let ofrecidas: Vec<IdAgente> = conectores::herramientas_con_cuentas()
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(ofrecidas, vec![IdAgente::ClaudeCode, IdAgente::CodexCli]);
}

/// Una cuenta de agy escrita a mano en las preferencias no llega a las cuentas detectadas.
#[test]
fn una_cuenta_de_agy_en_las_preferencias_no_aparece() {
    let manuales = [
        CuentaManual {
            agente: IdAgente::Agy,
            carpeta: std::path::Path::new("cuentas").join("agy"),
        },
        CuentaManual {
            agente: IdAgente::CodexCli,
            carpeta: std::path::Path::new("cuentas").join("codex"),
        },
    ];
    let cuentas = deteccion_agentes::con_cuentas_manuales(
        Vec::new(),
        &conectores::cuentas_de_la_biblioteca(&manuales),
    );
    let herramientas: Vec<Herramienta> = cuentas.iter().map(|cuenta| cuenta.herramienta).collect();
    assert_eq!(herramientas, vec![Herramienta::CodexCli]);
}

/// El alta de una carpeta que ya es una cuenta detectada de esa herramienta, aunque se escriba
/// con barra final, se rechaza con ese motivo y no se guarda; una carpeta nueva sí se guarda.
#[test]
fn el_alta_de_una_carpeta_ya_usada_se_rechaza_y_no_se_guarda() {
    let usada = Cuenta {
        herramienta: Herramienta::CodexCli,
        carpeta: std::path::Path::new("cuentas").join("una"),
        origen: OrigenDeLaCuenta::Busqueda,
    };
    let detectadas = [usada.clone()];
    let mut carpeta_con_barra = usada.carpeta.clone().into_os_string();
    carpeta_con_barra.push(std::path::MAIN_SEPARATOR_STR);
    let repetida = CuentaManual {
        agente: IdAgente::CodexCli,
        carpeta: carpeta_con_barra.into(),
    };
    let nueva = CuentaManual {
        agente: IdAgente::CodexCli,
        carpeta: std::path::Path::new("cuentas").join("otra"),
    };
    let mut cuentas = Vec::new();

    assert_eq!(
        conectores::anadir_cuenta_manual(&mut cuentas, repetida, &detectadas),
        Err(deteccion_agentes::RechazoDeAlta::CarpetaYaUsada(usada))
    );
    assert_eq!(cuentas, Vec::new());
    assert_eq!(
        conectores::anadir_cuenta_manual(&mut cuentas, nueva.clone(), &detectadas),
        Ok(())
    );
    assert_eq!(cuentas, vec![nueva]);
}

/// El programa elegido a mano para Claude Code se usa en la consola de todas sus cuentas,
/// también en la por defecto, como en la 0.15, que solo tenía esa.
#[test]
fn el_programa_elegido_a_mano_vale_para_todas_las_cuentas() {
    let personal = carpeta_temporal_contexto("ag-programa-elegido");
    for carpeta in [".claude", ".claude2"] {
        std::fs::create_dir_all(personal.join(carpeta)).expect("crear la cuenta");
        std::fs::write(personal.join(carpeta).join("settings.json"), "{}").expect("ajustes");
    }
    let programas = personal.join("programas");
    std::fs::create_dir_all(&programas).expect("crear la carpeta del programa");
    let programa = programas.join(format!("claude{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&programa, "").expect("crear el programa");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&programa, std::fs::Permissions::from_mode(0o755))
            .expect("hacerlo ejecutable");
    }
    let mut elegidos = conectores::RutasDeAgentes::new();
    elegidos.insert(IdAgente::ClaudeCode, programa.clone());

    let cuentas = deteccion_agentes::buscar_cuentas(&personal, |_| None);
    let entorno = conectores::EntornoDeBusqueda {
        personal: &personal,
        datos_locales: None,
        path: OsStr::new(""),
        pathext: OsStr::new(""),
        sistema: conectores::sistema_actual(),
    };
    let agentes = conectores::agentes_de_las_cuentas(&entorno, &elegidos, &cuentas);
    let consolas: Vec<_> = agentes
        .iter()
        .filter(|agente| agente.id == IdAgente::ClaudeCode)
        .map(|agente| {
            (
                agente.cuenta.es_la_por_defecto(),
                agente.cli.as_ref().map(|cli| cli.ejecutable.clone()),
            )
        })
        .collect();

    assert_eq!(
        consolas,
        vec![
            (true, Some(programa.clone())),
            (false, Some(programa.clone()))
        ]
    );
    std::fs::remove_dir_all(personal).expect("limpiar");
}
