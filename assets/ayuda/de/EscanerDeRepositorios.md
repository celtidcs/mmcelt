# 🔍 Automatischer Quelltext-Ordner-Scanner

Wenn Sie bereits ein Programmierprojekt auf Ihrem Rechner oder einen Ordner mit Quellcode haben, müssen Sie Ihre Mindmap nicht Knoten für Knoten von Grund auf neu erstellen. Der **Code-Scanner** von MMCelt liest die Struktur Ihres Projekts ein und erzeugt innerhalb von Sekunden automatisch eine interaktive visuelle Map.

### 🛠️ So nutzen Sie den Scanner gewinnbringend:

**🧠 Wozu dient das Scannen Ihres Codes?**
Indem Sie einen Dateiordner in eine Mindmap umwandeln, erreichen Sie:
- **Einen Überblick über Ihre Architektur aus der Vogelperspektive:** Verstehen Sie schnell, wie Ihre Module, Bibliotheken und Komponenten organisiert sind, ohne sich in Dutzenden von Unterordnern zu verlieren.
- **Direkte Verknüpfungen zu Ihren Dateien:** Jeder Kasten der Map ist mit seinem realen Pfad auf der Festplatte (`file_path`) verknüpft, sodass sowohl Sie als auch die KIs genau wissen, welche physische Datei welchem Konzept entspricht.
- **Der ideale Ausgangspunkt für die Arbeit mit KI:** Sie können jedes KI-Modell bitten, die gewonnene Struktur zu analysieren, doppelten Code aufzuspüren, Refactorings vorzuschlagen oder zirkuläre Abhängigkeiten zu identifizieren.

**Schritte zur Verwendung des Scanners:**
1. Öffnen Sie in der oberen Leiste das Menü **`📁 Datei`** → **`🔍 Code-Ordner scannen...`**.
2. Es erscheint ein Systemdialog, in dem Sie den Stammordner Ihres Repositorys oder Softwareprojekts auswählen.
3. MMCelt durchsucht den Ordnerbaum absolut sicher:
   - **Filtert technischen Ballast:** Verwirft automatisch umfangreiche Build-Ordner oder Abhängigkeiten, die keinen konzeptionellen Mehrwert bieten (wie `node_modules/`, `target/`, `dist/`, `.git/`, virtuelle Python-Umgebungen usw.).
   - **Baut die Hierarchie auf:** Platziert den Hauptordner in der Mitte und verzweigt die wichtigsten Module, Pakete und Codedateien.
4. Sobald die Map erstellt ist, können Sie die Knoten nach Belieben umordnen, Farben ändern, erklärende Notizen hinzufügen oder Bereiche zur Überprüfung markieren.

**🤖 Nächster Schritt mit der KI:**
Sobald Ihr Code gescannt ist:
- Nutzen Sie `🤖 Künstliche Intelligenz` → `👁️ KI-Markdown-Vorschau (.md)...`, um die architektonische Zusammenfassung zu sehen.
- Senden Sie das Projekt an Ihren bevorzugten Konsolen-Agenten (`📤 Senden an...`) oder exportieren Sie es für einen Web-Chat. Die KI weiß genau, wo jede Datei liegt!
