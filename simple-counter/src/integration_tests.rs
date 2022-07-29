#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            count: 1u64,
            auth: vec![Addr::unchecked(USER)],
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::{CountResponse, ExecuteMsg, QueryMsg};

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Increment { count: 11u64 };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            let res = app.execute(Addr::unchecked(USER), cosmos_msg);
            match res {
                Err(_error) => {}
                _ => panic!("Must return unauthorized error"),
            }

            let msg = ExecuteMsg::Increment { count: 8u64 };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // Query the contract to verify the counter was incremented
            let config_msg = QueryMsg::GetCount {};
            let count_response: CountResponse = app
                .wrap()
                .query_wasm_smart(cw_template_contract.addr(), &config_msg)
                .unwrap();
            assert_eq!(count_response.count, 9);
        }
    }
    mod reset {
        use super::*;
        use crate::msg::{CountResponse, ExecuteMsg, QueryMsg};

        #[test]
        fn reset() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Reset { count: 0u64 };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            let config_msg = QueryMsg::GetCount {};
            let count_response: CountResponse = app
                .wrap()
                .query_wasm_smart(cw_template_contract.addr(), &config_msg)
                .unwrap();
            assert_eq!(count_response.count, 0);
        }
    }
}
