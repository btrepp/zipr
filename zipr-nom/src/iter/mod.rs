//! Lazy iterators for the zip file
//!
//! These allow you to iterate through the file
//! getting the next entry. Note this can fail at any time
//! so usually have result items

mod central_directory;
mod zip_entries;

pub use central_directory::*;
pub use zip_entries::*;
