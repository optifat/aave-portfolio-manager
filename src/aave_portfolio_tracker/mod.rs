use std::env;

use ethers::types::Address;
use tokio::sync::{mpsc, Mutex};

use crate::commands::{BotCommand, TrackerCommand};
use crate::data_fetchers::AavePortfolioFetcher;
use crate::data_fetchers::defi_llama_data_fetcher::DefiLlamaDataFetcher;
use crate::data_fetchers::eth_chain_data_fetcher::EthChainDataFetcher;
use config::AavePortfolioTrackerConfig;

pub mod config;

pub struct AavePortfolioTracker {
    aave_portfolio_fetcher: AavePortfolioFetcher,
    to_bot_sender: mpsc::Sender<BotCommand>,
    from_bot_receiver: Mutex<mpsc::Receiver<TrackerCommand>>,
    pub config: AavePortfolioTrackerConfig,
}

impl AavePortfolioTracker {
    pub fn new(
        config: AavePortfolioTrackerConfig,
        to_bot_sender: mpsc::Sender<BotCommand>,
        from_bot_receiver: mpsc::Receiver<TrackerCommand>,
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
            from_bot_receiver: Mutex::new(from_bot_receiver),
        })
    }

    pub async fn start(&self) {
        while let Some(message) = self.from_bot_receiver.lock().await.recv().await {
            if let Err(e) = match message {
                TrackerCommand::GetPortfolio => {
                    let portfolio = self.aave_portfolio_fetcher.fetch_portfolio().await.unwrap();
                    self
                        .to_bot_sender
                        .send(BotCommand::NotifyHealthDrop {
                            portfolio: portfolio,
                        }).await
                }
            } {
                log::error!("Failed to send telegram notification: {}", e)
            }
        }
    }

    pub async fn run(&self) {
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
            .to_bot_sender
            .send(BotCommand::NotifyHealthDrop {
                portfolio: portfolio,
            })
            .await
        {
            return log::info!("Tracker service failed to communicate with bot: {}", e);
        };
    }
}
