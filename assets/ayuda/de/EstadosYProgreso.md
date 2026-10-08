# 📊 Lebenszyklus-Zustände, Fortschritt und Prioritäten

Eine Mindmap in MMCelt ist nicht nur eine statische Zeichnung: Sie ist ein lebendiges Dashboard zur Nachverfolgung Ihres Projekts. Jeder Knoten kann einen Fortschrittsstatus und eine Priorität haben, die Ihnen auf einen Blick zeigen, was fertig ist, was gerade entsteht und welche Unklarheiten den Weg versperren.

### 🏷️ Die 6 Zustände eines Knotens

Wenn Sie einen Knoten auswählen und die **Seitenleiste / Inspektor** (das Panel auf der rechten Bildschirmseite) betrachten, können Sie ihm mit einem einfachen Klick einen dieser sechs Zustände zuweisen:

1. **💡 Idee:** Ein vorläufiger Vorschlag oder Gedanke, der noch nicht bewertet wurde. Der ideale Anfangszustand für Brainstorming-Phasen.
2. **🔍 In Recherche:** Aufgabe oder Konzept in der technischen Erkundungsphase, Dokumentationsstudie oder Machbarkeitsanalyse vor Beginn der Programmierung.
3. **⏳ In Bearbeitung:** Aktive Arbeit, die genau in diesem Moment durchgeführt wird.
4. **❓ Blockade / Frage:** Kritischer Punkt, an dem die Entwicklung stillsteht, weil eine Entscheidung aussteht oder ein technisches Rätsel gelöst werden muss.
5. **✅ Abgeschlossen:** Erfolgreich fertiggestellte, getestete und validierte Aufgabe oder Komponente.
6. **⛔ Verworfen:** Eine Option, die geprüft, deren Umsetzung jedoch abgelehnt wurde. Sie als verworfen auf der Map zu belassen, ist äußerst wertvoll, um später nicht erneut über dieselbe Idee zu stolpern.

**⚡ Prioritäten (🔽 Niedrig, 🔷 Mittel, ⚡ Hoch, 🔥 Kritisch):**
Neben dem Status können Sie die Dringlichkeit jedes Zweigs festlegen. Sowohl das Menü im Inspektor als auch der Knoten auf der Leinwand zeigen diese Symbole an:
- **🔽 Niedrig und 🔷 Mittel:** Optionale Verbesserungen, untergeordnete Aufgaben oder gewöhnliche Routinearbeiten.
- **⚡ Hoch:** Zentrale Module und prioritäre Komponenten, die baldmöglichst umgesetzt werden sollten.
- **🔥 Kritisch:** Höchste Dringlichkeitsstufe oder Blockaden, die das gesamte Projekt am Vorankommen hindern.

### 🤖 Wie beeinflussen die Zustände die Arbeit mit der KI?

Die Zustände Ihrer Knoten sind nicht bloß Farben für Ihr Auge; die künstliche Intelligenz liest und interpretiert sie mit größter Sorgfalt:
- **Priorität für Ihre Blockaden:** Alle als **`❓ Blockade / Frage`** markierten Knoten werden in einem hervorgehobenen Abschnitt des Auftragsdokuments gebündelt. Die KI weiß, dass sie sich auf die Klärung dieser offenen Fragen konzentrieren muss, bevor sie Neues erfindet.
- **Respekt vor Verworfenem:** Wenn Sie einen Zweig als **`⛔ Verworfen`** kennzeichnen, versteht die KI, dass dieser Pfad bewusst abgelehnt wurde, und wird ihn Ihnen nicht erneut vorschlagen.
- **Kontext des Fertiggestellten:** Die Knoten **`✅ Abgeschlossen`** signalisieren der KI, welche Systemteile bereits existieren und funktionieren, damit sie darauf aufbaut, ohne doppelten Aufwand zu betreiben.
- **Automatische Metriken:** Im Export-Header berechnet MMCelt eine globale Übersicht (Fortschritt in Prozent, erledigte vs. offene Aufgaben), damit das Modell den genauen Reifegrad des Projekts kennt.

### 💡 Was jedes Symbol der Karte bedeutet
Fahren Sie mit der Maus über ein Symbol, um seine Bedeutung zu sehen: Das Emoji vor dem Titel ist der **Status**; die Symbole oben rechts sind die **Priorität** und die **menschliche Kontrolle**; unten rechts die **Rolle** des Knotens (das ⚡ von „Aufgabe / Aktion“ ist nicht das der hohen Priorität: der eingeblendete Text unterscheidet sie); und das 📝 unten sagt `📝 Enthält Notizen: auswählen, um sie im Inspektor zu lesen`. Während Sie einen Knoten ziehen, erscheint nichts.
