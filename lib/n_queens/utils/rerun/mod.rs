use std::ops::AddAssign;

use hashbrown::HashMap;
use ndarray::Array2;

// Imports /////////////////////////////////////////////////////////////////////
use crate::Stats;

// Constants ///////////////////////////////////////////////////////////////////
const GENERATION_TIME_SEQ: &'static str = "generation";

// Structs /////////////////////////////////////////////////////////////////////
pub struct Logger {
    rec: rerun::RecordingStream,
}

impl Logger {
    pub fn connect() -> Self {
        let rec =
            rerun::RecordingStreamBuilder::new("n_queens_ga").spawn().unwrap();

        Self { rec }
    }

    pub fn log(&self, stats: &Stats) {
        // objective_value: /best, /worst, /avg
        objective_value(
            &self.rec,
            stats.best.len() - 1,
            *stats.best.last().unwrap(),
            *stats.worst.last().unwrap(),
            0.,
        );

        // population size
        population_size(
            &self.rec,
            stats.best.len() - 1,
            *stats.population_size.last().unwrap(),
        );

        // population objective value distribution
        population_objective_value_distribution(
            &self.rec,
            &stats.ov_distribution,
        );

        // chromosome heatmap
        chromosome_heatmap(&self.rec, &stats.chromosome_heatmap);
    }
}

// Functions ///////////////////////////////////////////////////////////////////
fn objective_value(
    rec: &rerun::RecordingStream,
    generation: usize,
    curr_best: usize,
    curr_worst: usize,
    curr_avg: f64,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ =
        rec.log("objective_value/best", &rerun::Scalar::new(curr_best as f64));

    let _ = rec
        .log("objective_value/worst", &rerun::Scalar::new(curr_worst as f64));

    let _ = rec.log("objective_value/avg", &rerun::Scalar::new(curr_avg));
}

fn population_size(
    rec: &rerun::RecordingStream,
    generation: usize,
    population_size: usize,
) {
    rec.set_time_sequence(GENERATION_TIME_SEQ, generation as u32);

    let _ =
        rec.log("population/size", &rerun::Scalar::new(population_size as f64));
}

fn population_objective_value_distribution(
    rec: &rerun::RecordingStream,
    distribution: &[(usize, usize)],
) {
    let objective_values: Vec<usize> =
        distribution.iter().map(|(ov, _)| *ov).collect();

    let max = *objective_values.iter().max().unwrap();
    let max = ((max / 100) + 1) * 100;

    let mut map = HashMap::<usize, usize>::new();
    for i in 0..=max {
        map.insert(i, 0);
    }

    // Collect
    for ov in objective_values {
        map.entry(ov).or_default().add_assign(1);
    }

    // Convert hash map to vector and sort it
    let mut results: Vec<(usize, usize)> = map.into_iter().collect();
    results.sort_by_key(|(ov, _amount)| *ov);

    let _ = rec.log(
        "population/objective_value_distribution",
        &rerun::BarChart::new(
            results
                .into_iter()
                .map(|(_ov, amount)| amount as u64)
                .collect::<Vec<u64>>(),
        ),
    );
}
use ndarray::ShapeBuilder;
fn chromosome_heatmap(_rec: &rerun::RecordingStream, _data: &Array2<usize>) {
    // let m = data.shape()[0];
    // let n = data.shape()[1];
    // let d = Array2::<u32>::from_shape_vec((m, n), data.into_iter().map(|x| *x as u32).collect()).unwrap();

    // let tensor = rerun::Tensor::try_from(&d).unwrap()
    //     .with_dim_names(["row", "col"]);

    let mut data = ndarray::Array::<u8, _>::default((8, 3, 3).f());
    data.map_inplace(|x| *x = rand::random());

    // let tensor = rerun::Tensor::try_from(data)
    //     .unwrap()
    //     .with_dim_names(["width", "height", "channel", "batch"]);

    // rec.log("tensor", &tensor).unwrap();
}

////////////////////////////////////////////////////////////////////////////////
