use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Checks if the given timestamp (in nanoseconds since Unix epoch) is after the current time.
///
/// Returns:
/// - `Ok(true)` if the given time is in the future,
/// - `Ok(false)` otherwise.
/// - `Err(String)` if there's an error getting the current system time.
pub fn is_after_now(nanos: u128) -> Result<bool, String> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    Ok(nanos > current_time)
}

/// Adds the given nanoseconds to the current system time and returns the result as nanoseconds since Unix epoch.
///
/// Returns:
/// - `Ok(u128)` with the sum of current time and given nanoseconds,
/// - `Err(String)` if there's an error getting the current system time.
pub fn add_nanos(nanos: u128) -> Result<u128, String> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();

    current_time
        .checked_add(nanos)
        .ok_or_else(|| "arithmetic overflow occurred when adding nanoseconds".to_string())
}
