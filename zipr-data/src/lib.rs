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
mod util;

mod compression_method;
mod cp437;
mod dos_date;
mod dos_time;
mod version;
mod wintimestamp;

pub use compression_method::*;
pub use cp437::*;
pub use dos_date::*;
pub use dos_time::*;
pub use version::*;
pub use wintimestamp::*;
