use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the config toml file
    #[arg(short, long, value_name = "CONFIG FILE", default_value = "Config.toml")]
    pub config: PathBuf,
}
