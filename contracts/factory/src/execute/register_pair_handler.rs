
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::{must_pay, one_coin, PaymentError};
use crate::core::error::{ContractResult};
use crate::core::handler::ExecuteHandler;
use crate::core::messages::factory::RegisterPairParams;
use crate::core::modules::{Asset, Assets};
use crate::state::state::State;

pub struct RegisterPairHandler {}

impl ExecuteHandler<RegisterPairParams, State<'_>> for RegisterPairHandler {
    fn handle(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        state: State<'_>,
        msg: RegisterPairParams
    ) -> ContractResult<Response> {
        must_pay(&info, "uluna")?;
        let config = state.config.load(deps.storage)?;
        let coin = one_coin(&info)?;

        if coin.amount != config.register_pair_fee {
            return Err(PaymentError::MissingDenom(format!("must pay {} uluna", config.register_pair_fee.clone())).into())
        }

        let assets = Assets {
            assets: vec![Asset::from(coin)]
        };

        state.pairs.save(deps.storage, msg.pair_address.clone(), &msg.pair_address)?;

        let response = Response::new()
            .add_messages(assets.to_transfer_messages(&config.owner)?);
        Ok(response)
    }
}