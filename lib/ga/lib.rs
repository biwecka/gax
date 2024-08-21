#![feature(let_chains)]

// Modules /////////////////////////////////////////////////////////////////////
pub mod encoding;
pub mod operators;
pub mod process;
pub mod utils;
#[rustfmt::skip] pub mod parameters;
#[rustfmt::skip] mod builder;
pub mod dynamics;
mod runtime_data;
mod tools;

// Re-Exports //////////////////////////////////////////////////////////////////
pub use builder::*;

// Imports /////////////////////////////////////////////////////////////////////
use dynamics::Dynamic;
use encoding::{Context, Encoding, Genotype, ObjectiveValue, Phenotype};
use operators::{Crossover, Mutation};
use parameters::Parameters;
use process::{
    rejection::Rejection, replacement::Replacement, selection::Selection,
    termination::Termination,
};
use rayon::prelude::*;
use runtime_data::RuntimeData;

#[cfg(feature = "cache")]
use hashbrown::HashMap;

#[cfg(feature = "rerun_logger")]
use tools::rerun_logger::RerunLogger;

// Macros //////////////////////////////////////////////////////////////////////
macro_rules! measure_runtime_start {
    ($alg:ident) => {
        #[cfg(feature = "log_runtimes")]
        {
            $alg.runtime_reference = std::time::Instant::now();
        };
    };
}

macro_rules! measure_runtime_end {
    ($alg:ident) => {
        #[cfg(feature = "log_runtimes")]
        {
            let elapsed = $alg.runtime_reference.elapsed();
            $alg.runtimes.push(elapsed);
        };
    };
}

// Algorithm ///////////////////////////////////////////////////////////////////

pub struct Algorithm<
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
    Ph: Phenotype<Ov, Ctx, Ge>,
    Cr: Crossover<Ctx, Ge>,
    Mu: Mutation<Ctx, Ge>,
    T: From<Ov>,
    Se: Selection<Ov, Ctx, Ge, T>,
    Re: Rejection<Ov, Ctx, Ge>,
    Rp: Replacement<(Ge, Ov)>,
    Te: Termination<Ov>,
    Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
> {
    encoding: Encoding<Ov, Ctx, Ge, Ph>,
    params: Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    dynamics: Vec<Dy>,

    #[cfg(feature = "cache")]
    cache: HashMap<Ge, Ov>,

    #[cfg(feature = "rerun_logger")]
    rerun_logger: RerunLogger,

    #[cfg(feature = "log_runtimes")]
    runtime_reference: std::time::Instant,

    #[cfg(feature = "log_runtimes")]
    runtimes: Vec<std::time::Duration>,
}

impl<
        Ov: ObjectiveValue,
        Ctx: Context,
        Ge: Genotype<Ctx>,
        Ph: Phenotype<Ov, Ctx, Ge>,
        Cr: Crossover<Ctx, Ge>,
        Mu: Mutation<Ctx, Ge>,
        T: From<Ov>,
        Se: Selection<Ov, Ctx, Ge, T>,
        Re: Rejection<Ov, Ctx, Ge>,
        Rp: Replacement<(Ge, Ov)>,
        Te: Termination<Ov>,
        Dy: Dynamic<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    > Algorithm<Ov, Ctx, Ge, Ph, Cr, Mu, T, Se, Re, Rp, Te, Dy>
{
    /*
    #[cfg(not(feature = "cache"))]
    pub fn run(mut self) -> Vec<(Ge, Ov)> {
        // Create initial population
        let mut population: Vec<(Ge, Ov)> = {
            // Generate individuals
            let individuals: Vec<Ge> = Ge::generate(
                self.params.population_size,
                &self.encoding.context,
            );

            // Evaluate the individuals
            let mut population: Vec<(Ge, Ov)> = individuals
                .into_iter()
                .map(|chromosome| {
                    // Create derived phenotype from blueprint.
                    let derivative = self
                        .encoding
                        .phenotype
                        .derive(&chromosome, &self.encoding.context);

                    // Evaluate the derivative
                    let evaluation =
                        derivative.evaluate(&self.encoding.context);
                    drop(derivative);

                    // Return
                    (chromosome, evaluation)
                })
                .collect();

            // Sort the vector and return it as population
            population.par_sort_by_key(|(_, x)| x.clone());

            // Return
            population
        };

        // Initialize runtime data
        let mut rtd = RuntimeData::init(&population, &self.params);

        // Initialize rerun logger
        let rerun_logger = if cfg!(feature = "rerun-log") {
            let logger = tools::rerun::RerunLogger::connect("ga");
            logger.log(&rtd);
            Some(logger)
        } else {
            None
        };

        // Start loop
        while !self.params.termination.stop(rtd.generation, &rtd.current_best)
        {
            // Increment generation counter
            rtd.inc_generation();

            // Select
            let now = std::time::Instant::now();
            let (selection_size_raw, selection_size_corrected) = self
                .params
                .replacement
                .selection_size(self.params.population_size);

            let (parents, distinct_selections) = self
                .params
                .selection
                .exec(selection_size_corrected, &population);

            let duration_select = now.elapsed();

            // Crossover, Mutation, Rejection
            let now = std::time::Instant::now();
            let mut offspring: Vec<(Ge, Ov)> = parents
                .par_chunks(2)
                .map(|parents| {
                    // Get source of randomness
                    let mut rng = rand::thread_rng();

                    assert_eq!(parents.len(), 2);
                    let a = parents[0];
                    let b = parents[1];

                    // Crossover
                    let (mut x0, mut x1) = self.params.crossover.exec(
                        &a.0,
                        &b.0,
                        &self.encoding.context,
                        &mut rng,
                    );

                    // Mutation
                    self.params.mutation.exec(&mut x0, &self.encoding.context, &mut rng);
                    self.params.mutation.exec(&mut x1, &self.encoding.context, &mut rng);

                    // Evaluation
                    let y0: (Ge, Ov) = {
                        let ph = self
                            .encoding
                            .phenotype
                            .derive(&x0, &self.encoding.context);
                        let ov = ph.evaluate(&self.encoding.context);
                        drop(ph);

                        (x0, ov)
                    };

                    let y1: (Ge, Ov) = {
                        let ph = self
                            .encoding
                            .phenotype
                            .derive(&x1, &self.encoding.context);
                        let ov = ph.evaluate(&self.encoding.context);
                        drop(ph);

                        (x1, ov)
                    };

                    // Rejection
                    let (z0, z1) = self.params.rejection.exec(
                        a,
                        b,
                        &y0,
                        &y1,
                        &self.encoding.context,
                    );

                    // Return
                    vec![z0.clone(), z1.clone()]
                })
                .flatten()
                .collect();

            let duration_cx_mu_re = now.elapsed();

            // Correct offspring length (might be off by one, because of
            // selection size correction to get PAIRS of parents).
            let now = std::time::Instant::now();
            offspring.truncate(selection_size_raw);

            let duration_truncate = now.elapsed();

            // Calculate the average mean objective value of the offspring
            let now = std::time::Instant::now();
            let offspring_mean: f32 = Ov::calc_average(
                &offspring.iter().map(|(_, ov)| ov.clone()).collect::<Vec<_>>(),
            );
            let duration_calc_mean = now.elapsed();

            // Replace (population must be sorted; offspring is not).
            let now = std::time::Instant::now();
            self.params.replacement.exec(&mut population, offspring);
            let duration_replace = now.elapsed();

            // Sort the new population
            let now = std::time::Instant::now();
            population.par_sort_by_key(|(_, x)| x.clone());
            let duration_sort = now.elapsed();

            // Update runtime data
            let now = std::time::Instant::now();
            rtd.update(
                &population,
                self.params.replacement.elite_size(self.params.population_size),
                selection_size_corrected,
                distinct_selections,
                offspring_mean,
                0,
            );
            let duration_rtd_update = now.elapsed();

            rtd.update_execution_times(vec![
                duration_select,
                duration_cx_mu_re,
                duration_truncate,
                duration_calc_mean,
                duration_replace,
                duration_sort,
                std::time::Duration::from_millis(0), // Cache update
                duration_rtd_update,
            ]);

            // Log (to 'rerun' or 'console')
            if cfg!(feature = "rerun-log") {
                if let Some(logger) = &rerun_logger {
                    logger.log(&rtd);
                } else {
                    unreachable!();
                }

                // Minimal console log
                println!(
                    "[{:>7}] best:{:>4} mean: {:>4.2} worst:{:>4}",
                    rtd.generation,
                    rtd.current_best.to_usize(),
                    rtd.current_mean,
                    rtd.current_worst.to_usize(),
                );
            } else {
                println!(
                    "[{}] best = {:?}, mean = {}, worst = {:?}",
                    rtd.generation,
                    rtd.current_best,
                    rtd.current_mean,
                    rtd.current_worst,
                );
            }

            // Execute dynamics
            for dyn_exe in &self.dynamics {
                dyn_exe.exec(&mut self.params, &rtd);
            }
        }

        // Return
        population
    }
    */

    pub fn run(mut self) -> Vec<(Ge, Ov)> {
        // Create initial population
        let mut population: Vec<(Ge, Ov)> = {
            // Generate individuals
            let individuals: Vec<Ge> = Ge::generate(
                self.params.population_size,
                &self.encoding.context,
            );

            // Evaluate the individuals
            let mut population: Vec<(Ge, Ov)> = individuals
                .into_iter()
                .map(|chromosome| {
                    // Create derived phenotype from blueprint.
                    let derivative = self
                        .encoding
                        .phenotype
                        .derive(&chromosome, &self.encoding.context);

                    // Evaluate the derivative
                    let evaluation =
                        derivative.evaluate(&self.encoding.context);
                    drop(derivative);

                    // Return
                    (chromosome, evaluation)
                })
                .collect();

            // Sort the vector and return it as population
            population.par_sort_by_key(|(_, x)| x.clone());

            // Return
            population
        };

        // Initialize runtime data
        let mut rtd = RuntimeData::init(&population, &self.params);

        // Create and populate cache
        #[cfg(feature = "cache")]
        {
            self.cache = HashMap::<Ge, Ov>::from_iter(population.clone());
        };

        // Initialize rerun logger
        #[cfg(feature = "rerun_logger")]
        {
            self.rerun_logger.log(&rtd);
        };

        // Start loop
        while !self.params.termination.stop(rtd.generation, &rtd.current_best) {
            // Increment generation counter
            rtd.inc_generation();

            // Select
            measure_runtime_start!(self);
            let (selection_size_raw, selection_size_corrected) = self
                .params
                .replacement
                .selection_size(self.params.population_size);

            let (parents, distinct_selections) = self
                .params
                .selection
                .exec(selection_size_corrected, &population);

            measure_runtime_end!(self);

            // Crossover, Mutation, Rejection
            measure_runtime_start!(self);
            let cx_mu_re: Vec<(((Ge, Ov), (Ge, Ov)), usize)> = parents
                .par_chunks(2)
                .map(|parents| {
                    // Get source of randomness
                    let mut rng = rand::thread_rng();

                    assert_eq!(parents.len(), 2);
                    let a = parents[0];
                    let b = parents[1];

                    // Crossover
                    let (mut x0, mut x1) = self.params.crossover.exec(
                        &a.0,
                        &b.0,
                        self.params.crossover_rate,
                        &mut rng,
                        &self.encoding.context,
                    );

                    // Mutation
                    self.params.mutation.exec(
                        &mut x0,
                        self.params.mutation_rate,
                        &mut rng,
                        &self.encoding.context,
                    );
                    self.params.mutation.exec(
                        &mut x1,
                        self.params.mutation_rate,
                        &mut rng,
                        &self.encoding.context,
                    );

                    // Evaluation
                    let mut cache_hits = 0;

                    let y0: (Ge, Ov) = {
                        // If the cache feature is enabled, check the cache
                        let ov = if cfg!(feature = "cache")
                            && let Some(cached_ov) = self.cache.get(&x0)
                        {
                            // Increase cache hit count
                            cache_hits += 1;

                            // Return the cached objective value
                            cached_ov.clone()
                        }
                        // If the cache feature is not enabled, calculate the
                        // objective value as usual.
                        else {
                            // Create derived phenotype
                            let ph = self
                                .encoding
                                .phenotype
                                .derive(&x0, &self.encoding.context);

                            // Calculate objective value and return it
                            ph.evaluate(&self.encoding.context)
                        };

                        (x0, ov)
                    };

                    let y1: (Ge, Ov) = {
                        // If the cache feature is enabled, check the cache
                        let ov: Ov = if cfg!(feature = "cache")
                            && let Some(cached_ov) = self.cache.get(&x1)
                        {
                            // Increase cache hit count
                            cache_hits += 1;

                            // Return the cached objective value
                            cached_ov.clone()
                        }
                        // If the cache feature is not enabled, calculate the
                        // objective value as usual.
                        else {
                            // Create derived phenotype
                            let ph = self
                                .encoding
                                .phenotype
                                .derive(&x1, &self.encoding.context);

                            // Calculate objective value and return it
                            ph.evaluate(&self.encoding.context)
                        };

                        (x1, ov)
                    };

                    // Rejection
                    let (z0, z1) = self.params.rejection.exec(
                        a,
                        b,
                        &y0,
                        &y1,
                        &self.encoding.context,
                    );

                    // Return
                    ((z0.clone(), z1.clone()), cache_hits)
                })
                .collect::<Vec<(((Ge, Ov), (Ge, Ov)), usize)>>();

            // Extract offspring and the nuber of cache hits from the results
            // of crossover, mutation and rejection
            let (offspr, ch_num): (Vec<((Ge, Ov), (Ge, Ov))>, Vec<usize>) =
                cx_mu_re.into_iter().unzip();

            let cache_hits: usize = ch_num.into_iter().sum();

            let mut offspring: Vec<(Ge, Ov)> =
                offspr.into_iter().flat_map(|(a, b)| vec![a, b]).collect();

            measure_runtime_end!(self);

            // Correct offspring length (might be off by one, because of
            // selection size correction to get PAIRS of parents).
            measure_runtime_start!(self);
            offspring.truncate(selection_size_raw);

            measure_runtime_end!(self);

            // Calculate the average mean objective value of the offspring
            measure_runtime_start!(self);
            let offspring_mean: f32 = Ov::calc_average(
                &offspring.iter().map(|(_, ov)| ov.clone()).collect::<Vec<_>>(),
            );

            measure_runtime_end!(self);

            // Replace (population must be sorted; offspring is not).
            measure_runtime_start!(self);
            self.params.replacement.exec(&mut population, offspring);

            measure_runtime_end!(self);

            // Sort the new population
            measure_runtime_start!(self);
            population.par_sort_by_key(|(_, x)| x.clone());

            measure_runtime_end!(self);

            // Update cache
            #[cfg(feature = "cache")]
            {
                measure_runtime_start!(self);
                population.iter().for_each(|(ge, ov)| {
                    self.cache.insert(ge.clone(), ov.clone());
                });

                measure_runtime_end!(self);
            };

            // Update runtime data
            measure_runtime_start!(self);
            rtd.update(
                &population,
                self.params.replacement.elite_size(self.params.population_size),
                selection_size_corrected,
                distinct_selections,
                offspring_mean,
                cache_hits,
            );

            measure_runtime_end!(self);

            #[cfg(feature = "log_runtimes")]
            {
                rtd.update_execution_times(self.runtimes);
                self.runtimes = vec![];
            }

            // Log (to 'rerun' or 'console')
            #[cfg(feature = "rerun_logger")]
            {
                // Send runtime data to logger
                self.rerun_logger.log(&rtd);

                // Also print out minimal information to the console
                println!(
                    "[{:>7}] best:{:>4} mean: {:>4.2} worst:{:>4}",
                    rtd.generation,
                    rtd.current_best.to_usize(),
                    rtd.current_mean,
                    rtd.current_worst.to_usize(),
                );
            };

            #[cfg(not(feature = "rerun_logger"))]
            {
                // Print some more information, if the "rerun_logger" feature
                // is NOT enabled.
                println!(
                    "[{}] best = {:?}, mean = {}, worst = {:?}, cache-hits = {}",
                    rtd.generation,
                    rtd.current_best,
                    rtd.current_mean,
                    rtd.current_worst,
                    rtd.cache_hits,
                );
            };

            // Execute dynamics
            for dyn_exe in &self.dynamics {
                dyn_exe.exec(&mut self.params, &rtd);
            }
        }

        // Print result to console
        println!(
            "[{}] best = {:?}, mean = {}, worst = {:?}, cache-hits = {}",
            rtd.generation,
            rtd.current_best,
            rtd.current_mean,
            rtd.current_worst,
            rtd.cache_hits,
        );

        // Return
        population
    }
}

////////////////////////////////////////////////////////////////////////////////
