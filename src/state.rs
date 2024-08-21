use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

use crate::msg::{Move, Winner};

#[cw_serde]
pub struct State {
    pub winner: Option<Winner>,
    pub board: [Option<Move>; 9],
}

pub const BOARDS: Item<HashMap<String, State>> = Item::new("board");
