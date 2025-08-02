use std::sync::Arc;

use teloxide::{ApiError, prelude::*, utils::command::BotCommands};
use tokio::sync::mpsc;

use super::external_command::ExternalCommand;
use crate::{cross_service_commands::BotToTrackerCommand, portfolio::AavePortfolio};

pub(super) struct TelegramClient {
    pub(super) bot: Arc<Bot>,
    chat_id: ChatId,
    to_tracker_sender: mpsc::Sender<BotToTrackerCommand>,
}

impl TelegramClient {
    pub(super) fn new(
        bot_token: &str,
        user_id: i64,
        to_tracker_sender: mpsc::Sender<BotToTrackerCommand>,
    ) -> Self {
        log::info!("Starting TelegramClient");

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

    pub(super) async fn answer(&self, msg: Message, cmd: ExternalCommand) -> ResponseResult<()> {
        let bot = &self.bot;

        if msg.chat.id == self.chat_id {
            if let Err(_) = self.access_answer(cmd).await {
                let error = "Failed to process incoming external command";
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

    async fn access_answer(&self, cmd: ExternalCommand) -> anyhow::Result<()> {
        match cmd {
            ExternalCommand::Help => {
                self.send_message(ExternalCommand::descriptions().to_string())
                    .await
            }
            ExternalCommand::Portfolio => Ok(self
                .to_tracker_sender
                .send(BotToTrackerCommand::GetPortfolio)
                .await?),
        }
    }

    async fn no_access_answer(cmd: ExternalCommand) -> String {
        let more_info = "More info: https://github.com/optifat/aave-portfolio-manage";
        match cmd {
            ExternalCommand::Help => format!(
                "Current implementation of this bot is for personal usage only.\n {}",
                more_info
            ),
            _ => format!("You have no access!\n {}", more_info),
        }
    }
}
