# 🟢 Integration with Codex CLI & ChatGPT

Whether you use the command line or prefer a web browser, you can leverage OpenAI's artificial intelligence with your mind maps in MMCelt.

### 💻 Codex CLI (in the terminal)

Codex CLI is a terminal agent running directly on your computer. It is the most streamlined and automated way to work:

1. Go to the menu `🤖 Artificial Intelligence` → `📤 Send to...` → `Codex CLI`.
2. You will see a full preview with the contents of your map and the task for the AI. Review it carefully.
3. When you click **Start in console**, MMCelt will automatically open the terminal with the environment prepared and start watching for changes to incorporate the nodes Codex creates.
4. Codex CLI reads the map and instructions through the MCP server. As it analyzes your project, it can create branches, suggest ideas, and update the map live.

> 💡 **Note:** To allow Codex CLI to communicate with MMCelt, remember to connect it beforehand via `🤖 Artificial Intelligence` → `🔌 Connect MMCelt with my AIs...` and restart your terminal if it was already open.

### 🌐 ChatGPT on the web or a Custom GPT

If you do not use the technical console and prefer chatting in a web browser (with free ChatGPT, Plus, or your own Custom GPT):

1. Export your map with `📁 File` → `🤖 Export Markdown for AI (.md)` (or inspect the text on screen using `🤖 Artificial Intelligence` → `👁️ Preview AI Markdown (.md)...`).
2. Open your browser, navigate to ChatGPT, and attach the `.md` file or paste the text directly into the conversation.
3. Ask it to expand the map, analyze risks, explore alternatives, or propose new tasks.
4. When ChatGPT replies (typically returning a structured block in JSON or Markdown), copy it.
5. Return to MMCelt and select `🤖 Artificial Intelligence` → `📥 Import from AI (ChatGPT, Claude, Gemini)...`. Paste the text and click import: the new branches will merge into your map while keeping all your previous work intact.

### 🔒 Key differences and data privacy

It is important to understand how each environment interacts with your data:
- **Codex CLI (local with MCP):** Runs on your own machine and directly accesses the designated project folder through the MCP protocol.
- **ChatGPT in the web browser:** Runs on OpenAI's servers and has no direct access to your local files or the MCP server. Interaction is completely manual and secure through exporting and importing Markdown or JSON text.
