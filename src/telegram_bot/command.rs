use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Debug, Clone, Copy)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum TelegramBotExternalCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get current portfolio.")]
    Portfolio,
}
