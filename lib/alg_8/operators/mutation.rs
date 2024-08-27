// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, Rng};

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    UniformSwap,
    // Non-uniform random values based on normal-distribution.
    // NormalSwap,
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn exec(
        &self,
        chromosome: &mut Chromosome,
        rate: f32,
        rng: &mut ThreadRng,
        ctx: &Context,
    ) {
        match self {
            Mutation::UniformSwap => {
                swap_uniform_dist(chromosome, rate, &ctx.rand_time, rng);
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
fn swap_uniform_dist<D: rand::distributions::Distribution<usize>>(
    chromosome: &mut Chromosome,
    rate: f32,
    generator: &D,
    rng: &mut ThreadRng,
) {
    let matrix = &mut chromosome.0;
    let rows = matrix.shape()[0];
    let cols = matrix.shape()[1];

    for row in 0..rows {
        for col in 0..cols {
            // Only consider genes for mutation which contain "1". This
            // reflects the behavior described in Abramsons paper.
            if matrix[[row, col]] != 1 {
                continue;
            }

            // Decide wether to mutate or not
            if rng.gen::<f32>() > rate {
                continue;
            }

            // Generate the time index (row index) to swap with
            let mut swap_time = generator.sample(rng);
            while swap_time == row {
                swap_time = generator.sample(rng);
            }

            // Swap
            matrix.swap([row, col], [swap_time, col])
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
