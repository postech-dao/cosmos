use pdao_beacon_chain_common::message::DeliverableMessage;
use pdao_colony_contract_common::light_client::{BlockFinalizationProof, Header, MerkleProof};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub header: Header,
    pub chain_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetHeader {},
    CheckVerify {
        message: DeliverableMessage,
        block_height: u64,
        proof: MerkleProof,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetHeaderResponse {
    pub header: Header,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CheckVerifyResponse {
   pub is_verified: bool,
}
