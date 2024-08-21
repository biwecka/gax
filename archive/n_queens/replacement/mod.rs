use crate::encoding::{Cost, Genotype};

#[allow(unused)]
pub enum Replace {
    Full,
    Elite(f32),
}

impl Replace {
    /// Calculates the selection size and elite size.
    /// The return value first contains the selection size, then the elite
    /// size.
    pub fn selection_size(&self, population_size: usize) -> usize {
        match self {
            Self::Full => population_size,
            Self::Elite(_) => {
                population_size - self.elite_size(population_size)
            }
        }
    }

    pub fn elite_size(&self, population_size: usize) -> usize {
        match self {
            Self::Full => 0,
            Self::Elite(rate) => {
                // Calculate elite size
                let mut elite_size =
                    (population_size as f32 * rate).floor() as usize;

                // If elite size is 0, set it to 1. This ensures to always have
                // at least one elitist chromosome to be taken over to the new
                // generation.
                if elite_size < 1 {
                    elite_size = 1;
                }

                // Return
                elite_size
            }
        }
    }

    pub fn exec(
        &self,
        population: &mut Vec<(Genotype, Cost)>,
        offspring: Vec<(Genotype, Cost)>,
    ) {
        population.splice(self.elite_size(population.len()).., offspring);
    }
}
