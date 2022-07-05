use anyhow::{anyhow, Result};
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
