use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub register_token_fee: Uint128,
    pub register_pair_fee: Uint128,
    pub owner: String,
    pub token_burner_code_id: u64,
}