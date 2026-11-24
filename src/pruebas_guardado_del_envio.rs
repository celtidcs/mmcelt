//! Lo que se guarda en segundo plano es lo que cuenta como guardado (0.16.0).
//!
//! El envío a un agente guarda una instantánea del mapa en otro hilo. Lo que la persona haga
//! mientras tanto no está en ese archivo y tiene que seguir contando como pendiente.
//!
//! - Un nodo nuevo de la IA abierto durante el guardado sigue contando al cerrar.
//! - Una edición hecha durante el guardado deja el mapa como cambiado.
//! - Sin nada durante el guardado, todo queda guardado y al cerrar no se pregunta.

use super::carpeta_temporal_contexto;
use super::pruebas_vigilancia::{
    encolar_envio_de_un_mapa_sin_archivo, esperar_el_envio_y_su_vigilancia,
};
use crate::cierre_con_cambios::hay_que_preguntar_al_cerrar;
use crate::ui::dialogo_de_cierre::cambios_pendientes_al_cerrar;

/// Título que la persona escribe mientras el envío guarda.
const TITULO_EDITADO_DURANTE_EL_GUARDADO: &str = "Editado mientras se guardaba";

/// Lo que hace la persona mientras el trabajador guarda la instantánea.
enum DuranteElGuardado {
    Nada,
    AbrirElNodoNuevo,
    EditarLaRaiz,
}

/// Envía a un agente un mapa sin archivo cuya raíz es nueva de la IA, hace `accion` mientras el
/// guardado sigue sin recoger y devuelve la aplicación con el envío ya atendido.
fn enviar_y_durante_el_guardado(
    nombre: &str,
    accion: DuranteElGuardado,
) -> (crate::AplicacionMapaMental, std::path::PathBuf) {
    let (mut app, ctx) = crate::arnes_interfaz::aplicacion_de_prueba();
    let raiz = carpeta_temporal_contexto(nombre);
    let proyecto = app.mapa_mut().proyecto_para_editar();
    let id_raiz = proyecto.root_id;
    proyecto
        .nodes
        .get_mut(&id_raiz)
        .expect("raíz")
        .nuevo_de_la_ia = true;
    proyecto.marcar_modificado();

    let ruta = encolar_envio_de_un_mapa_sin_archivo(&mut app, &raiz);
    match accion {
        DuranteElGuardado::Nada => {}
        DuranteElGuardado::AbrirElNodoNuevo => app.mapa_mut().seleccionar_nodo(Some(id_raiz)),
        DuranteElGuardado::EditarLaRaiz => {
            let proyecto = app.mapa_mut().proyecto_para_editar();
            proyecto.nodes.get_mut(&id_raiz).expect("raíz").title =
                TITULO_EDITADO_DURANTE_EL_GUARDADO.to_owned();
            proyecto.marcar_modificado();
        }
    }
    esperar_el_envio_y_su_vigilancia(&mut app, &ctx, &ruta);
    (app, raiz)
}

/// El resalte quitado durante el guardado no está en el archivo.
#[test]
fn abrir_un_nodo_nuevo_mientras_se_guarda_sigue_pendiente() {
    let (app, raiz) =
        enviar_y_durante_el_guardado("d1a-vistos", DuranteElGuardado::AbrirElNodoNuevo);
    let cambios = cambios_pendientes_al_cerrar(&app);
    assert!(cambios.vistos_sin_guardar, "{cambios:?}");
    assert!(hay_que_preguntar_al_cerrar(cambios));
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// La edición hecha durante el guardado no está en el archivo.
#[test]
fn editar_mientras_se_guarda_deja_el_mapa_cambiado() {
    let (app, raiz) = enviar_y_durante_el_guardado("d1b-edicion", DuranteElGuardado::EditarLaRaiz);
    let cambios = cambios_pendientes_al_cerrar(&app);
    assert!(cambios.mapa_cambiado, "{cambios:?}");
    assert!(hay_que_preguntar_al_cerrar(cambios));
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}

/// Sin nada durante el guardado, lo guardado es todo lo que hay.
#[test]
fn sin_cambios_durante_el_guardado_no_queda_nada_pendiente() {
    let (app, raiz) = enviar_y_durante_el_guardado("d1c-nada", DuranteElGuardado::Nada);
    let cambios = cambios_pendientes_al_cerrar(&app);
    assert!(!hay_que_preguntar_al_cerrar(cambios), "{cambios:?}");
    drop(app);
    std::fs::remove_dir_all(raiz).expect("limpiar");
}
