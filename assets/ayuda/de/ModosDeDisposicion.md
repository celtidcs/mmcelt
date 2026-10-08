# 📐 Räumliche Layout-Modi

Damit Ihre Mindmap stets übersichtlich, sauber und lesbar bleibt, verfügt MMCelt über Algorithmen, die die Position jedes Knotens berechnen und verhindern, dass sich Zweige überlappen.

Sie können die Anordnung Ihrer Map jederzeit nach Ihren visuellen Vorlieben anpassen:

### 🎨 Verfügbare Modi:

1. **Ausgewogener Baum (Links/Rechts):**
   - Platziert die zentrale Idee am Ursprung der Arbeitsfläche.
   - Verteilt die Hauptäste ausgewogen nach links und rechts, sodass das visuelle Gewicht gleichmäßig verteilt wird.
   - Berechnet die Höhe jedes Unterastes automatisch, sodass sich Texte niemals überlagern. Dies ist der Standardmodus und für die meisten Projekte am bequemsten.

2. **Radial / Kreisförmig:**
   - Platziert den Hauptgedanken im Zentrum und fächert die Zweige kreisförmig im 360-Grad-Winkel darum herum auf.
   - Ideal für Brainstorming-Sitzungen oder Maps mit vielen Hauptthemen rund um ein zentrales Konzept.

3. **Freie manuelle Positionierung:**
   - Ermöglicht es Ihnen, jeden Knoten mit der linken Maustaste frei an die exakt gewünschte Position zu ziehen.
   - In diesem Modus erzwingt die Engine keine Knotenpositionen, was Ihnen die volle manuelle Kontrolle über individuelle Schemata gibt.

### ⚡ Modi wechseln und neu anordnen

- Öffnen Sie in der oberen Leiste das Menü **`🎨 Ansicht und Layout`**, um das gewünschte Layout auszuwählen.
- Wenn Sie die Positionen neu berechnen und die Map aufräumen möchten, klicken Sie auf **`🎨 Ansicht und Layout`** → **`🔄 Knoten neu anordnen`** (oder nutzen Sie das entsprechende Tastenkürzel). Alle Knoten ordnen sich wieder harmonisch an.

### 🔀 Einen Knoten auf einem anderen ablegen
Wenn Sie einen Knoten ziehen und ihn mit seiner Mitte über einer anderen Karte loslassen, erscheint dort ein kleines Menü, damit Sie entscheiden, was Sie vorhatten:

- **`➕ Zum Kind machen`**: Der Knoten hängt mit allen seinen Zweigen nun am Knoten darunter. Die Option ist deaktiviert, wenn das nicht geht: Der Knoten darunter ist bereits sein Elternknoten, der gezogene ist die Wurzel, oder der Knoten darunter liegt in seinen eigenen Zweigen (es entstünde eine Schleife).
- **`↔ Zum Geschwister machen`**: Der Knoten hängt nun am selben Elternknoten wie der Knoten darunter, direkt dahinter. Nicht verfügbar, wenn der Knoten darunter die Wurzel ist oder beide bereits Geschwister sind.
- **`🔗 Mit Verknüpfung verbinden`**: Erstellt eine Querverbindung zum Knoten darunter und setzt den gezogenen Knoten an seinen Platz zurück. Nicht verfügbar, wenn sie bereits Eltern und Kind oder schon verbunden sind: Es wäre eine zweite Linie über der, die sie bereits verbindet.
- **`➡ Hierher verschieben, ohne zu verdecken`**: Legt ihn neben den Knoten darunter, an die nächste freie Stelle, ohne eine Karte zu verdecken.
- **`↩ Abbrechen`**: Setzt ihn an seinen Platz zurück. `Esc` oder ein Klick außerhalb des Menüs bewirken dasselbe.

Solange das Menü offen ist, wirken die Tasten, die die Mindmap ändern, nicht. Jede Option lässt sich mit `Strg + Z` rückgängig machen.
