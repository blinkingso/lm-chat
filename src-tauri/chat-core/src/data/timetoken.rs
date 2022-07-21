use std::time::{SystemTime, SystemTimeError};

/// It allows clients to resume streaming from where
/// they left off for added resiliency
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub struct TimeToken {
    /// TimeToken
    pub t: u64,
    /// Origin region
    pub r: u32,
}

impl TimeToken {
    pub fn new(time: SystemTime, region: u32) -> Result<Self, SystemTimeError> {
        let time = time.duration_since(SystemTime::UNIX_EPOCH)?;
        let secs = time.as_secs();
        let nanos = time.subsec_nanos();
        let t = (secs * 10_000_000) | (u64::from(nanos) / 100);

        Ok(Self { t, r: region })
    }
}
