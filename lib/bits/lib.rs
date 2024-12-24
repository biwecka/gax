//! The `bits` library crate provides high-performance bit vector and bit
//! matrix implementations, which combine a feature-rich API with common
//! methods as well as higher level functionality, without compromising on
//! performance.
//!
//! The set of structs this crate exposes follows a certain naming scheme:
//! -   Bit Vectors:
//!     -   `Bits8`     : bit vector with up to 8 bits
//!     -   `Bits16`    : bit vector with up to 16 bits
//!     -   `Bits32`    : bit vector with up to 32 bits
//!     -   ...
//!     -   `Bits128`   : bit vector with up to 128 bits
//!
//! -   Bit Matrices:
//!     -   `BitsMatrix8x8`
//!     -   `BitsMatrix8x16`
//!     -   ...
//!     -   `BitsMatrix8x128`
//!     -   `BitsMatrix16x8`
//!     -   `BitsMatrix16x16`
//!     -   ...
//!     -   `BitsMatrix128x128`
//!
//! Example uses of these bit vectors and bit matrices can be found in
//! `tests.rs`.

// In the constructor of the generated "bits structs" (e.g. Bits8, Bits32)
// the function `unbounded_shl` is used, which is only available on nightly
// Rust at the time of writing this. To enable this functionality the feature
// must be enabled manually as seen below.
#![feature(unbounded_shifts)]

// Modules /////////////////////////////////////////////////////////////////////
pub mod cols;
pub mod matrix;
pub mod rows;

mod generator;

#[cfg(test)]
mod tests;

// Bits8 ///////////////////////////////////////////////////////////////////////
mod b8 {
    crate::generate_bits!(pub struct Bits8 {
        type: u8,
        iterators:
            ones    : OnesBits8,
            zeros   : ZerosBits8,
            group   : GroupBits8,
            holes   : HolesBits8,
    });
}

pub use b8::*;

// Bits16 //////////////////////////////////////////////////////////////////////
mod b16 {
    crate::generate_bits!(pub struct Bits16 {
        type: u16,
        iterators:
            ones    : OnesBits16,
            zeros   : ZerosBits16,
            group   : GroupBits16,
            holes   : HolesBits16,
    });
}

pub use b16::*;

// Bits32 //////////////////////////////////////////////////////////////////////
mod b32 {
    crate::generate_bits!(pub struct Bits32 {
        type: u32,
        iterators:
            ones    : OnesBits32,
            zeros   : ZerosBits32,
            group   : GroupBits32,
            holes   : HolesBits32,
    });
}

pub use b32::*;

// Bits64 //////////////////////////////////////////////////////////////////////
mod b64 {
    crate::generate_bits!(pub struct Bits64 {
        type: u64,
        iterators:
            ones    : OnesBits64,
            zeros   : ZerosBits64,
            group   : GroupBits64,
            holes   : HolesBits64,
    });
}

pub use b64::*;

// Bits128 /////////////////////////////////////////////////////////////////////
mod b128 {
    crate::generate_bits!(pub struct Bits128 {
        type: u128,
        iterators:
            ones    : OnesBits128,
            zeros   : ZerosBits128,
            group   : GroupBits128,
            holes   : HolesBits128,
    });
}

pub use b128::*;

////////////////////////////////////////////////////////////////////////////////
