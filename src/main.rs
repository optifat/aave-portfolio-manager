use std::sync::Arc;

use dotenvy::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{aave_portfolio_tracker::AavePortfolioTracker, logger::init_logger};

mod aave_portfolio_tracker;
mod app_config;
mod common_data;
mod data_fetchers;
mod logger;
mod portfolio;
mod telegram_bot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = app_config::load_config()?;

    init_logger(config.logging);
    log::info!("Starting the service");

    let aave_portfolio_tracker =
        Arc::new(AavePortfolioTracker::new(config.aave_portfolio_tracker)?);

    let scheduler = JobScheduler::new().await?;
    let job = Job::new_async(config.cron_schedule, move |_, _| {
        let worker = aave_portfolio_tracker.clone();
        Box::pin(async move {
            worker.run().await;
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    tokio::signal::ctrl_c().await?;
    log::info!("Stopping the service");

    Ok(())
}
