use pdao_cosmos_interact::utils::{mnemonic_to_private_key, private_to_pub_and_account};
use pdao_cosmos_interact::*;
use serde_json::json;
use std::{thread, time};

//Current test contract address: juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf
//Contract auth address: juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a

// check whether the full node is responding by a simple request
#[ignore]
#[tokio::test]
async fn check_connection() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let query_info = "net_info?";
    let client = reqwest::Client::new();
    let response = request(&client, &format!("{}/{}", _config.rpc, query_info), None)
        .await
        .unwrap();

    let listening = response["result"]["listening"].as_bool().unwrap();

    assert!(listening, "listening should be true");
}

// check the latest block number recognized by the full node **twice** with some delay,
// so that we can assure that the full node is properly updating its blocks
#[ignore]
#[tokio::test]
async fn check_block_number() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let query_info = "abci_info?";
    let client = reqwest::Client::new();
    let response_first = request(&client, &format!("{}/{}", _config.rpc, query_info), None)
        .await
        .unwrap();

    let first_block_height = response_first["result"]["response"]["last_block_height"]
        .as_str()
        .unwrap();

    let mut response_second;
    let mut second_block_height = first_block_height;

    let five_secs = time::Duration::from_secs(5);
    while first_block_height == second_block_height {
        thread::sleep(five_secs);

        response_second = request(&client, &format!("{}/{}", _config.rpc, query_info), None)
            .await
            .unwrap();

        second_block_height = response_second["result"]["response"]["last_block_height"]
            .as_str()
            .unwrap();
    }

    assert!(first_block_height < second_block_height);
}

//by requesting the full node, checks whether the account given by the config has enough native token to pay gas fee
#[ignore]
#[tokio::test]
async fn check_account_gas_fee() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let min_balance = 20000000u64;

    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password)
        .unwrap()
        .into();
    let (_, sender_account_address) =
        private_to_pub_and_account(&sender_private_key, &_config.account_prefix).unwrap();

    let client = reqwest::Client::new();
    let response = request(
        &client,
        &format!(
            "{}/cosmos/bank/v1beta1/balances/{}",
            _config.full_node_url, sender_account_address
        ),
        None,
    )
    .await
    .unwrap();

    let current_balance = response["balances"].as_array().unwrap()[0]["amount"]
        .as_str()
        .unwrap();

    assert!(min_balance < current_balance.parse::<u64>().unwrap());
}

#[ignore]
#[tokio::test]
async fn test_query_get_count() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let msg = json!({
        "get_count": {}
    });

    let encode_msg = base64::encode(&serde_json::to_vec(&msg).unwrap());

    let response = query::send_query(
        &_config.full_node_url,
        "juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf",
        encode_msg.as_str(),
    )
    .await
    .unwrap();

    let count = response["data"]["count"].as_u64().unwrap();

    assert_eq!(count, 50);
}

#[ignore]
#[tokio::test]
async fn test_query_get_auth() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let msg = json!({
        "get_auth": {}
    });

    let encode_msg = base64::encode(&serde_json::to_vec(&msg).unwrap());

    let response = query::send_query(
        &_config.full_node_url,
        "juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf",
        encode_msg.as_str(),
    )
    .await
    .unwrap();

    let auth = response["data"]["auth"].as_array().unwrap()[0]
        .as_str()
        .unwrap();

    assert_eq!(auth, "juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a");
}

#[ignore]
#[tokio::test]
async fn test_execute_increment_fail() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password)
        .unwrap()
        .into();

    // This should be failed since the count is above 10
    let msg = json!({
        "increment": {"count": 20u64}
    });

    let _result = execute::send_execute(
        &sender_private_key,
        &_config.chain_id,
        &_config.rpc,
        &_config.full_node_url,
        &_config.denom,
        &_config.account_prefix,
        10000,
        "juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf",
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        None,
    )
    .await;
    // deliver_tx failed: TxResult { code: Err(5), data: None, log: Log("\ngithub.com/CosmWasm/wasmd/x/wasm/keeper.Keeper.execute\n\tgithub.com/CosmWasm/wasmd@v0.27.0/x/wasm/keeper/keeper.go:364\ngithub.com/CosmWasm/wasmd/x/wasm/keeper.PermissionedKeeper.Execute\n\tgithub.com/CosmWasm/wasmd@v0.27.0/x/wasm/keeper/contract_keeper.go:51\ngithub.com/CosmWasm/wasmd/x/wasm/keeper.msgServer.ExecuteContract\n\tgithub.com/CosmWasm/wasmd@v0.27.0/x/wasm/keeper/msg_server.go:93\ngithub.com/CosmWasm/wasmd/x/wasm/types._Msg_ExecuteContract_Handler.func1\n\tgithub.com/CosmWasm/wasmd@v0.27.0/x/wasm/types/tx.pb.go:849\ngithub.com/cosmos/cosmos-sdk/baseapp.(*MsgServiceRouter).RegisterService.func2.1\n\tgithub.com/cosmos/cosmos-sdk@v0.45.4/baseapp/msg_service_router.go:113\ngithub.com/CosmWasm/wasmd/x/wasm/types._Msg_ExecuteContract_Handler\n\tgithub.com/CosmWasm/wasmd@v0.27.0/x/wasm/types/tx.pb.go:851\ngithub.com/cosmos/cosmos-sdk/baseapp.(*MsgServiceRouter).RegisterService.func2\n\tgithub.com/cosmos/cosmos-sdk@v0.45.4/baseapp/msg_service_router.go:117\ngithub.com/cosmos/cosmos-sdk/baseapp.(*BaseApp).runMsgs\n\tgithub.com/cosmos/cosmos-sdk@v0.45.4/baseapp/baseapp.go:736\ngithub.com/cosmos/cosmos-sdk/baseapp.(*BaseApp).runTx\n\tgithub.com/cosmos/cosmos-sdk@v0.45.4/baseapp/baseapp.go:693\ngithub.com/cosmos/cosmos-sdk/baseapp.(*BaseApp).DeliverTx\n\tgithub.com/cosmos/cosmos-sdk@v0.45.4/baseapp/abci.go:276\ngithub.com/tendermint/tendermint/abci/client.(*localClient).DeliverTxAsync\n\tgithub.com/tendermint/tendermint@v0.34.19/abci/client/local_client.go:93\ngithub.com/tendermint/tendermint/proxy.(*appConnConsensus).DeliverTxAsync\n\tgithub.com/tendermint/tendermint@v0.34.19/proxy/app_conn.go:85\ngithub.com/tendermint/tendermint/state.execBlockOnProxyApp\n\tgithub.com/tendermint/tendermint@v0.34.19/state/execution.go:320\ngithub.com/tendermint/tendermint/state.(*BlockExecutor).ApplyBlock\n\tgithub.com/tendermint/tendermint@v0.34.19/state/execution.go:140\ngithub.com/tendermint/tendermint/consensus.(*State).finalizeCommit\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:1655\ngithub.com/tendermint/tendermint/consensus.(*State).tryFinalizeCommit\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:1564\ngithub.com/tendermint/tendermint/consensus.(*State).enterCommit.func1\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:1499\ngithub.com/tendermint/tendermint/consensus.(*State).enterCommit\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:1537\ngithub.com/tendermint/tendermint/consensus.(*State).addVote\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:2151\ngithub.com/tendermint/tendermint/consensus.(*State).tryAddVote\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:1949\ngithub.com/tendermint/tendermint/consensus.(*State).handleMsg\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:856\ngithub.com/tendermint/tendermint/consensus.(*State).receiveRoutine\n\tgithub.com/tendermint/tendermint@v0.34.19/consensus/state.go:763\nfailed to execute message; message index: 0: Unauthorized: execute wasm contract failed"), info: Info(""), gas_wanted: Gas(2000000), gas_used: Gas(136934), events: [Event { type_str: "coin_spent", attributes: [Tag { key: Key("spender"), value: Value("juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a") }, Tag { key: Key("amount"), value: Value("2000000ujunox") }] }, Event { type_str: "coin_received", attributes: [Tag { key: Key("receiver"), value: Value("juno17xpfvakm2amg962yls6f84z3kell8c5lxtqmvp") }, Tag { key: Key("amount"), value: Value("2000000ujunox") }] }, Event { type_str: "transfer", attributes: [Tag { key: Key("recipient"), value: Value("juno17xpfvakm2amg962yls6f84z3kell8c5lxtqmvp") }, Tag { key: Key("sender"), value: Value("juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a") }, Tag { key: Key("amount"), value: Value("2000000ujunox") }] }, Event { type_str: "message", attributes: [Tag { key: Key("sender"), value: Value("juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a") }] }, Event { type_str: "tx", attributes: [Tag { key: Key("fee"), value: Value("2000000ujunox") }] }, Event { type_str: "tx", attributes: [Tag { key: Key("acc_seq"), value: Value("juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a/7") }] }, Event { type_str: "tx", attributes: [Tag { key: Key("signature"), value: Value("0L/dWmc2BtaBOFzqx8FWH8gVgOFFeES9KzsSZKclJ58JhsVjM8ekbR6uPLfaHMDxzKuI3AAd/Mc9FHzjYbtmNQ==") }] }], codespace: Codespace("wasm") }
}

#[ignore]
#[tokio::test]
async fn test_execute_increment() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password)
        .unwrap()
        .into();

    let msg = json!({
        "increment": {"count": 5u64}
    });

    let _result = execute::send_execute(
        &sender_private_key,
        &_config.chain_id,
        &_config.rpc,
        &_config.full_node_url,
        &_config.denom,
        &_config.account_prefix,
        10000,
        "juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf",
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        None,
    )
    .await;
    // [{"events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"amount","value":"10000ujunox"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"amount","value":"10000ujunox"}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"amount","value":"10000ujunox"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"method","value":"try_increment"},{"key":"count","value":"105"}]}]}]
}

#[ignore]
#[tokio::test]
async fn test_execute_reset() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password)
        .unwrap()
        .into();

    let msg = json!({
        "reset": {"count": 50u64}
    });

    let _result = execute::send_execute(
        &sender_private_key,
        &_config.chain_id,
        &_config.rpc,
        &_config.full_node_url,
        &_config.denom,
        &_config.account_prefix,
        10000,
        "juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf",
        serde_json::to_vec(&msg).unwrap(),
        2000000,
        2000000,
        None,
    )
    .await;
    // [{"events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"amount","value":"10000ujunox"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"amount","value":"10000ujunox"}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"amount","value":"10000ujunox"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"method","value":"reset"}]}]}]
}

#[ignore]
#[tokio::test]
async fn test_store_contract() {
    // Sender publickey {"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A+Su6DQKrg16phy/7s6lsJUbfD/RuAKsP2WZUUL6KPoI"}
    // Sender account id juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a
    // Mnemonic "youth amused accident boring boss sniff solid inmate small body slow surround survey have rough pill risk ankle extra useful slush junk rally slogan"
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password)
        .unwrap()
        .into();

    let code_id = deploy::store_contract(
        &sender_private_key,
        "../artifacts/simple_counter.wasm",
        &_config.rpc,
        &_config.full_node_url,
        &_config.chain_id,
        &_config.denom,
        Some("test memo"),
        20000000,
        20000000,
        &_config.account_prefix,
    )
    .await
    .unwrap();

    println!("{}", code_id);
    // [{"events":[{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgStoreCode"},{"key":"module","value":"wasm"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"}]},{"type":"store_code","attributes":[{"key":"code_id","value":"2924"}]}]}]
    // code_id = 2924
}

#[ignore]
#[tokio::test]
async fn test_instantiate_contract() {
    let full_path = format!(
        "{}{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        "/test_config_example.json"
    );
    let _config = Config::read_from_path(full_path);

    let sender_private_key = mnemonic_to_private_key(_config.mnemonic, &_config.password)
        .unwrap()
        .into();
    let (_, sender_account_id) =
        private_to_pub_and_account(&sender_private_key, &_config.account_prefix).unwrap();

    let msg = json!({
        "count": 100u64,
        "auth": [sender_account_id.to_string()]
    });

    let contract_address = deploy::instantiate_contract(
        &sender_private_key,
        2924,
        &_config.rpc,
        &_config.full_node_url,
        &_config.chain_id,
        &_config.denom,
        Some("test memo"),
        serde_json::to_vec(&msg).unwrap(),
        20000000,
        20000000,
        &_config.account_prefix,
        10000,
    )
    .await
    .unwrap();

    println!("{}", contract_address);
    // [{"events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"amount","value":"10000ujunox"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"amount","value":"10000ujunox"}]},{"type":"instantiate","attributes":[{"key":"_contract_address","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"code_id","value":"2924"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgInstantiateContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"sender","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"amount","value":"10000ujunox"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf"},{"key":"method","value":"instantiate"},{"key":"auth","value":"juno175ersy4z8pmqqx5pmjgfn7qv4ksxslwq56e89a"},{"key":"count","value":"100"}]}]}]
    // contract_address = juno1cc9juyhhv74uwflnaw3x3h4qy8xglplshlt9fc6h39vfyd8kfjdsggfpvf
}
