use std::sync::Arc;

use teloxide::{prelude::*, utils::command::BotCommands, ApiError};
use tokio::sync::{mpsc, Mutex};

use crate::{
    commands::{BotCommand, TrackerCommand},
    portfolio::AavePortfolio,
};
use command::TelegramBotExternalCommand;

pub mod command;

pub struct TelegramBot {
    bot: Arc<Bot>,
    chat_id: ChatId,
    to_tracker_sender: mpsc::Sender<TrackerCommand>,
    from_tracker_receiver: Mutex<mpsc::Receiver<BotCommand>>,
}

impl TelegramBot {
    pub async fn new(
        bot_token: &str,
        user_id: i64,
        to_tracker_sender: mpsc::Sender<TrackerCommand>,
        from_tracker_receiver: mpsc::Receiver<BotCommand>,
    ) -> Self {
        log::info!("Starting TelegramBot");

        Self {
            bot: Arc::new(Bot::new(bot_token)),
            chat_id: ChatId(user_id),
            to_tracker_sender,
            from_tracker_receiver: Mutex::new(from_tracker_receiver),
        }
    }

    pub async fn start(&self) {
        while let Some(message) = self.from_tracker_receiver.lock().await.recv().await {
            if let Err(e) = match message {
                BotCommand::NotifyHealthDrop { portfolio } => self.send_portfolio_notification(&portfolio).await
            } {
                log::error!("Failed to send telegram notification: {}", e)
            }
        }
    }

    pub async fn send_portfolio_notification(
        &self,
        portfolio: &AavePortfolio,
    ) -> anyhow::Result<()> {
        log::info!("Notifying the user");
        self.send_message(portfolio.to_telegram_message()).await
    }

    async fn send_message(&self, message: String) -> anyhow::Result<()> {
        self.bot.send_message(self.chat_id, message).send().await?;
        Ok(())
    }

    pub async fn answer(
        &self,
        msg: Message,
        cmd: TelegramBotExternalCommand,
    ) -> ResponseResult<()> {
        let bot = &self.bot;

        if msg.chat.id == self.chat_id {
            if let Err(_) = self.access_answer(cmd).await {
                let error = "Failed to process incoming command";
                log::error!("{}: cmd {:?}", error, cmd);
                return ResponseResult::Err(teloxide::RequestError::Api(ApiError::Unknown(error.into())));
            }
        } else {
            bot.send_message(msg.chat.id, Self::no_access_answer(cmd).await)
                .await?;
        }

        Ok(())
    }

    async fn access_answer(&self, cmd: TelegramBotExternalCommand) -> anyhow::Result<()> {
        match cmd {
            TelegramBotExternalCommand::Help => {
                self.send_message(TelegramBotExternalCommand::descriptions().to_string()).await
            },
            TelegramBotExternalCommand::Portfolio => {
                Ok(self.to_tracker_sender.send(TrackerCommand::GetPortfolio).await?)
            }
        }
    }

    async fn no_access_answer(cmd: TelegramBotExternalCommand) -> String {
        let more_info = " More info: https://github.com/optifat/aave-portfolio-manage";
        match cmd {
            TelegramBotExternalCommand::Help => format!(
                "Current implementation of this bot is for personal usage only.\n {}",
                more_info
            ),
            _ => format!("You have no access!\n {}", more_info),
        }
    }
}
