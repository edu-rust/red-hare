use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_dir: String,
}

pub fn load_log_dir() -> Result<String, String> {
    let logging_config = load_log_config()?;
    Ok(logging_config.log_dir)
}

fn load_log_config() -> Result<LoggingConfig, String> {
    let contents = fs::read_to_string("../../config.toml").map_err(|error| error.to_string())?;
    let config = toml::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(config)
}
