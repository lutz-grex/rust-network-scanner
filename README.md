# 🔍 Rust Network Scanner

An asynchronous network scanner in Rust that scans IP addresses and port ranges, collects banner information, and identifies CVE vulnerabilities (Common Vulnerabilities and Exposures). Optionally, results can be saved as a JSON file.

## ✨ Features

- 🔌 **Asynchronous network scanning** with configurable concurrency and timeout
- 🧾 **Banner detection** to identify service information (e.g., HTTP servers)
- 🔐 **CVE detection via Vulners API** (optional)
- 💾 **Output as JSON file**
- ⚙️ **CLI using Clap**
- 🌐 (Optional) REST API with Rocket

## 📦 Dependencies

- [`tokio`](https://docs.rs/tokio) – Asynchronous runtime
- [`rocket`](https://rocket.rs) – Web framework (optional)
- [`reqwest`](https://docs.rs/reqwest) – HTTP client
- [`serde`](https://serde.rs) – Serialization/Deserialization
- [`anyhow`](https://docs.rs/anyhow) – Error handling
- [`clap`](https://docs.rs/clap) – CLI argument parsing


## 🚀 Installation

```bash
git clone https://github.com/dein-benutzername/rust-network-scanner.git
cd rust-network-scanner
cargo build --release
```

## 🔧 Example Usage

```bash
cargo run -- scan \
  192.168.1.1 \
  --ports 80,443,22 \
  --timeout 1000 \
  --concurrency 200 \
  --cve \
  --output output.json
```

### Arguments

| Argument         |                          Description                           |
|------------------|:--------------------------------------------------------------:|
| `target`         | Target IP or range (e.g `192.168.0.1`, `10.0.0.0/24`)          |
| `--ports`        | Comma-separated list of ports or ranges (`80`, `22-25`, `443`) |
| `--timeout`      |          Timeout per connection (ms) [default: `500`]          |
| `--concurrency`  |        Number of parallel connections [default: `100`]         |
| `--cve`          |                      Enable CVE detection                      |
| `--output`       |               Optional path to JSON output file                |

## 📂 Project Structure

```txt
src/
├── cli/                # Command-line parsing
├── network/            # Network functions: scanning, CVE queries, etc.
├── services/           # File-writing services, etc.
├── routes/             # Optional Rocket API endpoints
├── thread_executor/    # Parallel processing
├── models/             # Structs & data models
└── main.rs             # Entry point
```

## 🔐 CVE Detection with Vulners API

The scanner integrates the [Vulners API](https://vulners.com/) to detect vulnerabilities based on service banners (e.g., Apache/2.4.41).


## 🌐 Optional: HTTP API with Rocket

The code is prepared to run as a REST API using Rocket. To enable the API, uncomment the following section in main.rs:

```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes::scan::routes())
        .mount("/api", routes::health::routes())
}
```

🛡 Security

The scanner can be linked with vulnerability databases like Vulners to identify known vulnerabilities based on banners or server information.

## 📄 Lizenz

MIT – feel free to use, modify, and share.

---
