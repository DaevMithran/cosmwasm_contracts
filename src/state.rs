use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

pub const COUNTER: Item<u128> = Item::new("counter");
pub const MINIMUM_DONATION: Item<Coin> = Item::new("minimum_donation");
pub const OWNER: Item<Addr> = Item::new("owner");
