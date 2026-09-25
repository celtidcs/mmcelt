# 📤 'Send to...', Agent Console & Live Watch

'Send to...' sets up a supervised session with a coding agent and keeps the loop closed. If the target has an official console, the conversation happens there; if it only speaks MCP, MMCelt leaves everything prepared and the agent picks it up from its own interface. Either way, the work comes back to the mind map through the MMCelt MCP server.

### 👁️ Picking an agent does not send anything yet
Clicking an agent opens a **preview** of the complete prompt, split into four blocks:

1. **MMCelt contract** (read-only): the minimum integrity rules. Discover the authorised workspace, read human decisions before writing, never edit the `.mmcelt` file by hand, and report progress through MCP. It is shown protected so it cannot be deleted by accident.
2. **Common project rules** (editable): your neutral profile, the same one for every agent.
3. **Mind map context** (read-only): produced by the AI export. You change it in the map, which is its source of truth.
4. **Task for this session** (editable): what you want done now. It is not carried over to the next session.

`Restore this session` puts rules and task back as they were. `Save as common rules` is a separate action: only that button writes the project profile to `.mmcelt/instrucciones-agente.md`. `Cancel` saves nothing, exports nothing, watches nothing and leaves no session record.

### ⚖️ Precedence stated inside the prompt itself
Human corrections in the map **>** MMCelt contract **>** project rules **>** task for this session.

### 📚 Native instructions are shown, not pasted
If the project root holds `AGENTS.md`, `CLAUDE.md` or `GEMINI.md`, the preview lists them and lets you read them, but **does not merge them** into the shared prompt: each tool discovers its own file with its own scope, and copying them all would create duplicated, contradictory rules.

### 🚀 What happens when you confirm
The button states what will actually happen: `Start in console` when MMCelt has located the agent console, and `Prepare for MCP` when there is none. In that second case the preview warns you beforehand, with a highlighted notice above the agent name.

**With a console**, the order is fixed, and every step must succeed before the next one runs:

1. **Saves the mind map** to its `.mmcelt` file.
2. **Exports the AI document** next to the map, with the `_AI.md` suffix.
3. **Writes the session record**.
4. **Opens the agent console** in the project folder.
5. **Starts watching** the map.

If the console never opens, MMCelt **does not claim it was sent**, does not start watching, and marks the session record as failed.

**Without a console** —an agent whose MCP server is registered but whose executable MMCelt cannot find— everything else still happens: it saves the map, exports the `_AI.md`, writes the session record and starts watching. Only the fourth step is skipped, and the record stays in the **prepared** state instead of started, because MMCelt launched nothing. The agent picks the work up from its own interface and reports back through MCP like any other.

### 🗄️ A project from an older version is not touched just by looking at it
A folder used with an earlier version does not have `.mmcelt/configuracion.json` yet. Opening the preview **does not create it**: the prompt is composed in memory and nothing is written to disk. The project identity is written when you confirm, which is the explicit gesture. If you cancel, the folder stays exactly as it was.

### 🗂️ The session record: `.mmcelt/sesiones`
Every confirmation creates a folder named after the date, the agent and an identifier. Inside it there are two files:

- `inicio.md`: the exact prompt you approved, character for character.
- `sesion.json`: format version, identifier, UTC date, agent, map path, detected and merged sources, prompt size and state (prepared, started or failed).

No passwords, keys, e-mail addresses or account identifiers are written. It is local material of yours.

### 🔁 Repeat a session
Open the agent console in the project folder and ask it to read the `inicio.md` of that folder. It receives exactly what it got the first time, with no clipboard and no need to remember what you typed. Handy to reproduce a failure and to compare how two agents answer the same task.

### 🔑 Accounts come from the CLI, not from MMCelt
MMCelt never asks for, stores or transforms credentials. It relies on the session you already have authenticated in `claude`, `codex` or `gemini`. It uses no paid API, picks no model and decides no billing.

### 🔗 MCP and console are two separate capabilities
The 'Send to...' menu marks every agent with an icon:

- ✨ **Console and MCP**: it can talk to you and report back to the map.
- 🖥 **Console only**: MMCelt found its executable, but MCP is not registered yet.
- 🔗 **MCP only**: it can report back, but MMCelt cannot open a console for it.
- 🔌 **Neither of the two**.

An agent listed as MCP-connected **does not mean** a console can be opened for it.

### 🌌 Official Console Agents (CLI)
MMCelt exclusively retains the three official terminal clients: **Claude Code**, **Codex CLI**, and **Gemini CLI** (the latter with an account notice). Graphical and non-official clients have been retired to ensure that only verified and straightforward options are provided.

### 💻 Where it is verified and where it is only implemented
- **Windows**: this is the platform with a verified route. It opens a fresh console window directly on the agent executable. To locate it, `PATHEXT` wins: if an npm install left an extensionless Unix script next to the `.cmd` launcher, the launcher is chosen, because it is the only one Windows can run.
- **Linux**: implemented, but **there is no verified real route yet**. It uses the first known terminal it finds among `x-terminal-emulator`, `gnome-terminal`, `konsole` and `xfce4-terminal`. If none exists, it says so and launches nothing.
- **macOS**: **not** claimed yet. The project has no tested route there and prefers not to promise one.

### 🔄 Automatic closed-loop reload
When an agent updates the map on disk through `mmcelt_sync_ai_progress`, MMCelt notices immediately and the canvas reloads on its own, showing the new nodes, states and priorities.

### 🛡️ Protection of unsaved local changes
If you are editing the map and have pending changes, automatic reload pauses so your work is never overwritten. It resumes as soon as you save with `Ctrl + S`.

### 🛑 How to stop watching, and how to stop the console
Watching ends cleanly when you use `Stop tracking` in the artificial intelligence menu, create a new map with `Ctrl + N`, open another file from the `File` menu, or close the application.

The agent console is a **separate process**: you close it in its own window. Stopping the watch does not close the console, and closing the console does not delete the session record.
