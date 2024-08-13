//! Genetic Algorithm which solves the n-Queens problem.
//!

// Modules /////////////////////////////////////////////////////////////////////
mod encoding;
mod operators;
mod termination;
mod selection;
mod replacement;


use std::usize;

// Imports /////////////////////////////////////////////////////////////////////
use encoding::{Context, Cost, Genotype, Phenotype};
use operators::{crossover::Crossover, mutation::Mutation, rejection::Rejection};
use replacement::Replace;
use termination::Termination;
use selection::Selection;

// Struct //////////////////////////////////////////////////////////////////////
struct Stats {
    pub generation: usize,
    pub current_best: Cost,
}

impl std::default::Default for Stats {
    fn default() -> Self {
        Self {
            generation: 0,
            current_best: usize::MAX,
        }
    }
}


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
    pub fn run(self) -> Vec<(Genotype, Cost)> {
        // Init population
        let individuals: Vec<Genotype> = Genotype::gnerate(
            self.params.population_size,
            &self.context
        );

        // Evaluate individuals to create initial population
        let mut population: Vec<(Genotype, Cost)> = individuals
            .into_iter()
            .map(|chromosome| {
                // Derive Phenotype from chromosome/genotype
                let ph = self.phenotype.derive(&chromosome);
                let cost = ph.evaluate();
                drop(ph);
                (chromosome, cost)
            })
            .collect();


        // Sort population
        population.sort_by_key(|(_, x)| *x);

        // println!("population:");
        // for p in &population {
        //     println!("{p:?}");
        // }

        println!("start = {}", population.first().unwrap().1);

        // Genetic evolution
        let mut stats = Stats::default();

        while !self.params.termination.check(&stats) {
            stats.generation += 1;

            // Calculate selection and elite size
            let mut selection_size = self.params.replacement
                .selection_size(self.params.population_size);

            if selection_size % 2 != 0 {
                selection_size += 1;
            }

            // Selection
            let (parents, _distinct_selections) = self.params.selection
                .exec(selection_size, &population);

            // println!("distinct selection = {}", distinct_selections);

            // println!("parents:");
            // for p in &parents {
            //     println!("{p:?}");
            // }

            // Crossover + Mutation
            let mut offspring: Vec<(Genotype, Cost)> = parents
                .chunks(2)
                .map(|asdf| {
                    // println!("{:?}", parents);
                    // if asdf.len() < 2 {
                    //     dbg!(population.len());
                    //     dbg!(self.params.replacement.selection_size(self.params.population_size));
                    //     dbg!(selection_size);
                    //     dbg!(parents.len());
                    // }

                    let a = asdf[0];
                    let b = asdf[1];

                    // Crossover
                    let (mut x, mut y) = self.params.crossover
                        .exec(&a.0, &b.0, self.params.crossover_rate);

                    // Mutation
                    self.params.mutation
                        .exec(&mut x, self.params.mutation_rate, &self.context);
                    self.params.mutation
                        .exec(&mut y, self.params.mutation_rate, &self.context);

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
                    let (o0, o1) = self.params.rejection
                        .exec(a.clone(), b.clone(), child0, child1);

                        // println!("a = {a:?}");
                        // println!("b = {b:?}");
                        // println!("o0= {o0:?}");
                        // println!("o1= {o1:?}");

                    // Return
                    vec![o0, o1]
                })
                .flatten()
                .collect();

            // println!("offspring:");
            // for o in &offspring {
            //     println!("{o:?}");
            // }

            // let len_before = offspring.len();
            offspring.truncate(self.params.replacement.selection_size(self.params.population_size));

            // let len_after = offspring.len();
            // println!("offspring truncated {} element(s)", len_before-len_after);

            // Replace (population sorted; offspring not)
            self.params.replacement.exec(&mut population, offspring);

            // println!("elite size = {}", self.params.replacement.elite_size(self.params.population_size));

            // println!("replaced pop");
            // for p in &population {
            //     println!("{p:?}");
            // }

            // Sort
            population.sort_by_key(|(_, x)| *x);

            // println!("[{}] current best = {:?}", stats.generation, population.first().unwrap());

            // println!("--------------------------------");

            // Update stats (TODO)
        }

        population
    }
}

// Run /////////////////////////////////////////////////////////////////////////
pub fn run() {
    let ga = GeneticAlgorithm {
        params: Parameters {
            population_size: 1000,
            crossover_rate: 0.5,
            mutation_rate: 0.9,

            crossover: Crossover::VariableSinglePoint,
            mutation: Mutation::RandomizeBits(1),
            rejection: Rejection::None,

            selection: Selection::RouletteWheel,
            replacement: Replace::Full,
            // termination: Termination::ObjectiveValue(2),
            termination: Termination::Generations(100_000),
        },

        context: Context::init(8),
        phenotype: Phenotype::init(8),
    };

    let solutions = ga.run();

    let best_solution = if let Some(best) = solutions.first() {
        best
    } else {
        unreachable!()
    };

    println!("best cost = {}", best_solution.1);
    println!("genotype  = {:?}", best_solution.0);
}

////////////////////////////////////////////////////////////////////////////////
