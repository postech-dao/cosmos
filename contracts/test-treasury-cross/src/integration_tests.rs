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
                        amount: Uint128::new(10),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
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

    mod get_balance {
        use super::*;
        use crate::msg::{BalanceResponse, QueryMsg};

        #[test]
        fn get_balance() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Query the contract to verify the counter was incremented
            let config_msg = QueryMsg::GetBalance {cw_template_contract.addr(), NATIVE_DENOM.to_string()};
            let balance_response: BalanceResponse = app
                .wrap()
                .query_wasm_smart(cw_template_contract.addr(), &config_msg)
                .unwrap();
            assert_eq!(balance_response.balane, 10);
        }
    }

    mod get_all_balance {
        use super::*;
        use crate::msg::{BalanceResponse, QueryMsg};

        #[test]
        fn get_all_balance() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Query the contract to verify the counter was incremented
            let config_msg = QueryMsg::GetBalance {cw_template_contract.addr()};
            let balance_response: BalanceResponse = app
                .wrap()
                .query_wasm_smart(cw_template_contract.addr(), &config_msg)
                .unwrap();
            assert_eq!(true);
        }
    }
}
