# 🛑 User Control & Correction Directives (Human-in-the-Loop)

Turns MMCelt into your **oversight and veto console** over AI proposals.

### 🛡️ The Correction Workflow:
1. The AI presents a mind map or an execution plan.
2. If you spot an architectural error or an unwanted library:
   - Change the node status to **`⛔ Dismissed`** or **`⚠️ Requires Correction`**.
   - In the **"🛑 Correction / Required AI Instruction"** field, write your explicit instruction (e.g. *"Do not use external libraries; implement using Rust standard library"*).
3. Open menu **`🤖 Artificial Intelligence` > `🛑 Send Corrections and Directives to the AI...`**.
4. Click **`📋 Copy Correction Directives`** and paste into your AI chat (or let Claude Code call `mmcelt_get_human_feedback`).
5. The AI reads mandatory veto directives and realigns code immediately.

### ✅ Approving a node, and what happens next
When you validate a node, mark it as **`🛡️ Human Approved`** in the inspector. That is your signature on that content. Human review, priority, and status controls all show homogeneous icons in the menu and directly on the canvas node.

**That signature expires automatically.** If later the AI modifies that node — its notes, status, priority, or file path —, the node automatically reverts to **`⏳ Pending Human Review`**.

The reason is simple: you approved specific text, not the node for all eternity. Without this expiration, you would see your own approval on changes you never read, which defeats the entire purpose of this tool.

Setting the exact same value does not count as a change; routine AI sync passes that modify nothing will **not** force you to re-review previously approved work.

Text written in the correction field is **never cleared automatically**, even after the AI complies: you can always verify whether it faithfully implemented your directive.
