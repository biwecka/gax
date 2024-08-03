//! Algorithm V2
//!
//! Limitations:
//! 1. Maximum of 127 times
//! 2. Maximum of 127 per resource (teachers, rooms, classes)
//! 3. No possible to assign multiple resources of one type (e.g. teachers) to
//!    one event.
//! 4. Every events has to have the same 3 resources: teacher, class, room
//!

// Modules /////////////////////////////////////////////////////////////////////
// mod crossover;
// mod fitness;
// mod mutation;
// mod population;
// mod selection;
// mod replace;
mod encoding;
mod stats;

// Imports /////////////////////////////////////////////////////////////////////

use xhstt::parser::{
    instances::Instance,
    solution_groups::solution::events::Event as SolutionEvent,
};

// Constants ///////////////////////////////////////////////////////////////////
const POPULATION_SIZE: usize = 64;
const GENERATIONS: usize = 1_000_000;

// Algorithm ///////////////////////////////////////////////////////////////////

// Run this algorithm.
pub fn run(instance: Instance) -> Vec<SolutionEvent> {
    // let (encoding, groupings, idmaps) = encoding::init(&instance);

    let xdb = xhstt::db::Database::init(&instance).unwrap();

    // Return
    vec![]
}

////////////////////////////////////////////////////////////////////////////////
