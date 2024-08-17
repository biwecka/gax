//! Algorithm V5
//! This algorithm uses the "oxigen" genetic-algorithm-framework
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
    let ctx = Context::init(16);
    let phenotype = Phenotype::blueprint(16);

    let encoding = ga::encoding::Builder::new()
        .set_context(ctx)
        .set_phenotype(phenotype)
        .build();

    let parameters = ga::parameters::Builder::for_encoding(&encoding)
        .set_population_size(10)
        .set_selection(Select::RouletteWheel)
        .set_crossover(Crossover::VariableSinglePoint(0.9))
        .set_mutation(Mutation::RandomizeNGenes(0.01, 8))
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteAbsolute(1))
        .set_termination(Terminate::ObjectiveValue(0.into()))
        .build();

    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        .build();
}

////////////////////////////////////////////////////////////////////////////////
