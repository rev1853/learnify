use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    RegisterToken (RegisterTokenParams),
    RegisterPair (RegisterPairParams)
}


#[cw_serde]
pub struct RegisterTokenParams {
    pub token_address: String,
}

#[cw_serde]
pub struct RegisterPairParams {
    pub pair_address: String
}