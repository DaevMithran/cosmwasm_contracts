use std::collections::HashMap;

use contract::{execute, query};
use cosmwasm_std::{entry_point, to_json_binary, Deps, QueryResponse, StdResult};
use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response};
use error::ContractError;
use msg::{ExecMsg, QueryMsg};
use state::BOARDS;

mod contract;
mod error;
mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, ContractError> {
    BOARDS.save(deps.storage, &HashMap::new())?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecMsg::Connect { opponent } => execute::connect(deps, info, opponent),
        ExecMsg::Play {
            opponent,
            index,
            entry,
        } => execute::play(deps, info, opponent, index, entry),
    }
}

pub fn query(
    deps: Deps, // not mutable as query cannot mutate
    _env: Env,  // no msg info as query is independent of querier / funds
    info: MessageInfo,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Result { opponent, entry } => {
            to_json_binary(&query::result(deps, info, opponent, entry)?)
        }
        QueryMsg::Board { opponent, entry } => {
            to_json_binary(&query::board(deps, info, opponent, entry)?)
        }
    }
}
