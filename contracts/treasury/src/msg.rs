use pdao_beacon_chain_common::message::DeliverableMessage;
use pdao_colony_contract_common::light_client::{BlockFinalizationProof, Header};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub header: Header,
    pub chain_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    LightClientUpdate {
        header: Header,
        proof: BlockFinalizationProof,
    },
    Transfer {
        recipient: String,
        amount: Uint128,
        denom: String,
        message: DeliverableMessage,
        block_height: u64,
        proof: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetHeader {},
    GetBalance {
        denom: String,
    },

    GetAllBalances {
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetHeaderResponse {
    pub header: Header,
}
