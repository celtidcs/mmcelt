//! Validación de las invariantes estructurales y límites de un mapa.

use super::tipos::{
    Nodo, Proyecto, LONGITUD_MAXIMA_NOTAS, LONGITUD_MAXIMA_TEXTO_DESCRIPTIVO,
    LONGITUD_MAXIMA_TITULO, NODOS_MAXIMOS, PROFUNDIDAD_MAXIMA,
};
use crate::error::{AppError, AppResult, FalloEstructural};
use std::collections::HashSet;
use uuid::Uuid;

impl Proyecto {
    /// Comprueba que el proyecto describe un árbol válido antes de usarlo.
    ///
    /// **Debe invocarse sobre todo proyecto que provenga del exterior**: archivos
    /// cargados desde disco, respuestas de modelos de IA y datos del servidor MCP. El
    /// resto del código (`layout`, `eliminar_nodo`, `profundidad_del_nodo`, la capa de dibujo) da
    /// por supuestas estas garantías; sin ellas, un archivo manipulado o corrupto
    /// congelaba la aplicación o agotaba su pila.
    ///
    /// El recorrido es **iterativo a propósito**: usa una pila explícita en el montículo
    /// en lugar de recursión, porque su cometido es precisamente admitir sin morir las
    /// mismas entradas que antes desbordaban la pila.
    ///
    /// # Garantías que verifica
    /// 1. `root_id` existe dentro de `nodes`.
    /// 2. El nodo raíz no declara ningún padre (`parent_id` es `None`).
    /// 3. El número de nodos no supera [`NODOS_MAXIMOS`].
    /// 4. Todo identificador de `children` referencia un nodo existente cuyo `parent_id`
    ///    apunta de forma recíproca a dicho padre.
    /// 5. Todo `parent_id` referencia un nodo existente que incluye a dicho nodo entre
    ///    sus `children`.
    /// 6. No hay ciclos: ningún nodo se alcanza dos veces desde la raíz.
    /// 7. Ningún nodo queda huérfano fuera del árbol de la raíz.
    /// 8. La profundidad no supera [`PROFUNDIDAD_MAXIMA`].
    /// 9. Toda coordenada de posición es un número finito.
    /// 10. Los dos extremos de cada conexión cruzada existen dentro de `nodes`.
    ///
    /// # Devuelve
    /// `Ok(())` si la estructura es válida.
    ///
    /// # Errores
    /// Devuelve [`AppError::EstructuraInvalida`] con la variante de
    /// [`FalloEstructural`] que identifica el nodo concreto que incumple la garantía,
    /// para que el usuario o una IA puedan repararlo.
    pub fn validar_estructura(&self) -> AppResult<()> {
        self.validar_la_raiz_y_el_tamano()?;
        let visitados = self.recorrer_el_arbol_comprobando_el_parentesco()?;
        self.validar_que_ningun_nodo_queda_fuera_del_arbol(&visitados)?;
        self.validar_que_las_posiciones_son_finitas()?;
        self.validar_las_conexiones_cruzadas()
    }

    /// Comprueba las tres garantías que no hacen falta recorrer el árbol para verificar.
    ///
    /// Son las tres primeras de [`Self::validar_estructura`]: que la raíz existe, que no
    /// declara padre y que el mapa no se pasa de nodos. Van juntas y primero porque, sin
    /// raíz, no hay nada que recorrer.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] con la variante que identifica el incumplimiento.
    fn validar_la_raiz_y_el_tamano(&self) -> AppResult<()> {
        // 1. La raíz debe existir. Sin ella no hay nada que recorrer.
        let Some(nodo_raiz) = self.nodes.get(&self.root_id) else {
            return Err(AppError::estructura(FalloEstructural::RaizInexistente {
                root_id: self.root_id.to_string(),
            }));
        };

        // 2. La raíz no puede declarar un padre.
        if let Some(id_padre) = nodo_raiz.parent_id {
            return Err(AppError::estructura(FalloEstructural::RaizConPadre {
                root_id: self.root_id.to_string(),
                padre: id_padre.to_string(),
            }));
        }

        // 3. Tamaño total, antes de recorrer nada.
        if self.nodes.len() > NODOS_MAXIMOS {
            return Err(AppError::estructura(FalloEstructural::DemasiadosNodos {
                cantidad: self.nodes.len(),
                limite: NODOS_MAXIMOS,
            }));
        }

        Ok(())
    }

    /// Recorre el árbol desde la raíz comprobando ciclos, profundidad y reciprocidad.
    ///
    /// Es un recorrido en profundidad con pila explícita, y no recursivo: un mapa muy hondo
    /// agotaría la pila del proceso. Cada entrada lleva el nodo y su profundidad, de modo que
    /// el límite se comprueba sobre la marcha sin una segunda pasada.
    ///
    /// # Devuelve
    /// Los nodos alcanzados desde la raíz, que es lo que hace falta después para saber si
    /// alguno se ha quedado fuera del árbol.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] si hay un ciclo, si se pasa de profundidad, si un
    /// padre o un hijo no existen, o si el parentesco no es recíproco.
    fn recorrer_el_arbol_comprobando_el_parentesco(&self) -> AppResult<HashSet<Uuid>> {
        let mut visitados: HashSet<Uuid> = HashSet::with_capacity(self.nodes.len());
        let mut pila: Vec<(Uuid, usize)> = vec![(self.root_id, 0)];

        while let Some((id_actual, profundidad)) = pila.pop() {
            if profundidad > PROFUNDIDAD_MAXIMA {
                return Err(AppError::estructura(
                    FalloEstructural::ProfundidadExcesiva {
                        limite: PROFUNDIDAD_MAXIMA,
                    },
                ));
            }

            // Llegar dos veces al mismo nodo significa que hay un ciclo o que dos
            // padres comparten el mismo hijo. Ambos casos rompen las suposiciones
            // del resto del código.
            if !visitados.insert(id_actual) {
                return Err(AppError::estructura(FalloEstructural::CicloDetectado {
                    nodo: id_actual.to_string(),
                }));
            }

            let Some(nodo) = self.nodes.get(&id_actual) else {
                // Solo alcanzable desde un `children` colgante, comprobado más abajo;
                // se mantiene por prudencia ante futuras rutas de entrada.
                return Err(AppError::estructura(FalloEstructural::HijoInexistente {
                    padre: "desconocido".to_string(),
                    hijo: id_actual.to_string(),
                }));
            };

            self.comprobar_el_padre_del_nodo(id_actual, nodo)?;
            self.comprobar_los_hijos_del_nodo(id_actual, nodo, profundidad, &mut pila)?;
        }

        Ok(visitados)
    }

    /// Comprueba que el nodo tiene un padre coherente con su sitio en el árbol.
    ///
    /// La raíz no puede tener padre; cualquier otro nodo debe tener uno que exista y que lo
    /// incluya entre sus hijos.
    ///
    /// # Parámetros
    /// - `id_actual`: el nodo que se comprueba.
    /// - `nodo`: ese mismo nodo, ya localizado.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] si el padre no existe o no es recíproco.
    fn comprobar_el_padre_del_nodo(&self, id_actual: Uuid, nodo: &Nodo) -> AppResult<()> {
        if id_actual == self.root_id {
            if let Some(id_padre) = nodo.parent_id {
                return Err(AppError::estructura(FalloEstructural::RaizConPadre {
                    root_id: self.root_id.to_string(),
                    padre: id_padre.to_string(),
                }));
            }
        } else if let Some(id_padre) = nodo.parent_id {
            let Some(padre) = self.nodes.get(&id_padre) else {
                return Err(AppError::estructura(FalloEstructural::PadreInexistente {
                    nodo: id_actual.to_string(),
                    padre: id_padre.to_string(),
                }));
            };
            if !padre.children.contains(&id_actual) {
                return Err(AppError::estructura(FalloEstructural::PadreNoReciproco {
                    nodo: id_actual.to_string(),
                    padre: id_padre.to_string(),
                }));
            }
        } else {
            return Err(AppError::estructura(FalloEstructural::PadreNoReciproco {
                nodo: id_actual.to_string(),
                padre: "ninguno".to_string(),
            }));
        }

        Ok(())
    }

    /// Comprueba los hijos declarados por un nodo y los deja preparados para visitarlos.
    ///
    /// # Parámetros
    /// - `id_actual`: el nodo cuyos hijos se comprueban.
    /// - `nodo`: ese mismo nodo, ya localizado.
    /// - `profundidad`: a qué altura del árbol está, para apilar los hijos un nivel más abajo.
    /// - `pila`: la pila del recorrido, donde se añaden los hijos que quedan por visitar.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] si un hijo no existe o no reconoce a su padre.
    fn comprobar_los_hijos_del_nodo(
        &self,
        id_actual: Uuid,
        nodo: &Nodo,
        profundidad: usize,
        pila: &mut Vec<(Uuid, usize)>,
    ) -> AppResult<()> {
        // Todo hijo declarado debe existir y no ser huérfano de padre.
        for &id_hijo in &nodo.children {
            let Some(hijo) = self.nodes.get(&id_hijo) else {
                return Err(AppError::estructura(FalloEstructural::HijoInexistente {
                    padre: id_actual.to_string(),
                    hijo: id_hijo.to_string(),
                }));
            };
            if hijo.parent_id.is_none() {
                return Err(AppError::estructura(FalloEstructural::HijoNoReciproco {
                    padre: id_actual.to_string(),
                    hijo: id_hijo.to_string(),
                }));
            }
            pila.push((id_hijo, profundidad + 1));
        }

        Ok(())
    }

    /// Comprueba que no ha quedado ningún nodo colgando fuera del árbol de la raíz.
    ///
    /// # Parámetros
    /// - `visitados`: los nodos que alcanzó el recorrido.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] con un ejemplo del nodo huérfano, para poder buscarlo.
    fn validar_que_ningun_nodo_queda_fuera_del_arbol(
        &self,
        visitados: &HashSet<Uuid>,
    ) -> AppResult<()> {
        if visitados.len() != self.nodes.len() {
            let ejemplo = self
                .nodes
                .keys()
                .find(|id| !visitados.contains(id))
                .map(|id| id.to_string())
                .unwrap_or_else(|| "desconocido".to_string());

            return Err(AppError::estructura(FalloEstructural::NodosHuerfanos {
                cantidad: self.nodes.len() - visitados.len(),
                ejemplo,
            }));
        }

        Ok(())
    }

    /// Comprueba que ninguna coordenada es infinita ni «no es un número».
    ///
    /// Una posición no finita se propaga por los cálculos del lienzo y de la disposición
    /// automática hasta dejar el mapa entero sin dibujar.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] señalando el nodo que la tiene.
    fn validar_que_las_posiciones_son_finitas(&self) -> AppResult<()> {
        for (id, nodo) in &self.nodes {
            if !nodo.pos[0].is_finite() || !nodo.pos[1].is_finite() {
                return Err(AppError::estructura(FalloEstructural::PosicionNoFinita {
                    nodo: id.to_string(),
                }));
            }
        }

        Ok(())
    }

    /// Comprueba que una conexión cruzada no apunte a nodos ausentes.
    ///
    /// El lienzo omite defensivamente esos extremos, pero las demás salidas —entre ellas
    /// Mermaid— trabajan con la garantía de que el grafo ya ha sido validado. Rechazar aquí el
    /// archivo evita que dos consumidores den interpretaciones distintas al mismo mapa.
    ///
    /// # Errores
    /// [`AppError::EstructuraInvalida`] con la conexión y el extremo concreto que no existe.
    fn validar_las_conexiones_cruzadas(&self) -> AppResult<()> {
        for conexion in &self.connections {
            if !self.nodes.contains_key(&conexion.from) {
                return Err(AppError::estructura(
                    FalloEstructural::ConexionConOrigenInexistente {
                        conexion: conexion.id.to_string(),
                        origen: conexion.from.to_string(),
                    },
                ));
            }
            if !self.nodes.contains_key(&conexion.to) {
                return Err(AppError::estructura(
                    FalloEstructural::ConexionConDestinoInexistente {
                        conexion: conexion.id.to_string(),
                        destino: conexion.to.to_string(),
                    },
                ));
            }
        }
        Ok(())
    }

    /// Comprueba que ningún texto del mapa se pase de largo.
    ///
    /// # Por qué está separado de [`Proyecto::validar_estructura`]
    ///
    /// Porque **no se aplica al abrir un archivo del usuario**, y eso es deliberado.
    ///
    /// Los topes nacieron para un caso concreto, escrito en la ficha del defecto: un agente
    /// en bucle puede llenar el disco con notas enormes sin salirse del espacio permitido.
    /// El destinatario es el agente, no la persona.
    ///
    /// Metidos dentro de `validar_estructura` hacían otra cosa muy distinta. Esa función se
    /// ejecuta **al cargar** (`storage::cargar_proyecto_de_archivo`), mientras que guardar no
    /// valida nada y la interfaz no limita lo que se teclea. Con eso, quien pegara un
    /// documento largo en las notas de un nodo guardaba sin problema y **al volver a abrir su
    /// mapa se encontraba con que el programa lo rechazaba**. Comprobado ejecutándolo: se
    /// guardaba con éxito y la carga devolvía «el campo `notes` tiene 70000 caracteres y
    /// supera el límite de 65536». El trabajo quedaba inaccesible, y antes del tope ese mismo
    /// archivo se abría.
    ///
    /// Así que el tope se queda en la frontera por la que entra lo que el usuario no
    /// controla: el servidor MCP y la importación desde una IA. Lo que ya está guardado en su
    /// disco se le abre siempre.
    ///
    /// # Devuelve
    ///
    /// `Ok(())` si todos los textos caben en su límite.
    ///
    /// # Errores
    ///
    /// [`FalloEstructural::TextoDemasiadoLargo`] con el campo, la longitud y el límite, para
    /// que el agente sepa exactamente qué recortar.
    pub fn validar_longitudes_de_texto(&self) -> AppResult<()> {
        self.validar_longitudes_del_proyecto()?;
        for (id, nodo) in &self.nodes {
            validar_longitud(&nodo.title, id, "title", LONGITUD_MAXIMA_TITULO)?;
            validar_longitud(&nodo.notes, id, "notes", LONGITUD_MAXIMA_NOTAS)?;
            validar_longitud(
                &nodo.correction_feedback,
                id,
                "correction_feedback",
                LONGITUD_MAXIMA_TEXTO_DESCRIPTIVO,
            )?;
        }
        Ok(())
    }

    /// Comprueba los cuatro textos que describen el proyecto, separados de sus nodos.
    fn validar_longitudes_del_proyecto(&self) -> AppResult<()> {
        validar_longitud(&self.title, "proyecto", "title", LONGITUD_MAXIMA_TITULO)?;
        validar_longitud(
            &self.creator_vision,
            "proyecto",
            "creator_vision",
            LONGITUD_MAXIMA_TEXTO_DESCRIPTIVO,
        )?;
        validar_longitud(
            &self.project_goals,
            "proyecto",
            "project_goals",
            LONGITUD_MAXIMA_TEXTO_DESCRIPTIVO,
        )?;
        validar_longitud(
            &self.target_audience_or_context,
            "proyecto",
            "target_audience_or_context",
            LONGITUD_MAXIMA_TEXTO_DESCRIPTIVO,
        )
    }
}

/// Aplica de forma uniforme un límite de caracteres y construye el error solo al superarlo.
fn validar_longitud(
    texto: &str,
    nodo: impl std::fmt::Display,
    campo: &'static str,
    limite: usize,
) -> AppResult<()> {
    let longitud = texto.chars().count();
    if longitud <= limite {
        return Ok(());
    }
    Err(AppError::estructura(
        FalloEstructural::TextoDemasiadoLargo {
            nodo: nodo.to_string(),
            campo,
            longitud,
            limite,
        },
    ))
}
