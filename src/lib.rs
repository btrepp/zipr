mod constants;
mod eocd;

pub mod parsers {
    pub use crate::eocd::parse_eocd;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


