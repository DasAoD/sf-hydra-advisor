# SF Hydra Pet Advisor

Desktop-App für Shakes & Fidget Gildenleiter – empfiehlt das optimale Gildenpet gegen die aktuelle Hydra-Klasse, basierend auf den Attributen der Gildenmitglieder.

> I have no programming background and built this entirely with AI support.  
> If that's fine with you — great. If not — no problem either.

---

## Features

- Login mit SF-Account (SSO)
- Automatische Gildendaten-Abfrage
- Pet-Bilder werden beim ersten Start automatisch vom S&F-CDN heruntergeladen
- Hydra-Klasse manuell wählbar (11 Klassen)
- Empfehlung des optimalen Pets + Elements
- Automatische Update-Prüfung beim Start

---

## Installation

> ℹ️ Die Installation erfordert **keine Administratorrechte**. Bei einem Update von einer älteren Version bitte zuerst die alte Version deinstallieren. Das Programm wird im Benutzerverzeichnis installiert (`%LOCALAPPDATA%\SF Hydra Advisor\`).

1. Installer (`SF.Hydra.Advisor_x.x.x_x64-setup.exe`) von der [Releases-Seite](https://github.com/DasAoD/sf-hydra-advisor/releases/latest) herunterladen
2. Installer ausführen
3. Beim ersten Start werden die Pet-Bilder automatisch heruntergeladen (~10-30 Sekunden)

---

## Build

Voraussetzungen:
- [Rust](https://rustup.rs/)
- [Tauri CLI v2](https://v2.tauri.app/start/prerequisites/)

```powershell
cargo tauri build
```

Für signierte Release-Builds wird ein Signing Key benötigt (siehe Tauri-Dokumentation).

---

## Technologie

- [Tauri v2](https://v2.tauri.app/) – Desktop-Framework (Rust + WebView)
- [sf-api](https://github.com/the-marenga/sf-api) von [marenga](https://github.com/the-marenga) – Shakes & Fidget API
- Unity Asset Bundle Decoder (C++ Crunch + Rust DXT5)

---

## Mitwirkende

Dieses Projekt wurde in Zusammenarbeit mit [Claude](https://claude.ai) (Sonnet 4.6) von [Anthropic](https://anthropic.com) entwickelt und iterativ ausgebaut.  
Der überwiegende Teil des Codes, der Architektur und der Dokumentation wurde durch KI generiert und gemeinsam verfeinert.

| Rolle | Person / Tool |
|---|---|
| Projektidee, Anforderungen & Tests | [DasAoD](https://github.com/DasAoD) |
| Code, Architektur, Dokumentation | Claude (Anthropic) |

---

## Lizenz

[MIT](LICENSE)
