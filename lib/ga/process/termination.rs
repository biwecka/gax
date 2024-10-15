// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::ObjectiveValue;

// Trait ///////////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// termination methods. The termination methods are used after each iteration
/// of the genetic algorithm to decide if the algorithm should halt.
///
pub trait Termination<Ov: ObjectiveValue>: Send + Sync {
    fn stop(&self, generation_num: usize, current_best: &Ov) -> bool;

    fn identifier(&self) -> String;
    fn max_generations(&self) -> Option<usize>;
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Terminate<Ov: ObjectiveValue> {
    /// Stop the GA after the given amount of generations
    Generations(usize),

    /// Stop the GA if the best individual's fitness is as good or better as
    /// the provided fitness value.
    ObjectiveValue(Ov),

    /// Stop after max amount of generations or after reaching the target
    /// objective value.
    GenOrOv(usize, Ov),
}

impl<Ov: ObjectiveValue> Termination<Ov> for Terminate<Ov> {
    fn stop(&self, generation_num: usize, current_best: &Ov) -> bool {
        match self {
            Self::Generations(generation_limit) => {
                generation_num >= *generation_limit
            }

            Self::ObjectiveValue(target) => *current_best <= *target,

            Self::GenOrOv(gen_limit, target) => {
                generation_num >= *gen_limit || current_best <= target
            }
        }
    }

    fn identifier(&self) -> String {
        match self {
            Self::Generations(n) => format!("gen-{n}"),
            Self::ObjectiveValue(ov) => format!("ov-{}", ov.to_usize()),
            Self::GenOrOv(g, ov) => format!("g-{}-ov-{}", g, ov.to_usize()),
        }
    }

    fn max_generations(&self) -> Option<usize> {
        match self {
            Self::Generations(g) => Some(*g),
            Self::ObjectiveValue(_) => None,
            Self::GenOrOv(g, _) => Some(*g),
        }
    }
}

// Functions ///////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
