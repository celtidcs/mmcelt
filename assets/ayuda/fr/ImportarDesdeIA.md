# 📥 Importation bidirectionnelle depuis l'IA

Vous permet d'importer dans MMCelt n'importe quel projet, conversation ou spécification générée par ChatGPT, Claude ou Gemini.

### 🔄 Mode d'emploi :
1. Dans MMCelt, allez dans **`🤖 Intelligence artificielle` > `📋 Copier le prompt principal pour l'IA...`**.
2. Collez cette instruction dans votre échange avec l'IA accompagnée de vos idées, code ou documents.
3. L'IA vous retournera un bloc structuré au format JSON.
4. Dans MMCelt, allez dans **`🤖 Intelligence artificielle` > `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...`**.
5. Collez le bloc obtenu et cliquez sur **`✨ Remplacer la carte par celle de l'IA`**.
6. MMCelt reconstruit la carte complète avec ses notes, tags, états, chemins de code et disposition automatique.

### ⚠️ L'importation remplace, elle n'ajoute pas
La carte actuellement ouverte **est remplacée** : les données transmises par l'IA prennent sa place et ne sont pas fusionnées avec l'existant.

Avant de procéder au remplacement, MMCelt conserve une copie de sauvegarde de la carte précédente. Si vous constatez une erreur d'importation, redémarrez l'application : à l'ouverture, elle vous proposera de récupérer cette carte.

La bonne pratique consiste néanmoins à **enregistrer avec `Ctrl + S` avant d'importer**. La sauvegarde de secours est un filet de sécurité, pas un substitut à l'enregistrement manuel.

### 🎨 Ce qui n'est pas transmis dans le JSON
L'IA retourne la structure et le contenu, non la mise en page. Les positions ajustées manuellement et les branches repliées sont réinitialisées : la carte est réorganisée selon la disposition automatique.

Tout ce qui est essentiel au développement est fidèlement préservé : titres, notes, chemins de fichiers, tags, rôles, états, priorités, connexions croisées et corrections exigées.
