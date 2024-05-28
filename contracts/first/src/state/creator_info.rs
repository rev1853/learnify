use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct CreatorInfo {
    pub name: String,
    pub email: String,
    pub age: u32,
    pub phone: String
}