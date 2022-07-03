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
                std::env::vars()
                    .nth(1)
                    .expect("Environment variable for the config file path is missing"),
            )
            .expect("Failed to locate the config file"),
        )
        .expect("Failed to parse the config")
    }
}

#[tokio::test]
async fn check_connection() {
    let _config = Config::read_from_env();
    // check whether the full node is responding by a simple request
    unimplemented!();
}

#[tokio::test]
async fn check_block_number() {
    let _config = Config::read_from_env();
    // check the latest block number recognized by the full node **twice** with some delay,
    // so that we can assure that the full node is properly updating its blocks
    unimplemented!();
}

#[tokio::test]
async fn check_account() {
    let _config = Config::read_from_env();
    // by requesting the full node, check whether the account given by the config has enough native token to pay gas fee
    unimplemented!();
}

#[tokio::test]
async fn upload_modify_and_query() {
    let _config = Config::read_from_env();
    // upload the contract, submit a transaction that modifies its state, and query the state
    unimplemented!();
}
