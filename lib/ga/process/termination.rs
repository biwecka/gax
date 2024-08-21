// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::ObjectiveValue;

// Trait ///////////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// termination methods. The termination methods are used after each iteration
/// of the genetic algorithm to decide if the algorithm should halt.
///
pub trait Termination<Ov: ObjectiveValue>: Send + Sync {
    fn stop(&self, generation_num: usize, current_best: &Ov) -> bool;
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Terminate<Ov: ObjectiveValue> {
    /// Stop the GA after the given amount of generations
    Generations(usize),

    /// Stop the GA if the best individual's fitness is as good or better as
    /// the provided fitness value.
    ObjectiveValue(Ov),
}

impl<Ov: ObjectiveValue> Termination<Ov> for Terminate<Ov> {
    fn stop(&self, generation_num: usize, current_best: &Ov) -> bool {
        match self {
            Self::Generations(generation_limit) => {
                generation_num >= *generation_limit
            }

            Self::ObjectiveValue(target) => *current_best <= *target,
        }
    }
}

// Functions ///////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
