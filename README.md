# 🌟 OCS01 Auto Runner 

[![Rust Version](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Project Status](https://img.shields.io/badge/Status-Active-brightgreen)](https://github.com/wannabedev29/ocs01-auto/commits/main)

A blazing fast ⚡ Rust automation tool for interacting with **OCS01 smart contracts** on the Octra blockchain network.

---

## 🎯 Features

| Feature | Description |
|---------|-------------|
| 🔐 **Secure Wallet** | Auto-load from encrypted `wallet.json` |
| 🤖 **Smart Execution** | Runs all contract methods sequentially |
| 📈 **Auto Reporting** | Generates detailed `ocs01_report.txt` |
| 🎲 **Randomized Inputs** | Dynamic params for math operations |
| 🛡️ **Rust Safety** | Memory-safe and thread-safe |

---

## 📁 Project Structure

```text
ocs01-auto/
├── 📄 Cargo.toml           # Project config
├── 📄 exec_interface.json  # ABI definitions
├── 📂 src/                 # Source code
│   └── 📄 main.rs          # Core logic
└── 📄 wallet.json          # Wallet config
```
---

---

## 🛠️ Installation & Usage

### Prerequisites
- Rust 1.70+ ([install guide](https://www.rust-lang.org/tools/install))

### ✅ 3. Configure Wallet
Edit wallet.json:
```text
{
  "priv": "<your-private-key>",
  "addr": "<your-wallet-address>",
  "rpc": "https://octra.network"
}
```
---

### Quick Start
```bash
# Clone repository
git clone https://github.com/wannabedev29/ocs01-auto.git
cd ocs01-auto

# Configure wallet (edit before use!)
cp wallet.example.json wallet.json
nano wallet.json

# Build and run
cargo build --release
./target/release/ocs01-auto
```
---
### 📄 Sample Output
```text

✅ Wallet loaded: oct67eeuEafdHwp3bXn58YjZkTF7HTEN...
💰 Balance: 148.846992 OCT
▶ greeting...
Result: Greetings, oct67eeu...
▶ contract info...
Result: OCS01: math & test token distribution contract (v.0.0.12)
▶ claim token...
TX Hash: e1695e38cd531b79...
▶ check token balance...
Result: 1000000000000
...
🎯 Done! Report saved in ocs01_report.txt
```
---
## 📜 License
MIT © 2024 wannabedev29
