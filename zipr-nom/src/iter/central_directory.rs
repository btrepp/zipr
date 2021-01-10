use core::iter::Iterator;
use nom::Finish;
use zipr_core::data::file::CentralDirectoryEntry;

pub enum CentralDirectoryIteratorError<'a> {
    NoEndofDirectory(nom::error::Error<&'a [u8]>),
    InvalidDirectoryEntry(nom::error::Error<&'a [u8]>),
}

impl<'a> From<nom::error::Error<&'a [u8]>> for CentralDirectoryIteratorError<'a> {
    fn from(e: nom::error::Error<&'a [u8]>) -> Self {
        CentralDirectoryIteratorError::InvalidDirectoryEntry(e)
    }
}

enum State {
    Initialized,
    Errored,
    Entries,
}

pub struct CentralDirectoryIterator<'a> {
    unprocessed: &'a [u8],
    state: State,
}

type Result<'a, T> = core::result::Result<T, CentralDirectoryIteratorError<'a>>;

impl<'a> CentralDirectoryIterator<'a> {
    pub fn create(file: &[u8]) -> CentralDirectoryIterator {
        let state = State::Initialized;
        CentralDirectoryIterator {
            unprocessed: file,
            state,
        }
    }

    fn next_entry(it: &mut CentralDirectoryIterator<'a>) -> Result<'a, CentralDirectoryEntry<'a>> {
        match crate::data::parse_directory_header(it.unprocessed).finish() {
            Err(e) => {
                it.state = State::Errored;
                Err(CentralDirectoryIteratorError::InvalidDirectoryEntry(e))
            }
            Ok((rem, dir)) => {
                it.unprocessed = rem;
                Ok(dir)
            }
        }
    }

    fn from_initialize(
        it: &mut CentralDirectoryIterator<'a>,
    ) -> Result<'a, CentralDirectoryEntry<'a>> {
        match crate::search::find_end_of_central_directory(it.unprocessed).finish() {
            Err(e) => {
                it.state = State::Errored;
                Err(CentralDirectoryIteratorError::NoEndofDirectory(e))
            }
            Ok((_, eocd)) => {
                let start = eocd.offset_start_directory as usize;
                let end = start + eocd.size_of_directory as usize;
                it.unprocessed = &it.unprocessed[start..end];
                it.state = State::Entries;
                Self::next_entry(it)
            }
        }
    }
}

impl<'a> Iterator for CentralDirectoryIterator<'a> {
    type Item = Result<'a, CentralDirectoryEntry<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Errored => None,
            State::Initialized => Some(Self::from_initialize(self)),
            State::Entries => {
                if self.unprocessed.is_empty() {
                    None
                } else {
                    Some(Self::next_entry(self))
                }
            }
        }
    }
}

pub fn iterate_central_directory(file: &[u8]) -> CentralDirectoryIterator<'_> {
    CentralDirectoryIterator::create(file)
}
