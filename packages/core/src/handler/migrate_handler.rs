use crate::error::ContractResult;
use cosmwasm_std::{DepsMut, Env, Response};

pub trait MigrateHandler<T, S> {
    fn handle(deps: DepsMut, env: Env, state: S, msg: T) -> ContractResult<Response>;
}
