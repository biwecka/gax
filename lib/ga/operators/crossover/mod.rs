//! This module provides the actual function implementations of the most
//! commonly used crossover methods.
//!
//! The different variations of genetic crossover are not universally applicable
//! too any kind of problem and genotype. For example direct binary/value
//! encoding will most certanly need other crossover methods than a chromosome
//! using permutation encoding.
//!
//! Therefore this crate cannot provide a struct/enum, which implements
//! the [`super::Crossover`] trait. To make it still straight forward to
//! implement this trait for your own struct/enum, this module exposes
//! functions which implement commonly known crossover methods.
//!

mod single_point;
pub use single_point::*;

mod multi_point;
pub use multi_point::*;

mod uniform;
pub use uniform::*;

mod ordered;
pub use ordered::*;

mod pmx;
pub use pmx::*;
