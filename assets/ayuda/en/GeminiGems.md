# 🔵 Integration with Gemini CLI

Gemini is Google's artificial intelligence model family, and **Gemini CLI** is its official command-line tool. MMCelt connects directly with Gemini CLI to analyze, expand, and enrich your mind maps.

### 💻 Using Gemini CLI in the terminal

1. Go to `🤖 Artificial Intelligence` → `🔌 Connect MMCelt with my AIs...` and connect Gemini CLI.
2. Open your map and choose `🤖 Artificial Intelligence` → `📤 Send to...` → `Gemini CLI`.
3. Review the preview of the document and the task that the AI will receive.
4. Click **Start in console**. MMCelt will automatically open the terminal in the prepared environment and start watching for changes to incorporate the nodes Gemini creates.

### 💡 Tips and Tricks: Free API key and compatibility

If launching Gemini CLI prompts for authentication or you experience issues with browser login, the fastest, most direct and reliable method is to use a **free API key** from Google AI Studio:

1. Sign in to Google AI Studio (`https://aistudio.google.com/app/apikey`) with your Google account and click **Create API Key**. It is completely free.
2. Copy the generated key.
3. Set the environment variable in your system before launching the console or in your user profile:
   - **On Windows (PowerShell):**
     ```powershell
     $env:GEMINI_API_KEY="your_key_here"
     ```
   - **On Linux or macOS (Bash/Zsh):**
     ```bash
     export GEMINI_API_KEY="your_key_here"
     ```
4. With this variable set, Gemini CLI works immediately without requesting browser sign-ins.
5. You can check the official Gemini CLI documentation on its repository: `https://github.com/google-gemini/gemini-cli`.

**ℹ️ Current account compatibility status in Gemini CLI:**
Confirmed: since June 18, 2026, Google removed interactive «Sign in with Google» for personal accounts in Gemini CLI, including free, Google AI Pro, and Ultra accounts.
- **September 13, 2026:** first real test, with the literal message: *«This client is no longer supported for Gemini Code Assist for individuals. To continue using Gemini, please migrate to the Antigravity suite of products»*.
- **Repeated afterward with several older versions of Gemini CLI**, to rule out that it was an issue with the installed version: the result was the same with all of them.
- **Confirmed with a free Google AI Studio API key**: the connection worked end-to-end, creating verified nodes and links on disk, with no personal-account restriction.

The API key approach described above **is the only confirmed path** for personal accounts: interactive sign-in is no longer available for them.

### 🌐 Gemini in the web browser

If you prefer not to use the terminal:
1. Export your map with `📁 File` → `🤖 Export Markdown for AI (.md)` (or click `🤖 Artificial Intelligence` → `👁️ Preview AI Markdown (.md)...`).
2. Open Gemini in your browser, paste the text, and submit your prompt.
3. Copy Gemini's generated response and return to MMCelt: click `🤖 Artificial Intelligence` → `📥 Import from AI (ChatGPT, Claude, Gemini)...` to incorporate the new branches into your map.
