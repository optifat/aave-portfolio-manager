use std::{env, sync::Arc};

use teloxide::prelude::*;
use tokio::sync::mpsc;

use crate::cross_service_commands::{BotToTrackerCommand, TrackerToBotCommand};
use external_command::ExternalCommand;
use telegram_client::TelegramClient;

mod external_command;
mod telegram_client;

pub fn start_telegram_bot(
    to_tracker_sender: mpsc::Sender<BotToTrackerCommand>,
    from_tracker_receiver: mpsc::Receiver<TrackerToBotCommand>,
) -> anyhow::Result<()> {
    let bot_token: String = env::var("BOT_TOKEN")?;
    let user_id: i64 = env::var("TG_USER_ID")?.parse()?;

    let telegram_bot = Arc::new(TelegramClient::new(&bot_token, user_id, to_tracker_sender));

    start_portfolio_tracker_listener(telegram_bot.clone(), from_tracker_receiver);
    start_external_command_listener(bot_token, telegram_bot);

    Ok(())
}

fn start_external_command_listener(bot_token: String, telegram_bot: Arc<TelegramClient>) {
    log::info!("Starting telegram bot external command listener");
    tokio::spawn(async move {
        ExternalCommand::repl(Bot::new(bot_token), move |msg, cmd| {
            let clone = telegram_bot.clone();
            async move { TelegramClient::answer(&clone, msg, cmd).await }
        })
        .await;
    });
}

fn start_portfolio_tracker_listener(
    telegram_bot: Arc<TelegramClient>,
    mut from_tracker_receiver: mpsc::Receiver<TrackerToBotCommand>,
) {
    log::info!("Starting from tracker to bot command listener");
    tokio::spawn(async move {
        while let Some(message) = from_tracker_receiver.recv().await {
            if let Err(e) = match message {
                TrackerToBotCommand::NotifyHealthDrop { portfolio } => {
                    telegram_bot.send_portfolio_notification(&portfolio).await
                }
            } {
                log::error!("Failed to send telegram notification: {}", e)
            }
        }
    });
}
