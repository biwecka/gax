//! TODO: write crate documentation.
//!
//!

// Modules /////////////////////////////////////////////////////////////////////
mod fitness;
mod population;
mod selection;
mod stats;

use fitness::Fitness;
use population::Chromosome;
// Imports /////////////////////////////////////////////////////////////////////
use xhstt::parser::instances::Instance;

// Algorithm ///////////////////////////////////////////////////////////////////

/// Run this algorithm.
pub fn run(instance: Instance) {
    // Turn instance into custom data structure.
    let data = xhstt::model::Data::init(&instance);
    let cstr = xhstt::model::Constraints::init(&instance);

    // Calculate important stats of the problem instance.
    let stats = stats::calc(&data);

    // Initialize population
    let mut population = population::initialize(100, &stats);

    for generation in 1..=2 {
        let start = std::time::Instant::now();

        // Evaluate population
        let evalpop: Vec<(Chromosome, Fitness)> = population.clone()
            .into_iter()
            .map(|chromosome| {
                let fitness = fitness::eval(&chromosome, &data, &cstr, &stats);
                (chromosome, fitness)
            })
            .collect();

        // Print time
        println!("Generation {} took {:?}", generation, start.elapsed());
    }
}

////////////////////////////////////////////////////////////////////////////////
