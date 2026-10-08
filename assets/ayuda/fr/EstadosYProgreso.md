# 📊 États de cycle de vie, progrès et priorités

Une carte mentale dans MMCelt n'est pas un simple dessin statique : c'est un tableau de bord vivant pour le suivi de votre projet. Chaque nœud peut porter un état d'avancement et une priorité qui vous permettent de savoir d'un coup d'œil ce qui est prêt, ce qui est en cours de construction et quels doutes bloquent la progression.

### 🏷️ Les 6 états d'un nœud

Lorsque vous sélectionnez un nœud et consultez l'**Inspecteur latéral** (le panneau situé à droite de l'écran), vous pouvez lui attribuer l'un de ces six états d'un simple clic :

1. **💡 Idée :** Une proposition ou suggestion préliminaire qui n'a pas encore été évaluée. C'est l'état initial parfait pour les séances de brainstorming.
2. **🔍 En étude :** Tâche ou concept en phase d'exploration technique, de lecture documentaire ou d'étude de faisabilité avant de commencer à coder.
3. **⏳ En cours :** Travail actif en train d'être réalisé en ce moment même.
4. **❓ Blocage / Question :** Point critique où le développement est arrêté faute d'une décision ou de la résolution d'une inconnue technique.
5. **✅ Terminé :** Tâche ou composant finalisé avec succès, testé et validé.
6. **⛔ Écarté :** Une option qui a été examinée mais dont l'implémentation a été rejetée. La laisser sur la carte comme écartée est très précieux pour éviter de trébucher à nouveau sur la même idée plus tard.

**⚡ Priorités (🔽 Basse, 🔷 Moyenne, ⚡ Haute, 🔥 Critique) :**
À côté de l'état, vous pouvez marquer l'urgence de chaque branche. Le menu de l'inspecteur et le nœud sur la toile affichent ces icônes :
- **🔽 Basse et 🔷 Moyenne :** Améliorations facultatives, tâches secondaires ou travail quotidien ordinaire.
- **⚡ Haute :** Modules centraux et composants prioritaires à construire dès que possible.
- **🔥 Critique :** Urgences maximales ou blocages empêchant le reste du projet d'avancer.

### 🤖 Comment les états influencent-ils le travail avec l'IA ?

Les états de vos nœuds ne sont pas de simples couleurs pour vous ; l'intelligence artificielle les lit et les interprète avec grand soin :
- **Priorité à vos blocages :** Tous les nœuds marqués comme **`❓ Blocage / Question`** sont regroupés dans une section mise en avant du document de commande. L'IA sait qu'elle doit se concentrer sur la résolution de ces inconnues avant d'inventer de nouvelles choses.
- **Respect de ce qui est écarté :** Si vous marquez une branche comme **`⛔ Écarté`**, l'IA comprend que cette voie a été délibérément rejetée et n'insistera pas pour vous la proposer.
- **Contexte des réalisations terminées :** Les nœuds **`✅ Terminé`** indiquent à l'IA quelles parties de votre système existent déjà et fonctionnent, afin qu'elle s'appuie dessus sans dupliquer les efforts.
- **Métriques automatiques :** Dans l'en-tête de l'exportation, MMCelt calcule une synthèse globale (pourcentage d'avancement, tâches terminées vs en attente) pour que le modèle connaisse le stade exact de maturité du projet.

### 💡 Ce que signifie chaque icône de la carte
Survolez une icône pour voir sa signification : l'emoji devant le titre est le **statut** ; ceux du coin supérieur droit sont la **priorité** et le **contrôle humain** ; en bas à droite, le **Rôle** du nœud (le ⚡ de « Tâche / Action » n'est pas celui de la priorité Haute : le texte qui apparaît les distingue) ; et le 📝 du bas indique `📝 Contient des notes : sélectionnez-le pour les lire dans l'inspecteur`. Rien n'apparaît pendant que vous faites glisser un nœud.
