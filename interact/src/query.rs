use crate::request;
use serde_json::Value;

pub async fn send_query(
    rest_api_endpoint: &str,
    contract_address: &str,
    encode_msg: &str,
) -> Value {
    let client = reqwest::Client::new();

    request(
        &client,
        &format!(
            "https://{}/cosmwasm/wasm/v1/contract/{}/smart/{}",
            rest_api_endpoint, contract_address, encode_msg
        ),
        None,
    )
    .await
    .unwrap()
}
