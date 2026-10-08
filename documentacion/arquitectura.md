# Arquitectura de MMCelt

Este documento describe cómo está organizado el código, por qué se tomaron las
decisiones técnicas relevantes y qué invariantes deben respetarse al modificarlo.

---

## 1. Visión general

MMCelt consta de **dos programas independientes** que se comunican únicamente a través
de archivos en disco:

```
┌──────────────────────────────────────────────────────────────────┐
│  Aplicación de escritorio (Rust)                                 │
│                                                                  │
│   ┌────────────┐   ┌────────────┐   ┌───────────────────────┐    │
│   │  ui/       │──►│  layout    │──►│  model                │    │
│   │ (egui)     │   │ (posición) │   │ (datos + validación)  │    │
│   └─────┬──────┘   └────────────┘   └───────────┬───────────┘    │
│         │                                       │                │
│         │          ┌────────────┐   ┌───────────▼───────────┐    │
│         └─────────►│ ai_bridge  │   │  storage              │    │
│                    │ ai_export  │   │  (persistencia)       │    │
│                    └────────────┘   └───────────┬───────────┘    │
│                                                 │                │
│                    ┌────────────┐               │                │
│                    │  error     │◄──────────────┘                │
│                    │ (AppError) │                                │
│                    └────────────┘                                │
└─────────────────────────────────────┬────────────────────────────┘
                                      │
                              archivos .mmcelt
                                      │
┌─────────────────────────────────────▼────────────────────────────┐
│  mmcelt.exe --mcp-server                                         │
│  El mismo binario, hablando JSON-RPC 2.0 por stdio · 6 tools     │
└──────────────────────────────────────────────────────────────────┘
```

No hay comunicación directa entre ambos: el servidor MCP escribe un archivo y la
aplicación lo lee, o al revés. Esto los mantiene desacoplados, pero implica que **el
formato del archivo es el contrato entre ellos**, y ambos deben respetarlo.

Cada archivo declara `schema_version: 1` por separado de `generated_by`, que identifica la
aplicación y versión que lo escribió. La ausencia de ambos campos identifica un archivo anterior:
`serde` aporta los valores vigentes al abrirlo, de modo que se actualiza sin perder compatibilidad
en el siguiente guardado.

---

## 2. Estructura de carpetas

```
MMCelt/
├── build.rs                    Captura versión, fecha y commit al compilar
├── Cargo.toml                  Dependencias y perfil de publicación
│
├── src/                        Aplicación de escritorio (Rust)
│   ├── main.rs                 Punto de entrada; elige entre app y servidor MCP
│   ├── model.rs                Modelo de datos y validación estructural
│   │                           (`model/jerarquia.rs`: hacer hijo o hermano sin crear ciclos; `model/conexiones.rs`: qué conexión admite la interfaz;
│   │                            `model/clasificacion.rs`: estado, prioridad, control humano y rol)
│   ├── error.rs                Manejo centralizado de errores
│   ├── storage.rs              Persistencia en disco
│   ├── layout.rs               Disposición espacial de los nodos
│   ├── ai_bridge.rs            Importación desde respuestas de IA
│   ├── ai_export.rs            Exportación a Markdown para IA
│   ├── mcp_server.rs           Servidor MCP integrado (`--mcp-server`)
│   ├── conectores.rs           Detección y configuración de agentes de IA
│   ├── consola_windows.rs      Conserva `--version` sin mostrar consola al abrir la interfaz
│   ├── proyecto_trabajo.rs     Identidad estable y límite seguro de la carpeta de un proyecto
│   ├── sesiones_agentes.rs     Compone el prompt neutral y persiste el expediente de cada envío
│   ├── lanzador_agentes.rs     Traduce una sesión en orden de consola y abre la terminal
│   ├── vigilante.rs            Avisa cuando un agente cambia el mapa en disco (notify)
│   ├── autoguardado.rs         Copia de seguridad periódica y recuperación
│   ├── historial.rs            Deshacer y rehacer, por instantáneas del mapa
│   ├── busqueda.rs             Encuentra nodos por título o etiqueta
│   ├── preferencias.rs         Ajustes que se conservan entre sesiones
│   ├── version.rs              Identificación de la compilación
│   ├── version_publicada.rs    Compara versiones y valida la respuesta de GitHub, sin red
│   ├── comprobacion_de_version.rs Consulta HTTPS de la última versión, en un hilo de trabajo
│   ├── theme.rs                Paletas de color y estilo de toda la ventana
│   ├── textos.rs               Todo lo que la aplicación le dice al usuario, en los seis idiomas (595 textos)
│   ├── audit_checks.rs         Pruebas de regresión y barreras (603 en Windows y 602 en Linux, medido el 2026-10-07)
│   ├── arnes_interfaz.rs       Ejecuta la interfaz e interactúa con clics reales en pruebas, sin abrir ventana
│   └── ui/                     Capa de presentación
│       ├── mod.rs              Estado global, ciclo de vida de eframe y carga de fuente CJK embebida
│       ├── canvas.rs           Lienzo 2D infinito
│       ├── sidebar.rs          Inspector de propiedades
│       ├── toolbar.rs          Barra de herramientas
│       ├── ai_modal.rs         Ventanas modales, incluida «Acerca de»
│       ├── conexiones_modal.rs Ventana de conexión con agentes de IA
│       ├── sesion_agente_modal.rs Vista previa supervisada de una sesión de agente
│       ├── dialogos.rs         Diálogos nativos del sistema
│       ├── estado_version_nueva.rs Comprobación de versión de este arranque y su aviso
│       ├── suelta_de_nodo.rs   Menú al soltar un nodo encima de otro y su resolución
│       ├── menu_contextual_nodo.rs Menú del clic derecho sobre un nodo
│       ├── help_system.rs      Sistema de ayuda contextual: los 24 temas y sus tres piezas
│       └── ayuda_textos.rs     Las 72 piezas de la ayuda por idioma; incrusta las guías
│
├── assets/
│   ├── ayuda/                  Las 144 guías de la ayuda: 24 temas x 6 idiomas, en Markdown
│   ├── fuentes/                Subconjunto embebido de Noto Sans SC (~1,06 MB) para caracteres CJK
│   ├── icono/                  Iconos en varios formatos y tamaños (.svg, .ico, .png, .rgba)
│   └── capturas/               Capturas de pantalla para la documentación
├── skills/                     Habilidad de Claude Code y Antigravity
├── documentacion/integraciones/ Guías de integración por plataforma
└── documentacion/              Esta documentación
```

> El servidor MCP vive en `src/mcp_server.rs` y se activa con `mmcelt --mcp-server`. No
> es un programa aparte: es el mismo binario. Ver la sección 14.

---

## 3. Capas y responsabilidades

| Capa | Módulos | Responsabilidad | Qué **no** debe hacer |
|---|---|---|---|
| Presentación | `ui/` | Dibujar, capturar entrada, mostrar errores | Contener lógica de negocio o tocar el disco directamente |
| Dominio | `model.rs` | Estructura del mapa, invariantes, plantillas | Saber de `egui` ni de rutas de archivo |
| Geometría | `layout.rs` | Calcular posiciones | Modificar el contenido de los nodos |
| Persistencia | `storage.rs` | Leer y escribir archivos | Abrir ventanas ni interactuar con el usuario |
| Traducción IA | `ai_bridge.rs`, `ai_export.rs` | Convertir entre el modelo y el texto para IA | Tocar el disco |
| Errores | `error.rs` | Definir y registrar los fallos | Redactar mensajes visibles (eso lo hace `ui/`) |
| Textos | `textos.rs` | Guardar cada rótulo y aviso, con su traducción | Contener claves de protocolo o de archivo: eso es contrato, no texto para leer |

**La regla de dependencia va hacia dentro.** `ui/` conoce a `model`, pero `model` no
conoce a `ui/`. La única excepción es `error.rs`, del que dependen todas las capas por ser
transversal.

### Por qué los diálogos están en `ui/dialogos.rs` y no en `storage.rs`

Originalmente `storage.rs` mezclaba dos cosas: escribir archivos y abrir diálogos
nativos del sistema. La cabecera del propio archivo lo reconocía.

No son la misma responsabilidad. Un diálogo **no persiste nada**: abre una ventana modal
y pregunta al usuario por una ruta. Eso es interacción, no acceso a datos.

Separarlos tiene una consecuencia práctica: `storage.rs` queda como lógica pura sobre
rutas y contenidos, y por tanto **se puede probar sin abrir ninguna ventana**. Las
pruebas de `audit_checks.rs` lo hacen.

### Por qué `storage.rs` no llama a `std::fs`

Es la aplicación de la **inversión de dependencias** al único sitio del programa donde de
verdad hacía falta.

`storage.rs` es la capa que decide *qué* se guarda, *cómo* se serializa y *cuándo* un archivo
es válido. Eso es política. Abrir un archivo y escribir bytes es un detalle. Mientras la
política llamaba directamente a `std::fs`, las dos cosas estaban atadas y **no había forma de
ejercitar la primera sin ejecutar la segunda**: cualquier prueba de la escritura atómica o de
la carga con validación tenía que crear archivos de verdad en una carpeta temporal, lo que
obliga a serializar las pruebas y deja basura en el disco si una falla a medias.

Ahora `storage.rs` depende de un contrato, `SistemaDeArchivos`, con cuatro operaciones: leer,
leer con límite, escribir de forma atómica y crear una carpeta. La implementación real,
`DiscoLocal`, es la que
usa el programa en marcha, y las funciones de siempre —`guardar_proyecto_en_archivo`,
`cargar_proyecto_de_archivo`, `escribir_de_forma_atomica`— siguen ahí como envoltorios suyos,
de modo que ningún otro módulo tuvo que cambiar.

**Y no es una interfaz decorativa**, que es el riesgo de aplicar esta regla por costumbre: una
abstracción con una sola implementación que nadie sustituye nunca no demuestra nada. La prueba
`el_mapa_va_y_vuelve_por_un_sistema_de_archivos_que_no_es_el_disco` pone otra implementación,
que guarda en memoria, escribe un mapa, lo recupera y comprueba que **no se ha creado ningún
archivo** y que la validación estructural sigue en el camino de carga.

La escritura real crea el temporal con exclusividad y lo identifica mediante el PID más un
contador atómico del proceso. Dos hilos pueden guardar el mismo destino simultáneamente sin
compartir temporal: ambas operaciones terminan y el renombrado deja una de las dos versiones
completa, nunca una mezcla. Si queda un temporal antiguo, la creación exclusiva avanza a otro
nombre en vez de reutilizarlo.

### La interfaz no toca el disco

La tabla de arriba dice que `ui/` no debe tocar el disco directamente. Hasta la 0.9.3 lo hacía
en seis sitios: cuatro lecturas para calcular la huella de un archivo, una creación de carpeta
y una escritura. Ahora todos pasan por `storage`, que expone `leer_bytes`,
`huella_del_archivo`, `crear_carpeta` y `escribir_de_forma_atomica`.

La escritura era la que importaba: guardaba el texto de correcciones con `fs::write`, que
vacía el destino antes de escribir. Era la única escritura no atómica del programa.

---

## 3 bis. Cómo está organizado el estado de la aplicación

`AplicacionMapaMental`, en `aplicacion.rs`, es lo único que sobrevive de un fotograma al
siguiente. `egui` dibuja en modo inmediato y reconstruye la interfaz entera entre treinta y
sesenta veces por segundo, así que nada de lo que se pinta puede guardar estado por su cuenta.
El módulo es hermano de `ui`, no descendiente suyo: dibujar una pantalla ya no concede acceso
privilegiado a las invariantes de la aplicación.

El tipo llegó a declarar **cuarenta y siete campos en una sola lista**, con el mapa del usuario,
la cámara, las ventanas, el autoguardado y el vigilante todos al mismo nivel. Ahora la raíz tiene
seis campos privados. Tres recogen estado por una razón de cambio y los otros tres son los
servicios, el envío a agentes y el ciclo de vida del archivo:

| Grupo | Qué reúne | Por qué va junto |
|---|---|---|
| `EstadoDelMapa` | Documento, nodo seleccionado e historial | Son la unidad sobre la que operan las acciones de edición, deshacer y rehacer |
| `EstadoDelLienzo` | Cámara, índice espacial, título y conexión que se están editando | Son gestos o geometría transitorios y **no forman parte del documento** |
| `EstadoDePresentacion` | Ventanas, tema, preferencias, búsqueda y borradores visibles | Solo existen para reconstruir la presentación inmediata entre fotogramas |
| `EstadoAgentes` | Vista previa de sesión, fase del envío, trabajador, detección y carpeta autorizada | Cambia como parte del único recorrido «Enviar a…» |
| `EstadoPersistencia` | Ruta, autoguardado, recuperación, vigilante, cargador, huella y conflicto externo | Forma el ciclo de vida del archivo activo |
| `ServiciosAplicacion` | Contratos ya compuestos para abrir, guardar y exportar | La interfaz pide una operación, pero no elige el adaptador que la ejecuta |

Los seis campos de la raíz son privados. Los tres agregados nuevos —mapa, lienzo y presentación—
tienen también todos sus campos privados. `ui` solo recibe consultas u operaciones nombradas. La
selección comprueba que el nodo exista, el historial no se presta para escribir y el tema solo se
consulta. Otros datos transitorios, como el borrador de una ventana o el gesto de arrastre, se
prestan de forma mutable porque `egui` necesita editarlos directamente; esa parte es una frontera
de organización, no una garantía de invariante completa. `EstadoAgentes` y `EstadoPersistencia`
siguen además dentro de `ui` y con campos `pub(crate)`.

La barrera se comprobó introduciendo un acceso directo temporal desde `ui/canvas.rs`: escribir la
selección fue rechazado con `E0616`, fabricar el agregado con `E0451` y sustituirlo con `E0277`.
Retiradas las mutaciones, las 398 pruebas existentes volvieron a pasar.

Dos detalles que no son evidentes:

- **Las banderas conservan el prefijo `modal_`** dentro de `VentanasAbiertas`, y no es
  redundancia. Cuatro pruebas de `audit_checks.rs` analizan el **texto del fuente** buscando
  esos nombres —una de ellas impide que un modal abierto deje pasar la tecla Supr hasta el mapa
  de detrás—, porque no hay forma de preguntar por los campos declarados sin macros. Un
  renombrado que las dejara sin encontrar nada no las rompería: las dejaría pasando siempre.
- **Dos `Default` están escritos a mano** y dicen por qué: las galletas de ayuda arrancan
  encendidas, y el zoom en `1.0` y no en el `0.0` que da `f32`, que no sería un mapa pequeño
  sino un lienzo en blanco y una división por cero en las conversiones de coordenadas.

## 4. Decisión central: validar en la frontera

Es la decisión arquitectónica más importante del proyecto, y nació de la auditoría de
código de 2026-08-21.

### El problema

Todo el código de recorrido del mapa —`layout`, `eliminar_nodo`, `profundidad_del_nodo`, la capa de
dibujo— asume tres cosas:

1. Que el mapa es un árbol de verdad, sin ciclos.
2. Que el nodo raíz existe.
3. Que todo identificador referenciado apunta a un nodo real.

**Nada garantizaba esas suposiciones.** Y los datos entran por tres puertas distintas,
todas ellas ajenas al control del programa:

| Frontera | De dónde vienen los datos |
|---|---|
| `storage::cargar_proyecto_de_archivo` | Un archivo del disco: puede estar corrupto, editado a mano, o llegado por correo o por una carpeta sincronizada en la nube |
| `ai_bridge::importar_de_texto_de_ia` | Texto pegado desde un modelo de IA: un tercero |
| `model::desde_escaneo_de_carpeta` | El sistema de archivos: puede tener enlaces circulares |

Cuando una suposición fallaba, el resultado no era un error: era la **congelación** de la
aplicación (bucle infinito) o su **muerte** (desbordamiento de pila). Con
`panic = "abort"` en el perfil de publicación, morir significa cerrarse de golpe sin
guardar nada.

### La solución

`Proyecto::validar_estructura()` comprueba las diez garantías que el resto del código da
por supuestas, y se invoca en las tres fronteras. Un mapa que no la supere **no entra**.

Entre esas garantías están los dos extremos de cada conexión cruzada. El lienzo conserva
comprobaciones defensivas al dibujar, pero una conexión colgante se rechaza antes de que distintos
consumidores —por ejemplo el lienzo y Mermaid— puedan interpretar de forma diferente el mismo
archivo.

```rust
// src/storage.rs
let project: Proyecto = serde_json::from_str(&content)
    .map_err(|e| AppError::formato("al interpretar el mapa mental", e))?;

// Frontera de confianza: nada de lo que hay debajo tolera un árbol inválido.
project.validar_estructura()?;
```

### Dos detalles que importan

**La validación es iterativa, no recursiva.** Usa una pila explícita en el montículo.
Si fuera recursiva, moriría por desbordamiento al intentar validar precisamente los
archivos que pretende rechazar.

**Los errores identifican el nodo concreto.** Un mensaje como «archivo inválido» no
sirve de nada; `FalloEstructural` incluye el identificador del nodo que incumple la
garantía, de modo que el usuario o una IA puedan repararlo.

### Invariante que debe mantenerse

> **Toda ruta nueva por la que un `Proyecto` entre en el programa desde el exterior debe
> llamar a `validar_estructura()` antes de devolverlo.**

Aun así, los recorridos de `layout.rs` conservan su propia defensa (marca de visitados y
límite de profundidad), porque el usuario puede modificar el mapa después de cargarlo y
la disposición se recalcula sobre ese estado ya editado.

---

## 5. Tolerancia a lo que escriben los modelos de IA

Las cinco enumeraciones del modelo (`EstadoNodo`, `PrioridadNodo`, `RolNodo`,
`TipoRelacion`, `EstadoRevision`) se deserializan **a través de `String`**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum EstadoNodo { … }
```

### Por qué

Los archivos `.mmcelt` los puede generar un modelo de IA a través del servidor MCP, y un
modelo escribe con frecuencia `"In Progress"` donde el formato espera `"EnProgreso"`.

Con enumeraciones cerradas de `serde`, **un solo valor desconocido en un solo nodo hacía
fracasar la deserialización del archivo entero**. El usuario se quedaba con un archivo
que ya no podía abrir, y el error no decía qué nodo lo causaba.

Ahora cada enumeración tiene un `desde_texto()` que acepta sinónimos en español e inglés
y degrada a la variante por defecto ante lo desconocido. **El resto del mapa se
conserva.**

La serialización sigue emitiendo el nombre canónico, así que el formato en disco no
cambia y los archivos antiguos siguen siendo válidos.

> Nota: `#[serde(other)]` no sirve aquí. Solo funciona en enumeraciones etiquetadas
> interna o adyacentemente, no en las que se serializan como una cadena simple.

### Una sola tabla por enumeración

`ai_bridge.rs` tenía sus propias tablas de equivalencias, duplicadas respecto a las del
modelo. Ahora `interpretar_estado` y sus compañeras delegan en `EstadoNodo::desde_texto`, de modo
que existe **una única** tabla por enumeración, compartida por la carga de archivos y el
importador de IA.

El servidor MCP integrado usa esas mismas tablas: al formar parte del mismo binario, no
necesita copia alguna.

---

## 6. Manejo de errores

Todas las capas internas devuelven `AppResult<T>`, es decir, `Result<T, AppError>`.

```
model / storage / ai_bridge          ui/
   ─────────────────────────►  reportar_error()
      propagan AppError          ├─► registrar() → mmcelt-errores.log
      con `?`                    └─► establecer_estado() → barra de estado
```

`AppError` tiene cinco variantes que **distinguen la causa**: `Lectura`, `Escritura`,
`Formato`, `EstructuraInvalida` e `ImportacionIa`. Antes todo era `Result<T, String>`, y
quien recibía el error no podía distinguir un archivo inexistente de un JSON corrupto,
porque ambos llegaban como texto ya formateado.

**`AplicacionMapaMental::reportar_error` es el único punto donde un error se convierte en texto
visible.** Las capas internas se limitan a propagarlo.

Cada mensaje incluye **qué puede hacer el usuario**:

> El mapa mental está dañado y no se puede abrir: el mapa contiene un ciclo: se llega
> dos veces al nodo 7f3a…. Se ha impedido su carga para no bloquear la aplicación.

El registro en `mmcelt-errores.log` es *best-effort*: si no se puede escribir, se omite
sin propagar el fallo. Registrar un error nunca debe provocar otro.

---

## 7. Recorridos iterativos

`layout.rs`, `model.rs` y `ui/canvas.rs` recorren el árbol **sin recursión**, con pila
explícita y marca de visitados. Afecta a seis funciones: `calcular_altura_de_la_rama`,
`disponer_rama`, `arcos_requeridos`, `recoger_descendientes`, `profundidad_del_nodo` y
`dibujar_conexiones_de_la_rama`.

La última era la excepción: dibujaba las curvas de la jerarquía
descendiendo recursivamente y sin marcar por dónde había pasado.

La razón es directa: las versiones recursivas mataban el proceso ante un mapa con un
ciclo o con una rama muy larga.

`calcular_altura_de_la_rama` es la más interesante. Necesita medir los hijos **antes** que
el padre, porque la altura de un nodo depende de la suma de las de su descendencia
(recorrido en postorden). La versión iterativa lo consigue apilando cada nodo dos veces,
con una marca que indica si sus hijos ya se encolaron:

```rust
if !hijos_listos {
    pila.push((id_actual, true));          // volver a este nodo después…
    for &id_hijo in &nodo.children {
        pila.push((id_hijo, false));       // …pero primero, sus hijos
    }
    continue;
}
// Segunda visita: todos los hijos tienen ya su altura registrada.
```

---

## 8. Seguridad del servidor MCP

El servidor MCP es el componente de **mayor privilegio efectivo** del sistema: concede a
un agente de IA autónomo la capacidad de escribir archivos en el disco del usuario.

Aplica cuatro restricciones:

| Restricción | Implementación |
|---|---|
| Carpeta obligatoria | Sin `MMCELT_WORKSPACE` definida, `espacio_de_trabajo()` devuelve error y **no se opera con ningún archivo** |
| Espacio de trabajo acotado | `validar_ruta()` encadena seis comprobaciones, cada una en su propia función: ruta vacía, `..` (mirando **las dos barras**, porque en Linux la invertida no separa), dos puntos dentro de un nombre (flujos alternativos de NTFS), nombres que acaban en punto o espacio (ingobernables en Windows), contención dentro de la raíz una vez resuelta con `Path::canonicalize`, y extensión admitida para ese uso |
| Extensiones permitidas | Solo `.mmcelt`, `.json` y `.md` |
| Sin sobrescritura silenciosa | El archivo previo se copia con marca de tiempo antes de pisarlo. La única excepción está escrita en el código —la sincronización de progreso— y no se puede pedir desde fuera |

Tres detalles de la implementación que no son evidentes:

- **`Path::starts_with` compara por componentes, no por texto.** La comparación textual tiene
  un fallo clásico —`/datos_privados` empieza por `/datos` y pasaría por interior suyo— del
  que este método no adolece: `/datos_privados` no empieza por el componente `datos`.
- **El filtro de `..` no es redundante con `canonicalize`.** Este último solo resuelve rutas
  que ya existen, y al crear un mapa lo normal es que no exista todavía.
- **La carpeta es obligatoria a propósito.** Antes, si la variable faltaba, el espacio pasaba
  a ser el directorio desde el que se hubiera arrancado el proceso. La ausencia de
  configuración se convertía así en ausencia de límite, que es lo contrario de lo que la
  ventana de conexión le promete al usuario.

La copia lleva marca de tiempo por la misma razón: con un nombre fijo, la segunda escritura
del agente sobre el mismo archivo machacaba la copia del trabajo del usuario y dejaba en su
lugar el primer intento del propio agente. Vive en `respaldo.rs`, un único sitio que usan
tanto el servidor como el registro de agentes.

Aquí no hace falta un atacante para que haya daño: basta una ruta alucinada por el
modelo para destruir un archivo del usuario.

---

## 9. Una sola implementación de cada cosa

`ai_export.rs` es el **único** sitio donde se genera el documento Markdown para IA, y lo
usan tanto la aplicación como el servidor MCP.

No siempre fue así. Hubo un servidor MCP escrito aparte, en otro lenguaje, que
reimplementaba el modelo, la validación y la exportación. La auditoría de código lo
recogió como hallazgo B-3 (duplicación), y ya entonces había divergido: la numeración de
secciones saltaba de la 4 a la 5 en ambas copias, huella de un copiar y pegar.

Al integrar el servidor en el ejecutable, esa duplicación desapareció por construcción:
no hay dos implementaciones que mantener sincronizadas porque solo hay una.

> Queda el salto de numeración en las secciones del documento exportado. Se conserva a
> propósito para no romper los mapas y prompts que ya se apoyan en esos números.

---

## 10. El formato `.mmcelt`

JSON con los nodos en un **mapa plano** indexado por UUID; la jerarquía se expresa
mediante las listas `children` y los campos `parent_id`.

```json
{
  "id": "…", "title": "Mi proyecto",
  "creator_vision": "…", "project_goals": "…",
  "root_id": "uuid-de-la-raíz",
  "nodes": {
    "uuid-de-la-raíz": {
      "id": "uuid-de-la-raíz",
      "parent_id": null,
      "children": ["uuid-hijo"],
      "title": "Idea central",
      "status": "Idea", "priority": "Media", "role": "IdeaCentral",
      "review_status": "PendienteRevision",
      "pos": [0.0, 0.0], "collapsed": false
    }
  },
  "connections": [],
  "layout_mode": "HorizontalTree"
}
```

### Por qué un mapa plano y no anidado

Permite que las **conexiones cruzadas** (relaciones entre nodos de ramas distintas)
referencien cualquier nodo por identificador, algo imposible en una estructura anidada.

**Tiene una consecuencia de seguridad que conviene recordar**: el límite de anidamiento
de `serde_json` (128 niveles) **no protege este formato**. Un documento de profundidad
sintáctica 3 puede describir un árbol lógico de cientos de miles de niveles. De ahí que
`PROFUNDIDAD_MAXIMA` deba comprobarse explícitamente en la validación.

### Campos con valor por defecto

`file_path`, `review_status` y `correction_feedback` llevan `#[serde(default)]`, así que los
archivos creados antes de que existieran siguen cargándose.

Por el mismo motivo, quitar un campo no rompe los archivos ya guardados: `serde` descarta lo
que no reconoce. Es lo que ocurrió con `color_override` y `custom_width`, dos campos que se
guardaban en cada nodo y que nada leía ni escribía.

---

## 11. Elecciones que conviene conocer

### `panic = "abort"` en el perfil de publicación

Reduce el tamaño del binario y mejora el rendimiento, pero **no hay desenrollado de
pila**: un panic cierra el proceso de golpe, sin destructores.

Se mantiene deliberadamente. Lo peligroso no era la opción, sino que hubiera panics
alcanzables desde un archivo de entrada. Eliminados esos con la validación, la opción
deja de ser un multiplicador de daño.

> Al añadir código nuevo conviene recordarlo: aquí un `unwrap()` sobre datos externos no
> produce un mensaje de error, sino la pérdida del trabajo del usuario.

### `egui` en modo inmediato

La interfaz se reconstruye entera en cada fotograma desde el estado actual; no hay árbol
de componentes ni estado duplicado en la vista.

Implicación práctica: **las funciones de dibujo se ejecutan decenas de veces por
segundo**. Un nodo puede desaparecer entre dos fotogramas, así que toda consulta al mapa
debe contemplar la ausencia (`if let Some(...)`), nunca indexar directamente.

### Sin dependencias de red

No hay `reqwest`, `hyper` ni equivalentes. La aplicación **nunca se conecta a internet**,
no maneja claves de API y no ejecuta procesos externos.

Es una decisión de diseño, no un descuido: elimina de raíz la filtración de credenciales,
la inyección en peticiones y la exfiltración de datos.

> **Si algún día se añade una integración directa con un modelo, todo el análisis de
> seguridad del proyecto cambia** y hay que rehacerlo.

---

## 12. Autoguardado: qué es y dónde escribe

El módulo `autoguardado.rs` guarda cada dos minutos —o cada cuanto haya elegido el
usuario— una copia del proyecto si ha
cambiado, y al arrancar ofrece recuperarla si la sesión anterior terminó mal.

### La distinción que gobierna el diseño

**Autoguardar no es guardar.** Es una red de seguridad, y de ahí salen sus dos reglas:

1. **No interrumpe.** Un diálogo modal cada dos minutos sería insufrible, así que el
   autoguardado no puede preguntar dónde escribir: necesita una ubicación conocida de
   antemano.
2. **No escribe donde sorprenda.** Nunca toca el archivo del usuario ni deja artefactos
   junto a él.

### Por qué el directorio de datos y no la carpeta de la aplicación

Escribir junto al ejecutable parece lo cómodo, pero falla en el caso más habitual: si el
binario está en `Program Files` o en `/usr/local/bin`, **el proceso no tiene permiso de
escritura**, y el autoguardado fallaría en silencio justo cuando más falta hace. Además,
nadie busca sus datos en la carpeta del programa.

| Sistema | Ubicación |
|---|---|
| Windows | `%APPDATA%\MMCelt\recuperacion\` |
| macOS | `~/Library/Application Support/MMCelt/recuperacion/` |
| Linux | `$XDG_DATA_HOME/mmcelt/recuperacion/` o `~/.local/share/mmcelt/…` |

La variable `MMCELT_DATOS` permite reubicarlo, lo que sirve para instalaciones portables
y para aislar las pruebas.

### La huella: qué cuenta como «ha cambiado»

Reescribir la copia cada dos minutos aunque nadie toque nada sería un desperdicio, así
que se compara una huella del contenido. Lo que entra en ella importa:

| Entra | No entra |
|---|---|
| Título, metadatos, autor | **Posiciones de los nodos** |
| Título, notas, etiquetas, estado, prioridad, rol de cada nodo | **`updated_at`** |
| Estructura (`children`) y conexiones | Nivel de zoom y desplazamiento |

Las posiciones quedan fuera a propósito: cambian con solo recalcular la disposición, y
`updated_at` cambia sola. Incluirlas haría que la huella difiriera siempre y el
autoguardado escribiría en cada intervalo aunque el usuario solo estuviera leyendo.

### Detalle que es fácil pasar por alto

`egui` solo repinta cuando hay actividad. Sin `ctx.request_repaint_after(…)`, una ventana
inactiva dejaría de recibir fotogramas y **el temporizador no se dispararía**: justo el
caso en el que el autoguardado más falta hace, con el usuario ausente.

### La recuperación no se aplica sola

Al arrancar se busca la copia, pero solo se **ofrece**. El usuario pudo haber cerrado a
propósito sin guardar, y devolverle sin preguntar justo lo que descartó sería peor que no
ofrecer nada.

Una copia que no supere `validar_estructura` se descarta en silencio: restaurar un mapa
con un ciclo reintroduciría el problema que el resto del programa se esfuerza en rechazar.

### Dónde se propone guardar

`Proyecto::carpeta_proyecto_sugerida()` deduce la carpeta del proyecto de código buscando
el prefijo común de las rutas `file_path` de los nodos. Los mapas del escáner de
repositorios y los que genera una IA sobre un código concreto las llevan.

Se usa para **proponer** el destino en el diálogo de guardar, nunca para escribir sin
preguntar: hacerlo podría colar archivos dentro de un repositorio ajeno.

Solo se sugiere si la carpeta existe de verdad (una IA puede inventarse rutas) y si el
prefijo común tiene más de un componente, ya que `C:\` o `/` no son «la carpeta del
proyecto».

> En el flujo MCP este problema no se plantea: el agente indica la ruta explícitamente y
> el mapa se escribe en la carpeta del proyecto, acotada por `MMCELT_WORKSPACE`.

---

## 13. Escala de la interfaz y preferencias

El módulo `preferencias.rs` conserva entre sesiones dos ajustes de presentación: la escala
de la interfaz y el tema visual.

### Qué había y qué faltaba

`egui` trae desde el principio el escalado global de la interfaz con `Ctrl` `+` / `-` /
`0` (`Options::zoom_with_keyboard`, activo por defecto). No hubo que programar el
escalado.

Lo que faltaba eran dos cosas:

1. **Persistencia.** `egui` no guarda el `zoom_factor` entre sesiones —su propio código
   lo marca como pendiente—, así que el ajuste volvía a 1,0 en cada arranque. Para quien
   trabaja en un televisor, eso convierte una función útil en una molestia diaria.
2. **Descubribilidad.** La función existía pero no estaba documentada ni visible en
   ninguna parte, de modo que en la práctica era como si no existiera.

### Cómo se sincroniza

`AplicacionMapaMental::sincronizar_escala_interfaz` se ejecuta al principio de cada fotograma y
hace tres cosas distintas según el momento:

| Momento | Qué hace |
|---|---|
| Primer fotograma | Aplica la escala guardada. No puede hacerse en el constructor: el contexto aún no conoce el monitor |
| Primer arranque | Propone una escala según la resolución y marca la sugerencia como mostrada |
| Fotogramas siguientes | Detecta si el factor ya no coincide con el guardado —señal de que el usuario usó los atajos— y persiste el valor nuevo |

Ese tercer punto es la clave: `egui` aplica los atajos por su cuenta al final de cada
fotograma, así que la aplicación no los intercepta, sino que **observa el resultado**.
Intentar aplicar la escala guardada en cada fotograma pisaría los cambios del usuario.

### El límite de lo que se puede deducir

| Dato | ¿Observable? |
|---|---|
| Resolución del monitor | ✅ Sí |
| Escala de pantalla del sistema (DPI) | ✅ Sí |
| Tamaño físico en pulgadas | ⚠️ No de forma fiable |
| **Distancia de visionado** | ❌ **No** |

El último es el que manda, y no hay forma de conocerlo. Un televisor de 42 pulgadas a tres
metros y un monitor de 24 a medio metro presentan al sistema **exactamente los mismos
datos**, y sin embargo necesitan tamaños de letra muy distintos.

Por eso `escala_sugerida` es deliberadamente conservadora: solo corrige el caso inequívoco
—resolución muy alta con el sistema sin escalar, donde la interfaz sale diminuta para
cualquiera— y respeta el escalado que el sistema operativo ya aplique, para no acumular
factores. Todo lo demás se deja en manos del usuario, y se recuerda.

### Robustez

Unas preferencias ilegibles o con valores absurdos no impiden arrancar: se parte de los
valores por defecto y la escala se recorta al rango admitido. Sin ese recorte, un archivo
editado a mano con una escala de 500 dejaría la interfaz inutilizable, y en el caso
extremo impediría llegar al menú para corregirlo.

Guardar es *best-effort*: no poder recordar una preferencia es una molestia, no un error
que merezca interrumpir al usuario.

---

## 14. El ejecutable como servidor MCP

`mmcelt --mcp-server` convierte el propio binario en servidor del Model Context Protocol.
La aplicación y el servidor son **el mismo archivo**, distinguidos por un argumento.

El saludo declara como versión del servidor la versión real de MMCelt y negocia la versión MCP
`2024-11-05`. Como actualmente solo se implementa esa versión, una petición desconocida recibe la
única compatible en vez de hacer que el servidor finja entender un contrato futuro.

### Por qué el servidor vive dentro del ejecutable

Hubo un servidor equivalente escrito aparte, en otro lenguaje, que se eliminó del
proyecto. Tenía dos problemas:

1. **Exigía tener ese lenguaje instalado**, lo que rompe el caso que más importa:
   **pasarle la aplicación a otra persona**. Si alguien recibe MMCelt como programa
   portable y le falta ese requisito, el servidor no arranca y la conexión falla, por muy
   cómoda que sea la interfaz que se la ofrezca.
2. **Era una implementación paralela**: reimplementaba el modelo, la validación y la
   exportación, con la garantía de que antes o después divergirían. La auditoría ya lo
   recogió como hallazgo B-3.

| | Servidor aparte | Servidor integrado |
|---|---|---|
| Requiere otro tiempo de ejecución | Sí | **No** |
| Sirve para una app portable | No | **Sí** |
| Comparte la validación del modelo | No, la reimplementa | **Sí, es el mismo código** |

Esa última fila importa tanto como la portabilidad: al reutilizar `model`, `storage` y
`ai_export`, el servidor integrado **no puede divergir** de lo que hace la aplicación.

### Seguridad

Se concede a un agente autónomo la capacidad de escribir en el disco del usuario, así que
se aplican cuatro restricciones:

1. **Espacio de trabajo acotado** por `MMCELT_WORKSPACE`. Si la variable no está definida, el
   servidor no opera con ningún archivo: un límite que se abre solo cuando falta su
   configuración no es un límite.
2. **Extensiones por operación.** Un mapa se escribe como `.mmcelt` o `.json`, y un documento
   como `.md`. No hay una lista común: cuando la había, el exportador de Markdown podía
   escribir sobre un mapa y convertirlo en texto, que era la forma de rodear el punto 4. Al
   leer no se exige extensión, porque el agente puede querer abrir un archivo con cualquier
   nombre.
3. **Copia con marca de tiempo** antes de sobrescribir, para que las sucesivas no se pisen
   entre sí. Una sola excepción, `mmcelt_sync_ai_progress`, que se llama sin parar y
   llenaría la carpeta; está declarada en el código, en `PoliticaDeCopia`, y no se puede
   pedir desde fuera.

   Hubo una segunda, el argumento `overwrite`, ya retirado: se justificaba como
   «una decisión explícita de quien llama», y quien llama es el modelo. Un agente se eximía
   a sí mismo de dejar copia con solo pedirlo.
4. **Lo que no ha escrito la IA no se pisa.** Crear un mapa encima de un archivo cuyos nodos
   no sean todos trabajo de la IA se rechaza. Sin esto, la valla de la sincronización —que
   no revive descartados ni firma aprobaciones— se rodeaba con solo llamar a la herramienta
   de al lado. Se mira el estado de revisión de cada nodo, y no si hay correcciones: un mapa
   escrito a mano en la aplicación no tiene ninguna marca de supervisión, y era justo el
   caso que quedaba desprotegido.

---

## 15. Conexión con agentes de IA

`conectores.rs` detecta los agentes instalados y registra MMCelt en ellos.
`ui/conexiones_modal.rs` lo presenta como una ventana con un botón por agente.

### Composición y consentimiento de las instrucciones

`sesiones_agentes.rs` es el único módulo que compone una sesión. Recibe cuatro bloques tipados,
valida tamaños y caracteres de control, incorpora solo las fuentes elegidas y devuelve un
`PromptSesionCompuesto` inmutable con SHA-256. La ventana de proyecto, el modal de confirmación y
el repositorio de sesiones consumen ese mismo valor: ninguno vuelve a concatenar cadenas.

`configuracion_agente_proyecto.rs` conserva en `.mmcelt/configuracion-agente.json` el idioma del
documento y el consentimiento durable de cada fuente nativa. La aceptación guarda nombre,
contenido y huella; toda lectura vuelve a resolver la ruta mediante `ContextoProyecto`, por lo que
un enlace simbólico exterior no puede ganar confianza. La escritura usa el reemplazo atómico común
y una configuración corrupta produce un error tipado.

`ui/proyecto_ia_modal.rs` solo dibuja borradores y devuelve acciones semánticas. Cambiar el idioma,
guardar reglas o aceptar una fuente se ejecuta después del callback de `egui`, fuera del préstamo
visual. El selector genera `CambiarIdiomaDocumento` y persiste solo esa preferencia de inmediato;
`GuardarReglas` sigue siendo la única acción que escribe el perfil común. Ambos controles ocupan
un pie con altura reservada fuera del contenido de las cuatro pestañas, que conserva su propio
desplazamiento. La misma separación se mantiene en `ui/sesion_agente_modal.rs`: el botón confirma
una `ConfirmacionSesion` con la revisión del mapa y las huellas externas. Antes de encolar el
trabajo, `aplicacion.rs` relee esas entradas; cualquier diferencia falla cerrada con
`ConfirmacionCaducada`.

Las dos ventanas conservan el contexto con reglas distintas y deliberadas. «Proyecto e
instrucciones» es un editor del **proyecto activo**: si se abre otro mapa mientras permanece
visible, reconstruye sus borradores para la nueva raíz y todo guardado o consentimiento se resuelve
desde la misma `raiz_proyecto` que enseña en pantalla. La ventana de envío, en cambio, captura un
`ContextoProyecto` al prepararse y lo mantiene hasta confirmar o cancelar: representa una sesión
concreta ya compuesta, no un editor que deba seguir futuros cambios de mapa. Esta diferencia evita
tanto escribir o consentir en una carpeta distinta de la mostrada como alterar silenciosamente una
sesión que la persona ya estaba revisando.

### El mapa que vuelve solo (`vigilante.rs`)

El servidor MCP ya permitía que un agente **escribiera** el mapa; lo que faltaba era que la
ventana abierta se enterase. Eso es `vigilante.rs`, y son 120 líneas.

**No hay sondeo.** La aplicación es reactiva —`ctx.request_repaint_after` la despierta cada dos
minutos para el autoguardado, no en bucle—, así que preguntar «¿ha cambiado?» cada pocos
segundos habría significado despertarla constantemente para nada. En su lugar, la caja `notify`
usa el aviso del propio sistema operativo y un hilo llama a `ctx.request_repaint()` cuando llega
algo. En reposo el coste es cero.

**Se vigila el directorio, no el archivo.** No es un detalle menor: un guardado atómico escribe
un temporal y lo renombra encima del destino, así que quien vigile el archivo se pierde
exactamente el evento que le importa.

**Tres filtros antes de tocar nada**, en este orden:

1. **¿Lo he escrito yo?** Se compara una huella del contenido con la de la última escritura
   propia. Se compara el **contenido y no la marca de tiempo**: la resolución del reloj del
   sistema de archivos no es infinita y el agente puede escribir en el mismo instante que el
   autoguardado. Sin este filtro, el autoguardado dispararía su propia recarga cada dos minutos.
2. **¿Está entero?** Si no deserializa como JSON válido y no supera `validar_estructura`, se
   descarta en silencio y se espera al siguiente aviso. Un agente puede escribir en varios
   pasos, y un error en pantalla porque el archivo estaba a medias sería ruido.
3. **¿Tiene el usuario trabajo sin guardar?** Si lo tiene, **el mapa en memoria no se toca**:
   se avisa y decide la persona. Si no, se recarga —dejando antes una copia de recuperación—.

### Por qué no hay motor de fusión

El primer diseño contemplaba fusionar nodo a nodo cuando el usuario y la IA tocaran el mismo
mapa. **Se descartó, y la razón es de producto, no técnica**: el flujo es por turnos. Cuando
mandas el mapa con «Enviar a…» estás mirando el terminal del modelo, que es donde le das las
instrucciones, no editando el mapa.

Esa observación convirtió la pieza más cara del diseño —un algoritmo de fusión, el código más
difícil de probar de los tres— en la comprobación de un booleano. Queda escrito aquí porque es
el tipo de decisión que alguien intentará «mejorar» dentro de un año.

### Identidad del proyecto y canal de vuelta

`proyecto_trabajo.rs` separa la carpeta física de su identidad lógica. La conexión explícita crea
`.mmcelt/configuracion.json` mediante la persistencia atómica común; abrirla otra vez conserva el
UUID, y encontrarla bajo otra raíz produce `CargaProyecto::Trasladado` en vez de aceptar el cambio
silenciosamente. `ContextoProyecto` es la única puerta para resolver rutas relativas y comprueba
también enlaces existentes antes de autorizar una escritura.

`devolucion_agentes.rs` modela el contrato neutral `ReciboDevolucion`. El servidor MCP lo prepara
antes de guardar y lo publica después del mapa en `.mmcelt/devolucion.json`; creación y
sincronización comparten validación, formato y escritura atómica. Los proyectos sin configuración
mantienen la compatibilidad anterior y no generan recibo hasta una conexión explícita. El recibo
es trazabilidad durable para la persona y para el cliente MCP, no una credencial de autorización:
la aplicación observa y valida el mapa real, porque un recibo anterior podría seguir en disco
después de una edición distinta. `ReciboDevolucion::validar` comprueba que identidad, raíz y ruta
sean coherentes antes de guardar esa trazabilidad; que devuelva una ruta no concede permiso para
escribir, no valida el contenido del mapa y no dispara una recarga.

La carpeta `.mmcelt` es un límite interno. Ninguna ruta suministrada a una herramienta MCP puede
entrar en ella, aunque permanezca dentro del espacio de trabajo general. Solo los recorridos
internos que crean identidad, sesiones y recibos pueden escribir ahí.

### Composición de los servicios de persistencia

`proyectos.rs` expone tres contratos pequeños: `RepositorioMapas`, `PreparadorCarpetas` y
`RepositorioDocumentos`. `servicios_aplicacion.rs` los compone una sola vez para la interfaz y es
el único punto de esa capa que selecciona `RepositorioLocal`. El trabajador de envío realiza una
segunda composición deliberada porque posee sus dependencias y las mueve a otro hilo; compartir
el contenedor de la interfaz introduciría acoplamiento y requisitos de sincronización sin aportar
comportamiento.

El módulo `servicios_aplicacion` es ahora hijo privado de `aplicacion`, y su `RepositorioLocal`
también es privado. La interfaz no puede construirlo, invocar su fábrica ni crearle un alias:
las tres mutaciones fueron rechazadas por el compilador con `E0603` o `E0624`. Al existir esta
barrera real se retiró la antigua búsqueda textual, que podía esquivarse con un alias o con una
pantalla nueva no incluida en su lista. El recorrido
`el_envio_es_un_caso_de_uso_independiente_de_la_interfaz` aporta la prueba de comportamiento:
sustituye persistencia, expedientes y lanzador y comprueba el orden y el corte ante cada error.

El trabajador en segundo plano llama a las mismas funciones seguras de `storage` que el adaptador
local. Hoy no duplica ninguna política, pero son dos puntos que habrá que cambiar juntos si el
guardado local adquiere una operación adicional; este límite queda registrado para no convertir
la separación entre hilos en una falsa promesa de composición única.

El estado reactivo también está dividido por motivo de cambio. `EstadoAgentes` conserva sesión,
detección y trabajador de envío; `EstadoPersistencia` conserva ruta, recuperación, autoguardado,
huellas y vigilancia. `AplicacionMapaMental` continúa siendo el agregado raíz que exige `eframe`,
pero ya no inicializa ni enumera individualmente esos trece detalles operativos.

La orquestación de la vigilancia vive en `vigilancia_mapa.rs`, que recibe únicamente
`EstadoPersistencia`: inicia o detiene el observador, solicita las lecturas en segundo plano y
clasifica cada resultado como ignorado, retenido por cambios locales o apto para sustituir el mapa.
No recibe `&mut AplicacionMapaMental`. La aplicación conserva solo el adaptador de presentación:
guarda la recuperación antes de sustituir, invalida el índice espacial, reinicia el historial,
ajusta la selección y muestra el estado correspondiente. Esta frontera evita que una decisión de
archivo vuelva a mezclarse con el objeto raíz y permite probar la clasificación sin conceder al
módulo acceso a toda la interfaz.

La orquestación del envío a agentes de IA vive en `envio_agente_comando.rs`, que separa la
preparación de borradores, el diálogo de selección de rutas, la confirmación de envíos y la
atención de respuestas en segundo plano fuera de `AplicacionMapaMental`. Las funciones del módulo
reciben exclusivamente las estructuras acotadas de dominio (`EstadoAgentes`, `EstadoPersistencia`,
`Proyecto`, `ServiciosAplicacion`) en lugar de `&mut AplicacionMapaMental`. La aplicación conserva
métodos de fachada para coordinar la interacción con la interfaz (`preparar_sesion_agente`,
`cancelar_sesion_agente`, `confirmar_sesion_agente`, `exportar_markdown_dialogo`) y despacha la
reacción del trabajador asíncrono en cada fotograma mediante `procesar_eventos_de_envio`.
Los módulos de interfaz (`toolbar.rs` y `sesion_agente_modal.rs`) se comunican únicamente a través
de la aplicación sin acoplarse directamente a `envio_agente_comando`, manteniendo intactos los
trinquetes de acoplamiento de la regla P6.

El ciclo de persistencia del mapa activo vive en `persistencia_mapa_comando.rs`, submódulo privado
de `aplicacion`. Recibe `EstadoPersistencia`, `Proyecto` y, solo para el guardado, los
`ServiciosAplicacion`; concentra autoguardado, recuperación, registro de huella y selección de la
carpeta sugerida sin conocer `AplicacionMapaMental`. La aplicación conserva adaptadores de
presentación para abrir diálogos, traducir resultados a avisos y aplicar el proyecto recuperado.
Este registro privado evita añadir una dependencia transversal a la raíz: P6 mantiene los quince
módulos alcanzados por `aplicacion.rs` y reduce su tipo mayor de 73 a 72 métodos.

### Sesión supervisada y consola neutral

`sesiones_agentes.rs` compone cuatro bloques —contrato, reglas comunes, contexto y encargo— y
persiste el prompt aprobado en `.mmcelt/sesiones/<sesión>/inicio.md`, junto a metadatos sin
credenciales. `lanzador_agentes.rs` traduce únicamente Claude Code, Codex CLI y Gemini CLI a una
orden con programa, argumentos y directorio separados. El prompt completo no viaja en `argv`:
solo una referencia relativa al expediente.

La interfaz conserva un `EditorSesionAgente` en memoria. Elegir o cancelar no guarda, exporta ni
vigila. Al confirmar, la secuencia es guardar mapa → exportar `_AI.md` → persistir expediente →
lanzar CLI → vigilar. El ejecutor es inyectable para comprobar ese límite sin abrir ventanas.

El adaptador de OpenAI no confunde superficies. Codex CLI, la aplicación Codex y su extensión
pueden compartir `~/.codex/config.toml` cuando se ejecutan en el mismo host; una conversación de
ChatGPT web no lee ese archivo local. Por tanto, el núcleo solo modela capacidades comprobables
—`ejecutable` y `conectado_por_mcp`— y no deduce acceso al disco a partir del nombre «ChatGPT» o
«Codex». La interfaz puede abrir la CLI y preparar un expediente, pero no inyecta mensajes en una
tarea gráfica ya abierta.

Un destino solo MCP sigue el mismo camino sin el paso de proceso: deja el expediente `Preparado`
y activa vigilancia. Esto permite Antigravity sin confundirlo con Gemini CLI. Si una carpeta
anterior carece de identidad, la vista usa un `ContextoProyecto` efímero sin escribir; la
identidad durable nace únicamente al confirmar.

### Por qué Windows ya no abre una consola vacía

El mismo ejecutable sirve como aplicación gráfica, como servidor MCP y para consultar
`--version`. La compilación distribuida declara el subsistema gráfico de Windows: al abrir MMCelt
desde el Explorador solo aparece la ventana de la aplicación. Las compilaciones de depuración
conservan el subsistema de consola para que los diagnósticos sigan visibles durante el desarrollo.

Antes de imprimir `--version`, `consola_windows.rs` intenta asociarse a la consola del proceso
padre. Si la salida ya es una tubería redirigida, Windows la conserva; si el programa se abrió
desde un lugar sin consola, el intento falla en silencio porque es un caso normal. El modo MCP no
usa esta adaptación: el cliente le entrega directamente sus tuberías de entrada y salida.

La aceptación no deduce el resultado del código fuente. Lee la cabecera PE del ejecutable release
—el campo `Subsystem` debe valer 2—, captura la salida real de `--version` y negocia una sesión MCP
completa con el mismo archivo.

### Por qué está dentro de la aplicación y no solo en un script

La conexión se hace desde la propia aplicación. Hubo scripts que la repetían desde la
terminal, y se retiraron: mantenían una segunda copia de la misma lógica que acabó
desincronizándose del programa.
Siguen siendo útiles para instalar sin abrir la interfaz, pero no resuelven el caso
principal:

- Quien recibe MMCelt de un conocido **no va a abrir PowerShell**.
- Si instala un agente nuevo, tendría que **acordarse** de volver a ejecutar el script.

Dentro de la aplicación, la detección ocurre **al abrir la ventana**, así que lo instalado
después aparece solo.

### Dónde guarda su configuración cada agente

No siempre donde uno esperaría. Estas rutas se comprobaron sobre instalaciones reales, no
se dedujeron:

| Agente | Ubicación | Formato |
|---|---|---|
| Claude Code | `~/.claude.json` | JSON |
| Claude Desktop | Varía según el sistema | JSON |
| Antigravity | `~/.gemini/antigravity/mcp_config.json` — **no** en `~/.antigravity` | JSON |
| Cursor | `~/.cursor/mcp.json` | JSON |
| Windsurf | `~/.codeium/windsurf/mcp_config.json` | JSON |
| Codex CLI | `~/.codex/config.toml` | **TOML** |

Dos detalles que cuestan tiempo si no se saben: Antigravity crea su `mcp_config.json` con
**0 bytes**, y el `.claude.json` de Claude Code guarda además el historial y las
preferencias del usuario.

### Por qué no se usa un analizador de TOML

Codex es el único que no usa JSON. Añadir una dependencia de TOML para una operación tan
acotada —comprobar si una sección existe y, si no, añadirla al final— no compensa, así que
se trabaja sobre el texto. El resto del archivo no se toca.

### Precauciones

Estos archivos pertenecen a otros programas y suelen contener ajustes que costó trabajo
dejar bien:

- **Copia de seguridad** con marca de tiempo antes de modificar.
- **Se conserva todo lo demás**: otros servidores MCP y el resto de claves.
- Un archivo ilegible **se deja intacto** y se informa, en lugar de sobrescribirlo.
  Podría ser la configuración completa del usuario, dañada por otra causa y aún
  recuperable a mano.

---

## 16. El tema viste toda la ventana

### El problema

`ThemeConfig` (en `theme.rs`) definía la paleta del mapa, y el lienzo la usaba al
dibujarse. Pero la barra de menús, el panel lateral, la barra de estado, los botones, los
desplegables, los campos de texto y los cuadros de diálogo no los dibuja la aplicación:
los dibuja `egui`, con el estilo que guarda en su contexto.

Ese estilo nunca se configuró. `egui` usaba el suyo por omisión, que es **oscuro y fijo**,
de modo que:

- el **tema oscuro** parecía correcto por casualidad, porque su paleta se parecía al
  estilo por omisión;
- el **tema claro** salía partido en dos: un lienzo claro rodeado de barras grises
  oscuras;
- el **alto contraste** perdía su razón de ser en todo lo que no fuera el mapa.

Ningún ajuste de los colores de `ThemeConfig` podía arreglarlo, porque el problema no
estaba en los colores sino en que la mitad de la ventana no los leía.

### La solución

`ThemeConfig::visuals()` traduce la paleta a los `egui::Visuals`: fondos de panel y
ventana, los cinco estados de los controles, el color de selección, los bordes, los
redondeos y las sombras.

Se instala en dos puntos, y solo en esos dos:

| Punto | Cuándo | Por qué |
|---|---|---|
| `AplicacionMapaMental::new` | Al arrancar, antes del primer fotograma | El tema guardado en las preferencias tiene que estar puesto antes de dibujar nada |
| `AplicacionMapaMental::establecer_tema` | Al elegir otro tema en el menú | Cambiar la paleta del lienzo sin reinstalar el estilo dejaría la ventana a medio pintar |

Aplicarlo en cada fotograma habría sido más simple de escribir, pero reconstruye el estilo
sesenta veces por segundo para un dato que cambia cuando el usuario abre un menú. Por eso
`establecer_tema` recibe el `&egui::Context`: es lo que permite hacerlo una sola vez, en el
momento en que de verdad cambia algo.

### Contraste como prueba, no como criterio

Las paletas se comprueban solas. `theme.rs` incluye pruebas que calculan la relación de
contraste de la norma WCAG 2.1 —luminancia relativa sobre canales linealizados— y exigen:

| Elemento | Mínimo |
|---|---|
| Texto principal y de apoyo, sobre cualquier superficie | 4,5:1 |
| Texto atenuado, sobre las cinco superficies donde se dibuja | 4,5:1 |
| Acierto, aviso y peligro, sobre los cinco estados de un control | 4,5:1 |
| Colores de rama, líneas, bordes y símbolos sobre una rama | 3:1 |
| Lienzo contra panel, y nodo contra nodo raíz | 1,12:1 (que se distingan) |
| Retícula contra lienzo | Entre 1,05:1 y 2,2:1 (que se vea, sin competir) |

Al texto atenuado se le exige el umbral del **texto** y no el de un elemento gráfico, aunque
el nombre sugiera lo contrario: no es decoración, es la barra de estado, los encabezados de
los menús y los recuentos de las etiquetas. Y hay una prueba aparte que comprueba que los
tres niveles de texto conservan su jerarquía, para que subir el contraste del atenuado no
acabe igualándolo con el de apoyo.

Ese último límite superior no es decorativo: procede de un defecto real, la retícula que
se pintaba blanca por un color premultiplicado mal construido. La prueba impide que vuelva.

También se comprueba que ningún color de la paleta lleve transparencia, salvo la sombra.
La misma trampa de la premultiplicación había reaparecido en `connection_line`, y la
prueba la detectó.

---

## 17. El icono, en dos sitios a la vez

Windows saca el icono de un programa de dos fuentes distintas, y hay que alimentar las dos:

| Fuente | Dónde se ve | Cómo se alimenta |
|---|---|---|
| El icono de la **ventana**, que fija el programa al arrancar | Barra de título, barra de tareas, `Alt`+`Tab` | `main.rs` pasa `IconData` al constructor de la ventana |
| El **recurso del ejecutable**, incrustado al compilar | Explorador de archivos, accesos directos, menú de inicio | `build.rs` incrusta `mmcelt.ico` con `winresource` |

Poner solo el primero deja el `.exe` con el icono blanco genérico, que es justo donde más
se nota. Poner solo el segundo funciona en Windows pero no en otros sistemas.

**Por qué los píxeles van en crudo.** `main.rs` incrusta `mmcelt-64.rgba`, que son los
16.384 bytes del icono sin comprimir, y no un PNG. Descodificar un PNG exigiría añadir el
paquete `image` —y sus dependencias— al programa, para dibujar un icono. El archivo en
crudo no necesita descodificador y el compilador lo mete tal cual en el binario.

El riesgo de guardar píxeles sin cabecera es que nada dice cuánto miden. Lo cubre la
prueba `el_icono_de_la_ventana_tiene_el_tamano_declarado`, que comprueba que el archivo
ocupa exactamente `ICONO_LADO × ICONO_LADO × 4` bytes y que no es del todo transparente.

**`winresource` no puede tumbar la compilación.** Si falla al incrustar el recurso,
`build.rs` avisa y sigue: un ejecutable sin dibujo sigue siendo un ejecutable. Es el mismo
criterio que ya se aplicaba a los datos de git.

---

## 18. Pruebas y arnés de interfaz

Todo se ejecuta con `cargo test`. No hace falta ninguna herramienta externa.

| Archivo | Qué cubre |
|---|---|
| `src/main.rs` (módulo `tests`) | Jerarquía, borrado en cascada, exportación e importación |
| `src/audit_checks.rs` | Regresión de los hallazgos de la auditoría, más el servidor MCP, los conectores, el autoguardado, la soberanía de datos, el aislamiento de perfil, las preferencias y la identificación de la compilación |
| `src/arnes_interfaz.rs` | Ejecutor headless de la interfaz: renderiza fotogramas reales de `egui`, mide la geometría de los paneles y simula clics de ratón (`hacer_clic_en_texto`) sin abrir una ventana en pantalla |
| `src/theme.rs` (módulo `tests`) | Contraste y legibilidad de las tres paletas según WCAG 2.1, y que el estilo de `egui` se construya a partir del tema |

`audit_checks.rs` merece una explicación. Sus pruebas nacieron **reproduciendo** los
fallos: demostraban que el defecto existía. Tras corregirlos, cada una se invirtió y
ahora comprueba que el fallo no puede volver. Cada prueba conserva en su documentación la
descripción del defecto original, de modo que si alguna falla, indica exactamente qué se
ha roto.

Las del servidor MCP ejercitan `procesar_peticion` directamente, **sin lanzar ningún
proceso**: se le pasa una petición JSON-RPC y se comprueba la respuesta, que es todo lo
que hace el bucle principal. Son más rápidas que arrancar el servidor y no dependen de
nada externo.

---

## 19. Soporte tipográfico CJK y fuente embebida

`egui` incluye por omisión una fuente latina ligera que carece de glifos para ideogramas chinos,
japoneses o coreanos. En sistemas donde las fuentes del sistema no están configuradas o en
entornos portables, los textos en chino se mostraban como cuadros vacíos (`▯`).

### La solución: subconjunto optimizado de Noto Sans SC

En lugar de incrustar una fuente china completa (que suele superar los 25 MB), se preparó un
subconjunto optimizado de **Noto Sans SC** (`assets/fuentes/NotoSansSC-subconjunto.ttf`, ~1,06 MB)
que cubre:
- El estándar completo **GB2312 Nivel 1** (3.755 caracteres chinos simplificados de uso frecuente).
- Todos los caracteres adicionales requeridos por las traducciones de la interfaz, el inspector,
  los modales y las 24 guías de ayuda en chino.
- Símbolos y flechas de uso habitual en la interfaz.

### Integración en el motor de fuentes

En `src/ui/mod.rs`, durante la inicialización de `eframe`, la fuente se carga como datos estáticos
(`FUENTE_CJK_BYTES = include_bytes!("../../assets/fuentes/NotoSansSC-subconjunto.ttf")`) y se
configura en `egui::FontDefinitions` como **fuente de respaldo (fallback)** en las familias
`Proportional` y `Monospace`. De este modo:
1. Los textos en alfabeto latino, cirílico o símbolos básicos siguen usando la tipografía estándar
   de `egui`.
2. Cuando el motor de renderizado encuentra un carácter CJK, recurre automáticamente a la fuente
   embebida sin latencia ni configuración del usuario.
3. Una prueba exhaustiva (`todos_los_caracteres_cjk_del_programa_tienen_glifo_en_la_fuente_embebida`)
   inspecciona la tabla `cmap` de la fuente en cada compilación para garantizar que ni un solo
   carácter del programa queda sin glifo.

---

## 20. Puertas del compilador para código que deja de usarse

La raíz del binario trata `dead_code`, `unused_imports`, `unused_variables` y `unused_mut` como
errores. En la compilación de producción se declaran con `forbid`: un módulo interior no puede
silenciarlos con un `#[allow(...)]`. Esto impide conservar una función, variante, importación o
variable simplemente porque quizá vuelva a resultar útil en el futuro.

El arnés que genera `rustc --test` introduce por su cuenta una excepción a `dead_code`. Rust
rechaza combinar esa excepción técnica con un `forbid` incondicional, por lo que durante las
pruebas los mismos avisos quedan en nivel `deny`. La puerta no se ha supuesto: se comprobó que
una excepción local rompe `cargo check --bin mmcelt` con `E0453`, y que las 397 pruebas pueden
seguir compilándose con la configuración específica del arnés.

Esta política no demuestra que una abstracción usada sea necesaria ni que una función tenga una
responsabilidad adecuada. Esas propiedades se comprueban ejecutando el comportamiento que
justifica el código. Tampoco sustituye a Rustdoc y Clippy: una prueba estructural separada cubre
el único hueco confirmado, una línea vacía entre la documentación `///` y el campo de una
estructura.

---

## 21. Comprobación de versión nueva

Es la única conexión de red de la aplicación, y está repartida en tres piezas para que cada una
tenga un solo motivo para cambiar y para que ninguna prueba necesite red:

| Pieza | Responsabilidad | Red |
|---|---|---|
| `version_publicada.rs` | Leer `tag_name`, comparar versiones como números, validar repositorio y direcciones, componer el enlace | No |
| `comprobacion_de_version.rs` | El rasgo `ConsultaDeVersion`, la consulta real `ConsultaHttps` (`ureq` + `rustls`) y el hilo de trabajo | Sí |
| `ui/estado_version_nueva.rs` | Arrancar una vez si la opción está activa y recoger el resultado sin esperar | No |

**Decisiones y por qué:**

- **La respuesta es un dato ajeno.** Solo se toma `tag_name` y solo si es `X.Y.Z`. El enlace del
  aviso se compone con la raíz web y el repositorio de la configuración; `html_url` se ignora,
  para que una respuesta manipulada no pueda llevar al usuario a otro sitio.
- **Desactivada, no existe la consulta.** `arrancar_si_procede` comprueba la opción antes de
  llamar a la fábrica de la consulta: con la opción apagada no llega a crearse nada capaz de salir
  a la red. Una prueba lo vigila contando las llamadas.
- **Ningún fallo es un pánico.** En publicación `panic = "abort"`: un pánico en el hilo cerraría
  el programa entero. Cada fallo vuelve como `FalloDeComprobacion`, se registra como
  `AppError::ComprobacionDeVersion` (severidad de aviso) y no se muestra.
- **Configurable sin recompilar.** Repositorio, raíces de API y web, tiempo de espera y límite de
  respuesta viven en `preferencias.json` (`ajustes_de_version`) con valores de fábrica
  documentados en `preferencias.rs`, y se validan en la frontera: solo `https://` con un nombre de
  servidor, y un repositorio `dueño/nombre`.
- **Solo el arranque real consulta.** `nueva_con_contexto` arranca la comprobación; las
  construcciones de prueba (`nueva_limpia`) nunca salen a la red.

