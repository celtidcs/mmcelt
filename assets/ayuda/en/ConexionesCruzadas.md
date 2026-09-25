# 🔗 Cross Links and Dependencies

They say something the branch structure **cannot express**: that two parts of the project, hanging
from different places, depend on each other.

### 🌳 Why they are needed

A mind map is a tree: every idea hangs from another one. That works until you find that "payment
system" needs "user registration", and the two sit on branches that never meet.

You could move them so they end up together, but then the map stops reflecting how you think and
starts reflecting a limitation of the tool. Cross links avoid that: **you leave each idea where it
makes sense and draw the relationship separately.**

### 🏷️ The five types, and when to use each

The type you pick **changes what the AI understands**, so it is worth not choosing at random.

- **➡️ Dependency (requires):** A cannot work without B. The most common one.
  *Example: "Send invoice" requires "Customer tax details".*
- **⛔ Blocks:** B cannot move forward until A is resolved. Stronger than a dependency: it
  describes something **stopped right now**.
  *Example: "Choose the database" blocks "Design the data model".*
- **🔀 Alternative to:** two **mutually exclusive** paths; picking one discards the other.
  *Example: "Desktop application" is an alternative to "Web application".*
- **✨ Synergy with:** they do not need each other, but together they are worth more than apart.
  *Example: "Tagging system" and "Search".*
- **💡 Inspired by:** a reference. You are copying an idea or a pattern from elsewhere in the map.
  *Example: "Admin panel" inspired by "User panel".*

### 🛠️ How to create one

1. Select the source node: the one that **requires**, **blocks** or **draws inspiration**.
2. Press **`🔗 Cross Link`** in the inspector on the right.
   It is also in **`✏️ Edit`** → **`🔗 Create Cross Link / Dependency...`**.
3. Pick the target node, the type, and write **why** that relationship exists.

**Order matters.** "A requires B" and "B requires A" are different things, and the AI reads it
literally. If you get it wrong, delete the link and create it the other way round.

**And the reason matters more than it seems.** A link with no explanation tells the AI there is a
relationship, but not what to do with it. With the reason written down, it can reason about it.

### ✋ When NOT to use them

This matters just as much, because a map full of crossing arrows cannot be read:

- **If the relationship is "one thing is part of the other"**, that is not a cross link: it is a
  child. Hang it where it belongs.
- **If everything depends on everything**, do not draw thirty links. It usually means a node is
  missing that would gather that shared idea.
- **If the relationship is obvious** — two tasks in the same block that clearly follow
  one another in direct sequence —,
  you gain nothing by drawing it.

A practical rule: **draw the link if it took you some thought to notice it existed.** Those are
the ones the AI cannot work out on its own, and the ones that get forgotten in meetings.

### 👀 Where you see them afterwards

Links leaving or arriving at a node appear in the inspector on the right, under
**`🔗 Cross Links:`**, and can be deleted from there. That section **only appears if the node has
any**, so if you cannot see it, that node has none.

On the canvas they are drawn as curves, styled differently per type. And in the document sent to
the AI they travel in a section of their own, the dependency matrix.
