// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::rngs::ThreadRng;

// Mutation ////////////////////////////////////////////////////////////////////
#[allow(unused)]
#[derive(Clone)]
pub enum Mutation {
    /// Swaps to random genes.
    UniformSwap,

    /// Non-uniform random values based on normal-distribution.
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
                ga::operators::mutation::swap_uniform_dist_u8(
                    chromosome.as_mut_slice(),
                    rate,
                    &ctx.rand_event,
                    rng,
                )
            }

            Mutation::GaussSwap => {
                ga::operators::mutation::swap_normal_dist_u8(
                    chromosome.as_mut_slice(),
                    rate,
                    &ctx.gauss_rand_event,
                    rng,
                )
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
