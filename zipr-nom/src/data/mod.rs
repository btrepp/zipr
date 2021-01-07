//! Parsers for the data structures
//! Note these are the very low level components.
//! Most expect to be given the correct slice as is.
//! For more user friendly parsers use the higher level functions

pub mod central_directory;
pub mod compression_method;
pub mod end_of_central_directory;
pub mod extra_field;
pub mod local_file;
pub mod ntfs;
pub mod path;
