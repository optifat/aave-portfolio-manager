use serde::Deserialize;

use crate::{aave_portfolio_tracker::config::AavePortfolioTrackerConfig, logger::LoggingConfig};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub aave_portfolio_tracker: AavePortfolioTrackerConfig,
    pub cron_schedule: String,
}

pub fn load_config() -> anyhow::Result<AppConfig> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    Ok(settings.try_deserialize()?)
}
