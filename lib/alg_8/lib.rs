//! Algorithm V8:
//! - direct encoding (like alg_2 & alg_6)
//! - assigns multiple events to a time (instead of assigning a time to an
//!   event like alg_2 & alg_6)
//! - this encoding is inspired by:
//!     - "A parallel genetic algorithm for solving the school timetabling
//!        problem" (by Abramson and Abela; 1992)
//!     - Making a class schedule using a genetic algorithm:
//!       https://www.codeproject.com/Articles/23111/Making-a-Class-Schedule-Using-a-Genetic-Algorithm
//!

// Modules /////////////////////////////////////////////////////////////////////
mod dynamics;
mod encoding;
mod operators;

// Imports /////////////////////////////////////////////////////////////////////
use dynamics::Dynamic;
use encoding::{Chromosome, Context, Phenotype};
use ga::{
    encoding::Phenotype as _,
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
};
use operators::{Crossover, Mutation};
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
    let ph = Phenotype::blueprint(&db, &ctx);

    // Create encoding and parameters
    let encoding = ga::encoding::Builder::new()
        .set_context(ctx.clone())
        .set_phenotype(ph.clone())
        .build();

    let parameters = ga::parameters::Builder::for_encoding(&encoding)
        .set_population_size(2_000)
        .set_crossover_rate(None)
        .set_mutation_rate(0.01)
        .set_selection(Select::Tournament(4))
        .set_crossover(Crossover::VariableSinglePoint)
        .set_mutation(Mutation::GaussSwap)
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteRelative(0.01))
        // .set_termination(Terminate::Generations(10))
        .set_termination(Terminate::ObjectiveValue(0.into()))
        .build();

    let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
        .set(vec![Dynamic::SuccessDrivenGaussDistrStdDeviation(0.05, 0.1, 1.)])
        .build();

    // Create algorithm and let it run!
    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        // .set_dynamics::<()>(None)
        .set_dynamics(Some(dynamics))
        .build();

    let results = alg.run();

    // Get the best result and convert it to a list of solution events
    let best: &Chromosome = &results.first().unwrap().0;
    let timetable: Phenotype = ph.derive(best, &ctx);

    timetable.to_solution_events(&db, &ctx)
}

////////////////////////////////////////////////////////////////////////////////
