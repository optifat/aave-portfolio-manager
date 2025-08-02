use dotenvy::dotenv;

use aave_portfolio_tracker::start_aave_portfolio_tracker;
use cross_service_commands::{BotToTrackerCommand, TrackerToBotCommand};
use logger::init_logger;
use telegram_bot::start_telegram_bot;

mod aave_portfolio_tracker;
mod app_config;
mod common_data;
mod cross_service_commands;
mod data_fetchers;
mod logger;
mod portfolio;
mod telegram_bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::info!("Starting the service");

    dotenv().ok();
    let config = app_config::load_config()?;

    init_logger(config.logging);

    let (bot_tx, tracker_rx) = tokio::sync::mpsc::channel::<BotToTrackerCommand>(8);
    let (tracker_tx, bot_rx) = tokio::sync::mpsc::channel::<TrackerToBotCommand>(8);

    start_telegram_bot(bot_tx, bot_rx)?;
    start_aave_portfolio_tracker(config.aave_portfolio_tracker, tracker_tx, tracker_rx).await?;

    tokio::signal::ctrl_c().await?;
    log::info!("Stopping the service");

    Ok(())
}
