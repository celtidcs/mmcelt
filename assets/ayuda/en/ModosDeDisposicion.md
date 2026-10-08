# 📐 Spatial Layout Modes

To keep your mind map clear, clean, and legible at all times, MMCelt features algorithms that compute the position of each node and prevent branches from overlapping.

You can choose how to organize your map at any time based on your visual preferences:

### 🎨 Available Modes:

1. **Balanced Tree (Left/Right):**
   - Places the central idea at the canvas origin.
   - Distributes main branches evenly to the left and right, balancing visual weight equally.
   - Automatically calculates the height of each sub-branch so texts never collide. It is the default mode and the most convenient for most projects.

2. **Radial / Circular:**
   - Places the central idea at the midpoint and expands branches in a 360-degree circular fan around it.
   - Ideal for brainstorming sessions or maps with many primary themes around a single concept.

3. **Free Manual Positioning:**
   - Allows you to move any node freely by dragging it with the left mouse button to the exact desired location.
   - In this mode the engine does not constrain node positions, giving you full manual control to diagram custom layouts.

### ⚡ How to change modes and reorganize

- In the top bar, open the **`🎨 View and Design`** menu to select your preferred layout mode.
- If at any point you wish to recalculate positions and tidy up the map, click **`🎨 View and Design`** → **`🔄 Rearrange the nodes`** (or use the corresponding shortcut). All nodes will realign harmoniously.

### 🔀 Dropping a node onto another
If you drag a node and drop it with its centre over another card, a small menu appears at that point so you can decide what you meant:

- **`➕ Make child`**: the node, with all its branches, now hangs from the node underneath. It is disabled when that is not possible: the node underneath is already its parent, the one you drag is the root, or the node underneath is inside its own branches (it would create a loop).
- **`↔ Make sibling`**: the node now hangs from the same parent as the node underneath, right after it. Not available when the node underneath is the root or when they are already siblings.
- **`🔗 Connect with a link`**: creates a cross-connection to the node underneath and puts the dragged node back where it was. Not available when they are already parent and child or already connected: it would be a second line on top of the one that links them.
- **`➡ Move here without covering`**: places it next to the node underneath, in the nearest free spot, without covering any card.
- **`↩ Cancel`**: puts it back where it was. `Esc` or a click outside the menu do the same.

While the menu is open, the keys that change the map do nothing. Any of the options can be undone with `Ctrl + Z`.
