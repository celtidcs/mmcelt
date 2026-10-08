# ℹ️ Version et compilation

### 👀 Où la trouver
Le numéro de version apparaît dans la **barre de titre de la fenêtre** et dans la **barre d'état**, accompagné de la date de compilation.

Pour un diagnostic complet : **❓ Aide → ℹ️ À propos de MMCelt**, ou en cliquant sur la version dans la barre d'état. Sont affichés la version, la date, le commit, la branche et — information essentielle — **le chemin exact de l'exécutable en cours d'utilisation**.

### 💻 En ligne de commande
```
mmcelt --version
```

Pratique lorsque vous disposez de plusieurs copies sur le disque et souhaitez vérifier laquelle est active sans lancer l'interface graphique.

### 🤔 Pourquoi le numéro de version ne suffit-il pas ?
Le numéro de version ne change pas entre deux compilations successives. Il indique la version cible *visée*, non **le binaire précis** qui a été exécuté.

La question se pose fréquemment lorsque plusieurs exécutables coexistent sur le disque : celui du dépôt, une version portable sur clé USB ou une copie de test. Tous portent le même nom. La date, le commit et le chemin d'accès permettent de les identifier formellement.

### ⚠️ Mention « modifications non enregistrées »
Si cet avertissement apparaît, cela signifie que le programme a été compilé avec des modifications qui n'avaient pas encore été validées dans le dépôt git : cet exécutable ne correspond exactement à aucun commit propre.

### ⬆ Avis de nouvelle version
Au démarrage, MMCelt demande à GitHub quelle est la dernière version publiée. Si elle est plus récente que la vôtre, la barre supérieure affiche `⬆ Nouvelle version disponible :` avec le numéro de version. C'est un lien : un clic ouvre la page de cette version dans le navigateur. **Le programme ne télécharge ni n'installe rien** ; la mise à jour reste votre décision.

S'il n'y a pas de connexion ou si GitHub ne répond pas, rien ne se passe : il n'y a simplement pas d'avis. Pour désactiver la vérification, décochez `Vérifier au démarrage s'il existe une nouvelle version` dans le menu `🎨 Affichage et disposition` ; désactivée, le programme ne se connecte pas à internet au démarrage.
