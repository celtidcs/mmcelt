# 📤 « Envoyer à... », console de l'agent et surveillance

« Envoyer à... » prépare une session supervisée avec un agent de programmation et referme la boucle. Si la destination dispose d'une console officielle, la conversation s'y déroule ; si elle ne parle que MCP, MMCelt laisse tout préparé et l'agent le récupère depuis sa propre interface. Dans les deux cas, le travail revient à la carte par le serveur MCP de MMCelt.

### 👁️ Choisir un agent n'envoie encore rien
Cliquer sur un agent ouvre un **aperçu** du prompt complet, réparti en quatre blocs :

1. **Contrat MMCelt** (lecture seule) : les règles minimales d'intégrité. Découvrir l'espace autorisé, lire les décisions humaines avant d'écrire, ne jamais modifier le fichier `.mmcelt` à la main et renvoyer l'avancement par MCP. Il est affiché protégé pour ne pas être effacé par mégarde.
2. **Règles communes du projet** (modifiable) : votre profil neutre, le même pour tous les agents.
3. **Contexte de la carte** (lecture seule) : produit par l'exportation pour l'IA. Il se change dans la carte, qui en est la source.
4. **Tâche de cette session** (modifiable) : ce que vous voulez obtenir maintenant. Elle n'est pas conservée d'une session à l'autre.

`Restaurer cette session` remet les règles et la tâche dans leur état initial. `Enregistrer comme règles communes` est une action distincte : elle seule écrit le profil du projet dans `.mmcelt/instrucciones-agente.md`. `Annuler` n'enregistre rien, n'exporte rien, ne surveille rien et ne laisse aucun dossier de session.

### ⚖️ Précédence déclarée dans le prompt lui-même
Corrections humaines de la carte **>** contrat MMCelt **>** règles du projet **>** tâche de la session.

### 📚 Les instructions natives sont montrées, pas collées
Si la racine du projet contient `AGENTS.md`, `CLAUDE.md` ou `GEMINI.md`, l'aperçu les énumère et permet de les lire, mais **ne les incorpore pas** au prompt commun : chaque outil découvre le sien avec sa propre portée, et tout copier créerait des règles dupliquées et contradictoires.

### 🚀 Ce qui se passe à la confirmation
Le bouton annonce ce qui va réellement arriver : `Démarrer dans la console` quand MMCelt a localisé la console de l'agent, et `Préparer pour MCP` quand il n'y en a pas. Dans ce second cas, l'aperçu le signale à l'avance, par un avertissement mis en évidence au-dessus du nom de l'agent.

**Avec console**, l'ordre est fixe et chaque étape doit réussir avant la suivante :

1. **Enregistre la carte** dans son fichier `.mmcelt`.
2. **Exporte le document pour l'IA** à côté de la carte, avec le suffixe `_AI.md`.
3. **Écrit le dossier de session**.
4. **Ouvre la console** de l'agent dans le dossier du projet.
5. **Active la surveillance** de la carte.

Si la console ne s'ouvre pas, MMCelt **ne dit pas « envoyé »**, ne commence pas à surveiller et marque le dossier de session comme échoué.

**Sans console** —un agent dont le serveur MCP est enregistré mais dont l'exécutable n'est pas localisé par MMCelt— tout le reste a bien lieu : la carte est enregistrée, le `_AI.md` exporté, le dossier de session écrit et la surveillance activée. Seule la quatrième étape est omise, et le dossier reste à l'état **préparé** au lieu de démarré, parce que MMCelt n'a rien lancé. L'agent récupère le travail depuis sa propre interface et le renvoie par MCP comme n'importe quel autre.

### 🗄️ Un projet d'une version antérieure n'est pas modifié quand on le regarde
Un dossier utilisé avec une version antérieure ne possède pas encore `.mmcelt/configuracion.json`. Ouvrir l'aperçu **ne le crée pas** : le prompt est composé en mémoire et rien n'est écrit sur le disque. L'identité du projet s'écrit à la confirmation, qui est le geste explicite. Si vous annulez, le dossier reste exactement dans son état initial.

### 🗂️ Le dossier de session : `.mmcelt/sesiones`
Chaque confirmation crée un dossier nommé d'après la date, l'agent et un identifiant. Il contient deux fichiers :

- `inicio.md` : le prompt exact que vous avez approuvé, caractère par caractère.
- `sesion.json` : version du format, identifiant, date UTC, agent, chemin de la carte, sources détectées et incorporées, taille du prompt et état (préparé, démarré ou échoué).

Aucun mot de passe, aucune clé, aucune adresse électronique et aucun identifiant de compte n'y sont écrits. C'est votre matériel local.

### 🔁 Rejouer une session
Ouvrez la console de l'agent dans le dossier du projet et demandez-lui de lire le `inicio.md` de ce dossier. Il reçoit exactement ce qu'il avait reçu la première fois, sans presse-papiers et sans devoir vous souvenir de ce que vous aviez écrit. Utile pour reproduire une panne et pour comparer deux agents sur la même tâche.

### 🔑 Les comptes viennent du CLI, pas de MMCelt
MMCelt ne demande, ne conserve et ne transforme aucun identifiant. Il s'appuie sur la session déjà authentifiée dans `claude`, `codex` ou `gemini`. Il n'utilise aucune API payante, ne choisit pas de modèle et ne décide d'aucune facturation. L'authentification reste entièrement du côté de l'outil officiel.

### 🔗 MCP et console sont deux capacités distinctes
Le menu « Envoyer à... » marque chaque agent d'une icône :

- ✨ **Console et MCP** : il peut dialoguer et renvoyer le travail à la carte.
- 🖥 **Console seule** : MMCelt trouve son exécutable, mais MCP n'est pas enregistré.
- 🔗 **MCP seul** : il peut renvoyer le travail, mais MMCelt ne peut pas lui ouvrir de console.
- 🔌 **Aucune des deux**.

Qu'un agent apparaisse connecté par MCP **ne veut pas dire** qu'on puisse lui ouvrir une console.

### 🌌 Agents officiels en console (CLI)
MMCelt conserve exclusivement les trois clients officiels de terminal : **Claude Code**, **Codex CLI** et **Gemini CLI** (ce dernier avec un avertissement sur le compte). Les clients graphiques ont été retirés afin de ne proposer que des options vérifiées et simples d'utilisation.

### 💻 Où c'est vérifié et où ce n'est qu'implémenté
- **Windows** : c'est la plateforme au parcours vérifié. MMCelt ouvre une nouvelle fenêtre de console directement sur l'exécutable de l'agent. Pour le localiser, `PATHEXT` prime : si une installation npm a laissé un script Unix sans extension à côté du lanceur `.cmd`, c'est le lanceur qui est retenu, seul exécutable pour Windows.
- **Linux** : implémenté, mais **sans parcours réel vérifié à ce jour**. Il utilise le premier terminal connu qu'il trouve parmi `x-terminal-emulator`, `gnome-terminal`, `konsole` et `xfce4-terminal`. S'il n'y en a aucun, il le signale et ne lance rien.
- **macOS** : ce n'est **pas** encore affirmé. Le projet n'y dispose d'aucun parcours vérifié et préfère ne rien promettre.

### 🔄 Rechargement automatique en boucle fermée
Quand un agent met à jour la carte sur le disque via `mmcelt_sync_ai_progress`, MMCelt détecte le changement aussitôt et le canevas se recharge tout seul, avec les nouveaux nœuds, états et priorités.

### 🛡️ Protection des modifications locales non enregistrées
Si vous modifiez la carte et avez des changements en attente, le rechargement automatique se met en pause pour ne pas écraser votre travail. Il reprend dès que vous enregistrez avec `Ctrl + S`.

### 🛑 Comment arrêter la surveillance et la console
La surveillance s'arrête proprement avec `Arrêter le suivi` dans le menu d'intelligence artificielle, en créant une carte avec `Ctrl + N`, en ouvrant un autre fichier depuis le menu `Fichier` ou en fermant l'application.

La console de l'agent est un **processus distinct** : elle se ferme dans sa propre fenêtre. Arrêter la surveillance ne ferme pas la console, et fermer la console n'efface pas le dossier de session.
