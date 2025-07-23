use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
};
use serde::{Serialize, Serializer};
use std::{collections::HashMap, sync::Arc};

use crate::{aave_portfolio::token_addresses::variable_debt_token_set, eth_node_requests::erc20};

use super::token_addresses::a_token_set;

#[derive(Serialize)]
pub struct Portfolio {
    #[serde(serialize_with = "serialize_balances")]
    collateral: HashMap<String, U256>,
    #[serde(serialize_with = "serialize_balances")]
    debt: HashMap<String, U256>,
}

pub async fn get_portfolio(
    provider: &Arc<Provider<Http>>,
    wallet: Address,
) -> anyhow::Result<Portfolio> {
    let mut collateral = HashMap::new();
    for (token_symbol, token_address) in a_token_set() {
        let balance = erc20::get_token_balance(provider, *token_address, wallet).await?;
        collateral.insert(token_symbol.to_string(), balance);
    }

    let mut debt = HashMap::new();
    for (token_symbol, token_address) in variable_debt_token_set() {
        let balance = erc20::get_token_balance(provider, *token_address, wallet).await?;
        debt.insert(token_symbol.to_string(), balance);
    }

    Ok(Portfolio { collateral, debt })
}

fn serialize_balances<S>(value: &HashMap<String, U256>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let balance_vec = value
        .iter()
        .map(|(token, balance)| format!("{}: {}", token, balance))
        .collect::<Vec<_>>();
    balance_vec.serialize(serializer)
}
