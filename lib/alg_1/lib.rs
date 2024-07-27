//! TODO: write crate documentation.
//!
//!

// Modules /////////////////////////////////////////////////////////////////////
mod fitness;
mod population;
mod selection;
mod stats;

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
    let population = population::initialize(100, &stats);

    let start = std::time::Instant::now();

    let mut f = None;

    for chromosome in population {
        let val = fitness::eval(&chromosome, &data, &cstr, &stats);
        if f.is_none() {
            f = Some(val);
        }
    }

    let elapsed = start.elapsed();

    println!("{f:?}",);
    println!("elapsed = {elapsed:?}");
}

////////////////////////////////////////////////////////////////////////////////
