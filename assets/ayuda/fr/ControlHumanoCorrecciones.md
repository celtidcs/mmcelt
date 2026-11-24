# 🛑 Contrôle humain et directives de correction (Human-in-the-Loop)

Transforme MMCelt en votre **console de supervision et de veto** face aux propositions de l'IA.

### 🛡️ Le flux de correction :
1. L'IA vous présente une carte heuristique ou un plan de travail.
2. Si vous constatez une erreur architecturale ou une bibliothèque indésirable :
   - Changez l'état du nœud en **`⛔ Écarté`** ou **`⚠️ Correction requise`**.
   - Dans le champ **« 🛑 Correction / Directive exigée à l'IA »**, écrivez votre instruction explicite (ex. *« Ne pas utiliser de bibliothèques externes ; implémenter avec la bibliothèque standard de Rust »*).
3. Ouvrez le menu **`🤖 Intelligence artificielle` > `🛑 Envoyer corrections et directives à l'IA...`**.
4. Cliquez sur **`📋 Copier les directives de correction`** et collez-les dans votre échange avec l'IA (ou laissez Claude Code utiliser `mmcelt_get_human_feedback`).
5. L'IA prend connaissance des ordres de veto obligatoires et réajuste immédiatement son code.

### 🟣 Ce qu'a fait l'IA, d'un coup d'œil
- Les nœuds que vous créez vous-même naissent **`⚪ Non marqué`** : ils ne portent aucune icône de révision.
- Un nœud que l'IA **vient de créer** est dessiné avec **une autre couleur de fond et de bordure** jusqu'à ce que vous le sélectionniez. Il retrouve ensuite son aspect normal et conserve l'icône **`🤖 Généré par l'IA`**.
- Si l'IA **modifie un nœud qui existait déjà**, il passe en **`⏳ En attente de révision`** et l'horloge est peinte dans la **couleur d'avertissement**.

Ouvrir la carte, ou la recharger quand l'IA la modifie, n'enlève pas la mise en évidence : seule la sélection du nœud l'enlève. L'enlever n'est pas une modification de la carte : cela ne s'annule pas. En revanche, cela n'est conservé dans le fichier qu'à l'enregistrement : si vous fermez MMCelt sans avoir enregistré, il vous demande si vous voulez enregistrer.

### ✅ Approuver un nœud, et que se passe-t-il ensuite
Lorsque vous validez un nœud, attribuez-lui l'état **`🛡️ Approuvé par l'humain`** dans l'inspecteur. C'est votre signature sur ce contenu. Les contrôles de révision humaine, de priorité et d'état affichent tous des icônes homogènes dans le menu et sur le nœud de la toile.

**Cette signature expire automatiquement.** Si ultérieurement l'IA modifie ce nœud — ses notes, son état, sa priorité ou son chemin de fichier —, le nœud repasse automatiquement en **`⏳ En attente de révision`**. Il en va de même pour un nœud **`⚪ Non marqué`** ou **`⚠️ Correction requise`** que l'IA modifie.

La raison en est simple : vous avez validé un texte précis, et non le nœud pour toujours. Sans cette expiration, vous verriez votre propre validation sur des modifications que vous n'avez pas lues, ce qui va à l'encontre de la vocation de ce programme.

Réécrire la même valeur exacte ne compte pas comme une modification ; les synchronisations de routine de l'IA qui ne changent rien ne vous forcent **pas** à réviser ce qui a déjà été approuvé.

Ce que vous avez saisi dans le champ de correction **n'est jamais effacé**, même après l'application de la consigne par l'IA : vous pouvez ainsi vérifier à tout moment si elle a fidèlement exécuté votre demande.
