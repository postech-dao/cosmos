#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use pdao_beacon_chain_common::message::DeliverableMessage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetHeaderResponse, InstantiateMsg, QueryMsg};
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
        ExecuteMsg::LightClientUpdate { header, proof } => execute_light_client_update(deps, _env, info, header, proof),
        ExecuteMsg::Transfer {recipient, amount, denom, message, block_height, proof} => execute_transfer(),
    }
}

pub fn execute_light_client_update(
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

    Ok(Response::new().add_attribute("method", "execute_light_client_update"))
}

fn execute_transfer(
    deps: DepsMut<'_>, //ToDo: Wrong Spelling?
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    denom: String,
    message: DeliverableMessage,
    block_height: u64,
    proof: MerkleProof, //ToDo: Must match the data type
) {
    let _result = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if state.light_client.update(header, proof) {
            Ok(state)
        } else {
            Err(ContractError::UpdateFail {})
        }
    })?;

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    let amount_int: u128 = amount.parse().unwrap();
    let success_response: VerifyResponse = to_binary(&query_verify(deps, _env, info, message, block_height, proof)?); //ToDo: not sure
    if success_response.verify_success { //ToDo: not sure
        msgs.push(CosmosMsg::Bank(BankMsg::Send{
            to_address: recipient,
            amount: coins(
                amount_int, 
                denom,
            ),
        }));
    
        Ok(
            Response::new()
            .add_attribute("method", "send_coin_from_to")
            .add_messages(msgs)
        )
    } else {
        return Err(ContractError::NotVerified {});
    }
    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHeader {} => to_binary(&query_header(deps)?),
        QueryMsg::Verify {
            message,
            block_height,
            proof,
        } => to_binary(&query_verify(deps, _env, info, message, block_height, proof)?),
        QueryMsg::GetBalance {address, denom} => to_binary(&query_balance(deps, address, denom)?),
        QueryMsg::GetAllBalance {address} => to_binary(&query_all_balance(address)?),
    }
}

fn query_header(deps: Deps) -> StdResult<GetHeaderResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetHeaderResponse {
        header: state.light_client.last_header,
    })
}

fn query_verify(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    message: DeliverableMessage,
    block_height: u64,
    proof: String,
) -> StdResult<VerifyResponse> {
    let state = STATE.load(deps.storage)?;
    if state.light_client.verify_commitment(message, block_height, proof){
        Ok(VerifyResponse { verify_success: true })
    } else {
        Ok(VerifyResponse { verify_success: false})
    }
}

fn query_balance(
    deps:Deps,
    address: String, 
    denom: String,
) -> StdResult<Coin> {

    let querier = deps.querier.query_balance(address, denom)?;
    Ok(querier)
}

fn query_all_balance(
    deps:Deps,
    address: String,
) -> StdResult<Coin> {

    let querier = deps.querier.query_all_balance(address)?;
    Ok(querier)
}