// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};

// Traits //////////////////////////////////////////////////////////////////////
/// The [`OffspringRejectionStrategies`] trait must be implemented by structs
/// or enums which represent one or multiple offspring rejection strategies for
/// the genetic algorithm.
///
/// An offspring rejection strategy usually checks if the produced offspring
/// is better (in some way) than one or both of its parents. If this is not the
/// case the rejection strategy can for example reject the offspring and in turn
/// use one of the parents instead.
///
/// With [`OffspringRejection`] this crate provides an implementation of this
/// trait which contains common offspring rejection strategies.
///
pub trait OffspringRejectionStrategies {
    /// Executes the offspring rejection.
    fn exec<Ct: Context, Ge: Genotype<Ct>, Ov: ObjectiveValue>(
        &self,
        bias: f32,
        // Parents
        p0: (Ge, Ov),
        p1: (Ge, Ov),
        // Offspring
        o0: (Ge, Ov),
        o1: (Ge, Ov),
    ) -> Vec<(Ge, Ov)>;
}

// Enums ///////////////////////////////////////////////////////////////////////
/// An implementation of the [`OffspringRejectionStrategies`] trait, which
/// provides common offspring rejection strategies.
pub enum OffspringRejection {
    /// No offspring rejection. The offspring individuals are always taken as
    /// they are and no checks if they actually are better than their parents
    /// are performed.
    None,
}

impl OffspringRejectionStrategies for OffspringRejection {
    fn exec<Ct: Context, Ge: Genotype<Ct>, Ov: ObjectiveValue>(
        &self,
        bias: f32,
        // Parents
        p0: (Ge, Ov),
        p1: (Ge, Ov),
        // Offspring
        o0: (Ge, Ov),
        o1: (Ge, Ov),
    ) -> Vec<(Ge, Ov)> {
        match self {
            OffspringRejection::None => vec![o0, o1],
        }
    }
}

// Functions ///////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
