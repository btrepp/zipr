use std::{path::Path, str::from_utf8};
use zipr_core::data::{DosDate, DosTime, ZipPath};
pub trait ToPath {
    fn to_path(&self) -> &Path;
}

impl<'a> ToPath for ZipPath<'a> {
    fn to_path(&self) -> &Path {
        let bytes = self.to_bytes();
        let string = from_utf8(bytes).unwrap();
        Path::new(string)
    }
}

pub trait ToNaiveDate {
    fn to_date(&self) -> chrono::NaiveDate;
}

impl ToNaiveDate for DosDate {
    fn to_date(&self) -> chrono::NaiveDate {
        let year = self.year();
        let month = self.month();
        let day = self.day();
        chrono::NaiveDate::from_ymd(year.into(), month.into(), day.into())
    }
}

pub trait ToNaiveTime {
    fn to_time(&self) -> chrono::NaiveTime;
}

impl ToNaiveTime for DosTime {
    fn to_time(&self) -> chrono::NaiveTime {
        let hour = self.hour();
        let min = self.min();
        let sec = self.sec();
        chrono::NaiveTime::from_hms(hour.into(), min.into(), sec.into())
    }
}
