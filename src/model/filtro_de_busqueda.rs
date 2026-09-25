//! # Lo que se le pide al buscador (`model/filtro_de_busqueda.rs`)
//!
//! Aquí vive **qué se busca**, no **cómo se busca**. La diferencia no es de estilo: el filtro
//! está escrito enteramente con vocabulario del dominio —un estado, una prioridad, un texto—
//! y no sabe nada de recorrer el mapa, que es trabajo de [`crate::busqueda`].
//!
//! Por eso está en `model/` y no junto al algoritmo. Una barrera lo dijo antes que nadie:
//! poner el tipo en `busqueda.rs` metía dos asuntos en un módulo.

use super::{EstadoNodo, PrioridadNodo};

/// En qué orden salen los resultados de una búsqueda.
///
/// Se pide explícitamente porque los dos órdenes sirven a preguntas distintas y ninguno es
/// mejor que el otro.
///
/// **No es un booleano** a propósito: un `bool` en la firma serían dos funciones metidas en
/// una, y en la llamada no se leería cuál de las dos se está pidiendo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrdenDeResultados {
    /// Como se leen en el mapa: de la raíz hacia abajo, rama por rama.
    ///
    /// Es el de siempre y el de por defecto. Quien escribe en el buscador está pensando en un
    /// nodo que ya tiene en la cabeza, y lo reconoce por dónde estaba.
    #[default]
    ComoEnElMapa,
    /// De crítica a baja y, a igualdad de prioridad, por título.
    ///
    /// Es el de quien pide una lista de trabajo. Lo del título no es un adorno: sin ese
    /// segundo criterio el empate lo desharía el recorrido del mapa, y la misma búsqueda
    /// podría dar dos listas distintas.
    PorPrioridad,
}

/// Lo que se le pide al buscador: texto, estado, prioridad y orden.
///
/// Los tres criterios se combinan con **«y»**: un nodo entra si cumple todos los que estén
/// puestos. Cada uno es opcional, y `None` significa «cualquiera», no «ninguno».
///
/// Es **un solo valor** y no tres sueltos porque son una sola cosa: lo que se está pidiendo
/// ahora mismo. Tres variables separadas no obligan a nadie a mantenerlas coherentes.
///
/// **No admite elegir varios estados o varias prioridades a la vez** a propósito: nadie lo ha
/// pedido, y construir esa abstracción por si acaso complica la interfaz y el tipo sin que
/// exista hoy un caso que la necesite.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FiltroDeBusqueda {
    /// Lo que ha escrito el usuario. Los espacios de los extremos se ignoran.
    pub texto: String,
    /// Estado exigido, o `None` para no filtrar por estado.
    pub estado: Option<EstadoNodo>,
    /// Prioridad exigida, o `None` para no filtrar por prioridad.
    pub prioridad: Option<PrioridadNodo>,
    /// En qué orden se quieren los resultados.
    pub orden: OrdenDeResultados,
}

impl FiltroDeBusqueda {
    /// Un filtro que solo busca texto, con el orden de siempre.
    ///
    /// Es la búsqueda de toda la vida, la que hace quien teclea en el campo y no toca los
    /// desplegables. Existe para que las barreras no tengan que enumerar los campos que **no**
    /// están filtrando, que es donde se cuelan los descuidos al leerlas.
    ///
    /// **Solo se compila en las pruebas**, y se declara así en vez de dejarlo público porque en
    /// producción nadie lo llama: el panel modifica el filtro que ya tiene guardado, no construye
    /// uno nuevo en cada fotograma. Un constructor sin usuarios en producción es código muerto, y
    /// el programa lo rechaza al compilar.
    ///
    /// # Parámetros
    /// - `texto`: lo que se busca en títulos y etiquetas.
    ///
    /// # Devuelve
    /// Un filtro con ese texto, sin filtrar por estado ni por prioridad, y en el orden del mapa.
    #[cfg(test)]
    pub fn por_texto(texto: &str) -> Self {
        Self {
            texto: texto.to_string(),
            ..Self::default()
        }
    }

    /// Si no se ha pedido nada todavía.
    ///
    /// El orden no cuenta: elegir cómo ordenar una lista que no se ha pedido no es pedirla.
    ///
    /// # Devuelve
    /// `true` si no hay texto —ni siquiera espacios— ni estado ni prioridad.
    pub fn sin_usar(&self) -> bool {
        self.texto.trim().is_empty() && self.estado.is_none() && self.prioridad.is_none()
    }
}
