use crate::error::ContractResult;
use cosmwasm_std::{DepsMut, Env, Reply, Response};

pub trait ReplyHandler<S> {
    fn handle(deps: DepsMut, env: Env, state: S, msg: Reply) -> ContractResult<Response>;
}
