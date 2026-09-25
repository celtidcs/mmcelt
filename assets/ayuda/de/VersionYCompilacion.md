# ℹ️ Version und Build

### 👀 Anzeigeorte
Die Versionsnummer erscheint in der **Titelleiste des Fensters** und in der **Statusleiste** zusammen mit dem Build-Datum.

Vollständige Details finden Sie unter **❓ Hilfe → ℹ️ Über MMCelt** oder durch Klick auf die Version in der Statusleiste. Angezeigt werden Version, Datum, Commit, Branch und — besonders hilfreich — **der exakte Pfad der aktuell ausgeführten Datei**.

### 💻 Über das Terminal
```
mmcelt --version
```

Hilfreich, wenn mehrere Binärdateien auf dem Datenträger existieren und Sie ohne Starten der GUI prüfen möchten, welche Version vorliegt.

### 🤔 Warum reicht die Versionsnummer allein nicht aus?
Die Versionsnummer ändert sich bei wiederholten Builds nicht zwangsläufig. Sie gibt die Zielversion an, nicht **welche konkrete Datei** gestartet wurde.

Dies ist entscheidend, wenn mehrere Kopien existieren: im Projektordner, portabel auf einem USB-Stick oder in Testverzeichnissen. Datum, Commit und Pfad ermöglichen die zweifelsfreie Unterscheidung.

### ⚠️ Hinweis auf „nicht gespeicherte Änderungen“
Erscheint dieser Hinweis, wurde das Programm mit Änderungen kompiliert, die noch nicht im Git-Repository committet waren: Diese Binärdatei entspricht keinem sauberen Git-Stand.
