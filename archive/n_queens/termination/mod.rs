use crate::{encoding::Cost, Stats};

/// An implementation of the [`TerminationStrategies`] trait, which provides
/// commonly used termination strategies.
#[allow(unused)]
pub enum Termination {
    /// Stop the GA after the given amount of generations
    Generations(usize),

    /// Stop the GA if the best individual's fitness is as good or better as
    /// the provided fitness value.
    ObjectiveValue(Cost),
}

impl Termination {
    pub fn check(&self, generation_num: usize, stats: &Stats) -> bool {
        match self {
            Self::Generations(max) => generations(*max, generation_num),
            Self::ObjectiveValue(threshold) => {
                objective_value(*threshold, stats)
            }
        }
    }
}

fn generations(max: usize, generation_num: usize) -> bool {
    generation_num >= max
}

fn objective_value(threshold: Cost, stats: &Stats) -> bool {
    *stats.best.last().unwrap() <= threshold
}
