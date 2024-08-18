//! Algorithm V6
//!

// Modules /////////////////////////////////////////////////////////////////////
mod encoding;
mod operators;
mod utils;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::{Context, Phenotype};
use ga::process::{
    rejection::Reject, replacement::Replace, selection::Select,
    termination::Terminate,
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

    // Check if the instance complies with the algorithms limitations
    // TODO

    // Initialize context and phenotype
    let ctx = Context::init(&db);
    let ph = Phenotype::blueprint(&db, &ctx);

    // Create encoding and parameters
    let encoding =
        ga::encoding::Builder::new().set_context(ctx).set_phenotype(ph).build();

    let parameters = ga::parameters::Builder::for_encoding(&encoding)
        .set_population_size(2_000)
        .set_selection(Select::Tournament(2))
        .set_crossover(Crossover::VariableNPoint(1., 10))
        .set_mutation(Mutation::RandomizeNGenes(0.6, 1))
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteAbsolute(1))
        .set_termination(Terminate::Generations(500_000))
        .build();

    // Create algorithm and let it run!
    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        .build();

    let _results = alg.run();

    // TODO: Ergebnis ausgeben und online checken, ob das passt!
    vec![]
}

////////////////////////////////////////////////////////////////////////////////
