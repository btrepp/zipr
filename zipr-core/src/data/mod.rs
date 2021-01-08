//! Pure data structures for zip files
//! https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
mod compression_method;
mod dos_date;
mod dos_time;
pub mod file;
mod zip_entry;
mod zip_path;

pub use ascii::AsciiStr;
use extra_field::ExtraField;
pub mod extra_field;
pub use compression_method::*;
pub use dos_date::*;
pub use dos_time::*;
pub use zip_entry::*;
pub use zip_path::*;
