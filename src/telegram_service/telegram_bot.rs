use std::sync::Arc;

use teloxide::{ApiError, prelude::*, utils::command::BotCommands};
use tokio::sync::mpsc;

use super::command::TelegramBotExternalCommand;
use crate::{commands::TrackerCommand, portfolio::AavePortfolio};

pub(super) struct TelegramBot {
    pub(super) bot: Arc<Bot>,
    chat_id: ChatId,
    to_tracker_sender: mpsc::Sender<TrackerCommand>,
}

impl TelegramBot {
    pub(super) fn new(
        bot_token: &str,
        user_id: i64,
        to_tracker_sender: mpsc::Sender<TrackerCommand>,
    ) -> Self {
        log::info!("Starting TelegramBot");

        Self {
            bot: Arc::new(Bot::new(bot_token)),
            chat_id: ChatId(user_id),
            to_tracker_sender,
        }
    }

    pub(super) async fn send_portfolio_notification(
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

    pub(super) async fn answer(
        &self,
        msg: Message,
        cmd: TelegramBotExternalCommand,
    ) -> ResponseResult<()> {
        let bot = &self.bot;

        if msg.chat.id == self.chat_id {
            if let Err(_) = self.access_answer(cmd).await {
                let error = "Failed to process incoming command";
                log::error!("{}: cmd {:?}", error, cmd);
                return ResponseResult::Err(teloxide::RequestError::Api(ApiError::Unknown(
                    error.into(),
                )));
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
                self.send_message(TelegramBotExternalCommand::descriptions().to_string())
                    .await
            }
            TelegramBotExternalCommand::Portfolio => Ok(self
                .to_tracker_sender
                .send(TrackerCommand::GetPortfolio)
                .await?),
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
