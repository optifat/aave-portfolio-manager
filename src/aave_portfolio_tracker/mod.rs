use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::commands::{BotCommand, TrackerCommand};
use config::AavePortfolioTrackerConfig;
use service::AavePortfolioTracker;

pub mod config;
mod service;

pub async fn start_aave_portfolio_tracker(
    config: AavePortfolioTrackerConfig,
    to_bot_sender: mpsc::Sender<BotCommand>,
    from_bot_receiver: mpsc::Receiver<TrackerCommand>,
) -> anyhow::Result<()> {
    let cron_schedule = config.cron_schedule.clone();
    let aave_portfolio_tracker = Arc::new(AavePortfolioTracker::new(
        config,
        to_bot_sender,
        from_bot_receiver,
    )?);

    let aave_portfolio_tracker_clone = aave_portfolio_tracker.clone();
    tokio::spawn(async move {
        aave_portfolio_tracker_clone.start().await;
    });

    let scheduler = JobScheduler::new().await?;
    let job = Job::new_async(cron_schedule, move |_, _| {
        let worker = aave_portfolio_tracker.clone();
        Box::pin(async move {
            worker.run().await;
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    Ok(())
}
