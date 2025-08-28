#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;

use ethers::{
    abi::Address,
    providers::{Http, Provider},
};

use aave::{AaveReserveData, AaveUserData};

use crate::portfolio::ERC20Balance;

mod aave;
mod erc20;

pub struct EthChainDataFetcher {
    provider: Arc<Provider<Http>>,
    wallet: Address,
}

impl EthChainDataFetcher {
    pub fn new(node_uri: String, wallet: Address) -> anyhow::Result<Self> {
        Ok(Self {
            provider: Arc::new(Provider::<Http>::try_from(node_uri)?),
            wallet,
        })
    }

    pub async fn fetch_tokens_balances(
        &self,
        tokens: &Vec<Address>,
    ) -> anyhow::Result<HashMap<String, ERC20Balance>> {
        erc20::get_erc20_user_balances(self.provider.clone(), self.wallet, tokens, None).await
    }

    pub async fn fetch_user_aave_data(&self) -> anyhow::Result<AaveUserData> {
        aave::get_user_aave_data(self.provider.clone(), self.wallet).await
    }

    pub async fn fetch_user_aave_reserves(&self) -> anyhow::Result<aave::UserAaveTokens> {
        aave::get_user_reserves(self.provider.clone(), self.wallet).await
    }

    pub async fn fetch_aave_reserves_data(
        &self,
        tokens: &Vec<Address>,
    ) -> anyhow::Result<Vec<AaveReserveData>> {
        aave::get_aave_reserve_data(self.provider.clone(), tokens, None).await
    }
}
