//! Algorithm V10:
//! - implements bitvector encoding (by Demirovic and Musilu)
//!

// Modules /////////////////////////////////////////////////////////////////////
mod dynamics;
mod encoding;
mod operators;

use dynamics::Dynamic;
// Imports /////////////////////////////////////////////////////////////////////
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
        .set_population_size(1_000)
        .set_crossover_rate(None)
        .set_mutation_rate(0.01)
        .set_selection(Select::RouletteWheel)
        .set_crossover(Crossover::Trade(1))
        .set_mutation(Mutation::Trade)
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteAbsolute(1))
        // .set_termination(Terminate::Generations(50_000))
        .set_termination(Terminate::ObjectiveValue(0.into()))
        .build();

    let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
        .set(vec![
            Dynamic::MutationRateCos(0.01, 0.1, 0.001)
        ])
        .build();

    // Create algorithm and let it run!
    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        // .set_dynamics::<()>(None)
        .set_dynamics(Some(dynamics))
        .set_custom_logger::<()>(None)
        .build();

    let results = alg.run();

    // Get the best result and convert it to a list of solution events.
    let best: &Chromosome = &results.first().unwrap().0;
    let timetable: Phenotype = ph.derive(best, &ctx);

    timetable.to_solution_events(&db, &ctx)
}

////////////////////////////////////////////////////////////////////////////////
