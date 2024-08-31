use cosmwasm_std::{Addr, DepsMut, Env, from_binary, MessageInfo, Response};
use cw20::Cw20Coin;
use core::handler::ExecuteHandler;
use crate::core::error::{ContractError, ContractResult};
use crate::core::handler::AssetExecuteHandler;
use crate::core::messages::token_burner::{ReceiveMsg, ReceiveParams};
use crate::core::modules::{Asset, Assets};
use crate::execute::burn_handler::BurnHandler;
use crate::state::state::State;


pub struct ReceiveHandler {}

impl ExecuteHandler<ReceiveParams, State<'_>> for ReceiveHandler {
    fn handle(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        state: State<'_>,
        msg: ReceiveParams
    ) -> ContractResult<Response> {
        let token_address = state.token_address.load(deps.storage)?;

        if info.sender.to_string().ne(&token_address) {
            return Err(ContractError::CustomError("Invalid Token".to_string()));
        }

        let new_info = MessageInfo {
            sender: Addr::unchecked(msg.sender),
            funds: vec![]
        };
        let asset = Asset::from(Cw20Coin {
            address: info.sender.to_string(),
            amount: msg.amount
        });
        let assets = Assets::default().add_asset(asset.into());

        let receive_msg: ReceiveMsg = from_binary(&msg.msg)?;

        return match receive_msg {
            ReceiveMsg::Burn(msg) => BurnHandler::handle(deps, env, new_info, state, msg, assets)
        }
    }
}