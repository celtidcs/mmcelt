# 📁 Quellcode-Dateizuordnung (file_path)

Verbinden Sie die Ideen und Entscheidungen Ihrer Mindmap direkt mit den realen Projektdateien auf dem Datenträger.

### 🛠️ Wie man eine Datei verknüpft und wozu das dient:

Eine Mindmap hilft Ihnen beim strategischen Denken: Sie können einen Knoten namens „Benutzer-Authentifizierung“ und einen weiteren namens „Datenbank“ haben. Ihr Computer und Ihre Programme bestehen jedoch aus konkreten Dateien (wie `src/auth.rs` oder `config/database.json`).

Das Dateifeld (`file_path`) bildet die Brücke zwischen der abstrakten Idee und der tatsächlichen Datei, in der ihr Quellcode oder ihre Dokumentation liegt. Damit ist die Mindmap nicht nur eine Zeichnung, sondern ein lebendiger Index Ihres Projekts.

**Schritte zur Dateiverknüpfung:**
1. Klicken Sie auf einen beliebigen Knoten Ihrer Mindmap, um ihn auszuwählen.
2. Schauen Sie in das rechte Seitenpanel (den Knoten-Inspektor).
3. Tragen Sie in das Feld **📁 Datei/Pfad:** den relativen Pfad bezüglich Ihres Projektordners ein. Zum Beispiel:
   - `src/login.rs` für eine Quellcodedatei.
   - `documentacion/requisitos.md` für erklärenden Text.
   - `frontend/komponenten/`, um einen ganzen Ordner anzugeben.

### 💡 Praktische Tipps und Einsatz mit der KI:

**🤖 Wie nutzt die künstliche Intelligenz diese Angabe?**
Wenn Sie mit einem Assistenten arbeiten (wie Claude Code, Codex CLI oder Gemini CLI) oder die Projektzusammenfassung als Markdown exportieren:
- Die KI liest exakt, welche Datei zu welchem Knoten gehört.
- Sie weiß im Voraus, wo Änderungen vorzunehmen sind, ohne blind Ihr gesamtes Repository durchsuchen zu müssen.
- Sie unterstützt Sie dabei, eine saubere und geordnete Übereinstimmung zwischen konzeptioneller Architektur und Quellcode zu wahren.

**Praktische Tipps:**
- **Verwenden Sie relative Pfade:** Geben Sie Pfade stets ausgehend vom Wurzelverzeichnis Ihres Projekts an (z. B. `src/main.rs` anstelle von `C:\MeineDokumente\Projekt\src\main.rs`). Wenn Sie das Projekt verschieben oder teilen, bleiben alle Verknüpfungen intakt.
- **Ordnerpfade:** Wenn ein Knoten mehrere Dateien zusammenfasst, lassen Sie den Pfad mit einem Schrägstrich enden (wie `src/dienste/`), um zu kennzeichnen, dass er das gesamte Verzeichnis repräsentiert.
