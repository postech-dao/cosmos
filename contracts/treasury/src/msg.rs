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
    Verify (VerifyMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance {
        token_address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VerifyMsg {
    message: DeliverableMessage,
    block_height: u64,
    proof: MerkleProof,
}

impl VerifyMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ExecuteMsg::Verify(self);
        to_binary(&msg)
    }
    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<
        T: Into<String>,
        C
    >(self, contract_addr: T) -> StdResult<CosmosMsg<C>>
    where
    C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
        contract_addr: contract_addr.into(),
        msg,
        funds: vec![],
        };
        Ok(execute.into())
    }
}