//! TODO: write crate documentation.
//!
//!

// Modules /////////////////////////////////////////////////////////////////////
mod crossover;
mod fitness;
mod mutation;
mod population;
mod selection;
mod stats;

// Imports /////////////////////////////////////////////////////////////////////
use fitness::Cost;
use population::Chromosome;
use xhstt::parser::instances::Instance;

// Constants ///////////////////////////////////////////////////////////////////
const POPULATION_SIZE: usize = 1_000;
const GENERATIONS: usize = 100;

// Algorithm ///////////////////////////////////////////////////////////////////

/// Run this algorithm.
pub fn run(instance: Instance) {
    // Turn instance into custom data structure.
    let data = xhstt::model::Data::init(&instance);
    let cstr = xhstt::model::Constraints::init(&instance);

    // Calculate important stats of the problem instance.
    let stats = stats::calc(&data);

    // Initialize population
    let mut population = population::initialize(POPULATION_SIZE, &stats);

    // Generation loop
    for gen_count in 1..=GENERATIONS {
        let start = std::time::Instant::now();

        // Evaluate population
        let mut curr_gen: Vec<(Chromosome, Cost)> = population
            .clone()
            .into_iter()
            .map(|chromosome| {
                let fitness = fitness::eval(&chromosome, &data, &cstr, &stats);
                (chromosome, fitness)
            })
            .collect();

        // Sort current generation (sort is always ascendingly)
        curr_gen.sort_by_key(|(_, cost)| std::cmp::Reverse(cost.0));

        // Print best cost
        println!("current best = {}", curr_gen.first().unwrap().1 .0);

        // Selection
        let parent_pairs =
            selection::roulette_wheel(POPULATION_SIZE / 2, curr_gen);

        // Crossover
        let mut children = crossover::single_point(parent_pairs, &stats);

        // Mutation
        children = mutation::random_single(children, &stats, 0.05);

        // Evaluate children
        // let mut children_eval: Vec<(Chromosome, Cost)> = children
        //     .into_iter()
        //     .map(|chromosome| {
        //         let fitness = fitness::eval(&chromosome, &data, &cstr, &stats);
        //         (chromosome, fitness)
        //     })
        //     .collect();

        // Sort children
        // children_eval.sort_by_key(|(_, cost)| std::cmp::Reverse(cost.0));

        // Replace
        let curr_best = population.first().cloned().unwrap();
        population = children;
        population.push(curr_best);

        // Print time
        println!("Generation {} took {:?}\n", gen_count, start.elapsed());
    }
}

////////////////////////////////////////////////////////////////////////////////
