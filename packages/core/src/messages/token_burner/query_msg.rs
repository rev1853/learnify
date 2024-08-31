use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;

#[cw_serde]
pub enum QueryMsg {
    Leaderboard (LeaderboardParams),

}

#[cw_serde]
pub struct LeaderboardParams {
    pub start_after: Option<String>,
    pub limit: Option<u32>
}