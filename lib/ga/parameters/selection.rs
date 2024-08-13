// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};

// Traits //////////////////////////////////////////////////////////////////////
/// The [`SelectionStrategies`] trait must be implemented by structs or enums
/// which represent one or multiple selection strategies for the genetic
/// algorithm.
pub trait SelectionStrategies {
    /// Execute the selection.
    fn exec<'a, Ct: Context, Ge: Genotype<Ct>, Ov: ObjectiveValue>(
        &self,
        individuals: &'a [(Ge, Ov)],
    ) -> (Vec<&'a (Ge, Ov)>, usize);
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Selection {
    Roulette,
}

impl SelectionStrategies for Selection {
    fn exec<'a, Ct: Context, Ge: Genotype<Ct>, Ov: ObjectiveValue>(
        &self,
        individuals: &'a [(Ge, Ov)],
    ) -> (Vec<&'a (Ge, Ov)>, usize) {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////
