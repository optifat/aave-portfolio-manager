use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AavePortfolioTrackerConfig {
    pub health_factor_notification_lower_limit: f64,
    pub health_factor_notification_upper_limit: f64,
    pub cron_schedule: String,
}

impl AavePortfolioTrackerConfig {
    pub fn verify(&self) -> anyhow::Result<()> {
        if self.health_factor_notification_lower_limit > self.health_factor_notification_upper_limit
        {
            anyhow::bail!(
                "Wrong aave portfolio config setup: lower limit ({}) is greater that upper limit ({})",
                self.health_factor_notification_lower_limit,
                self.health_factor_notification_upper_limit
            )
        }
        Ok(())
    }
}
