//! Algorithm V3
//!
//! Limitations:
//! 1. Only event-time allocation is missing (event resources are pre-defined).
//! 2. Events can only have a duration of 1.
//!
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
const POPULATION_SIZE: usize = 50;
const GENERATIONS: usize = 50_000;

// Algorithm ///////////////////////////////////////////////////////////////////

// Run this algorithm.
pub fn run(instance: Instance) -> Vec<SolutionEvent> {
    // Create an XHSTT database of the problem instance
    let db = xhstt::db::Database::init(&instance).unwrap();

    // Check if the instance complies with the algorithms limitations.
    assert!(utils::limitations::only_time_allocation_needed(&db));
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
                let allocation = allocation.derive(&chromosome);
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
        // #[allow(unused_assignments)]
        // let mut selection_method: String = "".into();
        // let parent_pairs = {
        //     if curr_best < curr_worst
        //         && curr_worst - curr_best < POPULATION_SIZE //(POPULATION_SIZE / 5)
        //     {
        //         selection_method = "rank".into();
        //         selection::rank(POPULATION_SIZE / 2, &curr_gen)
        //     } else {
        //         selection_method = "roulette".into();
        //         selection::roulette_wheel(POPULATION_SIZE / 2, &curr_gen)
        //     }
        // };

        let selection_method: String = "roulette (fixed)".into();
        let parent_pairs =
            selection::roulette_wheel(POPULATION_SIZE / 2, &curr_gen);

        let selected_avg = parent_pairs
            .iter()
            .map(|((_, c0), (_, c1))| c0 + c1)
            .sum::<usize>() as f32
            / (POPULATION_SIZE as f32);

        // Crossover
        let children = crossover::pmx(parent_pairs, &db);
        // let mut children = crossover::shift(parent_pairs, &db);

        // Mutation
        // children = mutation::random_multi_swap(children, 0.2, &db);

        // Inversion
        // children = mutation::inversion(children, 0.01);

        // Evaluate and sort children
        let mut children_eval: Vec<(Chromosome, usize)> = children
            .clone()
            .into_par_iter()
            // .into_iter()
            .map(|chromosome| {
                let allocation = allocation.derive(&chromosome);
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
            // POPULATION_SIZE / 10,
            1,
            curr_gen,
            children_eval,
        );

        // Print time
        if gen_count % 10 == 0 {
            println!(
            "Generation {} took {:.4?}: best={}, worst={} | {selection_method} | f(selected)' = {} | f' = {} | selection_differential = {}",
            gen_count,
            start.elapsed(),
            curr_best,
            curr_worst,
            selected_avg,
            curr_avg,
            selected_avg - curr_avg
        );
        }
    }

    // Get best individual
    let mut final_gen: Vec<(Chromosome, usize)> = population
        .clone()
        // .into_iter()
        .into_par_iter()
        .map(|chromosome| {
            let allocation = allocation.derive(&chromosome);
            let cost = fitness::calculate_cost(&allocation, &constraints);
            (chromosome, cost)
        })
        .collect();
    final_gen
        // .sort_by_key(|(_, cost)| *cost);
        .par_sort_by_key(|(_, cost)| *cost);

    let best = final_gen.first().unwrap();
    println!("final best = {}", best.1);

    println!(">>> END <<<");

    // Return
    utils::solution::create_from(&best.0, &allocation, &db)
}

////////////////////////////////////////////////////////////////////////////////
