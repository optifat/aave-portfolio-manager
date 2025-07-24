use std::collections::HashMap;

use anyhow::Context;
use ethers::types::Address;
use serde::{Deserialize, Serialize};

const DEFI_LLAMA_PRICE_FETCH_ROUTE: &'static str = "/prices/current";

#[derive(Debug, Serialize, Deserialize)]
struct PriceResponse {
    coins: HashMap<String, CoinData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinData {
    decimals: u8,
    symbol: String,
    price: f64,
    timestamp: u64,
    confidence: f64,
}

pub(super) async fn fetch_price(
    api_root: &str,
    network: &str,
    token: Address,
) -> anyhow::Result<f64> {
    let key = format!("{}:{:?}", network, token);
    let url = format!("{}{}/{}", api_root, DEFI_LLAMA_PRICE_FETCH_ROUTE, key);
    let response = reqwest::get(&url).await?;
    let price_response = response.json::<PriceResponse>().await?;
    let price = price_response
        .coins
        .get(&key)
        .with_context(|| format!("Failed to get CoinData for {}", key))?
        .price;
    Ok(price.abs())
}
