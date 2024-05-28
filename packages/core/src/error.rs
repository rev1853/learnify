use cosmwasm_std::StdError;
use thiserror::Error;
use cw_utils::PaymentError;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Voucher Invalid")]
    VoucherInvalid {},

    #[error("ActionNotFound")]
    ActionNotFound {},

    #[error("Custom Error val: {0}")]
    CustomError(String),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("{0} value is empty")]
    None(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

impl From<String> for ContractError {
    fn from(value: String) -> Self {
        Self::CustomError(value)
    }
}
