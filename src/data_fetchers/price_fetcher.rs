use ethers::types::Address;

#[async_trait::async_trait]
pub trait PriceFetcher {
    async fn fetch_price_in_usd(&self, token: Address) -> anyhow::Result<f64>;
}
