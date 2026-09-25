# 🧭 Project and instructions for AI

This is what turns your map into something an AI can truly understand.

### 🎯 Why is it needed?

A mind map on its own is just a collection of boxes and arrows. An AI can read them, but it has
no idea **why** they are there. It lacks what is in your head that you haven't written on any node:
what problem you are trying to solve, why you arranged the branches this way, and what you would
consider a successful outcome.

Without that, the AI fills in the gaps. And filling gaps with assumptions is precisely what makes
a response sound very plausible while being utterly useless.

### 📝 How to fill it in

1. Open **`🤖 Artificial Intelligence`** → **`🧭 Project and instructions for AI`** (also accessible from the side panel button or after editing in `✏️ Edit`).
2. Fill in the three fields. None are mandatory, but the more specific they are, the better:
   - **Vision:** what you are trying to create. In one or two sentences, in your own words.
   - **Goals:** what concrete results you seek. "Milestones and deliverables", not wishes.
   - **Context / Audience:** who will use it, or in what environment it must operate.

**A piece of advice:** write as if you were explaining it to someone who just joined the project today.
That is exactly what the AI is each time it opens your map: a new collaborator with zero memory
of previous conversations.

### 📦 What is sent to the AI, exactly

This is where misunderstandings usually happen. **It is not just these three fields that get sent.**
The manual export keeps an explanatory document with a preamble and ten structured sections:

1. **Preamble:** baseline operating instructions for the model.
2. **Your vision and goals:** what you enter in this window.
3. **Summary and metrics:** node count, branches radiating from the root, cross links, tags
   in use, and node distribution across states.
4. **The complete structure** of the map, including notes for every node, their tags, and file
   paths if you assigned them.
5. **Your corrections:** nodes where you demanded changes and those you approved.
6. **The cross-connection matrix** between distinct branches.
7. **Flagged questions and pending decisions** you marked on the map.
8. **A diagram** of the map, in Mermaid format.
9. **Suggested prompts:** ready-made prompts you can copy directly if desired.
10. **How to return work to you:** the contract governing the six MCP tools, the rule that no
    AI may declare your approval, and the exact values to use.

Sections 5, 6, and 7 only appear if you have content for them. The others are always included.
The three suggested prompts in section 9 are options for you: the agent does not receive all three
directives simultaneously.

In other words: the entire map travels, not a lossy summary. If you wrote a lengthy note on a node,
the AI is going to read it in full.

### 🧩 The four tabs of the window

To get started you only need **Project**. That is where the vision,
the goals, the target audience, and the author live.
The other tabs are optional and serve when you want finer control over a session:

- **Instructions** stores common rules for Claude, Codex, and Gemini. You can select the
  document language without altering the application language, restore a recommended template, or
  write your own custom rules. What you type is preserved verbatim: MMCelt never covertly translates it.
- **Task templates** provides three starting points. Clicking one copies only that template into the
  editable prompt; all three are never sent together. The prompt is preserved when closing the window
  and reappears when you select an agent.
- **Full preview** displays the exact result of the four blocks. The contract and context
  are protected; rules and prompt are the two blocks you control.

If `AGENTS.md`, `CLAUDE.md`, or `GEMINI.md` are present on disk, they appear as distinct sources. A
**New source** is never incorporated on its own. Click **Accept this version** only after reading it. If the
file changes later, it will appear as **Modified source; requires review**, will become unchecked,
and you will be able to compare the **Previously accepted version** with the **Current version**.
If none exist, the tab indicates the exact folder where you can create them manually.

### ✍️ You can review and refine the request before it leaves

This part often goes unnoticed, yet it is one of the most powerful features of the program.

When you use **`🤖 Artificial Intelligence`** → **`📤 Send to...`** and choose an agent, **nothing
is sent yet**. A dispatch preview window opens displaying the complete text across four distinct blocks:

| Block | Can it be edited? | What it is |
|---|---|---|
| **MMCelt contract (protected)** | No | Baseline rules, identical for all agents |
| **Common project rules** | **Yes** | Your way of working, applied to every session |
| **Mind map context (generated)** | No | Generated from the canvas; to alter it, edit the map |
| **Task for this session** | **Yes** | What you want to accomplish **now**, in this session |

If two instructions contradict each other, the document itself defines the hierarchy, from strongest
to weakest: **your corrections on the map**, the MMCelt contract, project rules, and finally, the task
for this session.

Below are four action buttons. `Reset this session` restores the prompt.
`Save as common rules` is the **only** action that permanently writes your
preferences to `.mmcelt/instrucciones-agente.md`.
`Cancel` discards everything without leaving any trace. And the final button launches the agent.

**Read through it at least once.** If the map, rules, or a source file change after opening the
confirmation dialog, MMCelt marks the preview as expired and requires you to review again. This guarantees
that `inicio.md` and the agent receive the exact same bytes you inspected on screen.

### 🤔 If you have never worked like this before

You don't have to fill in all three fields on day one. A map without a vision export still works.
However, the difference between an AI that merely repeats generic boilerplate and one that provides
genuine architectural value almost always lies right here, and not in which model you choose.

Start with the **Vision**, even if it is just two sentences. It is the single field that changes the outcome most.
