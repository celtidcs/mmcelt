# 🔵 Integration mit Antigravity CLI (agy)

Google Antigravity ist die moderne Entwicklungsplattform von Google, und **Antigravity CLI (`agy`)** ist das offizielle Befehlszeilenwerkzeug, das die bisherige Gemini CLI ablöst. MMCelt ermöglicht Ihnen die direkte Anbindung an Antigravity CLI, um Ihre Mindmaps zu analysieren, zu erweitern und auszubauen.

### 💻 Verwendung mit Antigravity CLI (`agy`) im Terminal

1. Rufen Sie `🤖 Künstliche Intelligenz` → `🔌 MMCelt mit meinen KIs verbinden...` auf und klicken Sie auf **Verbinden** neben Antigravity CLI (agy).
2. Öffnen Sie Ihre Mindmap und wählen Sie `🤖 Künstliche Intelligenz` → `📤 Senden an...` → `Antigravity CLI (agy)`.
3. Prüfen Sie die Vorschau des Dokuments und des Auftrags, den die KI erhalten wird.
4. Klicken Sie auf **In Konsole starten**. MMCelt öffnet automatisch das Terminal mit `agy -i` in der vorbereiteten Umgebung und überwacht Änderungen, um vom Agenten per MCP erstellte Knoten zu übernehmen.

### 💡 Authentifizierung mit normalem Google-Konto

Antigravity CLI arbeitet nativ mit Ihrem **Standard-Google-Konto**:
- Keine Google AI Studio API-Schlüssel oder zusätzlichen Umgebungsvariablen erforderlich.
- Wenn Sie noch nicht in `agy` angemeldet sind, öffnet das Werkzeug den Browser, damit Sie sich mit Ihrem normalen Google-Konto anmelden können.
- Der MCP-Server von MMCelt wird in `~/.gemini/config/mcp_config.json` registriert, sodass der Agent die Mindmap innerhalb des autorisierten Projektordners lesen und aktualisieren kann.

### 🌐 Gemini im Webbrowser

Wenn Sie das Terminal nicht nutzen möchten:
1. Exportieren Sie Ihre Mindmap über `📁 Datei` → `🤖 Markdown für die KI exportieren (.md)` (oder klicken Sie auf `🤖 Künstliche Intelligenz` → `👁️ KI-Markdown-Vorschau (.md)...`).
2. Öffnen Sie die Gemini-Website in Ihrem Browser, fügen Sie den Text ein und stellen Sie Ihre Anfrage.
3. Kopieren Sie die Antwort von Gemini und wechseln Sie zurück zu MMCelt: Klicken Sie auf `🤖 Künstliche Intelligenz` → `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...`, um die neuen Zweige in Ihre Mindmap zu übernehmen.
