use std::ops::Add;
use cosmwasm_std::{Addr, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Response, to_binary, Uint128, WasmMsg};
use cw20::{Cw20ExecuteMsg, Denom};
use crate::core::error::ContractResult;
use crate::core::handler::AssetExecuteHandler;
use crate::core::modules::Assets;
use crate::state::state::State;

pub struct BurnHandler {}

impl AssetExecuteHandler<Empty, State<'_>> for BurnHandler {
    fn handle(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        state: State<'_>,
        _msg: Empty,
        assets: Assets
    ) -> ContractResult<Response> {
        let token_address = state.token_address.load(deps.storage)?;
        let asset_amount = assets.find_denom(&Denom::Cw20(Addr::unchecked(token_address.clone()))).unwrap().amount;

        let burn_total = state.leaderboard
            .load(deps.storage, info.sender.to_string())
            .unwrap_or(Uint128::zero())
            .add(&asset_amount);
        state.leaderboard.save(deps.storage, info.sender.to_string(), &burn_total)?;

        let burn_param = Cw20ExecuteMsg::Burn {
            amount: asset_amount.clone()
        };
        let burn_msg = WasmMsg::Execute {
            msg: to_binary(&burn_param)?,
            funds: vec![],
            contract_addr: token_address.clone()
        };

        let response = Response::new()
            .add_message(CosmosMsg::Wasm(burn_msg))
            .add_attribute("amount", asset_amount.to_string())
            .add_attribute("sender", info.sender.to_string());

        return Ok(response)
    }
}
