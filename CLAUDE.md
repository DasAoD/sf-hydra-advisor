# sf-hydra-advisor – Projektkontext für Claude Code

Desktop-App für Shakes & Fidget Gildenleiter – empfiehlt das optimale Gildenpet gegen die aktuelle Hydra-Klasse, basierend auf den Attributen der Gildenmitglieder.

## Tech-Stack
- Tauri v2 (Rust + WebView) – Desktop-Framework
- sf-api (the-marenga/sf-api) für die S&F-API-Anbindung
- Unity Asset Bundle Decoder: C++ (Crunch) + Rust (DXT5) für Pet-Bilder, siehe `src-tauri/cpp/unitycrunch/`

## Struktur
- `src/` – Frontend
- `src-tauri/` – Rust-Backend, Tauri-Config, Capabilities
- `src-tauri/cpp/unitycrunch/` – C++ Crunch-Decoder für Unity Asset Bundles
- `src-tauri/icons/`, `src-tauri/nsis/` – Icons und Windows-Installer-Konfiguration

## Funktionsweise
- Login mit SF-Account (SSO)
- Automatische Gildendaten-Abfrage
- Pet-Bilder werden beim ersten Start automatisch vom S&F-CDN heruntergeladen (~10–30 Sek.)
- Hydra-Klasse manuell wählbar (11 Klassen)
- Empfehlung des optimalen Pets + Elements
- Automatische Update-Prüfung beim Start

## Build
Voraussetzungen: Rust, Tauri CLI v2
```powershell
cargo tauri build
```
Für signierte Release-Builds wird ein Signing Key benötigt (siehe Tauri-Dokumentation).

## Installation (Referenz für Nutzer-Support)
- Keine Administratorrechte erforderlich
- Windows SmartScreen zeigt beim ersten Start ggf. eine Warnung ("Unbekannter Herausgeber") – normal bei unsignierten Apps, "Weitere Informationen" → "Trotzdem ausführen"
- Installation im Benutzerverzeichnis: `%LOCALAPPDATA%\SF Hydra Advisor\`
- Bei Update von älterer Version: alte Version zuerst deinstallieren

## Hinweis zur Zielgruppe
Laut README explizit für Nutzer ohne Programmier-Hintergrund gebaut/dokumentiert – bei Nutzer-facing Texten (Fehlermeldungen, UI) entsprechend einfache, klare Sprache bevorzugen.
