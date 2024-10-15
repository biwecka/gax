// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::Distribution;

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    UniformSwap,
    // Non-uniform random values based on normal-distribution (gauss).
    GaussSwap,
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn identifier(&self) -> String {
        match self {
            Self::UniformSwap => "uni-sw".into(),
            Self::GaussSwap => "gauss-sw".into(),
        }
    }

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

            Mutation::GaussSwap => {
                swap_gauss_dist(chromosome, rate, &ctx.gauss_rand_event, rng);
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

fn swap_gauss_dist(
    chromosome: &mut Chromosome,
    rate: f32,
    generator: &rand_distr::Normal<f32>,
    rng: &mut ThreadRng,
) {
    let matrix = &mut chromosome.0;
    let rows = matrix.shape()[0];
    let cols = matrix.shape()[1];

    for row in 0..rows {
        for col in 0..cols {
            // Only consider genes for mutation which contain "1". This reflects
            // the behavior described in Abramsons papter.
            // TODO: remove this and check if helps the genetic process.
            if matrix[[row, col]] != 1 {
                continue;
            }

            // Decide wether to mutate or not
            if rng.gen::<f32>() > rate {
                continue;
            }

            // Generate the time index (row index) to swap with
            let mut offset = generator.sample(rng).round() as i32;
            let mut swap_time = row as i32 + offset;

            while offset == 0 || swap_time < 0 || swap_time >= rows as i32 {
                offset = generator.sample(rng).round() as i32;
                swap_time = row as i32 + offset;
            }

            // Swap
            matrix.swap([row, col], [swap_time as usize, col]);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
