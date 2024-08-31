use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

pub struct State<'a> {
    pub token_address: Item<'a, String>,
    pub leaderboard: Map<'a, String, Uint128>,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        return Self {
            token_address: Item::new("CONFIG"),
            leaderboard: Map::new("LEADERBOARD"),
        };
    }
}
