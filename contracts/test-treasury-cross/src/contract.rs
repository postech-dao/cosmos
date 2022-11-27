#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_sts::{CosmosMsg, coins, BankMsg, WasmMsg, Uint128, BankQuery}
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg, VerifyQueryMsg, VerifyResponse, BalanceResponse, BalanceAllResponse};

const CONTRACT_NAME: &str = "crates.io:treasury";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const LIGHT_CLIENT_CONTRACT_ADDRESS = "emtpy"

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
        ExecuteMsg::Transfer {recipient, amount, denom, message, block_height, proof} => execute_varified_transfer(deps, _env, info, recipient, amount, denom, message, block_height, proof),
    }
}

fn execute_varified_transfer(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    denom: String,
    message: DeliverableMessage,
    block_height: u64,
    proof: MerkleProof,
) -> Result<Response, ContractError> {

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    // TO DO (Begin)
    let query_msg: VerifyQueryMsg = VerifyQueryMsg::CheckVerify({
        message: message,
        block_height: block_height,
        proof: proof,
    });
    let query_response: VerifyResponse = deps.querier.query(
        &QueryRequest::Wasm(WasmQuery::Smart { 
            contract_addr: LIGHT_CLIENT_CONTRACT_ADDRESS,
            message: to_binary(&query_msg)?,
    }))?;

    if (query_response.is_verified) {
        let amount_int: u128 = amount.parse().unwrap();
        msgs.push(CosmosMsg::Bank(BankMsg::Send{
            to_address: recipient,
            amount: coins(
                amount_int, 
                denom,
            ),
        }));
    } else {
        Err(ContractError::VerifyFail {})
    }

    Ok(
        Response::new()
        .add_attribute("method", "execute_verified_transfer")
        .add_messages(msgs)
    )    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {

    match msg {
        QueryMsg::GetBalance {address, denom} => to_binary(&query_balance(deps, address, denom)?),
        QueryMsg::GetAllBalance {address} => to_binary(&query_all_balance(address)?),
    }
}

fn query_balance(
    deps:Deps,
    address: String, 
    denom: String,
) -> StdResult<Coin> {

    let balanceResponse: BalanceResponse = deps.querier.query_balance(address, denom)?;
    Ok(balanceResponse)
}

fn query_all_balance(
    deps:Deps,
    address: String,
) -> StdResult<Coin> {

    let balanceResponse: BalanceAllResponse = deps.querier.query_all_balance(address)?;
    Ok(balanceResponse)
}