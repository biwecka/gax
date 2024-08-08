//! Algorithm V2
//! This algorithm improves on V1 by storing the data/allocations in a matrix,
//! represented as vector. This improves cache locality and therefore boosts
//! performance by a lot.
//! In addition o that `rayon` is used to fully utilize multi-core CPUs.
//!
//! The chosen encoding is the same as in V1: a simple time-to-event allocation
//!
//! Limitations:
//! 1. Only event-time allocation is missing (event resources are pre-defined).
//! 2. Events can only have a duration of 1.
//! 3. Allocation's "event_2_time" map is exactly as long as the chromosome.
//! 4. All constraints are requred (hard constraints).
//! 5. No OrderEventsConstraint (because AppliesToEventPair can't be pre-calced)
//!
//! Results:
//! -   V1 had a runtime per generation (population size = 64) of 50ms
//!     V2 reduced it to 7ms (without rayon) and ~2ms with rayon.
//!

// Modules /////////////////////////////////////////////////////////////////////
mod crossover;
mod encoding;
mod fitness;
mod mutation;
mod population;
mod replace;
mod selection;
mod utils;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::chromosome::Chromosome;
use rayon::prelude::*;
use xhstt::parser::{
    instances::Instance,
    solution_groups::solution::events::Event as SolutionEvent,
};

// Constants ///////////////////////////////////////////////////////////////////
const POPULATION_SIZE: usize = 64;
const GENERATIONS: usize = 500_000;

// Algorithm ///////////////////////////////////////////////////////////////////

// Run this algorithm.
pub fn run(instance: Instance) -> Vec<SolutionEvent> {
    // Create an XHSTT database of the problem instance
    let db = xhstt::db::Database::init(&instance).unwrap();

    // Check if the instance complies with the algorithms limitations.
    assert!(utils::limitations::only_time_allocation_needed(&db));
    assert!(utils::limitations::only_duration_of_1(&db));
    assert!(utils::limitations::allocation_and_chromosome_same_length(&db));
    assert!(utils::limitations::only_hard_constraints(&db));

    // --- Prelude --- //
    // Create base allocation (as defined in XHSTT problem instance).
    let allocation = encoding::allocation::Allocation::init(&db);

    // Pre-calculate the affected indices of the constraints.
    let constraints = fitness::pre_calc_constraints(db.contraints(), &db);

    // --- Initialization --- //
    let mut population = population::initialize(POPULATION_SIZE, &db);

    // --- Loop --- //
    for gen_count in 1..=GENERATIONS {
        let start = std::time::Instant::now();

        // Evaluate population
        let mut curr_gen: Vec<(Chromosome, usize)> = population
            .clone()
            // .into_iter()
            .into_par_iter()
            .map(|chromosome| {
                let allocation = allocation.derive(chromosome.clone());
                let cost = fitness::calculate_cost(&allocation, &constraints);
                (chromosome, cost)
            })
            .collect();

        // Sort current generation
        curr_gen
            // .sort_by_key(|(_, cost)| *cost);
            .par_sort_by_key(|(_, cost)| *cost);

        // Stats
        let curr_best = curr_gen.first().unwrap().1;
        let curr_worst = curr_gen.last().unwrap().1;
        let curr_avg = curr_gen.iter().map(|(_, cost)| cost).sum::<usize>()
            as f32
            / (POPULATION_SIZE as f32);

        // Selection
        let parent_pairs = {
            if curr_best > curr_worst
                && curr_best - curr_worst < (POPULATION_SIZE)
            {
                selection::rank(POPULATION_SIZE / 2, &curr_gen)
            } else {
                selection::roulette_wheel(POPULATION_SIZE / 2, &curr_gen)
            }
        };
        // let parent_pairs = selection::roulette_wheel(POPULATION_SIZE / 2, &curr_gen);

        let selected_avg = parent_pairs
            .iter()
            .map(|((_, c0), (_, c1))| c0 + c1)
            .sum::<usize>() as f32
            / (POPULATION_SIZE as f32);

        // Crossover
        let mut children = crossover::static_single_point(parent_pairs, &db);

        // Mutation
        children = mutation::random_single(children, 0.4, &db);

        // Evaluate and sort children
        let mut children_eval: Vec<(Chromosome, usize)> = children
            .clone()
            .into_par_iter()
            .map(|chromosome| {
                let allocation = allocation.derive(chromosome.clone());
                let cost = fitness::calculate_cost(&allocation, &constraints);
                (chromosome, cost)
            })
            .collect();

        // Sort current generation
        children_eval
            // .sort_by_key(|(_, cost)| *cost);
            .par_sort_by_key(|(_, cost)| *cost);

        // Replace
        population = replace::elite_best_n(
            POPULATION_SIZE / 10,
            curr_gen,
            children_eval,
        );

        // Print time
        // if gen_count % 1_000 == 0 {
        println!(
            "Generation {} took {:.4?}: best={}, worst={} | f(selected)' = {} | f' = {} | selection_differential = {}",
            gen_count,
            start.elapsed(),
            curr_best,
            curr_worst,
            selected_avg,
            curr_avg,
            selected_avg - curr_avg
        );
        // }
    }

    // Get best individual
    let mut final_gen: Vec<(Chromosome, usize)> = population
        .clone()
        .into_iter()
        .map(|chromosome| {
            let allocation = allocation.derive(chromosome.clone());
            let cost = fitness::calculate_cost(&allocation, &constraints);
            (chromosome, cost)
        })
        .collect();
    final_gen.sort_by_key(|(_, cost)| *cost);

    let best = final_gen.first().unwrap();
    println!("final best = {}", best.1);

    println!(">>> END <<<");

    // Return
    utils::solution::create_from(&best.0, &db)
}

////////////////////////////////////////////////////////////////////////////////
