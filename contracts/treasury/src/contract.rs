#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, VerifyMsg};
use cw20_base::contract::{execute_transfer, query_balance};

const CONTRACT_NAME: &str = "crates.io:treasury";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature="library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _envs: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError>{
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer {recipient, amount} => try_execute_transfer(deps, _env, info, recipient, amount),
    }
}

pub fn try_execute_transfer(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    message: DeliverableMessage,
    block_height: u64,
    proof: MerkleProof,
) -> Result<Response, ContractError> {
    let light_client_msg = VerifyMsg {
        message: message,
        block_height: block_height,
        proof: proof,
    };
    
    let processed_msg = light_client_msg.clone().into.cosmos_msg(/* CA */)?;
    /* Light Client에서 Verify 문제 없을 때 */
    if (processed_msg.response == Response.OK) {
        execute_transfer(deps, _env, info, recipient, amount);
    } else {
        Err(ContractError::VerifyFail {})
    }
    Ok(Response::new().add_attribute("method", "execute_verify_and_transfer"))    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {token_address} => to_binary(&query_balance(deps, token_address)?),
    }
}