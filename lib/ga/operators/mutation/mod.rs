//! This module provides the actual function implementations of the most
//! commonly used mutation methods.
//!
//! The different variations of genetic mutations are not universally applicable
//! too any kind of problem and genotype. For example direct binary/value
//! encoding will most certanly need other mutation methods than a chromosome
//! using permutation encoding.
//!
//! Therefore this crate cannot provide a struct/enum, which implements
//! the [`super::Mutation`] trait. To make it still straight forward to
//! implement this trait for your own struct/enum, this module exposes
//! functions which implement commonly known mutation methods.
//!

mod randomize;
pub use randomize::*;

// mod swap;
// pub use swap::*;
