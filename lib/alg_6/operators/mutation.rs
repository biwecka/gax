// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::rngs::ThreadRng;

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    RandomValue,
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
                    &ctx.rand_time,
                    rng,
                )
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
// pub fn randomize_n_genes<'a>(
//     amount: usize,
//     chromosome: &'a mut [usize],
//     rate: f32,
//     ctx: &Context,
// ) {
//     // Randomness
//     let mut rng = rand::thread_rng();

//     // Check
//     if rng.gen_range(0. ..=1.) > rate {
//         return;
//     };

//     // Perform mutation
//     // let random_gene_index =
//     //     rand::distributions::Uniform::new(0, chromosome.len());

//     for _ in 0..amount {
//         let index = ctx.rand_event.sample(&mut rng);

//         // Get duration of this event
//         let duration = ctx.durations[index] as usize;

//         // Get random number generator for this event index
//         let random_time = ctx.rand_times_by_duration[duration - 1];
//         chromosome[index] = random_time.sample(&mut rng);
//     }
// }

// pub fn conventional<'a>(chromosome: &'a mut [usize], rate: f32, ctx: &Context) {
//     // Randomness
//     let mut rng = rand::thread_rng();

//     for (event_idx, gene) in chromosome.iter_mut().enumerate() {
//         // To mutate, or not to mutate - that's the question.
//         if rng.gen_range(0. ..=1.) > rate {
//             continue;
//         }

//         // Get the duration of this event
//         let duration = ctx.durations[event_idx] as usize;

//         // Get random number generator for this event index
//         let random_time = ctx.rand_times_by_duration[duration - 1];
//         *gene = random_time.sample(&mut rng);
//     }
// }

////////////////////////////////////////////////////////////////////////////////
