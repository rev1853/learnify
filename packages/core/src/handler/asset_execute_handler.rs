use crate::error::ContractResult;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::modules::Assets;

pub trait AssetExecuteHandler<T, S> {
    fn handle(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        state: S,
        msg: T,
        assets: Assets,
    ) -> ContractResult<Response>;
}
