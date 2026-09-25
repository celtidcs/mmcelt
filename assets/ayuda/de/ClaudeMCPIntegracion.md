# 🟣 MCP-Server und KI-Agenten

Eine sichere und direkte Brücke, über die Modelle künstlicher Intelligenz Mindmaps auf Ihrem Rechner lesen und erstellen können.

### ⚡ Einfache Ein-Klick-Verbindung

Sie müssen keine komplizierten Konfigurationsdateien bearbeiten. Verbinden Sie Ihre Agenten direkt aus der Anwendung heraus:

1. Öffnen Sie das Menü `🤖 Künstliche Intelligenz` → `🔌 MMCelt mit meinen KIs verbinden...`.
2. MMCelt sucht automatisch nach installierten Terminal-Assistenten auf Ihrem Rechner (Claude Code, Codex CLI und Gemini CLI).
3. Klicken Sie neben dem gewünschten Assistenten auf die Schaltfläche **Verbinden**. Fertig!

Wenn Sie später einen weiteren Assistenten installieren, öffnen Sie dieses Fenster einfach erneut: Die Erkennung erfolgt bei jedem Aufruf dynamisch. Der Status „Verbunden“ stellt sicher, dass der Assistent weiß, wo MMCelt zu finden ist und wie er über `--mcp-server` mit ihm kommuniziert.

### ⚙️ Manuelle Konfiguration und eingebetteter Server

Um Ihren MCP-Client manuell einzurichten, fügen Sie diesen Eintrag in dessen JSON-Konfiguration ein:
```json
{
  "mcpServers": {
    "mmcelt": {
      "command": "C:/pfad/zu/mmcelt.exe",
      "args": ["--mcp-server"],
      "env": { "MMCELT_WORKSPACE": "C:/pfad/zu/ihrem/projekt" }
    }
  }
}
```

Der MCP-Server ist **direkt in die ausführbare Datei von MMCelt eingebettet**: Er benötigt weder eine externe Laufzeitumgebung noch zusätzliche Abhängigkeiten.

### 🛡️ Höchste Sicherheit: Arbeitsordner und Backups

Ihre Sicherheit steht an erster Stelle. Wenn ein KI-Assistent mit MMCelt arbeitet, **hat er ausschließlich die Berechtigung, innerhalb Ihres Projektordners zu lesen und zu schreiben**. Dieser Bereich wird über die Umgebungsvariable `MMCELT_WORKSPACE` festgelegt.

Jeder Versuch der KI, auf Ordner außerhalb dieses Bereichs zuzugreifen, wird sofort abgewiesen. Zudem sorgen weitere Sicherheitsmechanismen für Schutz:
- **Automatische Sicherungskopien:** Bevor ein Agent eine bestehende Datei ändert, speichert MMCelt ein Backup mit exaktem Zeitstempel (z. B. `projekt.mmcelt.20260918-120000.bak`). Sollte Ihnen etwas nicht zusagen, bleibt Ihr vorheriger Stand vollständig erhalten.
- **Respekt vor Ihren Ideen:** Die KI kann neue Zweige und Verbindungen vorschlagen, wird jedoch **niemals von Ihnen erstellte Knoten löschen oder verschieben**. Alle KI-Vorschläge werden optisch als „KI-generiert“ markiert, sodass immer Sie entscheiden, ob Sie sie genehmigen oder korrigieren.

### 🔨 Die 6 offiziellen Werkzeuge des MCP-Servers

Der Assistent verfügt über sechs offizielle Werkzeuge, die speziell für die Zusammenarbeit mit Ihnen entwickelt wurden:

- `mmcelt_workspace_info`: Fragt ab, welcher Ordner freigegeben ist und welche `.mmcelt`-Maps darin liegen. Dies wird zuerst geprüft, um keine Pfade zu erfinden.
- `mmcelt_create_mindmap`: Erstellt neue Mindmaps im `.mmcelt`-Format.
- `mmcelt_read_mindmap`: Liest die Struktur Ihrer Map, Notizen und offenen Fragen.
- `mmcelt_sync_ai_progress`: Baut die Map aus, indem Knoten mit Rollen, Prioritäten und Beziehungen hinzugefügt werden.
- `mmcelt_get_human_feedback`: Liest Ihre Korrekturen und Anweisungen, bevor sensible Bereiche bearbeitet werden.
- `mmcelt_export_ai_markdown`: Wandelt die Mindmap in eine strukturierte Markdown-Zusammenfassung um.

### 🌱 Wenn Sie die Mindmap selbst erstellt haben: Wahrung Ihrer Urheberschaft

Der Agent kann Ihre Mindmap erweitern, sie jedoch niemals ersetzen. Er fügt Knoten mit Rollen und Schlagwörtern hinzu und zieht Verbindungen zwischen getrennten Zweigen.
Alles Hinzugefügte wird als KI-generiert gekennzeichnet, kann nicht als von Ihnen freigegeben signiert werden und wird niemals einen Pfad reaktivieren, den Sie als verworfen markiert haben.

### 🏷️ Knoten mit identischem Namen

In einem Entscheidungsdiagramm ist es völlig normal, dass sich Begriffe (wie „Ja“, „Nein“ oder „Offen“) in verschiedenen Zweigen wiederholen. Wenn die KI eine Änderung anfordert und dabei nur einen mehrdeutigen Namen nennt, lehnt MMCelt den Befehl ab und liefert die eindeutigen Bezeichner aller Treffer zurück, damit die KI präzisiert, welcher Knoten gemeint war. So werden Verwechslungen zuverlässig verhindert.

### 🧭 Und wenn keine Assistenten im Terminal installiert sind?

Keine Sorge: MMCelt ist auch ohne installierte Terminal-Agenten uneingeschränkt nutzbar. Sie können Webdienste wie ChatGPT, Claude.ai oder Gemini im Browser nutzen, indem Sie die Optionen `🤖 Künstliche Intelligenz` → `📋 Master-Prompt für KI kopieren...` oder `💾 .md-Datei für KI exportieren...` verwenden. Fügen Sie die Antwort anschließend einfach in `🤖 Künstliche Intelligenz` → `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...` ein, um den Text sofort in visuelle Knoten zu verwandeln.
