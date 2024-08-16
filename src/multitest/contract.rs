use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{
    error::ContractError,
    execute, instantiate,
    msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp},
    query,
};

pub struct CountingContract(Addr);

impl CountingContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        contract_id: u64,
        sender: &Addr,
        counter: impl Into<Option<u128>>,
        minimum_donation: Coin,
    ) -> StdResult<Self> {
        let counter = counter.into().unwrap_or_default();
        app.instantiate_contract(
            contract_id,
            sender.clone(),
            &InstantiateMsg {
                counter,
                minimum_donation,
            },
            &[],
            "Counting contract",
            Some(sender.to_string()), // only admins can perform migrations of smart contracts
        )
        .map(|addr| CountingContract(addr))
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn donate(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.addr().clone(),
            &ExecMsg::Donate {},
            funds,
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn increment(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.addr().clone(),
            &ExecMsg::Increment {},
            &[],
        )
        .map(|_| ())
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn withdraw(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.addr().clone(),
            &ExecMsg::Withdraw {},
            funds,
        )
        .map(|_| ())
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn query_value(&self, app: &App) -> StdResult<ValueResp> {
        app.wrap()
            .query_wasm_smart(self.addr().clone(), &QueryMsg::Value {})
    }
}

impl From<CountingContract> for Addr {
    fn from(contract: CountingContract) -> Self {
        contract.0
    }
}
