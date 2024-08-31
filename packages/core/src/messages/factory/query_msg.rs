use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Empty, Uint128};

#[cw_serde]
pub enum QueryMsg {
    RegisteredToken (Empty),
    RegisteredPair (Empty),
    Config (Empty)
}

#[cw_serde]
pub struct RegisteredTokensResponse {
    pub tokens: Vec<RegisteredTokenDetail>
}

#[cw_serde]
pub struct RegisteredTokenDetail {
    pub token_address: String,
    pub burn_address: String
}

pub mod registered_token {
    pub use crate::messages::factory::query_msg::{RegisteredTokensResponse, RegisteredTokenDetail};
}

#[cw_serde]
pub struct RegisteredPairResponse {
    pub tokens: Vec<String>
}

#[cw_serde]
pub struct ConfigResponse {
    pub register_burn_fee: Uint128,
    pub register_pair_fee: Uint128,
    pub owner: String,
    pub token_burner_code_id: u64
}