//! Algorithm V7
//! This algorithm is the successor of "alg_3".
//! It's now build with my onw genetic algorithm "framework" to allow for
//! easy testing of advanced concepts like self-parameterization.
//!
//! Limitations:
//! 1. Only event-time allocation is missing (event resources are pre-defined).
//!

// Modules /////////////////////////////////////////////////////////////////////
// mod dynamics;
mod encoding;
mod operators;
// mod utils;

// Imports /////////////////////////////////////////////////////////////////////
// #[allow(unused)]
// use dynamics::Dynamic;
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

    // Check if the instance complies with the algorithms limitations
    // TODO

    // Initialize context and phenotype
    let ctx = Context::init(&db);
    let ph = Phenotype::blueprint(&db, &ctx);

    // Create encoding and parameters
    let encoding = ga::encoding::Builder::new()
        .set_context(ctx.clone())
        .set_phenotype(ph.clone())
        .build();

    let parameters = ga::parameters::Builder::for_encoding(&encoding)
        .set_population_size(100)
        .set_crossover_rate(None)
        .set_mutation_rate(0.01)
        .set_selection(Select::RouletteWheel)
        .set_crossover(Crossover::Ordered)
        .set_mutation(Mutation::UniformSwap)
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteRelative(0.05))
        // .set_termination(Terminate::ObjectiveValue(0.into()))
        .set_termination(Terminate::Generations(100))
        .build();

    // let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
    //     .set(vec![
    //         // (target_success_rate, k-factor, default std. deviation)
    //         // Dynamic::SuccessDrivenBetaDistrStdDeviation(0.05, 5., 0.2),
    //         // Dynamic::SuccessDrivenNormalDistrStdDeviation(0.01, 1., 10.),

    //         // Dynamic::VariableMutationRateCos(0.01, 0.25, 0.005),
    //         // Dynamic::VariablePopulationSizeCos(1_000, 500., 0.005)
    //     ])
    //     .build();

    // Create algorithm and let it run!
    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        // .set_dynamics(Some(dynamics))
        .set_dynamics::<()>(None)
        .build();

    let results = alg.run();

    // Get the best result and convert it to a list of solution events.
    let best: &Chromosome = &results.first().unwrap().0;
    let timetable: Phenotype = ph.derive(best, &ctx);

    timetable.to_solution_events(&db, &ctx)
}

////////////////////////////////////////////////////////////////////////////////
