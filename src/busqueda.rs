//! # Búsqueda de nodos (`busqueda.rs`)
//!
//! Encuentra nodos por su título, por sus etiquetas, por su estado o por su prioridad. En un
//! mapa de cien nodos, dar con uno a ojo obliga a recorrer ramas plegadas y a mover la cámara
//! sin saber hacia dónde.
//!
//! ## Qué se mira y qué no
//!
//! Del texto se buscan el **título** y las **etiquetas**, que es lo que identifica a un nodo.
//! Las notas quedan fuera a propósito: son párrafos largos, y buscar dentro de ellos
//! devolvería medio mapa para cualquier palabra común, que es no devolver nada útil.
//!
//! ## Se busca como se escribe, no como se acentúa
//!
//! La comparación ignora mayúsculas y tildes: quien escriba `diseno` con el teclado a medio
//! camino encuentra igual el nodo «Diseño». Es lo que hace cualquier buscador y lo que la
//! gente da por hecho; exigir la tilde exacta convierte la búsqueda en un examen de ortografía.
//!
//! ## Buscar sin escribir nada
//!
//! Se puede pedir «las críticas que están en progreso» sin teclear una palabra, porque esa
//! petición no tiene ninguna. Por eso el filtro admite **texto, estado y prioridad por
//! separado**, y basta con que uno de los tres esté puesto. Lo que no devuelve nada es el
//! filtro **entero vacío**: ahí no es que no haya resultados, es que todavía no se ha buscado.
//!
//! ## El orden se pide, no se adivina
//!
//! Hay dos órdenes y **quien busca dice cuál quiere**, con [`crate::model::OrdenDeResultados`]:
//!
//! - [`crate::model::OrdenDeResultados::ComoEnElMapa`] es el de siempre y el de por defecto. Los resultados
//!   salen en el orden en que se leen —de la raíz hacia abajo, rama por rama—, que es lo que
//!   permite reconocerlos: quien mira la lista está pensando en dónde estaba el nodo.
//! - [`crate::model::OrdenDeResultados::PorPrioridad`] saca primero lo crítico y, a igualdad de prioridad,
//!   ordena por título. Lo segundo no es un adorno: sin ello el empate lo desharía el recorrido
//!   del mapa, y la misma búsqueda podría dar dos listas distintas.
//!
//! No es un booleano por un motivo concreto: un `bool` en la firma serían dos funciones
//! metidas en una, y en la llamada no se leería cuál de las dos se está pidiendo.
//!
//! ## El recorrido no es recursivo
//!
//! Va con pila propia y registro de visitados, como el resto de recorridos del programa. La
//! razón está en `arquitectura.md` y es concreta: un archivo con un ciclo entre nodos mataría
//! una función recursiva, y los archivos que hay que rechazar son justamente esos.

use crate::model::{FiltroDeBusqueda, OrdenDeResultados, PrioridadNodo, Proyecto};
use uuid::Uuid;

/// Cuántos nodos se devuelven como mucho.
///
/// Una lista más larga que la pantalla no ayuda a elegir: quien tenga tantos aciertos lo que
/// necesita es afinar la búsqueda, no recorrer doscientas filas.
pub const RESULTADOS_MAXIMOS: usize = 50;

/// Deja un texto en la forma con la que se comparan búsquedas: minúsculas y sin tildes.
///
/// Se traducen los diacríticos que aparecen en los seis idiomas de la interfaz escritos con
/// alfabeto latino. El ruso y el chino no los usan, y sus caracteres pasan intactos.
///
/// La `ñ` se convierte en `n`. Es deliberado: en una búsqueda vale más encontrar «año» al
/// teclear `ano` que obligar a colocar la tilde de la eñe.
///
/// # Parámetros
/// - `texto`: el texto a normalizar.
///
/// # Devuelve
/// El mismo texto en minúsculas y sin diacríticos latinos.
pub fn normalizar(texto: &str) -> String {
    texto
        .chars()
        .flat_map(|caracter| caracter.to_lowercase())
        .map(|minuscula| match minuscula {
            'á' | 'à' | 'ä' | 'â' | 'ã' | 'å' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            'ý' | 'ÿ' => 'y',
            otro => otro,
        })
        .collect()
}

/// Busca los nodos que cumplen todo lo que pide el filtro.
///
/// # Parámetros
/// - `proyecto`: el mapa donde buscar.
/// - `filtro`: qué se busca y en qué orden se quiere.
///
/// # Devuelve
/// Los identificadores de los nodos que cumplen el filtro, como mucho [`RESULTADOS_MAXIMOS`].
/// Un filtro sin usar devuelve la lista vacía: no es una búsqueda de todo, es que todavía no
/// se ha buscado nada.
///
/// El recorte a [`RESULTADOS_MAXIMOS`] se aplica **después** de ordenar. Al revés, con orden
/// por prioridad, el corte lo decidiría el orden de llegada y podría dejar fuera justo lo
/// crítico, que es lo contrario de lo que se ha pedido.
pub fn buscar_nodos(proyecto: &Proyecto, filtro: &FiltroDeBusqueda) -> Vec<Uuid> {
    if filtro.sin_usar() {
        return Vec::new();
    }

    let aguja = normalizar(filtro.texto.trim());
    let mut encontrados: Vec<(Uuid, PrioridadNodo, String)> = Vec::new();
    let mut visitados = std::collections::HashSet::new();
    let mut pendientes = vec![proyecto.root_id];

    while let Some(id) = pendientes.pop() {
        // Un archivo con un ciclo entre nodos dejaría este bucle sin final. No se confía en
        // que la validación de entrada lo haya impedido: aquí se comprueba otra vez, porque
        // colgar el programa buscando es peor que no encontrar.
        if !visitados.insert(id) {
            continue;
        }

        let Some(nodo) = proyecto.nodes.get(&id) else {
            continue;
        };

        if coincide(nodo, &aguja, filtro) {
            encontrados.push((id, nodo.priority, normalizar(&nodo.title)));
        }

        // En orden inverso porque la pila los devuelve del revés: así los hermanos salen en
        // el mismo orden en que están dibujados.
        for hijo in nodo.children.iter().rev() {
            pendientes.push(*hijo);
        }
    }

    if filtro.orden == OrdenDeResultados::PorPrioridad {
        // `sort_by` es estable, así que los empates que el criterio no deshaga conservan el
        // orden del mapa. El título se compara ya normalizado para que «Álvaro» no acabe
        // detrás de «Zulema» por culpa del acento.
        encontrados.sort_by(|uno, otro| {
            otro.1
                .cmp(&uno.1)
                .then_with(|| uno.2.cmp(&otro.2))
                .then_with(|| uno.0.cmp(&otro.0))
        });
    }

    encontrados.truncate(RESULTADOS_MAXIMOS);
    encontrados.into_iter().map(|(id, _, _)| id).collect()
}

/// Decide si un nodo cumple todo lo que pide el filtro.
///
/// # Parámetros
/// - `nodo`: el nodo a comprobar.
/// - `aguja`: el texto del filtro **ya normalizado y sin espacios de los extremos**. Vacío
///   significa que no se filtra por texto.
/// - `filtro`: de donde salen el estado y la prioridad exigidos.
///
/// # Devuelve
/// `true` si el nodo cumple los tres criterios, contando como cumplidos los que no se pidieron.
fn coincide(nodo: &crate::model::Nodo, aguja: &str, filtro: &FiltroDeBusqueda) -> bool {
    if filtro.estado.is_some_and(|estado| nodo.status != estado) {
        return false;
    }

    if filtro
        .prioridad
        .is_some_and(|prioridad| nodo.priority != prioridad)
    {
        return false;
    }

    if aguja.is_empty() {
        return true;
    }

    if normalizar(&nodo.title).contains(aguja) {
        return true;
    }

    nodo.tags
        .iter()
        .any(|etiqueta| normalizar(etiqueta).contains(aguja))
}
