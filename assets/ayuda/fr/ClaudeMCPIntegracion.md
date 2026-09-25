# 🟣 Serveur MCP et agents IA

Une passerelle directe et sécurisée permettant aux intelligences artificielles de lire et de créer des cartes mentales sur votre ordinateur.

### ⚡ Connexion simple en un clic

Vous n'avez pas besoin de modifier des fichiers de configuration complexes. Connectez vos agents directement depuis l'application :

1. Rendez-vous dans le menu `🤖 Intelligence artificielle` → `🔌 Connecter MMCelt à mes IA...`.
2. MMCelt détectera automatiquement les assistants de terminal installés sur votre machine (Claude Code, Codex CLI et Gemini CLI).
3. Cliquez sur le bouton **Connecter** en face de l'assistant de votre choix. Et voilà !

Si vous installez un nouvel assistant plus tard, rouvrez simplement cette fenêtre : la détection s'exécute à chaque ouverture. L'état « Connecté » garantit que l'assistant sait où trouver MMCelt et comment communiquer avec lui via `--mcp-server`.

### ⚙️ Configuration manuelle et serveur intégré

Pour configurer manuellement votre client MCP, ajoutez cette entrée dans sa configuration JSON :
```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/chemin/vers/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": { "MMCELT_WORKSPACE": "C:/chemin/vers/votre/projet" }
    }
  }
}
```

Le serveur MCP est **directement intégré dans l'exécutable de MMCelt** : aucun environnement d'exécution externe ni dépendance supplémentaire n'est requis.

### 🛡️ Sécurité totale : dossier de travail et sauvegardes

Votre tranquillité d'esprit passe avant tout. Lorsqu'un assistant IA collabore avec MMCelt, **il n'est autorisé à lire et écrire qu'à l'intérieur du dossier de votre projet**. Ce périmètre est délimité par la variable `MMCELT_WORKSPACE`.

Toute tentative de l'IA d'accéder à d'autres répertoires sera rejetée immédiatement. De plus, par sécurité :
- **Sauvegardes automatiques :** Avant qu'un agent ne modifie un fichier existant, MMCelt conserve une copie horodatée (par exemple `projet.mmcelt.20260918-120000.bak`). Si les modifications ne vous conviennent pas, votre travail précédent reste intact.
- **Respect de votre création :** L'IA peut proposer de nouvelles branches et liaisons, mais ne **supprimera ni ne déplacera jamais** les nœuds que vous avez créés. Tout ce que l'IA propose est marqué visuellement « Généré par l'IA » afin que vous gardiez toujours la main pour approuver ou corriger.

### 🔨 Les 6 outils officiels du serveur MCP

L'assistant dispose de six outils officiels conçus spécialement pour collaborer avec vous :

- `mmcelt_workspace_info` : Consulte le dossier autorisé et les cartes `.mmcelt` présentes. C'est le premier outil appelé pour éviter d'inventer des chemins.
- `mmcelt_create_mindmap` : Crée de nouvelles cartes mentales au format `.mmcelt`.
- `mmcelt_read_mindmap` : Lit la structure de votre carte, vos notes et les questions en suspens.
- `mmcelt_sync_ai_progress` : Développe la carte en ajoutant des nœuds avec leurs rôles, priorités et relations.
- `mmcelt_get_human_feedback` : Lit vos corrections et directives avant d'effectuer des modifications sensibles.
- `mmcelt_export_ai_markdown` : Convertit la carte en un résumé structuré en Markdown.

### 🌱 Si vous avez créé la carte vous-même : respect absolu de votre travail

L'agent peut enrichir votre carte, mais jamais la remplacer. Il ajoute des nœuds avec leurs rôles et étiquettes, et crée des connexions entre branches distinctes.
Tout ajout est marqué comme généré par l'IA, ne peut être validé sans votre approbation, et ne réactivera pas un chemin que vous avez marqué comme écarté.

### 🏷️ Nœuds portant le même nom

Sur une carte de décision, il est normal de répéter des mots courants (comme « Oui », « Non » ou « En cours ») dans différentes branches. Si l'IA demande la modification d'un nœud en fournissant uniquement un nom ambigu, MMCelt rejette l'instruction et renvoie les identifiants uniques de chaque correspondance afin que l'IA précise quel nœud cibler. Tout risque de confusion est ainsi écarté.

### 🧭 Et si vous n'avez pas d'assistants installés dans la console ?

Ne vous inquiétez pas : MMCelt est pleinement fonctionnel même sans aucun agent dans le terminal. Vous pouvez interagir avec des services web comme ChatGPT, Claude.ai ou Gemini dans votre navigateur à l'aide des options `🤖 Intelligence artificielle` → `📋 Copier le prompt principal pour l'IA...` ou `💾 Exporter le fichier .md pour l'IA...`. Il vous suffit ensuite de coller la réponse dans `🤖 Intelligence artificielle` → `📥 Importer depuis l'IA (ChatGPT, Claude, Gemini)...` pour convertir instantanément le texte en nœuds visuels.
