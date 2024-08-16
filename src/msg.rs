// messages module is public for external contract to communicate with it

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u128,
    pub minimum_donation: Coin,
}

#[cw_serde]
pub enum ExecMsg {
    Increment {},
    Donate {},
    Withdraw {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
}

#[cw_serde]
pub struct ValueResp {
    pub value: u128,
}
