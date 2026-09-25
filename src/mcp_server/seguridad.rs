//! Seguridad de rutas, espacio de trabajo y escrituras MCP.
//!
//! Todas las operaciones con archivos pasan por estas funciones. Aquí se delimitan la raíz
//! autorizada, las extensiones válidas, los enlaces y la política de copias de seguridad.

use super::error::ErrorHerramienta;
use super::operaciones::VARIABLE_ESPACIO_TRABAJO;
use crate::devolucion_agentes::{OperacionDevolucion, ReciboDevolucion};
use crate::proyecto_trabajo::{CargaProyecto, ContextoProyecto, RepositorioProyecto};
use std::path::{Path, PathBuf};
/// Para qué se va a usar una ruta, y por tanto qué se le puede exigir.
///
/// Antes había un solo `bool` —«¿es para escribir?»— y una única lista de extensiones para
/// **todas** las escrituras. Eso dejaba una puerta abierta de par en par: la herramienta que
/// exporta Markdown aceptaba como destino cualquier ruta terminada en `.mmcelt` y escribía
/// texto encima. Un agente al que le estorbara el criterio del usuario no tenía más que
/// exportar sobre el mapa: el archivo dejaba de ser un mapa, y a partir de ahí la
/// comprobación que protege la supervisión ya no encontraba nada que proteger.
///
/// Con un contrato por operación, ese camino no existe: el exportador solo escribe Markdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UsoDeLaRuta {
    /// Se va a leer. No se exige extensión: da igual cómo se llame lo que se abre.
    Lectura,
    /// Se va a escribir un mapa, que es lo que el usuario considera su trabajo.
    EscrituraDeMapa,
    /// Se va a escribir el documento para la IA, que el programa regenera cuando quiere.
    EscrituraDeMarkdown,
}

impl UsoDeLaRuta {
    /// Extensiones admitidas para este uso, o `None` si no se exige ninguna.
    fn extensiones(self) -> Option<&'static [&'static str]> {
        match self {
            UsoDeLaRuta::Lectura => None,
            UsoDeLaRuta::EscrituraDeMapa => Some(&["mmcelt", "json"]),
            UsoDeLaRuta::EscrituraDeMarkdown => Some(&["md"]),
        }
    }
}

// Seguridad: acotamiento de rutas
// ============================================================================

/// Devuelve la carpeta dentro de la cual puede operar el servidor.
///
/// Es el valor de `MMCELT_WORKSPACE`, que MMCelt escribe en la configuración del agente al
/// conectarlo. **Sin ella, el servidor no opera con archivos.**
///
/// Antes caía a la carpeta de trabajo del proceso, y eso convertía la ausencia de la
/// configuración en ausencia de límite: bastaba con que el cliente no propagara el entorno,
/// con que alguien editara el archivo a mano o con lanzar `mmcelt --mcp-server` desde una
/// terminal, para que el espacio pasara a ser el directorio desde el que se arrancó —la raíz
/// de un repositorio, o el disco entero— sin ningún aviso. Mientras tanto, la ventana de
/// conexión le prometía al usuario que «los agentes solo podrán leer y escribir dentro de
/// esta carpeta».
///
/// Un control de seguridad que se abre del todo cuando falta su configuración no es un
/// control. Ahora falla cerrado.
///
/// # Errores
/// Devuelve error si la variable no está definida, está vacía, o apunta a una carpeta
/// que no existe en el disco.
pub(super) fn espacio_de_trabajo() -> Result<PathBuf, ErrorHerramienta> {
    let bruta = std::env::var_os(VARIABLE_ESPACIO_TRABAJO)
        .map(PathBuf::from)
        .filter(|ruta| !ruta.as_os_str().is_empty())
        .ok_or_else(|| {
            ErrorHerramienta::seguridad(format!(
                "El servidor no tiene definida la carpeta de trabajo \
                 ({VARIABLE_ESPACIO_TRABAJO}), así que no puede leer ni escribir nada. \
                 Vuelve a conectar el agente desde MMCelt, en «🤖 Inteligencia Artificial → \
                 🔌 Conectar MMCelt con mis IAs...», para que quede escrita en su \
                 configuración."
            ))
        })?;

    if !bruta.exists() {
        return Err(ErrorHerramienta::seguridad(format!(
            "La carpeta de trabajo configurada («{}») no existe en el disco. \
             Crea la carpeta o actualiza la variable de entorno {VARIABLE_ESPACIO_TRABAJO}.",
            bruta.display()
        )));
    }

    bruta.canonicalize().map_err(|e| {
        ErrorHerramienta::seguridad(format!(
            "No se pudo resolver la carpeta de trabajo («{}»): {e}",
            bruta.display()
        ))
    })
}

/// Carga la identidad portable si la carpeta ya fue confirmada desde MMCelt.
fn contexto_proyecto_configurado() -> Result<Option<ContextoProyecto>, ErrorHerramienta> {
    let espacio = espacio_de_trabajo()?;
    if !espacio.join(".mmcelt/configuracion.json").is_file() {
        return Ok(None);
    }
    match RepositorioProyecto::cargar(&espacio)
        .map_err(|error| ErrorHerramienta::seguridad(error.mensaje_usuario().en(super::operaciones::idioma_del_usuario())))?
    {
        CargaProyecto::Confirmado(contexto) => Ok(Some(contexto)),
        CargaProyecto::Trasladado { .. } => Err(ErrorHerramienta::seguridad(
            "La carpeta conserva la identidad de otro emplazamiento. Vuelve a conectarla desde MMCelt para confirmar el traslado."
                .to_string(),
        )),
    }
}

/// Prepara la trazabilidad de una escritura cuando el espacio tiene identidad portable.
///
/// El recibo describe lo que el agente afirma devolver. No sustituye la validación de la ruta ni
/// autoriza la escritura, que ya debe haber atravesado las comprobaciones de este módulo.
pub(super) fn preparar_devolucion(
    ruta: &Path,
    nombre_agente: Option<&str>,
    operacion: OperacionDevolucion,
) -> Result<Option<(ContextoProyecto, ReciboDevolucion)>, ErrorHerramienta> {
    let Some(contexto) = contexto_proyecto_configurado()? else {
        return Ok(None);
    };
    let relativa = ruta.strip_prefix(contexto.raiz()).map_err(|_| {
        ErrorHerramienta::seguridad("El mapa queda fuera del proyecto activo.".to_string())
    })?;
    let modelo = nombre_agente
        .map(str::trim)
        .filter(|nombre| !nombre.is_empty())
        .unwrap_or("Agente MCP");
    let recibo =
        ReciboDevolucion::nuevo(&contexto, relativa, modelo, operacion).map_err(|error| {
            ErrorHerramienta::argumento(
                error
                    .mensaje_usuario()
                    .en(super::operaciones::idioma_del_usuario()),
            )
        })?;
    Ok(Some((contexto, recibo)))
}

/// Conserva atómicamente el recibo después de guardar el mapa y su Markdown.
///
/// La aplicación no usa este archivo como señal de recarga: el vigilante observa y valida el mapa
/// real. Esta escritura solo deja una evidencia durable para la persona y el cliente MCP.
pub(super) fn publicar_devolucion(
    contexto: &ContextoProyecto,
    recibo: &ReciboDevolucion,
) -> Result<(), ErrorHerramienta> {
    recibo.validar(contexto).map_err(|error| {
        ErrorHerramienta::seguridad(
            error
                .mensaje_usuario()
                .en(super::operaciones::idioma_del_usuario()),
        )
    })?;
    let ruta = contexto
        .resolver_relativa(Path::new(".mmcelt/devolucion.json"))
        .map_err(|error| {
            ErrorHerramienta::seguridad(
                error
                    .mensaje_usuario()
                    .en(super::operaciones::idioma_del_usuario()),
            )
        })?;
    let contenido = serde_json::to_vec_pretty(recibo).map_err(|error| {
        ErrorHerramienta::ejecucion(format!("No se pudo crear el recibo: {error}"))
    })?;
    crate::storage::escribir_de_forma_atomica(&ruta, &contenido).map_err(|error| {
        ErrorHerramienta::ejecucion(
            error
                .mensaje_usuario()
                .en(super::operaciones::idioma_del_usuario()),
        )
    })
}

// ============================================================================
/// Resuelve una ruta que todavía no existe, apoyándose en el primer antepasado que sí.
///
/// `Path::canonicalize` deshace los enlaces y las uniones, pero **solo funciona sobre algo
/// que exista**. Para una ruta que se va a crear hay que subir hasta el antepasado más
/// profundo que ya esté en el disco, resolver ese, y volver a colgar de él los componentes
/// que faltaban. Así el resultado tiene los enlaces deshechos aunque el destino sea nuevo, y
/// la comprobación de contención decide sobre la ruta real y no sobre el texto.
///
/// # Parámetros
/// - `candidata`: la ruta a resolver, ya compuesta a partir del espacio de trabajo.
///
/// # Devuelve
/// La ruta con los enlaces deshechos hasta donde el disco permite comprobarlo.
///
/// # Errores
/// Devuelve un error de seguridad si ningún antepasado de la ruta existe: sin un punto de
/// apoyo real no hay forma de saber adónde apunta, y adivinar es lo que abría el escape.
fn resolver_desde_el_antepasado_existente(candidata: &Path) -> Result<PathBuf, ErrorHerramienta> {
    let mut pendientes: Vec<std::ffi::OsString> = Vec::new();
    let mut cursor = candidata;

    loop {
        if let Some(nombre) = cursor.file_name() {
            pendientes.push(nombre.to_os_string());
        }

        let Some(padre) = cursor.parent() else {
            return Err(ErrorHerramienta::seguridad(
                "No se puede comprobar dónde acabaría esa ruta: ninguna de sus carpetas \
                 existe. Crea la carpeta destino dentro del espacio de trabajo y vuelve a \
                 intentarlo."
                    .to_string(),
            ));
        };

        if let Ok(base) = padre.canonicalize() {
            let mut resuelta = base;
            // Se apilaron de dentro hacia fuera, así que se devuelven al revés.
            for nombre in pendientes.iter().rev() {
                resuelta.push(nombre);
            }
            return Ok(resuelta);
        }

        cursor = padre;
    }
}

/// Comprueba que una ruta enviada por un modelo es segura, y la devuelve resuelta.
///
/// Es la única puerta por la que pasan todas las rutas del servidor MCP. Rechaza, en este
/// orden:
///
/// 1. La ruta vacía.
/// 2. Cualquier `..`, partiendo por **las dos barras**. En Linux la barra invertida no separa
///    componentes, sino que es un carácter normal de nombre, así que `..\fuera.mmcelt` llega
///    como un único componente y una comprobación basada solo en `Component::ParentDir` no
///    vería nada.
/// 3. La carpeta `.mmcelt`, reservada para identidad, sesiones y recibos del programa.
/// 4. Los dos puntos dentro de un nombre. En NTFS, `documento.txt:mapa.mmcelt` no nombra un
///    archivo: nombra un flujo de datos alternativo colgado de `documento.txt`, invisible en
///    el explorador y en las copias de seguridad.
/// 5. Los nombres terminados en punto o en espacio, y los compuestos solo de puntos, que en
///    Windows no se pueden borrar ni gestionar con las herramientas del sistema.
/// 6. Todo lo que, una vez resuelto, caiga fuera del espacio de trabajo.
/// 7. Las extensiones que ese uso concreto no admita.
///
/// El paso 2 no es redundante con la resolución posterior: `canonicalize` neutraliza los `..`
/// pero **solo si la ruta existe**, y aquí el caso corriente es una ruta que todavía no
/// existe porque se va a crear.
///
/// # Parámetros
/// - `ruta`: la ruta tal como la envió el modelo, sin tocar.
/// - `uso`: para qué se va a usar, que decide qué extensiones se admiten.
///
/// # Devuelve
/// La ruta resuelta, con los enlaces deshechos y garantizada dentro del espacio de trabajo.
///
/// # Errores
/// [`ErrorHerramienta::argumento`] si la ruta viene vacía, y [`ErrorHerramienta::seguridad`]
/// en todos los demás casos.
pub(super) fn validar_ruta(ruta: &str, uso: UsoDeLaRuta) -> Result<PathBuf, ErrorHerramienta> {
    if ruta.trim().is_empty() {
        return Err(ErrorHerramienta::argumento(
            "La ruta del archivo no puede estar vacía.",
        ));
    }

    rechazar_si_sube_de_carpeta(ruta)?;
    rechazar_si_entra_en_la_carpeta_de_control(ruta)?;
    rechazar_si_algun_nombre_lleva_dos_puntos(ruta)?;
    rechazar_si_algun_nombre_acaba_en_punto_o_espacio(ruta)?;

    let raiz = espacio_de_trabajo()?;
    let candidata = raiz.join(ruta);

    // La ruta puede no existir aún, porque se va a crear. En ese caso hay que resolver el
    // antepasado que **sí** exista, y volver a colgar de él lo que falte.
    let resuelta = match candidata.canonicalize() {
        Ok(r) => r,
        Err(_) => resolver_desde_el_antepasado_existente(&candidata)?,
    };

    if !resuelta.starts_with(&raiz) {
        return Err(ErrorHerramienta::seguridad(
            "Ruta fuera del espacio de trabajo permitido. \
             Solo se permite operar dentro de la carpeta asignada al agente."
                .to_string(),
        ));
    }

    rechazar_si_la_extension_no_se_admite(&resuelta, uso)?;

    Ok(resuelta)
}

/// Impide que una ruta proporcionada por un agente entre en la carpeta privada `.mmcelt`.
///
/// Se parte por las dos barras por el mismo motivo que en la protección contra `..`: el texto
/// puede proceder de un cliente ejecutado en otro sistema operativo. La comparación no distingue
/// mayúsculas para que una carpeta equivalente en Windows no atraviese la valla con otra caja.
///
/// # Errores
/// [`ErrorHerramienta::seguridad`] si cualquiera de los componentes se llama `.mmcelt`.
fn rechazar_si_entra_en_la_carpeta_de_control(ruta: &str) -> Result<(), ErrorHerramienta> {
    if ruta
        .split(['/', '\\'])
        .any(|componente| componente.eq_ignore_ascii_case(".mmcelt"))
    {
        return Err(ErrorHerramienta::seguridad(
            "La carpeta «.mmcelt» está reservada para los datos de control de MMCelt y no se puede modificar desde una herramienta MCP."
                .to_string(),
        ));
    }
    Ok(())
}

/// Rechaza cualquier ruta que contenga `..`, mirando el texto antes de resolver nada.
///
/// # Por qué no basta con `canonicalize`
///
/// `canonicalize` sí neutraliza los `..`, pero **solo si la ruta existe**. Cuando no existe se
/// resuelve la carpeta padre, y si esa tampoco existe se usa tal cual, con sus `..` intactos.
/// Como `Path::starts_with` compara componente a componente,
/// `<espacio>/sin_crear/../../fuera/x.mmcelt` empieza literalmente por el espacio de trabajo y
/// pasaba el control; al escribir, el sistema operativo sí resolvía los `..` y el archivo
/// acababa fuera.
///
/// En Windows eso no llegaba a ocurrir, pero por accidente: `canonicalize` devuelve allí rutas
/// con un prefijo de espacio de nombres extendido que la ruta sin resolver no lleva, de modo
/// que la comparación falla y la ruta se rechaza. En Linux y macOS no hay tal prefijo, así que
/// la protección dependía de en qué sistema se ejecutara.
///
/// # Por qué se parte por las dos barras
///
/// En Linux la barra invertida no es un separador: es un carácter normal dentro de un nombre.
/// `Path::new("..\fuera.mmcelt").components()` devuelve entonces un único componente
/// `Normal`, no un `ParentDir`, así que una comprobación basada solo en los componentes no ve
/// nada y la ruta pasa. Comprobado ejecutando la batería en un Linux real: la petición se
/// aceptaba y se creaba un archivo llamado literalmente «..\fuera.mmcelt» dentro del espacio
/// de trabajo.
///
/// No llegaba a escaparse —el archivo queda dentro—, pero la valla dejaba de hacer lo que dice
/// hacer, y ese nombre lleva una barra invertida que en Windows sí separa: el mismo archivo,
/// copiado a Windows, ya no nombra lo mismo.
///
/// Las dos veces la lección es la misma: **un control de seguridad no puede descansar en un
/// detalle de plataforma.**
///
/// # Parámetros
/// - `ruta`: la ruta tal como la envió el modelo, sin resolver.
///
/// # Errores
/// [`ErrorHerramienta::seguridad`] si aparece `..` como componente o como segmento entre
/// barras de cualquiera de los dos tipos.
fn rechazar_si_sube_de_carpeta(ruta: &str) -> Result<(), ErrorHerramienta> {
    let sube = Path::new(ruta)
        .components()
        .any(|componente| matches!(componente, std::path::Component::ParentDir))
        || ruta.split(['/', '\\']).any(|segmento| segmento == "..");

    if sube {
        return Err(ErrorHerramienta::seguridad(
            "La ruta no puede contener «..». Indica la ruta dentro del espacio de trabajo."
                .to_string(),
        ));
    }
    Ok(())
}

/// Rechaza las rutas en las que algún nombre lleva dos puntos.
///
/// # De qué protege
///
/// En NTFS, «documento.txt:mapa.mmcelt» no nombra un archivo: nombra un **flujo de datos
/// alternativo** colgado de «documento.txt». `Path::extension()` sobre esa cadena devuelve
/// «mmcelt», así que el filtro de extensiones lo aprobaba, y lo que se escribía quedaba pegado
/// a un archivo del usuario sin aparecer en el explorador, ni en un listado, ni en las copias
/// de seguridad. Comprobado con `dir /r`: el archivo visible conservaba sus 33 bytes y llevaba
/// 7,8 KB escondidos en tres flujos, que además se podían volver a leer con
/// `mmcelt_read_mindmap`.
///
/// Era un almacén persistente e invisible dentro del espacio del usuario, que ninguna de las
/// otras vallas inspecciona. Fuera de Windows esto solo sería un nombre de archivo raro, pero
/// la comprobación no cuesta nada y el servidor se ejecuta donde se ejecute.
///
/// De paso cierra las rutas absolutas con letra de unidad antes de que lleguen al resto.
///
/// # Por qué solo mira los componentes de nombre
///
/// El prefijo de unidad de una ruta ya resuelta lleva dos puntos con todo derecho, y esta
/// comprobación también recibe rutas absolutas: la del Markdown acompañante se compone a
/// partir de la del mapa, que a esas alturas ya está resuelta. Mirar todos los componentes
/// dejaba catorce pruebas en rojo.
///
/// # Parámetros
/// - `ruta`: la ruta a comprobar.
///
/// # Errores
/// [`ErrorHerramienta::seguridad`] si algún componente de nombre contiene `:`.
fn rechazar_si_algun_nombre_lleva_dos_puntos(ruta: &str) -> Result<(), ErrorHerramienta> {
    let lleva_dos_puntos = Path::new(ruta).components().any(|componente| {
        matches!(componente, std::path::Component::Normal(_))
            && componente.as_os_str().to_string_lossy().contains(':')
    });

    if lleva_dos_puntos {
        return Err(ErrorHerramienta::seguridad(
            "El nombre de un archivo o de una carpeta no puede llevar dos puntos. En Windows \
             eso no nombra un archivo, sino un flujo de datos oculto dentro de otro."
                .to_string(),
        ));
    }
    Ok(())
}

/// Rechaza los nombres que acaban en punto o en espacio, y los compuestos solo de puntos.
///
/// # De qué protege
///
/// En Windows, crear rutas con prefijo extendido (`\\?\`) permite nombres como `....` o
/// `carpeta.` que el subsistema Win32 normal **no puede borrar ni manipular** con las
/// herramientas estándar: ni el Explorador, ni `Remove-Item`, ni `del`. Un agente podría
/// dejar dentro del espacio del usuario archivos que este no tiene forma de quitar.
///
/// # Parámetros
/// - `ruta`: la ruta a comprobar.
///
/// # Errores
/// [`ErrorHerramienta::seguridad`] si algún componente de nombre encaja en alguno de los tres
/// casos.
fn rechazar_si_algun_nombre_acaba_en_punto_o_espacio(ruta: &str) -> Result<(), ErrorHerramienta> {
    let nombre_ingobernable = Path::new(ruta).components().any(|componente| {
        if let std::path::Component::Normal(nombre) = componente {
            let texto = nombre.to_string_lossy();
            texto.ends_with('.') || texto.ends_with(' ') || texto.chars().all(|c| c == '.')
        } else {
            false
        }
    });

    if nombre_ingobernable {
        return Err(ErrorHerramienta::seguridad(
            "El nombre de un archivo o carpeta no puede terminar en punto ni en espacio, \
             ni estar compuesto únicamente de puntos. En Windows estos nombres no se pueden \
             gestionar ni borrar con las herramientas habituales del sistema."
                .to_string(),
        ));
    }
    Ok(())
}

/// Comprueba que la extensión de la ruta es una de las que admite ese uso.
///
/// Es la última valla, y se aplica sobre la ruta **ya resuelta**: sobre el texto sin resolver,
/// un nombre como «mapa.mmcelt.txt» podría confundirla.
///
/// # Parámetros
/// - `resuelta`: la ruta ya canonizada y comprobada como interior al espacio de trabajo.
/// - `uso`: para qué se va a usar, que es lo que decide qué extensiones valen. Si el uso no
///   restringe ninguna, no se comprueba nada.
///
/// # Errores
/// [`ErrorHerramienta::seguridad`] si la extensión no está entre las admitidas, diciendo
/// cuáles lo están.
fn rechazar_si_la_extension_no_se_admite(
    resuelta: &Path,
    uso: UsoDeLaRuta,
) -> Result<(), ErrorHerramienta> {
    let Some(admitidas) = uso.extensiones() else {
        return Ok(());
    };

    let extension = resuelta
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !admitidas.contains(&extension.as_str()) {
        return Err(ErrorHerramienta::seguridad(format!(
            "Extensión no permitida para esta operación: «{extension}». Solo se admiten: {}.",
            admitidas.join(", ")
        )));
    }
    Ok(())
}

/// Qué se hace con lo que ya hay en el archivo antes de escribir encima.
///
/// Existe para que la decisión se tome **en el código y por escrito**, no a partir de un
/// argumento que rellena el propio modelo. Antes era un `bool` alimentado por el argumento
/// `overwrite` de la herramienta, y con eso un agente se eximía a sí mismo de dejar copia:
/// bastaba con pedirlo. La descripción publicada lo llamaba «una decisión explícita de quien
/// llama», pero quien llama es el modelo, no la persona cuyo mapa se estaba pisando.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PoliticaDeCopia {
    /// Guarda una copia con la fecha en el nombre antes de pisar el archivo.
    ///
    /// Es la política de toda escritura que pueda alcanzar algo que no escribió el agente
    /// en esta misma llamada. No hay forma de pedir que se salte.
    DejarCopia,
    /// Escribe sin copia previa.
    ///
    /// **Reservado a `mmcelt_sync_ai_progress`**, que actualiza el mismo archivo una y otra
    /// vez conforme avanza el trabajo: una copia por cada informe de progreso llenaría la
    /// carpeta del usuario de archivos que no ha pedido. Esa herramienta no borra nodos, no
    /// retira un descarte ni firma una aprobación, así que lo que puede estropear está
    /// acotado. Ninguna otra la usa.
    SinCopiaPorSincronizacion,
}

/// Escribe un archivo conservando el contenido anterior si ya existía.
///
/// # Parámetros
/// - `ruta`: destino, ya validado.
/// - `contenido`: lo que se va a escribir.
/// - `politica`: si se conserva copia de lo que había. Ver [`PoliticaDeCopia`].
///
/// # Devuelve
/// La ruta de la copia de seguridad creada, o `None` si no hizo falta.
///
/// # Errores
/// Devuelve error de ejecución si la copia, la carpeta destino o la escritura fallan.
pub(super) fn escribir_con_respaldo(
    ruta: &Path,
    contenido: &str,
    politica: PoliticaDeCopia,
) -> Result<Option<PathBuf>, ErrorHerramienta> {
    let mut respaldo = None;

    if politica == PoliticaDeCopia::DejarCopia {
        respaldo = crate::respaldo::copiar_antes_de_sobrescribir(ruta).map_err(|e| {
            ErrorHerramienta::ejecucion(format!("No se pudo crear la copia de seguridad: {e}"))
        })?;
    }

    if let Some(carpeta) = ruta.parent() {
        std::fs::create_dir_all(carpeta).map_err(|e| {
            ErrorHerramienta::ejecucion(format!("No se pudo crear la carpeta destino: {e}"))
        })?;
    }

    // Atómica: aquí escribe un agente de IA, y lo hace en cada informe de progreso. Un corte
    // a media escritura dejaría el mapa del usuario truncado.
    crate::storage::escribir_de_forma_atomica(ruta, contenido.as_bytes())
        .map_err(|e| ErrorHerramienta::ejecucion(format!("No se pudo escribir el archivo: {e}")))?;

    Ok(respaldo)
}
