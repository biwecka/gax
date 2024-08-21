use std::usize;

use rand::rngs::ThreadRng;

// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Variable single-point crossover takes one argument:
    /// 1) f32      representing the crossover rate
    VariableSinglePoint,

    /// Variable n-point crossover takes two arguments:
    /// 1) f32      representing the crossover rate
    /// 2) usize    representing the amount of crossover points
    VariableNPoint(usize),
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut ThreadRng,
        _context: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::VariableSinglePoint => {
                let (a, b) = ga::operators::crossover::single_point(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    rate,
                    rng,
                );

                (a.into(), b.into())
            }

            Crossover::VariableNPoint(num_points) => {
                let (a, b) = ga::operators::crossover::multi_point(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    rate,
                    *num_points,
                    rng,
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
    RandomValue,
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn exec(
        &self,
        chromosome: &mut Chromosome,
        rate: f32,
        rng: &mut ThreadRng,
        context: &Context,
    ) {
        match self {
            Mutation::RandomValue => {
                ga::operators::mutation::randomize_single_dist(
                    chromosome.as_mut_slice(),
                    rate,
                    &context.random_position,
                    rng,
                )
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
