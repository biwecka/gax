//! Algorithm V11:
//! - implements bitvector encoding (by Demirovic and Musilu)
//!

// Modules /////////////////////////////////////////////////////////////////////

/// The dynamics module must be public for the auto-runner to construct the
/// algorithm's configuration.
pub mod dynamics;

/// Encoding module.
mod encoding;

/// The operators module must also be public for the auto-runner to construct
/// the algorithm's configuration.
pub mod operators;

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
        .set_mutation_rate(0.010)
        .set_selection(Select::LinearRank(2.0))
        .set_crossover(Crossover::Trade(1))
        .set_mutation(Mutation::Trade)
        .set_rejection(Reject::None)
        .set_replacement(Replace::EliteAbsolute(1))
        .set_termination(Terminate::GenOrOv(500_000, 0.into()))
        .build();

    #[allow(unused)]
    let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
        .set(vec![
            // Dynamic::MutationRateCos(0.01, 0.1, 0.001),
            // Dynamic::GaussRandomTime(0.01),  // for GaussMoveSingleTimeAlloc
            // Dynamic::GaussRandomEvent(0.01), // for GaussTrade
            // Dynamic::TargetMeanByVariableMutationRate(1.2, 0.005_000),
            // Dynamic::IncreasingLinearRankSelectionPressure,
            // Dynamic::RotatingMutationMethods,
            Dynamic::StateMachine,
        ])
        .build();

    // Create algorithm and let it run!
    let alg = ga::Builder::new()
        .set_encoding(encoding)
        .set_parameters(parameters)
        .set_dynamics::<()>(None)
        // .set_dynamics(Some(dynamics))
        .set_custom_logger::<()>(None)
        .build();

    let report = alg.run();

    // Get the best result and convert it to a list of solution events.
    let best: &Chromosome = &report.population.first().unwrap().0;
    let timetable: Phenotype = ph.derive(best, &ctx);

    timetable.to_solution_events(&db, &ctx)
}

////////////////////////////////////////////////////////////////////////////////
pub struct AutoRunParameters {
    pub population_size: usize,
    pub mutation_rate: f32,
    pub selection: ga::process::selection::Select,
    pub crossover: crate::operators::Crossover,
    pub mutation: crate::operators::Mutation,
    pub replacement: ga::process::replacement::Replace,
}

pub fn auto_run(
    instance: Instance,
    params: AutoRunParameters,
    dynamics: Option<Vec<()>>,
) -> Vec<Event> {
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
        .set_population_size(params.population_size)
        .set_crossover_rate(None)
        .set_mutation_rate(params.mutation_rate)
        .set_selection(params.selection)
        .set_crossover(params.crossover)
        .set_mutation(params.mutation)
        .set_rejection(Reject::None)
        .set_replacement(params.replacement)
        .set_termination(Terminate::GenOrOv(500_000, 0.into()))
        .build();

    let alg = match dynamics {
        Some(d) => {
            let dynamics = ga::dynamics::Builder::for_parameters(&parameters)
                .set(d)
                .build();

            ga::Builder::new()
                .set_encoding(encoding)
                .set_parameters(parameters)
                .set_dynamics(Some(dynamics))
                .set_custom_logger::<()>(None)
                .build()
        }

        None => ga::Builder::new()
            .set_encoding(encoding)
            .set_parameters(parameters)
            .set_dynamics::<()>(None)
            .set_custom_logger::<()>(None)
            .build(),
    };

    let report = alg.run();

    // Get the best result and convert it to a list of solution events.
    let best: &Chromosome = &report.population.first().unwrap().0;
    let timetable: Phenotype = ph.derive(best, &ctx);

    timetable.to_solution_events(&db, &ctx)
}

////////////////////////////////////////////////////////////////////////////////
