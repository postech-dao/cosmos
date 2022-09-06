use crate::request;
use serde_json::Value;
use std::error::Error;

pub async fn send_query(
    rest_api_endpoint: &str,
    contract_address: &str,
    encode_msg: &str,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let result = request(
        &client,
        &format!(
            "{}/cosmwasm/wasm/v1/contract/{}/smart/{}",
            rest_api_endpoint, contract_address, encode_msg
        ),
        None,
    )
    .await?;

    Ok(result)
}

pub async fn get_code_id(
    rest_api_endpoint: &str,
    contract_address: &str,
) -> Result<u64, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = request(
        &client,
        &format!(
            "{}/cosmwasm/wasm/v1/contract/{}",
            rest_api_endpoint, contract_address
        ),
        None,
    )
    .await?;

    let code_id = response["contract_info"]["code_id"]
        .as_str()
        .ok_or("Failed to convert code id to &str")?
        .parse::<u64>()?;

    Ok(code_id)
}

pub async fn get_sequence_number(
    rest_api_endpoint: &str,
    address: &str,
) -> Result<u64, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = request(
        &client,
        &format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            rest_api_endpoint, address
        ),
        None,
    )
    .await?;

    let sequence_number = response["account"]["sequence"]
        .as_str()
        .ok_or("Failed to convert sequence number to &str")?
        .parse::<u64>()?;

    Ok(sequence_number)
}

pub async fn get_account_number(
    rest_api_endpoint: &str,
    address: &str,
) -> Result<u64, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = request(
        &client,
        &format!(
            "{}/cosmos/auth/v1beta1/accounts/{}",
            rest_api_endpoint, address
        ),
        None,
    )
    .await?;

    let account_number = response["account"]["account_number"]
        .as_str()
        .ok_or("Failed to convert account number to &str")?
        .parse::<u64>()?;

    Ok(account_number)
}
