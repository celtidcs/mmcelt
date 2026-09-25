# 📤 „Senden an...“, Agentenkonsole und Überwachung

„Senden an...“ bereitet eine überwachte Sitzung mit einem Programmieragenten vor und schließt die Schleife. Hat das Ziel eine offizielle Konsole, findet das Gespräch dort statt; spricht es nur MCP, legt MMCelt alles bereit und der Agent holt es sich über seine eigene Oberfläche. In beiden Fällen kehrt die Arbeit über den MCP-Server von MMCelt zur Mindmap zurück.

### 👁️ Einen Agenten auszuwählen sendet noch nichts
Ein Klick auf einen Agenten öffnet eine **Vorschau** des vollständigen Prompts, aufgeteilt in vier Blöcke:

1. **MMCelt-Vertrag** (schreibgeschützt): die Mindestregeln für die Integrität. Den freigegebenen Bereich zuerst erkunden, menschliche Entscheidungen vor dem Schreiben lesen, die `.mmcelt`-Datei niemals von Hand bearbeiten und Fortschritte über MCP zurückmelden. Er wird geschützt angezeigt, damit er nicht versehentlich gelöscht wird.
2. **Gemeinsame Projektregeln** (bearbeitbar): Ihr neutrales Profil, für alle Agenten dasselbe.
3. **Mindmap-Kontext** (schreibgeschützt): vom KI-Export erzeugt. Er wird in der Mindmap geändert, die seine Quelle ist.
4. **Auftrag dieser Sitzung** (bearbeitbar): was jetzt erreicht werden soll. Er wird nicht in die nächste Sitzung übernommen.

`Diese Sitzung wiederherstellen` setzt Regeln und Auftrag auf den Anfangszustand zurück. `Als gemeinsame Regeln speichern` ist eine eigene Aktion: nur sie schreibt das Projektprofil nach `.mmcelt/instrucciones-agente.md`. `Abbrechen` speichert nichts, exportiert nichts, überwacht nichts und hinterlässt keine Sitzungsakte.

### ⚖️ Vorrang, im Prompt selbst erklärt
Menschliche Korrekturen der Mindmap **>** MMCelt-Vertrag **>** Projektregeln **>** Auftrag der Sitzung.

### 📚 Native Anweisungen werden gezeigt, nicht eingefügt
Liegen im Projektstamm `AGENTS.md`, `CLAUDE.md` oder `GEMINI.md`, zählt die Vorschau sie auf und lässt sie lesen, fügt sie aber **nicht** in den gemeinsamen Prompt ein: Jedes Werkzeug findet seine eigene Datei mit eigenem Geltungsbereich, und alle zu kopieren würde doppelte, widersprüchliche Regeln erzeugen.

### 🚀 Was beim Bestätigen geschieht
Die Schaltfläche sagt, was wirklich passiert: `In Konsole starten`, wenn MMCelt die Konsole des Agenten gefunden hat, und `Für MCP vorbereiten`, wenn es keine gibt. Im zweiten Fall weist die Vorschau vorher darauf hin, mit einem hervorgehobenen Hinweis über dem Namen des Agenten.

**Mit Konsole** liegt die Reihenfolge fest, und jeder Schritt muss gelingen, bevor der nächste läuft:

1. **Speichert die Mindmap** in ihrer `.mmcelt`-Datei.
2. **Exportiert das KI-Dokument** neben der Mindmap, mit der Endung `_AI.md`.
3. **Schreibt die Sitzungsakte**.
4. **Öffnet die Konsole** des Agenten im Projektordner.
5. **Startet die Überwachung** der Mindmap.

Öffnet sich die Konsole nicht, sagt MMCelt **nicht „gesendet“**, beginnt keine Überwachung und markiert die Sitzungsakte als fehlgeschlagen.

**Ohne Konsole** —ein Agent, dessen MCP-Server registriert ist, dessen ausführbare Datei MMCelt aber nicht findet— geschieht alles Übrige trotzdem: Die Mindmap wird gespeichert, die `_AI.md` exportiert, die Sitzungsakte geschrieben und die Überwachung gestartet. Nur der vierte Schritt entfällt, und die Akte bleibt im Zustand **vorbereitet** statt gestartet, weil MMCelt nichts gestartet hat. Der Agent holt sich die Arbeit über seine eigene Oberfläche und meldet sie wie jeder andere über MCP zurück.

### 🗄️ Ein Projekt aus einer älteren Version wird beim Ansehen nicht verändert
Ein Ordner, der mit einer früheren Version benutzt wurde, hat noch keine `.mmcelt/configuracion.json`. Die Vorschau zu öffnen **legt sie nicht an**: Der Prompt entsteht im Speicher, auf die Festplatte wird nichts geschrieben. Die Projektidentität wird beim Bestätigen geschrieben, denn das ist die ausdrückliche Geste. Brechen Sie ab, bleibt der Ordner genau so, wie er war.

### 🗂️ Die Sitzungsakte: `.mmcelt/sesiones`
Jede Bestätigung legt einen Ordner mit Datum, Agent und Kennung an. Darin liegen zwei Dateien:

- `inicio.md`: der genaue Prompt, den Sie freigegeben haben, Zeichen für Zeichen.
- `sesion.json`: Formatversion, Kennung, UTC-Datum, Agent, Pfad der Mindmap, erkannte und übernommene Quellen, Prompt-Größe und Zustand (vorbereitet, gestartet oder fehlgeschlagen).

Es werden keine Kennwörter, Schlüssel, E-Mail-Adressen oder Kontokennungen geschrieben. Es ist Ihr lokales Material.

### 🔁 Eine Sitzung wiederholen
Öffnen Sie die Konsole des Agenten im Projektordner und bitten Sie ihn, die `inicio.md` dieses Ordners zu lesen. Er bekommt genau dasselbe wie beim ersten Mal, ohne Zwischenablage und ohne dass Sie sich an Ihren Text erinnern müssen. Nützlich, um einen Fehler nachzustellen und um zwei Agenten am selben Auftrag zu vergleichen.

### 🔑 Die Konten liefert das CLI, nicht MMCelt
MMCelt fragt keine Zugangsdaten ab, speichert und verändert keine. Es stützt sich auf die Sitzung, die in `claude`, `codex` oder `gemini` bereits angemeldet ist. Es nutzt keine kostenpflichtige API, wählt kein Modell und entscheidet nichts über die Abrechnung. Die Authentifizierung bleibt vollständig beim offiziellen Werkzeug.

### 🔗 MCP und Konsole sind zwei getrennte Fähigkeiten
Das Menü „Senden an...“ kennzeichnet jeden Agenten mit einem Symbol:

- ✨ **Konsole und MCP**: kann sprechen und Arbeit an die Mindmap zurückgeben.
- 🖥 **Nur Konsole**: MMCelt findet die ausführbare Datei, aber MCP ist nicht eingetragen.
- 🔗 **Nur MCP**: kann zurückmelden, aber MMCelt kann keine Konsole öffnen.
- 🔌 **Keines von beidem**.

Dass ein Agent als über MCP verbunden erscheint, **heißt nicht**, dass sich für ihn eine Konsole öffnen lässt.

### 🌌 Offizielle Konsolen-Agenten (CLI)
MMCelt unterstützt ausschließlich die drei offiziellen Terminal-Clients: **Claude Code**, **Codex CLI** und **Gemini CLI** (letzterer mit Kontohinweis). Grafische Clients wurden entfernt, um sicherzustellen, dass nur überprüfte und einfach zu bedienende Optionen angeboten werden.

### 💻 Wo es geprüft ist und wo es nur umgesetzt ist
- **Windows**: die Plattform mit geprüftem Ablauf. MMCelt öffnet ein neues Konsolenfenster direkt auf der ausführbaren Datei des Agenten. Beim Suchen gilt `PATHEXT`: Hat eine npm-Installation ein Unix-Skript ohne Endung neben den `.cmd`-Starter gelegt, wird der Starter gewählt, denn nur ihn kann Windows ausführen.
- **Linux**: umgesetzt, aber **noch ohne geprüften echten Ablauf**. Es nutzt das erste bekannte Terminal, das es findet, unter `x-terminal-emulator`, `gnome-terminal`, `konsole` und `xfce4-terminal`. Gibt es keines, meldet es das und startet nichts.
- **macOS**: wird noch **nicht** zugesichert. Das Projekt hat dort keinen erprobten Ablauf und verspricht lieber nichts.

### 🔄 Automatisches Neuladen in der Regelschleife
Aktualisiert ein Agent die Mindmap auf der Festplatte über `mmcelt_sync_ai_progress`, erkennt MMCelt das sofort und die Zeichenfläche lädt von selbst neu, mit den neuen Knoten, Zuständen und Prioritäten.

### 🛡️ Schutz ungespeicherter lokaler Änderungen
Bearbeiten Sie die Mindmap und haben offene Änderungen, pausiert das automatische Neuladen, damit Ihre Arbeit nicht überschrieben wird. Sobald Sie mit `Ctrl + S` speichern, läuft es weiter.

### 🛑 Überwachung und Konsole beenden
Die Überwachung endet sauber über `Verfolgung beenden` im Menü für künstliche Intelligenz, beim Anlegen einer neuen Mindmap mit `Ctrl + N`, beim Öffnen einer anderen Datei über das Menü `Datei` oder beim Schließen der Anwendung.

Die Konsole des Agenten ist ein **eigener Prozess**: Sie schließen sie in ihrem eigenen Fenster. Die Überwachung zu beenden schließt die Konsole nicht, und die Konsole zu schließen löscht die Sitzungsakte nicht.
