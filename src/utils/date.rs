use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn is_after_now(nanos: Option<u128>) -> Result<bool, String> {
    let nanos = match nanos {
        Some(nanos) => nanos,
        None => return Ok(false),
    };
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    Ok(nanos > current_time)
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
