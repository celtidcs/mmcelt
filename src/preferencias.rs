//! # Módulo de Preferencias del Usuario (`preferencias.rs`)
//!
//! Guarda entre sesiones los ajustes de presentación: la escala de la interfaz y el tema
//! visual.
//!
//! ## Por qué existe
//!
//! `egui` ya permite escalar toda la interfaz con `Ctrl` `+` / `-` / `0`, y lo hace desde
//! el principio sin que haya que programar nada. Pero **no recuerda el ajuste**: al
//! cerrar la aplicación, el factor vuelve a 1,0.
//!
//! Para quien trabaja en un monitor corriente eso da igual. Para quien usa la aplicación
//! en un televisor, en una pantalla 4K o simplemente a más distancia de lo habitual,
//! significa reajustar el tamaño **cada vez que abre el programa**, lo cual convierte una
//! función útil en una molestia.
//!
//! Este módulo cierra ese hueco: la escala elegida se guarda y se restaura al arrancar.
//!
//! ## Lo que el software puede y no puede deducir
//!
//! Conviene tenerlo claro, porque marca el límite de cualquier ajuste automático:
//!
//! | Dato | ¿Se puede saber? |
//! |---|---|
//! | Resolución del monitor | ✅ Sí |
//! | Escala de pantalla del sistema (DPI) | ✅ Sí |
//! | Tamaño físico de la pantalla en pulgadas | ⚠️ No de forma fiable |
//! | **A qué distancia se sienta el usuario** | ❌ **No** |
//!
//! Ese último punto es el decisivo. Un televisor de 42 pulgadas a tres metros presenta al
//! sistema operativo exactamente los mismos datos que un monitor de 24 pulgadas a medio
//! metro: misma resolución, misma escala. El sistema no tiene forma de distinguirlos, y
//! sin embargo el tamaño de letra adecuado es muy distinto.
//!
//! Por eso la estrategia no es adivinar, sino **proponer un punto de partida razonable y
//! recordar lo que el usuario decida**. Que es, al final, lo que resuelve su problema.

use crate::error::{AppError, AppResult};
use crate::textos::{Idioma, Texto};
use crate::theme::AppThemeMode;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Nombre del archivo de preferencias dentro del directorio de datos.
const ARCHIVO_PREFERENCIAS: &str = "preferencias.json";

/// Escala mínima admitida para la interfaz.
///
/// **No coincide con el límite de `egui`**, que llega hasta 0,2 en su zoom por teclado. Es
/// más estrecho a propósito: por debajo de la mitad los rótulos de los menús dejan de
/// leerse. Que los dos rangos sean distintos obliga a devolver el zoom al rango propio
/// cuando el usuario se sale con `Ctrl` `-`; de eso se ocupa [`ajuste_de_escala`].
///
/// El comentario anterior afirmaba lo contrario —«coincide con el límite que impone
/// egui»—, y de ese supuesto salía el defecto que [`ajuste_de_escala`] documenta.
pub const ESCALA_MINIMA: f32 = 0.5;

/// Escala máxima admitida para la interfaz.
///
/// Tampoco coincide con la de `egui`, que llega a 5,0. Ver [`ESCALA_MINIMA`].
pub const ESCALA_MAXIMA: f32 = 3.0;

/// Diferencia a partir de la cual se considera que el zoom ha cambiado de verdad.
///
/// El factor de zoom es un número en coma flotante y puede variar en cifras
/// insignificantes sin que nadie haya tocado nada. Comparar con igualdad exacta haría que
/// la interfaz guardara las preferencias en fotogramas en los que no ha pasado nada.
pub const TOLERANCIA_ESCALA: f32 = 0.01;

/// Escala por defecto: el tamaño nativo que decida el sistema operativo.
pub const ESCALA_POR_DEFECTO: f32 = 1.0;

/// Escalas ofrecidas como botones en el menú, con la clave de su texto descriptivo.
///
/// Son las que cubren los casos habituales sin obligar a ajustar a mano. La de 200 % está
/// pensada precisamente para televisores y pantallas vistas desde el sofá.
///
/// El texto descriptivo se resuelve en el idioma activo a través del enum [`Texto`],
/// evitando cadenas fijas en este módulo.
pub const ESCALAS_SUGERIDAS: &[(f32, Texto)] = &[
    (0.8, Texto::VerEscalaCompacto),
    (1.0, Texto::VerEscalaNormal),
    (1.25, Texto::VerEscalaComodo),
    (1.5, Texto::VerEscalaGrande),
    (2.0, Texto::VerEscalaTelevisor),
];

/// Lo que hay que hacer cuando el zoom real de `egui` y la escala guardada no coinciden.
///
/// Lo devuelve [`ajuste_de_escala`]; ver allí el porqué de cada campo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AjusteDeEscala {
    /// La escala que debe quedar vigente, ya dentro del rango admitido.
    pub escala: f32,
    /// Si hay que imponerle a `egui` esa escala porque su zoom se ha salido del rango.
    pub forzar_zoom: bool,
    /// Si la preferencia cambia y, por tanto, hay que guardarla y avisar al usuario.
    pub guardar: bool,
}

/// Decide qué hacer cuando el zoom de `egui` y la escala guardada dejan de coincidir.
///
/// `egui` atiende `Ctrl` `+`, `Ctrl` `-` y `Ctrl` `0` por su cuenta, sin avisar a nadie, y
/// admite un rango **más amplio** que el de esta aplicación: de 0,2 a 5,0 frente a
/// [`ESCALA_MINIMA`]–[`ESCALA_MAXIMA`]. La interfaz se entera de que el usuario ha usado
/// esos atajos precisamente porque el zoom deja de coincidir con lo guardado.
///
/// # El bucle que esto evita
///
/// La versión anterior recortaba el valor al rango propio y lo guardaba, pero **nunca se lo
/// devolvía a `egui`**. Con treinta pulsaciones de `Ctrl` `+` el zoom real quedaba en 3,9 y
/// la preferencia en 3,0: al fotograma siguiente seguían sin coincidir, así que se volvía a
/// escribir `preferencias.json` y a reiniciar el mensaje de la barra de estado. Y al
/// siguiente. Sesenta escrituras en disco por segundo, indefinidamente, sin más forma de
/// pararlo que volver al rango a mano. Le ocurría justo a quien trabaja en un televisor,
/// que es para quien existe este módulo.
///
/// Por eso, cuando el zoom se ha salido del rango, no basta con recortar la preferencia:
/// hay que imponerle a `egui` el valor admitido, y así el fotograma siguiente ya coincide.
///
/// # Parámetros
/// - `zoom_real`: el factor que `egui` tiene aplicado en este fotograma.
/// - `guardada`: la escala que consta en las preferencias.
///
/// # Devuelve
/// `None` si no hay nada que hacer —los dos valores coinciden dentro de
/// [`TOLERANCIA_ESCALA`]—, o el ajuste que hay que aplicar.
pub fn ajuste_de_escala(zoom_real: f32, guardada: f32) -> Option<AjusteDeEscala> {
    if (zoom_real - guardada).abs() <= TOLERANCIA_ESCALA {
        return None;
    }

    let escala = zoom_real.clamp(ESCALA_MINIMA, ESCALA_MAXIMA);

    Some(AjusteDeEscala {
        escala,
        // Solo si el recorte ha cambiado algo: cuando el usuario se queda dentro del rango,
        // su zoom se respeta tal cual y no se le mueve la interfaz bajo los dedos.
        forzar_zoom: (escala - zoom_real).abs() > f32::EPSILON,
        // Un zoom de 3,9 con la preferencia ya en 3,0 no cambia la preferencia: hay que
        // devolver el zoom a su sitio, pero no reescribir el archivo ni repetir el aviso.
        guardar: (escala - guardada).abs() > f32::EPSILON,
    })
}

/// Ajustes de presentación que se conservan entre sesiones.
///
/// El `#[serde(default)]` va en la estructura, no en cada campo: así un campo que falte toma
/// su valor de [`Default`], que es **la única definición** de lo que vale por omisión. Antes
/// la escala lo tenía por duplicado —en `Default` y en una función suelta para `serde`— y las
/// dos podían separarse sin que nada avisara.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferencias {
    /// Factor de escala de toda la interfaz.
    ///
    /// Multiplica el tamaño de textos, botones, márgenes y paneles. No afecta al zoom del
    /// lienzo, que es independiente y se maneja con la rueda del ratón.
    pub escala_interfaz: f32,

    /// Tema visual activo.
    pub tema: AppThemeMode,

    /// Carpeta dentro de la cual pueden trabajar los agentes de IA.
    ///
    /// Se guarda porque, si no, la ventana de conexión proponía en cada arranque la carpeta
    /// por omisión mientras el agente conectado seguía trabajando en la que se eligió la vez
    /// anterior: la pantalla decía una cosa y el límite real era otro, justo en el sitio
    /// donde el programa promete al usuario dónde puede escribir su agente.
    ///
    /// Vacía significa «la que se proponga por omisión».
    pub espacio_trabajo_ia: String,

    /// Cada cuántos minutos se guarda la copia de seguridad, o cero si está desactivado.
    ///
    /// Los valores admitidos son los de `autoguardado::MINUTOS_DE_AUTOGUARDADO_OFRECIDOS`.
    /// Uno que no esté entre ellos se ignora al aplicarlo, porque este archivo es texto y
    /// cualquiera puede escribir en él a mano.
    pub intervalo_autoguardado_minutos: u64,

    /// Idioma de la interfaz.
    ///
    /// Se guarda por lo mismo que la escala: elegirlo en cada arranque convertiría una
    /// comodidad en una molestia. Un archivo de preferencias anterior a que esto existiera no
    /// lo trae, y entonces vale el idioma en que se escribió el programa.
    pub idioma: Idioma,

    /// Si ya se mostró la sugerencia inicial de escala.
    ///
    /// Evita repetirla en cada arranque: se propone una vez y, a partir de ahí, manda lo
    /// que el usuario haya decidido.
    pub sugerencia_mostrada: bool,

    /// Plantillas de encargo definidas por el usuario a nivel global.
    #[serde(default)]
    pub plantillas_usuario: Vec<PlantillaUsuario>,
}

/// Plantilla de encargo personalizada guardada por el usuario en sus preferencias globales.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlantillaUsuario {
    /// Nombre visible de la plantilla en el selector desplegable.
    pub nombre: String,
    /// Contenido textual del encargo a cargar.
    pub contenido: String,
}

impl Default for Preferencias {
    /// Las preferencias de quien abre MMCelt por primera vez.
    ///
    /// No son valores neutros elegidos al azar: son los que hacen que el programa se
    /// pueda usar sin configurar nada. El espacio de trabajo va vacío a propósito —hasta
    /// que el usuario elige uno, ningún agente puede escribir en ninguna parte— y
    /// `sugerencia_mostrada` en `false` para que la propuesta de carpeta aparezca una vez.
    ///
    /// El idioma sale de [`Idioma::default`], que lo deduce del sistema en el primer
    /// arranque en lugar de imponer el castellano.
    fn default() -> Self {
        Self {
            escala_interfaz: ESCALA_POR_DEFECTO,
            tema: AppThemeMode::default(),
            espacio_trabajo_ia: String::new(),
            intervalo_autoguardado_minutos:
                crate::autoguardado::MINUTOS_DE_AUTOGUARDADO_POR_DEFECTO,
            idioma: Idioma::default(),
            sugerencia_mostrada: false,
            plantillas_usuario: Vec::new(),
        }
    }
}

impl Preferencias {
    /// Preferencias de un equipo donde MMCelt todavía no ha guardado nada.
    ///
    /// Es el **único** sitio que consulta el idioma del sistema. [`Self::default`] no lo
    /// hace, y no debe hacerlo: también es el punto de partida al leer un archivo que ya
    /// existe, y allí un idioma detectado le cambiaría la interfaz por debajo a quien lleva
    /// tiempo usando el programa.
    ///
    /// # Devuelve
    /// Las preferencias de fábrica, con el idioma que pide el sistema operativo.
    pub fn primer_arranque() -> Self {
        Self {
            idioma: Idioma::del_sistema(),
            ..Self::default()
        }
    }

    /// Carga las preferencias guardadas, o devuelve las de primer arranque si no existen.
    ///
    /// Un contenido que no sea JSON se interpreta campo a campo como valores predeterminados,
    /// pero un fallo del sistema al leer el archivo se conserva como [`AppError::Lectura`].
    pub fn cargar() -> AppResult<Self> {
        let Some(carpeta) = crate::autoguardado::directorio_datos() else {
            return Ok(Self::primer_arranque());
        };

        Self::cargar_de(&carpeta)
    }

    /// Carga las preferencias guardadas **en la carpeta que se le indique**.
    ///
    /// Misma promesa que [`Self::cargar`]. Existe para que una prueba pueda darle su propia
    /// carpeta en lugar de reescribir una variable de entorno que comparten todos los hilos.
    ///
    /// # Parámetros
    /// - `carpeta_datos`: carpeta de datos donde vive `preferencias.json`.
    pub fn cargar_de(carpeta_datos: &Path) -> AppResult<Self> {
        let ruta = ruta_preferencias_en(carpeta_datos);
        let contenido = match std::fs::read_to_string(&ruta) {
            Ok(contenido) => contenido,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::primer_arranque());
            }
            Err(error) => return Err(AppError::lectura(&ruta, error)),
        };

        Ok(Self::desde_texto(&contenido))
    }

    /// Interpreta el contenido del archivo de preferencias.
    ///
    /// Va aparte de [`Self::cargar`] para poder comprobarse sin tocar el disco: `cargar`
    /// busca la ruta y lee el archivo, y esto es la parte que decide qué valores salen.
    ///
    /// Nunca falla. Un archivo ilegible, o que no sea siquiera un objeto JSON, devuelve las
    /// preferencias por omisión, porque unas preferencias rotas no pueden impedir arrancar.
    ///
    /// # Parámetros
    /// - `contenido`: el texto del archivo, tal cual se leyó.
    ///
    /// # Devuelve
    /// Las preferencias, con cada campo leído por separado y la escala ya acotada al rango
    /// admitido.
    pub fn desde_texto(contenido: &str) -> Self {
        // Campo a campo, y no de una vez, a propósito.
        //
        // `serde_json::from_str::<Self>(...).unwrap_or_default()` parece equivalente y no lo
        // es: `#[serde(default)]` cubre los campos **ausentes**, pero un campo presente con
        // un valor que serde no sabe leer —un idioma escrito mal, un tema de una versión
        // posterior, un número donde iba un texto— hace fallar la lectura **entera**, y el
        // `unwrap_or_default` la convertía en preferencias de fábrica sin decir nada.
        //
        // Lo que se perdía así no era solo cosmético: `espacio_trabajo_ia` es la carpeta
        // dentro de la cual pueden escribir los agentes de IA. Al quedar vacía se vuelve a la
        // sugerida por omisión, que es exactamente el desajuste que el campo existe para
        // evitar —la pantalla diciendo una cosa y el límite real siendo otro—. Y en cuanto el
        // usuario tocara el tema o el idioma, el guardado consolidaba los valores de fábrica
        // encima de su archivo.
        //
        // Así, un valor ilegible se queda en su valor por omisión y **no arrastra a los
        // demás**.
        let mut preferencias = Self::default();

        if let Ok(serde_json::Value::Object(guardado)) = serde_json::from_str(contenido) {
            /// Lee un campo, o se queda con lo que hubiera si no se puede interpretar.
            fn leer<T: serde::de::DeserializeOwned>(
                guardado: &serde_json::Map<String, serde_json::Value>,
                nombre: &str,
                destino: &mut T,
            ) {
                if let Some(valor) = guardado.get(nombre) {
                    if let Ok(leido) = serde_json::from_value(valor.clone()) {
                        *destino = leido;
                    }
                }
            }

            leer(
                &guardado,
                "escala_interfaz",
                &mut preferencias.escala_interfaz,
            );
            leer(&guardado, "tema", &mut preferencias.tema);
            leer(
                &guardado,
                "espacio_trabajo_ia",
                &mut preferencias.espacio_trabajo_ia,
            );
            leer(
                &guardado,
                "intervalo_autoguardado_minutos",
                &mut preferencias.intervalo_autoguardado_minutos,
            );
            leer(&guardado, "idioma", &mut preferencias.idioma);
            leer(
                &guardado,
                "sugerencia_mostrada",
                &mut preferencias.sugerencia_mostrada,
            );
            leer(
                &guardado,
                "plantillas_usuario",
                &mut preferencias.plantillas_usuario,
            );
        }

        // Una escala fuera de rango dejaría la interfaz inutilizable, y en el caso
        // extremo impediría llegar al menú para corregirla.
        preferencias.escala_interfaz = preferencias
            .escala_interfaz
            .clamp(ESCALA_MINIMA, ESCALA_MAXIMA);

        preferencias
    }

    /// Guarda las preferencias **en la carpeta que se le indique**.
    ///
    /// # Parámetros
    /// - `carpeta_datos`: carpeta de datos donde escribir `preferencias.json`.
    ///
    /// # Devuelve
    /// `Ok(())` si se guardaron correctamente.
    pub fn guardar_en(&self, carpeta_datos: &Path) -> AppResult<()> {
        let ruta = ruta_preferencias_en(carpeta_datos);

        if let Some(carpeta) = ruta.parent() {
            std::fs::create_dir_all(carpeta)
                .map_err(|error| AppError::escritura(carpeta, error))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|error| AppError::formato("al serializar las preferencias", error))?;

        // Atómica también, aunque unas preferencias truncadas ya no borren el resto de los
        // ajustes desde la 0.4.7: es la misma utilidad y no cuesta nada.
        crate::storage::escribir_de_forma_atomica(&ruta, json.as_bytes())
    }
}

/// Devuelve la ruta del archivo de preferencias **dentro de la carpeta indicada**.
///
/// # Parámetros
/// - `carpeta_datos`: carpeta de datos de la aplicación.
pub fn ruta_preferencias_en(carpeta_datos: &Path) -> PathBuf {
    carpeta_datos.join(ARCHIVO_PREFERENCIAS)
}

/// Propone una escala inicial a partir de las características del monitor.
///
/// **No adivina la distancia de visionado**, que es lo que de verdad determina el tamaño
/// adecuado y que ningún programa puede conocer. Se limita a corregir el caso claro: una
/// pantalla de resolución muy alta cuyo sistema operativo no está aplicando escala
/// ninguna, donde la interfaz sale diminuta para cualquiera.
///
/// # Parámetros
/// - `tamano_monitor`: resolución del monitor en puntos de interfaz, si se conoce.
/// - `escala_del_sistema`: puntos por píxel que informa el sistema operativo.
///
/// # Devuelve
/// La escala sugerida. Devuelve [`ESCALA_POR_DEFECTO`] cuando no hay motivo claro para
/// cambiarla, que es el caso más frecuente.
pub fn escala_sugerida(tamano_monitor: Option<egui::Vec2>, escala_del_sistema: Option<f32>) -> f32 {
    let Some(monitor) = tamano_monitor else {
        return ESCALA_POR_DEFECTO;
    };

    // Si el sistema ya está escalando (por ejemplo, Windows al 150 %), respetarlo:
    // volver a escalar encima produciría una interfaz desproporcionada.
    if escala_del_sistema.unwrap_or(1.0) > 1.2 {
        return ESCALA_POR_DEFECTO;
    }

    let ancho = monitor.x;

    // Los umbrales corresponden a resoluciones reales, no a números redondos elegidos al
    // azar: 3840 es 4K y 2560 es QHD. Por debajo de eso, la escala nativa está bien.
    if ancho >= 3800.0 {
        1.5
    } else if ancho >= 2500.0 {
        1.25
    } else {
        ESCALA_POR_DEFECTO
    }
}
