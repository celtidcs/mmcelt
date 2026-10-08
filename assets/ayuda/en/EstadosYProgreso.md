# 📊 Lifecycle States, Progress, and Priorities

A mind map in MMCelt is not just a static drawing: it is a live tracking dashboard for your project. Each node can have a progress state and a priority that help you know at a glance what is ready, what is being built, and what uncertainties block the path.

### 🏷️ The 6 States of a Node

When you select a node and look at the **side Inspector** (the panel on the right side of the screen), you can assign any of these six states with a single click:

1. **💡 Idea:** A preliminary proposal or suggestion that has not yet been evaluated. It is the perfect initial state for brainstorming sessions.
2. **🔍 Researching:** Task or concept in the technical exploration phase, reading documentation, or feasibility study before starting to code.
3. **⏳ In Progress:** Active work that is currently taking place.
4. **❓ Blocker / Question:** Critical point where development is stopped because a decision needs to be made or a technical unknown needs resolution.
5. **✅ Completed:** Successfully finished, tested, and validated task or component.
6. **⛔ Dismissed:** An option that was explored but decided against implementation. Leaving it on the map as dismissed is very valuable to avoid stumbling on the same idea again later.

**⚡ Priorities (🔽 Low, 🔷 Medium, ⚡ High, 🔥 Critical):**
Alongside the state, you can set the urgency of each branch. Both the Inspector dropdown and the node on the canvas display these icons:
- **🔽 Low and 🔷 Medium:** Optional enhancements, secondary tasks, or regular day-to-day work.
- **⚡ High:** Core modules and high-priority components that should be built as soon as possible.
- **🔥 Critical:** Maximum urgencies or critical blockers preventing the rest of the project from moving forward.

### 🤖 How do states influence your work with AI?

The states of your nodes are not just colors for you; artificial intelligence reads and interprets them with great care:
- **Priority to your blockers:** All nodes marked as **`❓ Blocker / Question`** are grouped in a prominent section of the prompt document. The AI knows it must focus on resolving those unknowns before inventing new things.
- **Respect for what is dismissed:** If you mark a branch as **`⛔ Dismissed`**, the AI understands that this path was intentionally rejected and will not insist on proposing it to you.
- **Context of completed work:** The **`✅ Completed`** nodes indicate to the AI which parts of your system already exist and work, so it builds upon them without duplicating effort.
- **Automatic metrics:** In the export header, MMCelt calculates an overall summary (completion percentage, completed vs pending tasks) so the model knows the exact maturity phase of the project.

### 💡 What each card icon means
Hover over an icon to see what it means: the emoji before the title is the **status**; the ones in the top-right corner are the **priority** and the **human control**, for example `Priority: ⚡ High`; at the bottom right, the node's **Role**, for example `Role: 📌 Subtopic / Module` (the ⚡ of "Task / Action" is not the High priority one: the tooltip tells them apart); and the 📝 at the bottom says `📝 Has notes: select it to read them in the inspector`. It does not appear while you drag a node.
