// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, Rng};

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    RandomValue,

    ///
    BetaRandom,
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
            Mutation::RandomValue => {
                ga::operators::mutation::randomize_multi_dist(
                    chromosome.as_mut_slice(),
                    rate,
                    &ctx.rand_time_uniform,
                    rng,
                )
            }

            Mutation::BetaRandom => beta_random_multi_dist(
                chromosome.as_mut_slice(),
                rate,
                &ctx.rand_time_beta,
                rng,
            ),
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
pub fn beta_random_multi_dist(
    chromosome: &mut [usize],
    rate: f32,
    generators: &[crate::utils::beta_distr::DynamicBetaDistribution],
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
