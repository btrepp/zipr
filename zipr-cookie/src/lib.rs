#![no_std]
//! Serialize zipr data structures using cookie-factory
//!
//! This contains the serializers for writing the zip files
//! You still need to call the cookie factory functions

pub mod data;
mod iter_bytes;
mod layout;
pub use iter_bytes::*;

pub use cookie_factory;
