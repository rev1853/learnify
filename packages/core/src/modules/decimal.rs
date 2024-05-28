use cosmwasm_std::{Decimal, Uint128};

pub trait DecimalExt {
    fn from_num(value: impl Into<Uint128>) -> Decimal {
        return Decimal::from_atomics(value, 0).unwrap()
    }
}

impl DecimalExt for Decimal {}