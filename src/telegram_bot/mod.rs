use std::sync::Arc;
use teloxide::prelude::*;
use tokio::sync::Mutex;

use crate::portfolio_data::portfolio::AavePortfolio;

pub struct TelegramBot {
    bot: Arc<Mutex<Bot>>,
    chat_id: ChatId,
}

impl TelegramBot {
    pub fn new(bot_token: &str, user_id: i64) -> Self {
        Self {
            bot: Arc::new(Mutex::new(Bot::new(bot_token))),
            chat_id: ChatId(user_id),
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
        let bot = self.bot.lock().await;
        bot.send_message(self.chat_id, message).send().await?;
        Ok(())
    }
}
