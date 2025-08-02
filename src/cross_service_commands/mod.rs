use crate::portfolio::AavePortfolio;

pub enum TrackerToBotCommand {
    NotifyHealthDrop { portfolio: AavePortfolio },
}

pub enum BotToTrackerCommand {
    GetPortfolio,
}
