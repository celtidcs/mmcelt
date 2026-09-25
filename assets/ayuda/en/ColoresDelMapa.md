# 🎨 What the Colors Mean

Colors are **functional, not decorative**: they show which branch every element belongs to.

### 🌈 Branch lines connecting nodes
Each primary branch — direct child of central idea — receives a unique color, and **all descendants inherit it**:

- 1st branch: Blue
- 2nd branch: Emerald
- 3rd branch: Amber
- 4th branch: Purple
- 5th branch: Pink
- 6th branch: Turquoise
- 7th branch: Orange
- 8th branch: Indigo

This allows you to **trace any conceptual thread to its origin**, even in complex maps with crisscrossing branches.

With more than eight branches, the palette cycles back: the 9th branch returns to Blue.

### 👁️ High Contrast theme uses different colors
If **👁️ High Contrast** is active, that list differs from what you see: the first branch is not blue, it is **cyan** — pure saturated cyan on black background with white borders —, and remaining branches also use saturated tones.

This is intentional. Standard blue is a medium tone that **fails WCAG accessibility contrast minimums** on pure black backgrounds: users selecting this theme need maximum readability, so palette choices prioritize measured contrast over visual harmony. Pure cyan provides optimal contrast against black.

The rule **never changes**: each branch maintains its color and descendants inherit it. Only the palette assignments adapt.

### 🟠 Cross links are distinct
They are always styled in **amber**, with a dedicated tone independent of branch colors. This instantly distinguishes hierarchical tree links from cross-branch dependencies.

All cross links draw identically; what distinguishes them is **the text label** on their midpoint: custom label entered on creation, or link type name if left blank.

In exported Markdown Mermaid diagrams, line styles vary so models discern link types at a glance.

### 🌍 Application Language
Menu **`🎨 View and Design` → `Idioma / Language`**. Spanish, English, French, German, Russian, and Simplified Chinese are available, and the choice is remembered when you close the program.

The entire program speaks all six: menus, notifications, inspector, windows, dialogs, this very help system, and the document delivered to AI.

What **never** changes language is what goes inside your files: field names in `.mmcelt`, what is sent to AI, and what is stored in agent configuration. They are interchange formats, not text to read, and translating them would break already saved maps.

### 🌗 Three Visual Themes
Menu **`🎨 View and Design` → `Visual Theme:`**. All three share identical semantics — blue remains the first branch across all themes — and alter background reading contrast:

- **🌙 Dark Theme** (default): slate blue. Canvas, panels, and nodes use three distinct stepped tones, distinguishing layers without harsh borders. Easiest on eyes in low light.
- **☀️ Light Theme**: warm paper. Background is ivory rather than pure white: with white canvas, white node cards lose separation, and harsh white increases fatigue. Text is ink gray rather than harsh black.
- **👁️ High Contrast**: black background, solid white borders, and saturated colors. Designed for low-vision users or high-glare environments.

**Selected theme is saved on restart**, alongside interface scale factor.

Theme colors are verified by automated tests against WCAG contrast requirements: all text maintains compliant readability against its background.
