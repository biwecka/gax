//! Algorithm V5
//! This algorithm uses the "oxigen" genetic-algorithm-framework
//!
//! Limitations:
//!

// Modules /////////////////////////////////////////////////////////////////////
mod encoding;
mod operators;

use std::marker::PhantomData;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::{Context, Phenotype};
use ga::{process::{rejection::Reject, replacement::Replace, selection::Select, termination::Terminate}, Encoding, Parameters};
use operators::{Crossover, Mutation};

// Constants ///////////////////////////////////////////////////////////////////


// Algorithm ///////////////////////////////////////////////////////////////////
pub fn run() {
    let ctx = Context::init(16);
    let phenotype = Phenotype::blueprint(16);

    let encoding = Encoding {
        context: ctx,
        phenotype,

        objective_value: PhantomData,
        genotype: PhantomData,
    };

    let parameters = Parameters {
        population_size: 10,

        selection: Select::RouletteWheel,
        crossover: Crossover::VariableSinglePoint(0.9),
        mutation: Mutation::RandomizeNGenes(0.01, 8),
        rejection: Reject::None,

        replacement: Replace::EliteAbsolute(1),
        termination: Terminate::ObjectiveValue(0.into()),

        t: PhantomData,
        objective_value: PhantomData,
        context: PhantomData,
        genotype: PhantomData,
    };

    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        .build();
}

////////////////////////////////////////////////////////////////////////////////
