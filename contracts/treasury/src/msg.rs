use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use pdao_cosmos_light_client::msg::{ExecuteMsg}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amout: Uint128,
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
    GetBalance {
        token_address: String,
    },
}