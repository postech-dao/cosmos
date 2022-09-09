use pdao_beacon_chain_common::message::DeliverableMessage;
use pdao_colony_contract_common::light_client::{BlockFinalizationProof, Header, MerkleProof};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub header: Header,
    pub chain_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Update {
        header: Header,
        proof: BlockFinalizationProof,
    },
    Verify {
        message: DeliverableMessage,
        block_height: u64,
        proof: MerkleProof,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetHeader {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetHeaderResponse {
    pub header: Header,
}
