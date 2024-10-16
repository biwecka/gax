// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    operators::{Crossover, Mutation},
    parameters::Parameters,
    process::{
        rejection::Rejection, replacement::Replacement, selection::Selection,
        termination::Termination,
    },
};

use shannon_entropy::normalized_shannon_entropy;
use statrs::statistics::{Data, Distribution, Median};

// Runtime Data ////////////////////////////////////////////////////////////////
pub struct RuntimeData<
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
> {
    pub generation: usize,
    pub population_size: usize,
    pub elite: usize,

    pub best: Ov,
    pub worst: Ov,
    pub mean: f64,
    pub median: f64,
    pub variance: f64,
    pub std_dev: f64,
    pub diversity: f64,

    pub selection_corrected: usize,
    pub distinct_selections: usize,
    // pub offspring_mean: f32,
    pub cache_hits: usize,
    pub execution_times: Vec<u128>,

    /// True, when the current generation improved on the best solution.
    pub success: bool,
    pub last_success: usize,

    /// Moving average calculated by a PT1-lowpass filter function.
    pub success_rate_pt1: f32,

    // PhantomData
    objective_value: std::marker::PhantomData<Ov>,
    context: std::marker::PhantomData<Ctx>,
    genotype: std::marker::PhantomData<Ge>,
    crossover: std::marker::PhantomData<Cr>,
    mutation: std::marker::PhantomData<Mu>,
    t: std::marker::PhantomData<T>,
    selection: std::marker::PhantomData<Se>,
    rejection: std::marker::PhantomData<Re>,
    replacement: std::marker::PhantomData<Rp>,
    termination: std::marker::PhantomData<Te>,
}

impl<
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
    > RuntimeData<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>
{
    pub fn init(
        initial_population: &[(Ge, Ov)],
        _params: &Parameters<Ov, Ctx, Ge, Cr, Mu, T, Se, Re, Rp, Te>,
    ) -> Self {
        // Set initial generation
        let generation = 0;

        // Population size
        let population_size = initial_population.len();

        // Population stats
        let best = initial_population.first().unwrap().1.clone();
        let worst = initial_population.last().unwrap().1.clone();

        let objective_values =
            initial_population.iter().map(|(_, ov)| ov.to_usize());

        let diversity = normalized_shannon_entropy(objective_values.clone());

        let objective_values_f64 =
            objective_values.into_iter().map(|x| x as f64).collect::<Vec<_>>();

        let dataset = Data::new(objective_values_f64);
        let mean = dataset.mean().unwrap_or(0.);
        let median = dataset.median();
        let variance = dataset.variance().unwrap_or(0.);
        let std_dev = dataset.std_dev().unwrap_or(0.);

        Self {
            generation,
            population_size,
            elite: 0,

            best,
            worst,
            mean,
            median,
            variance,
            std_dev,
            diversity,

            selection_corrected: 0,
            distinct_selections: 0,

            cache_hits: 0,
            execution_times: vec![],

            success: false,
            last_success: 0,
            success_rate_pt1: 0.,

            objective_value: std::marker::PhantomData,
            context: std::marker::PhantomData,
            genotype: std::marker::PhantomData,
            crossover: std::marker::PhantomData,
            mutation: std::marker::PhantomData,
            t: std::marker::PhantomData,
            selection: std::marker::PhantomData,
            rejection: std::marker::PhantomData,
            replacement: std::marker::PhantomData,
            termination: std::marker::PhantomData,
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
        cache_hits: usize,
    ) {
        self.selection_corrected = selection_corrected;
        self.distinct_selections = distinct_selections;

        // Update population size
        self.population_size = population.len();

        // Update current best and re-calculate success rates
        self.best = {
            let new_best = population.first().unwrap().1.clone();

            if new_best < self.best {
                self.success_rate_pt1 =
                    crate::utils::pt1(self.success_rate_pt1, 1., 100.);

                self.success = true;
                self.last_success = self.generation;
            } else {
                self.success_rate_pt1 =
                    crate::utils::pt1(self.success_rate_pt1, 0., 100.);

                self.success = false;
            }

            new_best
        };

        // Update current worst
        self.worst = population.last().unwrap().1.clone();

        // Update diversity, mean, meadian, variance and std_dev.
        let objective_values = population.iter().map(|(_, ov)| ov.to_usize());

        self.diversity = normalized_shannon_entropy(objective_values.clone());

        let objective_values_f64 =
            objective_values.into_iter().map(|x| x as f64).collect::<Vec<_>>();

        let dataset = Data::new(objective_values_f64);
        self.mean = dataset.mean().unwrap_or(0.);
        self.median = dataset.median();
        self.variance = dataset.variance().unwrap_or(0.);
        self.std_dev = dataset.std_dev().unwrap_or(0.);

        // Update elite
        self.elite = elite;

        // Update cache hits
        self.cache_hits = cache_hits;
    }

    #[cfg(feature = "log_runtimes")]
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
