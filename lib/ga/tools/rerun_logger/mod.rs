// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    operators::{Crossover, Mutation},
    process::{
        rejection::Rejection, replacement::Replacement, selection::Selection,
        termination::Termination,
    },
    runtime_data::RuntimeData,
};
use simple_moving_average::SMA;

use rerun::{RecordingStream, RecordingStreamBuilder, Scalar};

#[cfg(any(
    feature = "log_ov_dist",
    feature = "log_diversity",
    feature = "log_runtimes"
))]
use rerun::BarChart;

// Constants ///////////////////////////////////////////////////////////////////
const GENERATION_TIME_SEQ: &str = "generation";

// Rerun Logger ////////////////////////////////////////////////////////////////
pub struct RerunLogger {
    rec: RecordingStream,
}

impl RerunLogger {
    pub fn connect(name: &str) -> Self {
        let rec = RecordingStreamBuilder::new(name).spawn().unwrap();
        Self { rec }
    }

    pub fn log<
        Ov: ObjectiveValue + Into<T>,
        Ctx: Context,
        Ge: Genotype<Ctx>,
        Cr: Crossover<Ctx, Ge>,
        Mu: Mutation<Ctx, Ge>,
        T,
        Se: Selection<Ov, Ctx, Ge, T>,
        Re: Rejection<Ov, Ctx, Ge>,
        Rp: Replacement<(Ge, Ov)>,
        Te: Termination<Ov>,
    >(
        &self,
        rtd: &RuntimeData<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    ) {
        objective_values(
            &self.rec,
            rtd.generation,
            rtd.current_best.to_usize(),
            rtd.current_worst.to_usize(),
            rtd.current_mean as f64,
            rtd.offspring_mean as f64,
        );

        success_rates(
            &self.rec,
            rtd.generation,
            rtd.success_rate_pt1,
            rtd.success_rate_sma.get_average(),
        );

        #[cfg(feature = "log_pop_stats")]
        {
            population_stats(
                &self.rec,
                rtd.generation,
                rtd.population_size,
                rtd.elite,
                rtd.selection_corrected,
                rtd.distinct_selections,
            );
        };

        #[cfg(feature = "log_ov_dist")]
        {
            objective_value_distribution(
                &self.rec,
                rtd.generation,
                rtd.objective_value_distribution.clone(),
            );
        };

        #[cfg(feature = "log_diversity")]
        {
            population_diversity(
                &self.rec,
                rtd.generation,
                rtd.population_diversity_distribution.clone(),
            );
        };

        #[cfg(feature = "log_cache_hits")]
        {
            cache_hits(
                &self.rec,
                rtd.generation,
                rtd.cache_hits,
                rtd.selection_corrected - rtd.cache_hits,
            );
        };

        #[cfg(feature = "log_runtimes")]
        {
            execution_times(
                &self.rec,
                rtd.generation,
                rtd.execution_times.clone(),
            );
        };
    }

    #[cfg(feature = "log_dynamics")]
    pub fn log_mutation_std_deviation(
        &self,
        generation: usize,
        std_deviation: f32,
    ) {
        self.rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

        let _ = self
            .rec
            .log("mutation/std_dev", &Scalar::new(std_deviation as f64));
    }

    #[cfg(feature = "log_dynamics")]
    pub fn log_mutation_rate(&self, generation: usize, mutation_rate: f32) {
        self.rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

        let _ =
            self.rec.log("mutation/rate", &Scalar::new(mutation_rate as f64));
    }
}

// Functions ///////////////////////////////////////////////////////////////////

/// Log best, worst and mean objective value of the current generation
fn objective_values(
    rec: &RecordingStream,
    generation: usize,
    curr_best: usize,
    curr_worst: usize,
    curr_mean: f64,
    offspring_mean: f64,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log("objective_value/best", &Scalar::new(curr_best as f64));

    let _ = rec.log("objective_value/worst", &Scalar::new(curr_worst as f64));

    let _ = rec.log("objective_value/mean", &Scalar::new(curr_mean));

    let _ =
        rec.log("objective_value/offspring_mean", &Scalar::new(offspring_mean));
}

/// Log the success rate (multiple values because of different calculation
/// methods).
fn success_rates(rec: &RecordingStream, generation: usize, pt1: f32, sma: f32) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log("success_rate/pt1", &Scalar::new(pt1 as f64));

    let _ = rec.log("success_rate/sma", &Scalar::new(sma as f64));
}

/// Log population statistics: total size, elite size, distinct selections,
/// total selections, ...
#[cfg(feature = "log_pop_stats")]
fn population_stats(
    rec: &RecordingStream,
    generation: usize,
    population_size: usize,
    elite: usize,
    selections: usize,
    distinct_selections: usize,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log("pop/size", &Scalar::new(population_size as f64));

    let _ = rec.log("pop/elite", &Scalar::new(elite as f64));

    let _ = rec.log("pop/selections", &Scalar::new(selections as f64));

    let _ = rec.log(
        "pop/distinct_selections",
        &Scalar::new(distinct_selections as f64),
    );
}

/// Log objective value distribution
#[cfg(feature = "log_ov_dist")]
fn objective_value_distribution(
    rec: &RecordingStream,
    generation: usize,
    mut distribution: Vec<usize>,
) {
    // Fill array to next 100
    let target = ((distribution.len() / 100) + 1) * 100;
    let diff = target - distribution.len();
    distribution.extend(vec![0; diff]);

    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log(
        "pop/ov",
        &BarChart::new(
            distribution.into_iter().map(|x| x as u64).collect::<Vec<_>>(),
        ),
    );
}

/// Log population diversity
#[cfg(feature = "log_diversity")]
fn population_diversity(
    rec: &RecordingStream,
    generation: usize,
    mut distribution: Vec<usize>,
) {
    // Fill array to next 100
    let target = ((distribution.len() / 100) + 1) * 100;
    let diff = target - distribution.len();
    distribution.extend(vec![0; diff]);

    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log(
        "pop/diversity",
        &BarChart::new(
            distribution.into_iter().map(|x| x as u64).collect::<Vec<_>>(),
        ),
    );
}

/// Log cache hits (and misses)
#[cfg(feature = "log_cache_hits")]
fn cache_hits(
    rec: &RecordingStream,
    generation: usize,
    cache_hits: usize,
    cache_misses: usize,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log("internal/cache_hits", &Scalar::new(cache_hits as f64));

    let _ = rec.log("internal/cache_misses", &Scalar::new(cache_misses as f64));
}

/// Log execution times
#[cfg(feature = "log_runtimes")]
fn execution_times(
    rec: &RecordingStream,
    generation: usize,
    execution_times: Vec<u128>,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log(
        "internal/execution_times",
        &BarChart::new(
            execution_times.into_iter().map(|x| x as u64).collect::<Vec<u64>>(),
        ),
    );
}

////////////////////////////////////////////////////////////////////////////////