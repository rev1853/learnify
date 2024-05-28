use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct CitizenInfo {
    pub full_name: String,
    pub gender: String,
    pub address: String,
}