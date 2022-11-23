#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, BankMsg, Uint128, Coin, coins};
use cw2::set_contract_version;
use pdao_beacon_chain_common::message::DeliverableMessage;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetHeaderResponse, BalanceResponse InstantiateMsg, QueryMsg};
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
        ExecuteMsg::Transfer {recipient, amount, denom, message, block_height, header, proof} => execute_light_client_update(deps, _env, info, header, proof),
        // ExecuteMsg::Transfer {recipient, amount, denom, message, block_height, header, proof} => execute_transfer(deps, _env, info, recipient, amount, denom, message, block_height, header, proof),
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

// fn execute_transfer(
//     deps: DepsMut<'_>,
//     _env: Env,
//     info: MessageInfo,
//     recipient: String,
//     amount: Uint128,
//     denom: String,
//     message: DeliverableMessage,
//     block_height: u64,
//     header: String,
//     proof: String,
// ) {
//     if amount == Uint128::zero() {
//         Err(ContractError::InvalidZeroAmount {})
//     }

//     let mut msgs: Vec<CosmosMsg> = vec![];

//     let amount_int: u128 = amount.u128();

//     let _result = STATE.update(deps.storage, |state| -> Result<_, ContractError> {
//         if state
//             .light_client
//             .verify_commitment(message, block_height, proof)
//         {
//             msgs.push(CosmosMsg::Bank(BankMsg::Send{
//                 to_address: recipient,
//                 amount: coins(
//                     amount_int, 
//                     denom,
//                 ),
//             }));
        
//             Ok(
//                 Response::new()
//                 .add_attribute("method", "execute_transfer")
//                 .add_messages(msgs)
//             )
//         } else {
//             Err(ContractError::VerifyFail {})
//         }
//     })?;
// }

// fn execute_transfer_(
//     deps: DepsMut<'_>,
//     _env: Env,
//     info: MessageInfo,
//     recipient: String,
//     amount: Uint128,
//     denom: String,
//     message: DeliverableMessage,
//     block_height: u64,
//     header: String,
//     proof: String,
// ) {
//     if amount == Uint128::zero() {
//         return Err(ContractError::InvalidZeroAmount {});
//     }

//     let mut msgs: Vec<CosmosMsg> = vec![];

//     let amount_int: u128 = amount.parse().unwrap();
//     let success_response: bool = query_verify(deps, _env, info, message, block_height, proof)?;
//     if success_response.verify_success {
//         msgs.push(CosmosMsg::Bank(BankMsg::Send{
//             to_address: recipient,
//             amount: coins(
//                 amount_int, 
//                 denom,
//             ),
//         }));
    
//         Ok(
//             Response::new()
//             .add_attribute("method", "send_coin_from_to")
//             .add_messages(msgs)
//         )
//     } else {
//         return Err(ContractError::VerifyFail{});
//     }
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHeader {} => to_binary(&query_header(deps)?),
        QueryMsg::GetBalance {denom} => to_binary(&deps.querier.query_balance(_env.contract.address, denom)?),
        // QueryMsg::GetBalance {denom} => to_binary(&query_balance(deps, _env, denom)?),
        QueryMsg::GetAllBalance {} => to_binary(&deps.querier.query_all_balances(_env.contract.address)?),
        // QueryMsg::GetAllBalance {} => to_binary(&query_all_balance(deps, _env)?),
    }
}

fn query_header(deps: Deps) -> StdResult<GetHeaderResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetHeaderResponse {
        header: state.light_client.last_header,
    })
}

// fn query_balance(
//     deps:Deps,
//     env: Env,
//     denom: String,
// ) -> StdResult<Coin> {

//     let querier = deps.querier.query_balance(env.contract.address, denom)?;
//     Ok(querier)
// }

// fn query_all_balance(
//     deps:Deps,
//     env: Env,
// ) -> StdResult<Coin> {
    
//     let querier = deps.querier.query_all_balances(env.contract.address)?;
//     Ok(querier)
// }

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr};

    // fn get_auth_vec() -> Vec<Addr> {
    //     let mut auth = Vec::new();
    //     let addr1 = Addr::unchecked("Windy");
    //     let addr2 = Addr::unchecked("Gomesy");
    //     auth.push(addr1); // Now it knows: it's Vec<String>
    //     auth.push(addr2);
    // }

    #[test]
    fn query_test(){
        let mut deps = mock_dependencies_with_balance(&coins(123456, "gold"));
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2,"token"));
        let msg = InstantiateMsg{header, chain_name};
        let _res = instantiate(deps.as_mut(), env, info, msg);

        let denom = String::from("gold");

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {denom}).unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(123456, value.amount)
    }
}

//     #[test]

//     fn proper_initialization() {
//         let mut deps = mock_dependencies();
//         let chain_name = "chain name";
//         let header = Header("abc");

//         let msg = InstantiateMsg { header, chain_name };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies();
//         let auth = get_auth_vec();

//         let msg = InstantiateMsg { count: 17, auth };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("Gomesy", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment { count: 2 };
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(19, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies();
//         let auth = get_auth_vec();

//         let msg = InstantiateMsg { count: 17, auth };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("Windy", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
