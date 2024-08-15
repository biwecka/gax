//! The code of this `plotters_piston` library crate is a copy of the original
//! [plotters-piston](https://github.com/plotters-rs/plotters-piston) library.
//!
//! For the purpose of actually using it, the original code has been copied
//! and the dependencies were updated, to make it compatible with the current
//! versions of `piston`.
//!
//! In essence this library provides a compatibility layer, which makes it
//! possible to use `piston` in combination with `plotters`.

// Modules /////////////////////////////////////////////////////////////////////
mod backend;

// Re-Exports //////////////////////////////////////////////////////////////////
pub use backend::{draw_piston_window, PistonBackend};

////////////////////////////////////////////////////////////////////////////////
