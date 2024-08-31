#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::core::error::ContractResult;
use crate::core::handler::InstantiateHandler;
use crate::core::messages::factory::InstantiateMsg;
use crate::instantiate::create_handler::CreateHandler;
use crate::state::state::State;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    return CreateHandler::handle(deps, env, info, State::new(), msg)
}
