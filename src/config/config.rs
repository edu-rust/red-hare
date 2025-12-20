use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_rdb_dir: String,
}

pub fn load_config() -> Result<Config, String> {
    let contents = fs::read_to_string("config.toml").map_err(|error| error.to_string())?;
    let config = toml::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(config)
}
