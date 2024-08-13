use crate::{encoding::Cost, Stats};


/// An implementation of the [`TerminationStrategies`] trait, which provides
/// commonly used termination strategies.
pub enum Termination {
    /// Stop the GA after the given amount of generations
    Generations(usize),

    /// Stop the GA if the best individual's fitness is as good or better as
    /// the provided fitness value.
    ObjectiveValue(Cost),
}

impl Termination {
    pub fn check(&self, stats: &Stats) -> bool {
        match self {
            Self::Generations(max) => generations(*max, stats),
            Self::ObjectiveValue(threshold) => objective_value(*threshold, stats)
        }
    }
}

fn generations(max: usize, stats: &Stats) -> bool {
    if stats.generation >= max {
        true
    } else {
        false
    }
}

fn objective_value(threshold: Cost, stats: &Stats) -> bool {
    if stats.current_best <= threshold {
        true
    } else {
        false
    }
}