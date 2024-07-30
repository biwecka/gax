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
mod replace;

// Imports /////////////////////////////////////////////////////////////////////
use fitness::Cost;
use population::Chromosome;
use xhstt::parser::instances::Instance;
use xhstt::parser::solution_groups::solution::events::Event as SolutionEvent;

// Constants ///////////////////////////////////////////////////////////////////
const POPULATION_SIZE: usize = 64;
const GENERATIONS: usize = 50_000;

// Algorithm ///////////////////////////////////////////////////////////////////

/// Run this algorithm.
pub fn run(instance: Instance) -> Vec<SolutionEvent> {
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
        curr_gen.sort_by_key(|(_, cost)| cost.0);
        // curr_gen.sort_by_key(|(_, cost)| std::cmp::Reverse(cost.0));

        // Print best cost
        let curr_best = curr_gen.first().unwrap().1.0;
        let curr_worst = curr_gen.last().unwrap().1.0;
        // println!("current best = {}", curr_best);
        // println!("current last = {}", curr_worst);

        // Selection
        let parent_pairs = {
            if curr_best > curr_worst && curr_best - curr_worst < 50 {
                selection::rank(POPULATION_SIZE / 2, &curr_gen)
            } else {
                selection::roulette_wheel(POPULATION_SIZE / 2, &curr_gen)
            }
        };

        // Crossover
        let mut children = crossover::changing_multi_point(parent_pairs, &stats);

        // Mutation
        children = mutation::random_single(children, &stats, 0.5);


        // Evaluate and sort children
        let mut children_eval: Vec<(Chromosome, Cost)> = children
            .clone()
            .into_iter()
            .map(|chromosome| {
                let fitness = fitness::eval(&chromosome, &data, &cstr, &stats);
                (chromosome, fitness)
            })
            .collect();

        // Sort current generation (sort is always ascendingly)
        children_eval.sort_by_key(|(_, cost)| cost.0);


        // Replace
        population = replace::elite_best_n(8, curr_gen, children_eval);

        // Print time
        println!("Generation {} took {:?}: best={}, worst={}", gen_count, start.elapsed(), curr_best, curr_worst);
    }

    // Get best individual
    let mut final_gen: Vec<(Chromosome, Cost)> = population
        .clone()
        .into_iter()
        .map(|chromosome| {
            let fitness = fitness::eval(&chromosome, &data, &cstr, &stats);
            (chromosome, fitness)
        })
        .collect();

    final_gen.sort_by_key(|(_, cost)| cost.0);

    let best_solution = final_gen.first().unwrap();
    println!("final best = {}", best_solution.1.0);

    println!(">>> END <<<");

    // Return
    convert_to_solution(&best_solution.0, &data, &stats)
}

fn convert_to_solution(
    chromosome: &Chromosome,
    data: &xhstt::model::Data,
    stats: &crate::stats::Stats,
) -> Vec<SolutionEvent> {
    let mut events = vec![];

    for (locus, gene) in chromosome.0.iter().enumerate() {
        // Translate locus and gene
        let event_index = locus;
        let time_index = gene.0;

        // Get event and time
        let event_id = stats.events.get(event_index).unwrap().clone();
        let event = data.get_event_by_id(&event_id);
        let time = data.get_time_by_idx(time_index).clone();

        // Create event
        events.push(SolutionEvent {
            reference: event.id.0.clone(),
            // duration: Some(event.duration),
            duration: None,
            resources: None,
            time: Some(
                xhstt::parser::solution_groups::solution::events::TimeRef {
                    reference:time.id.0
                }
            ),
        })
    }

    // Return
    events
}

////////////////////////////////////////////////////////////////////////////////
