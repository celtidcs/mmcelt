# 🎨 Bedeutung der Farben

Farben sind **funktional, nicht dekorativ**: Sie zeigen an, zu welchem Zweig jedes Element gehört.

### 🌈 Verbindungslinien zwischen Knoten
Jeder Hauptzweig — direkte Kindknoten der Hauptidee — erhält eine eigene Farbe, und **alle untergeordneten Knoten erben diese**:

- 1. Zweig: Blau
- 2. Zweig: Smaragdgrün
- 3. Zweig: Bernstein
- 4. Zweig: Violett
- 5. Zweig: Rosa
- 6. Zweig: Türkis
- 7. Zweig: Orange
- 8. Zweig: Indigo

Dies ermöglicht es Ihnen, **jeden Gedankenstrang bis zu seinem Ursprung zu verfolgen**, selbst wenn die Map komplex verschachtelt ist.

Bei mehr als acht Zweigen wiederholt sich die Farbpalette: Der neunte Zweig beginnt wieder bei Blau.

### 👁️ Farben im Modus „Hoher Kontrast“
Wenn **👁️ Hoher Kontrast** aktiv ist, weicht die Farbliste von der obigen Darstellung ab: Der erste Zweig ist nicht blau, sondern **Cyan** — reines, gesättigtes Cyan auf schwarzem Hintergrund mit weißem Rand —, und die übrigen Zweige nutzen ebenfalls kontraststarke, gesättigte Töne.

Dies ist beabsichtigt. Das Standardblau erreicht auf rein schwarzem Hintergrund **nicht den von den WCAG-Barrierefreiheitsrichtlinien geforderten Mindestkontrast**: Nutzer dieses Themas benötigen maximale Lesbarkeit, weshalb die Farbpalette nach messbarem Kontrast statt nach visueller Harmonie gewählt wird. Reines Cyan bietet den höchsten Kontrast gegen Schwarz.

Die Grundregel **bleibt unverändert**: Jeder Zweig behält seine Farbe und vererbt sie weiter. Nur die konkrete Farbzuweisung wird angepasst.

### 🟠 Querverbindungen heben sich ab
Sie werden stets in **Bernstein** dargestellt, mit einer festen Farbe unabhängig vom Zweig. Dadurch lassen sich hierarchische Baumstrukturen sofort von transversalen Abhängigkeiten unterscheiden.

Alle Querverbindungen werden einheitlich gezeichnet; den Unterschied macht **die Textbeschriftung** in der Mitte: Der beim Erstellen eingegebene Text oder, falls leer gelassen, der Name des Verbindungstyps.

Im Mermaid-Diagramm des exportierten Dokuments variiert die Linienart, damit KI-Modelle den Verbindungstyp direkt erkennen.

### 🌍 Sprache der Anwendung
Menü **`🎨 Ansicht und Layout` → `Idioma / Language`**. Spanisch, Englisch, Französisch, Deutsch, Russisch und vereinfachtes Chinesisch stehen zur Verfügung, und die Auswahl bleibt beim Schließen des Programms gespeichert.

Das gesamte Programm spricht alle sechs: die Menüs, die Benachrichtigungen, der Inspektor, die Fenster, die Dialoge, diese Hilfe selbst und das Dokument, das an die KI übergeben wird.

Was **niemals** die Sprache ändert, ist der Inhalt Ihrer Dateien: die Feldnamen in `.mmcelt`, was an die KI gesendet wird und was in der Konfiguration der Agenten gespeichert ist. Es sind technische Formate, kein Text zum Lesen, und ihre Übersetzung würde bereits gespeicherte Maps unbrauchbar machen.

### 🌗 Die drei visuellen Themen
Menü **`🎨 Ansicht und Layout` → `Farbschema:`**. Alle drei Themen teilen dieselbe Semantik — Blau bleibt in allen der erste Zweig — und verändern den Lesehintergrund:

- **🌙 Dunkles Thema** (Standard): Schieferblau. Arbeitsfläche, Seitenleisten und Knoten nutzen drei abgestufte Farbtöne, die Ebenen ohne harte Kanten trennen. Ideal bei schwachem Licht.
- **☀️ Helles Thema**: Warmes Papier. Der Hintergrund ist Elfenbein statt reinem Weiß: Auf rein weißer Fläche verlieren weiße Knoten ihren Kontrast, und grelles Weiß ermüdet die Augen. Texte sind in Tinte-Grau gehalten.
- **👁️ Hoher Kontrast**: Schwarzer Hintergrund, durchgezogene weiße Ränder und gesättigte Farben. Entwickelt für Personen mit Sehbeeinträchtigungen oder Umgebungen mit starken Reflexionen.

**Das gewählte Thema bleibt beim Neustart gespeichert**, zusammen mit der Skalierung der Benutzeroberfläche.

Die Farben aller drei Themen wurden durch automatisierte Tests gegen die WCAG-Kontrastanforderungen geprüft.
