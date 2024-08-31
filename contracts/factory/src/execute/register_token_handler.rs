use cosmwasm_std::{DepsMut, Env, MessageInfo, ReplyOn, Response, SubMsg, to_binary, WasmMsg};
use cw_utils::{must_pay, one_coin, PaymentError};
use crate::core::error::ContractResult;
use crate::core::handler::ExecuteHandler;
use crate::core::messages::factory::RegisterTokenParams;
use crate::core::messages::token_burner::InstantiateMsg;
use crate::core::modules::{Asset, Assets};
use crate::state::state::State;

pub struct RegisterTokenHandler {}

impl ExecuteHandler<RegisterTokenParams, State<'_>> for RegisterTokenHandler {
    fn handle(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        state: State<'_>,
        msg: RegisterTokenParams
    ) -> ContractResult<Response> {
        must_pay(&info, "uluna")?;
        let config = state.config.load(deps.storage)?;
        let coin = one_coin(&info)?;

        if coin.amount != config.register_token_fee {
            return Err(PaymentError::MissingDenom(format!("must pay {} uluna", config.register_token_fee.clone())).into())
        }

        let assets = Assets {
            assets: vec![Asset::from(coin)]
        };

        let instantiate_params = InstantiateMsg {
            token_address: msg.token_address.clone()
        };
        let instantiate_msg = WasmMsg::Instantiate {
            msg: to_binary(&instantiate_params)?,
            funds: vec![],
            label: "Create token burner".to_string(),
            code_id: config.token_burner_code_id.clone(),
            admin: Some(env.contract.address.to_string())
        };

        let response = Response::new()
            .add_submessage(SubMsg {
                msg: instantiate_msg.into(),
                id: crate::reply::register_token_reply_handler::CODE,
                reply_on: ReplyOn::Success,
                gas_limit: None
            })
            .add_messages(assets.to_transfer_messages(&config.owner)?)
            .add_attribute("token_address", msg.token_address.clone());

        Ok(response)
    }
}