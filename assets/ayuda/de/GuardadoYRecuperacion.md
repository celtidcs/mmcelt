# 💾 Speichern und Wiederherstellung

### 💾 Manuelles Speichern (Strg + S)
Speichert am von Ihnen gewählten Ort. Falls ein Fehler auftritt — fehlende Berechtigungen, voller Datenträger, nicht synchronisierter Cloudordner —, **informiert die Anwendung Sie sofort**. Sie meldet niemals einen erfolgreichen Speichervorgang ohne vorherige Prüfung.

Bei der Ordnerauswahl öffnet sich der Dialog am wahrscheinlichsten Ort: im Verzeichnis der aktuellen Datei oder **im Verzeichnis des Projekts, auf das sich die Dateipfade der Knoten beziehen**.

### 🛟 Automatisches Backup
Alle zwei Minuten wird bei Änderungen ein Sicherheits-Backup geschrieben. Schließt sich das Programm unerwartet, wird Ihnen beim nächsten Start die Wiederherstellung angeboten.

Dieses Intervall **lässt sich ändern oder ganz abschalten**, unter `🎨 Ansicht und Layout → 💾 Automatisches Speichern`: aus, jede Minute, alle zwei, fünf oder zehn Minuten. Die Wahl bleibt über Sitzungen hinweg erhalten.

**Ersetzt nicht das manuelle Speichern.** Es ist ein Sicherheitsnetz, kein dauerhafter Ablageort. Nutzen Sie weiterhin Strg + S.

Die Kopie wird im Datenordner von MMCelt abgelegt (`%APPDATA%\MMCelt\recuperacion\` unter Windows), **niemals in Ihren Projektordnern**: So entstehen keine unkontrollierten Dateien im Projektverzeichnis. Es wird nur die jeweils letzte Sitzung aufbewahrt und nach einem regulären Speichern gelöscht.

In automatischen Layout-Modi zählt das Verschieben von Knoten nicht als Änderung: Die Engine positioniert sie ohnehin neu, wodurch unnötige Schreibzugriffe beim reinen Betrachten vermieden werden.

Im Modus **Manuelle freie Position** zählt das Verschieben hingegen als Änderung, da die manuelle Anordnung Ihre bewusste Arbeit darstellt.

### ⚠️ Wiederherstellungsangebot beim Start
Der Dialog zeigt Erstellungszeitpunkt und Map-Titel des Backups an. Sie entscheiden: Wurde das Programm bewusst ohne Speichern geschlossen, verwerfen Sie die Kopie. Eine automatische Wiederherstellung ohne Bestätigung erfolgt nicht.

### 🛡️ Beschädigte Dateien
Dateien mit ungültigen Strukturen — zyklische Abhängigkeiten, fehlender Hauptknoten, fehlerhafte Referenzen — werden beim Öffnen unter Angabe des betroffenen Knotens abgewiesen. Dies verhindert Programmabstürze. Wurde die Datei von einer KI erzeugt, fordern Sie eine Neugenerierung an; liegt eine `.bak`-Datei vor, versuchen Sie diese zu öffnen.

### 🌐 Offene Austauschformate (OPML und FreeMind .mm)
Neben dem nativen Format `.mmcelt` können Sie Mindmaps in zwei offenen, nicht-proprietären Standards exportieren und importieren: OPML (hierarchische Gliederungen) und `.mm` (FreeMind und Freeplane).

Beim Export nach OPML oder `.mm` werden Knotentitel, Baumhierarchie und Fließtextnotizen in den Standard-Tags des Formats gespeichert und können von externen Anwendungen gelesen werden. Die spezifischen MMCelt-Attribute, die diese klassischen Formate nicht nativ unterstützen (Status, Priorität, Rolle, menschliche Überprüfung, Codepfade und Querverbindungen), werden als strukturiertes JSON am Ende jeder Notiz serialisiert.

Beim Reimport dieser Datei in MMCelt wird die Map zu 100 % exakt wiederhergestellt, einschließlich aller Metadaten und Querverbindungen. Wird die Datei in einem Drittanbieter-Werkzeug geöffnet, bleibt die Gliederung sichtbar und die Metadaten lassen sich als Informationstext in der Notiz einsehen.

Wird eine externe Datei ohne MMCelt-Metadaten importiert, weist das Programm sichere Standardwerte zu (Status Idee, Priorität Mittel, Rolle Unterthema, außer bei der Wurzel, die Zentrale Idee erhält), wendet automatisch ein ausgewogenes visuelles Layout an, damit sich die Knoten nicht in der Mitte überlagern, und zeigt einen klaren Hinweisdialog an, der die initialisierten Felder transparent erläutert.
