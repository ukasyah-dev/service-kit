use std::str::FromStr;

use env_logger::Builder;
use log::LevelFilter;

use crate::config::Config;

pub fn init(config: Config) {
    let mut level_filter = LevelFilter::Info;

    match LevelFilter::from_str(&config.log_level) {
        Ok(level) => level_filter = level,
        Err(_) => {
            println!(
                "Invalid log level '{}', defaulting to 'info'",
                config.log_level
            );
        }
    }

    Builder::new().filter_level(level_filter).init();
}
