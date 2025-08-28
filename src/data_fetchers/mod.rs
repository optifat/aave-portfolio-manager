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

        let aave_collateral_reserves_data = self
            .eth_chain_data_fetcher
            .fetch_aave_reserves_data(&assets_data.collateral)
            .await?;

        let user_a_tokens = aave_collateral_reserves_data
            .iter()
            .map(|x| x.a_token)
            .collect();

        let supply = self
            .eth_chain_data_fetcher
            .fetch_tokens_data(&user_a_tokens)
            .await?;

        log::info!("Fetching debt balances");

        let aave_debt_reserves_data = self
            .eth_chain_data_fetcher
            .fetch_aave_reserves_data(&assets_data.debt)
            .await?;

        let user_debt_tokens = aave_debt_reserves_data
            .iter()
            .map(|x| x.variable_debt_token)
            .collect();

        let debt = self
            .eth_chain_data_fetcher
            .fetch_tokens_data(&user_debt_tokens)
            .await?;

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
