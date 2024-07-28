//! TODO: write crate documentation.
//!
//!

// Modules /////////////////////////////////////////////////////////////////////
mod fitness;
mod population;
mod selection;
mod stats;

// Imports /////////////////////////////////////////////////////////////////////
use fitness::Cost;
use population::Chromosome;
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

    // Generation loop
    for gen_count in 1..=8 {
        let start = std::time::Instant::now();

        // Evaluate population
        let mut curr_gen: Vec<(Chromosome, Cost)> = population.clone()
            .into_iter()
            .map(|chromosome| {
                let fitness = fitness::eval(&chromosome, &data, &cstr, &stats);
                (chromosome, fitness)
            })
            .collect();

        // Sort current generation (sort is always ascendingly)
        curr_gen.sort_by_key(|(_, cost)| std::cmp::Reverse(cost.0));

        // Selection
        let parent_pairs = selection::roulette_wheel(50, curr_gen);

        // Print time
        println!("Generation {} took {:?}", gen_count, start.elapsed());
    }
}

////////////////////////////////////////////////////////////////////////////////
