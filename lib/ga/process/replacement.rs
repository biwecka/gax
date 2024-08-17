// Imports /////////////////////////////////////////////////////////////////////
// use crate::encoding::{Context, Genotype, ObjectiveValue};

// Trait ///////////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// rejection methods. The rejection methods are executed after crossover and
/// mutation are finished, to ensure certain features in the offspring
/// chromosomes (e.g. better than either of the parents).
///
pub trait Replacement<T> {
    fn elite_size(&self, population_size: usize) -> usize;

    fn selection_size(&self, population_size: usize) -> usize;

    fn exec(&self, population: &mut Vec<T>, offspring: Vec<T>);
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Replace {
    /// No elitism, full replacement of the current population with the
    /// offspring/children.
    Full,

    /// Elitism with an absolute size.
    EliteAbsolute(usize),

    /// Elitism with relative size of the elite. A minimum absolute size of 1
    /// is ensured.
    EliteRelative(f32),
}

impl<T> Replacement<T> for Replace {
    fn elite_size(&self, population_size: usize) -> usize {
        match self {
            Self::Full => 0,
            Self::EliteAbsolute(n) => *n,
            Self::EliteRelative(f) => {
                // Calcualte elite size
                let elite_size = (population_size as f32 * f).floor() as usize;

                // Retrun (ensure minimum of 1)
                elite_size.max(1)
            }
        }
    }

    fn selection_size(&self, population_size: usize) -> usize {
        let elite_size =
            <Replace as Replacement<T>>::elite_size(self, population_size);

        population_size - elite_size
    }

    fn exec(&self, population: &mut Vec<T>, offspring: Vec<T>) {
        // Calculate elite size
        let elite_size =
            <Replace as Replacement<T>>::elite_size(self, population.len());

        // Mutate the population
        population.splice(elite_size.., offspring);
    }
}

// Functions ///////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
