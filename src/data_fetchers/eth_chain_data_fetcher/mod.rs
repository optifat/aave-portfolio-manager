use std::sync::Arc;

use ethers::{
    abi::Address,
    providers::{Http, Provider},
};

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
        erc20::get_token_balance(&self.provider, token, self.wallet).await
    }
}
