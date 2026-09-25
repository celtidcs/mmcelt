# 🤖 The Full AI Workflow

Other topics explain each piece in isolation. This topic covers **the entire journey**: what leaves your
computer, why, what decisions you make, and what the AI can do when it responds.

If you only read one topic about AI, read this one.

### 🗺️ The journey in four movements

**1. You prepare the map.** Nothing you do here is sent yet. You build the branches,
write notes, mark states and questions, trace cross connections, and describe the project vision.
The more of what is in your head is written down, the less the AI will have to guess.

**2. The application composes a document.** It does not send your `.mmcelt` file. For an automated
session it bundles four blocks, always in the same order: MMCelt contract, project rules, context of the
map, and a single assignment. The full map travels inside the context, not as a summary.

**3. You review and approve it.** This is the step that almost no one uses and the one that provides the most
protection. Before anything leaves, you see the complete text and can edit two of its four blocks.

**4. The AI returns work to the map.** Not by directly editing your file, but through six
constrained tools with rules. Whatever it adds is born marked as AI-generated, and you decide whether to approve it.

### 📤 The two ways it leaves

| | How | When to use |
|---|---|---|
| **Manual** | `📁 File` → `🤖 Export Markdown for AI (.md)`, and upload the file to a chat | Any model, including browser-based ones. No installation needed |
| **Automated** | `🤖 Artificial Intelligence` → `📤 Send to...` and choose your agent | You have the agent installed and want the map to update itself |

The response returns via `📥 Import from AI (ChatGPT, Claude, Gemini)...` if you did it manually,
or automatically if you used automated dispatch.

### ✍️ What you decide, and where

- **Your overall intent:** `🤖 Artificial Intelligence` → `🧭 Project and instructions for AI` → **Project** tab.
- **What is accepted and what is not:** mark nodes as `🛡️ Human Approved` or
  `⚠️ Requires Correction`, and send them with
  `🤖 Artificial Intelligence` → `🛑 Send Corrections and Directives to the AI...`.
- **Preview before sending:** with `🤖 Artificial Intelligence` → `👁️ Preview AI Markdown (.md)...`
  you can read on screen the exact generated text before dispatching to a console or copying to a chat.
- **What is requested this time:** in the dispatch preview, the “This session's prompt” block.
- **How you want it to always work:** in that same preview, the common rules block.

Before dispatch you can also prepare everything from **`🧭 Project and instructions for AI`**.
The **Instructions** tab lets you select the **AI document language**, save common rules,
and inspect native prompt sources. The **Prompt templates** tab copies a single recipe to the prompt,
and **Full view** displays the exact prompt text.

A native prompt file is never included simply because it exists on disk. First it appears as **New source**;
after clicking **Accept this version** it remains preselected as long as its bytes do not change. If it changes,
it unchecks itself and displays the previous version alongside the current one. You can decide whether to
include or exclude a source from the final session using its checkbox.

If two instructions contradict each other, the document itself establishes the chain of command, from strongest
to weakest: **your corrections on the map**, the MMCelt contract, the project rules, and, in the last place,
the prompt of this session.

### 🔙 What the AI can and cannot do upon return

**It can** read the map, read your corrections, add nodes, report progress, and create a new
map. There are six tools and no more.

**It cannot** edit your `.mmcelt` file on its own, leave the workspace folder you authorized,
or **claim that you have approved something**. Everything it contributes is born marked as
AI-generated, and promoting that to approved is your decision, always.

Furthermore, an audit trail is preserved: the exact text you approved is saved in `.mmcelt/sesiones`,
and if an automated change arrives on a map you modified, a `.bak` backup copy is saved first.

Confirmation is not a decorative snapshot. Right before starting, MMCelt rechecks the map,
rules, and source files. If any of them changed, it shows that the confirmation has expired,
does not launch the agent, and asks you to review again.

### ⚠️ What this is not

The document tells the AI that your map content consists of **data, not instructions**.
That reduces misunderstandings, but **it is not a security sandbox**: a title or note imported
from outside could be phrased as if it were an instruction.

What truly protects you is something else: the authorized workspace folder, the fact that
tools are limited to six, and that you review the preview before anything leaves.
That is why it is worth reading through at least once.

**💡 What to do if you have no console agents installed?**
If upon entering `🤖 Artificial Intelligence` → `📤 Send to...` you see the message "No installed agents detected",
do not worry: **you do not need technical consoles or advanced agents to use AI with MMCelt**.

You can work comfortably from your regular web browser:
1. Use `📁 File` → `🤖 Export Markdown for AI (.md)` (or inspect the text first with `🤖 Artificial Intelligence` → `👁️ Preview AI Markdown (.md)...`).
2. Open any AI chat service in your browser (ChatGPT, Claude, Gemini...).
3. Paste the exported text and give whatever instructions you desire to continue the map.
4. When the AI replies with a block in JSON or Markdown format, copy it.
5. In MMCelt, click `🤖 Artificial Intelligence` → `📥 Import from AI (ChatGPT, Claude, Gemini)...` and paste the response. Your new branches will appear on the map immediately.

If in the future you install an agent in your system terminal (such as Claude Code, Codex CLI, or Gemini CLI),
MMCelt will recognize it as soon as it is in your PATH.

### 📚 Where to go next, depending on your goal

- **Clearly articulating your intent:** the creator vision topic.
- **Steering an AI that drifts:** the human control and corrections topic.
- **Understanding the generated document:** the Markdown export topic.
- **Manually bringing back a pasted response:** the AI import topic.
- **Sending and watching it return automatically:** the dispatch and file watcher topic.
- **Connecting your agent via MCP:** the 🟣 MCP Server & AI Agents topic.

All are available in this same selector, in the order they are typically needed.
