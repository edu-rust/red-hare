use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_rdb_path: String,
    pub log_aof_path: String
    // pub log_level: String,
}

pub fn load_log_config() -> Result<LoggingConfig, String> {
    let contents = fs::read_to_string("./config/config.toml").map_err(|error| error.to_string())?;
    let config = toml::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(config)
}
