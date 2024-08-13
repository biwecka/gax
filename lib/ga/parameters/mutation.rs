// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype};

// Traits //////////////////////////////////////////////////////////////////////
/// The [`MutationStrategies`] trait must be implemented by structs or enums
/// which represent one or mutation selection strategies for the genetic
/// algorithm.
pub trait MutationStrategies {
    /// Execute the selection.
    fn exec<'a, Ct: Context, Ge: Genotype<Ct>>(&self, a: &mut Ge, rate: f32);
}

////////////////////////////////////////////////////////////////////////////////
