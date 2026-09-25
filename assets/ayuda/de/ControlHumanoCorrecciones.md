# 🛑 Benutzerkontrolle und Korrekturanweisungen (Human-in-the-Loop)

Macht MMCelt zu Ihrer **Überwachungs- und Veto-Konsole** gegenüber KI-Vorschlägen.

### 🛡️ Der Korrektur-Workflow:
1. Die KI liefert eine Mindmap oder einen Entwicklungsplan.
2. Falls Sie Architekturfehler oder unerwünschte Bibliotheken entdecken:
   - Setzen Sie den Knoten auf **`⛔ Verworfen`** oder **`⚠️ Korrektur erforderlich`**.
   - Schreiben Sie in das Feld **„🛑 Korrektur / Geforderte KI-Anweisung“** Ihre verbindliche Anweisung (z. B. *„Keine externen Bibliotheken nutzen; mit Rust-Standardbibliothek implementieren“*).
3. Öffnen Sie das Menü **`🤖 Künstliche Intelligenz` > `🛑 Korrekturen und Anweisungen an die KI senden...`**.
4. Klicken Sie auf **`📋 Korrekturanweisungen kopieren`** und fügen Sie diese in den KI-Chat ein (oder lassen Sie Claude Code `mmcelt_get_human_feedback` aufrufen).
5. Die KI liest die Veto-Vorgaben und passt ihren Code umgehend an.

### ✅ Knoten freigeben und automatische Gültigkeit
Wenn Sie einen Knoten für gut befinden, weisen Sie ihm im Inspektor **`🛡️ Menschlich freigegeben`** zu. Dies gilt als Ihre formale Signatur. Die Kontrollen für menschliche Prüfung, Priorität und Status zeigen einheitliche Symbole im Menü und direkt auf dem Knoten der Leinwand.

**Diese Freigabe erlischt automatisch bei Änderungen.** Ändert die KI später diesen Knoten — seine Notizen, seinen Status, die Priorität oder den Dateipfad —, wechselt der Status automatisch zu **`⏳ Ausstehende Prüfung`**.

Der Grund ist einleuchtend: Sie haben einen spezifischen Stand freigegeben, nicht den Knoten auf Dauer. Ohne dieses Erlöschen stünde Ihre Freigabe auf Inhalten, die Sie nie geprüft haben.

Wird derselbe Wert unverändert erneut übertragen, gilt dies nicht als Änderung; routinemäßige KI-Synchronisationen ohne inhaltliche Anpassung erfordern **keine** erneute Prüfung bereits genehmigter Knoten.

Der Text im Korrekturfeld **wird niemals automatisch gelöscht**, selbst wenn die KI die Anweisung befolgt hat: So können Sie jederzeit nachvollziehen, ob Ihre Vorgabe korrekt umgesetzt wurde.
