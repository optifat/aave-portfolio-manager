use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Debug, Clone, Copy)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub(super) enum TelegramBotExternalCommand {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Get current portfolio.")]
    Portfolio,
}
