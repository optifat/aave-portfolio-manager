use std::collections::HashMap;

use eth_chain_data_fetcher::EthChainDataFetcher;
use ethers::types::U256;
use price_fetcher::PriceFetcher;

use crate::common_data::{AAVE_ORACLE_BASE_UNIT, AAVE_WAD};
use crate::portfolio::AavePortfolio;

pub mod defi_llama_data_fetcher;
pub mod eth_chain_data_fetcher;
pub mod price_fetcher;

pub struct AavePortfolioFetcher {
    eth_chain_data_fetcher: EthChainDataFetcher,
    #[allow(dead_code)]
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
        log::info!("Fetching user aave assets");
        let assets_data = self
            .eth_chain_data_fetcher
            .fetch_user_aave_reserves()
            .await?;

        log::info!("Fetching supply balances");

        let mut supply = HashMap::new();
        for token in assets_data.collateral {
            let reserve_data = self
                .eth_chain_data_fetcher
                .fetch_aave_reserve_data(token)
                .await?;
            let balance = self
                .eth_chain_data_fetcher
                .fetch_balance(reserve_data.a_token)
                .await?;

            let symbol = self
                .eth_chain_data_fetcher
                .fetch_token_symbol(token)
                .await?;
            supply.insert(symbol, balance);
        }

        log::info!("Fetching debt balances");

        let mut debt = HashMap::new();
        for token in assets_data.debt {
            let reserve_data = self
                .eth_chain_data_fetcher
                .fetch_aave_reserve_data(token)
                .await?;
            let balance = self
                .eth_chain_data_fetcher
                .fetch_balance(reserve_data.variable_debt_token)
                .await?;

            let symbol = self
                .eth_chain_data_fetcher
                .fetch_token_symbol(token)
                .await?;
            debt.insert(symbol, balance);
        }

        log::info!("Fetching combined data");
        let combined_data = self.eth_chain_data_fetcher.fetch_user_aave_data().await?;

        let net =
            Self::u256_cast(combined_data.total_collateral_base - combined_data.total_debt_base)?
                / AAVE_ORACLE_BASE_UNIT as f64;
        let health_factor = Self::u256_cast(combined_data.health_factor)? / AAVE_WAD as f64;
        Ok(AavePortfolio {
            supply,
            debt,
            net,
            health_factor,
        })
    }

    fn u256_cast(input: U256) -> anyhow::Result<f64> {
        if input > U256::from(u128::MAX) {
            anyhow::bail!("Failed to convert {} U256 to u128", input);
        }

        Ok(input.as_u128() as f64)
    }
}
