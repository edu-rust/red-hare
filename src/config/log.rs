use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_rdb_dir: String,
    // pub log_level: String,
}

pub fn load_config() -> Result<Config, String> {
    let contents = fs::read_to_string("./config/config.toml").map_err(|error| error.to_string())?;
    let config = toml::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(config)
}
// pub fn load_logging_level() -> Result<String, String> {
//     let config = load_config()?;
//     Ok(config.logging.log_level)
// }