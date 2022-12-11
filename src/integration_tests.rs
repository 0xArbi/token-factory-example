#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::helpers::CwTemplateContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use cosmwasm_std::{Addr, CustomQuery};
    use cw_multi_test::{Contract, ContractWrapper, Executor};

    use schemars::JsonSchema;
    use serde::de::DeserializeOwned;
    use token_bindings::{TokenFactoryMsg, TokenFactoryQuery};
    use token_bindings_test::error::ContractError;
    use token_bindings_test::TokenFactoryApp;

    pub fn contract<C, Q>() -> Box<dyn Contract<C, Q>>
    where
        C: Clone + Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
        Q: CustomQuery + DeserializeOwned + 'static,
        ContractWrapper<
            ExecuteMsg,
            InstantiateMsg,
            QueryMsg,
            ContractError,
            ContractError,
            cosmwasm_std::StdError,
            TokenFactoryMsg,
            TokenFactoryQuery,
        >: Contract<C, Q>,
    {
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

    fn mock_app() -> TokenFactoryApp {
        TokenFactoryApp::new()
    }

    fn proper_instantiate() -> (TokenFactoryApp, CwTemplateContract) {
        let mut app = mock_app();

        let c: Box<dyn Contract<TokenFactoryMsg, TokenFactoryQuery>> = contract();

        let cw_template_id = app.store_code(contract());

        let msg = InstantiateMsg { count: 1i32 };
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
        use cosmwasm_std::{to_binary, CosmosMsg, WasmMsg};

        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Increment {};
            let wasm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: cw_template_contract.addr().to_string(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            });
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), wasm_msg).unwrap();
        }
    }
}
