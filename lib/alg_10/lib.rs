//! Algorithm V10:
//! - implements bitvector encoding (by Demirovic and Musilu)
//!

// Modules /////////////////////////////////////////////////////////////////////
// mod dynamics;
mod encoding;
// mod operators;

// Imports /////////////////////////////////////////////////////////////////////
// use dynamics::Dynamic;
use encoding::Context;
// use ga::{
//     encoding::Phenotype as _,
//     process::{
//         rejection::Reject, replacement::Replace, selection::Select,
//         termination::Terminate,
//     },
// };
// use operators::{Crossover, Mutation};
use xhstt::{
    db::Database,
    parser::{instances::Instance, solution_groups::solution::events::Event},
};

// Function ////////////////////////////////////////////////////////////////////
pub fn run(instance: Instance) -> Vec<Event> {
    // Create an XHSTT database of the problem instance
    let db = Database::init(&instance).unwrap();

    // Initialize context and phenotype
    let ctx = Context::init(&db);

    for _ in 0..10_000 {
        for e_idx in 0..ctx.num_events {
            encoding::EventGene::generate(ctx.durations[e_idx].into(), &ctx);
        }
    }

    println!("done");

    // Return
    vec![]
}

////////////////////////////////////////////////////////////////////////////////
