use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;

#[cw_serde]
pub enum ReceiveMsg {
    Burn (Empty),
}