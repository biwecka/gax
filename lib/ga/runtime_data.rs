// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};

// Runtime Data ////////////////////////////////////////////////////////////////
pub struct RuntimeData<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>> {
    pub generation: usize,
    pub population_size: usize,
    pub current_best: (Ge, Ov),
    pub current_worst: (Ge, Ov),
    pub current_mean: f32,

    pub elite: usize,
    pub selection_corrected: usize,
    pub distinct_selections: usize,
    pub offspring_mean: f32,
    pub cache_hits: usize,

    pub execution_times: Vec<u128>,

    pub objective_value_distribution: Vec<usize>,
    pub population_diversity_distribution: Vec<usize>,

    // PhantomData
    context: std::marker::PhantomData<Ctx>,
}

impl<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>>
    RuntimeData<Ov, Ctx, Ge>
{
    pub fn init(initial_population: &[(Ge, Ov)]) -> Self {
        // Calculate data
        let generation = 0;
        let population_size = initial_population.len();
        let current_best = match initial_population.first() {
            Some(x) => x.clone(),
            None => unreachable!(),
        };
        let current_worst = match initial_population.last() {
            Some(x) => x.clone(),
            None => unreachable!(),
        };
        let current_mean: f32 = Ov::calc_average(
            &initial_population
                .iter()
                .map(|(_, ov)| ov.clone())
                .collect::<Vec<_>>(),
        );

        Self {
            generation,
            population_size,
            current_best,
            current_worst,
            current_mean,

            elite: 0,
            selection_corrected: 0,
            distinct_selections: 0,
            offspring_mean: 0.,
            cache_hits: 0,

            execution_times: vec![],
            objective_value_distribution: vec![],
            population_diversity_distribution: vec![],

            context: std::marker::PhantomData,
        }
    }

    pub fn inc_generation(&mut self) {
        self.generation += 1;
    }

    pub fn update(
        &mut self,
        population: &[(Ge, Ov)],
        elite: usize,
        selection_corrected: usize,
        distinct_selections: usize,
        offspring_mean: f32,
        cache_hits: usize,
    ) {
        // Pre-calculate filtered list with objective values only
        let objective_values =
            population.iter().map(|(_, ov)| ov.clone()).collect::<Vec<_>>();

        self.population_size = population.len();
        self.current_best = match population.first() {
            Some(x) => x.clone(),
            None => unreachable!(),
        };
        self.current_worst = match population.last() {
            Some(x) => x.clone(),
            None => unreachable!(),
        };
        self.current_mean = Ov::calc_average(&objective_values);
        self.elite = elite;
        self.selection_corrected = selection_corrected;
        self.distinct_selections = distinct_selections;
        self.offspring_mean = offspring_mean;
        self.cache_hits = cache_hits;

        // Calculate objective value distribution
        self.objective_value_distribution =
            Ov::calc_distribution(&objective_values);

        // // Calculate population diversity
        self.population_diversity_distribution =
            Ge::calc_diversity(&population);
    }

    pub fn update_execution_times(
        &mut self,
        execution_times: Vec<std::time::Duration>,
    ) {
        // Convert
        let times_in_microseconds = execution_times
            .into_iter()
            .map(|t| t.as_micros())
            .collect::<Vec<u128>>();

        // Set
        self.execution_times = times_in_microseconds;
    }
}

////////////////////////////////////////////////////////////////////////////////
