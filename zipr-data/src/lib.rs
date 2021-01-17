//! Zipr data structures
//!
//! Data structures used to model a zip file
//! These contain the minimum create/extract functions
//! to represent a zip.
//! Most data structures are in borrowed, though some owned
//! Types sit at the top. It should be possible to create
//! any data structure here through public apis, and then
//! parse it through to the serialization libraries

#![no_std]
pub mod borrowed;
pub mod constants;

mod compression_method;
mod dos_date;
mod dos_time;
mod wintimestamp;

pub use compression_method::*;
pub use dos_date::*;
pub use dos_time::*;
pub use wintimestamp::*;
