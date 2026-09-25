# 🔵 Integration mit Gemini CLI

Gemini ist das KI-Modell von Google, und **Gemini CLI** ist das offizielle Befehlszeilenwerkzeug. MMCelt ermöglicht Ihnen die direkte Anbindung an Gemini CLI, um Ihre Mindmaps zu analysieren, zu erweitern und auszubauen.

### 💻 Verwendung mit Gemini CLI im Terminal

1. Rufen Sie `🤖 Künstliche Intelligenz` → `🔌 MMCelt mit meinen KIs verbinden...` auf und verbinden Sie Gemini CLI.
2. Öffnen Sie Ihre Mindmap und wählen Sie `🤖 Künstliche Intelligenz` → `📤 Senden an...` → `Gemini CLI`.
3. Prüfen Sie die Vorschau des Dokuments und des Auftrags, den die KI erhalten wird.
4. Klicken Sie auf **In Konsole starten**. MMCelt öffnet automatisch das Terminal mit der vorbereiteten Umgebung und überwacht Änderungen, um von Gemini erstellte Knoten zu übernehmen.

### 💡 Tipps und Tricks: Kostenloser API-Schlüssel und Kompatibilität

Falls Gemini CLI beim Start nach Authentifizierung verlangt oder Probleme bei der Browser-Anmeldung auftreten, ist die Nutzung eines **kostenlosen API-Schlüssels** von Google AI Studio der schnellste und zuverlässigste Weg:

1. Melden Sie sich bei Google AI Studio (`https://aistudio.google.com/app/apikey`) mit Ihrem Google-Konto an und klicken Sie auf **Create API Key**. Dies ist kostenlos.
2. Kopieren Sie den erzeugten Schlüssel.
3. Setzen Sie die Umgebungsvariable vor dem Starten der Konsole oder im Benutzerprofil:
   - **Unter Windows (PowerShell):**
     ```powershell
     $env:GEMINI_API_KEY="ihr_schluessel_hier"
     ```
   - **Unter Linux oder macOS (Bash/Zsh):**
     ```bash
     export GEMINI_API_KEY="ihr_schluessel_hier"
     ```
4. Mit dieser Variablen arbeitet Gemini CLI sofort ohne weitere Anmeldeaufforderungen im Browser.
5. Die offizielle Dokumentation von Gemini CLI finden Sie im Repository: `https://github.com/google-gemini/gemini-cli`.

**ℹ️ Aktueller Status der Kontokompatibilität in Gemini CLI:**
Bestätigt: Seit dem 18. Juni 2026 hat Google die interaktive Anmeldung „Sign in with Google“ für persönliche Konten in Gemini CLI entfernt, einschließlich kostenloser, Google AI Pro- und Ultra-Konten.
- **13. September 2026:** erster Test auf dem Rechner des Direktors, mit der wortgetreuen Meldung: *„This client is no longer supported for Gemini Code Assist for individuals. To continue using Gemini, please migrate to the Antigravity suite of products“*.
- **Anschließend mit mehreren älteren Versionen von Gemini CLI wiederholt**, um ein Problem der installierten Version auszuschließen: Das Ergebnis war bei allen gleich.
- **Bestätigt mit einem kostenlosen Google AI Studio API-Schlüssel**: Die Verbindung funktionierte durchgängig; Knoten und Verbindungen wurden auf dem Datenträger verifiziert, ohne jede Einschränkung für persönliche Konten.

Die oben beschriebene Methode mit dem API-Schlüssel **ist der einzige bestätigte Weg** für persönliche Konten: Die interaktive Anmeldung steht ihnen nicht mehr zur Verfügung.

### 🌐 Gemini im Webbrowser

Wenn Sie das Terminal nicht nutzen möchten:
1. Exportieren Sie Ihre Mindmap über `📁 Datei` → `🤖 Markdown für die KI exportieren (.md)` (oder klicken Sie auf `🤖 Künstliche Intelligenz` → `👁️ KI-Markdown-Vorschau (.md)...`).
2. Öffnen Sie die Gemini-Website in Ihrem Browser, fügen Sie den Text ein und stellen Sie Ihre Anfrage.
3. Kopieren Sie die Antwort von Gemini und wechseln Sie zurück zu MMCelt: Klicken Sie auf `🤖 Künstliche Intelligenz` → `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...`, um die neuen Zweige in Ihre Mindmap zu übernehmen.
