# ⌨️ Quick Creation and Keyboard Shortcuts

MMCelt is designed to let you create and edit branches at the speed of thought without taking your hands off the keyboard.

### ⚡ Standard Keyboard Shortcuts Table:
| Key | Immediate Action |
| :--- | :--- |
| **`Tab`** | Adds a **child node** under your currently selected node. |
| **`Enter`** | Adds a **sibling node** at the same hierarchical level. |
| **`Space` / `F2`** | Opens in-place text editor directly over the node. |
| **`Delete` / `Backspace`** | Safely deletes the selected node and all its child branches. |
| **`Double Click`** | Enters node text editing mode. |
| **`Ctrl + N`** | Starts a new map. A recovery backup is kept of the previous one. |
| **`Ctrl + S`** | Saves mind map to disk (`.mmcelt`). |
| **`Ctrl + E`** | Exports rich Markdown file for AI (`.md`). |
| **`Ctrl + F`** | Centers camera on project central idea. |
| **`Ctrl + Z`** | Undoes the last change. Consecutive changes are undone together. |
| **`Ctrl + Y`** | Redoes the last undone change. `Ctrl + Shift + Z` also works. |

### ✍️ While typing, shortcuts affecting the map remain silent
`Tab`, `Enter`, `Delete`, `Backspace`, `Space`, `F2` and `Ctrl + N` only act when the keyboard
is free. As soon as you type in any text field — node notes, project vision,
the box where you paste AI responses —, those keys behave as expected when
typing: `Backspace` deletes a letter, not the node.

`Ctrl + S`, `Ctrl + E`, `Ctrl + F`, `Ctrl + Z` and `Ctrl + Y` remain active while typing.
They do not alter the node structure and allow you to save or undo safely.

### 💡 Pro Tip:
Select the central node and press `Tab` repeatedly to quickly create 4 or 5 structural pillars.

### 🔍 Finding a node in a large map

There is a search box at the top of the right-hand panel. Type part of a **title** or a **tag**
and matching nodes appear below; clicking one selects it and moves the view to it **without
changing the zoom**, so you keep the level of detail you were working at.

Case and accents are ignored: `diseno` finds "Diseño". **Notes are not searched**: they are long
paragraphs, and any common word would return half the map. The list stops at 50 results; with
more matches than that, what you need is a narrower search.
