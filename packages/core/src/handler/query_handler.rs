use crate::error::ContractResult;
use cosmwasm_std::{Binary, Deps, Env};

pub trait QueryHandler<T, S> {
    fn handle(deps: Deps, _env: Env, state: S, msg: T) -> ContractResult<Binary>;
}
