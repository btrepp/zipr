use core::{fmt::Display, str::FromStr};

/// Enum describing the compression method
/// note there are many of these. Further types will be added
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CompressionMethod {
    Stored,
    Deflate,
}

/// Error structure for failing to parse a compression method
#[derive(Debug)]
pub struct CompressionMethodParseError {}

impl FromStr for CompressionMethod {
    type Err = CompressionMethodParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Store" => Ok(CompressionMethod::Stored),
            "Deflate" => Ok(CompressionMethod::Deflate),
            _ => Err(CompressionMethodParseError {}),
        }
    }
}

impl Display for CompressionMethodParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Compression Method Parsing Error")
    }
}
