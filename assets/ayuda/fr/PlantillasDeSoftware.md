# ✨ Modèles d'architecture logicielle

Démarrer un projet logiciel à partir d'une page blanche peut s'avérer intimidant. C'est pourquoi MMCelt inclut des **modèles d'architecture préconfigurés** fondés sur les meilleures pratiques de l'ingénierie moderne, prêts à l'emploi en un seul clic.

Ces modèles ne sont pas de simples schémas décoratifs : ils sont déjà structurés en couches logiques, dotés de couleurs sémantiques, de types de relations entre composants et de notes explicatives guidant à la fois votre conception et les réponses de l'intelligence artificielle.

### 🏛️ Modèles disponibles dans MMCelt

**1. Clean Architecture (Architecture propre et hexagonale) :**
Ce modèle vous aide à concevoir des systèmes robustes, maintenables et facilement testables, où les règles métier fondamentales sont totalement isolées des détails techniques et des bibliothèques externes.
Il s'articule autour de quatre couches concentriques :
- **Domaine (Cœur) :** Les entités et règles métier qui ne doivent jamais changer lorsque vous changez de base de données ou de framework.
- **Cas d'utilisation (Application) :** Les opérations concrètes que l'utilisateur ou le système peut exécuter (par exemple : inscrire un utilisateur, traiter une commande).
- **Infrastructure (Adaptateurs) :** La communication avec le monde extérieur : bases de données SQL/NoSQL, appels d'API externes, systèmes de messagerie et systèmes de fichiers.
- **Présentation (API et contrôleurs) :** Les points d'entrée du système, tels que les contrôleurs REST, les points de terminaison GraphQL ou les interfaces graphiques.

*Inclut des connexions croisées matérialisant l'inversion de dépendance : les couches externes connaissent les couches internes, mais le cœur ne dépend jamais de l'infrastructure.*

**2. Fullstack Web App (Application web complète) :**
Idéal pour planifier une application web moderne de bout en bout :
- **Frontend (Client) :** L'interface visuelle vue par l'utilisateur (composants interactifs, gestion d'état et mise en page).
- **Backend (Serveur) :** L'API hébergeant la logique métier, l'authentification, les autorisations et les validations.
- **Base de données et persistance :** Le modèle de données relationnel ou documentaire, les migrations et les caches haute performance.
- **DevOps et infrastructure :** Configuration des conteneurs Docker, pipelines d'intégration continue (CI/CD) et déploiement cloud.

### 🚀 Comment charger un modèle

1. Rendez-vous dans le menu supérieur **`📁 Fichier`** → **`✨ Modèles d'architecture`**.
2. Choisissez le modèle le plus adapté à votre objectif (**Clean Architecture** ou **Fullstack Web App**).
3. MMCelt charge la structure complète sur le canevas.
4. Vous pouvez ensuite personnaliser chaque branche : ajoutez vos propres modèles, renommez les services, définissez les priorités et cliquez sur `🤖 Intelligence artificielle` → `📤 Envoyer à...` pour inviter votre agent IA à coder selon ce schéma directeur.
