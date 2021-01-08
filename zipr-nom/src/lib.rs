#![no_std] 
extern crate alloc;
mod compression;
pub mod data;
mod search;

pub use compression::*;
pub use search::*;
