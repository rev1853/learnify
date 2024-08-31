use core::messages::token_burner::InstantiateMsg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use crate::instantiate::create_handler::CreateHandler;
use crate::state::state::State;
use core::error::{ContractResult};
use core::handler::InstantiateHandler;
use crate::core::handler::{ExecuteHandler, QueryHandler};
use crate::core::messages::token_burner::{ExecuteMsg, QueryMsg};
use crate::execute::receive_handler::ReceiveHandler;
use crate::query::leaderboard_handler::LeaderboardHandler;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    return CreateHandler::handle(deps, env, info, State::new(), msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> ContractResult<Response> {
    let state = State::new();
    return match msg {
        ExecuteMsg::Receive(msg) => ReceiveHandler::handle(deps, env, info, state, msg)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg
) -> ContractResult<Binary> {
    let state = State::new();
    return match msg {
        QueryMsg::Leaderboard(msg) => LeaderboardHandler::handle(deps, env, state, msg)
    }
}