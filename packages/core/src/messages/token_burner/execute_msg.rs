use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Empty, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    Burn (Empty),
}