// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::Distribution;

// Mutation ////////////////////////////////////////////////////////////////////

#[allow(unused)]
pub enum Mutation {
    /// Assigns a random value to genes.
    UniformSwap,

    /// Non-uniform random values based on normal-distribution (gauss).
    GaussSwap,
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

    // Generate a list of events in this time slot which should be mutated
    // This list contains a tuple, where the first value is the event index
    // and the second value is the target time slot.
    let mut mutations: Vec<usize> = vec![];
    let mut target_index: Vec<usize> = vec![];

    for (time_idx, events) in matrix.iter_mut().enumerate() {
        for event_idx in events.clone() {
            // Decide wether to mutate or not
            if rng.gen::<f32>() > rate {
                continue;
            }

            // Generate a target time slot for the mutation
            let mut swap_time = generator.sample(rng);
            while swap_time == time_idx {
                swap_time = generator.sample(rng);
            }

            // If so, add the event index and the index of this event in the
            // time allocation vector to the "list"
            mutations.push(event_idx);
            target_index.push(swap_time);
        }

        // Remove the mutated events from this time slot
        events.retain(|x| !mutations.contains(x));
    }

    // Add the mutated events to the target time slots
    for (i, target_index) in target_index.iter().enumerate() {
        matrix[*target_index].push(mutations[i]);

        // Remove duplicates
        super::remove_duplicates(&mut matrix[*target_index]);
    }
}

fn swap_gauss_dist(
    chromosome: &mut Chromosome,
    rate: f32,
    generator: &rand_distr::Normal<f32>,
    rng: &mut ThreadRng,
) {
    let matrix = &mut chromosome.0;
    let matrix_len = matrix.len() as i32;

    // Generate a list of events in this time slot which should be mutated
    // This list contains a tuple, where the first value is the event index
    // and the second value is the target time slot.
    let mut mutations: Vec<usize> = vec![];
    let mut target_index: Vec<usize> = vec![];

    for (time_idx, events) in matrix.iter_mut().enumerate() {
        for event_idx in events.clone() {
            // Decide wether to mutate or not
            if rng.gen::<f32>() > rate {
                continue;
            }

            // Generate a target time slot for the mutation
            let mut offset = generator.sample(rng).round() as i32;
            let mut swap_time = time_idx as i32 + offset;

            while offset == 0 || swap_time < 0 || swap_time >= matrix_len {
                offset = generator.sample(rng).round() as i32;
                swap_time = time_idx as i32 + offset;
            }

            // If so, add the event index and the index of this event in the
            // time allocation vector to the "list"
            mutations.push(event_idx);
            target_index.push(swap_time as usize);
        }

        // Remove the mutated events from this time slot
        events.retain(|x| !mutations.contains(x));
    }

    // Add the mutated events to the target time slots
    for (i, target_index) in target_index.iter().enumerate() {
        matrix[*target_index].push(mutations[i]);

        // Remove duplicates
        super::remove_duplicates(&mut matrix[*target_index]);
    }
}

////////////////////////////////////////////////////////////////////////////////
