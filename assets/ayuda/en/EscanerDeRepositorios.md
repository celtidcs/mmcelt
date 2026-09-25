# 🔍 Automatic Codebase Scanner

If you already have a programming project on your computer or a folder with source code, you do not need to create your mind map node by node from scratch. The MMCelt **code scanner** reads your project structure and automatically generates an interactive visual map in seconds.

### 🛠️ How to use the scanner and get the most out of it:

**🧠 Why scan your code?**
By converting a folder of files into a mind map you achieve:
- **A bird's-eye view of your architecture:** Quickly understand how your modules, libraries, and components are organized without getting lost in dozens of subfolders.
- **Direct links to your files:** Each box on the map is associated with its real path on disk (`file_path`), allowing both you and the AIs to know exactly which physical file corresponds to each concept.
- **The ideal starting point for working with AI:** You can ask any artificial intelligence model to analyze the resulting structure, detect duplicated code, propose refactorings, or identify circular dependencies.

**Steps to use the scanner:**
1. In the top bar, open the menu **`📁 File`** → **`🔍 Scan Code Folder...`**.
2. A system dialog will appear asking you to choose the root folder of your repository or software project.
3. MMCelt will explore the folder tree with complete safety:
   - **Filters technical noise:** Automatically discards heavy build folders or dependencies that add no conceptual value (such as `node_modules/`, `target/`, `dist/`, `.git/`, Python virtual environments, etc.).
   - **Creates the hierarchy:** Places the main folder in the center and branches out the key modules, packages, and code files.
4. Once the map is generated, you can rearrange nodes to your liking, change colors, add explanatory notes, or mark areas pending review.

**🤖 Next step with AI:**
Once your code is scanned:
- Use `🤖 Artificial Intelligence` → `👁️ Preview AI Markdown (.md)...` to see the architectural summary.
- Send the project to your preferred console agent (`📤 Send to...`) or export it to query a web chat. The AI will know exactly where each file lives!
