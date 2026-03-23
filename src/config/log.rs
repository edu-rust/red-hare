use serde::Deserialize;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::Other;
use std::sync::OnceLock;

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub log_dir: String,
    pub manifest: String,
}
pub fn load_manifest() -> Result<String, Error> {
    let logging_config = load_log_config()?;
    Ok(logging_config.manifest.clone())
}
pub fn load_log_dir() -> Result<String, Error> {
    let logging_config = load_log_config()?;
    Ok(logging_config.log_dir.clone())
}
static LOG_CONFIG: OnceLock<LoggingConfig> = OnceLock::new();
fn load_log_config() -> Result<&'static LoggingConfig, Error> {
    let config = LOG_CONFIG.get();
    if config.is_some() {
        return Ok(config.unwrap());
    }
    let contents = fs::read_to_string("./config.toml").map_err(|e| Error::new(Other, e))?;
    let config = toml::from_str(&contents).map_err(|e| Error::new(Other, e))?;
    LOG_CONFIG
        .set(config)
        .map_err(|_| Error::new(Other, "failed to set config"))?;
    Ok(LOG_CONFIG.get().unwrap())
}
