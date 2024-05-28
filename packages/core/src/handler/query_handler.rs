use cosmwasm_std::{Binary, Deps, Env};
use crate::error::ContractResult;

pub trait QueryHandler<T, S> {
    fn handle(
        deps: Deps,
        _env: Env,
        state: S,
        msg: T
    ) -> ContractResult<Binary>;
}
