use std::fmt::Display;
use zipr::{
    compression::DecompressError,
    data::borrowed::{NotValidOEM437, ZipPathError},
    nom::iter::ZipEntryIteratorError,
};

pub type AppResult<T> = Result<T, AppError>;
#[derive(Debug)]
pub enum AppError {
    Decompression(DecompressError),
    NomError(nom::error::Error<Vec<u8>>),
    ZipIteratorError(ZipEntryIteratorError),
    IOError(std::io::Error),
    OEM437Error(NotValidOEM437),
    ZipPathError(ZipPathError),
}

impl From<std::io::Error> for AppError {
    fn from(io: std::io::Error) -> Self {
        AppError::IOError(io)
    }
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

impl From<NotValidOEM437> for AppError {
    fn from(e: NotValidOEM437) -> Self {
        AppError::OEM437Error(e)
    }
}

impl From<ZipPathError> for AppError {
    fn from(e: ZipPathError) -> Self {
        AppError::ZipPathError(e)
    }
}

impl From<nom::error::Error<&'_ [u8]>> for AppError {
    fn from(e: nom::error::Error<&'_ [u8]>) -> Self {
        let data = e.input.to_vec();
        let error = nom::error::Error::new(data, e.code);
        AppError::NomError(error)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x?}", self))
    }
}

impl std::error::Error for AppError {}
