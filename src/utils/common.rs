use std::fs::create_dir_all;
use std::io::Error;
use std::io::ErrorKind::Other;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;

fn is_after_now_with_u128(nanos: u128) -> Result<bool, String> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    Ok(nanos > current_time)
}
pub fn is_after_now(nanos: Option<u128>) -> Result<bool, String> {
    let nanos = match nanos {
        Some(nanos) => nanos,
        None => return Ok(true),
    };
    is_after_now_with_u128(nanos)
}

pub fn add_nanos(nanos: u128) -> Result<u128, String> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();

    current_time
        .checked_add(nanos)
        .ok_or_else(|| "arithmetic overflow occurred when adding nanoseconds".to_string())
}
pub fn ensure_dir_exists(dir: &String) -> Result<(), Error> {
    let path = Path::new(&dir);
    if !path.exists() {
        create_dir_all(path)?;
    }
    if path.is_dir() {
        return Err(Error::new(Other, "log.dir is not a dir"));
    }
    Ok(())
}
