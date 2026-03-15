use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_rdb_path: String,
    pub log_aof_path: String, // pub log_level: String,
}

pub fn load_aof_path() -> Result<String, String> {
    let logging_config = load_log_config()?;
    Ok(logging_config.log_rdb_path)
}

pub fn load_rdb_path() -> Result<String, String> {
    let logging_config = load_log_config()?;
    Ok(logging_config.log_rdb_path)
}

fn load_log_config() -> Result<LoggingConfig, String> {
    let contents = fs::read_to_string("./config/config.toml").map_err(|error| error.to_string())?;
    let config = toml::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(config)
}
