#![no_std]
///! Handles decompression and compresison of zipr types
///!
///! This currently has implementations for store, and for deflate
///! which would be the most common types
///! Does depend on alloc for now, as expanding data requires either
///! an iterator or a heap
extern crate alloc;

mod compress;
mod decompress;

pub use compress::*;
pub use decompress::*;
