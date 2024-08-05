// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::chromosome::Chromosome;
use rand::seq::SliceRandom;
use xhstt::db::Database;

// Functions ///////////////////////////////////////////////////////////////////
pub fn initialize(size: usize, db: &Database) -> Vec<Chromosome> {
    // Setup source of randomness
    let mut rng = rand::thread_rng();
    // let random_times = Uniform::new(0, db.times().len());

    println!("db.events = {}", db.events().len());

    // Create population
    let mut population = Vec::with_capacity(size);
    for _ in 0..size {
        let mut chromosome: Vec<u16> =
            (0..db.events().len()).map(|x| x as u16).collect();
        chromosome.shuffle(&mut rng);

        population.push(chromosome.into());
    }

    // Return
    population
}

////////////////////////////////////////////////////////////////////////////////
