use std::collections::HashMap;

use eth_chain_data_fetcher::EthChainDataFetcher;
use price_fetcher::PriceFetcher;

use crate::portfolio_data::{
    erc20_registry::{a_token_set, variable_debt_token_set},
    portfolio::AavePortfolio,
};

pub mod defi_llama_data_fetcher;
pub mod eth_chain_data_fetcher;
pub mod price_fetcher;

pub struct AavePortfolioFetcher {
    eth_chain_data_fetcher: EthChainDataFetcher,
    price_fetcher: Box<dyn PriceFetcher>,
}

impl AavePortfolioFetcher {
    pub fn new(
        eth_chain_data_fetcher: EthChainDataFetcher,
        price_fetcher: Box<dyn PriceFetcher>,
    ) -> Self {
        Self {
            eth_chain_data_fetcher,
            price_fetcher,
        }
    }

    pub async fn fetch_portfolio(&self) -> anyhow::Result<AavePortfolio> {
        log::info!("Fetching supply balances");

        let mut supply = HashMap::new();
        let mut total_supply = 0.0;
        let mut collateral = 0.0;
        for token in a_token_set() {
            let balance = self
                .eth_chain_data_fetcher
                .fetch_balance(token.address)
                .await?;
            if balance != 0 {
                supply.insert(token.symbol.to_string(), balance);

                let price = self.price_fetcher.fetch_price_in_usd(token.address).await?;
                let balance_f64 = balance as f64 / f64::powi(10.0, token.decimals as i32);
                total_supply += price * balance_f64;
                // TODO: replace const 0.78 with value obtained from somewhere
                collateral += 0.78 * price * balance_f64;
            }
        }

        log::info!("Fetching debt balances");

        let mut debt = HashMap::new();
        let mut total_debt = 0.0;
        for token in variable_debt_token_set() {
            let balance = self
                .eth_chain_data_fetcher
                .fetch_balance(token.address)
                .await?;
            if balance != 0 {
                debt.insert(token.symbol.to_string(), balance);

                let price = self.price_fetcher.fetch_price_in_usd(token.address).await?;
                let balance_f64 = balance as f64 / f64::powi(10.0, token.decimals as i32);
                total_debt += price * balance_f64;
            }
        }

        let net = total_supply - total_debt;
        let health_factor = collateral / total_debt;
        Ok(AavePortfolio {
            supply,
            debt,
            net,
            health_factor,
        })
    }
}
