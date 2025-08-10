use std::path::PathBuf;

use serde::Deserialize;

use crate::aave_portfolio_tracker::config::AavePortfolioTrackerConfig;
use crate::logger::LoggingConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub aave_portfolio_tracker: AavePortfolioTrackerConfig,
}

pub fn load_config(config_path: PathBuf) -> anyhow::Result<AppConfig> {
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path))
        .build()?;

    Ok(settings.try_deserialize()?)
}
