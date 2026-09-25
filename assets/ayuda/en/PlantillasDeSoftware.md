# ✨ Architecture Templates

Starting a software project from a blank canvas can be overwhelming. That is why MMCelt includes **preconfigured architecture templates** based on modern engineering best practices, ready to use with a single click.

These templates are not mere decorative sketches: they come pre-organized into logical layers, with semantic colors, component relationship types, and explanatory notes that guide both your design and the responses of artificial intelligence.

### 🏛️ Templates Available in MMCelt

**1. Clean Architecture (Hexagonal Architecture):**
This template helps you create robust, maintainable, and easily testable systems, where your core business rules remain completely isolated from technical details and external libraries.
It is organized into four concentric layers:
- **Domain (Core):** Business entities and rules that should never change when you switch databases or frameworks.
- **Use Cases (Application):** Specific operations that the user or system can execute (e.g., register user, process order).
- **Infrastructure (Adapters):** Communication with the outside world: SQL/NoSQL databases, external API calls, messaging systems, and filesystems.
- **Presentation (API & Controllers):** Entry points into the system, such as REST controllers, GraphQL endpoints, or graphical interfaces.

*Includes cross connections representing dependency inversion: outer layers know about inner layers, but the core never depends on infrastructure.*

**2. Fullstack Web App:**
Ideal if you are planning a modern web application end-to-end:
- **Frontend (Client):** The visual interface seen by users (interactive components, state management, and layout).
- **Backend (Server):** The API containing business logic, authentication, authorization, and validations.
- **Database & Persistence:** The relational or document data model, migrations, and high-performance caches.
- **DevOps & Infrastructure:** Docker container configuration, continuous integration (CI/CD) pipelines, and cloud deployment.

### 🚀 How to load a template

1. Go to the top menu **`📁 File`** → **`✨ Architecture Templates`**.
2. Choose the template that best fits your goal (**Clean Architecture** or **Fullstack Web App**).
3. MMCelt will load the full structure onto the canvas.
4. From there, you can customize each branch: add your own models, rename services, define priorities, and click `🤖 Artificial Intelligence` → `📤 Send to...` to have your AI agent start writing code following this guide.
