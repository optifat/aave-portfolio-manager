use crate::portfolio::AavePortfolio;

pub enum BotCommand {
    NotifyHealthDrop { portfolio: AavePortfolio },
}

pub enum TrackerCommand {
    GetPortfolio,
}
