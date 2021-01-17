use core::{cmp::min, iter::Iterator};
use nom::Finish;
use zipr_data::{
    borrowed::file::CentralDirectoryEntry,
    constants::{self},
};

#[derive(Debug)]
pub enum CentralDirectoryIteratorError {
    NoEndOfDirectory(
        nom::error::Error<[u8; constants::END_OF_CENTRAL_DIRECTORY_MIN_SIZE as usize]>,
    ),
    InvalidDirectoryEntry(
        nom::error::Error<[u8; constants::CENTRAL_DIRECTORY_HEAD_MIN_LENGTH as usize]>,
    ),
}
pub struct CentralDirectoryIterator<'a> {
    unprocessed: &'a [u8],
    state: State,
}

enum State {
    Initialized,
    Errored,
    Entries,
}
pub fn iterate_central_directory(file: &[u8]) -> CentralDirectoryIterator {
    let state = State::Initialized;
    CentralDirectoryIterator {
        unprocessed: file,
        state,
    }
}
fn copy_slice_safe(a: &mut [u8], b: &[u8]) {
    let length = min(a.len(), b.len());
    let dest = &mut a[..length];
    let src = &b[..length];
    dest.copy_from_slice(src);
}

fn invalid_entry(error: nom::error::Error<&'_ [u8]>) -> CentralDirectoryIteratorError {
    let mut dest: [u8; constants::CENTRAL_DIRECTORY_HEAD_MIN_LENGTH as usize] =
        [0; constants::CENTRAL_DIRECTORY_HEAD_MIN_LENGTH as usize];
    copy_slice_safe(&mut dest, error.input);
    let error = nom::error::Error::new(dest, error.code);
    CentralDirectoryIteratorError::InvalidDirectoryEntry(error)
}

fn invalid_eocd(error: nom::error::Error<&'_ [u8]>) -> CentralDirectoryIteratorError {
    let mut dest: [u8; constants::END_OF_CENTRAL_DIRECTORY_MIN_SIZE as usize] =
        [0; constants::END_OF_CENTRAL_DIRECTORY_MIN_SIZE as usize];
    copy_slice_safe(&mut dest, error.input);
    let error = nom::error::Error::new(dest, error.code);
    CentralDirectoryIteratorError::NoEndOfDirectory(error)
}

fn next_entry<'a>(
    it: &mut CentralDirectoryIterator<'a>,
) -> Result<CentralDirectoryEntry<'a>, CentralDirectoryIteratorError> {
    match crate::data::parse_directory_header(it.unprocessed).finish() {
        Err(e) => {
            it.state = State::Errored;
            let error = invalid_entry(e);
            Err(error)
        }
        Ok((rem, dir)) => {
            it.unprocessed = rem;
            Ok(dir)
        }
    }
}

fn from_initialize<'a>(
    it: &mut CentralDirectoryIterator<'a>,
) -> Result<CentralDirectoryEntry<'a>, CentralDirectoryIteratorError> {
    match crate::search::find_end_of_central_directory(it.unprocessed).finish() {
        Err(e) => {
            it.state = State::Errored;
            let error = invalid_eocd(e);
            Err(error)
        }
        Ok((_, eocd)) => {
            let start = eocd.offset_start_directory as usize;
            let end = start + eocd.size_of_directory as usize;
            it.unprocessed = &it.unprocessed[start..end];
            it.state = State::Entries;
            next_entry(it)
        }
    }
}

impl<'a> Iterator for CentralDirectoryIterator<'a> {
    type Item = Result<CentralDirectoryEntry<'a>, CentralDirectoryIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Errored => None,
            State::Initialized => Some(from_initialize(self)),
            State::Entries => {
                if self.unprocessed.is_empty() {
                    None
                } else {
                    Some(next_entry(self))
                }
            }
        }
    }
}
