use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AavePortfolioTrackerConfig {
    pub health_factor_notification_limit: f64,
    pub cron_schedule: String,
}
