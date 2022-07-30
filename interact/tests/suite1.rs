#![cfg(feature = "dev")]

use cosmrs::{
    crypto::secp256k1,
    dev, rpc,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};
use pdao_cosmos_interact::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

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
async fn modify_and_query() {
    let _config = Config::read_from_env();
    // submit a transaction that modifies its state, and query the state

    let sender_private_key =
        secp256k1::SigningKey::from_bytes(&hex::decode("abcdabcdabcd").unwrap()).unwrap();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id("juno").unwrap();

    // For paying the gas fee
    let amount = Coin {
        amount: 100000u32.into(),
        denom: "ujunox".parse().unwrap(),
    };

    let msg = cosmrs::cosmwasm::MsgExecuteContract {
        sender: sender_account_id.clone(),
        contract: "juno12341231234".parse().unwrap(),
        msg: serde_json::to_vec(&"OneOfTheDefineMessageInSimpleCounter").unwrap(),
        funds: vec![amount.clone()],
    }
    .to_any()
    .unwrap();

    let chain_id = "uni-3".parse().unwrap();
    let sequence_number = 6;
    let gas = 100_0000;
    let fee = Fee::from_amount_and_gas(amount.clone(), gas);
    let timeout_height = 0u16;

    let tx_body = tx::Body::new(vec![msg], "memoemo", timeout_height);
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    // account_number: check at cosmos/auth/v1beta1/accounts/juno123412341234 (your address)
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, 123456).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let rpc_address = "https://TENDERMINT_RPC_ADDR".to_owned();
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
    let _tx_commit_response =
        rpc::Client::broadcast_tx_commit(&rpc_client, tx_raw.to_bytes().unwrap().into())
            .await
            .unwrap();

    // check the response
    unimplemented!();
}

#[ignore]
#[tokio::test]
async fn store_contract() {
    let _config = Config::read_from_env();
    // Submit a transaction that store the simple-counter contract

    let sender_private_key = secp256k1::SigningKey::random();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id("juno").unwrap();

    // For paying the gas fee
    let amount = Coin {
        amount: 100000u32.into(),
        denom: "umlg".parse().unwrap(),
    };

    let mut file = File::open("./simple-counter/artifact/simple_counter.wasm").unwrap();
    let mut data = Vec::new();
    (file.read_to_end(&mut data));

    let msg = cosmrs::cosmwasm::MsgStoreCode {
        sender: sender_account_id.clone(),
        wasm_byte_code: data,
        instantiate_permission: None,
    }
    .to_any()
    .unwrap();

    let chain_id = "malaga-420".parse().unwrap();
    let sequence_number = 0;
    let gas = 100_0000;
    let fee = Fee::from_amount_and_gas(amount.clone(), gas);
    let timeout_height = 0u16;

    let tx_body = tx::Body::new(vec![msg], "test memo", timeout_height);
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    // account_number: check at cosmos/auth/v1beta1/accounts/juno123412341234 (your address)
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, 123456).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let rpc_address = "https://rpc.malaga-420.cosmwasm.com:443".to_owned();
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
    let tx_commit_response =
        rpc::Client::broadcast_tx_commit(&rpc_client, tx_raw.to_bytes().unwrap().into())
            .await
            .unwrap();

    // check the response
    if tx_commit_response.check_tx.code.is_err() {
        panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
    }

    if tx_commit_response.deliver_tx.code.is_err() {
        panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
    }

    let tx = dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;
    assert_eq!(&tx_body, &tx.body);
    assert_eq!(&auth_info, &tx.auth_info);
}

#[ignore]
#[tokio::test]
async fn instantiate_contract() {
    let _config = Config::read_from_env();
    // Submit a transaction that store the simple-counter contract

    let sender_private_key = secp256k1::SigningKey::random();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id("juno").unwrap();

    // For paying the gas fee
    let amount = Coin {
        amount: 200000u32.into(),
        denom: "umlg".parse().unwrap(),
    };

    let contract_code_id: u64 = 1234;

    let msg_send = cosmrs::cosmwasm::MsgInstantiateContract {
        sender: sender_account_id.clone(),
        admin: None,
        code_id: contract_code_id,
        label: None,
        msg: "Instantiate simple counter".as_bytes(),
        funds: vec![amount.clone()],
    }
    .to_any()
    .unwrap();

    let chain_id = "malaga-420".parse().unwrap();
    let sequence_number = 0;
    let gas = 100_0000;
    let fee = Fee::from_amount_and_gas(amount.clone(), gas);
    let timeout_height = 0u16;

    let tx_body = tx::Body::new(vec![msg_send], "test memo", timeout_height);
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    // account_number: check at cosmos/auth/v1beta1/accounts/juno123412341234 (your address)
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, 123456).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let rpc_address = "https://rpc.malaga-420.cosmwasm.com:443".to_owned();
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
    let tx_commit_response =
        rpc::Client::broadcast_tx_commit(&rpc_client, tx_raw.to_bytes().unwrap().into())
            .await
            .unwrap();

    // check the response
    if tx_commit_response.check_tx.code.is_err() {
        panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
    }

    if tx_commit_response.deliver_tx.code.is_err() {
        panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
    }

    let tx = dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;
    assert_eq!(&tx_body, &tx.body);
    assert_eq!(&auth_info, &tx.auth_info);
}
