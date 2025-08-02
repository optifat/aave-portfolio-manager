use std::env;

use ethers::types::Address;
use tokio::sync::mpsc;

use super::config::AavePortfolioTrackerConfig;
use crate::cross_service_commands::TrackerToBotCommand;
use crate::data_fetchers::AavePortfolioFetcher;
use crate::data_fetchers::defi_llama_data_fetcher::DefiLlamaDataFetcher;
use crate::data_fetchers::eth_chain_data_fetcher::EthChainDataFetcher;

pub(super) struct AavePortfolioTracker {
    pub(super) aave_portfolio_fetcher: AavePortfolioFetcher,
    config: AavePortfolioTrackerConfig,
    to_bot_sender: mpsc::Sender<TrackerToBotCommand>,
}

impl AavePortfolioTracker {
    pub(super) fn new(
        config: AavePortfolioTrackerConfig,
        to_bot_sender: mpsc::Sender<TrackerToBotCommand>,
    ) -> anyhow::Result<Self> {
        let wallet_address: Address = env::var("ETH_ADDRESS")?.parse()?;
        let node_uri: String = env::var("NODE_URI")?;

        log::info!("Starting EthChainDataFetcher");
        let eth_chain_data_fetcher = EthChainDataFetcher::new(node_uri, wallet_address)?;

        log::info!("Starting DefiLlamaDataFetcher");
        let defi_llama_price_fetcher = DefiLlamaDataFetcher::new();

        log::info!("Starting AavePortfolioFetcher");
        let aave_portfolio_fetcher =
            AavePortfolioFetcher::new(eth_chain_data_fetcher, Box::new(defi_llama_price_fetcher));

        Ok(Self {
            aave_portfolio_fetcher,
            config,
            to_bot_sender,
        })
    }

    pub(super) async fn run_scheduled_job(&self) {
        log::info!("Fetching the AAVE v3 portfolio");

        let portfolio = match self.aave_portfolio_fetcher.fetch_portfolio().await {
            Ok(p) => p,
            Err(err) => return log::error!("Failed to fetch portfolio: {}", err),
        };

        if portfolio.health_factor >= self.config.health_factor_notification_limit {
            return log::info!(
                "Health factor is within an acceptable range, skipping notification"
            );
        }

        if let Err(e) = self
            .send_telegram_notification(TrackerToBotCommand::NotifyHealthDrop {
                portfolio: portfolio,
            })
            .await
        {
            return log::info!("Tracker service failed to communicate with bot: {}", e);
        };
    }

    pub(super) async fn send_telegram_notification(
        &self,
        command: TrackerToBotCommand,
    ) -> anyhow::Result<()> {
        self.to_bot_sender.send(command).await?;
        Ok(())
    }
}
