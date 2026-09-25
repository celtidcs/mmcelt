# 🔍 Scanner automatique de dossiers de code

Si vous avez déjà un projet de programmation sur votre ordinateur ou un dossier contenant du code source, vous n'avez pas besoin de créer votre carte mentale nœud par nœud à partir de zéro. Le **scanner de code** de MMCelt analyse la structure de votre projet et génère automatiquement une carte visuelle interactive en quelques secondes.

### 🛠️ Comment utiliser le scanner et en tirer le meilleur parti :

**🧠 Pourquoi scanner votre code ?**
En convertissant un dossier de fichiers en carte heuristique, vous obtenez :
- **Une vue d'ensemble de votre architecture :** Comprenez rapidement l'organisation de vos modules, bibliothèques et composants sans vous perdre dans des dizaines de sous-dossiers.
- **Des liens directs vers vos fichiers :** Chaque boîte de la carte est associée à son chemin réel sur le disque (`file_path`), ce qui permet à vous et aux IA de savoir avec précision quel fichier physique correspond à chaque concept.
- **Le point de départ idéal pour travailler avec l'IA :** Vous pouvez demander à n'importe quel modèle d'intelligence artificielle d'analyser la structure obtenue, de détecter le code dupliqué, de proposer des refactorisations ou d'identifier des dépendances circulaires.

**Étapes pour utiliser le scanner :**
1. Dans la barre supérieure, ouvrez le menu **`📁 Fichier`** → **`🔍 Analyser le dossier de code...`**.
2. Une boîte de dialogue du système s'affiche pour vous permettre de choisir le dossier racine de votre dépôt ou projet logiciel.
3. MMCelt explore l'arborescence des dossiers en toute sécurité :
   - **Filtre le bruit technique :** Ignore automatiquement les dossiers de compilation volumineux ou les dépendances sans valeur conceptuelle (comme `node_modules/`, `target/`, `dist/`, `.git/`, les environnements virtuels Python, etc.).
   - **Crée la hiérarchie :** Place le dossier principal au centre et ramifie les modules, packages et fichiers de code clés.
4. Une fois la carte générée, vous pouvez réorganiser les nœuds à votre guise, modifier les couleurs, ajouter des notes explicatives ou marquer les zones à réviser.

**🤖 Prochaine étape avec l'IA :**
Une fois votre code scanné :
- Utilisez `🤖 Intelligence artificielle` → `👁️ Aperçu du Markdown pour l'IA (.md)...` pour voir le résumé architectural.
- Envoyez le projet à votre agent en ligne de commande préféré (`📤 Envoyer à...`) ou exportez-le pour consulter un chat web. L'IA saura exactement où se trouve chaque fichier !
