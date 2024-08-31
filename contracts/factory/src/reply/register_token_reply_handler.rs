use cosmwasm_std::{DepsMut, Env, Reply, Response};
use crate::core::error::ContractResult;
use crate::core::handler::ReplyHandler;
use crate::core::modules::event::{AttributeHelper, get_event, get_wasm_event};
use crate::state::state::State;

pub const CODE: u64 = 100;

pub struct RegisterTokenReplyHandler {}

impl ReplyHandler<State<'_>> for RegisterTokenReplyHandler {
    fn handle(
        deps: DepsMut,
        _env: Env,
        state: State<'_>,
        msg: Reply
    ) -> ContractResult<Response> {
        let events = msg.result.unwrap().events;
        let event = get_wasm_event(&events, "token-burner");
        let wasm_event = get_event(&events, "wasm");
        let token_burner_address = event.get_attribute("_contract_address").unwrap().value;
        let token_address = wasm_event.get_attribute("token_address").unwrap().value;

        state.tokens.save(deps.storage, token_address, &token_burner_address)?;

        return Ok(Response::new())
    }
}