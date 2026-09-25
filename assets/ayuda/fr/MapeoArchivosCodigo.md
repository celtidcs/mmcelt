# 📁 Mappage des fichiers de code (file_path)

Reliez les idées et décisions de votre carte mentale aux fichiers réels de votre projet sur le disque.

### 🛠️ Comment lier un fichier et à quoi ça sert :

Une carte mentale vous aide à voir grand : vous pouvez avoir un nœud intitulé « Authentification des utilisateurs » et un autre nommé « Base de données ». Cependant, votre ordinateur et vos programmes sont constitués de fichiers concrets (comme `src/auth.rs` ou `config/database.json`).

Le champ de fichier (`file_path`) est le pont reliant l'idée abstraite au fichier réel où résident son code ou sa documentation. Ainsi, la carte n'est pas un simple schéma : elle devient un index vivant de votre projet.

**Étapes pour lier un fichier :**
1. Cliquez sur n'importe quel nœud de votre carte pour le sélectionner.
2. Regardez dans le panneau latéral droit (l'Inspecteur de nœuds).
3. Dans le champ **📁 Fichier/Chemin :**, saisissez le chemin du fichier relatif au dossier de votre projet. Par exemple :
   - `src/login.rs` pour un fichier de code.
   - `documentacion/requisitos.md` pour un texte explicatif.
   - `frontend/composants/` pour désigner tout un dossier.

### 💡 Conseils pratiques et utilisation avec l'IA :

**🤖 Comment l'intelligence artificielle en tire-t-elle parti ?**
Lorsque vous travaillez avec un assistant (comme Claude Code, Codex CLI ou Gemini CLI), ou lorsque vous exportez le résumé du projet en Markdown :
- L'IA lit exactement quel fichier correspond à chaque nœud.
- Elle sait d'avance où appliquer ses modifications sans chercher aveuglément dans tout votre dépôt.
- Elle vous aide à maintenir une correspondance claire et ordonnée entre l'architecture conceptuelle et le code source.

**Conseils pratiques :**
- **Utilisez des chemins relatifs :** Écrivez toujours les chemins à partir du dossier racine de votre projet (par exemple `src/main.rs` au lieu de `C:\MesDocuments\Projet\src\main.rs`). De cette façon, si vous déplacez votre projet ou le partagez, tous les liens restent valides.
- **Chemins de dossiers :** Si un nœud regroupe plusieurs fichiers, vous pouvez terminer le chemin par une barre oblique (comme `src/services/`) pour indiquer qu'il représente tout ce répertoire.
