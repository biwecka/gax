//! Genetic Algorithm which solves the n-Queens problem.
//!

// Modules /////////////////////////////////////////////////////////////////////
mod encoding;
mod operators;
mod replacement;
mod selection;
mod termination;
mod utils;

use std::sync::mpsc::Sender;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::{Context, Cost, Genotype, Phenotype};
use ndarray::Array2;
use operators::{
    crossover::Crossover, mutation::Mutation, rejection::Rejection,
};
use rayon::prelude::*;
use replacement::Replace;
// use rerun::{external::log::Log, Logger};
use selection::Selection;
use termination::Termination;
// use utils::plotter::Plotter;

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Default)]
struct Stats {
    pub population_size: Vec<usize>,
    pub best: Vec<usize>,
    pub worst: Vec<usize>,
    pub distinct_selections: Vec<usize>,
    pub cache_hits: Vec<usize>,

    pub best_derived: Vec<usize>,

    // >0, when avg. fitness of selection is better than avg. fitness of popul.
    pub selection_differential: Vec<f32>,
    pub selection_differential_derived: Vec<f32>,

    // Chromosome heatmap
    pub chromosome_heatmap: Array2<usize>,

    // Generations since last increase in fitness
    pub gens_since_increase: usize,

    pub ov_distribution: Vec<(usize, usize)>,
}
impl Stats {
    pub fn set_initial_population(
        &mut self,
        ctx: &Context,
        population: &[(Genotype, Cost)],
    ) {
        self.population_size = vec![population.len()];
        self.best = vec![population.first().unwrap().1];
        self.worst = vec![population.last().unwrap().1];
        self.distinct_selections = vec![0];
        self.cache_hits = vec![0];

        self.best_derived = vec![0];
        self.selection_differential = vec![0.];
        self.selection_differential_derived = vec![0.];
        self.chromosome_heatmap =
            Array2::zeros((ctx.board_size, ctx.board_size));

        // for x in 0..=self.worst.last().unwrap().clone() {
        //     self.ov_distribution.insert(x, 0);
        // }

        // for p in population {
        //     self.ov_distribution
        //         .entry(p.1)
        //         .or_default()
        //         .add_assign(1);
        // }
        self.ov_distribution =
            population.par_iter().map(|(_, x)| (*x, 1)).collect();
    }

    pub fn update(
        &mut self,
        population: &[(Genotype, Cost)],
        distinct_selections: usize,
        offspring_avg: f32,
        cache_hits: usize,
    ) {
        self.population_size.push(population.len());
        self.best.push(population.first().unwrap().1);
        self.worst.push(population.last().unwrap().1);
        self.distinct_selections.push(distinct_selections);
        self.cache_hits.push(cache_hits);

        let gen = self.best.len() - 1;
        let curr_best = self.best[gen];
        let prev_best = self.best[gen - 1];

        // self.best_derived.push(curr_best - prev_best);
        self.best_derived.push(prev_best - curr_best);

        if curr_best < prev_best {
            self.gens_since_increase = 0;
        } else {
            self.gens_since_increase += 1;
        }

        let population_avg: f32 =
            population.iter().map(|(_, x)| *x).sum::<usize>() as f32
                / population.len() as f32;
        self.selection_differential.push(offspring_avg - population_avg);

        for (chromosome, _) in population {
            let arr = &chromosome.0;
            for (row, col) in arr.iter().enumerate() {
                self.chromosome_heatmap[[row, *col]] += 1;
            }
        }

        // for x in 0..=self.worst.last().unwrap().clone() {
        //     self.ov_distribution.insert(x, 0);
        // }

        // for p in population {
        //     self.ov_distribution
        //         .entry(p.1)
        //         .or_default()
        //         .add_assign(1);
        // }

        self.ov_distribution =
            population.par_iter().map(|(_, x)| (*x, 1)).collect();
    }
}

// impl std::default::Default for Stats {
//     fn default() -> Self {
//         Self { generation: 0, current_best: usize::MAX }
//     }
// }

struct Parameters {
    // Numbers
    pub population_size: usize,
    pub crossover_rate: f32,
    pub mutation_rate: f32,

    // Operators
    crossover: Crossover,
    mutation: Mutation,
    rejection: Rejection,

    // Process
    selection: Selection,
    replacement: Replace,
    termination: Termination,
}

struct GeneticAlgorithm {
    // Parameters
    params: Parameters,

    // Encoding
    context: Context,
    phenotype: Phenotype,
}

impl GeneticAlgorithm {
    pub fn run(
        mut self,
        stats_ch: Option<Sender<Stats>>,
        logger: Option<utils::rerun::Logger>,
    ) -> Vec<(Genotype, Cost)> {
        // Init population
        let individuals: Vec<Genotype> =
            Genotype::gnerate(self.params.population_size, &self.context);

        // Evaluate individuals to create initial population
        let mut population: Vec<(Genotype, Cost)> = individuals
            // .into_iter()
            .into_par_iter()
            .map(|chromosome| {
                // Derive Phenotype from chromosome/genotype
                let ph = self.phenotype.derive(&chromosome);
                let cost = ph.evaluate();
                drop(ph);
                (chromosome, cost)
            })
            .collect();

        // Sort population
        population
            // .sort_by_key(|(_, x)| *x);
            .par_sort_by_key(|(_, x)| *x);

        println!("start = {}", population.first().unwrap().1);

        // Genetic evolution
        let mut stats = Stats::default();
        stats.set_initial_population(&self.context, &population);

        if let Some(logger) = &logger {
            logger.log(&stats);
        }

        let mut generation = 0;
        while !self.params.termination.check(generation, &stats) {
            // Increment generation counter
            generation += 1;

            // Calculate selection and elite size
            let mut selection_size = self
                .params
                .replacement
                .selection_size(self.params.population_size);

            if selection_size % 2 != 0 {
                selection_size += 1;
            }

            // Selection
            let (parents, distinct_selections) =
                self.params.selection.exec(selection_size, &population);

            // Crossover + Mutation
            let mut offspring: Vec<(Genotype, Cost)> = parents
                // .chunks(2)
                .par_chunks(2)
                .map(|asdf| {
                    let a = asdf[0];
                    let b = asdf[1];

                    // Crossover
                    let (mut x, mut y) = self.params.crossover.exec(
                        &a.0,
                        &b.0,
                        self.params.crossover_rate,
                    );

                    // Mutation
                    self.params.mutation.exec(
                        &mut x,
                        self.params.mutation_rate,
                        &self.context,
                    );
                    self.params.mutation.exec(
                        &mut y,
                        self.params.mutation_rate,
                        &self.context,
                    );

                    // Evaluation
                    let x_ph = self.phenotype.derive(&x);
                    let x_ov = x_ph.evaluate();
                    drop(x_ph);
                    let child0 = (x, x_ov);

                    let y_ph = self.phenotype.derive(&y);
                    let y_ov = y_ph.evaluate();
                    drop(y_ph);
                    let child1 = (y, y_ov);

                    // Offspring Rejection
                    let (o0, o1) = self.params.rejection.exec(
                        a.clone(),
                        b.clone(),
                        child0,
                        child1,
                    );

                    // Return
                    vec![o0, o1]
                })
                .flatten()
                .collect();

            // let len_before = offspring.len();
            offspring.truncate(
                self.params
                    .replacement
                    .selection_size(self.params.population_size),
            );

            let offspring_avg = offspring
                // .iter()
                .par_iter()
                .map(|(_, x)| *x)
                .sum::<usize>() as f32
                / offspring.len() as f32;

            // Replace (population sorted; offspring not)
            self.params.replacement.exec(&mut population, offspring);

            // Sort
            population
                // .sort_by_key(|(_, x)| *x);
                .par_sort_by_key(|(_, x)| *x);

            println!(
                "[{}] current best = {:?}",
                generation,
                population.first().unwrap()
            );

            // println!("-------------------------------------------------------");

            // Update stats
            stats.update(&population, distinct_selections, offspring_avg, 0);

            if let Some(sender) = &stats_ch {
                let _ = sender.send(stats.clone());
            }

            if let Some(logger) = &logger {
                logger.log(&stats);
            }

            // Adaption
            // if generation > 0 && generation % 8_000 == 0 {
            //     self.params.selection = Selection::RouletteWheel;
            // }
            // if generation > 0 && generation % 6_000 == 0 && generation % 8_000 != 0 {
            //     self.params.selection = Selection::Random;
            // }

            if generation == 10_000 {
                // self.params.mutation_rate = 0.4;
                // self.params.rejection = Rejection::BetterThanWorseParent;
                self.params.population_size = 1_000;
                self.params.selection = Selection::Random;
            } else if generation == 12_000 {
                self.params.population_size = 1_400;
                self.params.selection = Selection::Tournament(4);
            } else if generation == 14_000 {
                self.params.population_size = 1_800;
                self.params.selection = Selection::Random;
            } else if generation == 16_000 {
                self.params.population_size = 1_900;
                self.params.selection = Selection::Tournament(4);
            } else if generation == 18_000 {
                self.params.population_size = 2_000;
                self.params.selection = Selection::Random;
            } else if generation == 20_000 {
                self.params.selection = Selection::Tournament(8);
            }
        }

        population
    }
}

// Run /////////////////////////////////////////////////////////////////////////
pub fn run() {
    // let (plotter, stats_ch) = Plotter::init();
    let logger = utils::rerun::Logger::connect();
    std::thread::sleep(std::time::Duration::from_secs(1));

    let ga = GeneticAlgorithm {
        params: Parameters {
            population_size: 500,
            crossover_rate: 1.,
            mutation_rate: 0.2,

            crossover: Crossover::VariableMultiPoint(8),
            mutation: Mutation::RandomizeBits(8),
            rejection: Rejection::None,

            selection: Selection::RouletteWheel,
            replacement: Replace::Elite(0.01),
            termination: Termination::ObjectiveValue(0),
            // termination: Termination::Generations(100_000),
        },

        context: Context::init(128),
        phenotype: Phenotype::init(128),
    };

    // let join_handle = std::thread::spawn(move || {
    let solutions = ga.run(None /*Some(stats_ch)*/, Some(logger));
    // solutions
    // });

    // plotter.start();

    // let solutions = join_handle.join().unwrap();

    let best_solution =
        if let Some(best) = solutions.first() { best } else { unreachable!() };

    println!("best cost = {}", best_solution.1);
    println!("genotype  = {:?}", best_solution.0);
}

////////////////////////////////////////////////////////////////////////////////
