use std::usize;

// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Variable single-point crossover takes one argument:
    /// 1) f32      representing the crossover rate
    VariableSinglePoint(f32),

    /// Variable n-point crossover takes two arguments:
    /// 1) f32      representing the crossover rate
    /// 2) usize    representing the amount of crossover points
    VariableNPoint(f32, usize),
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        _context: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::VariableSinglePoint(rate) => {
                let (a, b) = ga::utils::crossover::variable_single_point(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    *rate,
                );

                (a.into(), b.into())
            }

            Crossover::VariableNPoint(rate, num_points) => {
                let (a, b) = ga::utils::crossover::variable_multi_point(
                    *num_points,
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    *rate,
                );

                (a.into(), b.into())
            }
        }
    }
}

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Randomize n Genes modifies n genes of the chromosome randomly.
    /// Parameters:
    /// 1) f32      representing the mutation rate
    /// 2) usize    representing the number of genes to modify
    RandomizeNGenes(f32, usize),
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn exec(&self, chromosome: &mut Chromosome, context: &Context) {
        match self {
            Mutation::RandomizeNGenes(rate, n) => {
                ga::utils::mutation::randomize_n_genes(
                    *n,
                    chromosome.as_mut_slice(),
                    *rate,
                    context.random_position,
                );
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
