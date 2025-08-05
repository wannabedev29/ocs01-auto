# 🚀 OCS01 Auto Runner

![Rust](https://img.shields.io/badge/Rust-2024-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Status](https://img.shields.io/badge/status-active-brightgreen)

Automation tool for interacting with **OCS01 smart contract** on OCTRA network.  
Runs all available contract methods in a single execution and saves the results.

---

## ✨ Features
- ✅ Auto-load wallet from `wallet.json`
- ✅ Executes **all contract methods** in one go
- ✅ Saves report to `ocs01_report.txt`
- ✅ Randomized params for math-based methods
- ✅ Built with Rust for speed and safety

---

## 📂 Project Structure
.
├── Cargo.toml
├── exec_interface.json
├── src/
│ └── main.rs
└── wallet.json

yaml
Copy
Edit

---

## ⚙️ Setup & Run

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
2. Clone Repo
bash
Copy
Edit
git clone https://github.com/wannabedev29/ocs01-auto.git
cd ocs01-auto
3. Configure Wallet
Edit wallet.json:

json
Copy
Edit
{
  "priv": "<your-private-key>",
  "addr": "<your-wallet-address>",
  "rpc": "https://octra.network"
}
4. Build & Run
bash
Copy
Edit
cargo build --release
./target/release/ocs01-auto
📝 Example Output
yaml
Copy
Edit
✅ Wallet loaded: oct67eeuEafdHwp3bXn58YjZkTF7HTEN...
💰 Balance: 148.846992 OCT
▶ greeting...
Result: Greetings, oct67eeu...
▶ contract info...
Result: OCS01: math & test token distribution contract (v.0.0.12)
...
🎯 Done! Hasil disimpan di ocs01_report.txt
📜 License
MIT License
