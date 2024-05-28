use cosmwasm_std::{DepsMut, Env, Reply, Response};
use crate::error::{ContractResult};

pub trait ReplyHandler<S> {

    fn handle(
        deps: DepsMut,
        env: Env,
        state: S,
        msg: Reply
    ) -> ContractResult<Response>;
}
