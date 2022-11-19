#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use pdao_beacon_chain_common::message::DeliverableMessage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetHeaderResponse, CheckVerifyResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use pdao_colony_contract_common::LightClient;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:light-client";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        light_client: LightClient::new(msg.header, msg.chain_name),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

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
        ExecuteMsg::Update { header, proof } => execute_update(deps, _env, info, header, proof),
        ExecuteMsg::Verify {
            message,
            block_height,
            proof,
        } => execute_verify(deps, _env, info, message, block_height, proof),
    }
}

pub fn execute_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    header: String,
    proof: String,
) -> Result<Response, ContractError> {
    let _result = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if state.light_client.update(header, proof) {
            Ok(state)
        } else {
            Err(ContractError::UpdateFail {})
        }
    })?;

    Ok(Response::new().add_attribute("method", "execute_update"))
}

pub fn execute_verify(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    message: DeliverableMessage,
    block_height: u64,
    proof: String,
) -> Result<Response, ContractError> {
    let _result = STATE.update(deps.storage, |state| -> Result<_, ContractError> {
        if state
            .light_client
            .verify_commitment(message, block_height, proof)
        {
            Ok(state)
        } else {
            Err(ContractError::VerifyFail {})
        }
    })?;

    Ok(Response::new().add_attribute("method", "execute_verify"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHeader {} => to_binary(&query_header(deps)?),
        QueryMsg::CheckVerify {
            message,
            block_height,
            proof,
        } => to_binary(&query_verify(deps, _env, info, message, block_height, proof)?), 
    }
}

fn query_header(deps: Deps) -> StdResult<GetHeaderResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetHeaderResponse {
        header: state.light_client.last_header,
    })
}

fn query_verify(deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    message: DeliverableMessage,
    block_height: u64,
    proof: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let is_verified: bool = false;
    if state.light_client.verify_commitment(message, block_height, proof) {
        is_verified = true;
    }

    OK(CheckVerifyResponse {
        is_verified: is_verified,
    })
}
