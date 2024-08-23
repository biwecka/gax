// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::rngs::ThreadRng;

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    UniformSwap,
    // Non-uniform random values based on normal-distribution.
    // NormalDistributedRandom,
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
            Mutation::UniformSwap => ga::operators::mutation::swap(
                chromosome.as_mut_slice(),
                rate,
                &ctx.rand_event,
                rng,
            ), // Mutation::NormalDistributedRandom => normal_random_multi_dist(
               //     chromosome.as_mut_slice(),
               //     rate,
               //     &ctx.rand_time,
               //     rng,
               // ),
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
