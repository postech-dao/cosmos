use async_trait::async_trait;
use cosmwasm_std::from_binary;
use cosmwasm_std::testing::{mock_dependencies, mock_env};
use pdao_colony_common::*;
use pdao_colony_contract_common::*;
use pdao_cosmos_interact::{execute, query};
use pdao_cosmos_light_client::query; //needs to be change
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;


use serde_json::json;
use std::collections::HashMap;

pub struct Juno {
    pub full_node_url: String,
    pub rpc_url: String,
    pub treasury_address: String,
    pub lightclient_address: String,
    pub relayer_account: String,

    pub chain_id: String,
    pub denom: String,
    pub mnemonic: String,
    pub password: String,
    pub account_prefix: String,
}

#[async_trait]
impl ColonyChain for Juno {
    async fn get_chain_name(&self) -> String {
        "juno".to_owned()
    }

    async fn get_last_block(&self) -> Result<Block, Error> {
        let height = query::get_latest_block_height(&self.rpc_url).await.unwrap();
        let timestamp = query::get_latest_block_timestamp(&self.rpc_url)
            .await
            .map_err(|_| Error::ConnectionError("test".to_string()))?;

        Ok(Block { height, timestamp })
    }

    async fn check_connection(&self) -> Result<(), Error> {
        let _height = query::get_latest_block_height(&self.rpc_url)
            .await
            .map_err(|_| Error::ConnectionError("test".to_string()))?;
            
        Ok(())
    }

    async fn get_contract_list(&self) -> Result<Vec<ContractInfo>, Error> {
        Ok(vec![
            ContractInfo {
                address: self.lightclient_address.clone(),
                contract_type: ContractType::LightClient,
                sequence: 0,
            },
            ContractInfo {
                address: self.treasury_address.clone(),
                contract_type: ContractType::Treasury,
                sequence: 0,
            },
        ])
    }

    async fn get_relayer_account_info(&self) -> Result<(String, Decimal), Error> {
        let balance_str = pdao_cosmos_interact::query::get_balance_amount(&self.full_node_url, &self.relayer_account)
            .await
            .map_err(|_| Error::ConnectionError("test".to_string()))?
            .as_str();
        let balance = Decimal::from_str(balance_str).map_err(|_| Error::ConnectionError("test".to_string()))?;

        Ok((self.relayer_account, balance))
    }

    async fn get_light_client_header(&self) -> Result<Header, Error> {
        let mut deps = mock_dependencies();
        let res = pdao_cosmos_light_client::query(deps.as_ref(), mock_env(), QueryMsg::GetHeader {})?;
        let response: GetHeaderResponse = from_binary(&res).map_err(|_| pdao_colony_common::Error::ConnectionError("test".to_string()));

        Ok(response.header.to_owned())
    }

    async fn get_treasury_fungible_token_balance(&self) -> Result<HashMap<String, Decimal>, Error> {
        Ok(vec![
            ("Bitcoin".to_owned(), dec!(123.45)),
            ("Ether".to_owned(), dec!(444.44)),
        ]
        .into_iter()
        .collect())
    }

    async fn get_treasury_non_fungible_token_balance(
        &self,
    ) -> Result<Vec<(String, String)>, Error> {
        Ok(vec![
            ("BAYC".to_owned(), "1".to_owned()),
            ("Sandbox Land".to_owned(), "2".to_owned()),
        ])
    }

    async fn update_light_client(&self, _message: LightClientUpdateMessage) -> Result<(), Error> {
        let msg = json!({
            "update": {
                "header": _message.header,
                "proof": _message.proof
            }
        });
        
        let result = execute::send_execute(
            self.mnemonic.clone(),
            &self.password,
            &self.chain_id,
            &self.rpc_url,
            &self.full_node_url,
            &self.denom,
            &self.account_prefix,
            10000,
            &self.lightclient_address,
            serde_json::to_vec(&msg).unwrap(),
            2000000,
            2000000,
            None,
        ).await
        .map_err(|_| pdao_colony_common::Error::TransactionRejected("test".to_string()));
        
        result
    }

    async fn transfer_treasury_fungible_token(
        &self,
        _message: FungibleTokenTransferMessage,
        _block_height: u64,
        _proof: MerkleProof,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn transfer_treasury_non_fungible_token(
        &self,
        _message: NonFungibleTokenTransferMessage,
        _block_height: u64,
        _proof: MerkleProof,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn deliver_custom_order(
        &self,
        _contract_name: &str,
        _message: CustomMessage,
        _block_height: u64,
        _proof: MerkleProof,
    ) -> Result<(), Error> {
        Ok(())
    }
}
