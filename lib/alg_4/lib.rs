//! Algorithm V4
//! This algorithm uses the "genevo" genetic-algorithm-framework
//!
//! Limitations:
//!

// Modules /////////////////////////////////////////////////////////////////////
mod utils;

use std::collections::HashSet;

// Imports /////////////////////////////////////////////////////////////////////
use genevo::{
    mutation::value::RandomValueMutator, operator::prelude::*,
    population::ValueEncodedGenomeBuilder, prelude::*,
};
use utils::Matrix2D;
use xhstt::{
    db::constraints::{
        avoid_clashes_constraint::AvoidClashesConstraint, Constraint,
    },
    parser::{
        instances::Instance,
        solution_groups::solution::events::Event as SolutionEvent,
    },
};

// Constants ///////////////////////////////////////////////////////////////////
const POPULATION_SIZE: usize = 100;
const GENERATIONS: usize = 20_000;

// Algorithm ///////////////////////////////////////////////////////////////////

pub type Chromosome = Vec<u8>;

/// Get all times that are allocated to the given events. Duplicates are
/// removed.
pub fn times_by_events(
    chromosome: &Chromosome,
    event_idxs: &[usize],
) -> Vec<u8> {
    let mut times: HashSet<u8> = HashSet::new();

    for event_idx in event_idxs {
        times.insert(chromosome[*event_idx]);
    }

    times.into_iter().collect()
}

#[derive(Clone, Debug)]
pub struct Problem {
    /// Time allocation vector.
    /// This vector assigns times to events by index: f(event_idx) -> time_idx.
    /// Time indices are in [0; 30), therefore u8 is sufficient.
    //  times: Vec<u8>,

    /// Resource allocation matrix.
    /// This matrix contains true and false values for every event-resource-
    /// combination, representing if a resource is assigned to an event.
    /// m(resource_idx, event_idx) -> bool
    resources: Matrix2D<bool>,

    /// Constraints
    constraints: Vec<(Constraint, Vec<usize>)>,
}

impl Problem {
    pub fn new(db: &xhstt::db::Database) -> Self {
        // Initialize resource matrix
        let mut resources =
            Matrix2D::init(db.resources().len(), db.events().len());

        // Fill resource matrix
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                resources.set_cell(resource_idx, event_idx, true);
            }
        }

        // Initialize constraints vector
        let mut constraints = vec![];

        // Precalculate constraints
        for constraint in db.contraints() {
            let indices = match constraint {
                Constraint::AssignTimeConstraint(x) => {
                    x.applies_to.resolve_idxs(db)
                }
                Constraint::AvoidClashesConstraint(x) => {
                    x.applies_to.resolve_idxs(db)
                }
            };

            constraints.push((constraint.clone(), indices));
        }

        // Return
        Self { resources, constraints }
    }

    /// Get the events (indices) that are assigned to the given resource.
    pub fn events_by_resource(&self, resource_idx: usize) -> Vec<usize> {
        self.resources
            .get_row(resource_idx)
            .iter()
            .enumerate()
            .filter_map(|(i, value)| match value {
                true => Some(i),
                false => None,
            })
            .collect()
    }
}

impl<'a> FitnessFunction<Chromosome, usize> for &'a Problem {
    fn fitness_of(&self, chromosome: &Chromosome) -> usize {
        let mut total_cost = 0;

        for (constraint, indices) in &self.constraints {
            match constraint {
                Constraint::AssignTimeConstraint(_) => {}
                Constraint::AvoidClashesConstraint(params) => {
                    total_cost += avoid_clashes_constraint(
                        self,       // problem (resource allocation)
                        chromosome, // chromosome (time allocation)
                        params,     // constraint parameters
                        indices,    // event indices (precalculation)
                    )
                }
            }
        }

        1_000 - total_cost
    }

    fn average(&self, values: &[usize]) -> usize {
        (values.iter().sum::<usize>() as f32 / values.len() as f32 + 0.5)
            .floor() as usize
    }

    fn highest_possible_fitness(&self) -> usize {
        // 0
        1_000
    }

    fn lowest_possible_fitness(&self) -> usize {
        // 400
        0
    }
}

fn avoid_clashes_constraint(
    problem: &Problem,
    chromosome: &Chromosome,
    // allocation: &Allocation,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize],
) -> usize {
    // Deviation
    let deviation: usize = resource_idxs
        // .par_iter()
        .iter()
        .map(|resource_idx| {
            // Get events by resource
            let event_idxs = problem.events_by_resource(*resource_idx);

            if event_idxs.len() < 2 {
                return 0;
            }

            // Get all times allocated to the events
            let times = times_by_events(&chromosome, &event_idxs);

            // If the times list is shorter than the event list, this means that
            // some events have the same time assigned.
            if times.len() < event_idxs.len() {
                event_idxs.len() - times.len()
            } else {
                0
            }
        })
        .sum();

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

pub fn run(instance: Instance) -> Vec<SolutionEvent> {
    // Create an XHSTT database of the problem instance
    let db = xhstt::db::Database::init(&instance).unwrap();

    let problem = Problem::new(&db);

    let initial_population: Population<Chromosome> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(
            db.events().len(),
            0,
            db.times().len() as u8,
        ))
        .of_size(POPULATION_SIZE)
        .uniform_at_random();

    let mut xhstt_sim = simulate(
        genetic_algorithm()
            .with_evaluation(&problem)
            .with_selection(RouletteWheelSelector::new(1., 2))
            .with_crossover(MultiPointCrossBreeder::new(2))
            .with_mutation(RandomValueMutator::new(
                0.01,
                0,
                db.times().len() as u8,
            ))
            .with_reinsertion(ElitistReinserter::new(&problem, false, 0.95))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(GenerationLimit::new(GENERATIONS as u64))
    .build();

    'sim: loop {
        let result = xhstt_sim.step();

        match result {
            Ok(SimResult::Intermediate(step)) => {
                print!("gen: {: >6} | ", step.iteration);
                print!("time: {:.4?} ms | ", step.duration.num_milliseconds());
                print!(
                    "best: {} | ",
                    1_000 - step.result.best_solution.solution.fitness
                );
                print!(
                    "worst: {} | ",
                    1_000 - step.result.evaluated_population.lowest_fitness()
                );
                print!(
                    "avg: {} | ",
                    1_000 - step.result.evaluated_population.average_fitness()
                );
                println!();
            }

            Ok(SimResult::Final(step, _time, _duration, _stop_reason)) => {
                print!("gen: {: >6} | ", step.iteration);
                print!("time: {:.4?} ms | ", step.duration.num_milliseconds());
                print!(
                    "best: {} | ",
                    1_000 - step.result.best_solution.solution.fitness
                );
                print!(
                    "worst: {} | ",
                    1_000 - step.result.evaluated_population.lowest_fitness()
                );
                print!(
                    "avg: {} | ",
                    1_000 - step.result.evaluated_population.average_fitness()
                );
                println!();

                println!(
                    "best fitness = {:?}",
                    1_000 - step.result.best_solution.solution.fitness
                );

                println!("done");

                // Return
                return utils::create_from(
                    &step.result.best_solution.solution.genome,
                    &db,
                );

                // break 'sim;
            }

            Err(e) => {
                println!("error: {e:?}");
                break 'sim;
            }
        }
    }

    vec![]
}

////////////////////////////////////////////////////////////////////////////////
