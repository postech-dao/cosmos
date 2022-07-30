use anyhow::{anyhow, Result};
use cosmrs::crypto;
use reqwest::Client;
use serde_json::Value;

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
