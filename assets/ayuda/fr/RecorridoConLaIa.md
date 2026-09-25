# 🤖 Le parcours complet avec l'IA

Les autres thèmes expliquent chaque élément séparément. Celui-ci raconte **le voyage dans son ensemble** : ce qui sort de votre
ordinateur, pourquoi, ce que vous décidez et ce que l'IA peut faire lorsqu'elle répond.

Si vous ne devez lire qu'un seul sujet sur l'IA, lisez celui-ci.

### 🗺️ Le parcours en quatre temps

**1. Vous préparez la carte.** Rien de ce que vous faites ici n'est encore envoyé. Vous construisez les branches,
rédigez des notes, indiquez les statuts et les doutes, tracez des connexions croisées et décrivez la vision du
projet. Plus ce que vous avez en tête est consigné par écrit, moins l'IA aura à supposer.

**2. Le logiciel compose un document.** Il n'envoie pas votre fichier `.mmcelt`. Pour une session automatique,
il rassemble quatre blocs, toujours dans le même ordre : contrat MMCelt, règles du projet, contexte de la
carte et une seule mission. La carte entière voyage à l'intérieur du contexte, pas sous forme de résumé.

**3. Vous le lisez et vous l'approuvez.** C'est l'étape que presque personne n'utilise et qui protège le plus.
Avant que quoi que ce soit ne parte, vous voyez le texte complet et pouvez modifier deux de ses quatre blocs.

**4. L'IA réinjecte le travail dans la carte.** Non pas en modifiant votre fichier, mais via six outils
encadrés par des règles. Tout ce qu'elle ajoute naît marqué comme généré par l'IA, et vous décidez si vous l'approuvez.

### 📤 Les deux manières d'exporter

| | Comment | Quand l'utiliser |
|---|---|---|
| **Manuelle** | `📁 Fichier` → `🤖 Exporter le Markdown pour l'IA (.md)`, puis téléversez le fichier dans le chat | Tout modèle, y compris sur navigateur. Aucune installation requise |
| **Automatique** | `🤖 Intelligence artificielle` → `📤 Envoyer à...` et choisissez votre agent | L'agent est installé et vous souhaitez que la carte se mette à jour seule |

La réponse revient par `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...` si vous avez procédé manuellement,
ou de manière transparente si vous avez utilisé l'envoi automatique.

### ✍️ Ce que vous décidez, et où

- **Votre intention générale :** `🤖 Intelligence artificielle` → `🧭 Projet et instructions pour l’IA` → onglet **Projet**.
- **Ce qui est valide et ce qui ne l'est pas :** marquez les nœuds comme `🛡️ Approuvé par l'humain` ou
  `⚠️ Correction requise`, puis transmettez-les avec
  `🤖 Intelligence artificielle` → `🛑 Envoyer corrections et directives à l'IA...`.
- **Vérifier avant d'envoyer :** avec `🤖 Intelligence artificielle` → `👁️ Aperçu du Markdown pour l'IA (.md)...`,
  vous pouvez lire à l'écran le texte exact généré avant de l'expédier vers une console ou de le copier dans un chat.
- **Ce qui est demandé cette fois-ci :** dans l'aperçu d'envoi, le bloc « Mission de cette session ».
- **Comment vous souhaitez qu'elle travaille toujours :** dans ce même aperçu, le bloc des règles communes.

Avant l'envoi, vous pouvez également tout préparer depuis **`🧭 Projet et instructions pour l’IA`**.
L'onglet **Instructions** permet de choisir la **Langue du document pour l’IA**, d'enregistrer des règles
communes et d'inspecter les sources natives. L'onglet **Modèles de mission** copie une seule recette dans la
mission, et **Aperçu complet** affiche la chaîne exacte.

Une source native n'est jamais intégrée du simple fait de sa présence sur le disque. Elle apparaît d'abord
comme **Nouvelle source** ; après avoir cliqué sur **Accepter cette version**, elle reste présélectionnée
tant que ses octets ne changent pas. Si elle change, elle se décoche et présente l'ancienne version aux
côtés de l'actuelle. Vous pouvez inclure ou exclure une source de la session finale via sa case à cocher.

Si deux instructions se contredisent, le document lui-même établit la chaîne de commandement, de la plus forte
à la plus faible : **vos corrections sur la carte**, le contrat MMCelt, les règles du projet et,
en tout dernier lieu, la mission de la session en cours.

### 🔙 Ce que l'IA peut faire à son retour, et ce qu'elle ne peut pas`

**Elle peut** lire la carte, lire vos corrections, ajouter des nœuds, rapporter des progrès et créer une
nouvelle carte. Ce sont six outils et pas un de plus.

**Elle ne peut pas** modifier votre fichier `.mmcelt` de son propre chef, sortir du dossier de travail que
vous avez autorisé, ni **décréter que vous avez validé quoi que ce soit**. Tout ce qu'elle apporte naît marqué
comme généré par l'IA, et faire passer cela à l'état approuvé relève exclusivement de votre décision, toujours.

De plus, une traçabilité est conservée : le texte exact que vous avez validé est consigné dans `.mmcelt/sesiones`,
et si une modification automatique survient sur une carte que vous avez retouchée, une copie `.bak` est créée au préalable.

La confirmation n'est pas une simple capture décorative. Juste avant le lancement, MMCelt revérifie
la carte, les règles et les fichiers sources. Si l'un d'eux a été modifié, l'application signale que la
confirmation est expirée, n'ouvre pas l'agent et vous invite à procéder à une nouvelle vérification.

### ⚠️ Ce que ce dispositif n'est pas

Le document indique à l'IA que le contenu de votre carte constitue des **données, non des ordres**.
Cela prévient les malentendus, mais **ne constitue pas une barrière de sécurité absolue** : un titre ou une
note importés de l'extérieur pourraient être formulés sous la forme d'une injection d'instructions.

La véritable protection repose sur un autre socle : le dossier de travail autorisé, le périmètre
strictement borné aux six outils, et le fait que vous inspectiez l'aperçu avant tout envoi.
C'est la raison pour laquelle cette relecture préalable mérite d'être effectuée au moins une fois.

**💡 Que faire si vous n'avez pas d'agents installés en console ?**
Si en accédant à `🤖 Intelligence artificielle` → `📤 Envoyer à...` le message « Aucun agent détecté » s'affiche, ne vous inquiétez pas : **vous n'avez nullement besoin de consoles techniques ni d'agents avancés pour exploiter l'IA avec MMCelt**.

Vous pouvez travailler en toute simplicité depuis votre navigateur web habituel :
1. Utilisez `📁 Fichier` → `🤖 Exporter le Markdown pour l'IA (.md)` (ou visualisez d'abord le texte via `🤖 Intelligence artificielle` → `👁️ Aperçu du Markdown pour l'IA (.md)...`).
2. Ouvrez dans votre navigateur n'importe quel service de chat IA (ChatGPT, Claude, Gemini...).
3. Collez le texte exporté et transmettez vos consignes pour faire évoluer la carte.
4. Lorsque l'IA vous répond par un bloc au format JSON ou Markdown, copiez-le.
5. Dans MMCelt, cliquez sur `🤖 Intelligence artificielle` → `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...` et collez la réponse. Vos nouvelles branches apparaîtront aussitôt.

Si vous installez ultérieurement un agent dans le terminal de votre système (comme Claude Code, Codex CLI ou Gemini CLI),
MMCelt le reconnaîtra automatiquement dès qu'il sera présent dans votre variable PATH.

### 📚 Où poursuivre, selon vos objectifs

- **Formuler clairement votre intention :** le sujet sur la vision du créateur.
- **Recadrer une IA qui dévie :** le sujet sur le contrôle humain et les corrections.
- **Comprendre le document généré :** le sujet sur l'exportation Markdown.
- **Rapporter manuellement une réponse copiée :** le sujet sur l'importation depuis l'IA.
- **Envoyer et observer le retour automatique :** le sujet sur l'envoi et la surveillance.
- **Connecter votre agent par MCP :** le sujet sur 🟣 Serveur MCP et agents IA.

Tous ces thèmes sont accessibles dans ce même sélecteur, ordonnés selon le fil naturel de vos besoins.
