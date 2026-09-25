# ⌨️ Création rapide et raccourcis clavier

MMCelt est conçu pour vous permettre de créer et d'éditer des branches à la vitesse de la pensée, sans lever les mains du clavier.

### ⚡ Tableau des raccourcis clavier standards :
| Touche | Action immédiate |
| :--- | :--- |
| **`Tab`** | Ajoute un **nœud enfant** sous le nœud sélectionné. |
| **`Entrée`** | Ajoute un **nœud frère** au même niveau hiérarchique. |
| **`Espace` / `F2`** | Ouvre l'éditeur de texte directement sur le canevas au-dessus du nœud. |
| **`Suppr` / `Retour arrière`** | Supprime le nœud sélectionné et toutes ses branches enfants en toute sécurité. |
| **`Double-clic`** | Entre dans le mode d'édition de texte du nœud. |
| **`Ctrl + N`** | Démarre une nouvelle carte. Une copie de récupération de la précédente est conservée. |
| **`Ctrl + S`** | Enregistre la carte heuristique sur le disque (`.mmcelt`). |
| **`Ctrl + E`** | Exporte le fichier Markdown enrichi pour l'IA (`.md`). |
| **`Ctrl + F`** | Centre la caméra sur l'idée centrale du projet. |
| **`Ctrl + Z`** | Annule la dernière modification. Les modifications consécutives s'annulent d'un seul coup. |
| **`Ctrl + Y`** | Rétablit la dernière modification annulée. `Ctrl + Maj + Z` fonctionne aussi. |

### ✍️ Pendant la saisie, les raccourcis modifiant la carte restent inactifs
`Tab`, `Entrée`, `Suppr`, `Retour arrière`, `Espace`, `F2` et `Ctrl + N` n'agissent que lorsque le clavier
est libre. Dès que vous écrivez dans un champ — les notes d'un nœud, la vision du projet,
la zone où vous collez la réponse de l'IA —, ces touches effectuent ce que vous attendez lors
de la frappe : `Retour arrière` efface une lettre, pas le nœud.

`Ctrl + S`, `Ctrl + E` et `Ctrl + F` restent actifs pendant la saisie : ils ne modifient pas la carte.

### 💡 Conseil Pro :
Sélectionnez le nœud central et appuyez plusieurs fois sur `Tab` pour créer rapidement 4 ou 5 piliers structurels.

### 🔍 Retrouver un nœud dans une grande carte

En haut du panneau de droite se trouve un champ de recherche. Saisissez une partie d'un
**titre** ou d'une **étiquette** : les nœuds correspondants apparaissent en dessous ; en cliquer
un le sélectionne et amène la vue jusqu'à lui **sans modifier le zoom**, afin de conserver le
niveau de détail auquel vous travailliez.

La casse et les accents sont ignorés : `diseno` trouve « Diseño ». Les **notes ne sont pas
parcourues** : ce sont de longs paragraphes, et n'importe quel mot courant renverrait la moitié
de la carte. La liste s'arrête à 50 résultats ; au-delà, mieux vaut affiner la recherche.
