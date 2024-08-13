// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype};

// Traits //////////////////////////////////////////////////////////////////////
/// The [`CrossoverStrategies`] trait must be implemented by structs or enums
/// which represent one or crossover selection strategies for the genetic
/// algorithm.
pub trait CrossoverStrategies {
    /// Execute the selection.
    fn exec<'a, Ct: Context, Ge: Genotype<Ct>>(
        &self,
        a: &Ge,
        b: &Ge,
        rate: f32,
    ) -> (Ge, Ge);
}

////////////////////////////////////////////////////////////////////////////////
