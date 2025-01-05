//! A compatible library for subset of stk1
//!
//! # CAUTION
//!
//! **THIS LIBRARY IS AN ALPHA VERSION**.
//! Compression and decompression itself is possible, but you will need to provide your own processing for file headers and data size outside the library.
//!
//! # NOTE
//!
//! * stk1 is a simple and easy to decompress compression format.
//! * Since no formal specification exists, this implementation is based on reverse engineering of the decoder.
//!
//! The following _incompatibilities_ exist:
//! * The various limits are not official values.
//! * Only compressed data is supported; headers are not.
//!
//! # Original specifications
//!
//! (C) Kawai Hidemi
//!
//! Related Documents: <http://osask.net/w/196.html> (But different from known final specifications)

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod stk1;
pub use stk1::*;
mod s7s;
pub use s7s::*;

mod cache;
mod lz;
mod var_slice;

#[derive(Debug)]
pub enum EncodeError {
    // InvalidData,
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidData,
    OutOfMemory,
}
