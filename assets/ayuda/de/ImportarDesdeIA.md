# 📥 Bidirektionaler Import aus der KI

Ermöglicht es Ihnen, beliebige von ChatGPT, Claude oder Gemini erstellte Projekte oder Spezifikationen in MMCelt zu importieren.

### 🔄 Anleitung:
1. Öffnen Sie in MMCelt **`🤖 Künstliche Intelligenz` > `📋 Master-Prompt für KI kopieren...`**.
2. Fügen Sie diesen Prompt zusammen mit Ihren Ideen, Code oder Dokumenten in den KI-Chat ein.
3. Die KI liefert einen strukturierten JSON-Block zurück.
4. Öffnen Sie in MMCelt **`🤖 Künstliche Intelligenz` > `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...`**.
5. Fügen Sie den JSON-Block ein und klicken Sie auf **`✨ Mindmap durch KI-Ausgabe ersetzen`**.
6. MMCelt rekonstruiert die gesamte Map mit Notizen, Tags, Statuswerten, Codepfaden und automatischem Layout.

### ⚠️ Importieren ersetzt die bestehende Map
Die aktuell geöffnete Map **wird vollständig ersetzt**: Die KI-Daten treten an ihre Stelle und werden nicht mit bestehenden Knoten zusammengeführt.

Vor dem Ersetzen speichert MMCelt ein Backup der bisherigen Map. Falls Sie den Import rückgängig machen möchten, starten Sie das Programm neu: Beim Start wird Ihnen die Wiederherstellung angeboten.

Dennoch empfiehlt es sich, **vor dem Importieren stets mit `Strg + S` zu speichern**. Das automatische Backup ist ein Sicherheitsnetz, kein Ersatz für manuelles Speichern.

### 🎨 Was nicht im JSON übertragen wird
Die KI liefert Struktur und Inhalt, keine Darstellungsinformationen. Manuell verschobene Positionen und eingeklappte Zweige werden beim Import zurückgesetzt: Die Map wird automatisch neu angeordnet.

Alle für die Entwicklung wesentlichen Daten bleiben vollständig erhalten: Titel, Notizen, Dateipfade, Tags, Rollen, Statuswerte, Prioritäten, Querverbindungen und Korrekturanweisungen.
