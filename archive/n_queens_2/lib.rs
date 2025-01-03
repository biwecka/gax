//! Algorithm n_queens_2
//! This is a re-implementation of the n_queens algorithm, but using my own
//! genetic algorithm framwork.
//!
//! Limitations:
//!

// Modules /////////////////////////////////////////////////////////////////////
mod encoding;
mod operators;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::{Context, Phenotype};
use ga::process::{
    rejection::Reject, replacement::Replace, selection::Select,
    termination::Terminate,
};
use operators::{Crossover, Mutation};

// Constants ///////////////////////////////////////////////////////////////////

// Algorithm ///////////////////////////////////////////////////////////////////
pub fn run() {
    let ctx = Context::init(128);
    let phenotype = Phenotype::blueprint(128);

    let encoding = ga::encoding::Builder::new()
        .set_context(ctx)
        .set_phenotype(phenotype)
        .build();

    let parameters = ga::parameters::Builder::for_encoding(&encoding)
        .set_population_size(500)
        .set_crossover_rate(None)
        .set_mutation_rate(0.01)
        .set_selection(Select::RouletteWheel)
        .set_crossover(Crossover::VariableNPoint(8))
        .set_mutation(Mutation::RandomValue)
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteRelative(0.01))
        .set_termination(Terminate::ObjectiveValue(0.into()))
        .build();

    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        .set_dynamics::<()>(None)
        .set_custom_logger::<()>(None)
        .build();

    let _solutions = alg.run();
}

////////////////////////////////////////////////////////////////////////////////
