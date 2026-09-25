//! # Módulo de Distribución Espacial y Curvas (`layout.rs`)
//!
//! Implementa los motores de auto-disposición geométrica de los nodos en el plano 2D:
//! - **Árbol Horizontal Balanceado (`disponer_en_arbol_horizontal`)**: Distribuye las ramas de primer nivel
//!   equitativamente a la derecha (+X) y a la izquierda (-X), calculando las alturas acumuladas
//!   de subramas para evitar cualquier solapamiento.
//! - **Distribución Radial (`disponer_en_radial`)**: Distribuye ramas en abanico circular.
//! - **Cálculo de Curvas Bézier Cúbicas (`calcular_curva_bezier`)**: Genera vectores suaves
//!   con puntos de control adaptativos según la dirección y distancia.

use crate::model::{ModoDisposicion, Proyecto};
use egui::Pos2;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Ancho mínimo por defecto para un nodo.
pub const ANCHO_MINIMO_NODO: f32 = 120.0;
/// Ancho máximo antes de truncar o envolver texto.
pub const ANCHO_MAXIMO_NODO: f32 = 240.0;
/// Altura base de un nodo estándar.
pub const ALTURA_BASE_NODO: f32 = 48.0;
/// Separación horizontal entre niveles de profundidad.
pub const SEPARACION_HORIZONTAL: f32 = 80.0;
/// Separación vertical entre ramas adyacentes.
pub const SEPARACION_VERTICAL: f32 = 24.0;

/// Ancho que se reserva por cada carácter del título al estimar el tamaño del nodo.
///
/// Es una aproximación: medir el texto de verdad exigiría la fuente y la escala del momento,
/// y esta estimación se usa también al repartir los nodos, fuera del dibujado.
const ANCHO_POR_CARACTER: f32 = 8.5;

/// Margen a los lados del título, sumado al ancho que ocupan sus caracteres.
const MARGEN_HORIZONTAL_DEL_TITULO: f32 = 40.0;

/// A partir de cuántos caracteres se da por hecho que el título ocupará dos líneas.
///
/// Se cuenta en caracteres, no en bytes. Antes se medía con `len()`, que devuelve bytes: en
/// un programa cuyo contenido está en español, un título con tildes o con emojis alcanzaba
/// el umbral antes que otro igual de largo sin ellas, y el ancho y el alto del mismo nodo se
/// calculaban con criterios distintos.
const CARACTERES_PARA_SEGUNDA_LINEA: usize = 25;

/// Altura extra cuando el título no cabe en una línea.
const ALTURA_EXTRA_POR_TITULO_LARGO: f32 = 16.0;

/// Altura extra cuando el nodo tiene notas, que se anuncian con una línea más.
const ALTURA_EXTRA_POR_NOTAS: f32 = 14.0;

/// Altura extra cuando el nodo lleva etiquetas, que ocupan su propia línea.
const ALTURA_EXTRA_POR_ETIQUETAS: f32 = 18.0;

/// Distancia **mínima** entre la idea central y sus pilares, en el árbol horizontal.
///
/// La distancia real se calcula contando el ancho de la raíz, como en el resto de
/// generaciones; esta es el suelo, para que un mapa de títulos cortos se siga viendo con el
/// mismo aire de siempre.
///
/// Los pilares se reparten a ambos lados, así que a la izquierda se aplica en negativo.
/// Estaban escritos como dos números sueltos, `220.0` y `-220.0`, en funciones distintas:
/// cambiar uno y olvidar el otro dejaba el árbol descentrado.
const SEPARACION_DEL_PRIMER_NIVEL: f32 = 220.0;

/// Holgura que se añade entre generaciones, además de la separación horizontal.
const HOLGURA_ENTRE_GENERACIONES: f32 = 60.0;

/// Distancia entre un anillo y el siguiente en la disposición radial.
///
/// Tiene que dar para el ancho de una tarjeta más su aire, o el hijo se dibuja encima del
/// padre: 160 no llegaba, porque una tarjeta mide hasta 240. Es la separación **entre
/// anillos consecutivos**, medida desde el centro del mapa: cada generación está a esta
/// distancia de la anterior, y solo se aleja más si su sector no da para las tarjetas.
///
/// Antes menguaba al bajar de generación, multiplicada por un factor de 0,85 que resultó no
/// tener efecto alguno: la distancia real la fijaba siempre el mínimo por sector, y el
/// término reducido no podía ganarle nunca. Cambiar el factor de 0,85 a 0,01 dejaba las
/// posiciones idénticas.
const DISTANCIA_ENTRE_ANILLOS: f32 = ANCHO_MAXIMO_NODO + SEPARACION_HORIZONTAL;

/// Cuánto se curva una conexión respecto a la distancia horizontal que salva.
const FACTOR_DE_CURVATURA: f32 = 0.5;

/// Curvatura mínima, para que dos nodos muy próximos no se unan con una recta.
const CURVATURA_MINIMA: f32 = 35.0;

/// Estima las dimensiones `(ancho, alto)` que ocupará un nodo en el lienzo
/// según la longitud de su título, la presencia de notas y la cantidad de etiquetas.
///
/// Es una estimación y no una medida: calcular el tamaño real exigiría medir el texto con
/// la fuente y la escala del momento, y esta función se usa también fuera del dibujado,
/// al repartir los nodos por el lienzo.
///
/// # Parámetros
/// - `title`: el título del nodo, del que se toma la longitud.
/// - `tiene_notas`: si el nodo tiene notas, que ocupan una línea más.
/// - `numero_de_etiquetas`: cuántas etiquetas lleva, que ocupan otra línea si hay alguna.
///
/// # Devuelve
/// El par `(ancho, alto)` en puntos del lienzo, sin aplicar el zoom.
pub fn estimar_tamano_del_nodo(
    title: &str,
    tiene_notas: bool,
    numero_de_etiquetas: usize,
) -> (f32, f32) {
    let caracteres = title.chars().count();
    let ancho = (caracteres as f32 * ANCHO_POR_CARACTER + MARGEN_HORIZONTAL_DEL_TITULO)
        .clamp(ANCHO_MINIMO_NODO, ANCHO_MAXIMO_NODO);

    let mut alto = ALTURA_BASE_NODO;
    if caracteres > CARACTERES_PARA_SEGUNDA_LINEA {
        alto += ALTURA_EXTRA_POR_TITULO_LARGO;
    }
    if tiene_notas {
        alto += ALTURA_EXTRA_POR_NOTAS;
    }
    if numero_de_etiquetas > 0 {
        alto += ALTURA_EXTRA_POR_ETIQUETAS;
    }

    (ancho, alto)
}

/// Aplica el algoritmo de disposición automática configurado en el proyecto.
///
/// El modo no se pasa como argumento: se lee del propio proyecto, para que la disposición
/// elegida viaje con el archivo y se conserve al volver a abrirlo.
///
/// # Parámetros
/// - `proyecto`: el mapa cuyos nodos se van a recolocar. Se modifican sus posiciones; ni la
///   jerarquía ni el contenido se tocan.
pub fn aplicar_disposicion_automatica(proyecto: &mut Proyecto) {
    match proyecto.layout_mode {
        ModoDisposicion::HorizontalTree => disponer_en_arbol_horizontal(proyecto),
        ModoDisposicion::RadialTree => disponer_en_radial(proyecto),
        ModoDisposicion::FreeDrag => {
            // No sobrescribe las posiciones manuales fijadas por el usuario
        }
    }
}

/// Distribuye el mapa como un arbol horizontal equilibrado.
///
/// Coloca la raíz en el origen y reparte sus hijos en dos mitades: la primera hacia
/// la derecha (+X) y la segunda hacia la izquierda (-X). Cada mitad se centra
/// verticalmente usando las alturas de subarbol precalculadas, de modo que ninguna
/// rama se solape con otra.
///
/// # Parámetros
/// - `proyecto`: proyecto cuyos nodos se reposicionan.
fn disponer_en_arbol_horizontal(proyecto: &mut Proyecto) {
    let root_id = proyecto.root_id;
    let hijos_de_la_raiz = match proyecto.nodes.get(&root_id) {
        Some(r) => r.children.clone(),
        None => return,
    };

    // La raíz manda: se planta en el origen y todo lo demás se coloca respecto a ella.
    if let Some(r) = proyecto.nodes.get_mut(&root_id) {
        r.pos = [0.0, 0.0];
    }

    if hijos_de_la_raiz.is_empty() {
        return;
    }

    let separacion = separacion_del_primer_nivel(proyecto, root_id);

    // Los pilares se reparten a los dos lados del centro, mitad y mitad. Cuando son impares,
    // el de más va a la derecha, que es el lado que se lee primero.
    let cuantos_a_la_derecha = hijos_de_la_raiz.len().div_ceil(2);
    let (a_la_derecha, a_la_izquierda) = hijos_de_la_raiz.split_at(cuantos_a_la_derecha);

    let mut alturas_de_las_ramas = HashMap::new();
    for &id_del_hijo in &hijos_de_la_raiz {
        calcular_altura_de_la_rama(proyecto, id_del_hijo, &mut alturas_de_las_ramas);
    }

    disponer_un_lado(
        proyecto,
        a_la_derecha,
        separacion,
        1.0,
        &alturas_de_las_ramas,
    );
    disponer_un_lado(
        proyecto,
        a_la_izquierda,
        -separacion,
        -1.0,
        &alturas_de_las_ramas,
    );
}

/// A qué distancia del centro se coloca la primera generación de nodos.
///
/// # Por qué se mide y no es un número fijo
///
/// Se cuenta el ancho de la raíz, igual que hace [`disponer_rama`] con cada generación
/// siguiente. Antes era una constante, y bastaba mientras los títulos fueran cortos: en cuanto
/// la raíz y un pilar sumaban más de 440 puntos de ancho —dos títulos de veinticuatro
/// caracteres ya lo hacen, y los que pone el escáner de repositorios son rutas enteras— **las
/// dos tarjetas se pintaban una encima de otra**.
///
/// Y no era solo feo: en la franja solapada el cursor está sobre los dos nodos a la vez, así
/// que el clic seleccionaba uno y el arrastre se llevaba el otro, según el orden en que
/// tocara recorrer el mapa.
///
/// # Parámetros
/// - `proyecto`: el mapa, para poder medir la raíz.
/// - `root_id`: el nodo raíz.
///
/// # Devuelve
/// La distancia, nunca menor que [`SEPARACION_DEL_PRIMER_NIVEL`], para que los mapas de
/// títulos cortos se sigan viendo igual que siempre.
fn separacion_del_primer_nivel(proyecto: &Proyecto, root_id: Uuid) -> f32 {
    let (ancho_de_la_raiz, _) = proyecto
        .nodes
        .get(&root_id)
        .map(|r| estimar_tamano_del_nodo(&r.title, !r.notes.is_empty(), r.tags.len()))
        .unwrap_or((ANCHO_MINIMO_NODO, ALTURA_BASE_NODO));

    // La mitad del ancho es lo que la raíz ocupa hacia ese lado; el resto es hueco.
    (ancho_de_la_raiz / 2.0 + SEPARACION_HORIZONTAL + HOLGURA_ENTRE_GENERACIONES)
        .max(SEPARACION_DEL_PRIMER_NIVEL)
}

/// Reparte verticalmente las ramas de uno de los dos lados del mapa, centradas en el origen.
///
/// # Por qué una sola función para los dos lados
///
/// Eran dos bloques de treinta líneas idénticos salvo por el signo de la separación y el del
/// sentido. Dos copias de un reparto vertical significan que cualquier ajuste —una holgura,
/// un centrado— hay que hacerlo dos veces, y basta con olvidarse de una para que el mapa
/// quede simétrico solo a medias, que es de las cosas más difíciles de ver mirando.
///
/// # Parámetros
/// - `proyecto`: el mapa, cuyas posiciones se escriben.
/// - `ramas`: los hijos de la raíz que van a este lado, en orden. Si está vacío no hace nada.
/// - `separacion`: distancia horizontal al centro, con signo: positiva a la derecha.
/// - `sentido`: hacia dónde siguen creciendo las generaciones de esta rama, `1.0` o `-1.0`.
/// - `alturas`: la altura ya calculada del subárbol de cada nodo.
fn disponer_un_lado(
    proyecto: &mut Proyecto,
    ramas: &[Uuid],
    separacion: f32,
    sentido: f32,
    alturas: &HashMap<Uuid, f32>,
) {
    if ramas.is_empty() {
        return;
    }

    let altura_de = |id: &Uuid| alturas.get(id).copied().unwrap_or(ALTURA_BASE_NODO);

    let altura_total: f32 = ramas.iter().map(altura_de).sum::<f32>()
        + (ramas.len().saturating_sub(1) as f32 * SEPARACION_VERTICAL);

    // Se empieza arriba del todo, a media altura del conjunto, para que el bloque entero
    // quede centrado en la raíz.
    let mut y = -altura_total / 2.0;
    for id_del_hijo in ramas {
        let altura = altura_de(id_del_hijo);
        disponer_rama(
            proyecto,
            *id_del_hijo,
            separacion,
            y + altura / 2.0,
            sentido,
            alturas,
        );
        y += altura + SEPARACION_VERTICAL;
    }
}

/// Calcula la altura que ocupa el subárbol de cada nodo y la deja en `alturas`.
///
/// La altura de un nodo es la mayor entre su propia altura y la suma de las alturas de
/// sus hijos más las separaciones. Se calcula de abajo arriba, así que los hijos deben
/// procesarse antes que el padre.
///
/// # Por qué es iterativa
///
/// La versión recursiva de esta función agotaba la pila del proceso ante un mapa con
/// un ciclo o con una rama de decenas de miles de niveles, matándolo con
/// `STATUS_STACK_OVERFLOW`. Aquí se usa una pila explícita en el montículo con marca de
/// visitados, de modo que ni la profundidad ni un ciclo pueden derribar el programa.
///
/// Aunque `Proyecto::validar_estructura` ya impide que tales mapas lleguen hasta aquí,
/// esta función conserva su propia defensa: el usuario puede modificar el mapa después
/// de cargarlo, y la disposición se recalcula sobre ese estado ya editado.
///
/// # Parámetros
/// - `proyecto`: proyecto cuyo subárbol se mide.
/// - `id_del_nodo`: raíz del subárbol.
/// - `alturas`: mapa donde se acumulan las alturas calculadas.
///
/// # Devuelve
/// La altura total del subárbol que cuelga de `id_del_nodo`.
fn calcular_altura_de_la_rama(
    proyecto: &Proyecto,
    id_del_nodo: Uuid,
    alturas: &mut HashMap<Uuid, f32>,
) -> f32 {
    // Cada entrada de la pila indica si los hijos del nodo ya se han encolado. En la
    // primera visita se encolan; en la segunda, ya con los hijos medidos, se calcula
    // la altura del nodo. Es la traducción directa de un recorrido en postorden.
    let mut pila: Vec<(Uuid, bool)> = vec![(id_del_nodo, false)];
    let mut en_proceso: HashSet<Uuid> = HashSet::new();

    while let Some((id_actual, hijos_listos)) = pila.pop() {
        let Some(nodo) = proyecto.nodes.get(&id_actual) else {
            alturas.insert(id_actual, ALTURA_BASE_NODO);
            continue;
        };

        let (_, altura_propia) =
            estimar_tamano_del_nodo(&nodo.title, !nodo.notes.is_empty(), nodo.tags.len());

        if nodo.children.is_empty() || nodo.collapsed {
            alturas.insert(id_actual, altura_propia);
            continue;
        }

        if !hijos_listos {
            // Un nodo que reaparece antes de haberse resuelto indica un ciclo: se
            // trata como hoja para cortar el recorrido en seco.
            if !en_proceso.insert(id_actual) {
                alturas.insert(id_actual, altura_propia);
                continue;
            }

            pila.push((id_actual, true));
            for &id_hijo in &nodo.children {
                pila.push((id_hijo, false));
            }
            continue;
        }

        // Segunda visita: todos los hijos tienen ya su altura registrada.
        let mut altura_hijos = 0.0;
        for (i, id_hijo) in nodo.children.iter().enumerate() {
            if i > 0 {
                altura_hijos += SEPARACION_VERTICAL;
            }
            altura_hijos += alturas.get(id_hijo).copied().unwrap_or(ALTURA_BASE_NODO);
        }

        en_proceso.remove(&id_actual);
        alturas.insert(id_actual, altura_propia.max(altura_hijos));
    }

    alturas
        .get(&id_del_nodo)
        .copied()
        .unwrap_or(ALTURA_BASE_NODO)
}

/// Sitúa un nodo y toda su rama en el plano, extendiéndose en la dirección indicada.
///
/// El recorrido es iterativo con pila explícita y marca de visitados, por el mismo
/// motivo que [`calcular_altura_de_la_rama`]: la versión recursiva moría por
/// desbordamiento de pila ante ramas muy largas o ciclos.
///
/// # Parámetros
/// - `proyecto`: proyecto cuyos nodos se reposicionan.
/// - `id_del_nodo`: nodo raíz de la rama a situar.
/// - `x`, `y`: coordenadas donde se coloca ese nodo.
/// - `direccion`: `+1.0` para desplegar hacia la derecha, `-1.0` hacia la izquierda.
/// - `alturas_de_las_ramas`: alturas precalculadas por [`calcular_altura_de_la_rama`].
fn disponer_rama(
    proyecto: &mut Proyecto,
    id_del_nodo: Uuid,
    x: f32,
    y: f32,
    direccion: f32, // +1.0 hacia la derecha, -1.0 hacia la izquierda
    alturas_de_las_ramas: &HashMap<Uuid, f32>,
) {
    let mut visitados: HashSet<Uuid> = HashSet::new();
    let mut pendientes: Vec<(Uuid, f32, f32)> = vec![(id_del_nodo, x, y)];

    while let Some((id_actual, pos_x, pos_y)) = pendientes.pop() {
        if !visitados.insert(id_actual) {
            continue; // Ya colocado: hay un ciclo o un hijo compartido.
        }

        let Some(nodo) = proyecto.nodes.get_mut(&id_actual) else {
            continue;
        };

        nodo.pos = [pos_x, pos_y];
        let (ancho, _) =
            estimar_tamano_del_nodo(&nodo.title, !nodo.notes.is_empty(), nodo.tags.len());
        let hijos = nodo.children.clone();
        let plegado = nodo.collapsed;

        if hijos.is_empty() || plegado {
            continue;
        }

        let altura_total_hijos: f32 = hijos
            .iter()
            .map(|id| {
                alturas_de_las_ramas
                    .get(id)
                    .copied()
                    .unwrap_or(ALTURA_BASE_NODO)
            })
            .sum::<f32>()
            + (hijos.len().saturating_sub(1) as f32 * SEPARACION_VERTICAL);

        let siguiente_x =
            pos_x + direccion * (ancho / 2.0 + SEPARACION_HORIZONTAL + HOLGURA_ENTRE_GENERACIONES);
        let mut y_actual = pos_y - altura_total_hijos / 2.0;

        for id_hijo in hijos {
            let altura_hijo = alturas_de_las_ramas
                .get(&id_hijo)
                .copied()
                .unwrap_or(ALTURA_BASE_NODO);
            let centro_y = y_actual + altura_hijo / 2.0;
            pendientes.push((id_hijo, siguiente_x, centro_y));
            y_actual += altura_hijo + SEPARACION_VERTICAL;
        }
    }
}

/// Calcula el rectángulo del lienzo que ocupa todo el mapa visible.
///
/// Cuenta la tarjeta entera de cada nodo, no solo el punto donde se ancla, y deja fuera los
/// que están ocultos bajo una rama plegada: lo que no se dibuja no ocupa sitio en pantalla.
///
/// # Parámetros
/// - `proyecto`: el mapa del que se mide la extensión.
///
/// # Devuelve
/// El rectángulo envolvente en coordenadas del lienzo, o `None` si no hay nada que encuadrar.
pub fn caja_del_mapa(proyecto: &Proyecto) -> Option<egui::Rect> {
    let ocultos = proyecto.nodos_ocultos_por_plegado();

    let mut caja: Option<egui::Rect> = None;

    for (id, nodo) in &proyecto.nodes {
        if ocultos.contains(id) {
            continue;
        }

        let (ancho, alto) =
            estimar_tamano_del_nodo(&nodo.title, !nodo.notes.is_empty(), nodo.tags.len());
        let centro = Pos2::new(nodo.pos[0], nodo.pos[1]);
        let del_nodo = egui::Rect::from_center_size(centro, egui::Vec2::new(ancho, alto));

        caja = Some(match caja {
            Some(actual) => actual.union(del_nodo),
            None => del_nodo,
        });
    }

    caja
}

/// Arco que necesita un nodo para sí mismo en la circunferencia donde se coloque.
///
/// Es el ancho de su tarjeta más el aire que la separa de la vecina. Se mide con el mismo
/// estimador que usa el dibujado, así que un mapa de títulos cortos no reserva el sitio de
/// uno de títulos largos: antes se contaba [`ANCHO_MAXIMO_NODO`] para todos, y eso hinchaba
/// el radio de cualquier mapa normal.
///
/// # Parámetros
/// - `nodo`: el nodo que se va a colocar.
///
/// # Devuelve
/// El arco, en unidades del lienzo, que hay que reservarle sobre la circunferencia.
fn arco_propio_del_nodo(nodo: &crate::model::Nodo) -> f32 {
    let (ancho, _) = estimar_tamano_del_nodo(&nodo.title, !nodo.notes.is_empty(), nodo.tags.len());
    ancho + SEPARACION_VERTICAL
}

/// Calcula, para cada nodo, el arco que necesitan él y toda su descendencia visible.
///
/// Es la medida con la que se reparte la circunferencia. Una rama de treinta hojas necesita
/// treinta veces más sitio que una hoja suelta, y si se les da el mismo sector —que es lo
/// que hacía la versión anterior— la rama gruesa solo cabe alejándose muchísimo del centro,
/// arrastrando consigo el radio de todo el anillo.
///
/// Un nodo plegado cuenta como una hoja: sus hijos no se dibujan, así que no ocupan sitio.
///
/// # Parámetros
/// - `proyecto`: el mapa que se va a disponer.
///
/// # Devuelve
/// Un índice de identificador al arco requerido por su subárbol.
///
/// Recorre en postorden con una pila propia, sin recursión, por el mismo motivo que el resto
/// del módulo: una rama muy profunda agotaría la del proceso. Los nodos ya calculados no se
/// vuelven a visitar, así que un hijo compartido por dos padres no dispara un recorrido
/// exponencial.
///
/// # El ciclo que esto tuvo que aprender a cortar
///
/// La primera versión decía cortar los ciclos y no los cortaba. Su única guarda era «este
/// nodo ya está calculado», y **un nodo dentro de un ciclo no se calcula nunca**: con
/// `A → B → A`, cada vuelta reapilaba `(A, pendiente)` y `(B, sin calcular)` sin resolver
/// ninguno de los dos. No era lentitud, era no terminar, con la pila creciendo una entrada
/// por vuelta hasta agotar la memoria del proceso.
///
/// Hace falta una segunda marca, la de «lo estoy calculando ahora mismo», que es justo la que
/// [`calcular_altura_de_la_rama`] ya llevaba. Un nodo que reaparece antes de resolverse se
/// trata como hoja: se le da su arco propio y se corta por ahí.
///
/// Hoy ningún mapa con ciclos llega hasta aquí, porque
/// [`crate::model::Proyecto::validar_estructura`] los rechaza y el cargador valida antes de
/// devolver nada. Eso hace de esto una defensa en profundidad, no un parche a un agujero
/// abierto; pero las otras tres travesías del módulo la tienen, y el comentario de esta
/// afirmaba tenerla.
fn arcos_requeridos(proyecto: &Proyecto) -> HashMap<Uuid, f32> {
    let mut requerido: HashMap<Uuid, f32> = HashMap::new();
    // Los que están a medio calcular: se han apilado sus hijos y falta sumar lo que salga.
    let mut en_proceso: HashSet<Uuid> = HashSet::new();
    let mut pila: Vec<(Uuid, bool)> = vec![(proyecto.root_id, false)];

    while let Some((id, hijos_ya_calculados)) = pila.pop() {
        if requerido.contains_key(&id) {
            continue;
        }

        let Some(nodo) = proyecto.nodes.get(&id) else {
            continue;
        };

        let propio = arco_propio_del_nodo(nodo);

        if hijos_ya_calculados {
            let de_los_hijos: f32 = nodo
                .children
                .iter()
                .filter_map(|hijo| requerido.get(hijo))
                .sum();
            // Nunca menos de lo que ocupa él: una rama de un solo hijo estrecho no puede
            // reservar menos sitio del que necesita su propia tarjeta.
            requerido.insert(id, de_los_hijos.max(propio));
            continue;
        }

        if nodo.children.is_empty() || nodo.collapsed {
            requerido.insert(id, propio);
            continue;
        }

        // Reaparece sin haberse resuelto: hay un ciclo. Se cierra como hoja.
        if !en_proceso.insert(id) {
            requerido.insert(id, propio);
            continue;
        }

        pila.push((id, true));
        for &hijo in &nodo.children {
            pila.push((hijo, false));
        }
    }

    requerido
}

/// Reparte un sector angular entre unos hermanos, en proporción a lo que necesita cada uno.
///
/// # Parámetros
/// - `proyecto`: el mapa, para consultar los nodos.
/// - `hermanos`: los nodos entre los que se reparte, en orden.
/// - `requerido`: el arco que necesita cada subárbol, de [`arcos_requeridos`].
/// - `inicio`: ángulo donde empieza el sector que se reparte.
/// - `sector`: cuánto ángulo hay que repartir.
///
/// # Devuelve
/// Una entrada por hermano con su ángulo de inicio y su porción, en el mismo orden.
///
/// El total nunca puede ser cero mientras haya un hermano: [`estimar_tamano_del_nodo`]
/// recorta el ancho con `clamp` a [`ANCHO_MINIMO_NODO`], así que hasta un título vacío pide
/// 120 más la separación. Aquí había una rama de reparto a partes iguales «por si un mapa
/// manipulado trae títulos vacíos»: era inalcanzable —sustituida por un `panic!`, la batería
/// entera pasaba, incluido un mapa con **todos** los títulos vacíos— y su justificación
/// nombraba un caso que no existe. En su lugar queda un suelo en el divisor, que no es una
/// rama que nadie pueda recorrer y evita que un cambio futuro de las constantes convierta
/// esto en una división por cero: eso metería infinitos en las posiciones, y un infinito
/// llega al archivo como `null` y lo deja ilegible.
fn repartir_sector(
    proyecto: &Proyecto,
    hermanos: &[Uuid],
    requerido: &HashMap<Uuid, f32>,
    inicio: f32,
    sector: f32,
) -> Vec<(Uuid, f32, f32)> {
    let arco_de = |id: &Uuid| -> f32 {
        requerido.get(id).copied().unwrap_or_else(|| {
            proyecto
                .nodes
                .get(id)
                .map(arco_propio_del_nodo)
                .unwrap_or(SEPARACION_VERTICAL)
        })
    };

    let total: f32 = hermanos.iter().map(arco_de).sum();

    let mut reparto = Vec::with_capacity(hermanos.len());
    let mut angulo = inicio;

    for id in hermanos {
        let porcion = sector * arco_de(id) / total.max(f32::EPSILON);
        reparto.push((*id, angulo, porcion));
        angulo += porcion;
    }

    reparto
}

/// Distribuye el mapa en abanico circular alrededor de la raíz.
///
/// Cada nodo recibe un **sector angular proporcional al sitio que necesita su rama**, y se
/// coloca sobre el anillo de su generación, en el centro de ese sector. Los anillos se miden
/// desde el centro del mapa, no desde el padre.
///
/// # Los dos defectos que corrige esta versión
///
/// La anterior repartía el sector **a partes iguales entre hermanos**, sin mirar cuántas
/// hojas colgaba cada uno, y calculaba la distancia **desde el padre**, ignorando el radio
/// ya recorrido. Las dos cosas se multiplican: una rama gruesa solo cabía en su porción
/// alejándose mucho, y esa distancia se sumaba otra vez en la generación siguiente, así que
/// el radio se multiplicaba por el factor de ramificación en cada nivel. Ciento seis nodos
/// daban un radio de 9693 —el mismo mapa en árbol horizontal ocupa 462 × 1080—, y un mapa
/// radial de cuarenta nodos se abría mostrando la tarjeta central y poco más.
///
/// Con el reparto proporcional, la rama gruesa se lleva el sector que le corresponde y no
/// necesita alejarse; con el radio acumulado, cada anillo está a una distancia fija del
/// anterior y no hereda la de nadie.
///
/// # Parámetros
/// - `proyecto`: proyecto cuyos nodos se reposicionan.
fn disponer_en_radial(proyecto: &mut Proyecto) {
    let root_id = proyecto.root_id;
    if let Some(r) = proyecto.nodes.get_mut(&root_id) {
        r.pos = [0.0, 0.0];
    }

    let hijos_de_la_raiz = match proyecto.nodes.get(&root_id) {
        Some(r) if !r.collapsed => r.children.clone(),
        _ => return,
    };

    if hijos_de_la_raiz.is_empty() {
        return;
    }

    let requerido = arcos_requeridos(proyecto);

    // Cada entrada del anillo lleva el nodo, el ángulo donde empieza su sector y cuánto
    // sector tiene. El ángulo al que se dibuja es el centro de esa porción.
    let mut anillo = repartir_sector(
        proyecto,
        &hijos_de_la_raiz,
        &requerido,
        0.0,
        std::f32::consts::TAU,
    );

    // Los nodos ya colocados no se vuelven a tocar: un hijo compartido o un ciclo dejaría el
    // bucle dando vueltas. Es la misma protección que tenía el recorrido anterior.
    let mut colocados: HashSet<Uuid> = HashSet::new();
    colocados.insert(root_id);

    let mut radio_anterior = 0.0_f32;

    while !anillo.is_empty() {
        // El anillo se aleja lo justo: una generación de separación, y lo que haga falta
        // para que la tarjeta más apretada quepa en el sector que le ha tocado.
        //
        // El primer anillo tenía además un suelo propio, `RADIO_DEL_PRIMER_ANILLO`, de 240.
        // No servía de nada: el primer anillo se calcula como `0 + DISTANCIA_ENTRE_ANILLOS`,
        // que son 320, y 320 nunca es menor que 240. Bajar la constante a 1,0 dejaba las
        // posiciones idénticas en los ocho mapas con que se comprobó. Era el mismo caso que
        // el factor de 0,85 que se retiró en la 0.4.4, y de la misma reescritura. Quien
        // quiera separar más la primera generación del centro tiene que tocar
        // `DISTANCIA_ENTRE_ANILLOS`, que es lo que de verdad manda.
        let mut radio = radio_anterior + DISTANCIA_ENTRE_ANILLOS;
        for &(id, _, sector) in &anillo {
            if let Some(nodo) = proyecto.nodes.get(&id) {
                radio = radio.max(arco_propio_del_nodo(nodo) / sector.max(f32::EPSILON));
            }
        }

        let mut siguiente = Vec::new();

        for (id, inicio, sector) in anillo {
            if !colocados.insert(id) {
                continue;
            }

            let angulo = inicio + sector / 2.0;

            let (hijos, plegado) = {
                let Some(nodo) = proyecto.nodes.get_mut(&id) else {
                    continue;
                };
                nodo.pos = [radio * angulo.cos(), radio * angulo.sin()];
                (nodo.children.clone(), nodo.collapsed)
            };

            if hijos.is_empty() || plegado {
                continue;
            }

            siguiente.extend(repartir_sector(
                proyecto, &hijos, &requerido, inicio, sector,
            ));
        }

        radio_anterior = radio;
        anillo = siguiente;
    }
}

/// Calcula los cuatro puntos de una curva de Bézier cúbica entre dos nodos.
///
/// La curvatura se toma proporcional a la distancia horizontal entre los extremos, con un
/// mínimo fijo. Así, dos nodos muy próximos no se unen con una curva casi recta —que
/// costaría distinguir de un cruce— y dos nodos alejados no reciben una comba desmedida.
///
/// # Parámetros
/// - `from`: punto de salida, en el borde del nodo padre.
/// - `to`: punto de llegada, en el borde del nodo hijo.
///
/// # Devuelve
/// Los cuatro puntos `[origen, control 1, control 2, destino]` que espera `CubicBezierShape`.
pub fn calcular_curva_bezier(from: Pos2, to: Pos2) -> [Pos2; 4] {
    let dx = (to.x - from.x).abs();
    let curvature = (dx * FACTOR_DE_CURVATURA).max(CURVATURA_MINIMA);

    let ctrl1 = if to.x >= from.x {
        Pos2::new(from.x + curvature, from.y)
    } else {
        Pos2::new(from.x - curvature, from.y)
    };

    let ctrl2 = if to.x >= from.x {
        Pos2::new(to.x - curvature, to.y)
    } else {
        Pos2::new(to.x + curvature, to.y)
    };

    [from, ctrl1, ctrl2, to]
}
