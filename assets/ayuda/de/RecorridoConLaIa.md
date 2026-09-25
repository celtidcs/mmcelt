# 🤖 Der vollständige Ablauf mit der KI

Die anderen Themen erklären jeden Baustein für sich. Dieses Thema schildert **die gesamte Reise**: was Ihren
Rechner verlässt, warum, was Sie entscheiden und was die KI tun kann, wenn sie antwortet.

Wenn Sie nur ein einziges Thema über die KI lesen, lesen Sie dieses.

### 🗺️ Der Ablauf in vier Schritten

**1. Sie bereiten die Map vor.** Nichts von dem, was Sie hier tun, wird bereits gesendet. Sie bauen Zweige auf,
schreiben Notizen, markieren Zustände und offene Fragen, ziehen Querverbindungen und formulieren die Vision des
Projekts. Je mehr von dem, was Sie im Kopf haben, aufgeschrieben ist, desto weniger muss die KI erraten.

**2. Das Programm stellt ein Dokument zusammen.** Es sendet nicht Ihre `.mmcelt`-Datei. Für eine automatische
Sitzung fasst es vier Blöcke immer in derselben Reihenfolge zusammen: MMCelt-Vertrag, Projektregeln, Kontext der
Map und einen einzelnen Auftrag. Die gesamte Map reist innerhalb des Kontexts mit, nicht als Zusammenfassung.

**3. Sie lesen und genehmigen es.** Dies ist der Schritt, den kaum jemand nutzt und der am meisten schützt.
Bevor etwas das System verlässt, sehen Sie den vollständigen Text und können zwei der vier Blöcke bearbeiten.

**4. Die KI gibt Arbeitsergebnisse an die Map zurück.** Nicht indem sie Ihre Datei eigenmächtig ändert, sondern
über sechs reglementierte Werkzeuge. Alles, was sie hinzufügt, entsteht als KI-generiert markiert, und Sie entscheiden, ob Sie es freigeben.

### 📤 Die zwei Wege des Versands

| | Wie | Wann sinnvoll |
|---|---|---|
| **Manuell** | `📁 Datei` → `🤖 Markdown für die KI exportieren (.md)`, und Datei im Chat hochladen | Jedes Modell, auch im Browser. Keine Installation erforderlich |
| **Automatisch** | `🤖 Künstliche Intelligenz` → `📤 Senden an...` und Agenten auswählen | Sie haben den Agenten installiert und möchten automatische Updates |

Die Antwort kommt über `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...` zurück, wenn Sie es manuell gemacht haben,
oder selbstständig, wenn Sie den automatischen Versand genutzt haben.

### ✍️ Was Sie entscheiden und wo

- **Was Sie bezwecken:** `🤖 Künstliche Intelligenz` → `🧭 Projekt und Anweisungen für die KI` → Reiter **Projekt**.
- **Was akzeptiert ist und was nicht:** Markieren Sie Knoten als `🛡️ Menschlich freigegeben` oder
  `⚠️ Korrektur erforderlich`, und senden Sie diese über
  `🤖 Künstliche Intelligenz` → `🛑 Korrekturen und Anweisungen an die KI senden...`.
- **Vor dem Senden prüfen:** Mit `🤖 Künstliche Intelligenz` → `👁️ KI-Markdown-Vorschau (.md)...`
  können Sie den exakten generierten Text am Bildschirm prüfen, bevor er an eine Konsole geht oder in einen Chat kopiert wird.
- **Was diesmal verlangt wird:** In der Versandvorschau der Block „Auftrag dieser Sitzung“.
- **Wie grundsätzlich gearbeitet werden soll:** In derselben Vorschau der Block der gemeinsamen Regeln.

Vor dem Versand können Sie alles auch unter **`🧭 Projekt und Anweisungen für die KI`** vorbereiten.
Der Reiter **Anweisungen** erlaubt die Auswahl der **Sprache des Dokuments für die KI**, das Speichern gemeinsamer
Regeln und die Prüfung nativer Quelldateien. Der Reiter **Auftragsvorlagen** kopiert eine gezielte Vorlage in den
Auftrag, und **Vollständige Vorschau** zeigt den exakten Prompt-String.

Eine native Quelle wird niemals allein deshalb einbezogen, weil sie auf der Festplatte existiert. Zuerst erscheint sie
als **Neue Quelle**; nach Klick auf **Diese Version akzeptieren** bleibt sie vorausgewählt, solange sich ihre Bytes nicht
ändern. Ändert sie sich, wird die Auswahl aufgehoben und die Vorversion neben der aktuellen Version angezeigt. Sie können
über die jeweilige Checkbox entscheiden, ob eine Quelle in die finale Sitzung einfließt.

Widersprechen sich zwei Anweisungen, legt das Dokument selbst die Rangfolge fest, von stark nach schwach:
**Ihre Korrekturen auf der Map**, der MMCelt-Vertrag, die Projektregeln und an letzter Stelle der Auftrag dieser Sitzung.

### 🔙 Was die KI bei der Rückkehr tun darf und was nicht

**Sie darf** die Map lesen, Ihre Korrekturen lesen, Knoten hinzufügen, Fortschritte melden und eine neue
Map anlegen. Es gibt genau sechs Werkzeuge und keines mehr.

**Sie darf nicht** Ihre `.mmcelt`-Datei eigenmächtig modifizieren, den freigegebenen Arbeitsordner verlassen,
noch **behaupten, dass Sie etwas freigegeben haben**. Alles, was sie beisteuert, wird als KI-generiert markiert,
und die Hochstufung zur Freigabe ist immer und ausschließlich Ihre persönliche Entscheidung.

Zudem bleibt eine lückenlose Spur: Der exakte Text, den Sie freigegeben haben, wird in `.mmcelt/sesiones` gespeichert,
und wenn eine automatische Änderung für eine von Ihnen bearbeitete Map eintrifft, wird zuvor ein `.bak`-Backup angelegt.

Die Bestätigung ist keine dekorative Momentaufnahme. Unmittelbar vor dem Start überprüft MMCelt die Map,
die Regeln und die Quelldateien erneut. Hat sich etwas geändert, wird angezeigt, dass die Bestätigung abgelaufen ist,
der Agent wird nicht gestartet und Sie werden um eine erneute Prüfung gebeten.

### ⚠️ Was dies nicht ist

Das Dokument teilt der KI mit, dass der Inhalt Ihrer Map aus **Daten und nicht aus Befehlen** besteht.
Das reduziert Missverständnisse, **ist jedoch keine absolute Sicherheitsbarriere**: Ein von außen importierter
Titel oder eine Notiz könnte so formuliert sein, als handele es sich um eine Instruktion.

Was wirklich schützt, ist etwas anderes: der autorisierte Arbeitsordner, die Begrenzung auf sechs Werkzeuge
und die Tatsache, dass Sie die Vorschau prüfen, bevor Daten das System verlassen.
Deshalb lohnt es sich, diese Vorschau mindestens einmal aufmerksam zu lesen.

**💡 Was tun, wenn keine Konsolen-Agenten installiert sind?**
Wenn Sie unter `🤖 Künstliche Intelligenz` → `📤 Senden an...` den Hinweis „Keine installierten Agenten erkannt“ sehen,
keine Sorge: **Sie benötigen weder technische Terminals noch fortgeschrittene Agenten, um KI mit MMCelt zu nutzen**.

Sie können bequem über Ihren gewohnten Webbrowser arbeiten:
1. Nutzen Sie `📁 Datei` → `🤖 Markdown für die KI exportieren (.md)` (oder prüfen Sie den Text vorab unter `🤖 Künstliche Intelligenz` → `👁️ KI-Markdown-Vorschau (.md)...`).
2. Öffnen Sie in Ihrem Browser einen beliebigen KI-Chatdienst (ChatGPT, Claude, Gemini...).
3. Fügen Sie den exportierten Text ein und geben Sie Ihre Anweisungen zur Fortführung der Map.
4. Sobald die KI mit einem strukturierten Block in JSON- oder Markdown-Format antwortet, kopieren Sie diesen.
5. Klicken Sie in MMCelt auf `🤖 Künstliche Intelligenz` → `📥 Aus KI importieren (ChatGPT, Claude, Gemini)...` und fügen Sie die Antwort ein. Ihre neuen Zweige erscheinen sofort auf der Map.

Falls Sie künftig einen Terminal-Agenten auf Ihrem System einrichten (wie Claude Code, Codex CLI oder Gemini CLI),
erkennt MMCelt diesen automatisch, sobald er in Ihrem Systempfad (PATH) liegt.

### 📚 Wie es weitergeht, je nach Ihrem Ziel

- **Ihre Absichten präzise formulieren:** das Thema zur Vision des Erstellers.
- **Eine abdriftende KI steuern:** das Thema zu menschlicher Kontrolle und Korrekturen.
- **Das generierte Dokument verstehen:** das Thema zum Markdown-Export.
- **Eine kopierte Antwort manuell einbinden:** das Thema zum Import aus der KI.
- **Senden und automatische Rückkehr beobachten:** das Thema zu Versand und Dateiüberwachung.
- **Ihren Agenten per MCP anbinden:** das Thema 🟣 MCP-Server und KI-Agenten.

Alle Themen stehen in diesem Auswahldialog bereit, geordnet nach dem typischen Arbeitsablauf.
