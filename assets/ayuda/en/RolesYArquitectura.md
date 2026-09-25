# 🏛️ Semantic Roles & Map Architecture

As a project expands, a mind map full of identical boxes can become confusing. To prevent this, MMCelt allows you to assign a **semantic or architectural role** to each node.

Roles define the nature of each idea and its weight within the project. This allows both you and artificial intelligence models to instantly distinguish a strategic pillar from a routine task or an open question.

### 📋 The 6 Roles Available in the Inspector

When selecting any node on the map, you will see the **"Role"** dropdown in the right side panel (Inspector). You can choose between six functions:

1. **🎯 Central Idea:**
   - The root node of the mind map, the origin of the entire tree.
   - Represents the product, company, research, or application you are building. A map contains only one central idea.

2. **🏛️ Strategic Pillar:**
   - The major structural columns supporting the project.
   - In software they typically represent primary layers (such as Frontend, Backend, Database, or Security); in business they could be Marketing, Sales, Finance, or Operations.

3. **📌 Subtopic / Module:**
   - Specific components or sections branching from a strategic pillar.
   - For example, within a *Backend* pillar, subtopics might be *Authentication Service*, *Payment Gateway*, or *Notification Engine*.

4. **❓ Hypothesis / Question:**
   - Open questions, architectural decisions, or experiments whose technical feasibility remains unknown.
   - For example: *“Should we use WebSockets or Server-Sent Events for the live chat?”*.

5. **⚡ Task / Action:**
   - Concrete, actionable, and executable steps with a well-defined deliverable.
   - For example: *“Design the user sign-in screen”* or *“Write unit tests for the pricing calculator”*.

6. **🔧 Resource / Tool:**
   - External libraries, dependencies, reference documentation, third-party APIs, or auxiliary tools required for the project.

### 💡 Changing a role and its impact on the map and AI

To change a node's role, select it and choose the desired option from the **"Role"** dropdown in the right side inspector.

**🎨 Visual distinction on the canvas:**
Each role brings a clear identity to the node card on the visual canvas:
- Distinctive icons and borders to identify them effortlessly.
- Connections and lines that reflect the organic structure of the project.

**🤖 Why do roles matter to AI?**
When you export or send your map to an AI (such as Claude Code, Codex CLI, or Gemini CLI), the model does not see a flat list of text strings:
- **Understands the architecture:** Knows that a *Strategic Pillar* demands high-level architectural consistency and must not be altered lightly.
- **Distinguishes doubts from certainties:** Treats *Hypotheses / Questions* as open design queries requiring technical analysis and trade-offs.
- **Generates coherent tasks:** When asked to flesh out a module, the AI will propose concrete *Tasks / Actions* and suggest recommended *Resources / Tools* to solve it.
