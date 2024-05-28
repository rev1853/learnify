use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractResult;
use crate::modules::asset::Asset;

pub trait AssetExecuteHandler<T, S> {
    fn handle(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        state: S,
        msg: T,
        assets: Vec<Asset>
    ) -> ContractResult<Response>;

}