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
        let mut rtd = RuntimeData::init(&population);

        // Initialize rerun logger
        let rerun_logger = if cfg!(feature = "rerun-log") {
            let logger = tools::rerun::RerunLogger::connect("ga");
            logger.log(&rtd);
            Some(logger)
        } else {
            None
        };

        // Start loop
        while !self.params.termination.stop(rtd.generation, &rtd.current_best.1)
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
                    assert_eq!(parents.len(), 2);
                    let a = parents[0];
                    let b = parents[1];

                    // Crossover
                    let (mut x0, mut x1) = self.params.crossover.exec(
                        &a.0,
                        &b.0,
                        &self.encoding.context,
                    );

                    // Mutation
                    self.params.mutation.exec(&mut x0, &self.encoding.context);
                    self.params.mutation.exec(&mut x1, &self.encoding.context);

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
                    rtd.current_best.1.to_usize(),
                    rtd.current_mean,
                    rtd.current_worst.1.to_usize(),
                );
            } else {
                println!(
                    "[{}] best = {:?}, mean = {}, worst = {:?}",
                    rtd.generation,
                    rtd.current_best.1,
                    rtd.current_mean,
                    rtd.current_worst.1,
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

    #[cfg(feature = "cache")]
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
        let mut rtd = RuntimeData::init(&population);

        // Create and populate cache
        let mut cache = HashMap::<Ge, Ov>::from_iter(population.clone());

        // Initialize rerun logger
        let rerun_logger = if cfg!(feature = "rerun-log") {
            let logger = tools::rerun::RerunLogger::connect("ga");
            logger.log(&rtd);
            Some(logger)
        } else {
            None
        };

        // Start loop
        while !self.params.termination.stop(rtd.generation, &rtd.current_best.1)
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
            let cx_mu_re: Vec<(((Ge, Ov), (Ge, Ov)), usize)> = parents
                .par_chunks(2)
                .map(|parents| {
                    assert_eq!(parents.len(), 2);
                    let a = parents[0];
                    let b = parents[1];

                    // Crossover
                    let (mut x0, mut x1) = self.params.crossover.exec(
                        &a.0,
                        &b.0,
                        &self.encoding.context,
                    );

                    // Mutation
                    self.params.mutation.exec(&mut x0, &self.encoding.context);
                    self.params.mutation.exec(&mut x1, &self.encoding.context);

                    // Evaluation
                    let mut cache_hits = 0;

                    let y0: (Ge, Ov) = {
                        let ov: Ov = if let Some(cached_ov) = cache.get(&x0) {
                            cache_hits += 1;
                            cached_ov.clone()
                        } else {
                            let ph = self
                                .encoding
                                .phenotype
                                .derive(&x0, &self.encoding.context);
                            ph.evaluate(&self.encoding.context)
                        };

                        (x0, ov)
                    };

                    let y1: (Ge, Ov) = {
                        let ov: Ov = if let Some(cached_ov) = cache.get(&x1) {
                            cache_hits += 1;
                            cached_ov.clone()
                        } else {
                            let ph = self
                                .encoding
                                .phenotype
                                .derive(&x1, &self.encoding.context);
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
            let cache_hits: usize =
                cx_mu_re.iter().map(|(_, hits)| *hits).sum();
            let mut offspring: Vec<(Ge, Ov)> = cx_mu_re
                .into_iter()
                .map(|((a, b), _)| vec![a, b])
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

            // Update cache
            let now = std::time::Instant::now();
            population.iter().for_each(|(ge, ov)| {
                cache.insert(ge.clone(), ov.clone());
            });
            let duration_cache_update = now.elapsed();

            // Update runtime data
            let now = std::time::Instant::now();
            rtd.update(
                &population,
                self.params.replacement.elite_size(self.params.population_size),
                selection_size_corrected,
                distinct_selections,
                offspring_mean,
                cache_hits,
            );
            let duration_rtd_update = now.elapsed();

            rtd.update_execution_times(vec![
                duration_select,
                duration_cx_mu_re,
                duration_truncate,
                duration_calc_mean,
                duration_replace,
                duration_sort,
                duration_cache_update,
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
                    rtd.current_best.1.to_usize(),
                    rtd.current_mean,
                    rtd.current_worst.1.to_usize(),
                );
            } else {
                println!(
                    "[{}] best = {:?}, mean = {}, worst = {:?}, cache-hits = {}",
                    rtd.generation,
                    rtd.current_best.1,
                    rtd.current_mean,
                    rtd.current_worst.1,
                    rtd.cache_hits,
                );
            }

            // Execute dynamics
            for dyn_exe in &self.dynamics {
                dyn_exe.exec(&mut self.params, &rtd);
            }
        }

        // Print result to console
        println!(
            "[{}] best = {:?}, mean = {}, worst = {:?}, cache-hits = {}",
            rtd.generation,
            rtd.current_best.1,
            rtd.current_mean,
            rtd.current_worst.1,
            rtd.cache_hits,
        );

        // Return
        population
    }
}

////////////////////////////////////////////////////////////////////////////////
