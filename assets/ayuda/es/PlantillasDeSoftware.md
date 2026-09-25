# ✨ Plantillas de Arquitectura de Software

Comenzar un proyecto de software desde una hoja en blanco puede ser abrumador. Por eso, MMCelt incluye **plantillas de arquitectura preconfiguradas** basadas en las mejores prácticas de la ingeniería moderna, listas para usar con un solo clic.

Estas plantillas no son simples esquemas decorativos: ya vienen organizadas por capas lógicas, con colores semánticos, tipos de relación entre componentes y notas explicativas que guían tanto tu diseño como las respuestas de la inteligencia artificial.

### 🏛️ Plantillas Disponibles en MMCelt

**1. Clean Architecture (Arquitectura Limpia y Hexagonal):**
Esta plantilla te ayuda a crear sistemas robustos, mantenibles y fáciles de probar, donde las reglas fundamentales de tu negocio están completamente aisladas de los detalles técnicos y las librerías externas.
Se organiza en cuatro capas concéntricas:
- **Dominio (Core):** Las entidades y reglas del negocio que nunca deberían cambiar cuando cambias de base de datos o de framework.
- **Casos de Uso (Aplicación):** Las operaciones concretas que el usuario o el sistema puede ejecutar (por ejemplo: registrar usuario, procesar pedido).
- **Infraestructura (Adaptadores):** La comunicación con el mundo exterior: bases de datos SQL/NoSQL, llamadas a APIs externas, sistemas de mensajería y ficheros.
- **Presentación (API y Controladores):** Los puntos de entrada al sistema, como controladores REST, endpoints GraphQL o interfaces visuales.

*Incluye conexiones cruzadas que representan la inversión de dependencias: las capas externas conocen a las internas, pero el núcleo nunca depende de la infraestructura.*

**2. Fullstack Web App (Aplicación Web Completa):**
Ideal si estás planificando una aplicación web moderna de principio a fin:
- **Frontend (Cliente):** La interfaz visual que ve el usuario (componentes interactivos, gestión de estado y diseño).
- **Backend (Servidor):** La API con la lógica de negocio, autenticación, autorización y validaciones.
- **Base de Datos y Persistencia:** El modelo de datos relacional o documental, migraciones y cachés de alto rendimiento.
- **DevOps e Infraestructura:** Configuración de contenedores Docker, canalizaciones de integración continua (CI/CD) y despliegue en la nube.

### 🚀 Cómo cargar una plantilla

1. Ve al menú superior **`📁 Archivo`** → **`✨ Plantillas de Arquitectura`**.
2. Elige la plantilla que mejor se adapte a tu objetivo (**Clean Architecture** o **Fullstack Web App**).
3. MMCelt cargará la estructura completa en el lienzo.
4. A partir de ahí, puedes personalizar cada rama: añade tus propios modelos, renombra servicios, define prioridades y pulsa `🤖 Inteligencia Artificial` → `📤 Enviar a...` para pedirle a tu agente de IA que empiece a escribir el código siguiendo esta guía.
