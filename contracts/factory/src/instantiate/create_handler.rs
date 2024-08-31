use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::core::error::ContractResult;
use crate::core::handler::InstantiateHandler;
use crate::core::messages::factory::InstantiateMsg;
use crate::state::config::Config;
use crate::state::state::State;

pub struct CreateHandler {}

impl InstantiateHandler<InstantiateMsg, State<'_>> for CreateHandler {
    fn handle(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        state: State<'_>,
        msg: InstantiateMsg
    ) -> ContractResult<Response> {
        let config = Config {
            register_token_fee: msg.register_token_fee.clone(),
            owner: msg.owner.clone(),
            register_pair_fee: msg.register_pair_fee.clone(),
            token_burner_code_id: msg.token_burner_code_id.clone()
        };
        state.config.save(deps.storage, &config)?;

        Ok(Response::new())
    }
}