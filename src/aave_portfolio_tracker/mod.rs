use std::env;

use ethers::types::Address;

use crate::data_fetchers::AavePortfolioFetcher;
use crate::data_fetchers::defi_llama_data_fetcher::DefiLlamaDataFetcher;
use crate::data_fetchers::eth_chain_data_fetcher::EthChainDataFetcher;
use crate::telegram_bot::TelegramBot;

pub struct AavePortfolioTracker {
    aave_portfolio_fetcher: AavePortfolioFetcher,
    telegram_bot: TelegramBot,
    pub health_factor_notification_limit: f64,
}

impl AavePortfolioTracker {
    pub fn new(health_factor_notification_limit: f64) -> anyhow::Result<Self> {
        log::info!("Parsing env vars");
        let bot_token = env::var("BOT_TOKEN")?;
        let tg_user_id: i64 = env::var("TG_USER_ID")?.parse()?;
        let wallet_address: Address = env::var("ETH_ADDRESS")?.parse()?;
        let node_uri: String = env::var("NODE_URI")?;

        log::info!("Starting TelegramBot");
        let telegram_bot = TelegramBot::new(bot_token.as_str(), tg_user_id);

        log::info!("Starting EthChainDataFetcher");
        let eth_chain_data_fetcher = EthChainDataFetcher::new(node_uri, wallet_address)?;

        log::info!("Starting DefiLlamaDataFetcher");
        let defi_llama_price_fetcher = DefiLlamaDataFetcher::new();

        log::info!("Starting AavePortfolioFetcher");
        let aave_portfolio_fetcher =
            AavePortfolioFetcher::new(eth_chain_data_fetcher, Box::new(defi_llama_price_fetcher));

        Ok(Self {
            aave_portfolio_fetcher,
            telegram_bot,
            health_factor_notification_limit,
        })
    }

    pub async fn run(&self) {
        log::info!("Fetching the portfolio");

        let portfolio = match self.aave_portfolio_fetcher.fetch_portfolio().await {
            Ok(p) => p,
            Err(err) => {
                log::error!("Failed to fetch portfolio: {}", err);
                return;
            }
        };

        if portfolio.health_factor < self.health_factor_notification_limit {
            match self
                .telegram_bot
                .send_portfolio_notification(&portfolio)
                .await
            {
                Err(err) => {
                    log::error!("Failed to notify: {}", err);
                    return;
                }
                Ok(_) => {}
            }
        }
    }
}
