use cosmrs::{
    dev, rpc,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};

use super::query::{get_account_number, get_sequence_number};
use super::utils::{mnemonic_to_private_key, private_to_pub_and_account};
use std::error::Error;

#[allow(clippy::too_many_arguments)]
pub async fn send_execute(
    mnemonic: String,
    password: &str,
    chain_id: &str,
    rpc_address: &str,
    api_address: &str,
    denom: &str,
    account_id: &str,
    funds: u32,
    contract_address: &str,
    execute_msg: Vec<u8>,
    gas_amount: u32,
    gas_limit: u64,
    tx_memo: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let (sender_public_key, sender_account_id) = {
        let sender_private_key = mnemonic_to_private_key(mnemonic.clone(), password)
            .unwrap()
            .into();
        private_to_pub_and_account(&sender_private_key, account_id)?
    };

    let funds = Coin {
        amount: funds.into(),
        denom: denom.parse()?,
    };

    let msg = cosmrs::cosmwasm::MsgExecuteContract {
        sender: sender_account_id.clone(),
        contract: contract_address.parse()?,
        msg: execute_msg,
        funds: vec![funds.clone()],
    }
    .to_any()?;

    let amount = Coin {
        amount: gas_amount.into(),
        denom: denom.parse()?,
    };

    let chain_id = chain_id.parse()?;
    let sequence_number = get_sequence_number(api_address, sender_account_id.as_ref()).await?;
    let fee = Fee::from_amount_and_gas(amount.clone(), gas_limit);
    let timeout_height = 0u16;

    let tx_body = tx::Body::new(vec![msg], tx_memo.ok_or("test memo")?, timeout_height);
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let account_number = get_account_number(api_address, sender_account_id.as_ref()).await?;
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

    let tx_raw = {
        let sender_private_key = mnemonic_to_private_key(mnemonic, password).unwrap().into();
        sign_doc.sign(&sender_private_key)?
    };

    let rpc_address = rpc_address.to_owned();
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str())?;
    let tx_commit_response =
        rpc::Client::broadcast_tx_commit(&rpc_client, tx_raw.to_bytes()?.into()).await?;

    // check the response
    if tx_commit_response.check_tx.code.is_err() {
        panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
    } else {
        println!("{}", tx_commit_response.check_tx.log);
    }

    if tx_commit_response.deliver_tx.code.is_err() {
        panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
    } else {
        println!("{}", tx_commit_response.deliver_tx.log);
    }

    let tx = dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;
    assert_eq!(&tx_body, &tx.body);
    assert_eq!(&auth_info, &tx.auth_info);

    Ok(())
}
