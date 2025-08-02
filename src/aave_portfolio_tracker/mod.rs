use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::cross_service_commands::{BotToTrackerCommand, TrackerToBotCommand};
use config::AavePortfolioTrackerConfig;
use service::AavePortfolioTracker;

pub mod config;
mod service;

pub async fn start_aave_portfolio_tracker(
    config: AavePortfolioTrackerConfig,
    to_bot_sender: mpsc::Sender<TrackerToBotCommand>,
    from_bot_receiver: mpsc::Receiver<BotToTrackerCommand>,
) -> anyhow::Result<()> {
    let cron_schedule = config.cron_schedule.clone();
    let aave_portfolio_tracker = Arc::new(AavePortfolioTracker::new(config, to_bot_sender)?);

    start_command_listener(aave_portfolio_tracker.clone(), from_bot_receiver);
    start_scheduled_job(cron_schedule, aave_portfolio_tracker).await?;

    Ok(())
}

fn start_command_listener(
    aave_portfolio_tracker: Arc<AavePortfolioTracker>,
    mut from_bot_receiver: mpsc::Receiver<BotToTrackerCommand>,
) {
    log::info!("Starting from bot to tracker command listener");
    tokio::spawn(async move {
        while let Some(message) = from_bot_receiver.recv().await {
            if let Err(e) = match message {
                BotToTrackerCommand::GetPortfolio => {
                    let portfolio = aave_portfolio_tracker
                        .aave_portfolio_fetcher
                        .fetch_portfolio()
                        .await
                        .unwrap();
                    aave_portfolio_tracker
                        .send_telegram_notification(TrackerToBotCommand::NotifyHealthDrop {
                            portfolio: portfolio,
                        })
                        .await
                }
            } {
                log::error!("Failed to send telegram notification: {}", e)
            }
        }
    });
}

async fn start_scheduled_job(
    cron_schedule: String,
    aave_portfolio_tracker: Arc<AavePortfolioTracker>,
) -> anyhow::Result<()> {
    log::info!("Starting tracker scheduled job");
    let scheduler = JobScheduler::new().await?;
    let job = Job::new_async(cron_schedule, move |_, _| {
        let worker = aave_portfolio_tracker.clone();
        Box::pin(async move {
            worker.run_scheduled_job().await;
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    Ok(())
}
