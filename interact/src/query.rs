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
            "{}/cosmwasm/wasm/v1/contract/{}/smart/{}",
            rest_api_endpoint, contract_address, encode_msg
        ),
        None,
    )
    .await
    .unwrap()
}

pub async fn get_sequence_number(rest_api_endpoint: &str, address: &str) -> u64 {
    let client = reqwest::Client::new();

    let response = request(
        &client,
        &format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            rest_api_endpoint, address
        ),
        None,
    )
    .await
    .unwrap();

    response["account"]["sequence"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

pub async fn get_account_number(rest_api_endpoint: &str, address: &str) -> u64 {
    let client = reqwest::Client::new();

    let response = request(
        &client,
        &format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            rest_api_endpoint, address
        ),
        None,
    )
    .await
    .unwrap();

    response["account"]["account_number"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap()
}
