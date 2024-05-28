use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_schema::serde::Serialize;
use cosmwasm_std::{ContractInfoResponse, Deps, Env, MessageInfo, StdResult};
use crate::error::{ContractError, ContractResult};

pub fn get_contract_info(deps: &Deps, env: &Env) -> StdResult<ContractInfoResponse> {
    return deps.querier.query_wasm_contract_info(env.contract.address.to_string());
}

pub fn is_admin(deps: &Deps, env: &Env, info: &MessageInfo) -> bool {
    if let Ok(contract_info) = get_contract_info(deps, env) {
        if let Some(admin) = contract_info.admin {
            return admin == info.sender.to_string()
        }
    }
    return false;
}

pub fn must_admin(deps: &Deps, env: &Env, info: &MessageInfo) -> ContractResult<bool> {
    if is_admin(deps, env, info) {
        return Ok(true);
    }
    return Err(ContractError::Unauthorized {})
}

pub fn is_classic(env: &Env) -> bool {
    let chain_id = env.block.chain_id.to_owned();
    return chain_id == "columbus-5" || chain_id == "rebel-2";
}

pub fn query<T: DeserializeOwned>(
    deps: &Deps,
    address: &str,
    msg: &impl Serialize,
) -> T {
    return deps.querier.query_wasm_smart::<T>(address, msg).unwrap();
}