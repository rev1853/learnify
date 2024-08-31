use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Empty, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    Receive (ReceiveParams)
}

#[cw_serde]
pub struct ReceiveParams {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}