use std::time::{SystemTime, UNIX_EPOCH};

/// 当前unix timestamp(秒)
pub fn current_seconds() -> u64 {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    time.as_secs()
}