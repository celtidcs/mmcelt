//! # Tipos del modelo de datos
//!
//! Este módulo define las estructuras de datos fundamentales de **MMCelt**:
//! - Representación del proyecto (`Proyecto`).
//! - Nodos individuales del mapa mental (`Nodo`).
//! - Metadatos de ingeniería de software y control humano (`EstadoRevision`, `RolNodo`, etc.).
//! - Relaciones cruzadas transversales (`ConexionCruzada`, `TipoRelacion`).
//!
//! La validación de las invariantes está en el submódulo hermano `validacion`. La creación de
//! ejemplos y el recorrido de carpetas viven fuera del dominio, en `plantillas` y
//! `escaner_repositorio`.
//!
//! ## Tolerancia a los valores escritos por modelos de IA
//!
//! Las enumeraciones de este módulo (`EstadoNodo`, `PrioridadNodo`, `RolNodo`,
//! `TipoRelacion`, `EstadoRevision`) se deserializan a través de `String` mediante
//! `#[serde(from = "String")]`, y cada una acepta varios sinónimos en español e inglés.
//!
//! El motivo es concreto: los archivos `.mmcelt` los puede generar un modelo de IA a
//! través del servidor MCP, y un modelo escribe con frecuencia `"In Progress"` donde
//! el formato espera `"EnProgreso"`. Con enumeraciones estrictas, **un solo valor
//! desconocido en un solo nodo inutilizaba el archivo entero**. Ahora ese valor
//! degrada a la variante por defecto y el resto del mapa se conserva.
//!
//! La serialización sigue emitiendo siempre el nombre canónico, así que el formato en
//! disco no cambia.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap, HashSet};
use uuid::Uuid;

/// Profundidad máxima admitida para el árbol de un mapa mental.
///
/// Un mapa creado por una persona no se acerca ni de lejos a este valor. El límite
/// existe para frenar archivos generados automáticamente o manipulados: los recorridos
/// del módulo `layout` descendían recursivamente y una rama de cientos de miles de
/// niveles agotaba la pila del proceso.
pub const PROFUNDIDAD_MAXIMA: usize = 512;

/// Número máximo de nodos admitidos en un mapa mental.
///
/// Actúa como segunda barrera frente a archivos desmesurados que, aun sin ser
/// profundos, harían inmanejable la interfaz.
pub const NODOS_MAXIMOS: usize = 100_000;

/// Longitud máxima en caracteres permitida para el título de un nodo o de un mapa mental.
pub const LONGITUD_MAXIMA_TITULO: usize = 1_000;

/// Longitud máxima en caracteres permitida para las notas de un nodo (64 KiB).
pub const LONGITUD_MAXIMA_NOTAS: usize = 65_536;

/// Longitud máxima en caracteres permitida para campos descriptivos (visión, objetivos, contexto o feedback).
pub const LONGITUD_MAXIMA_TEXTO_DESCRIPTIVO: usize = 16_384;

/// Estado del ciclo de vida y madurez de un concepto o tarea.
///
/// Al deserializar acepta sinónimos en español e inglés (ver [`EstadoNodo::desde_texto`]);
/// al serializar emite siempre el nombre canónico.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum EstadoNodo {
    /// Concepto incipiente o propuesta inicial.
    #[default]
    Idea,
    /// En análisis técnico, benchmarking o estudio de viabilidad.
    Investigando,
    /// En fase de desarrollo o implementación activa.
    EnProgreso,
    /// Incógnita crítica, bloqueo o duda que requiere decisión.
    DudaBloqueo,
    /// Tarea completada, hito validado o módulo implementado.
    Completado,
    /// Opción descartada o no viable (la IA no debe insistir en ella).
    Descartado,
}

impl EstadoNodo {
    /// Todos los estados, en el orden en que avanza el trabajo.
    ///
    /// Tenerlos aquí evita el problema que arrastraba el inspector: enumeraba las variantes
    /// a mano, así que una nueva existía en el modelo pero no se podía elegir en la
    /// interfaz. Es el mismo motivo por el que existen `AppThemeMode::TODOS` y
    /// `TemaDeAyuda::TODOS`.
    pub const TODOS: [EstadoNodo; 6] = [
        EstadoNodo::Idea,
        EstadoNodo::Investigando,
        EstadoNodo::EnProgreso,
        EstadoNodo::DudaBloqueo,
        EstadoNodo::Completado,
        EstadoNodo::Descartado,
    ];

    /// Interpreta un texto libre como estado, aceptando sinónimos y sin distinguir
    /// mayúsculas.
    ///
    /// Esta es la **única** tabla de equivalencias de estados del proyecto: la usan la
    /// deserialización de archivos, el importador de respuestas de IA y, por
    /// contrato documentado, el servidor MCP.
    ///
    /// # Parámetros
    /// - `texto`: valor recibido del exterior (archivo, IA o MCP).
    ///
    /// # Devuelve
    /// `Some(estado)` si el texto coincide con una variante admitida, o `None` si no.
    pub fn reconocer_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "idea" | "thought" => Some(EstadoNodo::Idea),
            "investigando" | "researching" | "research" => Some(EstadoNodo::Investigando),
            "enprogreso" | "en progreso" | "progreso" | "in_progress" | "in progress"
            | "progress" | "doing" => Some(EstadoNodo::EnProgreso),
            "dudabloqueo" | "duda" | "bloqueo" | "doubt" | "blocked" | "block" => {
                Some(EstadoNodo::DudaBloqueo)
            }
            "completado" | "done" | "completed" | "finished" => Some(EstadoNodo::Completado),
            "descartado" | "discarded" | "cancelled" | "canceled" | "rejected" => {
                Some(EstadoNodo::Descartado)
            }
            _ => None,
        }
    }

    /// Resuelve un texto en un [`EstadoNodo`], degradando a [`EstadoNodo::Idea`] si no se reconoce.
    pub fn desde_texto(texto: &str) -> Self {
        Self::reconocer_texto(texto).unwrap_or(EstadoNodo::Idea)
    }

    /// Nombre canónico con el que el estado se escribe en disco.
    ///
    /// Es el valor que debe usar cualquier herramienta externa que genere archivos
    /// `.mmcelt`, incluido el servidor MCP.
    pub fn nombre_canonico(&self) -> &'static str {
        match self {
            EstadoNodo::Idea => "Idea",
            EstadoNodo::Investigando => "Investigando",
            EstadoNodo::EnProgreso => "EnProgreso",
            EstadoNodo::DudaBloqueo => "DudaBloqueo",
            EstadoNodo::Completado => "Completado",
            EstadoNodo::Descartado => "Descartado",
        }
    }

    /// El nombre del estado, en el idioma que el usuario tenga elegido.
    ///
    /// # Cuidado al llamarla desde el exportador
    ///
    /// El Markdown que se le entrega a la IA la usa, y **ahí se le pasa siempre
    /// `Idioma::Espanol` a propósito**: lo que lee la máquina no debe cambiar porque la
    /// persona haya cambiado el idioma de su pantalla. Está explicado en `ai_export.rs`.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (EstadoNodo::Idea, Idioma::Espanol) => "💡 Idea",
            (EstadoNodo::Investigando, Idioma::Espanol) => "🔍 Investigando",
            (EstadoNodo::EnProgreso, Idioma::Espanol) => "⏳ En Progreso",
            (EstadoNodo::DudaBloqueo, Idioma::Espanol) => "❓ Duda / Bloqueo",
            (EstadoNodo::Completado, Idioma::Espanol) => "✅ Completado",
            (EstadoNodo::Descartado, Idioma::Espanol) => "⛔ Descartado",

            (EstadoNodo::Idea, Idioma::Ingles) => "💡 Idea",
            (EstadoNodo::Investigando, Idioma::Ingles) => "🔍 Researching",
            (EstadoNodo::EnProgreso, Idioma::Ingles) => "⏳ In Progress",
            (EstadoNodo::DudaBloqueo, Idioma::Ingles) => "❓ Blocker / Question",
            (EstadoNodo::Completado, Idioma::Ingles) => "✅ Completed",
            (EstadoNodo::Descartado, Idioma::Ingles) => "⛔ Dismissed",

            (EstadoNodo::Idea, Idioma::Frances) => "💡 Idée",
            (EstadoNodo::Investigando, Idioma::Frances) => "🔍 En étude",
            (EstadoNodo::EnProgreso, Idioma::Frances) => "⏳ En cours",
            (EstadoNodo::DudaBloqueo, Idioma::Frances) => "❓ Blocage / Question",
            (EstadoNodo::Completado, Idioma::Frances) => "✅ Terminé",
            (EstadoNodo::Descartado, Idioma::Frances) => "⛔ Écarté",

            (EstadoNodo::Idea, Idioma::Aleman) => "💡 Idee",
            (EstadoNodo::Investigando, Idioma::Aleman) => "🔍 In Recherche",
            (EstadoNodo::EnProgreso, Idioma::Aleman) => "⏳ In Bearbeitung",
            (EstadoNodo::DudaBloqueo, Idioma::Aleman) => "❓ Blockade / Frage",
            (EstadoNodo::Completado, Idioma::Aleman) => "✅ Abgeschlossen",
            (EstadoNodo::Descartado, Idioma::Aleman) => "⛔ Verworfen",

            (EstadoNodo::Idea, Idioma::Ruso) => "💡 Идея",
            (EstadoNodo::Investigando, Idioma::Ruso) => "🔍 Исследование",
            (EstadoNodo::EnProgreso, Idioma::Ruso) => "⏳ В работе",
            (EstadoNodo::DudaBloqueo, Idioma::Ruso) => "❓ Блокер / Вопрос",
            (EstadoNodo::Completado, Idioma::Ruso) => "✅ Завершено",
            (EstadoNodo::Descartado, Idioma::Ruso) => "⛔ Отклонено",

            (EstadoNodo::Idea, Idioma::ChinoSimplificado) => "💡 构思",
            (EstadoNodo::Investigando, Idioma::ChinoSimplificado) => "🔍 调研中",
            (EstadoNodo::EnProgreso, Idioma::ChinoSimplificado) => "⏳ 进行中",
            (EstadoNodo::DudaBloqueo, Idioma::ChinoSimplificado) => "❓ 阻塞 / 待决",
            (EstadoNodo::Completado, Idioma::ChinoSimplificado) => "✅ 已完成",
            (EstadoNodo::Descartado, Idioma::ChinoSimplificado) => "⛔ 已弃用",
        }
    }

    /// Devuelve el emoji representativo del estado.
    pub fn emoji(&self) -> &'static str {
        match self {
            EstadoNodo::Idea => "💡",
            EstadoNodo::Investigando => "🔍",
            EstadoNodo::EnProgreso => "⏳",
            EstadoNodo::DudaBloqueo => "❓",
            EstadoNodo::Completado => "✅",
            EstadoNodo::Descartado => "⛔",
        }
    }
}

impl From<String> for EstadoNodo {
    /// Conversión usada por `serde` al deserializar; delega en [`EstadoNodo::desde_texto`].
    fn from(texto: String) -> Self {
        EstadoNodo::desde_texto(&texto)
    }
}

impl From<EstadoNodo> for String {
    /// Conversión usada por `serde` al serializar; emite el nombre canónico.
    fn from(estado: EstadoNodo) -> Self {
        estado.nombre_canonico().to_string()
    }
}

/// Estado de supervisión y control humano para el bucle *Human-in-the-loop*.
///
/// Igual que el resto de enumeraciones del módulo, tolera valores desconocidos al
/// deserializar degradando a [`EstadoRevision::PendienteRevision`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum EstadoRevision {
    /// Generado automáticamente por un modelo de IA (pendiente de auditoría humana).
    GeneradoPorIA,
    /// Propuesto y a la espera de validación del usuario supervisor.
    #[default]
    PendienteRevision,
    /// Validado, autorizado y aprobado explícitamente por el usuario humano.
    AprobadoPorHumano,
    /// El usuario exige cambios obligatorios a la IA sobre este nodo.
    RequiereCorreccion,
}

impl EstadoRevision {
    /// Todos los estados de revisión, del menos supervisado al más.
    ///
    /// Tenerlos aquí evita el problema que arrastraba el inspector: enumeraba las variantes
    /// a mano, así que una nueva existía en el modelo pero no se podía elegir en la
    /// interfaz. Es el mismo motivo por el que existen `AppThemeMode::TODOS` y
    /// `TemaDeAyuda::TODOS`.
    pub const TODOS: [EstadoRevision; 4] = [
        EstadoRevision::GeneradoPorIA,
        EstadoRevision::PendienteRevision,
        EstadoRevision::AprobadoPorHumano,
        EstadoRevision::RequiereCorreccion,
    ];

    /// Interpreta un texto libre como estado de revisión, aceptando sinónimos.
    ///
    /// # Parámetros
    /// - `texto`: lo que venga escrito, tal cual. Puede llegar del archivo de un mapa,
    ///   de la importación de un modelo de IA o de una edición a mano, así que no se
    ///   distingue entre mayúsculas y minúsculas ni se exige el nombre canónico.
    ///
    /// # Devuelve
    /// `Some(estado)` si el texto coincide con una variante admitida, o `None` si no.
    pub fn reconocer_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "generadoporia" | "generado_por_ia" | "ai_generated" | "ia" => {
                Some(EstadoRevision::GeneradoPorIA)
            }
            "pendienterevision" | "pendiente" | "pending" | "pending_review" => {
                Some(EstadoRevision::PendienteRevision)
            }
            "aprobadoporhumano" | "aprobado" | "approved" | "human_approved" => {
                Some(EstadoRevision::AprobadoPorHumano)
            }
            "requierecorreccion" | "correccion" | "needs_correction" | "rejected" => {
                Some(EstadoRevision::RequiereCorreccion)
            }
            _ => None,
        }
    }

    /// Resuelve un texto en un [`EstadoRevision`], degradando a [`EstadoRevision::PendienteRevision`] si no se reconoce.
    pub fn desde_texto(texto: &str) -> Self {
        Self::reconocer_texto(texto).unwrap_or(EstadoRevision::PendienteRevision)
    }

    /// Nombre canónico con el que el estado se escribe en disco.
    pub fn nombre_canonico(&self) -> &'static str {
        match self {
            EstadoRevision::GeneradoPorIA => "GeneradoPorIA",
            EstadoRevision::PendienteRevision => "PendienteRevision",
            EstadoRevision::AprobadoPorHumano => "AprobadoPorHumano",
            EstadoRevision::RequiereCorreccion => "RequiereCorreccion",
        }
    }

    /// El texto del estado de revisión humana, en el idioma elegido.
    ///
    /// # Esto no es un rótulo decorativo
    ///
    /// Estas cuatro variantes son el control de supervisión humana: distinguen lo que ha
    /// escrito la máquina de lo que ha revisado una persona, y de ese estado depende que la
    /// sincronización con la IA pueda o no sobrescribir un nodo. Una traducción que difumine
    /// esa distinción —«validado» a secas, sin decir contra qué— es un fallo de seguridad,
    /// no de estilo. Las seis columnas se verificaron contra corpus normativos antes de
    /// integrarse.
    ///
    /// Lo que se traduce es **solo** este nombre. El nombre canónico con el que el estado
    /// viaja en el archivo `.mmcelt` y por el protocolo MCP lo da `como_texto`/`desde_texto`
    /// y **no se traduce nunca**.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (EstadoRevision::GeneradoPorIA, Idioma::Espanol) => "🤖 Generado por IA",
            (EstadoRevision::PendienteRevision, Idioma::Espanol) => "⏳ Pendiente de Revisión",
            (EstadoRevision::AprobadoPorHumano, Idioma::Espanol) => "🛡️ Aprobado por Usuario",
            (EstadoRevision::RequiereCorreccion, Idioma::Espanol) => "⚠️ Requiere Corrección",

            (EstadoRevision::GeneradoPorIA, Idioma::Ingles) => "🤖 AI Generated",
            (EstadoRevision::PendienteRevision, Idioma::Ingles) => "⏳ Pending Human Review",
            (EstadoRevision::AprobadoPorHumano, Idioma::Ingles) => "🛡️ Human Approved",
            (EstadoRevision::RequiereCorreccion, Idioma::Ingles) => "⚠️ Requires Correction",

            (EstadoRevision::GeneradoPorIA, Idioma::Frances) => "🤖 Généré par l'IA",
            (EstadoRevision::PendienteRevision, Idioma::Frances) => "⏳ En attente de révision",
            (EstadoRevision::AprobadoPorHumano, Idioma::Frances) => "🛡️ Approuvé par l'humain",
            (EstadoRevision::RequiereCorreccion, Idioma::Frances) => "⚠️ Correction requise",

            (EstadoRevision::GeneradoPorIA, Idioma::Aleman) => "🤖 KI-generiert",
            (EstadoRevision::PendienteRevision, Idioma::Aleman) => "⏳ Ausstehende Prüfung",
            (EstadoRevision::AprobadoPorHumano, Idioma::Aleman) => "🛡️ Menschlich freigegeben",
            (EstadoRevision::RequiereCorreccion, Idioma::Aleman) => "⚠️ Korrektur erforderlich",

            (EstadoRevision::GeneradoPorIA, Idioma::Ruso) => "🤖 Создано ИИ",
            (EstadoRevision::PendienteRevision, Idioma::Ruso) => "⏳ Ожидает проверки",
            (EstadoRevision::AprobadoPorHumano, Idioma::Ruso) => "🛡️ Одобрено человеком",
            (EstadoRevision::RequiereCorreccion, Idioma::Ruso) => "⚠️ Требует исправления",

            (EstadoRevision::GeneradoPorIA, Idioma::ChinoSimplificado) => "🤖 AI生成",
            (EstadoRevision::PendienteRevision, Idioma::ChinoSimplificado) => "⏳ 待人工复核",
            (EstadoRevision::AprobadoPorHumano, Idioma::ChinoSimplificado) => "🛡️ 人工已核准",
            (EstadoRevision::RequiereCorreccion, Idioma::ChinoSimplificado) => "⚠️ 需修正",
        }
    }

    /// Devuelve el emoji del estado de revisión.
    pub fn emoji(&self) -> &'static str {
        match self {
            EstadoRevision::GeneradoPorIA => "🤖",
            EstadoRevision::PendienteRevision => "⏳",
            EstadoRevision::AprobadoPorHumano => "🛡️",
            EstadoRevision::RequiereCorreccion => "⚠️",
        }
    }

    /// Símbolo visual para la tarjeta del nodo en el lienzo (C19-C, PH-0925-1).
    ///
    /// Devuelve símbolo para todos los estados de revisión humana: `GeneradoPorIA` (🤖),
    /// `AprobadoPorHumano` (🛡️), `RequiereCorreccion` (⚠️) y `PendienteRevision` (⏳).
    pub fn simbolo_en_nodo(&self) -> Option<&'static str> {
        match self {
            EstadoRevision::GeneradoPorIA => Some("🤖"),
            EstadoRevision::AprobadoPorHumano => Some("🛡️"),
            EstadoRevision::RequiereCorreccion => Some("⚠️"),
            EstadoRevision::PendienteRevision => Some("⏳"),
        }
    }
}

impl From<String> for EstadoRevision {
    /// Conversión usada por `serde` al deserializar.
    fn from(texto: String) -> Self {
        EstadoRevision::desde_texto(&texto)
    }
}

impl From<EstadoRevision> for String {
    /// Conversión usada por `serde` al serializar; emite el nombre canónico.
    fn from(estado: EstadoRevision) -> Self {
        estado.nombre_canonico().to_string()
    }
}

/// Nivel de prioridad o urgencia estratégica asignada a un nodo.
///
/// Tolera valores desconocidos al deserializar degradando a [`PrioridadNodo::Media`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum PrioridadNodo {
    /// Puede posponerse sin consecuencias.
    Baja = 1,
    /// Prioridad ordinaria; es el valor por defecto.
    #[default]
    Media = 2,
    /// Debe abordarse pronto.
    Alta = 3,
    /// Bloquea el avance del proyecto.
    Critica = 4,
}

impl PrioridadNodo {
    /// Todas las prioridades, de menor a mayor.
    ///
    /// Tenerlos aquí evita el problema que arrastraba el inspector: enumeraba las variantes
    /// a mano, así que una nueva existía en el modelo pero no se podía elegir en la
    /// interfaz. Es el mismo motivo por el que existen `AppThemeMode::TODOS` y
    /// `TemaDeAyuda::TODOS`.
    pub const TODOS: [PrioridadNodo; 4] = [
        PrioridadNodo::Baja,
        PrioridadNodo::Media,
        PrioridadNodo::Alta,
        PrioridadNodo::Critica,
    ];

    /// Interpreta un texto libre como prioridad, aceptando sinónimos.
    ///
    /// # Parámetros
    /// - `texto`: lo que venga escrito, tal cual. Puede llegar del archivo de un mapa,
    ///   de la importación de un modelo de IA o de una edición a mano, así que no se
    ///   distingue entre mayúsculas y minúsculas ni se exige el nombre canónico.
    ///
    /// # Devuelve
    /// `Some(prioridad)` si el texto coincide con una variante admitida, o `None` si no.
    pub fn reconocer_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "critica" | "crítica" | "critical" | "urgente" | "urgent" | "p1" => {
                Some(PrioridadNodo::Critica)
            }
            "alta" | "high" | "p2" => Some(PrioridadNodo::Alta),
            "media" | "medium" | "p3" | "normal" => Some(PrioridadNodo::Media),
            "baja" | "low" | "p4" => Some(PrioridadNodo::Baja),
            _ => None,
        }
    }

    /// Resuelve un texto en una [`PrioridadNodo`], degradando a [`PrioridadNodo::Media`] si no se reconoce.
    pub fn desde_texto(texto: &str) -> Self {
        Self::reconocer_texto(texto).unwrap_or(PrioridadNodo::Media)
    }

    /// Nombre canónico con el que la prioridad se escribe en disco.
    pub fn nombre_canonico(&self) -> &'static str {
        match self {
            PrioridadNodo::Baja => "Baja",
            PrioridadNodo::Media => "Media",
            PrioridadNodo::Alta => "Alta",
            PrioridadNodo::Critica => "Critica",
        }
    }

    /// El nombre de la prioridad, en el idioma que el usuario tenga elegido.
    ///
    /// El exportador de Markdown la llama con `Idioma::Espanol` fijo, por lo mismo que
    /// `EstadoNodo::nombre_para_interfaz`.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (PrioridadNodo::Baja, Idioma::Espanol) => "🔽 Baja",
            (PrioridadNodo::Media, Idioma::Espanol) => "🔷 Media",
            (PrioridadNodo::Alta, Idioma::Espanol) => "⚡ Alta",
            (PrioridadNodo::Critica, Idioma::Espanol) => "🔥 Crítica",

            (PrioridadNodo::Baja, Idioma::Ingles) => "🔽 Low",
            (PrioridadNodo::Media, Idioma::Ingles) => "🔷 Medium",
            (PrioridadNodo::Alta, Idioma::Ingles) => "⚡ High",
            (PrioridadNodo::Critica, Idioma::Ingles) => "🔥 Critical",

            (PrioridadNodo::Baja, Idioma::Frances) => "🔽 Basse",
            (PrioridadNodo::Media, Idioma::Frances) => "🔷 Moyenne",
            (PrioridadNodo::Alta, Idioma::Frances) => "⚡ Haute",
            (PrioridadNodo::Critica, Idioma::Frances) => "🔥 Critique",

            (PrioridadNodo::Baja, Idioma::Aleman) => "🔽 Niedrig",
            (PrioridadNodo::Media, Idioma::Aleman) => "🔷 Mittel",
            (PrioridadNodo::Alta, Idioma::Aleman) => "⚡ Hoch",
            (PrioridadNodo::Critica, Idioma::Aleman) => "🔥 Kritisch",

            (PrioridadNodo::Baja, Idioma::Ruso) => "🔽 Низкий",
            (PrioridadNodo::Media, Idioma::Ruso) => "🔷 Средний",
            (PrioridadNodo::Alta, Idioma::Ruso) => "⚡ Высокий",
            (PrioridadNodo::Critica, Idioma::Ruso) => "🔥 Критический",

            (PrioridadNodo::Baja, Idioma::ChinoSimplificado) => "🔽 低",
            (PrioridadNodo::Media, Idioma::ChinoSimplificado) => "🔷 中",
            (PrioridadNodo::Alta, Idioma::ChinoSimplificado) => "⚡ 高",
            (PrioridadNodo::Critica, Idioma::ChinoSimplificado) => "🔥 紧急",
        }
    }

    /// Símbolo visual o emoji representativo de la prioridad para destacar en el lienzo (C19-C, C19-E, PH-0925-1).
    ///
    /// Devuelve símbolo para todas las prioridades: `Critica` (🔥), `Alta` (⚡), `Media` (🔷) y `Baja` (🔽).
    pub fn simbolo(&self) -> Option<&'static str> {
        match self {
            PrioridadNodo::Critica => Some("🔥"),
            PrioridadNodo::Alta => Some("⚡"),
            PrioridadNodo::Media => Some("🔷"),
            PrioridadNodo::Baja => Some("🔽"),
        }
    }
}

impl From<String> for PrioridadNodo {
    /// Conversión usada por `serde` al deserializar.
    fn from(texto: String) -> Self {
        PrioridadNodo::desde_texto(&texto)
    }
}

impl From<PrioridadNodo> for String {
    /// Conversión usada por `serde` al serializar; emite el nombre canónico.
    fn from(prioridad: PrioridadNodo) -> Self {
        prioridad.nombre_canonico().to_string()
    }
}

/// Rol semántico y arquitectónico del nodo dentro del árbol.
///
/// Tolera valores desconocidos al deserializar degradando a [`RolNodo::Subtema`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum RolNodo {
    /// Idea o sistema raíz del mapa mental.
    IdeaCentral,
    /// Primer nivel de descomposición (capa arquitectónica o módulo principal).
    PilarEstrategico,
    /// Elemento o funcionalidad secundaria dependiente de un pilar.
    #[default]
    Subtema,
    /// Pregunta técnica, hipótesis a validar o decisión arquitectónica pendiente.
    HipotesisDuda,
    /// Tarea ejecutable concreta con entregable medible.
    AccionTarea,
    /// Herramienta, librería, especificación, API o documentación técnica.
    RecursoHerramienta,
}

impl RolNodo {
    /// Todos los roles, de lo más general a lo más concreto.
    ///
    /// Tenerlos aquí evita el problema que arrastraba el inspector: enumeraba las variantes
    /// a mano, así que una nueva existía en el modelo pero no se podía elegir en la
    /// interfaz. Es el mismo motivo por el que existen `AppThemeMode::TODOS` y
    /// `TemaDeAyuda::TODOS`.
    /// Icono del rol en la tarjeta: el mismo con el que empieza su nombre en el inspector.
    pub fn simbolo(&self) -> &'static str {
        match self {
            RolNodo::IdeaCentral => "🎯",
            RolNodo::PilarEstrategico => "🏛️",
            RolNodo::Subtema => "📌",
            RolNodo::HipotesisDuda => "❓",
            RolNodo::AccionTarea => "⚡",
            RolNodo::RecursoHerramienta => "🔧",
        }
    }

    pub const TODOS: [RolNodo; 6] = [
        RolNodo::IdeaCentral,
        RolNodo::PilarEstrategico,
        RolNodo::Subtema,
        RolNodo::HipotesisDuda,
        RolNodo::AccionTarea,
        RolNodo::RecursoHerramienta,
    ];

    /// Interpreta un texto libre como rol, aceptando sinónimos.
    ///
    /// # Parámetros
    /// - `texto`: lo que venga escrito, tal cual. Puede llegar del archivo de un mapa,
    ///   de la importación de un modelo de IA o de una edición a mano, así que no se
    ///   distingue entre mayúsculas y minúsculas ni se exige el nombre canónico.
    ///
    /// # Devuelve
    /// `Some(rol)` si el texto coincide con una variante admitida, o `None` si no.
    pub fn reconocer_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "ideacentral" | "idea central" | "central" | "root" | "raiz" | "raíz" => {
                Some(RolNodo::IdeaCentral)
            }
            "pilarestrategico" | "pilar" | "strategic" | "pillar" => {
                Some(RolNodo::PilarEstrategico)
            }
            "subtema" | "subtopic" | "topic" | "tema" => Some(RolNodo::Subtema),
            "hipotesisduda" | "duda" | "pregunta" | "hipotesis" | "hipótesis" | "question"
            | "doubt" => Some(RolNodo::HipotesisDuda),
            "acciontarea" | "accion" | "acción" | "tarea" | "task" | "action" => {
                Some(RolNodo::AccionTarea)
            }
            "recursoherramienta" | "recurso" | "herramienta" | "tool" | "resource" => {
                Some(RolNodo::RecursoHerramienta)
            }
            _ => None,
        }
    }

    /// Resuelve un texto en un [`RolNodo`], degradando a [`RolNodo::Subtema`] si no se reconoce.
    pub fn desde_texto(texto: &str) -> Self {
        Self::reconocer_texto(texto).unwrap_or(RolNodo::Subtema)
    }

    /// Nombre canónico con el que el rol se escribe en disco.
    pub fn nombre_canonico(&self) -> &'static str {
        match self {
            RolNodo::IdeaCentral => "IdeaCentral",
            RolNodo::PilarEstrategico => "PilarEstrategico",
            RolNodo::Subtema => "Subtema",
            RolNodo::HipotesisDuda => "HipotesisDuda",
            RolNodo::AccionTarea => "AccionTarea",
            RolNodo::RecursoHerramienta => "RecursoHerramienta",
        }
    }

    /// El nombre del rol, en el idioma que el usuario tenga elegido.
    ///
    /// Sigue el mismo criterio que `TipoRelacion::nombre_para_interfaz`: el nombre pertenece
    /// al rol, no a la pantalla que lo dibuja, así que vive en el modelo y no en `textos.rs`.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (RolNodo::IdeaCentral, Idioma::Espanol) => "🎯 Idea Central",
            (RolNodo::PilarEstrategico, Idioma::Espanol) => "🏛️ Pilar Estratégico",
            (RolNodo::Subtema, Idioma::Espanol) => "📌 Subtema / Módulo",
            (RolNodo::HipotesisDuda, Idioma::Espanol) => "❓ Hipótesis / Duda",
            (RolNodo::AccionTarea, Idioma::Espanol) => "⚡ Tarea / Acción",
            (RolNodo::RecursoHerramienta, Idioma::Espanol) => "🔧 Recurso / Herramienta",

            (RolNodo::IdeaCentral, Idioma::Ingles) => "🎯 Central Idea",
            (RolNodo::PilarEstrategico, Idioma::Ingles) => "🏛️ Strategic Pillar",
            (RolNodo::Subtema, Idioma::Ingles) => "📌 Subtopic / Module",
            (RolNodo::HipotesisDuda, Idioma::Ingles) => "❓ Hypothesis / Question",
            (RolNodo::AccionTarea, Idioma::Ingles) => "⚡ Task / Action",
            (RolNodo::RecursoHerramienta, Idioma::Ingles) => "🔧 Resource / Tool",

            (RolNodo::IdeaCentral, Idioma::Frances) => "🎯 Idée centrale",
            (RolNodo::PilarEstrategico, Idioma::Frances) => "🏛️ Pilier stratégique",
            (RolNodo::Subtema, Idioma::Frances) => "📌 Sous-thème / Module",
            (RolNodo::HipotesisDuda, Idioma::Frances) => "❓ Hypothèse / Question",
            (RolNodo::AccionTarea, Idioma::Frances) => "⚡ Tâche / Action",
            (RolNodo::RecursoHerramienta, Idioma::Frances) => "🔧 Ressource / Outil",

            (RolNodo::IdeaCentral, Idioma::Aleman) => "🎯 Zentrale Idee",
            (RolNodo::PilarEstrategico, Idioma::Aleman) => "🏛️ Strategische Säule",
            (RolNodo::Subtema, Idioma::Aleman) => "📌 Unterthema / Modul",
            (RolNodo::HipotesisDuda, Idioma::Aleman) => "❓ Hypothese / Frage",
            (RolNodo::AccionTarea, Idioma::Aleman) => "⚡ Aufgabe / Aktion",
            (RolNodo::RecursoHerramienta, Idioma::Aleman) => "🔧 Ressource / Werkzeug",

            (RolNodo::IdeaCentral, Idioma::Ruso) => "🎯 Главная идея",
            (RolNodo::PilarEstrategico, Idioma::Ruso) => "🏛️ Стратегический модуль",
            (RolNodo::Subtema, Idioma::Ruso) => "📌 Подраздел / Модуль",
            (RolNodo::HipotesisDuda, Idioma::Ruso) => "❓ Гипотеза / Вопрос",
            (RolNodo::AccionTarea, Idioma::Ruso) => "⚡ Задача / Действие",
            (RolNodo::RecursoHerramienta, Idioma::Ruso) => "🔧 Ресурс / Инструмент",

            (RolNodo::IdeaCentral, Idioma::ChinoSimplificado) => "🎯 核心概念",
            (RolNodo::PilarEstrategico, Idioma::ChinoSimplificado) => "🏛️ 战略支柱",
            (RolNodo::Subtema, Idioma::ChinoSimplificado) => "📌 子主题 / 模块",
            (RolNodo::HipotesisDuda, Idioma::ChinoSimplificado) => "❓ 假设 / 疑问",
            (RolNodo::AccionTarea, Idioma::ChinoSimplificado) => "⚡ 任务 / 执行项",
            (RolNodo::RecursoHerramienta, Idioma::ChinoSimplificado) => "🔧 资源 / 工具",
        }
    }
}

impl From<String> for RolNodo {
    /// Conversión usada por `serde` al deserializar.
    fn from(texto: String) -> Self {
        RolNodo::desde_texto(&texto)
    }
}

impl From<RolNodo> for String {
    /// Conversión usada por `serde` al serializar; emite el nombre canónico.
    fn from(rol: RolNodo) -> Self {
        rol.nombre_canonico().to_string()
    }
}

/// Tipo de relación semántica transversal entre dos nodos que no comparten rama directa.
///
/// Un texto desconocido al deserializar degrada a [`TipoRelacion::Sinergia`], que es la
/// relación más neutra de las disponibles. No debe confundirse con el valor de
/// [`Default`], que es `Dependencia`: ese es el que propone la interfaz al crear una
/// conexión nueva a mano.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum TipoRelacion {
    /// El nodo origen requiere que el nodo destino esté completado o disponible.
    #[default]
    Dependencia,
    /// El nodo origen toma como referencia o inspiración el nodo destino.
    InspiradoPor,
    /// El nodo origen imposibilita o bloquea el progreso del nodo destino.
    Bloquea,
    /// El nodo origen es una solución alternativa o excluyente al nodo destino.
    AlternativaA,
    /// Ambos nodos se potencian mutuamente al desarrollarse en conjunto.
    Sinergia,
}

impl TipoRelacion {
    /// Todas las relaciones, en el orden en que se ofrecen al usuario.
    ///
    /// Era la única enumeración del modelo sin esta lista, y se notaba: el desplegable para
    /// elegir el tipo de relación no llegó a existir, así que la interfaz creaba siempre
    /// `Dependencia` y las otras cuatro solo se podían obtener importando de una IA o
    /// editando el archivo a mano. El caso de uso que la propia documentación pone de
    /// ejemplo —«la elección de base de datos **bloquea** el diseño del API»— no se podía
    /// hacer con el programa.
    ///
    /// El orden lo encabeza `Dependencia` por ser la más común y la propuesta por omisión.
    pub const TODOS: [TipoRelacion; 5] = [
        TipoRelacion::Dependencia,
        TipoRelacion::Bloquea,
        TipoRelacion::AlternativaA,
        TipoRelacion::Sinergia,
        TipoRelacion::InspiradoPor,
    ];

    /// Interpreta un texto libre como tipo de relación, aceptando sinónimos.
    ///
    /// # Parámetros
    /// - `texto`: lo que venga escrito, tal cual. Puede llegar del archivo de un mapa,
    ///   de la importación de un modelo de IA o de una edición a mano, así que no se
    ///   distingue entre mayúsculas y minúsculas ni se exige el nombre canónico.
    ///
    /// # Devuelve
    /// `Some(tipo)` si el texto coincide con una variante admitida, o `None` si no.
    pub fn reconocer_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "dependencia" | "depends" | "depend" | "dependency" | "requires" => {
                Some(TipoRelacion::Dependencia)
            }
            "inspiradopor" | "inspirado" | "inspired" | "inspired_by" => {
                Some(TipoRelacion::InspiradoPor)
            }
            "bloquea" | "blocks" | "block" | "blocking" => Some(TipoRelacion::Bloquea),
            "alternativaa" | "alternativa" | "alternative" | "alt" => {
                Some(TipoRelacion::AlternativaA)
            }
            "sinergia" | "synergy" => Some(TipoRelacion::Sinergia),
            _ => None,
        }
    }

    /// Resuelve un texto en un [`TipoRelacion`], degradando a [`TipoRelacion::Sinergia`] si no se reconoce.
    pub fn desde_texto(texto: &str) -> Self {
        Self::reconocer_texto(texto).unwrap_or(TipoRelacion::Sinergia)
    }

    /// Nombre canónico con el que la relación se escribe en disco.
    pub fn nombre_canonico(&self) -> &'static str {
        match self {
            TipoRelacion::Dependencia => "Dependencia",
            TipoRelacion::InspiradoPor => "InspiradoPor",
            TipoRelacion::Bloquea => "Bloquea",
            TipoRelacion::AlternativaA => "AlternativaA",
            TipoRelacion::Sinergia => "Sinergia",
        }
    }

    /// Nombre de la relación, en el idioma que el usuario tenga elegido.
    ///
    /// Solo existía en castellano, y se mostraba pegado a rótulos ya traducidos: con la
    /// interfaz en inglés, crear una conexión escribía en la barra de estado «Cross link
    /// added: Dependencia (Requiere)».
    ///
    /// No va en el módulo de textos porque es un dato del modelo, no un rótulo de la
    /// interfaz: el nombre pertenece a la relación, igual que su símbolo.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (TipoRelacion::Dependencia, Idioma::Espanol) => "Dependencia (Requiere)",
            (TipoRelacion::InspiradoPor, Idioma::Espanol) => "Inspirado por",
            (TipoRelacion::Bloquea, Idioma::Espanol) => "Bloquea a",
            (TipoRelacion::AlternativaA, Idioma::Espanol) => "Alternativa a",
            (TipoRelacion::Sinergia, Idioma::Espanol) => "Sinergia con",

            (TipoRelacion::Dependencia, Idioma::Ingles) => "Dependency (requires)",
            (TipoRelacion::InspiradoPor, Idioma::Ingles) => "Inspired by",
            (TipoRelacion::Bloquea, Idioma::Ingles) => "Blocks",
            (TipoRelacion::AlternativaA, Idioma::Ingles) => "Alternative to",
            (TipoRelacion::Sinergia, Idioma::Ingles) => "Synergy with",

            (TipoRelacion::Dependencia, Idioma::Frances) => "Dépendance (requiert)",
            (TipoRelacion::InspiradoPor, Idioma::Frances) => "Inspiré par",
            (TipoRelacion::Bloquea, Idioma::Frances) => "Bloque",
            (TipoRelacion::AlternativaA, Idioma::Frances) => "Alternative à",
            (TipoRelacion::Sinergia, Idioma::Frances) => "Synergie avec",

            (TipoRelacion::Dependencia, Idioma::Aleman) => "Abhängigkeit (benötigt)",
            (TipoRelacion::InspiradoPor, Idioma::Aleman) => "Inspiriert von",
            (TipoRelacion::Bloquea, Idioma::Aleman) => "Blockiert",
            (TipoRelacion::AlternativaA, Idioma::Aleman) => "Alternative zu",
            (TipoRelacion::Sinergia, Idioma::Aleman) => "Synergie mit",

            (TipoRelacion::Dependencia, Idioma::Ruso) => "Зависимость (требует)",
            (TipoRelacion::InspiradoPor, Idioma::Ruso) => "Вдохновлено",
            (TipoRelacion::Bloquea, Idioma::Ruso) => "Блокирует",
            (TipoRelacion::AlternativaA, Idioma::Ruso) => "Альтернатива",
            (TipoRelacion::Sinergia, Idioma::Ruso) => "Синергия с",

            (TipoRelacion::Dependencia, Idioma::ChinoSimplificado) => "依赖（需要）",
            (TipoRelacion::InspiradoPor, Idioma::ChinoSimplificado) => "灵感来自",
            (TipoRelacion::Bloquea, Idioma::ChinoSimplificado) => "阻塞",
            (TipoRelacion::AlternativaA, Idioma::ChinoSimplificado) => "替代方案",
            (TipoRelacion::Sinergia, Idioma::ChinoSimplificado) => "协同于",
        }
    }

    /// Símbolo visual de la relación.
    pub fn simbolo(&self) -> &'static str {
        match self {
            TipoRelacion::Dependencia => "➡️",
            TipoRelacion::InspiradoPor => "💡",
            TipoRelacion::Bloquea => "⛔",
            TipoRelacion::AlternativaA => "🔀",
            TipoRelacion::Sinergia => "✨",
        }
    }
}

impl From<String> for TipoRelacion {
    /// Conversión usada por `serde` al deserializar.
    fn from(texto: String) -> Self {
        TipoRelacion::desde_texto(&texto)
    }
}

impl From<TipoRelacion> for String {
    /// Conversión usada por `serde` al serializar; emite el nombre canónico.
    fn from(relacion: TipoRelacion) -> Self {
        relacion.nombre_canonico().to_string()
    }
}

/// Representa una conexión cruzada no jerárquica entre dos nodos del grafo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConexionCruzada {
    /// Identificador único de la conexión.
    pub id: Uuid,
    /// ID del nodo de origen.
    pub from: Uuid,
    /// ID del nodo de destino.
    pub to: Uuid,
    /// Etiqueta explicativa del motivo de la relación.
    pub label: String,
    /// Naturaleza semántica de la conexión.
    pub relation_type: TipoRelacion,
}

/// Modo de distribución espacial de los nodos en el lienzo 2D.
///
/// Deriva `Hash` porque entra en la huella con la que el autoguardado decide si hay algo
/// nuevo que copiar: cambiar de disposición es trabajo del usuario como cualquier otro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ModoDisposicion {
    /// Árbol horizontal con ramas balanceadas a la izquierda y derecha del centro.
    #[default]
    HorizontalTree,
    /// Disposición circular radial alrededor de la idea central.
    RadialTree,
    /// Posicionamiento manual libre arrastrado por el usuario.
    FreeDrag,
}

impl ModoDisposicion {
    /// Nombre en español para el selector de la barra de herramientas.
    /// El nombre del modo de disposición, en el idioma que el usuario tenga elegido.
    ///
    /// # Cuidado al cambiar el de `FreeDrag`
    ///
    /// El aviso `Texto::AvisoArrastreEnModoAutomatico` **cita este nombre dentro de una frase**
    /// —«cambia a Posición Libre Manual»— para decirle al usuario a qué opción del menú tiene
    /// que ir. Si los dos dejan de coincidir, el aviso manda a una opción que en su idioma se
    /// llama de otra forma. La prueba `el_aviso_del_arrastre_nombra_la_opcion_tal_como_se_llama`
    /// lo comprueba en los seis idiomas.
    ///
    /// Como con las demás enumeraciones, esto es **solo** el nombre visible: el canónico con el
    /// que el modo se guarda en el archivo `.mmcelt` no se traduce nunca.
    ///
    /// # Parámetros
    /// - `idioma`: el elegido por el usuario.
    pub fn nombre_para_interfaz(&self, idioma: crate::textos::Idioma) -> &'static str {
        use crate::textos::Idioma;
        match (self, idioma) {
            (ModoDisposicion::HorizontalTree, Idioma::Espanol) => "Árbol Balanceado (Izq/Der)",
            (ModoDisposicion::RadialTree, Idioma::Espanol) => "Radial / Circular",
            (ModoDisposicion::FreeDrag, Idioma::Espanol) => "Posición Libre Manual",

            (ModoDisposicion::HorizontalTree, Idioma::Ingles) => "Balanced Tree (L/R)",
            (ModoDisposicion::RadialTree, Idioma::Ingles) => "Radial / Circular",
            (ModoDisposicion::FreeDrag, Idioma::Ingles) => "Free Drag",

            (ModoDisposicion::HorizontalTree, Idioma::Frances) => "Arbre équilibré (G/D)",
            (ModoDisposicion::RadialTree, Idioma::Frances) => "Radial / Circulaire",
            (ModoDisposicion::FreeDrag, Idioma::Frances) => "Position libre manuelle",

            (ModoDisposicion::HorizontalTree, Idioma::Aleman) => "Ausgeglichener Baum (Li/Re)",
            (ModoDisposicion::RadialTree, Idioma::Aleman) => "Radial / Kreisförmig",
            (ModoDisposicion::FreeDrag, Idioma::Aleman) => "Freie manuelle Position",

            (ModoDisposicion::HorizontalTree, Idioma::Ruso) => "Сбалансированное дерево (Л/П)",
            (ModoDisposicion::RadialTree, Idioma::Ruso) => "Радиальное / Круговое",
            (ModoDisposicion::FreeDrag, Idioma::Ruso) => "Свободное перемещение вручную",

            (ModoDisposicion::HorizontalTree, Idioma::ChinoSimplificado) => "平衡树（左/右）",
            (ModoDisposicion::RadialTree, Idioma::ChinoSimplificado) => "放射状 / 环形",
            (ModoDisposicion::FreeDrag, Idioma::ChinoSimplificado) => "自由手动拖拽",
        }
    }
}

/// Representa un nodo individual en el mapa mental.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nodo {
    /// Identificador universal único del nodo.
    pub id: Uuid,
    /// ID del nodo padre (`None` si es la idea central raíz).
    pub parent_id: Option<Uuid>,
    /// Lista de IDs de nodos hijos ordenados.
    pub children: Vec<Uuid>,
    /// Título o texto visible del concepto.
    pub title: String,
    /// Notas explicativas o especificación técnica detallada para la IA.
    pub notes: String,
    /// Ruta del archivo de código o módulo asociado (ej. `src/auth/jwt.rs`).
    #[serde(default)]
    pub file_path: Option<String>,
    /// Etiquetas o hashtags temáticos (ej. `["backend", "rust", "db"]`).
    pub tags: Vec<String>,
    /// Estado de madurez del nodo.
    pub status: EstadoNodo,
    /// Prioridad estratégica.
    pub priority: PrioridadNodo,
    /// Rol arquitectónico dentro del mapa.
    pub role: RolNodo,
    /// Estado de supervisión humana (Human-in-the-loop).
    #[serde(default)]
    pub review_status: EstadoRevision,
    /// Notas de corrección explícita exigidas a la IA por el usuario.
    #[serde(default)]
    pub correction_feedback: String,
    /// Coordenadas `[X, Y]` del nodo en el lienzo 2D infinito.
    pub pos: [f32; 2],
    /// Indica si sus ramas hijas están plegadas/ocultas.
    pub collapsed: bool,
}

impl Nodo {
    /// Deja constancia de que una persona ha escrito en este nodo.
    ///
    /// Hay que llamarla desde **todas** las puertas por las que se edita el contenido de un
    /// nodo en la aplicación: el título en el lienzo, el título en su puerta común y las
    /// notas del inspector.
    ///
    /// # Por qué existe
    ///
    /// El servidor MCP decide con `el_mapa_es_solo_de_la_ia` si puede sobrescribir un mapa
    /// sin dejar copia de seguridad, y ese criterio mira el estado de revisión de los nodos:
    /// su documentación dice que «cualquier otro estado significa que hay una persona
    /// detrás». La interfaz no cumplía esa promesa —editaba `title` y `notes` sin tocar el
    /// estado—, así que un mapa **creado por un agente y anotado después a mano** seguía
    /// contando como trabajo exclusivo de la IA.
    ///
    /// La consecuencia, reproducida de extremo a extremo contra el servidor: la persona
    /// escribía su decisión en las notas, el agente sincronizaba su progreso, y la nota
    /// desaparecía **sin dejar ni una copia**, con respuesta `success`. Es el flujo de
    /// trabajo más habitual del programa, y era el único que la valla no cubría.
    ///
    /// # Qué no toca
    ///
    /// Solo asciende desde [`EstadoRevision::GeneradoPorIA`]. Un nodo ya aprobado o marcado
    /// para corrección se queda como está: eso lo decide la persona en el desplegable de
    /// «Control Humano», y escribir una coma en las notas no debe deshacerlo.
    pub fn marcar_editado_por_una_persona(&mut self) {
        if self.review_status == EstadoRevision::GeneradoPorIA {
            self.review_status = EstadoRevision::PendienteRevision;
        }
    }

    /// Indica si la persona ha dejado en este nodo algo que la IA deba acatar.
    ///
    /// Cuenta como corrección cualquiera de estas tres cosas, y basta con una:
    ///
    /// - que el nodo esté marcado como que **requiere corrección**;
    /// - que tenga escrito un comentario de corrección, sea cual sea su estado de revisión;
    /// - que el nodo esté **descartado**, porque descartar es la forma más rotunda de decir
    ///   que no se siga por ahí.
    ///
    /// Vive aquí, y no en quien exporta, porque el criterio estaba escrito por triplicado: en
    /// las dos rutas de exportación a Markdown y en la herramienta que responde a los agentes
    /// por MCP. Afinarlo en una sola habría hecho que el veto del usuario apareciera en el
    /// documento que él copia a mano y no en lo que recibe el agente, o al revés. Esa
    /// asimetría es exactamente contra lo que existe este programa.
    pub fn exige_correccion_del_usuario(&self) -> bool {
        self.review_status == EstadoRevision::RequiereCorreccion
            || !self.correction_feedback.trim().is_empty()
            || self.status == EstadoNodo::Descartado
    }

    /// Crea un nuevo nodo con valores predeterminados.
    ///
    /// El identificador se genera aquí, y el resto de campos —rol, estado, prioridad,
    /// notas, etiquetas y estado de revisión— arrancan en su valor por omisión. El nodo
    /// nace sin hijos: colgarlo de su padre es cosa de [`Proyecto::anadir_hijo`].
    ///
    /// # Parámetros
    /// - `title`: el texto que se verá en el nodo.
    /// - `parent_id`: el identificador del nodo padre, o `None` si es la raíz del mapa.
    /// - `pos`: la posición `[x, y]` en coordenadas del lienzo, no de la pantalla.
    pub fn new(title: impl Into<String>, parent_id: Option<Uuid>, pos: [f32; 2]) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent_id,
            children: Vec::new(),
            title: title.into(),
            notes: String::new(),
            file_path: None,
            tags: Vec::new(),
            status: EstadoNodo::default(),
            priority: PrioridadNodo::default(),
            role: if parent_id.is_none() {
                RolNodo::IdeaCentral
            } else {
                RolNodo::Subtema
            },
            review_status: EstadoRevision::default(),
            correction_feedback: String::new(),
            pos,
            collapsed: false,
        }
    }
}

/// Versión vigente del esquema persistente `.mmcelt`.
pub const VERSION_ESQUEMA: u32 = 1;

/// Identifica el programa que escribió el archivo sin confundirlo con su formato.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneradoPor {
    /// Aplicación autora del archivo.
    pub app: String,
    /// Versión de la aplicación autora.
    pub version: String,
}

impl Default for GeneradoPor {
    /// Devuelve la identidad de esta compilación de MMCelt.
    fn default() -> Self {
        Self {
            app: "MMCelt".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Devuelve la versión del esquema usada al abrir archivos anteriores sin cabecera.
fn version_esquema_actual() -> u32 {
    VERSION_ESQUEMA
}

/// Número monotónico y barato que identifica el estado mutable de un proyecto en memoria.
///
/// No se guarda en el archivo: al abrir un mapa comienza de nuevo en cero. Su única finalidad
/// es permitir que historial y autoguardado descarten en O(1) los fotogramas sin cambios.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RevisionProyecto(u64);

impl RevisionProyecto {
    /// Revisión con la que nace o se deserializa un proyecto.
    pub const INICIAL: Self = Self(0);

    /// Devuelve la revisión siguiente, evitando desbordamientos observables.
    fn siguiente(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Representa el proyecto completo de mapa mental y sus metadatos estratégicos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proyecto {
    /// Versión del contrato JSON del archivo.
    #[serde(default = "version_esquema_actual")]
    pub schema_version: u32,
    /// Aplicación y versión que generaron el archivo.
    #[serde(default)]
    pub generated_by: GeneradoPor,
    /// ID único del proyecto.
    pub id: Uuid,
    /// Título global del mapa mental.
    pub title: String,
    /// Explicación detallada de lo que el creador ha intentado plasmar (Visión).
    pub creator_vision: String,
    /// Objetivos concretos, hitos y resultados esperados en el mundo real.
    pub project_goals: String,
    /// Público objetivo o contexto de aplicación del proyecto.
    pub target_audience_or_context: String,
    /// Nombre del autor o creador del mapa.
    pub author: String,
    /// Fecha de creación del proyecto (UTC).
    pub created_at: DateTime<Utc>,
    /// Fecha de última modificación (UTC).
    pub updated_at: DateTime<Utc>,
    /// ID del nodo raíz central.
    pub root_id: Uuid,
    /// Todos los nodos del mapa, indexados por su identificador.
    ///
    /// Es un mapa **ordenado**, y esa es la diferencia que importa. Con una tabla dispersa el
    /// recorrido sale en un orden distinto en cada ejecución, y eso se filtraba a todo lo que
    /// el programa escribe: el `.mmcelt` guardado, el documento para la IA y su diagrama
    /// salían con los bloques barajados aunque no hubiera cambiado nada. Como el servidor
    /// reescribe ambos en cada informe de progreso, dos sincronizaciones seguidas producían
    /// cientos de líneas de diferencia por nada.
    ///
    /// Se intentó ordenar en cada sitio que recorriera los nodos, y se olvidaron dos. Ordenar
    /// aquí lo arregla de raíz: no hay forma de recorrerlos desordenados.
    pub nodes: BTreeMap<Uuid, Nodo>,
    /// Lista de conexiones cruzadas transversales.
    pub connections: Vec<ConexionCruzada>,
    /// Modo de layout espacial activo.
    pub layout_mode: ModoDisposicion,
    /// Señal de cambio exclusiva de esta ejecución; nunca se escribe en `.mmcelt`.
    #[serde(skip)]
    revision: RevisionProyecto,
}

impl Default for Proyecto {
    /// Crea un proyecto vacío titulado «Nuevo Proyecto».
    ///
    /// No se puede usar `#[derive(Default)]` porque un proyecto válido necesita un nodo
    /// raíz con su identificador ya generado, y los campos derivados por defecto
    /// producirían un mapa sin raíz que no superaría la validación estructural.
    fn default() -> Self {
        Self::nuevo_vacio("Nuevo Proyecto")
    }
}

impl Proyecto {
    /// Crea un proyecto nuevo con un único nodo raíz y sin contenido.
    ///
    /// El nodo raíz recibe el mismo titulo que el proyecto y el rol
    /// [`RolNodo::IdeaCentral`]. El resultado siempre supera
    /// [`Proyecto::validar_estructura`].
    ///
    /// # Parámetros
    /// - `title`: titulo del proyecto y del nodo raíz.
    pub fn nuevo_vacio(title: impl Into<String>) -> Self {
        let title_str = title.into();
        let root_id = Uuid::new_v4();
        let mut nodo_raiz = Nodo::new(title_str.clone(), None, [0.0, 0.0]);
        nodo_raiz.id = root_id;
        nodo_raiz.role = RolNodo::IdeaCentral;
        nodo_raiz.notes = String::new();

        let mut nodes = BTreeMap::new();
        nodes.insert(root_id, nodo_raiz);

        Self {
            schema_version: VERSION_ESQUEMA,
            generated_by: GeneradoPor::default(),
            id: Uuid::new_v4(),
            title: title_str,
            creator_vision: String::new(),
            project_goals: String::new(),
            target_audience_or_context: String::new(),
            author: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            root_id,
            nodes,
            connections: Vec::new(),
            layout_mode: ModoDisposicion::HorizontalTree,
            revision: RevisionProyecto::INICIAL,
        }
    }

    /// Devuelve la revisión actual sin recorrer ni clonar el mapa.
    pub fn revision(&self) -> RevisionProyecto {
        self.revision
    }

    /// Señala que una operación ha modificado contenido significativo del proyecto.
    pub fn marcar_modificado(&mut self) {
        self.revision = self.revision.siguiente();
    }

    /// Devuelve el nodo raíz del mapa mental, si existe.
    ///
    /// Devuelve `Option` en lugar de entrar en panic porque el `root_id` puede no
    /// corresponder a ningún nodo en un archivo dañado. Los proyectos que han pasado
    /// por [`Proyecto::validar_estructura`] siempre devuelven `Some`.
    pub fn nodo_raiz(&self) -> Option<&Nodo> {
        self.nodes.get(&self.root_id)
    }

    // Nota: aquí existían `root_node_mut`, `get_node` y `get_node_mut`. Se eliminaron
    // por no tener ni un solo uso: todo el código accede directamente a `nodes`, que
    // es un `BTreeMap` público y ya ofrece `get` y `get_mut`. Eran una capa de
    // indirección que no aportaba nada y que el `allow(dead_code)` global mantenía
    // oculta.

    /// Crea un nodo hijo colgando del padre indicado y lo inserta en el mapa.
    ///
    /// El rol del nuevo nodo se asigna automáticamente según su profundidad: los hijos
    /// directos de la raíz son [`RolNodo::PilarEstrategico`] y el resto
    /// [`RolNodo::Subtema`]. La posición inicial se sitúa a la derecha del padre;
    /// normalmente se recalcula después con `layout::aplicar_disposicion_automatica`.
    ///
    /// Añadir un hijo **despliega** el padre si estaba plegado, para que el nodo nuevo
    /// sea visible de inmediato.
    ///
    /// # Parámetros
    /// - `parent_id`: identificador del nodo padre. Si no existe, el nodo se crea
    ///   igualmente pero queda huérfano.
    /// - `title`: título del nodo nuevo.
    ///
    /// # Devuelve
    /// El identificador del nodo recién creado.
    pub fn anadir_hijo(&mut self, parent_id: Uuid, title: impl Into<String>) -> Uuid {
        self.updated_at = Utc::now();
        let parent_pos = self
            .nodes
            .get(&parent_id)
            .map(|p| p.pos)
            .unwrap_or([0.0, 0.0]);
        let mut child = Nodo::new(
            title,
            Some(parent_id),
            [parent_pos[0] + 180.0, parent_pos[1]],
        );
        let id_del_hijo = child.id;

        // El rol se deduce de la profundidad: los hijos de la raíz son pilares y el resto,
        // subtemas. Es una suposición razonable que el usuario puede cambiar después.
        let profundidad = self.profundidad_del_nodo(&parent_id);
        if profundidad == 0 {
            child.role = RolNodo::PilarEstrategico;
        } else {
            child.role = RolNodo::Subtema;
        }

        self.nodes.insert(id_del_hijo, child);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id_del_hijo);
            parent.collapsed = false;
        }

        self.marcar_modificado();
        id_del_hijo
    }

    /// Crea un nodo hermano del indicado, es decir, hijo de su mismo padre.
    ///
    /// Si el nodo de referencia es la raíz —que no tiene padre— el nuevo nodo se crea
    /// como hijo suyo, que es el comportamiento esperado desde la interfaz.
    ///
    /// # Parámetros
    /// - `target_id`: nodo de referencia junto al que se crea el hermano.
    /// - `title`: título del nodo nuevo.
    ///
    /// # Devuelve
    /// El identificador del nodo creado, o `None` si el nodo de referencia no existe
    /// o carece de padre.
    pub fn anadir_hermano(&mut self, target_id: Uuid, title: impl Into<String>) -> Option<Uuid> {
        if target_id == self.root_id {
            return Some(self.anadir_hijo(self.root_id, title));
        }

        let parent_id = self.nodes.get(&target_id)?.parent_id?;
        let sibling_id = self.anadir_hijo(parent_id, title);
        Some(sibling_id)
    }

    /// Elimina un nodo **y toda su descendencia** en cascada.
    ///
    /// También descarta las conexiones cruzadas en las que participe cualquiera de los
    /// nodos eliminados, para no dejar referencias colgantes en el grafo.
    ///
    /// La raíz no se puede eliminar: si se solicita, se vacía su contenido borrando
    /// todos sus hijos, pero el nodo raíz permanece.
    ///
    /// # Parámetros
    /// - `id_del_nodo`: identificador del nodo a eliminar. Si no existe, no hace nada.
    pub fn eliminar_nodo(&mut self, id_del_nodo: Uuid) {
        if id_del_nodo == self.root_id {
            let revision_anterior = self.revision;
            // La raíz no se elimina: se vacía. Se toman los hijos y se borra cada
            // subárbol, dejando el nodo raíz en pie.
            let children = self
                .nodo_raiz()
                .map(|raiz| raiz.children.clone())
                .unwrap_or_default();
            for c in children {
                self.eliminar_nodo(c);
            }
            if self.revision != revision_anterior {
                self.revision = revision_anterior.siguiente();
            }
            return;
        }

        if !self.nodes.contains_key(&id_del_nodo) {
            return;
        }

        self.updated_at = Utc::now();

        // Toda la descendencia, para borrarla con el nodo.
        let mut por_borrar = Vec::new();
        self.recoger_descendientes(id_del_nodo, &mut por_borrar);
        por_borrar.push(id_del_nodo);

        // Se descuelga del padre antes de borrarlo, para no dejar una referencia a un nodo
        // que ya no existe.
        if let Some(parent_id) = self.nodes.get(&id_del_nodo).and_then(|n| n.parent_id) {
            if let Some(parent) = self.nodes.get_mut(&parent_id) {
                parent.children.retain(|&id| id != id_del_nodo);
            }
        }

        // Las conexiones cruzadas que tocaban a los nodos borrados se van con ellos: una
        // conexión a un nodo inexistente rompería el dibujado.
        self.connections
            .retain(|c| !por_borrar.contains(&c.from) && !por_borrar.contains(&c.to));

        // Y por último se retiran del mapa de nodos.
        for id in por_borrar {
            self.nodes.remove(&id);
        }
        self.marcar_modificado();
    }

    /// Reúne todos los descendientes de un nodo, a cualquier profundidad.
    ///
    /// El recorrido es **iterativo y con registro de visitados**. La versión recursiva
    /// anterior agotaba la pila del proceso ante un mapa con un ciclo o con una rama
    /// muy larga. Aunque [`Proyecto::validar_estructura`] ya impide que tales mapas se
    /// carguen, esta función mantiene su propia defensa: es la que se ejecuta al
    /// borrar, sobre datos que el usuario ha podido modificar después de la carga.
    ///
    /// # Parámetros
    /// - `id_del_nodo`: nodo cuya descendencia se quiere reunir (él mismo no se incluye).
    /// - `result`: vector donde se acumulan los identificadores encontrados.
    fn recoger_descendientes(&self, id_del_nodo: Uuid, result: &mut Vec<Uuid>) {
        let mut visitados: HashSet<Uuid> = HashSet::new();
        visitados.insert(id_del_nodo);

        let mut pendientes: Vec<Uuid> = vec![id_del_nodo];

        while let Some(actual) = pendientes.pop() {
            let Some(nodo) = self.nodes.get(&actual) else {
                continue;
            };

            for &id_hijo in &nodo.children {
                // `insert` devuelve false si ya estaba: así un ciclo se corta en seco
                // en lugar de repetirse para siempre.
                if visitados.insert(id_hijo) {
                    result.push(id_hijo);
                    pendientes.push(id_hijo);
                }
            }
        }
    }

    /// Dice a qué rama principal pertenece cada nodo.
    ///
    /// Las «ramas principales» son los hijos directos de la raíz, y el índice es su posición
    /// entre ellos: es lo que [`crate::theme::ThemeConfig::color_de_la_rama`] espera recibir
    /// para elegir un color, y lo que hace que todo lo que cuelga de un mismo pilar comparta
    /// el suyo.
    ///
    /// La ayuda del programa se lo promete al usuario con esas palabras: que puede «seguir
    /// cualquier hilo hasta el final y saber de dónde viene». Las líneas ya se dibujaban así;
    /// la barrita lateral de cada tarjeta, en cambio, usaba la **profundidad**, de modo que
    /// todos los nodos de un mismo nivel salían del mismo color vinieran de donde vinieran, y
    /// una tarjeta podía lucir una barra verde al final de una línea azul.
    ///
    /// # Devuelve
    /// El índice de rama de cada nodo alcanzable desde la raíz. La raíz no aparece: no
    /// pertenece a ninguna rama, es de donde salen todas.
    pub fn rama_de_cada_nodo(&self) -> HashMap<Uuid, usize> {
        let mut ramas: HashMap<Uuid, usize> = HashMap::new();

        let Some(raiz) = self.nodes.get(&self.root_id) else {
            return ramas;
        };

        for (indice, &id_de_la_rama) in raiz.children.iter().enumerate() {
            // Cada rama se recorre entera antes de pasar a la siguiente, marcando lo que
            // encuentra. Lo ya marcado no se vuelve a recorrer: un nodo alcanzable por dos
            // caminos —cosa que un archivo manipulado a mano puede traer— se queda con la
            // primera rama que lo alcance, en lugar de hacer que el recorrido no termine.
            //
            // Se comprueba con `entry` y no con `insert`, que era lo que había: `insert`
            // **sobrescribe** y luego devuelve lo que hubiera, así que el nodo acababa
            // marcado con la última rama que lo alcanzara, justo lo contrario de lo que
            // dice el párrafo de arriba. El recorrido terminaba igual, pero por lo que
            // decía el código, no por lo que decía su comentario.
            let mut pendientes = vec![id_de_la_rama];
            while let Some(actual) = pendientes.pop() {
                let Entry::Vacant(sin_rama) = ramas.entry(actual) else {
                    continue;
                };
                sin_rama.insert(indice);
                let Some(nodo) = self.nodes.get(&actual) else {
                    continue;
                };
                pendientes.extend(nodo.children.iter().copied());
            }
        }

        ramas
    }

    /// Devuelve los nodos que no deben verse por tener algún ancestro plegado.
    ///
    /// Plegar una rama solo dejaba de dibujar sus líneas y de recolocarla: las tarjetas de
    /// toda la descendencia seguían en pantalla, ahora sueltas y sin nada que las uniera al
    /// mapa. Y como la disposición automática ya no las tenía en cuenta, al reorganizar el
    /// resto se recomponía alrededor y las plegadas quedaban encima, superpuestas.
    ///
    /// El recorrido es iterativo y con marca de visitados, como los demás de este módulo: un
    /// mapa con un ciclo padre-hijo colgaría la versión recursiva.
    ///
    /// # Devuelve
    /// El conjunto de identificadores que hay que saltarse al dibujar. Los nodos plegados no
    /// están dentro: son los que hay que seguir viendo, con su `+` para desplegarlos.
    pub fn nodos_ocultos_por_plegado(&self) -> HashSet<Uuid> {
        let mut ocultos: HashSet<Uuid> = HashSet::new();
        let mut visitados: HashSet<Uuid> = HashSet::new();
        visitados.insert(self.root_id);

        // Cada entrada lleva si el nodo queda oculto por lo que hay por encima de él.
        let mut pendientes: Vec<(Uuid, bool)> = vec![(self.root_id, false)];

        while let Some((actual, oculto)) = pendientes.pop() {
            if oculto {
                ocultos.insert(actual);
            }

            let Some(nodo) = self.nodes.get(&actual) else {
                continue;
            };

            // Los hijos se ocultan si ya venían ocultos o si este nodo está plegado.
            let ocultar_hijos = oculto || nodo.collapsed;
            for &id_hijo in &nodo.children {
                if visitados.insert(id_hijo) {
                    pendientes.push((id_hijo, ocultar_hijos));
                }
            }
        }

        ocultos
    }

    /// Calcula a qué profundidad se encuentra un nodo, contando desde la raíz.
    ///
    /// La raíz está a profundidad 0, sus hijos a 1, y así sucesivamente.
    ///
    /// El ascenso por la cadena de padres lleva **registro de los nodos visitados**.
    /// La versión anterior no lo hacía y, ante un ciclo padre-hijo, entraba en un bucle
    /// infinito que congelaba la aplicación por completo, sin panic ni mensaje. Ante un
    /// ciclo, esta versión detiene el ascenso y devuelve la profundidad acumulada
    /// hasta ese punto.
    ///
    /// # Parámetros
    /// - `id_del_nodo`: identificador del nodo consultado.
    ///
    /// # Devuelve
    /// El número de saltos que separan el nodo de la raíz. Devuelve 0 si el nodo no
    /// existe.
    pub fn profundidad_del_nodo(&self, id_del_nodo: &Uuid) -> usize {
        let mut profundidad = 0;
        let mut current = *id_del_nodo;
        let mut visitados: HashSet<Uuid> = HashSet::new();
        visitados.insert(current);

        while let Some(node) = self.nodes.get(&current) {
            let Some(parent_id) = node.parent_id else {
                break; // Se alcanzó la raíz: fin normal del ascenso.
            };

            if !visitados.insert(parent_id) {
                break; // Ciclo detectado: se corta para no colgar el proceso.
            }

            profundidad += 1;
            current = parent_id;
        }

        profundidad
    }

    /// Crea una conexión cruzada entre dos nodos distintos.
    ///
    /// No mira la jerarquía a propósito: la usan los importadores y el servidor MCP, y lo que
    /// trae un archivo o un agente —incluida una relación etiquetada entre padre e hijo— se
    /// conserva tal cual. La interfaz pregunta antes a [`Proyecto::conexion_cruzada_admitida`]
    /// para no dibujar una segunda línea encima de la de la jerarquía (PH-1007-2).
    ///
    /// # Parámetros
    /// - `from`: nodo de origen de la relación.
    /// - `to`: nodo de destino.
    /// - `label`: explicación del motivo de la relación.
    /// - `relation_type`: naturaleza semántica de la conexión.
    ///
    /// # Devuelve
    /// El identificador de la conexión creada, o `None` si origen y destino coinciden
    /// o si alguno de los dos no existe en el mapa.
    pub fn anadir_conexion_cruzada(
        &mut self,
        from: Uuid,
        to: Uuid,
        label: impl Into<String>,
        relation_type: TipoRelacion,
    ) -> Option<Uuid> {
        if from == to || !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
            return None;
        }

        let id = Uuid::new_v4();
        self.connections.push(ConexionCruzada {
            id,
            from,
            to,
            label: label.into(),
            relation_type,
        });
        self.updated_at = Utc::now();
        self.marcar_modificado();
        Some(id)
    }

    /// Elimina una conexión cruzada por su identificador.
    ///
    /// # Parámetros
    /// - `id`: identificador de la conexión. Si no existe, no hace nada.
    pub fn quitar_conexion_cruzada(&mut self, id: Uuid) {
        let cantidad_anterior = self.connections.len();
        self.connections.retain(|c| c.id != id);
        if self.connections.len() != cantidad_anterior {
            self.updated_at = Utc::now();
            self.marcar_modificado();
        }
    }

    /// Deduce la carpeta del proyecto de código al que se refiere el mapa.
    ///
    /// Muchos mapas describen un proyecto real: los que genera el escáner de repositorios
    /// y los que produce una IA que está trabajando sobre un código concreto llevan la
    /// ruta del archivo correspondiente en el campo `file_path` de sus nodos.
    ///
    /// Esta función busca el **prefijo común más largo** de todas esas rutas, que es la
    /// carpeta raíz del proyecto.
    ///
    /// # Para qué sirve
    ///
    /// Para **proponer** un destino al guardar, no para elegirlo. Un mapa de un proyecto
    /// suele querer vivir junto a ese proyecto, y sugerir esa carpeta ahorra al usuario
    /// navegar hasta ella.
    ///
    /// La sugerencia nunca se aplica sola. Escribir por iniciativa propia en una carpeta
    /// deducida podría colar archivos dentro de un repositorio ajeno sin que el usuario
    /// se diera cuenta, así que la decisión sigue siendo suya: la carpeta solo llega
    /// preseleccionada al diálogo de guardar.
    ///
    /// # Devuelve
    /// La carpeta común a todas las rutas del mapa, o `None` si el mapa no referencia
    /// ningún archivo, si las rutas no comparten raíz (por ejemplo, unidades distintas en
    /// Windows) o si la carpeta deducida ya no existe en el disco.
    pub fn carpeta_proyecto_sugerida(&self) -> Option<std::path::PathBuf> {
        use std::path::{Component, Path, PathBuf};

        // Se parte de las rutas absolutas existentes: una ruta relativa como
        // `src/domain/` (las que traen las plantillas) no identifica ninguna carpeta real.
        let rutas: Vec<PathBuf> = self
            .nodes
            .values()
            .filter_map(|nodo| nodo.file_path.as_deref())
            .map(Path::new)
            .filter(|ruta| ruta.is_absolute())
            .map(PathBuf::from)
            .collect();

        if rutas.is_empty() {
            return None;
        }

        // La carpeta de partida de cada ruta: el propio directorio si lo es, y si no, el
        // que lo contiene.
        let carpetas: Vec<Vec<Component>> = rutas
            .iter()
            .map(|ruta| {
                let base = if ruta.is_dir() {
                    ruta.as_path()
                } else {
                    ruta.parent().unwrap_or(ruta.as_path())
                };
                base.components().collect()
            })
            .collect();

        // Prefijo común: se avanza componente a componente mientras todas coincidan.
        let primera = &carpetas[0];
        let mut comunes: Vec<Component> = Vec::new();

        for (indice, componente) in primera.iter().enumerate() {
            let todas_coinciden = carpetas
                .iter()
                .all(|ruta| ruta.get(indice) == Some(componente));

            if !todas_coinciden {
                break;
            }
            comunes.push(*componente);
        }

        // La raíz de un disco no es «la carpeta del proyecto»: es demasiado genérica para
        // proponerla, y abrir ahí el diálogo de guardado desconcierta.
        //
        // Contar componentes no bastaba. En Linux y macOS `/` es uno solo, pero en Windows
        // `C:\` son **dos** —el prefijo del disco y la raíz—, así que pasaba el filtro: bastaba
        // con que el mapa tuviera rutas absolutas bajo dos carpetas distintas del mismo disco
        // para que el diálogo se abriera en la raíz. Se comprueba, en su lugar, si queda algo
        // con nombre después de la parte de la raíz.
        let tiene_nombre_propio = comunes
            .iter()
            .any(|componente| matches!(componente, std::path::Component::Normal(_)));

        if !tiene_nombre_propio {
            return None;
        }

        let carpeta: PathBuf = comunes.iter().collect();

        // Solo se sugiere si existe de verdad: la ruta pudo escribirla una IA que se la
        // inventó, o proceder de otro equipo.
        if carpeta.is_dir() {
            Some(carpeta)
        } else {
            None
        }
    }

    /// Reune todas las etiquetas usadas en el mapa, sin repeticiones y ordenadas.
    ///
    /// # Devuelve
    /// La lista de etiquetas distintas presentes en cualquier nodo.
    pub fn todas_las_etiquetas(&self) -> Vec<String> {
        let mut tags = Vec::new();
        for node in self.nodes.values() {
            for tag in &node.tags {
                if !tags.contains(tag) {
                    tags.push(tag.clone());
                }
            }
        }
        tags.sort();
        tags
    }

    /// Devuelve todos los nodos que se encuentran en el estado indicado.
    ///
    /// # Parámetros
    /// - `status`: estado por el que filtrar.
    ///
    /// # Devuelve
    /// Los nodos coincidentes, en orden no determinado (el del mapa interno).
    pub fn nodos_con_estado(&self, status: EstadoNodo) -> Vec<&Nodo> {
        self.nodes.values().filter(|n| n.status == status).collect()
    }
}
