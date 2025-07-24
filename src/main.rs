use std::env;
use std::sync::Arc;

use dotenvy::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::aave_portfolio_tracker::AavePortfolioTracker;

mod aave_portfolio_tracker;
mod data_fetchers;
mod portfolio_data;
mod telegram_bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let health_factor_notification_limit = 1.2;
    let aave_portfolio_tracker =
        Arc::new(AavePortfolioTracker::new(health_factor_notification_limit)?);

    let scheduler = JobScheduler::new().await.unwrap();
    let cron_expr = env::var("CRON_EXPR").unwrap_or("0 */5 * * * *".into()); // 5 minutes

    let job = Job::new_async(cron_expr, move |_, _| {
        let worker = aave_portfolio_tracker.clone();
        Box::pin(async move {
            worker.run().await;
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    // Wait while the jobs run
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
