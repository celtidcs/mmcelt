---
name: mmcelt-mindmap
description: Genera, analiza y actualiza mapas mentales interactivos compatibles con la aplicación de escritorio MMCelt y exporta documentos Markdown enriquecidos para razonamiento profundo con IA.
---

# Mapas mentales de MMCelt

MMCelt es una aplicación de escritorio donde una persona dibuja el plano de su proyecto y
supervisa lo que hace un modelo de IA sobre él. Esta habilidad sirve para leer esos mapas,
ampliarlos y devolverlos sin romper nada.

Lo importante de MMCelt no es que dibuje mapas mentales, sino que el mapa viaja en los dos
sentidos. Cuando lo recibas, puede llevar nodos que la persona ha marcado como descartados
o con una corrección exigida: eso manda sobre cualquier propuesta tuya.

## Cuándo usar este Skill:
- Cuando el usuario te pida crear un mapa mental visual para un proyecto, idea, arquitectura de software, investigación o sesión de brainstorming.
- Cuando el usuario te proporcione un archivo `.mmcelt` (JSON) o un archivo `.md` exportado por MMCelt y te pida resolver dudas, expandir ramas o diseñar un plan de ejecución.
- Cuando el usuario quiera convertir una conversación larga o especificaciones de código en un mapa mental interactivo.

---

## 1. Formato de Salida JSON para MMCelt

Cuando el usuario te pida crear o estructurar un mapa mental para MMCelt, devuelve siempre un bloque de código `json` con la siguiente estructura:

```json
{
  "title": "Nombre del Proyecto",
  "creator_vision": "Explicación detallada de lo que se busca plasmar y la visión estratégica",
  "project_goals": "Objetivos concretos, hitos y resultados esperados en el mundo real",
  "target_audience_or_context": "Público objetivo, usuarios o entorno de aplicación",
  "root_node": {
    "title": "Idea Central",
    "notes": "Descripción conceptual central",
    "role": "IdeaCentral",
    "status": "Idea",
    "priority": "Alta",
    "tags": ["core"],
    "children": [
      {
        "title": "Pilar 1: Arquitectura y Backend",
        "notes": "Especificación de tecnologías, APIs y modelo de datos...",
        "file_path": "src/backend/",
        "role": "PilarEstrategico",
        "status": "EnProgreso",
        "priority": "Alta",
        "tags": ["backend", "rust"],
        "children": [
          {
            "title": "Módulo de Autenticación JWT",
            "notes": "Generación y verificación de tokens con rotación y middleware.",
            "file_path": "src/backend/auth.rs",
            "role": "Subtema",
            "status": "Idea",
            "priority": "Alta",
            "tags": ["auth", "security"],
            "review_status": "GeneradoPorIA",
            "correction_feedback": ""
          },
          {
            "title": "¿Qué motor de persistencia usar?",
            "notes": "Evaluar SQLite con WAL vs PostgreSQL local según volumen de datos.",
            "file_path": "src/backend/db.rs",
            "role": "HipotesisDuda",
            "status": "DudaBloqueo",
            "priority": "Critica",
            "tags": ["database", "duda"]
          }
        ]
      },
      {
        "title": "Pilar 2: Estrategia de Adopción",
        "notes": "Distribución portable de binarios sin instalador...",
        "role": "PilarEstrategico",
        "status": "Idea",
        "priority": "Media",
        "tags": ["distribucion"]
      }
    ]
  },
  "cross_connections": [
    {
      "from_title": "¿Qué motor de persistencia usar?",
      "to_title": "Pilar 2: Estrategia de Adopción",
      "label": "Impacto en portabilidad de un solo binario",
      "relation_type": "Dependencia"
    }
  ]
}
```

### Diccionario de Valores Válidos:
- **`role`**:
  - `"IdeaCentral"`: Concepto raíz del mapa.
  - `"PilarEstrategico"`: Primer nivel de temas estructurales.
  - `"Subtema"`: Ramificaciones y detalles.
  - `"HipotesisDuda"`: Preguntas abiertas, supuestos o decisiones pendientes.
  - `"AccionTarea"`: Tareas concretas ejecutables.
  - `"RecursoHerramienta"`: Librerías, documentación, APIs o herramientas.
- **`status`**:
  - `"Idea"` (💡 Idea por madurar)
  - `"Investigando"` (🔍 En análisis técnico)
  - `"EnProgreso"` (⏳ En desarrollo activo)
  - `"DudaBloqueo"` (❓ Bloqueo o duda crítica que requiere decisión)
  - `"Completado"` (✅ Validado o terminado)
  - `"Descartado"` (⛔ No viable o descartado)
- **`priority`**: `"Baja"`, `"Media"`, `"Alta"`, `"Critica"`.
- **`relation_type`**: `"Dependencia"`, `"InspiradoPor"`, `"Bloquea"`, `"AlternativaA"`, `"Sinergia"`.
- **`file_path`**: la ruta del archivo o la carpeta que representa el nodo, relativa a la
  raíz del proyecto (`"src/backend/auth.rs"`). Ponla siempre que el nodo corresponda a
  código real: es lo que permite seguir el mapa hasta el archivo.
- **`review_status`**: quién ha revisado ese nodo.
  - `"GeneradoPorIA"`: lo has escrito tú y nadie lo ha mirado todavía. **Marca así todo lo
    que añadas.**
  - `"PendienteRevision"`: está a la espera de que lo mire una persona.
  - `"AprobadoPorHumano"`: validado. No lo cambies sin que te lo pidan.
  - `"RequiereCorreccion"`: la persona ha encontrado algo mal. Lee `correction_feedback`.
- **`correction_feedback`**: el texto de la corrección exigida. **Si un nodo lo trae, es una
  orden, no una sugerencia.** Consérvalo tal cual al devolver el mapa: borrarlo equivale a
  ignorar lo que ha decidido la persona.

Los tres campos viajan en ambos sentidos: MMCelt los escribe al exportar y los lee al
importar, así que lo que pongas en ellos llega al mapa que la persona tiene delante.

---

## 2. Flujo de Lectura y Expansión de Mapas `.md` de MMCelt

Si el usuario te proporciona un documento `.md` exportado por MMCelt:
1. **Lee primero la sección 1 (`VISIÓN DEL CREADOR Y CONTEXTO`)** para entender la intención real.
2. **Prioriza la sección 5 (`PUNTOS DE DECISIÓN, DUDAS Y BLOQUEOS`)**: Para cada duda listada, analiza opciones viables y sugiere la solución técnica óptima.
3. **Ofrece nuevos subtemas estructurados** en formato compatible con MMCelt para que el usuario los pegue con el botón **"Importar desde IA"**.
