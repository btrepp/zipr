#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WinTimestamp(u64);
#[derive(Debug)]
pub struct WinTimestampError();

impl WinTimestamp {
    pub fn from_u64_unchecked(time: u64) -> Self {
        WinTimestamp(time)
    }

    pub fn from_u64(time: u64) -> Result<Self, WinTimestampError> {
        Ok(Self::from_u64_unchecked(time))
    }
}
