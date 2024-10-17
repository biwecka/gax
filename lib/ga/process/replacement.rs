// Imports /////////////////////////////////////////////////////////////////////
// use crate::encoding::{Context, Genotype, ObjectiveValue};

// Trait ///////////////////////////////////////////////////////////////////////
/// This trait is usually implemented by enums, which represent a set of
/// rejection methods. The rejection methods are executed after crossover and
/// mutation are finished, to ensure certain features in the offspring
/// chromosomes (e.g. better than either of the parents).
///
pub trait Replacement<T>: Send + Sync {
    fn elite_size(&self, population_size: usize) -> usize;

    /// Returns the raw selection size, and also the corrected. The correction
    /// is needed to ensure the selected number of parents is even, to perform
    /// crossover with pairs of parents.
    fn selection_size(&self, population_size: usize) -> (usize, usize);

    fn exec(&self, population: &mut Vec<T>, offspring: Vec<T>);

    fn identifier(&self) -> String;
}

// Implementation //////////////////////////////////////////////////////////////
#[derive(Clone)]
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

    fn selection_size(&self, population_size: usize) -> (usize, usize) {
        let elite_size =
            <Replace as Replacement<T>>::elite_size(self, population_size);

        let diff = population_size - elite_size;

        // Ensure selection size is a multiple of two
        if diff % 2 != 0 {
            (diff, diff + 1)
        } else {
            (diff, diff)
        }
    }

    fn exec(&self, population: &mut Vec<T>, offspring: Vec<T>) {
        // Calculate elite size
        let elite_size =
            <Replace as Replacement<T>>::elite_size(self, population.len());

        // Mutate the population
        population.splice(elite_size.., offspring);
    }

    fn identifier(&self) -> String {
        match self {
            Self::Full => "full".into(),
            Self::EliteAbsolute(n) => format!("eli-abs-{n}"),
            Self::EliteRelative(x) => format!("eli-rel-{:.4}", x),
        }
    }
}

// Functions ///////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
