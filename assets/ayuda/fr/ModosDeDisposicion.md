# 📐 Modes de disposition spatiale

Pour que votre carte mentale reste toujours claire, propre et lisible, MMCelt dispose d'algorithmes qui calculent la position de chaque nœud et empêchent les branches de se chevaucher.

Vous pouvez choisir la façon d'organiser votre carte à tout moment selon vos préférences visuelles :

### 🎨 Modes disponibles :

1. **Arbre équilibré (Gauche/Droite) :**
   - Place l'idée centrale à l'origine du canevas.
   - Répartit les branches principales de manière équilibrée à gauche et à droite, en distribuant équitablement le poids visuel.
   - Calcule automatiquement la hauteur de chaque sous-branche pour que les textes ne se superposent jamais. C'est le mode par défaut et le plus pratique pour la plupart des projets.

2. **Radial / Circulaire :**
   - Positionne l'idée centrale au milieu et déploie les branches en un éventail circulaire à 360 degrés tout autour.
   - Idéal pour les séances de remue-méninges (*brainstorming*) ou les cartes comportant de nombreux thèmes principaux autour d'un concept unique.

3. **Positionnement libre manuel :**
   - Vous permet de déplacer n'importe quel nœud librement en le faisant glisser avec le bouton gauche de la souris jusqu'à l'emplacement exact souhaité.
   - Dans ce mode, le moteur n'impose pas la position des nœuds, vous offrant un contrôle manuel total pour concevoir des schémas personnalisés.

### ⚡ Comment changer de mode et réorganiser

- Dans la barre supérieure, ouvrez le menu **`🎨 Affichage et disposition`** pour sélectionner le mode de disposition de votre choix.
- Si vous souhaitez à tout moment recalculer les positions et remettre de l'ordre dans la carte, cliquez sur **`🎨 Affichage et disposition`** → **`🔄 Réorganiser les nœuds`** (ou utilisez le raccourci correspondant). Tous les nœuds s'aligneront à nouveau harmonieusement.

### 🔀 Déposer un nœud sur un autre
Si vous faites glisser un nœud et le déposez avec son centre sur une autre carte, un petit menu apparaît à cet endroit pour que vous choisissiez ce que vous vouliez faire :

- **`➕ En faire un enfant`** : le nœud, avec toutes ses branches, dépend désormais du nœud situé dessous. L'option est désactivée si ce n'est pas possible : le nœud dessous est déjà son parent, celui que vous déplacez est la racine, ou le nœud dessous se trouve dans ses propres branches (cela créerait une boucle).
- **`↔ En faire un frère`** : le nœud dépend désormais du même parent que le nœud dessous, juste après lui. Indisponible si le nœud dessous est la racine ou s'ils sont déjà frères.
- **`🔗 Relier par un lien`** : crée une connexion transversale vers le nœud dessous et remet le nœud déplacé à sa place. Indisponible s'ils sont déjà parent et enfant ou déjà reliés : ce serait une seconde ligne par-dessus celle qui les unit.
- **`➡ Déplacer ici sans recouvrir`** : le place à côté du nœud dessous, dans l'espace libre le plus proche, sans recouvrir aucune carte.
- **`↩ Annuler`** : le remet à sa place. `Échap` ou un clic hors du menu font de même.

Tant que le menu est ouvert, les touches qui modifient la carte n'agissent pas. Chacune des options peut être annulée avec `Ctrl + Z`.
