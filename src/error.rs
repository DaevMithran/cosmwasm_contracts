use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not Connected - Connect with your opponent first")]
    NotConnected { player: String, opponent: String },

    #[error("Invalid Turn - It's not your turn")]
    InvalidTurn { player: String },

    #[error("Invalid Move - Already played")]
    InvalidMove { index: u8 },
}
