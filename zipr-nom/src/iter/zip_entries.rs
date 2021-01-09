use super::{CentralDirectoryIterator, CentralDirectoryIteratorError};
use crate::data::parse_local_file;
use nom::Finish;
use zipr_core::{data::ZipEntry, make_zip_entry};

/// An iterator for zip entries in a set of bytes
/// Will load them lazily, so you can only pull out as much as you need
pub struct ZipEntryIterator<'a> {
    file: &'a [u8],
    directories: CentralDirectoryIterator<'a>,
}

///Enum for the ways we can fail to load zip file entries
pub enum ZipEntryIteratorError<'a> {
    DirectoryError(CentralDirectoryIteratorError<'a>),
    LocalFileError(nom::error::Error<&'a [u8]>),
}

/// Creates an iterator over the zip entries
/// This will lazily pull them out until it ends naturally
/// or fails on the first error
pub fn zip_entry_iter(input: &[u8]) -> ZipEntryIterator<'_> {
    let directories = super::iterate_central_directory(&input);
    ZipEntryIterator {
        file: input,
        directories,
    }
}

impl<'a> core::iter::Iterator for ZipEntryIterator<'a> {
    type Item = Result<ZipEntry<'a>, ZipEntryIteratorError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.directories.next() {
            None => None,
            Some(Err(e)) => Some(Err(ZipEntryIteratorError::DirectoryError(e))),
            Some(Ok(directory)) => {
                let start = directory.relative_offset as usize;
                let end = self.file.len();
                let input = &self.file[start..end];
                match parse_local_file(input).finish() {
                    Err(e) => Some(Err(ZipEntryIteratorError::LocalFileError(e))),
                    Ok((_, local)) => {
                        let zip = make_zip_entry(&directory, local);
                        Some(Ok(zip))
                    }
                }
            }
        }
    }
}
