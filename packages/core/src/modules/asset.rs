use std::str::FromStr;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Decimal, MessageInfo, to_binary, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use crate::error::{ContractError, ContractResult};
use crate::modules::decimal::DecimalExt;
use crate::modules::share::Share;

#[cw_serde]
pub enum Asset {
    Coin(Coin),
    Token {
        address: String,
        amount: Uint128
    }
}

impl Asset {
    pub fn is_cw20(&self) -> bool {
        return match self {
            Asset::Coin(_) => false,
            Asset::Token { .. } => true
        }
    }

    pub fn to_transfer_msg(&self, recipient: String) -> CosmosMsg {
        return if self.is_cw20() {
            CosmosMsg::Wasm(
                WasmMsg::Execute {
                    contract_addr: self.cw20_address().unwrap(),
                    msg: to_binary(&Cw20ExecuteMsg::Transfer {
                        amount: self.amount(),
                        recipient
                    }).unwrap(),
                    funds: vec![],
                }
            )
        } else {
          CosmosMsg::Bank(
              BankMsg::Send {
                  to_address: recipient,
                  amount: vec![self.coin().unwrap()]
              }
          )
        }
    }

    pub fn cw20_address(&self) -> Option<String> {
        return match self {
            Asset::Coin(_) => None,
            Asset::Token { address, .. } => Some(address.to_owned())
        }
    }

    pub fn amount(&self) -> Uint128 {
        return match self {
            Asset::Coin(coin) => coin.amount.to_owned(),
            Asset::Token { amount, .. } => amount.to_owned()
        }
    }

    pub fn coin(&self) -> Option<Coin> {
        return match self {
            Asset::Coin(coin) => Some(coin.to_owned()),
            Asset::Token { .. } => None
        }
    }

    pub fn from_info(info: &MessageInfo) -> Vec<Asset> {
        return info.funds.to_owned().iter().map(|el| Asset::Coin(el.to_owned())).collect()
    }

    pub fn is_match(&self, assets: &Vec<Asset>) -> bool {
        assets.iter().find(|el| self.compare_asset(el)).is_some()
    }

    fn compare_asset(&self, asset: &Asset) -> bool {
        return match (self, asset) {
            (Asset::Coin(coin1), Asset::Coin(coin2)) => coin1.denom == coin2.denom && coin1.amount.eq(&coin2.amount),
            (
                Asset::Token {address: address1, amount: amount1},
                Asset::Token { address: address2, amount: amount2}
            ) => address1 == address2 && amount1.eq(amount2),
            _ => false
        }
    }

    pub fn to_zero(&self) -> Asset {
        return match self {
            Asset::Coin(coin) => Asset::Coin(Coin::new(0, coin.denom.clone())),
            Asset::Token { address, .. } => Asset::Token { address: address.clone(), amount: Uint128::new(0) }
        }
    }

    pub fn is_zero(&self) -> bool {
        return match self {
            Asset::Coin(coin) => coin.amount.is_zero(),
            Asset::Token { amount, .. } => amount.is_zero()
        }
    }
}

pub trait AssetOperation {
    fn divide_decimal(&self, decimal: &Decimal) -> Asset;
    fn multiply_decimal(&self, decimal: &Decimal) -> Asset;
    fn plus(&self, other: &Asset) -> ContractResult<Asset>;
    fn minus(&self, other: &Asset) -> ContractResult<Asset>;
}

impl AssetOperation for Asset {
    fn divide_decimal(&self, decimal: &Decimal) -> Asset {
        return match self {
            Asset::Coin(coin) => {
                let amount = (Decimal::from_num(coin.amount) / decimal).to_uint_floor().u128();
                Asset::Coin(Coin::new(amount, coin.denom.clone()))
            }
            Asset::Token { address, amount } => {
                Asset::Token {
                    address: address.clone(),
                    amount: (Decimal::from_num(amount.clone()) / decimal).to_uint_floor()
                }
            }
        }
    }

    fn multiply_decimal(&self, decimal: &Decimal) -> Asset {
        return match self {
            Asset::Coin(coin) => {
                let amount = (Decimal::from_num(coin.amount) * decimal).to_uint_floor().u128();
                Asset::Coin(Coin::new(amount, coin.denom.clone()))
            }
            Asset::Token { address, amount } => {
                Asset::Token {
                    address: address.clone(),
                    amount: (Decimal::from_num(amount.clone()) * decimal).to_uint_floor()
                }
            }
        }
    }

    fn plus(&self, other: &Asset) -> ContractResult<Asset> {
        return match (self, other) {
            (Asset::Coin(coin1), Asset::Coin(coin2)) => Ok(Asset::Coin(Coin::new(coin1.amount.u128() + coin2.amount.clone().u128(), coin1.denom.clone()))),
            (
                Asset::Token {address, amount: amount1},
                Asset::Token { amount: amount2, ..},
            ) => Ok(Asset::Token { address: address.clone(), amount: amount1.clone() + amount2.clone() }),
            _ => Err(ContractError::CustomError(String::from("Asset must be the same to add")))
        }
    }

    fn minus(&self, other: &Asset) -> ContractResult<Asset> {
        return match (self, other) {
            (Asset::Coin(coin1), Asset::Coin(coin2)) => Ok(Asset::Coin(Coin::new(coin1.amount.u128() - coin2.amount.clone().u128(), coin1.denom.clone()))),
            (
                Asset::Token {address, amount: amount1},
                Asset::Token { amount: amount2, ..},
            ) => Ok(Asset::Token { address: address.clone(), amount: amount1.clone() - amount2.clone() }),
            _ => Err(ContractError::CustomError(String::from("Asset must be the same to subtract")))
        }
    }
}

pub trait AssetsImpl {
    fn match_one(&self, assets: &Vec<Asset>) -> ContractResult<Asset>;
}

impl AssetsImpl for Vec<Asset> {
    fn match_one(&self, assets: &Vec<Asset>) -> ContractResult<Asset> {
        let assets1 = self.to_owned();
        let asset = assets1.iter().find(|el| el.is_match(assets));
        return match asset {
            None => Err(ContractError::None(format!("Required assets: {}", serde_json_wasm::to_string(assets).unwrap()))),
            Some(asset) => Ok(asset.to_owned())
        }
    }
}

pub fn calculate_shares(fund: &Asset, shares: &Vec<Share>) -> (Asset, Vec<CosmosMsg>) {
    let mut amount = fund.to_zero();
    let msgs = shares.iter()
        .filter(|el| !el.percentage.is_zero())
        .map(|el| {
            let share_amount = &fund.multiply_decimal(&el.percentage);
            amount = amount.plus(share_amount).unwrap();
            return share_amount.to_transfer_msg(el.address.clone())
        })
        .collect();
    return (amount, msgs)
}

pub fn spendable_asset(asset: &Asset, is_classic: bool) -> Asset {
    return if is_classic && !asset.is_cw20() {
        asset.multiply_decimal(&Decimal::from_str("0.995").unwrap())
    } else {
        asset.clone()
    }
}