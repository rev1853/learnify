use cosmwasm_std::{DepsMut, Env, Response};
use crate::error::{ContractResult};

pub trait MigrateHandler<T, S> {
    fn handle(
        deps: DepsMut,
        env: Env,
        state: S,
        msg: T,
    ) -> ContractResult<Response>;
}
