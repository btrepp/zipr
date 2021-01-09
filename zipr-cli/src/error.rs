use std::fmt::Display;
use zipr::{compression::DecompressError, nom::iter::ZipEntryIteratorError};

#[derive(Debug)]
pub enum AppError {
    Decompression(DecompressError),
    NomError(nom::error::Error<String>),
    Iterator,
}

impl From<ZipEntryIteratorError<'_>> for AppError {
    fn from(_: ZipEntryIteratorError<'_>) -> Self {
        AppError::Iterator
    }
}

impl From<nom::error::Error<&'_ [u8]>> for AppError {
    fn from(n: nom::error::Error<&'_ [u8]>) -> Self {
        let hex = nom::HexDisplay::to_hex(n.input, 8);
        let owned = nom::error::Error::new(hex, n.code);
        AppError::NomError(owned)
    }
}

impl From<DecompressError> for AppError {
    fn from(e: DecompressError) -> Self {
        AppError::Decompression(e)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for AppError {}
