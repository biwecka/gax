// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::chromosome::Chromosome;
use rand::{distributions::Uniform, prelude::Distribution};
use xhstt::db::Database;

// Functions ///////////////////////////////////////////////////////////////////
pub fn initialize(size: usize, db: &Database) -> Vec<Chromosome> {
    // Setup uniform distribution and source of randomness
    let mut rng = rand::thread_rng();
    let random_times = Uniform::new(0, db.times().len());

    // Create population
    let mut population = Vec::with_capacity(size);
    for _ in 0..size {
        let mut chromosome: Vec<u8> = vec![0; db.events().len()];
        for gene in chromosome.iter_mut() {
            *gene = random_times.sample(&mut rng) as u8;
        }

        population.push(chromosome.into());
    }

    // Return
    population
}

////////////////////////////////////////////////////////////////////////////////
