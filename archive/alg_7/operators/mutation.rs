// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::rngs::ThreadRng;

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    UniformSwap,

    // Non-uniform random values based on normal-distribution.
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
                ga::operators::mutation::swap_uniform_dist(
                    chromosome.as_mut_slice(),
                    rate,
                    &ctx.rand_event_uniform,
                    rng,
                )
            }

            Mutation::GaussSwap => ga::operators::mutation::swap_normal_dist(
                chromosome.as_mut_slice(),
                rate,
                &ctx.rand_event,
                rng,
            ),
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
