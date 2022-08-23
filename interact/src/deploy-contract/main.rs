use pdao_cosmos_interact::deploy::{instantiate_contract, store_contract};
use pdao_cosmos_interact::query;
use pdao_cosmos_interact::utils::{mnemonic_to_private_key, private_to_pub_and_account};
use pdao_cosmos_interact::Config;
use serde_json::json;

#[tokio::main]
async fn main() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/interact/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);
    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password);

    let code_id = store_contract(
        &sender_private_key,
        "artifacts/simple_counter.wasm",
        &_config.rpc,
        &_config.full_node_url,
        &_config.chain_id,
        &_config.denom,
        None,
        20000000,
        20000000,
        &_config.account_prefix,
    )
    .await;

    let (_, sender_account_address) =
        private_to_pub_and_account(&sender_private_key, &_config.account_prefix);
    let msg = json!({
        "count": 100u64,
        "auth": [sender_account_address.to_string()]
    });

    let contract_address = instantiate_contract(
        &sender_private_key,
        code_id,
        &_config.rpc,
        &_config.full_node_url,
        &_config.chain_id,
        &_config.denom,
        None,
        serde_json::to_vec(&msg).unwrap(),
        20000000,
        20000000,
        &_config.account_prefix,
        10000,
    )
    .await;

    println!("{}", code_id);
    println!("{}", contract_address);

    let code_id_from_query =
        query::get_code_id(&_config.full_node_url, contract_address.as_str()).await;
    assert_eq!(code_id, code_id_from_query);
}
