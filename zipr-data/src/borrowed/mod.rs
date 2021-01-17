//! Pure data structures for zip files
//! https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
pub mod extra_field;
pub mod file;

mod zip_entry;
mod zip_path;

pub use zip_entry::*;
pub use zip_path::*;
