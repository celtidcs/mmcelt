//! # Orquestación de la exportación e importación `.mm` (`exportacion_freemind_comando.rs`)
//!
//! Enlaza el diálogo de archivo, `crate::exportacion_freemind` (que solo sabe de XML) y el
//! estado de la aplicación. Vive fuera de `impl AplicacionMapaMental` a propósito, igual que
//! `exportacion_opml_comando.rs`: ver el motivo detallado ahí (A-3, P6, P7).

use crate::textos::Texto;
use crate::AplicacionMapaMental;

/// Exporta el mapa activo a un documento `.mm` de FreeMind/Freeplane (tanda 5, C21-B).
///
/// Los campos de MMCelt sin equivalente nativo en `.mm` viajan como JSON dentro de la nota de
/// cada nodo, según decide el módulo interno de metadatos externos del dominio. Nada se pierde.
pub fn exportar(app: &mut AplicacionMapaMental) {
    let idioma = app.idioma();
    let nombre_propuesto = format!("{}.mm", app.mapa().proyecto().title);
    let Some(ruta) =
        crate::ui::dialogos::pedir_archivo_freemind_para_exportar(&nombre_propuesto, idioma)
    else {
        return;
    };

    let xml = crate::exportacion_freemind::exportar_a_freemind(app.mapa().proyecto());
    match app.guardar_documento_auxiliar(&ruta, xml.as_bytes()) {
        Ok(()) => {
            let aviso = Texto::AvisoFreemindExportado
                .en(idioma)
                .replace("{}", &ruta.display().to_string());
            app.establecer_estado(aviso);
        }
        Err(e) => app.reportar_error(&e, Texto::ContextoExportarFreemind.en(idioma)),
    }
}

/// Importa un mapa desde un documento `.mm` (tanda 5, C21-B/C21-C/C21-D).
///
/// Si el archivo lleva el bloque JSON de MMCelt, reconstruye estado, prioridad, rol, revisión,
/// comentario, ruta, etiquetas, posición, plegado y conexiones cruzadas exactos. Si no —un
/// archivo genuinamente ajeno—, los nodos entran con valores por defecto y el aviso lo dice
/// explícitamente.
pub fn importar(app: &mut AplicacionMapaMental) {
    let idioma = app.idioma();
    let Some(ruta) = crate::ui::dialogos::pedir_archivo_freemind_para_importar(idioma) else {
        return;
    };

    let contenido = match crate::storage::leer_bytes(&ruta) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(e) => {
            app.reportar_error(&e, Texto::ContextoImportarFreemind.en(idioma));
            return;
        }
    };

    match crate::exportacion_freemind::importar_desde_freemind(&contenido) {
        Ok(importado) => {
            let ruta_texto = ruta.display().to_string();
            let (preparado, aviso) = preparar_mapa_importado(importado, idioma, &ruta_texto);
            app.sustituir_el_mapa_abierto(preparado.proyecto, None, &aviso);
        }
        Err(_) => {
            app.establecer_estado(Texto::AvisoFreemindNoSePudoInterpretar.en(idioma));
        }
    }
}

/// Prepara el proyecto reconstruido y el aviso tras un `importar_desde_freemind` (PH-0925-2).
///
/// Separada de [`importar`] para poder comprobarla sin diálogos de archivo: un documento ajeno no
/// trae posiciones de MMCelt, así que todos sus nodos nacen amontonados en el origen si no se
/// aplica la disposición automática. Un documento propio ya trae posiciones exactas y no se toca.
fn preparar_mapa_importado(
    mut importado: crate::exportacion_freemind::ProyectoImportadoFreemind,
    idioma: crate::textos::Idioma,
    ruta_texto: &str,
) -> (
    crate::exportacion_freemind::ProyectoImportadoFreemind,
    String,
) {
    let aviso = if importado.trae_metadatos_mmcelt {
        Texto::AvisoFreemindImportado
            .en(idioma)
            .replace("{}", ruta_texto)
    } else {
        crate::layout::aplicar_disposicion_automatica(&mut importado.proyecto);
        Texto::AvisoFreemindImportadoSinMetadatos
            .en(idioma)
            .replace("{}", ruta_texto)
    };
    (importado, aviso)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::textos::Idioma;

    /// **PH-0925-2** — un `.mm` ajeno amontona todos los nodos en el origen; el comando debe
    /// dejarlos en posiciones distintas llamando a la disposición automática.
    #[test]
    fn preparar_mapa_importado_dispone_un_mm_ajeno() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<map version="1.0.1">
  <node TEXT="Idea central" ID="ID_1">
    <node TEXT="Rama uno" ID="ID_2"/>
    <node TEXT="Rama dos" ID="ID_3"/>
    <node TEXT="Rama tres" ID="ID_4"/>
  </node>
</map>"#;
        let importado = crate::exportacion_freemind::importar_desde_freemind(xml).unwrap();
        assert!(!importado.trae_metadatos_mmcelt);

        let (preparado, _aviso) = preparar_mapa_importado(importado, Idioma::Espanol, "x.mm");

        let mut posiciones: Vec<[f32; 2]> =
            preparado.proyecto.nodes.values().map(|n| n.pos).collect();
        posiciones.dedup();
        assert!(
            posiciones.len() > 1,
            "el comando debe separar los nodos amontonados de un .mm ajeno"
        );
    }

    /// **PH-0925-2, lado positivo** — un `.mm` propio de MMCelt no se toca: sus posiciones deben
    /// sobrevivir exactas.
    #[test]
    fn preparar_mapa_importado_no_toca_un_mm_propio() {
        let mut original = crate::model::Proyecto::nuevo_vacio("Con posiciones");
        let raiz_id = original.root_id;
        let hijo_id = original.anadir_hijo(raiz_id, "Hijo");
        original.nodes.get_mut(&hijo_id).unwrap().pos = [77.0, -33.0];

        let xml = crate::exportacion_freemind::exportar_a_freemind(&original);
        let importado = crate::exportacion_freemind::importar_desde_freemind(&xml).unwrap();
        assert!(importado.trae_metadatos_mmcelt);

        let (preparado, _aviso) = preparar_mapa_importado(importado, Idioma::Espanol, "x.mm");

        for (id, nodo_original) in &original.nodes {
            assert_eq!(
                preparado.proyecto.nodes.get(id).unwrap().pos,
                nodo_original.pos
            );
        }
    }
}
