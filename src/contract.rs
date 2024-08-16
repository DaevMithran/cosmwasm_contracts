pub mod query {
    use crate::{msg::ValueResp, state::COUNTER};
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod execute {
    use crate::{
        error::ContractError,
        state::{COUNTER, MINIMUM_DONATION, OWNER},
    };
    use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, StdResult};

    pub fn increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let value = COUNTER.update(deps.storage, |x| -> StdResult<_> { Ok(x + 1) })?;

        let response = Response::new()
            .add_attribute("counter", value.to_string())
            .add_attribute("sender", info.sender);
        Ok(response)
    }

    pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let min_donation = MINIMUM_DONATION.load(deps.storage)?;

        if let Some(coin) = info
            .funds
            .iter()
            .find(|coin| coin.denom == min_donation.denom && coin.amount >= min_donation.amount)
        {
            COUNTER.update(deps.storage, |x| -> StdResult<_> {
                Ok(x + coin.amount.u128())
            })?;
        };

        let response = Response::new().add_attribute("sender", info.sender);
        Ok(response)
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;

        if info.sender != owner {
            return Err(ContractError::Unauthorized {
                owner: owner.to_string(),
            });
        }

        let balance = deps.querier.query_all_balances(&env.contract.address)?;

        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balance,
        };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }
}
