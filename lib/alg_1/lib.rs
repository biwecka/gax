//! TODO: write crate documentation.
//!
//!

// Modules /////////////////////////////////////////////////////////////////////
mod fitness;
mod population;
mod selection;
mod stats;

// Imports /////////////////////////////////////////////////////////////////////
use xhstt::model::instances::Instance;

// Algorithm ///////////////////////////////////////////////////////////////////

/// Run this algorithm.
pub fn run(instance: Instance) {
    // Calculate important stats of the problem instance.
    let stats = stats::calc(&instance);

    // Initialize population
    let population = population::initialize(100, &stats);

    let fitness = fitness::eval(
        population.first().unwrap(),
        &instance,
        &stats
    );

    println!("{fitness:?}");
}

////////////////////////////////////////////////////////////////////////////////
