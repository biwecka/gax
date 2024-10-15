// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, Rng};

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    UniformRandom,

    // /// Non-uniform random values based on beta-distribution.
    // BetaRandom,
    /// Non-uniform random values based on normal-distribution.
    NormalDistributedRandom,
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn identifier(&self) -> String {
        match self {
            Self::UniformRandom => "uni-rand".into(),
            Self::NormalDistributedRandom => "gauss-rand".into(),
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
            Mutation::UniformRandom => {
                ga::operators::mutation::randomize_multi_dist(
                    chromosome.as_mut_slice(),
                    rate,
                    &ctx.rand_time_uniform,
                    rng,
                )
            }

            // Mutation::BetaRandom => beta_random_multi_dist(
            //     chromosome.as_mut_slice(),
            //     rate,
            //     &ctx.rand_time,
            //     rng,
            // ),
            Mutation::NormalDistributedRandom => normal_random_multi_dist(
                chromosome.as_mut_slice(),
                rate,
                &ctx.rand_time,
                rng,
            ),
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
// pub fn beta_random_multi_dist(
//     chromosome: &mut [usize],
//     rate: f32,
//     generators: &[crate::utils::beta_dist::DynamicBetaDistribution],
//     rng: &mut ThreadRng,
// ) {
//     assert_eq!(chromosome.len(), generators.len());

//     for (i, gene) in chromosome.iter_mut().enumerate() {
//         // Decide wether to mutate or not
//         if rng.gen_range(0. ..=1.) > rate {
//             continue;
//         }

//         // Mutate the gene (pass in `*gene` as expected value)
//         *gene = generators[i].sample(*gene, rng);
//     }
// }

pub fn normal_random_multi_dist(
    chromosome: &mut [usize],
    rate: f32,
    generators: &[crate::utils::normal_dist::NormalDistribution],
    rng: &mut ThreadRng,
) {
    assert_eq!(chromosome.len(), generators.len());

    for (i, gene) in chromosome.iter_mut().enumerate() {
        // Decide wether to mutate or not
        if rng.gen_range(0. ..=1.) > rate {
            continue;
        }

        // Mutate the gene (pass in `*gene` as expected value)
        *gene = generators[i].sample(*gene, rng);
    }
}

////////////////////////////////////////////////////////////////////////////////
