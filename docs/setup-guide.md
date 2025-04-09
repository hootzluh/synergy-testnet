# Synergy Network Testnet Setup Guide

This guide walks you through setting up and running a node for the Synergy Network testnet.

---

## 🛠️ Prerequisites

Make sure your system has the following installed:

- Ubuntu 20.04 (native or WSL2)
- Git
- Docker
- Docker Compose
- Rust (via rustup)
- Go (v1.18+)
- Node.js & npm

---

## 🚀 Setup Steps

1. **Clone the Repository**
   ```bash
   git clone https://github.com/hootzluh/synergy-testnet.git
   cd synergy-testnet
   ```

2. **Install Rust (if not already installed)**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Build the Project**
   ```bash
   cargo build --release
   ```

4. **Start the Testnet Node**
   ```bash
   ./scripts/start-testnet.sh
   ```

5. **Stop the Testnet Node**
   ```bash
   ./scripts/stop-testnet.sh
   ```

---

## 🧪 Running Tests

Unit tests are located in the `tests/` directory. Run them with:
```bash
cargo test
```

---

## 📁 File Overview

- `src/` — Core blockchain logic
- `config/` — Network, consensus, and genesis configuration
- `scripts/` — Node management scripts
- `tests/` — Rust integration and unit tests
- `dependencies/` — External tools & manifest files
- `docs/` — Developer documentation

---

## 💬 Need Help?

Visit the GitHub Issues page or contact the Synergy Network dev team for support.
