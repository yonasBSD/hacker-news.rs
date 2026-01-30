# 🧡 Hacker News CLI

A lightweight, high-performance command-line tool written in **Rust** to browse Hacker News. It uses **ureq 3.x** for efficient synchronous networking and **indicatif** for a smooth user experience.

## ✨ Features

* **Fast & Minimal:** Built with a blocking I/O model to keep dependencies light.
* **Beautiful UI:** Color-coded output with progress bars and clickable links.
* **Customizable:** Filter by "Hottest" (Top) or "Latest" (New) stories.
* **Adjustable Count:** Fetch anywhere from 1 to 500 stories (defaults to 30).

---

## 🚀 Installation

### Prerequisites
* [Rust and Cargo](https://rustup.rs/) (latest stable version)

### Build from source
```bash
# Clone the repository
git clone [https://github.com/yourusername/hn-cli.git](https://github.com/yourusername/hn-cli.git)
cd hn-cli

# Build the project
cargo build --release

# The binary will be located at ./target/release/hn-cli
```

---

## 🛠 Usage

You can run the tool using `cargo run --` followed by flags, or execute the compiled binary directly.

### Commands
```bash
# Get the top 30 stories (default)
./hn-cli

# Get the 10 latest stories
./hn-cli --sort latest --count 10

# Get the top 50 stories using short flags
./hn-cli -s hottest -c 50
```

### Options
| Flag | Long Flag | Description | Default |
| :--- | :--- | :--- | :--- |
| `-s` | `--sort` | Sorting mode: `hottest` or `latest` | `hottest` |
| `-c` | `--count` | Number of stories to retrieve | `30` |
| `-h` | `--help` | Print help information | N/A |

---

## 🧪 Testing

The project includes a suite of unit and integration tests to ensure API compatibility and correct data parsing.

```bash
# Run all tests
cargo test
```

> **Note:** Some tests require an active internet connection to verify the Hacker News Firebase API endpoints.

---

## 📦 Project Structure

```text
.
├── src/
│   ├── main.rs          # Main logic, CLI parsing, and API handling
├── Cargo.toml           # Dependency management (ureq, clap, colored, etc.)
└── README.md            # Project documentation
```

## 📜 Dependencies
* **ureq 3.1.4:** Minimal blocking HTTP client.
* **clap 4.0:** Command Line Argument Parser for Rust.
* **serde:** De/serialization framework for JSON parsing.
* **colored:** Terminal string coloring.
* **indicatif:** Reporting progress for Rust CLI apps.

---
