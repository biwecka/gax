// Modules /////////////////////////////////////////////////////////////////////
pub mod encoding;
pub mod parameters;
pub mod stats;

// Imports /////////////////////////////////////////////////////////////////////
use parameters::{Adaption, Logger, Parameters};

// Structs /////////////////////////////////////////////////////////////////////
pub struct GeneticAlgorithm<Ov, Ct, Ge, Ph, St, Se, Cx, Mu, Of, Te, Ad, Lo>
where
    Ov: encoding::ObjectiveValue,
    Ct: encoding::Context,
    Ge: encoding::Genotype<Ct>,
    Ph: encoding::PhenotypeBase<Ov, Ct, Ge>,
    St: stats::Stats<Ov, Ct, Ge>,

    Se: parameters::SelectionStrategies,
    Cx: parameters::CrossoverStrategies,
    Mu: parameters::MutationStrategies,
    Of: parameters::OffspringRejectionStrategies,
    Te: parameters::TerminationStrategies<Ov>,
    Ad: Adaption,
    Lo: Logger,
{
    params: Parameters<Ov, Ct, Ge, St, Se, Cx, Mu, Of, Te, Ad, Lo>,
    stats: St,
    ctx: Ct,
    phenotype: Ph,
}

impl<Ct, Ov, Ge, Ph, St, Se, Cx, Mu, Of, Te, Ad, Lo>
    GeneticAlgorithm<Ov, Ct, Ge, Ph, St, Se, Cx, Mu, Of, Te, Ad, Lo>
where
    Ov: encoding::ObjectiveValue,
    Ct: encoding::Context,
    Ge: encoding::Genotype<Ct>,
    Ph: encoding::PhenotypeBase<Ov, Ct, Ge>,
    St: stats::Stats<Ov, Ct, Ge>,

    Se: parameters::SelectionStrategies,
    Cx: parameters::CrossoverStrategies,
    Mu: parameters::MutationStrategies,
    Of: parameters::OffspringRejectionStrategies,
    Te: parameters::TerminationStrategies<Ov>,
    Ad: Adaption,
    Lo: Logger,
{
    pub fn init(
        params: Parameters<Ov, Ct, Ge, St, Se, Cx, Mu, Of, Te, Ad, Lo>,
        stats: St,
        ctx: Ct,
        phenotype: Ph,
    ) -> Self {
        Self { params, stats, ctx, phenotype }
    }

    pub fn run(&self) {}
}

////////////////////////////////////////////////////////////////////////////////

// // Imports /////////////////////////////////////////////////////////////////////

// ////////////////////////////////////////////////////////////////////////////////

// pub enum Value {
//     Rate(f32),
//     Absolute(u32),
// }

// pub trait Params {
//     /// Returns the mutation rate which must be specified in the config.
//     fn mutation_rate(&self) -> f32;

//     /// Returns the crossover rate which must be specified in the config.
//     fn crossover_rate(&self) -> f32;

//     /// Calculate the absolute amount of elitist individuals based on the
//     /// current parameters.
//     fn calc_abs_elitism(&self) -> usize;

//     /// Get population size
//     fn population_size(&self) -> usize;

//     /// Check if adaptive parameter steering is enabled
//     fn adaptive_steering(&self) -> bool;

//     /// Mutate the parameters according to the defined adaptive rules.
//     fn adapt(&mut self);
// }

// pub trait Genotype: Sized + PartialEq + Eq + Clone {
//     fn generate<P: Params>(params: &P) -> Vec<Self>;

//     fn mutate(&mut self, rate: f32);
// }

// /// Implementors of this trait must ensure, that their ordering is always
// /// DESCENDING in the sense of the fitness measure they represent.
// /// Some examples:
// /// -   Fitness from 0.0 to 1.0, where 1.0 is the goal (maximization):
// ///     A sorted vector must then look like this [0.9, 0.7, 0.6, 0.6, ...]
// /// -   Fitness from 0 to 1000, where 0 is the goal (minimization):
// ///     A sorted vector must then look like this [20, 50, 110, 120, 300, ...]
// ///
// pub trait Fitness: PartialEq + Eq + PartialOrd + Ord + Clone {}

// #[derive(Clone)]
// pub struct Evaluation<G: Genotype, F: Fitness> {
//     genotype: G,
//     fitness: F,
// }

// impl<G: Genotype, F: Fitness> Evaluation<G, F> {
//     fn new(genotype: G, fitness: F) -> Self {
//         Self { genotype, fitness }
//     }

//     fn genotype(&self) -> &G {
//         &self.genotype
//     }

//     fn fitness(&self) -> &F {
//         &self.fitness
//     }
// }

// impl<G: Genotype, F: Fitness> std::cmp::Ord for Evaluation<G, F> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.fitness.cmp(&other.fitness)
//     }
// }

// impl<G: Genotype, F: Fitness> std::cmp::PartialOrd for Evaluation<G, F> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.fitness.cmp(&other.fitness))
//     }
// }

// impl<G: Genotype, F: Fitness> std::cmp::PartialEq for Evaluation<G, F> {
//     fn eq(&self, other: &Self) -> bool {
//         self.genotype.eq(&other.genotype)
//     }
// }

// impl<G: Genotype, F: Fitness> std::cmp::Eq for Evaluation<G, F> {}

// pub trait PhenotypeState {}
// pub struct Base;        impl PhenotypeState for Base {}
// pub struct Derived;     impl PhenotypeState for Derived {}

// pub trait PhenotypeBase {
//     fn derive<G: Genotype>(&self, chromosome: &G) -> impl PhenotypeDerived;
// }

// pub trait PhenotypeDerived {
//     fn evaluate<F: Fitness>(&self) -> F;
// }

// pub trait Stats: Default {
//     fn generation_num(&self) -> usize;
//     fn inc_generation_num(&mut self);
//     fn update(&mut self);
// }

// pub trait TerminationCriterion {
//     fn satisfied<M: Stats>(&self, stats: &M) -> bool;
// }

// pub struct Selection<G: Genotype, F: Fitness>(
//     Vec<Evaluation<G, F>>,  // Vector of selected individuals
//     Option<u32>             // Optional: amount of distinct selected individuals
// );

// pub trait Select<G, F>
// where
//     G: Genotype,
//     F: Fitness,
// {
//     fn select(&self, amount: usize) -> Selection<G, F>;
// }

// pub trait Crossover<G, F>
// where
//     G: Genotype,
//     F: Fitness,
// {
//     fn crossover(a: &G, b: &G, rate: f32) -> (G, G);
// }

// pub struct Runtime<P, G, Ph, F, S, C, M, T>
// where
//     P: Params,
//     G: Genotype,
//     Ph: PhenotypeBase,
//     F: Fitness,

//     S: Select<G, F>,
//     C: Crossover<G, F>,

//     M: Stats,
//     T: TerminationCriterion,
// {
//     params: P,
//     genotype: std::marker::PhantomData<G>,
//     phenotype: Ph,
//     fitness: std::marker::PhantomData<F>,
//     stats: std::marker::PhantomData<M>,
//     termination_criterion: T,
//     selection: S,
//     crossover: std::marker::PhantomData<C>,
// }

// impl<P, G, Ph, F, S, C, St, T> Runtime<P, G, Ph, F, S, C, St, T>
// where
//     P: Params,
//     G: Genotype,
//     Ph: PhenotypeBase,
//     F: Fitness,

//     S: Select<G, F>,
//     C: Crossover<G, F>,

//     St: Stats,
//     T: TerminationCriterion,
// {
//     pub fn setup(
//         params: P,
//         phenotype: Ph,
//         termination_criterion: T,
//         selection: S,
//     ) -> Self {
//         Self {
//             params,
//             genotype: std::marker::PhantomData,
//             phenotype,
//             fitness: std::marker::PhantomData,
//             stats: std::marker::PhantomData,
//             termination_criterion,
//             selection,
//             crossover: std::marker::PhantomData,
//         }
//     }

//     pub fn start(mut self) -> Evaluation<G, F> {
//         // Init population
//         let individuals: Vec<G> = G::generate(&self.params);

//         // Evaluate individuals to create initial population
//         let mut population: Vec<Evaluation::<G, F>> = individuals
//             .into_iter()
//             .map(|chromosome| {
//                 // Derive phenotype from chromosome/genotype
//                 let phenotype = self.phenotype.derive(&chromosome);
//                 let fitness = phenotype.evaluate::<F>();
//                 drop(phenotype);

//                 Evaluation::new(chromosome, fitness)
//             })
//             .collect();

//         // Sort population
//         population.sort();

//         // Genetic evolution loop
//         let mut stats = St::default();
//         loop {
//             // Calculations
//             let elitism_num = self.params.calc_abs_elitism();
//             let select_num = self.params.population_size() - elitism_num;
//             let cx_rate = self.params.crossover_rate();
//             let mut_rate = self.params.mutation_rate();

//             // Select
//             let Selection(parents, distinct) = self.selection.select(select_num);

//             // Crossover + Mutate
//             let offspring: Vec<Evaluation<G, F>> = parents
//                 .chunks(2)
//                 .map(|parents| {
//                     let a = &parents[0];
//                     let b = &parents[1];

//                     // Crossover
//                     let (mut x, mut y) =
//                         C::crossover(&a.genotype, &b.genotype, cx_rate);

//                     // Mutation
//                     x.mutate(mut_rate);
//                     y.mutate(mut_rate);

//                     // Evaluation
//                     let px = self.phenotype.derive(&x);
//                     let fx = px.evaluate::<F>();
//                     drop(px);
//                     let offspring_0 = Evaluation::new(x, fx);

//                     let py = self.phenotype.derive(&y);
//                     let fy = py.evaluate::<F>();
//                     drop(py);
//                     let offspring_1 = Evaluation::new(y, fy);

//                     // Offspring Rejection (TODO)

//                     // Return
//                     vec![offspring_0, offspring_1]
//                 })
//                 .flatten()
//                 .collect();

//             // Replace
//             population.splice(elitism_num.., offspring);

//             // Sort
//             population.sort();

//             // Stats
//             stats.update();

//             // Log stats (TODO)

//             // Check stop condition and break loop
//             if !self.termination_criterion.satisfied(&stats) {
//                 // Adaptive steering
//                 if self.params.adaptive_steering() { self.params.adapt() }

//                 // Increase generation counter
//                 stats.inc_generation_num();

//             } else {
//                 break;
//             }
//         }

//         // Return best chromosome (the population is assumed to be sorted)
//         population.first().unwrap().clone()
//     }
// }

// pub struct Timetable<S: PhenotypeState> {
//     marker: std::marker::PhantomData<S>,

//     x: Option<bool>,
// }

// impl PhenotypeBase for Timetable<Base> {
//     fn derive(&self) -> impl PhenotypeDerived {
//         Timetable::<Derived> { marker: std::marker::PhantomData, x: Some(true) }
//     }
// }

// impl PhenotypeDerived for Timetable<Derived> {
//     fn evaluate(&self) {
//         todo!()
//     }
// }

////////////////////////////////////////////////////////////////////////////////
