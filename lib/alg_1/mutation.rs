// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    population::{Chromosome, Gene},
    stats::Stats,
};
use rand::{distributions::Uniform, prelude::Distribution};

// Functions ///////////////////////////////////////////////////////////////////
pub fn random_single(
    mut children: Vec<Chromosome>,
    stats: &Stats,
    mutation_propability: f32,
) -> Vec<Chromosome> {
    // let mut mutated_children = Vec::with_capacity(children.len());

    // Get chromosome length
    let chr_len = stats.events.len();

    // Randomness
    let mut rng = rand::thread_rng();
    let index = Uniform::new(0, chr_len);
    let times = Uniform::new(0, stats.times);
    let probabilty = Uniform::new_inclusive(0., 1.);

    // Iterate parent pairs and perform crossover
    for child in children.iter_mut() {
        if probabilty.sample(&mut rng) > mutation_propability {
            continue;
        }

        let i = index.sample(&mut rng);
        let t = times.sample(&mut rng);

        child.0[i] = Gene(t);
    }

    // Return
    children
}

////////////////////////////////////////////////////////////////////////////////
