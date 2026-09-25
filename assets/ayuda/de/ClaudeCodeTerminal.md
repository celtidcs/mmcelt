# 🟣 Claude Code: MMCelt öffnet die Konsole

Claude Code ist der offizielle Terminal-Client von Anthropic, der für das Programmieren und die direkte Arbeit mit den Dateien auf Ihrem Rechner entwickelt wurde.

### Wie wird er zum ersten Mal verbunden?

1. Öffnen Sie Ihre Mindmap in MMCelt.
2. Rufen Sie das Menü `🤖 Künstliche Intelligenz` → `🔌 MMCelt mit meinen KIs verbinden...` auf.
3. Prüfen Sie, ob der Projektordner korrekt ist, und klicken Sie in der Zeile von Claude Code auf die Schaltfläche **Verbinden**.
4. Falls eine Claude-Code-Konsole geöffnet war, schließen Sie diese und starten Sie sie neu, damit sie die neue Verbindung zu MMCelt erkennt.

### Wie läuft die tägliche Arbeit ab?

1. Wählen Sie im Menü von MMCelt `🤖 Künstliche Intelligenz` → `📤 Senden an...` und dort **Claude Code** aus.
2. Es öffnet sich ein Fenster, in dem Sie die Anweisungen und den Kontext überprüfen können, die an den Assistenten gesendet werden.
3. Klicken Sie auf die Schaltfläche **In Konsole starten**. Sie müssen kein Terminal manuell öffnen: MMCelt öffnet automatisch eine neue Konsole im Projektordner, erstellt die Arbeitsakte und startet die Überwachung im Hintergrund.
4. Setzen Sie die Unterhaltung mit der KI in diesem Terminalfenster fort. Wenn der Assistent Ideen auf der Map vorschlägt oder ausbaut, nutzt er das Werkzeug `mmcelt_sync_ai_progress`, und Sie sehen die neuen Knoten direkt auf Ihrem Bildschirm erscheinen.

MMCelt nutzt die in Claude Code bereits eingerichtete Sitzung; es verlangt keine zusätzlichen API-Schlüssel und verwaltet nicht Ihre Anthropic-Abrechnung.
