#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdResult,
};

use contract::{execute, query};
use error::ContractError;
use msg::{ExecMsg, InstantiateMsg, QueryMsg};
use state::{COUNTER, MINIMUM_DONATION, OWNER};

mod contract;
mod error;
pub mod msg;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;
mod state;

// called when created for the first time
// smart contracts have multiple entry points unlike native binaries
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,     // querying and updating the smart contract state
    _env: Env,         // blockchain state. chain height, id, timestamp, contract address
    info: MessageInfo, // sender and funds
    msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;
    MINIMUM_DONATION.save(deps.storage, &msg.minimum_donation)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecMsg::Increment {} => execute::increment(deps, info),
        ExecMsg::Donate {} => execute::donate(deps, info),
        ExecMsg::Withdraw {} => execute::withdraw(deps, env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, // not mutable as query cannot mutate
    _env: Env,  // no msg info as query is independent of querier / funds
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value(deps)?),
    }
}
