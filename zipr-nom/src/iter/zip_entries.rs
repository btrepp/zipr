use core::cmp::min;

use super::CentralDirectoryIteratorError;
use crate::data::parse_local_file;
use nom::Finish;
use zipr_core::{
    constants,
    data::{file::CentralDirectoryEntry, ZipEntry},
    make_zip_entry,
};

/// An iterator for zip entries in a set of bytes
/// Will load them lazily, so you can only pull out as much as you need
pub struct ZipEntryIterator<'a> {
    file: &'a [u8],
    directories: super::CentralDirectoryIterator<'a>,
}

///Enum for the ways we can fail to load zip file entries
#[derive(Debug)]
pub enum ZipEntryIteratorError {
    DirectoryError(CentralDirectoryIteratorError),
    LocalFileError(nom::error::Error<[u8; constants::LOCAL_FILE_MIN_LENGTH as usize]>),
}

fn copy_slice_safe(a: &mut [u8], b: &[u8]) {
    let length = min(a.len(), b.len());
    let dest = &mut a[..length];
    let src = &b[..length];
    dest.copy_from_slice(src);
}

fn invalid_entry(error: nom::error::Error<&'_ [u8]>) -> ZipEntryIteratorError {
    let mut dest: [u8; constants::LOCAL_FILE_MIN_LENGTH as usize] =
        [0; constants::LOCAL_FILE_MIN_LENGTH as usize];
    copy_slice_safe(&mut dest, error.input);
    let error = nom::error::Error::new(dest, error.code);
    ZipEntryIteratorError::LocalFileError(error)
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

fn next_entry<'a>(
    input: &'a [u8],
    d: Result<CentralDirectoryEntry<'a>, CentralDirectoryIteratorError>,
) -> Result<ZipEntry<'a>, ZipEntryIteratorError> {
    let directory = d.map_err(ZipEntryIteratorError::DirectoryError)?;
    let start = directory.relative_offset as usize;
    let end = input.len();
    let input = &input[start..end];
    let (_, entry) = parse_local_file(input).finish().map_err(invalid_entry)?;
    let zip = make_zip_entry(&directory, entry);
    Ok(zip)
}

impl<'a> core::iter::Iterator for ZipEntryIterator<'a> {
    type Item = Result<ZipEntry<'a>, ZipEntryIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        let directories = &mut self.directories;
        let file = self.file;
        let next = directories.map(|e| next_entry(file, e)).next()?;
        Some(next)
    }
}
