// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    RuntimeData,
};
use rerun::{BarChart, RecordingStream, RecordingStreamBuilder, Scalar};

// Constants ///////////////////////////////////////////////////////////////////
const GENERATION_TIME_SEQ: &'static str = "generation";

// Rerun Logger ////////////////////////////////////////////////////////////////
pub struct RerunLogger {
    rec: RecordingStream,
}

impl RerunLogger {
    pub fn connect(name: &str) -> Self {
        let rec = RecordingStreamBuilder::new(name).spawn().unwrap();
        Self { rec }
    }

    pub fn log<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>>(
        &self,
        rtd: &RuntimeData<Ov, Ctx, Ge>,
    ) {
        objective_values(
            &self.rec,
            rtd.generation,
            rtd.current_best.1.to_usize(),
            rtd.current_worst.1.to_usize(),
            rtd.current_mean as f64,
            rtd.offspring_mean as f64,
        );

        population_stats(
            &self.rec,
            rtd.generation,
            rtd.population_size,
            rtd.elite,
            rtd.selection_corrected,
            rtd.distinct_selections,
        );

        objective_value_distribution(
            &self.rec,
            rtd.generation,
            rtd.objective_value_distribution.clone(),
        );
        population_diversity(
            &self.rec,
            rtd.generation,
            rtd.population_diversity_distribution.clone(),
        );

        internals(
            &self.rec,
            rtd.generation,
            rtd.cache_hits,
            rtd.execution_times.clone(),
        );
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

/// Log population statistics: total size, elite size, distinct selections,
/// total selections, ...
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

/// Log more technical data (cache hits, execution times)
fn internals(
    rec: &RecordingStream,
    generation: usize,
    cache_hits: usize,
    execution_times: Vec<u128>,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ = rec.log("internal/cache_hits", &Scalar::new(cache_hits as f64));

    let _ = rec.log(
        "internal/execution_times",
        &BarChart::new(
            execution_times.into_iter().map(|x| x as u64).collect::<Vec<u64>>(),
        ),
    );
}

////////////////////////////////////////////////////////////////////////////////
