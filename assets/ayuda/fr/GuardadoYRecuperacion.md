# 💾 Enregistrement et récupération

### 💾 Enregistrement manuel (Ctrl + S)
Enregistre à l'emplacement de votre choix. En cas d'erreur — permissions, disque saturé, dossier cloud non synchronisé —, **l'application vous en informe immédiatement**. Elle ne confirme jamais un enregistrement sans l'avoir vérifié.

Lors de la sélection du dossier, la boîte de dialogue s'ouvre à l'emplacement le plus pertinent : le dossier du fichier actuel ou **le dossier du projet de code référencé par la carte** si les nœuds contiennent des chemins de fichiers.

### 🛟 Sauvegarde automatique
Toutes les deux minutes, si la carte a été modifiée, une copie de sécurité est enregistrée. Si le programme se ferme de manière inattendue, sa récupération vous sera proposée au redémarrage.

Cet intervalle **peut être modifié, voire désactivé**, dans `🎨 Affichage et disposition → 💾 Enregistrement automatique` : désactivé, chaque minute, toutes les deux, cinq ou dix minutes. Le choix est conservé d'une session à l'autre.

**Elle ne remplace pas l'enregistrement manuel.** Il s'agit d'un filet de sécurité, non d'un stockage permanent. Continuez d'utiliser Ctrl + S.

La copie est stockée dans le dossier de données de MMCelt (`%APPDATA%\MMCelt\recuperacion\` sous Windows), **jamais dans vos dossiers de travail** : cela évite la création de fichiers indésirables dans vos projets. Seule la dernière session est conservée, et elle est supprimée dès que vous effectuez un enregistrement manuel.

Dans les modes de disposition automatique, déplacer un nœud ne marque pas le document comme modifié : le moteur le repositionnera de toute façon, évitant des écritures inutiles lors d'une simple consultation.

En **Position libre manuelle**, le déplacement compte comme une modification, car le positionnement manuel relève de votre travail délibéré.

### ⚠️ Si la récupération vous est proposée au démarrage
La boîte de dialogue affiche la date de la sauvegarde et le titre de la carte. Le choix vous appartient : si vous avez fermé volontairement sans enregistrer, écartez-la. Elle n'est jamais restaurée automatiquement.

### 🛡️ Fichiers corrompus
Une carte décrivant une structure invalide — boucles cycliques, absence de nœud racine, références brisées — est rejetée à l'ouverture en indiquant le nœud en cause. Ce n'est pas une contrainte arbitraire : la charger bloquerait l'application. Si le fichier a été généré par une IA, demandez-lui de le régénérer ; si une copie `.bak` existe à côté, essayez de l'ouvrir.

### 🌐 Formats ouverts d'échange (OPML et FreeMind .mm)
En plus du format natif `.mmcelt`, vous pouvez exporter et importer des cartes heuristiques dans deux standards ouverts non propriétaires : OPML (plans hiérarchiques) et `.mm` (FreeMind et Freeplane).

Lors de l'exportation vers OPML ou `.mm`, les titres de nœuds, l'arborescence et les notes textuelles sont enregistrés dans les balises standard du format, lisibles par n'importe quelle application externe. Les données propres à MMCelt non prises en charge nativement par ces formats (état, priorité, rôle, révision humaine, chemins de code et connexions transversales) sont sérialisées en JSON structuré au bas de chaque note.

La réimportation de ce fichier dans MMCelt reconstruit intégralement la carte à 100 % sans perdre la moindre métadonnée ni connexion latérale. Si vous ouvrez le fichier dans un autre outil, vous visualiserez l'arbre complet et pourrez lire les métadonnées sous forme de texte informatif dans la note.

Lors de l'importation d'un fichier externe conçu dans une autre application sans métadonnées MMCelt, le logiciel applique des valeurs par défaut sécurisées (état Idée, priorité Moyenne, rôle Sous-thème sauf la racine qui reçoit Idée centrale), applique automatiquement une disposition visuelle équilibrée pour éviter que les nœuds ne s'amassent au centre, et affiche un avertissement transparent détaillant les champs initialisés.
