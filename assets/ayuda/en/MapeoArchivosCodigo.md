# 📁 Code File Mapping (file_path)

Connect the ideas and decisions of your mind map directly to the real files in your project on disk.

### 🛠️ How to link a file and what it is for:

A mind map helps you think big: you can have a node titled “User Authentication” and another called “Database”. However, your computer and software are made up of concrete files (such as `src/auth.rs` or `config/database.json`).

The file field (`file_path`) is the bridge linking the abstract concept with the real file containing its code or documentation. This turns the map from a mere diagram into a living index of your project.

**Steps to link a file:**
1. Click on any node on your map to select it.
2. Look at the right-hand panel (the Node Inspector).
3. In the **📁 File/Path:** field, type the path of the file relative to your project folder. For example:
   - `src/login.rs` for a code file.
   - `documentacion/requisitos.md` for explanatory text.
   - `frontend/components/` to indicate an entire folder.

### 💡 Practical tips and usage with AI:

**🤖 How does the artificial intelligence take advantage of this?**
When working with an assistant (such as Claude Code, Codex CLI, or Gemini CLI), or when exporting the project summary in Markdown:
- The AI reads exactly which file corresponds to each node.
- It knows in advance where to apply changes without blindly searching your entire repository.
- It helps maintain a clean, organized correspondence between conceptual architecture and source code.

**Practical tips:**
- **Use relative paths:** Always write paths relative to the root folder of your project (e.g., `src/main.rs` instead of `C:\MyDocuments\Project\src\main.rs`). This ensures all links remain intact if you move or share the project.
- **Folder paths:** If a node groups multiple files, you can write the path ending with a slash (such as `src/services/`) to indicate that it represents that entire directory.
