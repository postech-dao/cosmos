pub mod deploy;
pub mod execute;
pub mod query;
pub mod utils;

use anyhow::{anyhow, Result};
use cosmrs::crypto;
use reqwest::Client;
use serde_json::Value;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub full_node_url: String,
    pub account_public: String,
    pub account_private: String,
    pub wasm_binary_path: String,
    pub rpc: String, // and so on
    pub chain_id: String,
    pub denom: String,
    pub mnemonic: String,
    pub password: String,
    pub account_prefix: String,
}

impl Config {
    pub fn set_env(key: &str, value: &str) {
        std::env::set_var(key, value);
        assert_eq!(std::env::var(key), Ok(value.to_string()));
    }

    pub fn read_from_env() -> Self {
        serde_json::from_str(
            &std::fs::read_to_string(
                std::env::var("config")
                    .expect("Environment variable for the config file path is missing"),
            )
            .expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }

    pub fn read_from_path() -> Self {
        let full_path = format!(
            "{}{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            "/test_config_example.json"
        );

        println!("{}", full_path);

        serde_json::from_str(
            &std::fs::read_to_string(full_path).expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }

    pub fn read_from_path_main() -> Self {
        let full_path = format!(
            "{}{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            "/interact/test_config_example.json"
        );

        println!("{}", full_path);

        serde_json::from_str(
            &std::fs::read_to_string(full_path).expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }
}

pub async fn request(client: &Client, url: &str, block_number: Option<u64>) -> Result<Value> {
    let response = if let Some(block_number) = block_number {
        client
            .get(url)
            .header("x-cosmos-block-height", format!("{}", block_number))
            .send()
            .await?
    } else {
        client.get(url).send().await?
    };
    let text = format!("{:?}", response);
    response
        .json::<Value>()
        .await
        .map_err(|_| anyhow!("Failed to parse to json: {}", text))
}

pub fn generate_keypair() {
    let mut rng = rand::thread_rng();
    let secret = secp256k1::SecretKey::new(&mut rng)
        .display_secret()
        .to_string();
    println!("Secret Key: {}", secret);

    let k = secp256k1::PublicKey::from_secret_key(
        &secp256k1::Secp256k1::new(),
        &secp256k1::SecretKey::from_slice(&hex::decode(secret.clone()).unwrap()).unwrap(),
    );
    println!("Public Key: {}", k);

    let key = crypto::secp256k1::SigningKey::from_bytes(&hex::decode(secret).unwrap()).unwrap();
    println!(
        "Cosmos Imported - Public Key: {}",
        key.public_key().to_string()
    );
    println!(
        "Cosmos Imported - Juno ID: {}",
        key.public_key().account_id("juno").unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        generate_keypair();
    }
}
