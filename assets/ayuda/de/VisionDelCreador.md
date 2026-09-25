# 🧭 Projekt und Anweisungen für die KI

Dies verwandelt Ihre Map in ein Arbeitsdokument, das eine KI wirklich verstehen kann.

### 🎯 Warum ist das erforderlich?

Eine Mindmap für sich genommen ist nur eine Ansammlung von Kästen und Pfeilen. Eine KI kann diese zwar
lesen, weiß jedoch nicht, **warum** sie dort stehen. Es fehlt ihr das implizite Wissen in Ihrem Kopf,
das an keinem Knoten notiert ist: Welches Problem Sie lösen möchten, warum Sie die Zweige so angeordnet
haben und was Sie als gelungenes Ergebnis betrachten würden.

Ohne diesen Kontext füllt die KI Leerstellen eigenmächtig aus. Und das Ausfüllen von Lücken mit Annahmen
ist genau das, was eine Antwort zwar hervorragend klingen, aber völlig nutzlos werden lässt.

### 📝 Wie die Eingabe erfolgt

1. Öffnen Sie **`🤖 Künstliche Intelligenz`** → **`🧭 Projekt und Anweisungen für die KI`** (auch über die Schaltfläche im rechten Seitenpanel oder nach Bearbeitung unter `✏️ Bearbeiten` erreichbar).
2. Füllen Sie die drei Felder aus. Keines ist zwingend vorgeschrieben, aber je konkreter, desto besser:
   - **Vision:** Was Sie inhaltlich abbilden möchten. In ein oder zwei prägnanten Sätzen, in Ihren eigenen Worten.
   - **Ziele:** Welche greifbaren Ergebnisse Sie anstreben. „Meilensteine und Liefergegenstände“, keine vagen Wünsche.
   - **Kontext / Zielgruppe:** Wer das Ergebnis nutzt oder in welchem technischen Umfeld es funktionieren muss.

**Ein Ratschlag:** Schreiben Sie so, als würden Sie das Vorhaben jemandem erklären,
der gerade neu ins Team kommt.
Genau das ist die KI bei jedem Öffnen Ihrer Map: ein neues Teammitglied ohne Gedächtnis an vorangegangene Dialoge.

### 📦 Was genau an die KI übermittelt wird

Hier entstehen leicht Missverständnisse. **Es werden keineswegs nur diese drei Eingabefelder übertragen.**
Der manuelle Export erzeugt ein vollständiges Erläuterungsdokument mit Präambel und zehn strukturierten Abschnitten:

1. **Präambel:** Grundlegende Arbeits- und Systemanweisungen für das Modell.
2. **Ihre Vision und Ihre Ziele:** Die Inhalte, die Sie in diesem Dialogfenster erfassen.
3. **Übersicht und Metriken:** Anzahl der Knoten, vom Kern ausgehende Hauptzweige,
   Querverbindungen, verwendete Tags und Statusverteilung aller Knoten.
4. **Die vollständige Struktur** der Map mit sämtlichen Notizen, Tags und Dateipfaden,
   sofern Sie solche hinterlegt haben.
5. **Ihre Korrekturen:** Knoten, bei denen Sie Änderungen eingefordert oder Freigaben erteilt haben.
6. **Die Matrix der Querverbindungen** zwischen verschiedenen Zweigen.
7. **Offene Fragen und Entscheidungsbedarfe**, die Sie auf der Map markiert haben.
8. **Ein Diagramm** der Map im Mermaid-Format.
9. **Prompt-Vorschläge:** Ausformulierte Anfragen, die Sie bei Bedarf direkt übernehmen können.
10. **Rückmelde-Protokoll:** Der Vertrag mit den sechs MCP-Werkzeugen, das strikte Verbot
    für KIs, Freigaben eigenmächtig zu erteilen, und die exakten Vorgabewerte.

Die Abschnitte 5, 6 und 7 erscheinen nur, wenn dort konkrete Inhalte vorliegen. Die übrigen Abschnitte
sind stets enthalten. Die drei Prompt-Vorschläge aus Abschnitt 9 sind Wahloptionen für Sie: Der Agent
erhält niemals alle drei Anweisungen gleichzeitig.

Das bedeutet: Die gesamte Map reist mit, keine verkürzte Zusammenfassung. Wenn Sie eine ausführliche Notiz
an einem Knoten hinterlegt haben, wird die KI diese vollständig lesen.

### 🧩 Die vier Reiter des Dialogfensters

Für den Einstieg genügt der Reiter **Projekt**.
Dort finden sich Vision, Ziele, Zielgruppe und Autor.
Die übrigen Reiter sind optional und dienen der feineren Steuerung einzelner Sitzungen:

- **Anweisungen** verwaltet gemeinsame Regeln für Claude, Codex und Gemini. Sie können die
  Dokumentsprache unabhängig von der Sprache der Benutzeroberfläche festlegen, eine empfohlene Vorlage
  wiederherstellen oder eigene Regeln verfassen. Ihre Eingaben bleiben wortgetreu erhalten: MMCelt übersetzt nichts heimlich.
- **Auftragsvorlagen** bietet drei strukturierte Startpunkte. Ein Klick kopiert genau diese eine Vorlage
  in den bearbeitbaren Auftrag; es werden niemals alle drei Vorlagen gemeinsam gesendet. Der Auftrag bleibt
  beim Schließen des Fensters erhalten und erscheint wieder, sobald Sie den Agenten auswählen.
- **Vollständige Vorschau** zeigt das exakte Gesamtergebnis aller vier Textblöcke. Vertrag und Kontext
  sind schreibgeschützt; Regeln und Auftrag sind die beiden Blöcke, die Sie frei bestimmen.

Falls `AGENTS.md`, `CLAUDE.md` oder `GEMINI.md` im Projektordner existieren, werden sie als separate Quellen aufgeführt.
Eine **Neue Quelle** wird niemals ungeprüft eingebunden. Klicken Sie erst nach Durchsicht auf **Diese Version akzeptieren**.
Wird die Datei später verändert, erscheint sie als **Geänderte Quelle; erfordert Überprüfung**, verliert ihre
Auswahl und erlaubt den Vergleich zwischen **Zuvor akzeptierte Version** und **Aktuelle Version**.
Sind keine Dateien vorhanden, weist der Reiter den genauen Ordnerpfad für die manuelle Erstellung aus.

### ✍️ Prüfung und Korrektur des Auftrags vor dem Versand

Dieser Arbeitsschritt wird oft übersehen, gehört jedoch zu den wertvollsten Schutzfunktionen des Programms.

Wenn Sie **`🤖 Künstliche Intelligenz`** → **`📤 Senden an...`** aufrufen und einen Agenten wählen, **wird noch
nichts an den Agenten gesendet**. Es öffnet sich eine Vorschau mit dem vollständigen Text in vier Blöcken:

| Block | Editierbar? | Bedeutung |
|---|---|---|
| **MMCelt-Vertrag (geschützt)** | Nein | Die Mindestregeln, identisch für alle Agenten |
| **Gemeinsame Projektregeln** | **Ja** | Ihre dauerhafte Arbeitsweise für jede Sitzung |
| **Mindmap-Kontext (erzeugt)** | Nein | Aus der Map generiert; Änderung erfordert Map-Anpassung |
| **Auftrag dieser Sitzung** | **Ja** | Das konkrete Ziel, das Sie **jetzt** erreichen möchten |

Widersprechen sich zwei Anweisungen, bestimmt das Dokument selbst die Rangfolge,
von stark nach schwach: **Ihre Korrekturen auf der Map**, der MMCelt-Vertrag,
die Projektregeln und zuletzt der Auftrag der Sitzung.

Darunter befinden sich vier Aktionsschaltflächen. `Diese Sitzung zurücksetzen` verwirft Anpassungen.
`Als gemeinsame Regeln speichern` ist die **einzige** Aktion, die Ihr Profil
dauerhaft in `.mmcelt/instrucciones-agente.md` festhält.
`Abbrechen` schließt den Vorgang rückstandsfrei. Die letzte Schaltfläche startet den Agenten.

**Lesen Sie die Vorschau mindestens einmal.** Ändern sich Map, Regeln oder eine Quelldatei nach dem Öffnen der
Bestätigung, stuft MMCelt diese als abgelaufen ein und verlangt eine erneute Prüfung. Dadurch erhalten `inicio.md`
und der Agent exakt dieselben Daten, die Sie auf dem Bildschirm geprüft haben.

### 🤔 Wenn Sie bisher noch nicht so gearbeitet haben

Sie müssen nicht am ersten Tag alle drei Felder ausfüllen. Eine Map ohne hinterlegte Vision lässt sich problemlos exportieren.
Doch der Unterschied zwischen einer KI, die lediglich Bekanntes wiedergibt, und einer KI, die substanziellen Mehrwert liefert,
entscheidet sich fast immer an dieser Stelle und selten am gewählten Sprachmodell.

Beginnen Sie mit der **Vision**, selbst wenn es nur zwei Sätze sind. Es ist das wirkungsvollste Einzelfeld überhaupt.
