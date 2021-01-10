use std::fmt::Display;
use zipr::{compression::DecompressError, nom::iter::ZipEntryIteratorError};

#[derive(Debug)]
pub enum AppError {
    Decompression(DecompressError),
    NomError(nom::error::Error<String>),
    ZipIteratorError(ZipEntryIteratorError),
}

impl From<ZipEntryIteratorError> for AppError {
    fn from(z: ZipEntryIteratorError) -> Self {
        AppError::ZipIteratorError(z)
    }
}

impl From<DecompressError> for AppError {
    fn from(e: DecompressError) -> Self {
        AppError::Decompression(e)
    }
}

impl From<nom::error::Error<&'_ [u8]>> for AppError {
    fn from(e: nom::error::Error<&'_ [u8]>) -> Self {
        let hex = nom::HexDisplay::to_hex(e.input, 8);
        let error = nom::error::Error::new(hex, e.code);
        AppError::NomError(error)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x?}", self))
    }
}

impl std::error::Error for AppError {}
