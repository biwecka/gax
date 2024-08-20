use rand::{prelude::Distribution, Rng};

// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Randomize n Genes modifies n genes of the chromosome randomly.
    /// Parameters:
    /// 1) f32      representing the mutation rate
    /// 2) usize    representing the number of genes to modify
    RandomizeNGenes(f32, usize),

    /// Conventional mutation method of mutating each gene of the input
    /// chromosome with the probability specified by the first parameter.
    Conventional(f32),
}

impl ga::operators::Mutation<Context, Chromosome> for Mutation {
    fn exec(&self, chromosome: &mut Chromosome, ctx: &Context) {
        match self {
            Mutation::RandomizeNGenes(rate, n) => {
                randomize_n_genes(*n, chromosome.as_mut_slice(), *rate, ctx);
            }

            Mutation::Conventional(rate) => {
                conventional(chromosome.as_mut_slice(), *rate, ctx)
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
pub fn randomize_n_genes<'a>(
    amount: usize,
    chromosome: &'a mut [usize],
    rate: f32,
    ctx: &Context,
) {
    // Randomness
    let mut rng = rand::thread_rng();

    // Check
    if rng.gen_range(0. ..=1.) > rate {
        return;
    };

    // Perform mutation
    // let random_gene_index =
    //     rand::distributions::Uniform::new(0, chromosome.len());

    for _ in 0..amount {
        let index = ctx.rand_event.sample(&mut rng);

        // Get duration of this event
        let duration = ctx.durations[index] as usize;

        // Get random number generator for this event index
        let random_time = ctx.rand_times_by_duration[duration - 1];
        chromosome[index] = random_time.sample(&mut rng);
    }
}

pub fn conventional<'a>(chromosome: &'a mut [usize], rate: f32, ctx: &Context) {
    // Randomness
    let mut rng = rand::thread_rng();

    for (event_idx, gene) in chromosome.iter_mut().enumerate() {
        // To mutate, or not to mutate - that's the question.
        if rng.gen_range(0. ..=1.) > rate {
            continue;
        }

        // Get the duration of this event
        let duration = ctx.durations[event_idx] as usize;

        // Get random number generator for this event index
        let random_time = ctx.rand_times_by_duration[duration - 1];
        *gene = random_time.sample(&mut rng);
    }
}

////////////////////////////////////////////////////////////////////////////////
