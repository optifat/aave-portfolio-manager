use dotenvy::dotenv;

use commands::{BotCommand, TrackerCommand};
use logger::init_logger;
use telegram_service::start_telegram_service;

use crate::aave_portfolio_tracker::start_aave_portfolio_tracker;

mod aave_portfolio_tracker;
mod app_config;
mod commands;
mod common_data;
mod data_fetchers;
mod logger;
mod portfolio;
mod telegram_service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::info!("Starting the service");

    dotenv().ok();
    let config = app_config::load_config()?;

    init_logger(config.logging);

    let (bot_tx, tracker_rx) = tokio::sync::mpsc::channel::<TrackerCommand>(8);
    let (tracker_tx, bot_rx) = tokio::sync::mpsc::channel::<BotCommand>(8);

    start_telegram_service(bot_tx, bot_rx)?;
    start_aave_portfolio_tracker(config.aave_portfolio_tracker, tracker_tx, tracker_rx).await?;

    tokio::signal::ctrl_c().await?;
    log::info!("Stopping the service");

    Ok(())
}
