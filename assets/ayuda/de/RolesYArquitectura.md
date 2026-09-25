# 🏛️ Semantische Rollen und Map-Architektur

Wenn ein Projekt wächst, kann eine Mindmap voller identischer Kästen unübersichtlich werden. Um dies zu verhindern, erlaubt MMCelt Ihnen, jedem Knoten eine **semantische bzw. architektonische Rolle** zuzuweisen.

Rollen definieren das Wesen jeder Idee und ihr Gewicht im Projektgefüge. Auf diese Weise können Sie und KI-Modelle sofort eine tragende Säule von einer einfachen Routineaufgabe oder einer offenen Frage unterscheiden.

### 📋 Die 6 im Inspektor verfügbaren Rollen

Wenn Sie einen Knoten auf der Map auswählen, finden Sie im rechten Seitenpanel (Inspektor) das Dropdown-Menü **„Rolle“**. Sie können zwischen sechs Funktionen wählen:

1. **🎯 Zentrale Idee:**
   - Der Wurzelknoten der Mindmap, der Ursprung des gesamten Baums.
   - Repräsentiert das Produkt, das Unternehmen, die Forschung oder die Anwendung, die Sie erstellen. In einer Map existiert genau eine zentrale Idee.

2. **🏛️ Strategische Säule:**
   - Die tragenden Hauptsäulen des Projekts.
   - In der Softwareentwicklung stellen sie typischerweise Hauptschichten dar (wie Frontend, Backend, Datenbank oder Sicherheit); im Geschäftsleben können dies Marketing, Vertrieb, Finanzen oder Betrieb sein.

3. **📌 Unterthema / Modul:**
   - Konkrete Komponenten oder Abschnitte, die einer strategischen Säule untergeordnet sind.
   - Beispielsweise könnten innerhalb der Säule *Backend* die Unterthemen *Authentifizierungsdienst*, *Zahlungsmodul* oder *Benachrichtigungsverwaltung* heißen.

4. **❓ Hypothese / Frage:**
   - Offene Fragestellungen, Architekturentscheidungen oder Experimente, deren technische Machbarkeit noch unklar ist.
   - Beispiel: *„Sollten wir WebSockets oder Server-Sent Events für den Live-Chat nutzen?“*.

5. **⚡ Aufgabe / Aktion:**
   - Konkrete, handlungsorientierte und ausführbare Schritte mit einem klaren Ergebnis.
   - Beispiel: *„Anmeldebildschirm gestalten“* oder *„Unit-Tests für die Preisberechnung schreiben“*.

6. **🔧 Ressource / Werkzeug:**
   - Externe Bibliotheken, Abhängigkeiten, Referenzdokumentationen, Drittanbieter-APIs oder Hilfswerkzeuge, die für das Projekt benötigt werden.

### 💡 Rollen anpassen und ihre Auswirkung auf Map und KI

Um die Rolle eines Knotens zu ändern, wählen Sie diesen aus und bestimmen die gewünschte Option im Dropdown **„Rolle“** des rechten Inspektors.

**🎨 Visuelle Unterscheidung auf der Arbeitsfläche:**
Jede Rolle verleiht dem Knotenkasten im visuellen Schema eine unverwechselbare Identität:
- Eindeutige Symbole und Rahmen zur mühelosen Erkennung.
- Verbindungslinien, die den organischen Aufbau des Projekts abbilden.

**🤖 Warum Rollen für die KI entscheidend sind:**
Wenn Sie Ihre Map exportieren oder an eine KI senden (wie Claude Code, Codex CLI oder Gemini CLI), sieht das Modell keine flache Textliste:
- **Versteht die Architektur:** Weiß, dass eine *Strategische Säule* Weitblick erfordert und nicht leichtfertig modifiziert werden darf.
- **Trennt Zweifel von Gewissheiten:** Behandelt *Hypothesen / Fragen* als Analysefragen, die technischer Prüfung und Abwägung bedürfen.
- **Erzeugt stimmige Aufgaben:** Wird die KI gebeten, ein Modul auszuarbeiten, formuliert sie konkrete *Aufgaben / Aktionen* und empfiehlt geeignete *Ressourcen / Werkzeuge*.
