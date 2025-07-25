#![allow(dead_code)]
use std::sync::Arc;

use ethers::{
    abi::Address,
    providers::{Http, Provider},
};

use crate::data_fetchers::eth_chain_data_fetcher::aave::AaveUserData;

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

    pub async fn fetch_balance(&self, token: Address) -> anyhow::Result<u128> {
        erc20::get_token_balance(self.provider.clone(), token, self.wallet).await
    }

    pub async fn fetch_token_decimals(&self, token: Address) -> anyhow::Result<u8> {
        erc20::get_token_decimals(self.provider.clone(), token).await
    }

    pub async fn fetch_token_symbol(&self, token: Address) -> anyhow::Result<String> {
        erc20::get_token_symbol(self.provider.clone(), token).await
    }

    pub async fn fetch_user_aave_data(&self) -> anyhow::Result<AaveUserData> {
        aave::get_user_aave_data(self.provider.clone(), self.wallet).await
    }

    pub async fn fetch_user_aave_reserves(&self) -> anyhow::Result<aave::UserAaveTokens> {
        aave::get_user_reserves(self.provider.clone(), self.wallet).await
    }
}
