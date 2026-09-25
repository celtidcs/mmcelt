# ✨ Architekturvorlagen für Software

Ein Softwareprojekt auf einem leeren Blatt zu beginnen, kann überwältigend sein. Deshalb enthält MMCelt **vorkonfigurierte Architekturvorlagen**, die auf bewährten Praktiken des modernen Software-Engineerings basieren und mit einem Klick einsatzbereit sind.

Diese Vorlagen sind keine bloßen Skizzen: Sie sind bereits in logische Schichten gegliedert, mit semantischen Farben, Beziehungstypen zwischen Komponenten und erklärenden Notizen versehen, die sowohl Ihren Entwurf als auch die Antworten der KI leiten.

### 🏛️ In MMCelt verfügbare Vorlagen

**1. Clean Architecture (Saubere & hexagonale Architektur):**
Diese Vorlage unterstützt Sie beim Erstellen robuster, wartbarer und leicht testbarer Systeme, bei denen Ihre zentralen Geschäftsregeln völlig von technischen Details und externen Bibliotheken isoliert sind.
Sie gliedert sich in vier konzentrische Schichten:
- **Domäne (Kern):** Geschäftsmodelle und -regeln, die sich bei einem Wechsel von Datenbank oder Framework niemals ändern sollten.
- **Anwendungsfälle (Applikation):** Konkrete Operationen, die der Benutzer oder das System ausführen kann (z. B. Benutzer registrieren, Bestellung verarbeiten).
- **Infrastruktur (Adapter):** Die Verbindung zur Außenwelt: SQL/NoSQL-Datenbanken, externe API-Aufrufe, Nachrichtensysteme und Dateisysteme.
- **Präsentation (API & Controller):** Einstiegspunkte in das System wie REST-Controller, GraphQL-Endpunkte oder grafische Benutzeroberflächen.

*Beinhaltet Querverbindungen zur Veranschaulichung der Dependency Inversion: Äußere Schichten kennen innere Schichten, aber der Kern hängt niemals von der Infrastruktur ab.*

**2. Fullstack Web App (Vollständige Webanwendung):**
Optimal, wenn Sie eine moderne Webanwendung von Grund auf planen:
- **Frontend (Client):** Die Benutzeroberfläche für den Benutzer (interaktive Komponenten, Statusverwaltung und Design).
- **Backend (Server):** Die API mit Geschäftslogik, Authentifizierung, Autorisierung und Validierungen.
- **Datenbank & Persistenz:** Das relationale oder dokumentenbasierte Datenmodell, Migrationen und Hochleistungscaches.
- **DevOps & Infrastruktur:** Docker-Container, CI/CD-Pipelines und Cloud-Bereitstellung.

### 🚀 So laden Sie eine Vorlage

1. Öffnen Sie im oberen Menü **`📁 Datei`** → **`✨ Architekturvorlagen`**.
2. Wählen Sie die passende Vorlage (**Clean Architecture** oder **Fullstack Web App**).
3. MMCelt lädt die vollständige Struktur auf die Arbeitsfläche.
4. Anschließend können Sie jeden Zweig anpassen: Eigene Modelle ergänzen, Dienste umbenennen, Prioritäten setzen und auf `🤖 Künstliche Intelligenz` → `📤 Senden an...` klicken, damit Ihr KI-Agent mit der Implementierung anhand dieser Leitlinie beginnt.
