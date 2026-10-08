# ⌨️ Quick Creation and Keyboard Shortcuts

MMCelt is designed to let you create and edit branches at the speed of thought without taking your hands off the keyboard.

### ⚡ Standard Keyboard Shortcuts Table:
| Key | Immediate Action |
| :--- | :--- |
| **`Tab` / `Insert`** | Adds a **child node** under your currently selected node. |
| **`Enter`** | Adds a **sibling node** at the same hierarchical level. |
| **`Space` / `F2`** | Opens in-place text editor directly over the node. |
| **`Delete` / `Backspace`** | Safely deletes the selected node and all its child branches. |
| **`Double Click`** | Enters node text editing mode. |
| **`Ctrl + N`** | Starts a new map. A recovery backup is kept of the previous one. |
| **`Ctrl + S`** | Saves mind map to disk (`.mmcelt`). |
| **`Ctrl + E`** | Exports rich Markdown file for AI (`.md`). |
| **`Home`** | Centers camera on project central idea. |
| **`Ctrl + F`** | Moves the cursor to the side panel search box, with its text selected so you can search for something else. |
| **`Ctrl + Z`** | Undoes the last change. Consecutive changes are undone together. |
| **`Ctrl + Y`** | Redoes the last undone change. `Ctrl + Shift + Z` also works. |

### ✍️ While typing, shortcuts affecting the map remain silent
`Tab`, `Insert`, `Enter`, `Delete`, `Backspace`, `Space`, `F2`, `Home` and `Ctrl + N` only act when the keyboard
is free. As soon as you type in any text field — node notes, project vision,
the box where you paste AI responses —, those keys behave as expected when
typing: `Backspace` deletes a letter, not the node, and `Home` moves the cursor to the start
of the line without moving the map.

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

### ✏️ Right-click on a node: "Node actions"
A right-click on a card selects it and opens the **Node actions** menu at that point, with the same options as the `✏️ Edit` menu and the inspector: add child (`Tab` / `Insert`), add sibling (`Enter`, not available on the root), create a cross-connection, edit the title (`Space` / `F2`), delete (`Delete`), and the status, priority, human control and role submenus, with the current value marked.

Dragging with the right button still moves the view and does not open the menu. `Esc` or a click outside closes it without changing anything. It does not open while the map is read-only.
