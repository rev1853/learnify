use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Empty, Uint128};

#[cw_serde]
pub enum QueryMsg {
    Leaderboard (LeaderboardParams),
}

pub mod leaderboard {
    pub use crate::messages::token_burner::query_msg::{LeaderboardParams, LeaderboardResponse, LeaderboardItem};
}

#[cw_serde]
pub struct LeaderboardResponse {
    pub leaderboard: Vec<LeaderboardItem>
}

#[cw_serde]
pub struct LeaderboardItem {
    pub address: String,
    pub amount: Uint128
}

#[cw_serde]
pub struct LeaderboardParams {
    pub start_after: Option<String>,
    pub limit: Option<u32>
}

