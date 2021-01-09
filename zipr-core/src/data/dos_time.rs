#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DosTime(u16);

impl DosTime {
    pub fn from_u16_unchecked(time: u16) -> Self {
        DosTime(time)
    }

    pub fn sec(&self) -> u8 {
        let sec = (self.0 & 0x1F) * 2;
        sec as u8
    }

    pub fn min(&self) -> u8 {
        let min = (self.0 >> 5) & 0x3F;
        min as u8
    }

    pub fn hour(&self) -> u8 {
        let hour = (self.0 >> 11) & 0x1F;
        hour as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

    const TIME1: u16 = 41164;
    #[test]
    fn test_time1_sec() {
        let time = DosTime::from_u16_unchecked(TIME1);
        let sec = time.sec();
        assert_eq!(24, sec);
    }

    #[test]
    fn test_time1_hour() {
        let time = DosTime::from_u16_unchecked(TIME1);
        let hour = time.hour();
        assert_eq!(20, hour);
    }

    #[test]
    fn test_time1_min() {
        let time = DosTime::from_u16_unchecked(TIME1);
        let min = time.min();
        assert_eq!(6, min);
    }
}
