# 🟢 Integration mit Codex CLI und ChatGPT

Ob Sie die Konsole nutzen oder den Webbrowser bevorzugen: Sie können die künstliche Intelligenz von OpenAI nahtlos mit Ihren Mindmaps in MMCelt verbinden.

### 💻 Codex CLI (im Terminal)

Codex CLI ist ein Konsolen-Agent, der direkt auf Ihrem Rechner läuft. Dies ist die schnellste und am stärksten automatisierte Arbeitsweise:

1. Rufen Sie das Menü `🤖 Künstliche Intelligenz` → `📤 Senden an...` → `Codex CLI` auf.
2. Sie sehen eine vollständige Vorschau mit dem Inhalt Ihrer Mindmap und dem Auftrag für die KI. Lesen Sie diesen in Ruhe durch.
3. Sobald Sie auf **In Konsole starten** klicken, öffnet MMCelt automatisch das Terminal mit der vorbereiteten Umgebung und überwacht Änderungen, um von Codex erstellte Knoten zu übernehmen.
4. Codex CLI liest die Map und die Anweisungen über den MCP-Server. Während es Ihr Projekt analysiert, kann es Zweige anlegen, Vorschläge machen und die Map live aktualisieren.

> 💡 **Hinweis:** Damit Codex CLI mit MMCelt kommunizieren kann, verbinden Sie es zuvor über `🤖 Künstliche Intelligenz` → `🔌 MMCelt mit meinen KIs verbinden...` und starten Sie das Terminal neu, falls es bereits geöffnet war.

### 🌐 ChatGPT im Web oder ein Custom GPT

Wenn Sie die technische Konsole nicht nutzen und lieber im Browser chatten (mit ChatGPT Free, Plus oder einem eigenen Custom GPT):

1. Exportieren Sie Ihre Mindmap über `📁 Datei` → `🤖 Markdown für die KI exportieren (.md)` (oder betrachten Sie den Text auf dem Bildschirm über `🤖 Künstliche Intelligenz` → `👁️ KI-Markdown-Vorschau (.md)...`).
2. Öffnen Sie Ihren Browser, rufen Sie ChatGPT auf und hängen Sie die `.md`-Datei an oder fügen Sie den Text direkt in die Unterhaltung ein.
3. Bitten Sie die KI, die Map zu erweitern, Risiken zu bewerten, Alternativen zu prüfen oder neue Aufgaben zu planen.
4. Sobald ChatGPT antwortet (üblicherweise mit einem strukturierten JSON- oder Markdown-Block), kopieren Sie diesen.
5. Wechseln Sie zurück zu MMCelt und wählen Sie `🤖 Künstliche Intelligenz` → `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...`. Fügen Sie den Text ein und bestätigen Sie: Die neuen Zweige fügen sich in Ihre Mindmap ein, wobei all Ihre bisherige Arbeit unangetastet bleibt.

### 🔒 Wichtige Unterschiede und Datenschutz

Es ist wichtig zu verstehen, wie beide Umgebungen mit Ihren Daten umgehen:
- **Codex CLI (lokal mit MCP):** Läuft auf Ihrem eigenen Rechner und greift über das MCP-Protokoll direkt auf den abgegrenzten Projektordner zu.
- **ChatGPT im Webbrowser:** Läuft auf den Servern von OpenAI und hat keinerlei direkten Zugriff auf Ihre lokalen Dateien oder den MCP-Server. Der Austausch erfolgt vollständig manuell und geschützt durch den Export und Import von Markdown- oder JSON-Text.
