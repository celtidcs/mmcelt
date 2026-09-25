# 🟣 MCP Server & AI Agents

A secure and direct bridge for artificial intelligence models to read and create mind maps on your machine.

### ⚡ Easy one-click connection

You don't need to edit complex configuration files. You can connect your agents directly from the application:

1. Go to the menu `🤖 Artificial Intelligence` → `🔌 Connect MMCelt with my AIs...`.
2. MMCelt will automatically look for installed terminal assistants on your machine (Claude Code, Codex CLI, and Gemini CLI).
3. Click the **Connect** button next to the assistant you wish to use. Done!

If you install a new assistant later, simply reopen that window: detection runs each time you open it. The “Connected” state guarantees that the assistant knows where to find MMCelt and how to communicate with it using `--mcp-server`.

### ⚙️ Manual configuration and embedded server

To manually configure your MCP client, add this entry to its JSON configuration:
```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/path/to/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": { "MMCELT_WORKSPACE": "C:/path/to/your/project" }
    }
  }
}
```

The MCP server is **embedded inside the MMCelt executable itself**: it requires no external runtime or extra dependencies.

### 🛡️ Complete security: workspace folder and backups

Your peace of mind comes first. When an AI assistant collaborates with MMCelt, **it is strictly restricted to reading and writing inside your project folder**. This boundary is defined by the `MMCELT_WORKSPACE` environment variable.

Any attempt by the AI to access folders outside this workspace will be immediately rejected. And for additional safety:
- **Automatic backup copies:** Before an agent modifies an existing file, MMCelt saves a copy with the exact timestamp (for instance `project.mmcelt.20260918-120000.bak`). If you disagree with the changes, your prior work remains completely intact.
- **Respect for your ideas:** The AI can suggest new branches and links, but it will **never delete or move** nodes you created. Everything suggested by the AI is visually marked as “AI generated” so you always decide whether to approve or correct it.

### 🔨 The 6 official MCP server tools

The assistant has six official tools designed specifically to collaborate with you:

- `mmcelt_workspace_info`: Checks which folder is authorized and which `.mmcelt` maps exist in it. This is the first tool it checks so it never invents paths.
- `mmcelt_create_mindmap`: Creates new mind maps in `.mmcelt` format.
- `mmcelt_read_mindmap`: Reads the structure of your map, notes, and open questions.
- `mmcelt_sync_ai_progress`: Develops the map by adding nodes with their roles, priorities, and relationships.
- `mmcelt_get_human_feedback`: Reads your corrections and directives before modifying sensitive areas.
- `mmcelt_export_ai_markdown`: Converts the map into a structured summary in Markdown.

### 🌱 If you created the map yourself: total respect for your authorship

The agent can expand your map, but never replace it. It adds nodes with their roles and tags, and creates connections between separate branches.
Everything it adds is born marked as AI generated, cannot sign anything as approved by you, and will never reconnect a path you marked as discarded.

### 🏷️ Nodes with the same name

In a decision map it is natural to repeat common words (such as “Yes”, “No”, or “Pending”) across different branches. If the AI requests modifying a node by specifying only an ambiguous name, MMCelt rejects the call and returns the unique identifiers of each match so the AI can clarify exactly which node it intended to modify. This avoids unwanted confusion.

### 🧭 What if you don't have assistants installed in your terminal?

Do not worry: MMCelt is fully useful even without any agent installed in your terminal. You can work with web services such as ChatGPT, Claude.ai, or Gemini in your browser using the menu options `🤖 Artificial Intelligence` → `📋 Copy Master Prompt for AI...` or `💾 Export .md File for AI...`. Then simply paste the response into `🤖 Artificial Intelligence` → `📥 Import from AI (ChatGPT, Claude, Gemini)...` to transform the text into visual nodes instantly.
