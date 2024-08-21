pub mod query {
    use cosmwasm_std::{Deps, MessageInfo, StdResult};

    use crate::{
        msg::{Move, Winner},
        state::BOARDS,
    };

    pub fn result(
        deps: Deps,
        info: MessageInfo,
        opponent: String,
        entry: Move,
    ) -> StdResult<Option<Winner>> {
        let boards = BOARDS.load(deps.storage)?;
        let board_name = match entry {
            Move::X => format!("{}:{}", info.sender, opponent),
            Move::Y => format!("{}:{}", opponent, info.sender),
        };

        if let Some(state) = boards.get(&board_name) {
            Ok(state.winner.clone())
        } else {
            Ok(None)
        }
    }

    pub fn board(
        deps: Deps,
        info: MessageInfo,
        opponent: String,
        entry: Move,
    ) -> StdResult<Option<[Option<Move>; 9]>> {
        let boards = BOARDS.load(deps.storage)?;
        let board_name = match entry {
            Move::X => format!("{}:{}", info.sender, opponent),
            Move::Y => format!("{}:{}", opponent, info.sender),
        };

        if let Some(state) = boards.get(&board_name) {
            Ok(Some(state.board.clone()))
        } else {
            Ok(None)
        }
    }
}

pub mod execute {
    use cosmwasm_std::{DepsMut, MessageInfo, Response};

    use crate::{
        error::ContractError,
        msg::Move,
        state::{State, BOARDS},
    };

    pub fn connect(
        deps: DepsMut,
        info: MessageInfo,
        opponent: String,
    ) -> Result<Response, ContractError> {
        let mut boards = BOARDS.load(deps.storage)?;
        boards.insert(
            format!("{}:{}", info.sender, opponent).to_string(),
            State {
                winner: None,
                board: [None; 9],
            },
        );
        BOARDS.save(deps.storage, &boards)?;
        Ok(Response::new())
    }

    pub fn play(
        deps: DepsMut,
        info: MessageInfo,
        opponent: String,
        index: u8,
        entry: Move,
    ) -> Result<Response, ContractError> {
        let mut boards = BOARDS.load(deps.storage)?;
        let board_name = match entry {
            Move::X => format!("{}:{}", info.sender, opponent),
            Move::Y => format!("{}:{}", opponent, info.sender),
        };
        if let Some(state) = boards.get_mut(&board_name) {
            if let Some(_) = &state.board[index as usize] {
                return Err(ContractError::InvalidMove { index });
            }

            let mut count = 0;
            for i in state.board.into_iter() {
                match i {
                    Some(_) => count += 1,
                    None => continue,
                }
            }

            if count % 2 == 0 && entry == Move::X || count % 2 != 0 && entry == Move::Y {
                state.board[index as usize] = Some(entry);
                // calculate and update winner if needed
            } else {
                return Err(ContractError::InvalidTurn {
                    player: info.sender.to_string(),
                });
            }
        } else {
            return Err(ContractError::NotConnected {
                player: info.sender.to_string(),
                opponent,
            });
        }
        Ok(Response::new())
    }
}
