# 🔍 Rust Network Scanner

Ein performanter, asynchroner Netzwerk-Scanner in Rust, der IP-Adressen und Portbereiche scannt, Bannerinformationen sammelt und CVE-Schwachstellen (Common Vulnerabilities and Exposures) identifiziert. Optional können Ergebnisse als JSON-Datei gespeichert werden.

## ✨ Features

- 🔌 **Asynchroner Netzwerk-Scan** mit konfigurierbarer Parallelität und Timeout
- 🧾 **Banner-Erkennung** zur Ermittlung von Dienstinformationen (z. B. HTTP-Server)
- 🔐 **CVE-Erkennung via Vulners API** (optional)
- 💾 **Ausgabe als JSON-Datei**
- ⚙️ **CLI mit Clap**
- 🌐 (Optional) REST-API mit Rocket

## 📦 Abhängigkeiten

- [`tokio`](https://docs.rs/tokio) – Asynchrone Runtime
- [`rocket`](https://rocket.rs) – Web-Framework (optional)
- [`reqwest`](https://docs.rs/reqwest) – HTTP-Client
- [`serde`](https://serde.rs) – Serialisierung/Deserialisierung
- [`anyhow`](https://docs.rs/anyhow) – Fehlerbehandlung
- [`clap`](https://docs.rs/clap) – CLI-Parsing

## 🚀 Installation

```bash
git clone https://github.com/dein-benutzername/rust-network-scanner.git
cd rust-network-scanner
cargo build --release
```

## 🔧 Beispiel-Nutzung

```bash
cargo run -- scan \
  192.168.1.1 \
  --ports 80,443,22 \
  --timeout 1000 \
  --concurrency 200 \
  --banner \
  --output output.json
```

### Argumente

| Argument         | Beschreibung                                                                 |
|------------------|------------------------------------------------------------------------------|
| `target`         | Ziel-IP oder Bereich (z. B. `192.168.0.1`, `10.0.0.0/24`)                     |
| `--ports`        | Kommaseparierte Liste von Ports oder Bereiche (`80`, `22-25`, `443`)         |
| `--timeout`      | Timeout pro Verbindung (ms) [default: `500`]                                 |
| `--concurrency`  | Anzahl paralleler Verbindungen [default: `100`]                              |
| `--banner`       | Versucht Bannerinformationen (z. B. Server-Header) zu lesen                   |
| `--output`       | Optionaler Pfad zur JSON-Ausgabedatei                                        |

## 📂 Projektstruktur

```txt
src/
├── cli/                # Kommandozeilen-Parsing
├── network/            # Netzwerkfunktionen: Scans, CVE-Abfrage, etc.
├── services/           # Dateischreibdienste u. Ä.
├── routes/             # Optionale Rocket-API-Endpunkte
├── thread_executor/    # Parallele Verarbeitung
├── models/             # Structs & Datenmodelle
└── main.rs             # Einstiegspunkt
```

## 🔐 CVE-Erkennung mit Vulners API

Der Scanner integriert die [Vulners API](https://vulners.com/) zur Erkennung von Schwachstellen basierend auf Dienst-Bannern (z. B. `Apache/2.4.41`).



## 🌐 Optional: HTTP API mit Rocket

Der Code ist vorbereitet für die Bereitstellung als REST API mit Rocket. Um die API zu aktivieren, entkommentiere folgenden Abschnitt in `main.rs`:

```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes::scan::routes())
        .mount("/api", routes::health::routes())
}
```

## 🛡 Sicherheit

Der Scanner kann mit Sicherheitsdatenbanken wie Vulners verknüpft werden, um bekannte Schwachstellen anhand von Bannern oder Serverinformationen zu identifizieren.

## 📄 Lizenz

MIT – feel free to use, modify and share.

---
