# 🎨 Signification des couleurs

Les couleurs **ne sont pas décoratives** : elles indiquent à quelle branche appartient chaque élément.

### 🌈 Les lignes reliant les nœuds
Chaque branche principale — enfant direct de l'idée centrale — reçoit une couleur propre, et **toute sa descendance en hérite** :

- 1re branche : Bleu
- 2e branche : Émeraude
- 3e branche : Ambre
- 4e branche : Pourpre
- 5e branche : Rose
- 6e branche : Turquoise
- 7e branche : Orange
- 8e branche : Indigo

Cela vous permet de **suivre n'importe quel fil conducteur jusqu'au bout** et de connaître son origine, même lorsque la carte est dense et que les branches se croisent.

Au-delà de huit branches, la palette est réutilisée en boucle : la neuvième redevient bleue.

### 👁️ Dans le thème Contraste élevé, les couleurs sont adaptées
Si vous activez le thème **👁️ Contraste élevé**, cette liste diffère de ce que vous voyez : la première branche n'est pas bleue, elle est **cyan** — un cyan pur et saturé sur fond noir avec bordure blanche —, et les autres branches adoptent également des teintes saturées.

Ce n'est ni un bug ni un oubli. Le bleu standard est un bleu moyen qui, sur fond noir, **n'atteint pas le contraste requis par les normes d'accessibilité WCAG** : les personnes qui choisissent ce thème en ont spécifiquement besoin, la palette y est donc déterminée par le contraste mesurable et non par l'harmonie visuelle. Le cyan pur offre l'un des meilleurs contrastes sur fond noir.

La règle de fonctionnement **ne change pas** : chaque branche conserve sa couleur et sa descendance en hérite. Seule l'attribution des teintes est adaptée.

### 🟠 Les liaisons croisées sont distinctes
Elles sont toujours affichées en **ambre**, avec une couleur dédiée indépendante de la branche. Cela permet de distinguer d'un coup d'œil un lien hiérarchique d'une dépendance transversale.

Toutes les liaisons croisées sont tracées de manière identique ; ce qui les différencie est **l'étiquette textuelle** située en leur point médian : le texte saisi à la création ou, s'il a été laissé vide, le nom du type de liaison.

Dans le diagramme Mermaid du document exporté, leur apparence varie pour permettre au modèle de discerner immédiatement le type de liaison.

### 🌍 Langue de l'application
Menu **`🎨 Affichage et disposition` → `Idioma / Language`**. L'espagnol, l'anglais, le français, l'allemand, le russe et le chinois simplifié sont disponibles, et votre choix est conservé à la fermeture du programme.

L'ensemble de l'application est disponible dans les six langues : les menus, les notifications, l'inspecteur, les fenêtres, les boîtes de dialogue, cette même aide et le document remis à l'IA.

Ce qui **ne change jamais** de langue, c'est ce qui se trouve à l'intérieur de vos fichiers : les noms des champs du `.mmcelt`, les données transmises à l'IA et la configuration des agents. Il s'agit de formats d'échange et non de texte de lecture, et les traduire briserait les cartes déjà enregistrées.

### 🌗 Les trois thèmes visuels
Menu **`🎨 Affichage et disposition` → `Thème visuel :`**. Les trois thèmes partagent la même sémantique — le bleu reste la première branche dans chacun d'eux — et adaptent le fond de lecture :

- **🌙 Thème Sombre** (par défaut) : bleu ardoise. Le canevas, les panneaux et les nœuds utilisent trois tons étagés distincts, ce qui sépare les éléments sans bordures agressives. C'est le plus reposant en faible luminosité.
- **☀️ Thème Clair** : papier chaud. Le fond est ivoire et non blanc pur, délibérément : sur un fond blanc pur, les nœuds perdent leur contraste et la luminosité fatigue prématurément les yeux. Les textes sont gris encre plutôt que noir absolu.
- **👁️ Contraste élevé** : fond noir, bordures blanches solides et couleurs saturées. Conçu pour les personnes malvoyantes ou les environnements à forts reflets.

**Le thème choisi est mémorisé au redémarrage**, tout comme l'échelle de l'interface.

Les couleurs des trois thèmes sont validées par des tests automatisés garantissant la conformité avec les exigences de contraste WCAG.
