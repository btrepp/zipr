use core::fmt::Debug;

#[derive(PartialEq, Copy, Clone)]
pub struct DosDate(u16);

impl DosDate {
    pub fn from_u16_unchecked(date: u16) -> Self {
        DosDate(date)
    }

    pub fn as_bytes(&self) -> u16 {
        self.0
    }

    pub fn year(&self) -> u16 {
        let year = (self.0 >> 9) + 1980;
        year as u16
    }

    pub fn month(&self) -> u8 {
        let mut month = (self.0 >> 5) & 0x0F;

        if month == 0 {
            month = 1
        }
        month as u8
    }

    pub fn day(&self) -> u8 {
        let mut day = self.0 & 0x1F;

        if day == 0 {
            day = 1
        }
        day as u8
    }
}

impl Debug for DosDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DosDate")
            .field("year", &self.year())
            .field("month", &self.month())
            .field("day", &self.day())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

    const DATE1: u16 = 20867;
    #[test]
    fn test_date1_year() {
        let date = DosDate::from_u16_unchecked(DATE1);
        let year = date.year();
        assert_eq!(2020, year);
    }

    #[test]
    fn test_date1_month() {
        let date = DosDate::from_u16_unchecked(DATE1);
        let month = date.month();
        assert_eq!(12, month);
    }

    #[test]
    fn test_date1_day() {
        let date = DosDate::from_u16_unchecked(DATE1);
        let day = date.day();
        assert_eq!(3, day);
    }
}
