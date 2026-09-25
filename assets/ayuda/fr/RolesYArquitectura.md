# 🏛️ Rôles sémantiques et architecture de la carte

À mesure qu'un projet grandit, une carte mentale remplie de boîtes identiques peut devenir confuse. Pour éviter cela, MMCelt vous permet d'attribuer à chaque nœud un **rôle sémantique ou architectural**.

Les rôles précisent la nature de chaque idée et son importance au sein du projet. Ainsi, vous comme les intelligences artificielles pouvez distinguer instantanément un pilier stratégique d'une simple tâche ponctuelle ou d'une question ouverte.

### 📋 Les 6 rôles disponibles dans l'Inspecteur

En sélectionnant un nœud sur la carte, vous trouverez le menu déroulant **« Rôle »** dans le panneau latéral droit (Inspecteur). Vous pouvez choisir parmi six fonctions :

1. **🎯 Idée centrale :**
   - Le nœud racine de la carte heuristique, l'origine de tout l'arbre.
   - Représente le produit, l'entreprise, la recherche ou l'application que vous construisez. Une carte ne comporte qu'une seule idée centrale.

2. **🏛️ Pilier stratégique :**
   - Les grandes colonnes maîtresses qui soutiennent le projet.
   - En informatique, ils représentent souvent les couches principales (comme Frontend, Backend, Base de données ou Sécurité) ; dans une entreprise, ce peut être Marketing, Ventes, Finance ou Opérations.

3. **📌 Sous-thème / Module :**
   - Composants ou sections concrètes rattachés à un pilier stratégique.
   - Par exemple, au sein du pilier *Backend*, les sous-thèmes pourraient être *Service d'authentification*, *Module de paiement* ou *Gestionnaire de notifications*.

4. **❓ Hypothèse / Question :**
   - Questions ouvertes, décisions de conception ou expérimentations dont la faisabilité technique reste à confirmer.
   - Par exemple : *« Vaut-il mieux utiliser WebSockets ou Server-Sent Events pour le chat ? »*.

5. **⚡ Tâche / Action :**
   - Étapes concrètes, actionnables et exécutables assorties d'un livrable clair.
   - Par exemple : *« Concevoir l'écran de connexion »* ou *« Écrire les tests unitaires pour le calcul des prix »*.

6. **🔧 Ressource / Outil :**
   - Bibliothèques externes, dépendances, documentations de référence, API tierces ou outils auxiliaires nécessaires au projet.

### 💡 Comment modifier un rôle et son impact sur la carte et l'IA

Pour changer le rôle d'un nœud, sélectionnez-le et choisissez l'option voulue dans le menu déroulant **« Rôle »** de l'inspecteur latéral droit.

**🎨 Distinction visuelle sur le canevas :**
Chaque rôle confère une identité distincte à la boîte du nœud sur le plan graphique :
- Icônes et bordures spécifiques pour une identification immédiate.
- Connexions et tracés illustrant la structure organique du projet.

**🤖 Pourquoi les rôles comptent-ils pour l'IA ?**
Lorsque vous exportez ou transmettez votre carte à une IA (telle que Claude Code, Codex CLI ou Gemini CLI), le modèle ne voit pas une simple liste plate de textes :
- **Comprend l'architecture :** Sait qu'un *Pilier stratégique* requiert une vision globale et ne doit pas être altéré à la légère.
- **Distingue les doutes des certitudes :** Traite les *Hypothèses / Questions* comme des points ouverts nécessitant une analyse technique et des scénarios alternatifs.
- **Génère des tâches cohérentes :** Lorsqu'on lui demande de développer un module, l'IA propose des *Tâches / Actions* concrètes et suggère des *Ressources / Outils* recommandés pour les accomplir.
