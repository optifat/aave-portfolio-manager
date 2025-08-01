use std::{env, sync::Arc};

use teloxide::prelude::*;
use tokio::sync::mpsc;

use crate::commands::{BotCommand, TrackerCommand};
use command::TelegramBotExternalCommand;
use telegram_bot::TelegramBot;

mod command;
mod telegram_bot;

pub fn start_telegram_service(
    to_tracker_sender: mpsc::Sender<TrackerCommand>,
    from_tracker_receiver: mpsc::Receiver<BotCommand>,
) -> anyhow::Result<()> {
    let bot_token: String = env::var("BOT_TOKEN")?;
    let user_id: i64 = env::var("TG_USER_ID")?.parse()?;

    let telegram_bot = Arc::new(TelegramBot::new(&bot_token, user_id, to_tracker_sender));

    start_portfolio_tracker_listener(telegram_bot.clone(), from_tracker_receiver);
    start_external_command_listener(bot_token, telegram_bot);

    Ok(())
}

fn start_external_command_listener(bot_token: String, telegram_bot: Arc<TelegramBot>) {
    tokio::spawn(async move {
        TelegramBotExternalCommand::repl(Bot::new(bot_token), move |msg, cmd| {
            let clone = telegram_bot.clone();
            async move { TelegramBot::answer(&clone, msg, cmd).await }
        })
        .await;
    });
}

fn start_portfolio_tracker_listener(
    telegram_bot: Arc<TelegramBot>,
    mut from_tracker_receiver: mpsc::Receiver<BotCommand>,
) {
    tokio::spawn(async move {
        while let Some(message) = from_tracker_receiver.recv().await {
            if let Err(e) = match message {
                BotCommand::NotifyHealthDrop { portfolio } => {
                    telegram_bot.send_portfolio_notification(&portfolio).await
                }
            } {
                log::error!("Failed to send telegram notification: {}", e)
            }
        }
    });
}
