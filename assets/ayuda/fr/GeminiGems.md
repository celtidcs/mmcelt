# 🔵 Intégration avec Antigravity CLI (agy)

Google Antigravity est la plateforme de développement avancée de Google, et **Antigravity CLI (`agy`)** est son outil officiel en ligne de commande qui remplace l'ancienne console Gemini CLI. MMCelt vous permet de vous connecter directement à Antigravity CLI pour analyser, enrichir et développer vos cartes mentales.

### 💻 Utilisation avec Antigravity CLI (`agy`) dans le terminal

1. Rendez-vous dans `🤖 Intelligence artificielle` → `🔌 Connecter MMCelt à mes IA...` et cliquez sur **Connecter** à côté d'Antigravity CLI (agy).
2. Ouvrez votre carte et sélectionnez `🤖 Intelligence artificielle` → `📤 Envoyer à...` → `Antigravity CLI (agy)`.
3. Vérifiez l'aperçu du document et de la mission que l'IA va recevoir.
4. Cliquez sur **Démarrer dans la console**. MMCelt ouvrira automatiquement le terminal exécutant `agy -i` avec l'environnement préparé et commencera à surveiller les modifications pour incorporer les nœuds créés par l'agent via MCP.

### 💡 Authentification avec un compte Google standard

Antigravity CLI fonctionne nativement avec votre **compte Google standard** :
- Aucune clé d'API Google AI Studio ni variable d'environnement supplémentaire n'est requise.
- Si vous n'êtes pas encore connecté dans `agy`, l'outil ouvrira le navigateur pour vous authentifier avec votre compte Google ordinaire.
- Le serveur MCP de MMCelt est enregistré dans `~/.gemini/config/mcp_config.json`, permettant à l'agent de lire et modifier la carte dans le dossier de projet autorisé.

### 🌐 Gemini dans le navigateur web

Si vous préférez ne pas utiliser le terminal :
1. Exportez votre carte via `📁 Fichier` → `🤖 Exporter le Markdown pour l'IA (.md)` (ou cliquez sur `🤖 Intelligence artificielle` → `👁️ Aperçu du Markdown pour l'IA (.md)...`).
2. Ouvrez le site web de Gemini dans votre navigateur, collez le texte et posez votre question.
3. Copiez la réponse de Gemini et revenez dans MMCelt : cliquez sur `🤖 Intelligence artificielle` → `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...` pour intégrer instantanément les nouvelles branches à votre carte.
