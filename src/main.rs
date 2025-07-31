use std::{env, sync::Arc};

use dotenvy::dotenv;
use teloxide::{repls::CommandReplExt, Bot};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    aave_portfolio_tracker::AavePortfolioTracker,
    commands::{BotCommand, TrackerCommand},
    logger::init_logger,
    telegram_bot::{command::TelegramBotExternalCommand, TelegramBot},
};

mod aave_portfolio_tracker;
mod app_config;
mod commands;
mod common_data;
mod data_fetchers;
mod logger;
mod portfolio;
mod telegram_bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = app_config::load_config()?;

    init_logger(config.logging);
    log::info!("Starting the service");

    let (bot_tx, tracker_rx) = tokio::sync::mpsc::channel::<TrackerCommand>(8);
    let (tracker_tx, bot_rx) = tokio::sync::mpsc::channel::<BotCommand>(8);

    let bot_token: String = env::var("BOT_TOKEN")?;
    let user_id: i64 = env::var("TG_USER_ID")?.parse()?;

    let telegram_bot = Arc::new(TelegramBot::new(&bot_token, user_id, bot_tx, bot_rx).await);
    let telegram_bot_clone = telegram_bot.clone();
    tokio::spawn(async move {
        TelegramBotExternalCommand::repl(Bot::new(bot_token), move |msg, cmd| {
            let clone = telegram_bot_clone.clone();
            async move { TelegramBot::answer(&clone, msg, cmd).await }
        })
        .await;
    });

    let aave_portfolio_tracker = Arc::new(AavePortfolioTracker::new(
        config.aave_portfolio_tracker,
        tracker_tx,
        tracker_rx,
    )?);

    let telegram_bot_clone = telegram_bot.clone();
    tokio::spawn(async move {
        telegram_bot_clone.start().await;
    });

    let aave_portfolio_tracker_clone = aave_portfolio_tracker.clone();
    tokio::spawn(async move {
        aave_portfolio_tracker_clone.start().await;
    });

    let scheduler = JobScheduler::new().await?;
    let job = Job::new_async(config.cron_schedule, move |_, _| {
        let worker = aave_portfolio_tracker.clone();
        Box::pin(async move {
            worker.run().await;
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    tokio::signal::ctrl_c().await?;
    log::info!("Stopping the service");

    Ok(())
}
