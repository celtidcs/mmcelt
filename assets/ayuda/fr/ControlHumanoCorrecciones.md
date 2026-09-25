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

### ✅ Approuver un nœud, et que se passe-t-il ensuite
Lorsque vous validez un nœud, attribuez-lui l'état **`🛡️ Approuvé par l'humain`** dans l'inspecteur. C'est votre signature sur ce contenu. Les contrôles de révision humaine, de priorité et d'état affichent tous des icônes homogènes dans le menu et sur le nœud de la toile.

**Cette signature expire automatiquement.** Si ultérieurement l'IA modifie ce nœud — ses notes, son état, sa priorité ou son chemin de fichier —, le nœud repasse automatiquement en **`⏳ En attente de révision`**.

La raison en est simple : vous avez validé un texte précis, et non le nœud pour toujours. Sans cette expiration, vous verriez votre propre validation sur des modifications que vous n'avez pas lues, ce qui va à l'encontre de la vocation de ce programme.

Réécrire la même valeur exacte ne compte pas comme une modification ; les synchronisations de routine de l'IA qui ne changent rien ne vous forcent **pas** à réviser ce qui a déjà été approuvé.

Ce que vous avez saisi dans le champ de correction **n'est jamais effacé**, même après l'application de la consigne par l'IA : vous pouvez ainsi vérifier à tout moment si elle a fidèlement exécuté votre demande.
