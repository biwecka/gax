// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype};
use rand::prelude::Distribution;

// Enum ////////////////////////////////////////////////////////////////////////
pub enum Mutation {
    RandomizeBits(usize),
}

impl Mutation {
    pub fn exec(
        &self,
        individual: &mut Genotype,
        rate: f32,
        context: &Context,
    ) {
        match self {
            Self::RandomizeBits(amount) => {
                randomize_bits(*amount, individual, rate, context)
            }
        }
    }
}

// Implementations /////////////////////////////////////////////////////////////
fn randomize_bits(
    amount: usize,
    chromosome: &mut Genotype,
    rate: f32,
    ctx: &Context,
) {
    // Randomness
    let mut rng = rand::thread_rng();

    // Check
    let probabilty = rand::distributions::Uniform::new_inclusive(0., 1.);
    if probabilty.sample(&mut rng) > rate {
        return;
    };

    // Perform mutation
    let interval = rand::distributions::Uniform::new(0, chromosome.0.len());

    for _ in 0..amount {
        let index = interval.sample(&mut rng);
        chromosome.0[index] = ctx.random_position.sample(&mut rng);
    }
}

////////////////////////////////////////////////////////////////////////////////
