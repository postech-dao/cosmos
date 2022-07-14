use pdao_cosmos_interact::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    full_node_url: String,
    account_public: String,
    account_private: String,
    wasm_binary_path: String,
    // and so on
}

impl Config {
    pub fn read_from_env() -> Self {
        serde_json::from_str(
            &std::fs::read_to_string(
                std::env::var("TEST_CONFIG")
                    .expect("Environment variable for the config file path is missing"),
            )
            .expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }
}

#[ignore]
#[tokio::test]
async fn check_connection() {
    let _config = Config::read_from_env();
    // check whether the full node is responding by a simple request
    unimplemented!();
}

#[ignore]
#[tokio::test]
async fn check_block_number() {
    let _config = Config::read_from_env();
    // check the latest block number recognized by the full node **twice** with some delay,
    // so that we can assure that the full node is properly updating its blocks
    unimplemented!();
}

/// by requesting the full node, checks whether the account given by the config has enough native token to pay gas fee
#[ignore]
#[tokio::test]
async fn check_account() {
    let _config = Config::read_from_env();

    let rest_api_endpoint = "TODO";
    let target_address = "TODO";
    let min_balance = 1234; // TODO;

    let client = reqwest::Client::new();
    let response = request(
        &client,
        &format!(
            "https://{}/cosmos/bank/v1beta1/balances/{}",
            rest_api_endpoint, target_address
        ),
        None,
    )
    .await
    .unwrap();

    let current_balance = response["balances"].as_array().unwrap()[0]["amount"]
        .as_str()
        .unwrap();

    assert!(min_balance <= current_balance.parse::<u64>().unwrap());
}

#[ignore]
#[tokio::test]
async fn upload_modify_and_query() {
    let _config = Config::read_from_env();
    // upload the contract, submit a transaction that modifies its state, and query the state
    unimplemented!();
}
