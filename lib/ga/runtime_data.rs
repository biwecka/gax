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
use simple_moving_average::{SumTreeSMA, SMA};

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
    pub current_best: Ov,
    pub current_worst: Ov,
    pub current_mean: f32,

    pub elite: usize,
    pub selection_corrected: usize,
    pub distinct_selections: usize,
    pub offspring_mean: f32,
    pub cache_hits: usize,

    pub execution_times: Vec<u128>,

    #[cfg(feature = "log_ov_dist")]
    pub objective_value_distribution: Vec<usize>,

    #[cfg(feature = "log_diversity")]
    pub population_diversity_distribution: Vec<usize>,

    /// True, when the current generation improved on the best solution.
    pub success: bool,
    pub last_success: usize,

    /// Moving average calculated by a PT1-lowpass filter function.
    pub success_rate_pt1: f32,

    /// Moving average calculated by a "simple moving average" algorithm.
    pub success_rate_sma: SumTreeSMA<f32, f32, 100>,

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
        // Calculate data
        let generation = 0;
        let population_size = initial_population.len();
        let current_best = match initial_population.first() {
            Some(x) => x.clone().1,
            None => unreachable!(),
        };
        let current_worst = match initial_population.last() {
            Some(x) => x.clone().1,
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

            #[cfg(feature = "log_ov_dist")]
            objective_value_distribution: vec![],

            #[cfg(feature = "log_diversity")]
            population_diversity_distribution: vec![],

            success: false,
            last_success: 0,
            success_rate_pt1: 0.,
            success_rate_sma: SumTreeSMA::new(),

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
        offspring_mean: f32,
        cache_hits: usize,
    ) {
        // Pre-calculate filtered list with objective values only
        let objective_values =
            population.iter().map(|(_, ov)| ov.clone()).collect::<Vec<_>>();

        // Update population size
        self.population_size = population.len();

        // Update current best and re-calculate success rates
        self.current_best = match population.first() {
            Some(x) => {
                let new_best = &x.1;

                if x.1 < self.current_best {
                    self.success_rate_sma.add_sample(1.);

                    self.success_rate_pt1 =
                        crate::utils::pt1(self.success_rate_pt1, 1., 100.);

                    self.success = true;
                    self.last_success = self.generation;
                } else {
                    self.success_rate_sma.add_sample(0.);

                    self.success_rate_pt1 =
                        crate::utils::pt1(self.success_rate_pt1, 0., 100.);

                    self.success = false;
                }

                new_best.clone()
            }
            None => unreachable!(),
        };

        // Update current worst
        self.current_worst = match population.last() {
            Some(x) => x.clone().1,
            None => unreachable!(),
        };

        // Update current mean
        self.current_mean = Ov::calc_average(&objective_values);

        // Update elite
        self.elite = elite;

        // Update number of selected individuals
        self.selection_corrected = selection_corrected;

        // Update number of distinct selected individuals
        self.distinct_selections = distinct_selections;

        // Update offspring mean objective value
        self.offspring_mean = offspring_mean;

        // Update cache hits
        self.cache_hits = cache_hits;

        // Calculate objective value distribution
        #[cfg(feature = "log_ov_dist")]
        {
            self.objective_value_distribution =
                Ov::calc_distribution(&objective_values);
        };

        // Calculate population diversity
        #[cfg(feature = "log_diversity")]
        {
            self.population_diversity_distribution =
                Ge::calc_diversity(population);
        };
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
