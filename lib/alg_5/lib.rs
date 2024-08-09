//! Algorithm V5
//! This algorithm uses the "oxigen" genetic-algorithm-framework
//!
//! Limitations:
//!

// Modules /////////////////////////////////////////////////////////////////////
// mod utils;

use oxigen::{
    AgeFunctions, CrossoverFunctions, MutationRates,
    PopulationRefitnessFunctions, SelectionFunctions, SelectionRates,
    SlopeParams, StopCriteria, SurvivalPressureFunctions,
};
// Imports /////////////////////////////////////////////////////////////////////
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
const GENERATIONS: usize = 10_000;

// Algorithm ///////////////////////////////////////////////////////////////////
pub fn run(instance: Instance) -> Vec<SolutionEvent> {
    // Create an XHSTT database of the problem instance
    let db = xhstt::db::Database::init(&instance).unwrap();

    //
    let (_solutions, generation, _progress, population) =
        oxigen::GeneticExecution::<u8, Chromosome>::new()
            .population_size(POPULATION_SIZE)
            .genotype_size(ProblemSize::new(&db))
            .mutation_rate(Box::new(MutationRates::Constant(0.01)))
            .selection_rate(Box::new(SelectionRates::Constant(1)))
            .select_function(Box::new(SelectionFunctions::Roulette))
            .crossover_function(Box::new(CrossoverFunctions::MultiCrossPoint))
            .population_refitness_function(Box::new(
                PopulationRefitnessFunctions::None,
            ))
            .survival_pressure_function(Box::new(
                SurvivalPressureFunctions::Worst,
            ))
            .age_function(Box::new(AgeFunctions::None))
            .stop_criterion(Box::new(StopCriteria::Generation(
                GENERATIONS as u64,
            )))
            .population_log(
                1_000,
                std::fs::File::create("population.txt").unwrap(),
            )
            .run();

    // println!("generation = {generation}");
    // println!("population = {:?}", population.iter().map(|x| x.fitness.unwrap()).collect::<Vec<_>>());

    vec![]
}

////////////////////////////////////////////////////////////////////////////////
// use oxigen::prelude::*;
// use oxigen:
// use oxigen::prelude::SmallRng;
use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Clone)]
struct Chromosome {
    times: Vec<u8>,

    num_times: u8,

    resources: ndarray::Array2<bool>,
    constriants: Vec<(Constraint, Vec<usize>)>,
}

impl Chromosome {
    /// Get the events (indices) that are assigned to the given resource.
    pub fn events_by_resource(&self, resource_idx: usize) -> Vec<usize> {
        self.resources
            .row(resource_idx)
            .iter()
            .enumerate()
            .filter_map(|(i, value)| match value {
                true => Some(i),
                false => None,
            })
            .collect()
    }
}

impl std::fmt::Display for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chromosome = {:?}", self.times)
    }
}

impl std::fmt::Debug for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chromosome = {:?}", self.times)
    }
}

impl std::cmp::PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.times.eq(&other.times)
    }
}

impl std::cmp::Eq for Chromosome {}

impl std::hash::Hash for Chromosome {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.times.hash(state)
    }
}

#[derive(Default)]
struct ProblemSize {
    pub events: u16,
    pub times: u8,

    resources: ndarray::Array2<bool>,
    constraints: Vec<(Constraint, Vec<usize>)>,
}

impl ProblemSize {
    pub fn new(db: &xhstt::db::Database) -> Self {
        // Initialize resource matrix
        let mut resources = ndarray::Array2::<bool>::default((
            db.resources().len(),
            db.events().len(),
        ));

        // Fill resource matrix
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                resources[[resource_idx, event_idx]] = true;
                // resources.set_cell(resource_idx, event_idx, true);
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
        Self {
            events: db.events().len() as u16,
            times: db.times().len() as u8,
            resources,
            constraints,
        }
    }
}

impl oxigen::Genotype<u8> for Chromosome {
    type ProblemSize = ProblemSize;

    fn iter(&self) -> std::slice::Iter<u8> {
        self.times.iter()
    }

    fn into_iter(self) -> std::vec::IntoIter<u8> {
        self.times.into_iter()
    }

    fn from_iter<I: Iterator<Item = u8>>(&mut self, genes: I) {
        self.times = genes.collect()
    }

    fn generate(problem: &Self::ProblemSize) -> Self {
        let mut individual = Chromosome {
            times: Vec::with_capacity(problem.events as usize),

            num_times: problem.times,
            resources: problem.resources.clone(),
            constriants: problem.constraints.clone(),
        };

        let mut rgen = SmallRng::from_entropy();
        for _ in 0..problem.events {
            individual.times.push(
                rgen.sample(rand::distributions::Uniform::from(
                    0..problem.times,
                )),
            );
        }

        individual
    }

    fn fitness(&self) -> f64 {
        let mut total_cost: f64 = 0.;

        for (constraint, indices) in &self.constriants {
            match constraint {
                Constraint::AssignTimeConstraint(_) => {}
                Constraint::AvoidClashesConstraint(params) => {
                    total_cost += f64::from(avoid_clashes_constraint(
                        self, params, &indices,
                    ));
                }
            }
        }

        total_cost
    }

    fn mutate(&mut self, rgen: &mut SmallRng, index: usize) {
        self.times[index] =
            rgen.sample(rand::distributions::Uniform::from(0..self.num_times))
    }

    fn is_solution(&self, fitness: f64) -> bool {
        fitness < 0.1
    }
}

fn avoid_clashes_constraint(
    chromosome: &Chromosome,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize],
) -> i32 {
    // Deviation
    let deviation: usize = resource_idxs
        // .par_iter()
        .iter()
        .map(|resource_idx| {
            // Get events by resource
            let event_idxs = chromosome.events_by_resource(*resource_idx);

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
    ((params.weight as usize) * params.cost_function.calc(deviation)) as i32
}

fn times_by_events(chromosome: &Chromosome, event_idxs: &[usize]) -> Vec<u8> {
    let mut times = std::collections::HashSet::<u8>::new();

    for event_idx in event_idxs {
        times.insert(chromosome.times[*event_idx]);
    }

    times.into_iter().collect()
}

////////////////////////////////////////////////////////////////////////////////
