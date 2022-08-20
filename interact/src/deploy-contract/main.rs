use cosmrs::{bip32, crypto::secp256k1};
use pdao_cosmos_interact::deploy::{instantiate_contract, store_contract};
use pdao_cosmos_interact::utils::private_to_pub_and_account;
use pdao_cosmos_interact::Config;
use serde_json::json;

#[tokio::main]
async fn main() {
    let _config = Config::read_from_env();

    let mnemonic = "coyote electric million purchase tennis skin quiz inside helmet call glimpse pulse turkey hint maze iron festival run bomb regular legend prepare service angry".to_string();
    let seed = bip32::Mnemonic::new(mnemonic, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let key: secp256k1::SigningKey = bip32::XPrv::new(seed).unwrap().into();
    store_contract(
        &key,
        "../simple-counter/artifacts/simple_counter-aarch64.wasm",
        "https://rpc.malaga-420.cosmwasm.com:443",
        "https://api.malaga-420.cosmwasm.com:443",
        "malaga-420",
        1411,
        "umlg",
        None,
        2000000,
        2000000,
        "wasm",
    )
    .await;

    let (_, sender_account_id) = private_to_pub_and_account(&key, "wasm");

    let msg = json!({
        "count": 100u64,
        "auth": [sender_account_id.to_string()]
    });

    // FIXME: bring code_id from store_contract
    let code_id = 0;
    instantiate_contract(
        &key,
        code_id,
        "https://rpc.malaga-420.cosmwasm.com:443",
        "https://api.malaga-420.cosmwasm.com:443",
        "malaga-420",
        1411,
        "umlg",
        None,
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        "wasm",
        10000,
    )
    .await;
    // FIXME: output contract_address
}
