# 🟢 Intégration avec Codex CLI et ChatGPT

Que vous utilisiez la console ou préfériez le navigateur internet, vous pouvez exploiter l'intelligence artificielle d'OpenAI avec vos cartes mentales dans MMCelt.

### 💻 Codex CLI (dans le terminal)

Codex CLI est un agent de console s'exécutant directement sur votre ordinateur. C'est la méthode la plus fluide et automatisée pour travailler :

1. Rendez-vous dans le menu `🤖 Intelligence artificielle` → `📤 Envoyer à...` → `Codex CLI`.
2. Vous verrez un aperçu complet contenant votre carte et la mission destinée à l'IA. Prenez le temps de le lire.
3. En cliquant sur **Démarrer dans la console**, MMCelt ouvrira automatiquement le terminal avec l'environnement préparé et commencera à surveiller les modifications pour incorporer les nœuds créés par Codex.
4. Codex CLI lira la carte et les instructions par l'intermédiaire du serveur MCP. Au fil de son analyse, il pourra créer des branches, proposer des idées et mettre à jour la carte en direct.

> 💡 **Remarque :** Pour que Codex CLI puisse communiquer avec MMCelt, pensez à le connecter au préalable depuis `🤖 Intelligence artificielle` → `🔌 Connecter MMCelt à mes IA...` et redémarrez votre terminal s'il était déjà ouvert.

### 🌐 ChatGPT sur le web ou un Custom GPT

Si vous n'utilisez pas la console et préférez échanger dans votre navigateur web (avec ChatGPT gratuit, Plus ou un Custom GPT dédié) :

1. Exportez votre carte avec `📁 Fichier` → `🤖 Exporter le Markdown pour l'IA (.md)` (ou examinez le texte à l'écran via `🤖 Intelligence artificielle` → `👁️ Aperçu du Markdown pour l'IA (.md)...`).
2. Ouvrez votre navigateur, rendez-vous sur ChatGPT et joignez le fichier `.md` ou collez le texte directement dans la conversation.
3. Demandez-lui d'enrichir la carte, d'analyser les risques, d'explorer des alternatives ou de planifier de nouvelles tâches.
4. Lorsque ChatGPT vous répond (généralement avec un bloc structuré en JSON ou Markdown), copiez-le.
5. Revenez dans MMCelt et sélectionnez `🤖 Intelligence artificielle` → `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...`. Collez le texte et validez : les nouvelles branches s'intégreront à votre carte en préservant l'ensemble de votre travail préalable.

### 🔒 Différences clés et confidentialité de vos données

Il convient de distinguer la manière dont chaque environnement interagit avec vos données :
- **Codex CLI (local avec MCP) :** S'exécute sur votre propre machine et accède directement au dossier de projet autorisé par le protocole MCP.
- **ChatGPT dans le navigateur web :** S'exécute sur les serveurs d'OpenAI et n'a aucun accès direct à vos fichiers locaux ni au serveur MCP. L'interaction se fait de façon entièrement manuelle et sécurisée via l'exportation et l'importation de texte Markdown ou JSON.
