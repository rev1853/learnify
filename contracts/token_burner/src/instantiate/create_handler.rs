use cosmwasm_std::{DepsMut, Env, Event, MessageInfo, Response};
use core::handler::InstantiateHandler;
use core::messages::token_burner::InstantiateMsg;
use crate::state::state::State;
use core::error::{ContractResult};

pub struct CreateHandler {}

impl InstantiateHandler<InstantiateMsg, State<'_>> for CreateHandler {
    fn handle(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        state: State<'_>,
        msg: InstantiateMsg
    ) -> ContractResult<Response> {
        state.token_address.save(deps.storage, &msg.token_address)?;

        let response = Response::new()
            .add_event(Event::new("token-burner"))
            .add_attribute("action", "create-token-burner")
            .add_attribute("token_address", msg.token_address.clone());

        Ok(response)
    }
}

