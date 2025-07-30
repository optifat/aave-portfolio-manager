use ethers::types::Address;

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait PriceFetcher: Send + Sync {
    async fn fetch_price_in_usd(&self, token: Address) -> anyhow::Result<f64>;
}
