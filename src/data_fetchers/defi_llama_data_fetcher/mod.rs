#![allow(dead_code)]
use ethers::types::Address;

use super::price_fetcher::PriceFetcher;

mod price_fetcher;

const DEFI_LLAMA_ENDPOINT: &'static str = "https://coins.llama.fi";

pub struct DefiLlamaDataFetcher {}

impl DefiLlamaDataFetcher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl PriceFetcher for DefiLlamaDataFetcher {
    async fn fetch_price_in_usd(&self, token: Address) -> anyhow::Result<f64> {
        price_fetcher::fetch_price(DEFI_LLAMA_ENDPOINT, "ethereum", token).await
    }
}
