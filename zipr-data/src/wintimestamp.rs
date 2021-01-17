use core::convert::{From, TryFrom};
/// Windows timestamp structure.
///
/// Stored internall as a u64.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WinTimestamp(u64);

/// Error type for when a u64 is not a valid
/// wintimestamp
#[derive(Debug)]
pub struct WinTimestampCreateError();

impl TryFrom<u64> for WinTimestamp {
    type Error = WinTimestampCreateError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(WinTimestamp(value))
    }
}

impl From<&WinTimestamp> for u64 {
    fn from(x: &WinTimestamp) -> Self {
        x.0
    }
}
