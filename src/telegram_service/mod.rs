use std::{env, sync::Arc};

use teloxide::prelude::*;
use tokio::sync::mpsc;

use crate::commands::{BotCommand, TrackerCommand};
use command::TelegramBotExternalCommand;
use telegram_bot::TelegramBot;

mod command;
mod telegram_bot;

pub async fn start_telegram_service(
    to_tracker_sender: mpsc::Sender<TrackerCommand>,
    from_tracker_receiver: mpsc::Receiver<BotCommand>,
) -> anyhow::Result<()> {
    let bot_token: String = env::var("BOT_TOKEN")?;
    let user_id: i64 = env::var("TG_USER_ID")?.parse()?;

    let telegram_bot = Arc::new(
        TelegramBot::new(
            &bot_token,
            user_id,
            to_tracker_sender,
            from_tracker_receiver,
        )
        .await,
    );
    let telegram_bot_clone = telegram_bot.clone();
    tokio::spawn(async move {
        TelegramBotExternalCommand::repl(Bot::new(bot_token), move |msg, cmd| {
            let clone = telegram_bot_clone.clone();
            async move { TelegramBot::answer(&clone, msg, cmd).await }
        })
        .await;
    });

    let telegram_bot_clone = telegram_bot.clone();
    tokio::spawn(async move {
        telegram_bot_clone.start().await;
    });

    Ok(())
}
