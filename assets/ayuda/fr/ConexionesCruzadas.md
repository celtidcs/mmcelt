# 🔗 Liaisons croisées et dépendances

Elles expriment ce que la structure en branches **ne peut pas dire** : que deux parties du projet,
rattachées à des endroits différents, dépendent l'une de l'autre.

### 🌳 Pourquoi elles sont nécessaires

Une carte mentale est un arbre : chaque idée pend d'une autre. Cela fonctionne jusqu'au moment où
« le système de paiement » a besoin de « l'inscription des utilisateurs », et où les deux se
trouvent sur des branches qui ne se rencontrent jamais.

Vous pourriez les déplacer pour les rapprocher, mais la carte cesserait alors de refléter votre
façon de penser pour refléter une limite de l'outil. Les liaisons croisées évitent cela :
**vous laissez chaque idée là où elle a du sens et vous tracez la relation à part.**

### 🏷️ Les cinq types, et quand utiliser chacun

Le type choisi **change ce que l'IA comprend** : il vaut donc mieux ne pas choisir au hasard.

- **➡️ Dépendance (requiert) :** A ne peut pas fonctionner sans B. La plus courante.
  *Exemple : « Envoyer la facture » requiert « Données fiscales du client ».*
- **⛔ Bloque :** B ne peut pas avancer tant que A n'est pas résolu. Plus fort qu'une dépendance :
  cela décrit quelque chose **d'arrêté en ce moment même**.
  *Exemple : « Choisir la base de données » bloque « Concevoir le modèle de données ».*
- **🔀 Alternative à :** deux chemins **exclusifs** ; choisir l'un écarte l'autre.
  *Exemple : « Application de bureau » est une alternative à « Application web ».*
- **✨ Synergie avec :** elles n'ont pas besoin l'une de l'autre, mais ensemble elles valent plus.
  *Exemple : « Système d'étiquettes » et « Moteur de recherche ».*
- **💡 Inspiré par :** une référence. Vous reprenez une idée ou un motif ailleurs dans la carte.
  *Exemple : « Panneau d'administration » inspiré par « Panneau utilisateur ».*

### 🛠️ Comment en créer une

1. Sélectionnez le nœud d'origine : celui qui **requiert**, **bloque** ou **s'inspire**.
2. Appuyez sur **`🔗 Liaison`** dans l'inspecteur, à droite.
   C'est aussi dans **`✏️ Édition`** → **`🔗 Créer une liaison croisée / dépendance...`**.
3. Choisissez le nœud cible, le type, et écrivez **pourquoi** cette relation existe.

**L'ordre compte.** « A requiert B » et « B requiert A » sont deux choses différentes, et l'IA le
lit littéralement. En cas d'erreur, supprimez la liaison et créez-la dans l'autre sens.

**Et le motif compte plus qu'il n'y paraît.** Une liaison sans explication indique à l'IA qu'il
existe une relation, mais pas quoi en faire. Avec le motif écrit, elle peut raisonner dessus.

### ✋ Quand NE PAS les utiliser

C'est tout aussi important, car une carte pleine de flèches croisées devient illisible :

- **Si la relation est « une chose fait partie de l'autre »**, ce n'est pas une liaison croisée :
  c'est un enfant. Rattachez-le où il faut.
- **Si tout dépend de tout**, ne tracez pas trente liaisons. C'est en général le signe qu'il
  manque un nœud réunissant cette idée commune.
- **Si la relation est évidente** — deux tâches du même bloc qui se suivent manifestement —, vous
  ne gagnez rien à la dessiner.

Une règle pratique : **tracez la liaison s'il vous a fallu réfléchir pour voir qu'elle existait.**
Ce sont celles que l'IA ne peut pas déduire seule, et celles qu'on oublie en réunion.

### 👀 Où les voir ensuite

Les liaisons qui partent d'un nœud ou y arrivent apparaissent dans l'inspecteur de droite, sous
**`🔗 Liaisons croisées :`**, et on peut les supprimer depuis là. Cette section **n'apparaît que si
le nœud en possède**, donc si vous ne la voyez pas, ce nœud n'en a aucune.

Sur le canevas, elles sont tracées en courbes, avec un style différent selon le type. Et dans le
document envoyé à l'IA, elles voyagent dans une section dédiée : la matrice des dépendances.
