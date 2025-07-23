use dotenvy::dotenv;
use ethers::abi::Address;
use ethers::providers::{Http, Provider};
use std::env;
use std::sync::Arc;
use teloxide::prelude::*;

use crate::aave_portfolio::portfolio::get_portfolio;

mod aave_portfolio;
mod eth_node_requests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::info!("Starting bot...");

    dotenv().ok();
    let bot_token = env::var("BOT_TOKEN")?;
    let tg_user_id: i64 = env::var("TG_USER_ID")?.parse()?;
    let wallet_address: Address = env::var("ETH_ADDRESS")?.parse()?;
    let node_uri: String = env::var("NODE_URI")?;

    let bot = Bot::new(bot_token);
    let provider = Arc::new(Provider::<Http>::try_from(node_uri)?);

    trigger_and_notify(&provider, wallet_address, bot.clone(), tg_user_id).await?;
    Ok(())
}

async fn trigger_and_notify(
    provider: &Arc<Provider<Http>>,
    wallet: Address,
    bot: Bot,
    user_id: i64,
) -> anyhow::Result<()> {
    let portfolio = get_portfolio(provider, wallet).await?;

    let result = bot
        .send_message(ChatId(user_id), serde_json::to_string_pretty(&portfolio)?)
        .send()
        .await;

    if let Err(err) = result {
        eprintln!("Failed to send message: {:?}", err);
    }
    Ok(())
}
