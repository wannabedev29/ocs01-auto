use serde::Deserialize;
use serde_json::json;
use std::{
    fs,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
    collections::HashMap
};
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{Signer, SigningKey};
use reqwest::blocking::Client;
use anyhow::{Result, bail};
use rand::Rng;

// =============================
// Struct Definitions
// =============================
#[derive(Deserialize)]
struct Wallet {
    #[serde(rename = "priv")]
    priv_: String,
    addr: String,
    rpc: String,
}

#[derive(Deserialize)]
struct Param {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    example: Option<String>,
    max: Option<u64>,
}

#[derive(Deserialize)]
struct Method {
    name: String,
    label: String,
    params: Vec<Param>,
    #[serde(rename = "type")]
    method_type: String,
}

#[derive(Deserialize)]
struct Interface {
    contract: String,
    methods: Vec<Method>,
}

#[derive(Deserialize)]
struct BalanceResponse {
    balance_raw: String,
    nonce: u64,
}

// =============================
// Helper: API Call
// =============================
fn api_call<T: for<'de> Deserialize<'de>>(
    client: &Client,
    method: &str,
    url: &str,
    data: Option<serde_json::Value>
) -> Result<T> {
    let resp = match method {
        "GET" => client.get(url).send()?,
        "POST" => client.post(url).json(&data).send()?,
        _ => bail!("Unsupported HTTP method"),
    };

    if resp.status().as_u16() >= 400 {
        bail!("api error: {}", resp.text()?);
    }

    Ok(resp.json()?)
}

// =============================
// Helper: Balance
// =============================
fn get_balance(client: &Client, api_url: &str, addr: &str) -> Result<(f64, u64)> {
    let balance: BalanceResponse = api_call(
        client,
        "GET",
        &format!("{}/balance/{}", api_url, addr),
        None
    )?;
    let oct_balance = balance.balance_raw.parse::<f64>()? / 1_000_000.0;
    Ok((oct_balance, balance.nonce))
}

// =============================
// Helper: TX Signing
// =============================
fn sign_tx(sk: &SigningKey, tx: &HashMap<&str, String>) -> String {
    let blob = format!(
        r#"{{"from":"{}","to_":"{}","amount":"{}","nonce":{},"ou":"{}","timestamp":{}}}"#,
        tx["from"], tx["to_"], tx["amount"], tx["nonce"], tx["ou"], tx["timestamp"]
    );
    let sig = sk.sign(blob.as_bytes());
    general_purpose::STANDARD.encode(sig.to_bytes())
}

// =============================
// View Call
// =============================
fn view_call(client: &Client, api_url: &str, contract: &str, method: &str, params: &[String], caller: &str) -> Result<String> {
    let res: serde_json::Value = api_call(
        client,
        "POST",
        &format!("{}/contract/call-view", api_url),
        Some(json!({
            "contract": contract,
            "method": method,
            "params": params,
            "caller": caller
        }))
    )?;

    if res["status"] == "success" {
        Ok(res["result"].as_str().unwrap_or("null").to_string())
    } else {
        bail!("Error: {:?}", res)
    }
}

// =============================
// TX Call with Retry
// =============================
fn call_contract_tx(client: &Client, api_url: &str, sk: &SigningKey, from: &str, contract: &str, method: &str, params: &[String]) -> Result<String> {
    for attempt in 1..=3 {
        match try_send_tx(client, api_url, sk, from, contract, method, params) {
            Ok(hash) => return Ok(hash),
            Err(e) => {
                eprintln!("⚠ Attempt {}/3 failed: {}", attempt, e);
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
    bail!("All retries failed")
}

fn try_send_tx(client: &Client, api_url: &str, sk: &SigningKey, from: &str, contract: &str, method: &str, params: &[String]) -> Result<String> {
    let (_, nonce) = get_balance(client, api_url, from)?;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

    let mut tx = HashMap::new();
    tx.insert("from", from.to_string());
    tx.insert("to_", contract.to_string());
    tx.insert("amount", "0".to_string());
    tx.insert("nonce", (nonce + 1).to_string());
    tx.insert("ou", "1".to_string());
    tx.insert("timestamp", timestamp.to_string());

    let signature = sign_tx(sk, &tx);
    let pub_key = general_purpose::STANDARD.encode(sk.verifying_key().to_bytes());

    let res: serde_json::Value = api_call(
        client,
        "POST",
        &format!("{}/call-contract", api_url),
        Some(json!({
            "contract": contract,
            "method": method,
            "params": params,
            "caller": from,
            "nonce": nonce + 1,
            "timestamp": timestamp,
            "signature": signature,
            "public_key": pub_key
        }))
    )?;

    Ok(res["tx_hash"].as_str().unwrap_or("").to_string())
}

// =============================
// Generate Params (random)
// =============================
fn generate_params(params: &[Param]) -> Vec<String> {
    let mut rng = rand::thread_rng();
    params.iter().map(|p| {
        if let Some(ex) = &p.example {
            ex.clone()
        } else if let Some(max) = p.max {
            rng.gen_range(1..=max).to_string()
        } else {
            rng.gen_range(1..=100).to_string()
        }
    }).collect()
}

// =============================
// Logging
// =============================
fn log_to_file(msg: &str) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("ocs01_report.txt")
        .unwrap();
    writeln!(file, "{}", msg).unwrap();
}

// =============================
// MAIN
// =============================
fn main() -> Result<()> {
    let wallet: Wallet = serde_json::from_str(&fs::read_to_string("wallet.json")?)?;
    let interface: Interface = serde_json::from_str(&fs::read_to_string("exec_interface.json")?)?;

    let sk_bytes = general_purpose::STANDARD.decode(&wallet.priv_)?;
    let sk = SigningKey::from_bytes(&sk_bytes.try_into().unwrap());
    let client = Client::builder().timeout(std::time::Duration::from_secs(100)).build()?;

    println!("✅ Wallet loaded: {}", wallet.addr);

    let (balance, _) = get_balance(&client, &wallet.rpc, &wallet.addr)?;
    println!("💰 Balance: {:.6} OCT", balance);

    for method in &interface.methods {
        println!("▶ {}...", method.label);
        let params = generate_params(&method.params);
        match method.method_type.as_str() {
            "view" => {
                match view_call(&client, &wallet.rpc, &interface.contract, &method.name, &params, &wallet.addr) {
                    Ok(result) => {
                        println!("Result: {}", result);
                        log_to_file(&format!("{}: {}", method.label, result));
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        log_to_file(&format!("{}: Error - {}", method.label, e));
                    }
                }
            }
            "call" => {
                match call_contract_tx(&client, &wallet.rpc, &sk, &wallet.addr, &interface.contract, &method.name, &params) {
                    Ok(tx_hash) => {
                        println!("TX Hash: {}", tx_hash);
                        log_to_file(&format!("{}: TX Hash {}", method.label, tx_hash));
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        log_to_file(&format!("{}: Error - {}", method.label, e));
                    }
                }
            }
            _ => println!("Unknown method type"),
        }
        std::thread::sleep(std::time::Duration::from_secs(2)); // Delay antar eksekusi
    }

    println!("\n🎯 Done! U ALREADY COOCKEDD FRR FRR ON GOD!");
    Ok(())
}

