use dotenvy::dotenv;
use ethers::abi::Address;
use std::env;

use crate::data_fetchers::AavePortfolioFetcher;
use crate::data_fetchers::defi_llama_data_fetcher::DefiLlamaDataFetcher;
use crate::data_fetchers::eth_chain_data_fetcher::EthChainDataFetcher;
use crate::telegram_bot::TelegramBot;

mod data_fetchers;
mod portfolio_data;
mod telegram_bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let bot_token = env::var("BOT_TOKEN")?;
    let tg_user_id: i64 = env::var("TG_USER_ID")?.parse()?;
    let wallet_address: Address = env::var("ETH_ADDRESS")?.parse()?;
    let node_uri: String = env::var("NODE_URI")?;

    let tg_bot = TelegramBot::new(bot_token.as_str(), tg_user_id);

    let eth_chain_data_fetcher = EthChainDataFetcher::new(node_uri, wallet_address)?;
    let defi_llama_price_fetcher = DefiLlamaDataFetcher::new();

    let portfolio_fetcher =
        AavePortfolioFetcher::new(eth_chain_data_fetcher, Box::new(defi_llama_price_fetcher));

    let portfolio = portfolio_fetcher.fetch_portfolio().await?;
    tg_bot.send_portfolio_notification(&portfolio).await?;

    Ok(())
}
