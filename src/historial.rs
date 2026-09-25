//! # Historial de cambios (`historial.rs`)
//!
//! Deshacer y rehacer. Guarda **instantáneas del proyecto entero**, no órdenes invertibles.
//!
//! ## Por qué instantáneas y no comandos
//!
//! La alternativa clásica es registrar cada operación con su inversa —«se añadió el nodo X,
//! para deshacer bórralo»— y eso ocupa mucho menos. Pero obliga a escribir, y a mantener
//! correcta, una inversa por cada operación que exista; el día que alguien añada una nueva y
//! olvide su inversa, deshacer deja el mapa en un estado que no existió nunca. En un programa
//! cuyo cometido es **no perder el trabajo del usuario**, ese modo de fallo no compensa.
//!
//! Una instantánea no puede equivocarse: es el mapa tal cual estaba.
//!
//! ## Cómo se entera de que algo ha cambiado
//!
//! Cada mutación significativa incrementa una revisión interna del proyecto. Al final del
//! fotograma, el historial compara esa revisión con la última observada. Si son iguales,
//! termina en tiempo constante: no recorre ni clona el mapa. Si difieren, conserva la
//! instantánea anterior.
//!
//! Las pruebas de bajo nivel conservan una entrada basada en [`calcular_huella`] para poder
//! construir estados artificiales con campos públicos. La aplicación real utiliza siempre
//! la revisión incremental.
//!
//! Se hereda además una decisión ya tomada y probada: esa huella incluye las posiciones de
//! los nodos **solo en «Posición Libre Manual»**, porque en los modos automáticos las
//! reescribe la disposición sola y el historial se llenaría de pasos que el usuario no dio.
//!
//! ## Por qué los cambios seguidos cuentan como uno
//!
//! Arrastrar un nodo cambia el mapa en cada fotograma. Sin agrupar, un arrastre de dos
//! segundos dejaría más de cien pasos y deshacerlo exigiría cien pulsaciones: deshacer sería
//! inservible justo cuando más se necesita. Los cambios separados por menos de
//! [`AGRUPACION`] se funden en un solo paso, así que un arrastre completo —o una ráfaga de
//! tecleo— se deshace de una vez, que es lo que espera quien lo hizo.

use crate::autoguardado::calcular_huella;
use crate::model::{Proyecto, RevisionProyecto};
use std::time::{Duration, Instant};

/// Cuántos pasos atrás se pueden deshacer.
///
/// Cada paso es una copia del mapa entero, así que esto acota lo que ocupa el historial.
/// Cincuenta cubre de sobra el arrepentimiento real —lo que uno recuerda haber hecho— sin
/// que un mapa grande multiplique la memoria del programa sin freno.
pub const PASOS_MAXIMOS: usize = 50;

/// Ventana dentro de la cual dos cambios cuentan como uno solo.
///
/// Medio segundo separa bien lo continuo de lo deliberado: un arrastre o una ráfaga de
/// tecleo caen dentro; dos acciones distintas del usuario, no.
pub const AGRUPACION: Duration = Duration::from_millis(500);

/// Los estados por los que ha pasado el mapa, y por los que puede volver a pasar.
///
/// Se usa desde la aplicación: [`Self::observar_si_cambio`] en cada fotograma, y
/// [`Self::deshacer`] o [`Self::rehacer`] cuando el usuario lo pide desde el menú o con el atajo.
pub struct HistorialDeCambios {
    /// Estados anteriores, del más antiguo al más reciente. Deshacer saca del final.
    pasado: Vec<Proyecto>,

    /// Estados que se deshicieron y pueden rehacerse. Cualquier cambio nuevo los invalida.
    futuro: Vec<Proyecto>,

    /// El mapa tal como estaba la última vez que se miró.
    ///
    /// Es `None` hasta el primer vistazo: un historial recién creado no tiene con qué
    /// comparar, y su primera observación solo sirve para tomar la referencia.
    ultimo_visto: Option<Proyecto>,

    /// Huella de [`Self::ultimo_visto`], para no comparar mapas enteros en cada fotograma.
    huella_vista: u64,

    /// Revisión observada por la ruta incremental que usa la interfaz.
    revision_vista: Option<RevisionProyecto>,

    /// Cuándo se detectó el último cambio, para poder agrupar los que van seguidos.
    momento_ultimo_cambio: Option<Instant>,
}

impl Default for HistorialDeCambios {
    /// El mismo historial vacío que devuelve [`HistorialDeCambios::nuevo`].
    ///
    /// Un historial recién creado no tiene nada que deshacer ni que rehacer, que es
    /// exactamente lo que se espera por omisión.
    fn default() -> Self {
        Self::nuevo()
    }
}

impl HistorialDeCambios {
    /// Crea un historial vacío, sin nada que deshacer ni rehacer.
    pub fn nuevo() -> Self {
        Self {
            pasado: Vec::new(),
            futuro: Vec::new(),
            ultimo_visto: None,
            huella_vista: 0,
            revision_vista: None,
            momento_ultimo_cambio: None,
        }
    }

    /// Mira el proyecto y anota el estado anterior si ha cambiado.
    ///
    /// Se llama en cada fotograma. La primera vez solo toma la referencia: no hay estado
    /// anterior que guardar.
    ///
    /// # Parámetros
    /// - `proyecto`: el mapa tal como está ahora.
    /// - `ahora`: el momento actual. Se pasa por parámetro, en lugar de leerlo aquí, para
    ///   que la agrupación pueda comprobarse en una prueba sin esperas reales.
    #[cfg(test)]
    pub fn observar(&mut self, proyecto: &Proyecto, ahora: Instant) {
        let huella = calcular_huella(proyecto);

        // Primer vistazo: no hay con qué comparar todavía.
        let Some(anterior) = self.ultimo_visto.as_ref() else {
            self.ultimo_visto = Some(proyecto.clone());
            self.huella_vista = huella;
            self.revision_vista = Some(proyecto.revision());
            return;
        };

        if huella == self.huella_vista {
            return;
        }

        // Un cambio nuevo invalida lo que se hubiera deshecho: ese futuro ya no ocurrió.
        self.futuro.clear();

        let va_seguido_del_anterior = self
            .momento_ultimo_cambio
            .is_some_and(|ultimo| ahora.duration_since(ultimo) < AGRUPACION);

        if !va_seguido_del_anterior {
            self.pasado.push(anterior.clone());

            // El más antiguo se cae al llegar al tope. Se quita por delante para conservar
            // siempre los últimos pasos, que son los que alguien va a querer deshacer.
            if self.pasado.len() > PASOS_MAXIMOS {
                self.pasado.remove(0);
            }
        }

        self.ultimo_visto = Some(proyecto.clone());
        self.huella_vista = huella;
        self.revision_vista = Some(proyecto.revision());
        self.momento_ultimo_cambio = Some(ahora);
    }

    /// Observa el proyecto usando únicamente su revisión para descartar el caso habitual.
    ///
    /// Esta es la ruta de producción del fotograma: si la revisión no ha cambiado, vuelve sin
    /// calcular una huella ni clonar el mapa. Las pruebas conservan una variante basada en la
    /// huella completa para construir estados artificiales sin pasar por las mutaciones normales.
    pub fn observar_si_cambio(
        &mut self,
        proyecto: &Proyecto,
        revision: RevisionProyecto,
        ahora: Instant,
    ) {
        if self.revision_vista == Some(revision) {
            return;
        }

        let Some(anterior) = self.ultimo_visto.as_ref() else {
            self.ultimo_visto = Some(proyecto.clone());
            self.revision_vista = Some(revision);
            return;
        };

        self.futuro.clear();
        let va_seguido = self
            .momento_ultimo_cambio
            .is_some_and(|ultimo| ahora.duration_since(ultimo) < AGRUPACION);
        if !va_seguido {
            self.pasado.push(anterior.clone());
            if self.pasado.len() > PASOS_MAXIMOS {
                self.pasado.remove(0);
            }
        }

        self.ultimo_visto = Some(proyecto.clone());
        self.revision_vista = Some(revision);
        self.momento_ultimo_cambio = Some(ahora);
    }

    /// Si hay algún paso al que volver.
    pub fn puede_deshacer(&self) -> bool {
        !self.pasado.is_empty()
    }

    /// Si hay algún paso deshecho que pueda recuperarse.
    pub fn puede_rehacer(&self) -> bool {
        !self.futuro.is_empty()
    }

    /// Devuelve el estado anterior del mapa, y guarda el actual para poder rehacerlo.
    ///
    /// # Parámetros
    /// - `actual`: el mapa tal como está ahora, que pasa a ser el futuro.
    ///
    /// # Devuelve
    /// El mapa anterior, o `None` si no hay nada que deshacer.
    pub fn deshacer(&mut self, actual: &Proyecto) -> Option<Proyecto> {
        let anterior = self.pasado.pop()?;
        self.futuro.push(actual.clone());
        self.fijar_referencia(&anterior);
        Some(anterior)
    }

    /// Devuelve el estado que se había deshecho, y guarda el actual para poder deshacerlo.
    ///
    /// # Parámetros
    /// - `actual`: el mapa tal como está ahora, que vuelve al pasado.
    ///
    /// # Devuelve
    /// El mapa rehecho, o `None` si no hay nada que rehacer.
    pub fn rehacer(&mut self, actual: &Proyecto) -> Option<Proyecto> {
        let siguiente = self.futuro.pop()?;
        self.pasado.push(actual.clone());
        self.fijar_referencia(&siguiente);
        Some(siguiente)
    }

    /// Vacía el historial y toma como referencia el mapa indicado.
    ///
    /// Se llama al abrir un archivo o empezar un mapa nuevo: los pasos del mapa anterior no
    /// tienen sentido en este, y deshacer sobre ellos traería de vuelta un mapa que no es el
    /// que hay abierto.
    ///
    /// # Parámetros
    /// - `proyecto`: el mapa que acaba de abrirse.
    pub fn olvidar(&mut self, proyecto: &Proyecto) {
        self.pasado.clear();
        self.futuro.clear();
        self.fijar_referencia(proyecto);
    }

    /// Toma un mapa como referencia para las comparaciones siguientes.
    ///
    /// Es lo que impide que el propio deshacer se registre como un cambio del usuario: tras
    /// volver atrás, el siguiente vistazo tiene que ver el mapa que ya esperaba. Y borra el
    /// momento del último cambio, para que lo que el usuario haga después empiece un paso
    /// nuevo en vez de agruparse con lo anterior.
    ///
    /// # Parámetros
    /// - `proyecto`: el mapa que pasa a ser la referencia.
    fn fijar_referencia(&mut self, proyecto: &Proyecto) {
        self.huella_vista = calcular_huella(proyecto);
        self.revision_vista = Some(proyecto.revision());
        self.ultimo_visto = Some(proyecto.clone());
        self.momento_ultimo_cambio = None;
    }
}
