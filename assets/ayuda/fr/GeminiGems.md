# 🔵 Intégration avec Gemini CLI

Gemini est l'intelligence artificielle développée par Google, et **Gemini CLI** est son outil officiel en ligne de commande. MMCelt vous permet de vous connecter directement à Gemini CLI pour analyser, enrichir et développer vos cartes mentales.

### 💻 Utilisation avec Gemini CLI dans le terminal

1. Rendez-vous dans `🤖 Intelligence artificielle` → `🔌 Connecter MMCelt à mes IA...` et cochez Gemini CLI.
2. Ouvrez votre carte et sélectionnez `🤖 Intelligence artificielle` → `📤 Envoyer à...` → `Gemini CLI`.
3. Vérifiez l'aperçu du document et de la mission que l'IA va recevoir.
4. Cliquez sur **Démarrer dans la console**. MMCelt ouvrira automatiquement le terminal avec l'environnement préparé et commencera à surveiller les modifications pour incorporer les nœuds créés par Gemini.

### 💡 Astuces et conseils : Clé d'API gratuite et compatibilité

Si au lancement de Gemini CLI la console vous demande une authentification ou si vous rencontrez des difficultés avec la connexion dans le navigateur, la méthode la plus rapide et la plus fiable consiste à utiliser une **clé d'API gratuite** de Google AI Studio :

1. Connectez-vous à Google AI Studio (`https://aistudio.google.com/app/apikey`) avec votre compte Google et cliquez sur **Create API Key** (Créer une clé d'API). C'est gratuit.
2. Copiez la clé générée.
3. Configurez la variable d'environnement sur votre système avant d'ouvrir la console ou dans votre profil utilisateur :
   - **Sous Windows (PowerShell) :**
     ```powershell
     $env:GEMINI_API_KEY="votre_cle_ici"
     ```
   - **Sous Linux ou macOS (Bash/Zsh) :**
     ```bash
     export GEMINI_API_KEY="votre_cle_ici"
     ```
4. Grâce à cette variable, Gemini CLI fonctionnera immédiatement sans solliciter de connexion supplémentaire dans le navigateur.
5. Vous pouvez consulter la documentation officielle de Gemini CLI sur son dépôt : `https://github.com/google-gemini/gemini-cli`.

**ℹ️ État actuel de compatibilité des comptes sur Gemini CLI :**
Confirmé : depuis le 18 juin 2026, Google a retiré la connexion interactive « Sign in with Google » de Gemini CLI pour les comptes personnels, y compris gratuits, Google AI Pro et Ultra.
- **13 septembre 2026 :** premier test sur la machine du Directeur, avec le message littéral : *« This client is no longer supported for Gemini Code Assist for individuals. To continue using Gemini, please migrate to the Antigravity suite of products »*.
- **Répété ensuite avec plusieurs anciennes versions de Gemini CLI**, pour écarter un problème lié à la version installée : le résultat a été identique avec toutes.
- **Confirmé avec une clé d'API gratuite Google AI Studio** : la connexion a fonctionné de bout en bout, créant des nœuds et des liens vérifiés sur le disque, sans aucune restriction de compte personnel.

La méthode par clé d'API détaillée ci-dessus **est la seule confirmée** pour les comptes personnels : la connexion interactive n'est plus disponible pour eux.

### 🌐 Gemini dans le navigateur web

Si vous préférez ne pas utiliser le terminal :
1. Exportez votre carte via `📁 Fichier` → `🤖 Exporter le Markdown pour l'IA (.md)` (ou cliquez sur `🤖 Intelligence artificielle` → `👁️ Aperçu du Markdown pour l'IA (.md)...`).
2. Ouvrez le site web de Gemini dans votre navigateur, collez le texte et posez votre question.
3. Copiez la réponse de Gemini et revenez dans MMCelt : cliquez sur `🤖 Intelligence artificielle` → `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...` pour intégrer instantanément les nouvelles branches à votre carte.
