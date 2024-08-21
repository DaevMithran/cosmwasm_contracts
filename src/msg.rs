use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(Copy)]
pub enum Move {
    X,
    Y,
}

#[cw_serde]
pub enum ExecMsg {
    Connect {
        opponent: String,
    },
    Play {
        opponent: String,
        index: u8,
        entry: Move,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Winner)]
    Result { opponent: String, entry: Move },
    #[returns([Option<Move>; 9])]
    Board { opponent: String, entry: Move },
}

#[cw_serde]
pub enum Winner {
    X,
    Y,
    Draw,
}
