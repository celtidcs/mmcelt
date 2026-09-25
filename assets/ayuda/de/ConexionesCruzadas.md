# 🔗 Querverbindungen und Abhängigkeiten

Sie sagen etwas, das die Aststruktur **nicht ausdrücken kann**: dass zwei Teile des Projekts, die
an verschiedenen Stellen hängen, voneinander abhängen.

### 🌳 Warum sie gebraucht werden

Eine Mindmap ist ein Baum: Jede Idee hängt an einer anderen. Das funktioniert, bis „das
Zahlungssystem“ die „Benutzerregistrierung“ braucht und beide an Ästen sitzen, die sich nie
berühren.

Sie könnten sie zusammenschieben, aber dann bildet die Karte nicht mehr Ihr Denken ab, sondern
eine Grenze des Werkzeugs. Querverbindungen vermeiden das: **Sie lassen jede Idee dort, wo sie
Sinn ergibt, und zeichnen die Beziehung getrennt ein.**

### 🏷️ Die fünf Typen und wann man sie nutzt

Der gewählte Typ **verändert, was die KI versteht**. Wählen Sie also nicht zufällig.

- **➡️ Abhängigkeit (benötigt):** A funktioniert ohne B nicht. Der häufigste Fall.
  *Beispiel: „Rechnung senden“ benötigt „Steuerdaten des Kunden“.*
- **⛔ Blockiert:** B kommt nicht weiter, solange A ungelöst ist. Stärker als eine Abhängigkeit:
  es beschreibt etwas, das **gerade jetzt stillsteht**.
  *Beispiel: „Datenbank festlegen“ blockiert „Datenmodell entwerfen“.*
- **🔀 Alternative zu:** zwei **sich ausschließende** Wege; eines wählen heißt das andere
  verwerfen.
  *Beispiel: „Desktop-Anwendung“ ist eine Alternative zu „Web-Anwendung“.*
- **✨ Synergie mit:** sie brauchen einander nicht, zusammen sind sie aber mehr wert.
  *Beispiel: „Schlagwortsystem“ und „Suche“.*
- **💡 Inspiriert von:** ein Verweis. Sie übernehmen eine Idee aus einem anderen Teil der Karte.
  *Beispiel: „Verwaltungsbereich“ inspiriert von „Benutzerbereich“.*

### 🛠️ So erstellen Sie eine

1. Wählen Sie den Ausgangsknoten: den, der **benötigt**, **blockiert** oder sich **inspiriert**.
2. Drücken Sie **`🔗 Querverbindung`** im Inspektor rechts.
   Es geht auch über **`✏️ Bearbeiten`** → **`🔗 Querverbindung / Abhängigkeit erstellen...`**.
3. Wählen Sie den Zielknoten und den Typ, und schreiben Sie **warum** es diese Beziehung gibt.

**Die Richtung zählt.** „A benötigt B“ und „B benötigt A“ sind verschiedene Aussagen, und die KI
liest das wörtlich. Bei einem Fehler löschen Sie die Verbindung und legen sie umgekehrt an.

**Und der Grund zählt mehr, als es scheint.** Eine Verbindung ohne Erklärung sagt der KI, dass es
eine Beziehung gibt, aber nicht, was damit zu tun ist. Mit dem Grund kann sie darüber nachdenken.

### ✋ Wann Sie sie NICHT nutzen sollten

Das ist genauso wichtig, denn eine Karte voller Kreuzpfeile versteht niemand mehr:

- **Wenn die Beziehung „das eine ist Teil des anderen“ lautet**, ist das keine Querverbindung,
  sondern ein Kindknoten. Hängen Sie ihn an die richtige Stelle.
- **Wenn alles von allem abhängt**, zeichnen Sie keine dreißig Verbindungen. Meist fehlt ein
  Knoten, der diese gemeinsame Idee zusammenfasst.
- **Wenn die Beziehung offensichtlich ist** — zwei Aufgaben desselben Blocks, die erkennbar
  aufeinander folgen —, gewinnen Sie nichts dadurch.

Eine praktische Regel: **Zeichnen Sie die Verbindung, wenn Sie erst nachdenken mussten, um sie zu
bemerken.** Genau die kann die KI nicht selbst herleiten, und genau die vergisst man in
Besprechungen.

### 👀 Wo Sie sie danach sehen

Verbindungen, die von einem Knoten ausgehen oder dort ankommen, erscheinen im Inspektor rechts
unter **`🔗 Querverbindungen:`** und lassen sich dort löschen. Dieser Abschnitt **erscheint nur,
wenn der Knoten welche hat**; sehen Sie ihn nicht, hat dieser Knoten keine.

Auf der Arbeitsfläche werden sie als Kurven gezeichnet, je nach Typ unterschiedlich. Und im
Dokument für die KI reisen sie in einem eigenen Abschnitt mit: der Abhängigkeitsmatrix.
