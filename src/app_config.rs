use std::path::PathBuf;

use serde::Deserialize;

use crate::aave_portfolio_tracker::config::AavePortfolioTrackerConfig;
use crate::logger::LoggingConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
    pub aave_portfolio_tracker: AavePortfolioTrackerConfig,
}

impl AppConfig {
    pub fn load(config_path: PathBuf) -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::from(config_path))
            .build()?;

        let config = settings.try_deserialize::<AppConfig>()?;
        config.verify()?;
        Ok(config)
    }

    fn verify(&self) -> anyhow::Result<()> {
        self.aave_portfolio_tracker.verify()
    }
}
