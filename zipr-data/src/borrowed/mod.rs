//! Borrowed Pure data structures for zip files
//! https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
//!
//! These structures have the form of Type<'a> where 'a
//! is a borrowed slice of bytes. This allows us to avoid copying
//! bytes if they already exist. This typically occurs in the case of
//! variable length items, so that we can avoid using alloc entirely
pub mod extra_field;
pub mod file;

mod zip_entry;
mod zip_path;

pub use oem_437::*;
pub use zip_entry::*;
pub use zip_path::*;
