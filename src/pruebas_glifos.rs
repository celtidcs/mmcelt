//! Glifos de la interfaz (0.15.0): todo carácter que MMCelt muestra tiene dibujo en las fuentes
//! que lleva dentro el ejecutable.
//!
//! MMCelt no usa las fuentes del sistema: las de `egui` y las de respaldo van compiladas en el
//! programa. Un carácter que no está en ellas sale como un recuadro vacío en cualquier
//! ordenador, sea cual sea su sistema. Esta prueba lee la tabla de caracteres de cada una de
//! esas fuentes y comprueba que cubren todo lo que pueden mostrar el programa y la ayuda.

use super::extraer_glifos_cmap_ttf;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

/// Familia con la que se dibujan los textos de la interfaz y de la ayuda.
const FAMILIA_DE_LA_INTERFAZ: egui::FontFamily = egui::FontFamily::Proportional;

/// Sitios que se citan de cada carácter sin dibujo, para no inundar el mensaje.
const SITIOS_POR_CARACTER: usize = 3;

/// Archivos de código que no muestran nada a la persona: sus textos son datos de prueba.
fn es_archivo_de_pruebas(ruta: &Path) -> bool {
    let nombre = ruta
        .file_name()
        .and_then(|nombre| nombre.to_str())
        .unwrap_or_default();
    nombre == "audit_checks.rs"
        || nombre.starts_with("pruebas_")
        || ruta.components().any(|parte| parte.as_os_str() == "tests")
}

/// Todos los archivos con la extensión dada bajo `carpeta`, en orden estable.
fn archivos_con_extension(carpeta: &Path, extension: &str) -> Vec<PathBuf> {
    let mut pendientes = vec![carpeta.to_path_buf()];
    let mut encontrados = Vec::new();
    while let Some(actual) = pendientes.pop() {
        let entradas = std::fs::read_dir(&actual).expect("la carpeta se lee");
        for entrada in entradas {
            let ruta = entrada.expect("la entrada se lee").path();
            if ruta.is_dir() {
                pendientes.push(ruta);
            } else if ruta.extension().is_some_and(|ext| ext == extension) {
                encontrados.push(ruta);
            }
        }
    }
    encontrados.sort();
    encontrados
}

/// Líneas que pueden llegar a la pantalla: en el código, todas salvo los comentarios de línea
/// entera; en la ayuda, todas.
fn lineas_visibles(ruta: &Path) -> Vec<(usize, String)> {
    let texto = std::fs::read_to_string(ruta).expect("el archivo se lee en UTF-8");
    let es_codigo = ruta.extension().is_some_and(|ext| ext == "rs");
    texto
        .lines()
        .enumerate()
        .filter(|(_, linea)| !(es_codigo && linea.trim_start().starts_with("//")))
        .map(|(indice, linea)| (indice + 1, linea.to_owned()))
        .collect()
}

/// Caracteres que `egui` nunca dibuja porque los trata como invisibles o como espacios
/// (`epaint`, funciones `invisible_char` y `glyph_id_resolution`): no necesitan glifo.
fn es_invisible_para_egui(caracter: char) -> bool {
    matches!(
        caracter,
        '\t' | '\r'
            | '\u{2009}'
            | '\u{202F}'
            | '\u{200B}'..='\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'..='\u{206F}'
            | '\u{FEFF}'
    )
}

/// Caracteres que tienen dibujo en alguna fuente de la familia de la interfaz.
///
/// Se lee la tabla `cmap` de cada fuente en lugar de preguntar a `egui`: su `has_glyph` da por
/// ausente todo lo que cae en la fuente que guarda el recuadro de sustitución, que es una de
/// emojis, y entonces ⏳ o ⚠ parecerían faltar sin faltar.
fn caracteres_con_dibujo() -> HashSet<char> {
    let fuentes = crate::aplicacion::definiciones_de_fuentes();
    let familia = fuentes
        .families
        .get(&FAMILIA_DE_LA_INTERFAZ)
        .expect("la familia de la interfaz existe");
    let mut caracteres = HashSet::new();
    for nombre in familia {
        let datos = fuentes
            .font_data
            .get(nombre)
            .expect("la fuente está registrada");
        caracteres.extend(extraer_glifos_cmap_ttf((*datos.font).as_ref()));
    }
    caracteres
}

/// Busca los caracteres sin dibujo en las fuentes de MMCelt y dice dónde aparecen.
fn caracteres_sin_dibujo(archivos: &[PathBuf]) -> BTreeMap<char, Vec<String>> {
    let con_dibujo = caracteres_con_dibujo();
    let mut faltan: BTreeMap<char, Vec<String>> = BTreeMap::new();
    for ruta in archivos {
        for (numero, linea) in lineas_visibles(ruta) {
            for caracter in linea.chars() {
                if !con_dibujo.contains(&caracter) && !es_invisible_para_egui(caracter) {
                    faltan
                        .entry(caracter)
                        .or_default()
                        .push(format!("{}:{numero}", ruta.display()));
                }
            }
        }
    }
    faltan
}

/// Ningún texto del programa ni de la ayuda usa un carácter que se dibuje como un recuadro.
#[test]
fn todo_lo_que_se_muestra_tiene_dibujo_en_las_fuentes_del_programa() {
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut archivos: Vec<PathBuf> = archivos_con_extension(&raiz.join("src"), "rs")
        .into_iter()
        .filter(|ruta| !es_archivo_de_pruebas(ruta))
        .collect();
    archivos.extend(archivos_con_extension(&raiz.join("assets/ayuda"), "md"));
    assert!(archivos.len() > 100, "solo {} archivos", archivos.len());

    let faltan = caracteres_sin_dibujo(&archivos);
    let informe: Vec<String> = faltan
        .iter()
        .map(|(caracter, sitios)| {
            format!(
                "U+{:04X} «{caracter}» en {} sitios, p. ej. {}",
                u32::from(*caracter),
                sitios.len(),
                sitios
                    .iter()
                    .take(SITIOS_POR_CARACTER)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect();
    assert!(
        faltan.is_empty(),
        "caracteres sin dibujo:\n{}",
        informe.join("\n")
    );
}

/// Las fuentes de respaldo no cambian cómo se dibujan el latín ni el cirílico: van detrás de las
/// de `egui` en las dos familias.
#[test]
fn las_fuentes_de_respaldo_van_detras_de_las_de_egui() {
    let propias = crate::aplicacion::definiciones_de_fuentes();
    let de_egui = egui::FontDefinitions::default();
    for familia in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        let originales = &de_egui.families[&familia];
        let actuales = &propias.families[&familia];
        assert_eq!(
            &actuales[..originales.len()],
            originales.as_slice(),
            "{familia:?}"
        );
        assert!(actuales.len() > originales.len(), "{familia:?}");
    }
}
