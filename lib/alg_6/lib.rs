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
use ga::{process::{rejection::Reject, selection::Select}, Parameters};
use operators::{Crossover, Mutation};

// Constants ///////////////////////////////////////////////////////////////////


// Algorithm ///////////////////////////////////////////////////////////////////
pub fn run() {
    let ctx = Context::init(16);
    let phenotype = Phenotype::blueprint(16);

    let alg = ga::Algorithm::builder()
        .set_encoding(ctx, phenotype) //::<Cost, Context, Chromosome, Phenotype>
        .set_parameters(Parameters {
            population_size: 10,
            crossover: Crossover::VariableSinglePoint(0.9),
            mutation: Mutation::RandomizeNGenes(0.01, 8),

            selection: Select::RouletteWheel,
            rejection: Reject::None,

            t: PhantomData,
            objective_value: PhantomData,
            context: PhantomData,
            genotype: PhantomData,
        });
}

////////////////////////////////////////////////////////////////////////////////
