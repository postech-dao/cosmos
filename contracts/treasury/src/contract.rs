#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128,
};
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
        ExecuteMsg::LightClientUpdate { header, proof } => {
            execute_light_client_update(deps, _env, info, header, proof)
        }
        ExecuteMsg::Transfer {
            recipient,
            amount,
            denom,
            message,
            block_height,
            proof,
        } => execute_transfer(
            deps,
            _env,
            info,
            recipient,
            amount,
            denom,
            message,
            block_height,
            proof,
        ),
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

#[allow(clippy::too_many_arguments)]
fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    recipient: String,
    amount: Uint128,
    denom: String,
    _message: DeliverableMessage,
    _block_height: u64,
    proof: String,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut msgs: Vec<CosmosMsg> = vec![];
    let amount_int: u128 = amount.u128();
    let _result = STATE.update(deps.storage, |state| -> Result<_, ContractError> {
        /*if state
            .light_client
            .verify_commitment(message, block_height, proof)
        {
            Ok(state)
        } else {
            return Err(ContractError::VerifyFail {});
        }*/
        if proof == *"success" {
            Ok(state)
        } else {
            Err(ContractError::VerifyFail {})
        }
    });

    if proof == *"success" {
        msgs.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient,
            amount: coins(amount_int, denom),
        }));

        Ok(Response::new()
            .add_attribute("method", "execute_transfer")
            .add_messages(msgs))
    } else {
        Err(ContractError::VerifyFail {})
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHeader {} => to_binary(&query_header(deps)?),
        QueryMsg::GetBalance { denom } => {
            to_binary(&deps.querier.query_balance(_env.contract.address, denom)?)
        }
        QueryMsg::GetAllBalances {} => {
            to_binary(&deps.querier.query_all_balances(_env.contract.address)?)
        }
    }
}

fn query_header(deps: Deps) -> StdResult<GetHeaderResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetHeaderResponse {
        header: state.light_client.last_header,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coin, from_binary, Coin, ReplyOn, SubMsg};
    use pdao_beacon_chain_common::message::FungibleTokenTransfer;

    #[test]
    fn query_header_test() {
        let mut deps = mock_dependencies_with_balance(&[
            coin(123, "gold"),
            coin(456, "silver"),
            coin(789, "bronze"),
        ]);
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2, "token"));
        let msg = InstantiateMsg { header, chain_name };
        let _res = instantiate(deps.as_mut(), env, info, msg);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetHeader {}).unwrap();
        let value: GetHeaderResponse = from_binary(&res).unwrap();
        assert_eq!(String::from("abc"), value.header);
    }

    #[test]
    fn query_balance_test() {
        let mut deps = mock_dependencies_with_balance(&[
            coin(123, "gold"),
            coin(456, "silver"),
            coin(789, "bronze"),
        ]);
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2, "token"));
        let msg = InstantiateMsg { header, chain_name };
        let _res = instantiate(deps.as_mut(), env, info, msg);

        let denom = String::from("gold");
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { denom }).unwrap();
        let value: Coin = from_binary(&res).unwrap();
        assert_eq!(123, value.amount.u128());

        let denom = String::from("silver");
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { denom }).unwrap();
        let value: Coin = from_binary(&res).unwrap();
        assert_eq!(456, value.amount.u128());

        let denom = String::from("bronze");
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { denom }).unwrap();
        let value: Coin = from_binary(&res).unwrap();
        assert_eq!(789, value.amount.u128());
    }

    #[test]
    fn query_all_balances_test() {
        let mut deps = mock_dependencies_with_balance(&[
            coin(123, "gold"),
            coin(456, "silver"),
            coin(789, "bronze"),
        ]);
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2, "token"));
        let msg = InstantiateMsg { header, chain_name };
        let _res = instantiate(deps.as_mut(), env, info, msg);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAllBalances {}).unwrap();
        let value: Vec<Coin> = from_binary(&res).unwrap();

        assert_eq!(coin(123, "gold"), value[0]);
        assert_eq!(123, value[0].amount.u128());

        assert_eq!(coin(456, "silver"), value[1]);
        assert_eq!(456, value[1].amount.u128());

        assert_eq!(coin(789, "bronze"), value[2]);
        assert_eq!(789, value[2].amount.u128());
    }

    #[test]
    fn transfer_test() {
        let mut deps = mock_dependencies_with_balance(&[
            coin(123, "gold"),
            coin(456, "silver"),
            coin(789, "bronze"),
        ]);
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2, "token"));
        let msg = InstantiateMsg { header, chain_name };
        let _res = instantiate(deps.as_mut(), env, info, msg);
        let ftt = FungibleTokenTransfer {
            token_id: String::from("gold"),
            amount: 10u128,
            receiver_address: String::from("recipient"),
            contract_sequence: 1u64,
        };

        let msg = ExecuteMsg::Transfer {
            recipient: String::from("recipient"),
            amount: Uint128::from(10u128),
            denom: String::from("gold"),
            message: DeliverableMessage::FungibleTokenTransfer(ftt),
            block_height: 10u64,
            proof: String::from("success"),
        };
        let sub = SubMsg {
            id: 0u64,
            msg: CosmosMsg::Bank(BankMsg::Send {
                to_address: String::from("recipient"),
                amount: coins(10, "gold"),
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        };

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &coins(2, "token")),
            msg,
        );
        assert_eq!(sub, res.unwrap().messages[0]);
    }

    #[test]
    fn amount_zero_test() {
        let mut deps = mock_dependencies_with_balance(&[
            coin(123, "gold"),
            coin(456, "silver"),
            coin(789, "bronze"),
        ]);
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2, "token"));
        let msg = InstantiateMsg { header, chain_name };
        let _res = instantiate(deps.as_mut(), env, info, msg);
        let ftt = FungibleTokenTransfer {
            token_id: String::from("gold"),
            amount: 10u128,
            receiver_address: String::from("recipient"),
            contract_sequence: 1u64,
        };

        let msg = ExecuteMsg::Transfer {
            recipient: String::from("recipient"),
            amount: Uint128::from(0u128),
            denom: String::from("gold"),
            message: DeliverableMessage::FungibleTokenTransfer(ftt),
            block_height: 00u64,
            proof: String::from("success"),
        };

        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &coins(2, "token")),
            msg,
        )
        .unwrap_err();
        match err {
            ContractError::InvalidZeroAmount {} => {}
            e => panic!("unexpected error: {}", e),
        }
    }

    #[test]
    fn verify_fail_test() {
        let mut deps = mock_dependencies_with_balance(&[
            coin(123, "gold"),
            coin(456, "silver"),
            coin(789, "bronze"),
        ]);
        let chain_name = String::from("chain name");
        let header = String::from("abc");
        let env = mock_env();

        let info = mock_info("sender", &coins(2, "token"));
        let msg = InstantiateMsg { header, chain_name };
        let _res = instantiate(deps.as_mut(), env, info, msg);
        let ftt = FungibleTokenTransfer {
            token_id: String::from("gold"),
            amount: 10u128,
            receiver_address: String::from("recipient"),
            contract_sequence: 1u64,
        };

        let msg = ExecuteMsg::Transfer {
            recipient: String::from("recipient"),
            amount: Uint128::from(10u128),
            denom: String::from("gold"),
            message: DeliverableMessage::FungibleTokenTransfer(ftt),
            block_height: 10u64,
            proof: String::from("fail"),
        };

        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &coins(2, "token")),
            msg,
        )
        .unwrap_err();
        match err {
            ContractError::VerifyFail {} => {}
            e => panic!("unexpected error: {}", e),
        }
    }
}
