use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

pub fn init_logger(logging: LoggingConfig) {
    env_logger::Builder::new()
        .filter_level(logging.level.parse().unwrap_or(log::LevelFilter::Info))
        .init();
}
