use crate::error::{ContractError, ContractResult};
use crate::modules::decimal::DecimalExt;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Coin, CosmosMsg, Decimal, MessageInfo, Response, Uint128,
    WasmMsg,
};
use cw20::{Cw20Coin, Cw20ExecuteMsg, Denom};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[cw_serde]
pub struct Asset {
    pub denom: Denom,
    pub amount: Uint128,
}

impl Asset {
    pub fn is_cw20(&self) -> bool {
        return match self.denom {
            Denom::Native(_) => false,
            Denom::Cw20(_) => true,
        };
    }

    pub fn cw20_address(&self) -> Option<String> {
        return match self.denom.clone() {
            Denom::Native(_) => None,
            Denom::Cw20(address) => Some(address.to_string()),
        };
    }

    pub fn coin(&self) -> Option<Coin> {
        return match self.denom.clone() {
            Denom::Native(denom) => Some(coin(self.amount.into(), denom)),
            Denom::Cw20(_) => None,
        };
    }

    pub fn spendable(mut self, is_classic: bool) -> Self {
        return if is_classic && !self.is_cw20() {
            self.clone()
                .multiply_decimal(&Decimal::from_str("0.995").unwrap())
        } else {
            self
        };
    }
}

impl From<Coin> for Asset {
    fn from(value: Coin) -> Self {
        return Asset {
            denom: Denom::Native(value.denom),
            amount: value.amount,
        };
    }
}

impl From<Cw20Coin> for Asset {
    fn from(value: Cw20Coin) -> Self {
        return Asset {
            denom: Denom::Cw20(Addr::unchecked(value.address)),
            amount: value.amount,
        };
    }
}

pub trait AssetOperation {
    fn divide_decimal(self, decimal: &Decimal) -> Asset;
    fn multiply_decimal(self, decimal: &Decimal) -> Asset;
    fn plus(self, other: &Asset) -> Asset;
    fn minus(self, other: &Asset) -> Asset;
}

impl AssetOperation for Asset {
    fn divide_decimal(mut self, decimal: &Decimal) -> Self {
        self.amount = Decimal::from_num(self.amount).div(decimal).to_uint_floor();
        self
    }

    fn multiply_decimal(mut self, decimal: &Decimal) -> Self {
        self.amount = Decimal::from_num(self.amount).mul(decimal).to_uint_floor();
        self
    }

    fn plus(mut self, other: &Asset) -> Self {
        if other.denom.eq(&other.denom) {
            self.amount = self.amount.add(other.amount);
        }
        self
    }

    fn minus(mut self, other: &Asset) -> Self {
        if other.denom.eq(&other.denom) {
            self.amount = self.amount.sub(other.amount);
        }
        self
    }
}

#[cw_serde]
pub struct Assets {
    pub assets: Vec<Asset>,
}

impl Default for Assets {
    fn default() -> Self {
        Assets {
            assets: vec![],
        }
    }
}

impl Assets {
    pub fn add_asset(mut self, new_asset: Asset) -> Self {
        for asset in self.assets.iter_mut() {
            if asset.denom == new_asset.denom {
                asset.amount += new_asset.amount;
                return self;
            }
        }

        self.assets.push(new_asset);
        self
    }

    pub fn find_denom(&self, denom: &Denom) -> Option<&Asset> {
        self.assets.iter().find(|&asset| asset.denom == *denom)
    }

    pub fn to_transfer_messages(&self, recipient: &String) -> ContractResult<Vec<CosmosMsg>> {
        let mut messages: Vec<CosmosMsg> = vec![];
        let mut coins: Vec<Coin> = vec![];

        for asset in &self.assets {
            match &asset.denom {
                Denom::Native(denom) => {
                    coins.push(Coin {
                        denom: denom.clone(),
                        amount: asset.amount,
                    })
                }
                Denom::Cw20(contract_address) => {
                    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_address.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::Transfer {
                            recipient: recipient.clone(),
                            amount: asset.amount,
                        })?,
                        funds: vec![],
                    });
                    messages.push(transfer_msg);
                }
            }
        }
        if coins.len() > 0 {
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                amount: coins,
                to_address: recipient.clone()
            }))
        }

        Ok(messages)
    }
}

impl From<Vec<Coin>> for Assets {
    fn from(value: Vec<Coin>) -> Self {
        return Assets {
            assets: value.iter().map(|el| Asset::from(el.clone())).collect(),
        };
    }
}
