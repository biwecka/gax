// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    stats::Stats,
};

// Traits //////////////////////////////////////////////////////////////////////
/// The [`TerminationStrategies`] trait must be implemented by structs or enums
/// which represent one or multiple termination strategies for the genetic
/// algorithm.
/// With [`Termination`], this crate provides an enum with commonly used
/// termination strategies.
/// If custom termination strategies are needed, this trait can implemented
/// manually.
pub trait TerminationStrategies<Ov: ObjectiveValue> {
    /// Performs the termination check.
    fn check<Ct, Ge, St>(&self, stats: &St) -> bool
    where
        Ct: Context,
        Ge: Genotype<Ct>,
        St: Stats<Ov, Ct, Ge>;
}

// Enums ///////////////////////////////////////////////////////////////////////
/// An implementation of the [`TerminationStrategies`] trait, which provides
/// commonly used termination strategies.
pub enum Termination<Ov: ObjectiveValue> {
    /// Stop the GA after the given amount of generations
    Generations(usize),

    /// Stop the GA if the best individual's fitness is as good or better as
    /// the provided fitness value.
    ObjectiveValue(Ov),
}

impl<Ov: ObjectiveValue> TerminationStrategies<Ov> for Termination<Ov> {
    fn check<Ct: Context, Ge: Genotype<Ct>, St: Stats<Ov, Ct, Ge>>(
        &self,
        stats: &St,
    ) -> bool {
        match self {
            Self::Generations(gen_limit) => generations(*gen_limit, stats),
            Self::ObjectiveValue(target) => objective_value(target, stats),
        }
    }
}

// Functions ///////////////////////////////////////////////////////////////////
fn generations<St, Ov, Ct, Ge>(gen_limit: usize, stats: &St) -> bool
where
    St: Stats<Ov, Ct, Ge>,
    Ov: ObjectiveValue,
    Ct: Context,
    Ge: Genotype<Ct>,
{
    if stats.generation() >= gen_limit {
        true
    } else {
        false
    }
}

fn objective_value<St, Ov, Ct, Ge>(target: &Ov, stats: &St) -> bool
where
    St: Stats<Ov, Ct, Ge>,
    Ov: ObjectiveValue,
    Ct: Context,
    Ge: Genotype<Ct>,
{
    if stats.curr_best() <= *target {
        true
    } else {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////
