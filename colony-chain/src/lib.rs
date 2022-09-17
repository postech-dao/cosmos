use async_trait::async_trait;
use pdao_colony_common::*;
use pdao_colony_contract_common::*;
use pdao_cosmos_interact::query;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;

pub struct Juno {
    pub full_node_url: String,
    pub rpc_url: String,
    pub treasury_address: String,
    pub lightclient_address: String,
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
            .unwrap();

        Ok(Block { height, timestamp })
    }

    async fn check_connection(&self) -> Result<(), Error> {
        let _height = query::get_latest_block_height(&self.rpc_url).await;
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
        Ok(("0x12341234".to_owned(), dec!(12.34)))
    }

    async fn get_light_client_header(&self) -> Result<Header, Error> {
        Ok("Hmm".to_owned())
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
        Ok(())
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
