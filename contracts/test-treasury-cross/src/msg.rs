use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use pdao_cosmos_light_client::msg::{ExecuteMsg}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amout: Uint128,
        denom: String,
    },

    Verify (VerifyMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance {
        address: String,
        denom: String,
    },

    GetAllBalance {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance {
        address: String,
        denom: String,
    },

    GetAllBalance {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VerifyQueryMsg {
    CheckVerify {
        message: DeliverableMessage,
        block_height: u64,
        proof: MerkleProof,
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BalanceResponse {
   pub balance: Coin,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BalanceAllResponse {
   pub balanceAll: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VerifyResponse {
   pub is_verified: bool,
}