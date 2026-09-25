# 🧭 Projet et instructions pour l’IA

C'est ce qui transforme votre carte en un document qu'une IA peut véritablement comprendre.

### 🎯 Pourquoi est-ce indispensable ?

Une carte mentale, prise isolément, n'est qu'un ensemble de boîtes et de flèches. Une IA peut les lire,
mais elle ignore **pourquoi** elles sont là. Il lui manque ce que vous avez en tête et que vous n'avez
écrit sur aucun nœud : quel problème vous tentez de résoudre, pourquoi vous avez agencé les branches ainsi,
et ce que vous considéreriez comme un résultat satisfaisant.

Sans cela, l'IA comble les vides. Et combler les vides avec des suppositions est précisément ce qui produit
une réponse très séduisante mais totalement inutile.

### 📝 Comment renseigner ces informations

1. Ouvrez **`🤖 Intelligence artificielle`** → **`🧭 Projet et instructions pour l’IA`** (également accessible depuis le bouton du panneau latéral ou après édition dans `✏️ Édition`).
2. Renseignez les trois champs. Aucun n'est obligatoire, mais plus ils sont précis, meilleur est le résultat :
   - **Vision :** ce que vous cherchez à concrétiser. En une ou deux phrases, avec vos propres mots.
   - **Objectifs :** les résultats tangibles attendus. Des « jalons et livrables », pas de simples souhaits.
   - **Contexte / Public :** qui va l'utiliser, ou dans quel environnement le projet doit s'intégrer.

**Un conseil :** écrivez comme si vous expliquiez le projet à une personne venant tout juste de rejoindre l'équipe.
C'est très exactement ce qu'est l'IA chaque fois qu'elle ouvre votre carte : un nouvel arrivant, sans mémoire
des échanges antérieurs.

### 📦 Ce qui est envoyé à l'IA, exactement

C'est ici qu'il convient d'éviter tout malentendu. **Ces trois champs ne sont pas les seuls à être transmis.**
L'exportation manuelle préserve un document explicatif complet comprenant un préambule et dix sections structurées :

1. **Préambule :** les consignes de départ et le cadre opérationnel pour le modèle.
2. **Votre vision et vos objectifs :** ce que vous avez saisi dans cette fenêtre.
3. **Résumé et métriques :** nombre de nœuds, branches issues de l'idée centrale, liaisons
   croisées, étiquettes employées et répartition des nœuds par état.
4. **La structure complète** de la carte, avec les notes de chaque nœud, leurs étiquettes et les chemins
   de fichiers si vous les avez spécifiés.
5. **Vos corrections :** les nœuds sur lesquels vous avez exigé un changement et ceux que vous avez approuvés.
6. **La matrice des connexions croisées** entre branches distinctes.
7. **Les doutes et décisions en suspens** que vous avez signalés.
8. **Un diagramme** de la carte, au format Mermaid.
9. **Suggestions de prompts :** des requêtes prêtes à l'emploi que vous pouvez copier si vous le souhaitez.
10. **Comment vous restituer le travail :** le contrat avec les six outils MCP, la règle stipulant
    qu'aucune IA ne peut décréter votre approbation, et les valeurs exactes à utiliser.

Les sections 5, 6 et 7 n'apparaissent que si vous avez du contenu à y inscrire. Les autres sont systématiques.
Les trois suggestions de prompts de la section 9 sont des options pour vous : l'agent ne reçoit pas les trois
ordres en même temps.

En clair : c'est la carte entière qui voyage, pas un résumé tronqué. Si vous avez rédigé une longue note sur
un nœud, l'IA la lira intégralement.

### 🧩 Les quatre onglets de la fenêtre

Pour commencer, l'onglet **Projet** suffit amplement.
Vous y retrouvez la vision, les objectifs, le public et l'auteur.
Les autres onglets sont facultatifs et interviennent lorsque vous désirez affiner le pilotage d'une session :

- **Instructions** conserve des règles communes pour Claude, Codex et Gemini. Vous pouvez choisir la
  langue du document sans modifier la langue de l'application, restaurer un modèle recommandé ou rédiger
  vos propres règles. Ce que vous écrivez est préservé textuellement : MMCelt ne le traduit jamais à votre insu.
- **Modèles de mission** propose trois points de départ. Cliquer sur l'un d'eux copie uniquement ce modèle
  dans la mission modifiable ; les trois ne sont jamais envoyés ensemble. La mission est conservée à la fermeture
  de la fenêtre et réapparaît lorsque vous choisissez l'agent.
- **Aperçu complet** affiche le résultat exact des quatre blocs. Le contrat et le contexte
  sont protégés ; les règles et la mission sont les deux blocs dont vous décidez.

Si `AGENTS.md`, `CLAUDE.md` ou `GEMINI.md` sont présents sur le disque, ils sont présentés comme des sources distinctes.
Une **Nouvelle source** ne s'intègre jamais seule. Cliquez sur **Accepter cette version** uniquement après l'avoir lue.
Si le fichier change ultérieurement, il apparaîtra comme **Source modifiée ; nécessite une révision**, sera décoché
et vous pourrez comparer la **Version acceptée précédente** avec la **Version actuelle**.
Si aucun n'existe, l'onglet indique le dossier précis où vous pouvez les créer manuellement.

### ✍️ Vous pouvez corriger la demande avant qu'elle ne parte

Cette étape passe souvent inaperçue, alors qu'elle constitue l'une des fonctions les plus précieuses du logiciel.

Lorsque vous utilisez **`🤖 Intelligence artificielle`** → **`📤 Envoyer à...`** et choisissez un agent, **rien
n'est encore expédié**. Un aperçu s'ouvre, présentant le texte complet réparti en quatre blocs :

| Bloc | Modifiable ? | Description |
|---|---|---|
| **Contrat MMCelt (protégé)** | Non | Les règles fondamentales, identiques pour tous les agents |
| **Règles communes du projet** | **Oui** | Votre méthode de travail, reconduite à chaque session |
| **Contexte de la carte (généré)** | Non | Généré à partir de la carte ; pour le changer, modifiez la carte |
| **Tâche de cette session** | **Oui** | Ce que vous voulez obtenir **maintenant**, pour cette intervention |

Si deux instructions se contredisent, le texte lui-même fixe la hiérarchie, de la plus forte à la plus
faible : **vos corrections sur la carte**, le contrat MMCelt, les règles du projet et, enfin,
la tâche de la session en cours.

En dessous se trouvent quatre boutons d'action. `Réinitialiser cette session` rétablit la demande.
`Enregistrer comme règles communes` est la **seule** action qui enregistre durablement
votre profil dans `.mmcelt/instrucciones-agente.md`.
`Annuler` ne conserve rien et ne laisse aucune trace. Le dernier bouton lance l'agent.

**Lisez cet aperçu au moins une fois.** Si la carte, les règles ou un fichier source changent après l'ouverture
de la confirmation, MMCelt la déclare expirée et exige une nouvelle vérification. Ainsi, le dossier `inicio.md`
et l'agent reçoivent exactement les octets que vous avez validés à l'écran.

### 🤔 Si vous n'avez jamais travaillé ainsi

Il n'est pas nécessaire de remplir les trois champs dès le premier jour. Une carte sans vision s'exporte parfaitement.
Mais la différence entre une IA qui récite ce que vous saviez déjà et une IA qui vous apporte un éclairage pertinent
réside presque toujours ici, et non dans le modèle que vous employez.

Commencez par la **Vision**, même s'il ne s'agit que de deux lignes. C'est le champ qui transforme le plus la réponse.
