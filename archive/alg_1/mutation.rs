// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    population::{Chromosome, Gene},
    stats::Stats,
};
use rand::{distributions::Uniform, prelude::Distribution};

// Functions ///////////////////////////////////////////////////////////////////
#[allow(unused)]
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

    // Iterate and mutate
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

#[allow(unused)]
pub fn random_multi(
    mut children: Vec<Chromosome>,
    stats: &Stats,
    mutation_propability: f32,
) -> Vec<Chromosome> {
    // Get chromosome length
    let chr_len = stats.events.len();

    // Randomness
    let mut rng = rand::thread_rng();
    let index = Uniform::new(0, chr_len);
    let times = Uniform::new(0, stats.times);
    let probability = Uniform::new_inclusive(0., 1.);

    // Iterate and mutate
    for child in children.iter_mut() {
        if probability.sample(&mut rng) > mutation_propability {
            continue;
        }

        let i0 = index.sample(&mut rng);
        let t0 = times.sample(&mut rng);

        child.0[i0] = Gene(t0);

        let i1 = index.sample(&mut rng);
        let t1 = times.sample(&mut rng);

        child.0[i1] = Gene(t1);

        let i2 = index.sample(&mut rng);
        let t2 = times.sample(&mut rng);

        child.0[i2] = Gene(t2);
    }

    // Return
    children
}

////////////////////////////////////////////////////////////////////////////////
