//! Algorithm V7
//! This algorithm is the successor of "alg_3".
//! It's now build with my onw genetic algorithm "framework" to allow for
//! easy testing of advanced concepts like self-parameterization.
//!
//! Limitations:
//! 1. Only event-time allocation is missing (event resources are pre-defined).
//!
//! Best Result:
//! - Cost = 0 (in the first generation; multiple times in a row; something must
//!   have been messed up with the random number generator or something, but
//!   the results were valid based on HSEval)
//!
//! - Cost = 0 (after learning that it's allowed to split up events of duration
//!            > 1 into multiple sub-events)
//!

// Modules /////////////////////////////////////////////////////////////////////
mod dynamics;
mod encoding;
mod logger;
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
        .set_population_size(1_000)
        .set_crossover_rate(None)
        .set_mutation_rate(0.01)
        .set_selection(Select::Tournament(10))
        .set_crossover(Crossover::Ordered)
        .set_mutation(Mutation::GaussSwap)
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteRelative(0.01))
        .set_termination(Terminate::ObjectiveValue(0.into()))
        // .set_termination(Terminate::Generations(20))
        .build();

    // let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
    //     .set(vec![
    //         // (target_success_rate, k-factor, default std. deviation)
    //         Dynamic::SuccessDrivenNormalDistrStdDeviation(0.05, 1., 1.),
    //     ])
    //     .build();

    // Create algorithm and let it run!
    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        // .set_dynamics(Some(dynamics))
        .set_dynamics::<()>(None)
        // .set_custom_logger(Some(logger::Logger::default()))
        .set_custom_logger::<()>(None)
        .build();

    let report = alg.run();

    // Get the best result and convert it to a list of solution events.
    let best: &Chromosome = &report.population.first().unwrap().0;
    let timetable: Phenotype = ph.derive(best, &ctx);

    timetable.to_solution_events(&db, &ctx)
}

////////////////////////////////////////////////////////////////////////////////
