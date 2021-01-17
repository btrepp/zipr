use core::{fmt::Display, str::FromStr};

/// Enum describing the compression method
/// note there are many of these. We don't implement them all
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CompressionMethod {
    Stored,
    Deflate,
}

#[derive(Debug)]
pub struct CompressionMethodError {}

impl FromStr for CompressionMethod {
    type Err = CompressionMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Store" => Ok(CompressionMethod::Stored),
            "Deflate" => Ok(CompressionMethod::Deflate),
            _ => Err(CompressionMethodError {}),
        }
    }
}

impl Display for CompressionMethodError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Compression Method Parsing Error")
    }
}
