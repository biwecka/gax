// Modules /////////////////////////////////////////////////////////////////////
mod adaption;
mod crossover;
mod logger;
mod mutation;
mod offspring_rejection;
mod replacement;
mod selection;
mod termination;

// Imports /////////////////////////////////////////////////////////////////////
pub use adaption::*;
pub use crossover::*;
pub use logger::*;
pub use mutation::*;
pub use offspring_rejection::*;
pub use replacement::*;
pub use selection::*;
pub use termination::*;

use crate::{
    encoding::{Context, Genotype, ObjectiveValue},
    stats::Stats,
};

// pub trait GeneticAlgorithmParameters<
//     Ov: ObjectiveValue,
//     Ct: Context,
//     Ge: Genotype<Ct>,
//     St: Stats<Ov, Ct, Ge>, //, Ct, Ge, GeneticAlgorithmParameters<Ov, Ct, Ge, St>>,
//     // Am: AdaptionMetrics<Ov, St>,
// > {
//     /// Function to calculate the amount of individuals that must be selected
//     /// from the current population, to yield just enough offspring, based
//     /// on the chosen replacement strategy.
//     fn selection_size(&self) -> usize;

//     /// Calculates the size of the elite group, based on the chosen replacement
//     /// strategy.
//     fn elite_size(&self) -> usize;

//     /// Automatically adapts the parameters according to the strategy specified
//     /// in the `adaption` field.
//     fn adapt(&mut self, stats: &St);

//     /// Call the logger
//     fn log(&self, stats: &St);
// }

// Structs /////////////////////////////////////////////////////////////////////
pub struct Parameters<Ov, Ct, Ge, St, Se, Cx, Mu, Of, Te, Ad, Lo>
where
    Ov: ObjectiveValue,
    Ct: Context,
    Ge: Genotype<Ct>,
    St: Stats<Ov, Ct, Ge>,

    Se: SelectionStrategies,
    Cx: CrossoverStrategies,
    Mu: MutationStrategies,
    Of: OffspringRejectionStrategies,
    Te: TerminationStrategies<Ov>,
    Ad: Adaption,
    Lo: Logger,
{
    // TODO: remove?
    marker0: std::marker::PhantomData<Ge>,
    marker1: std::marker::PhantomData<Ov>,
    marker2: std::marker::PhantomData<Ct>,
    marker3: std::marker::PhantomData<St>,

    pub population_size: usize,

    pub selection: Se,

    pub crossover_rate: f32,
    pub crossover: Cx,
    pub mutation_rate: f32,
    pub mutation: Mu,
    pub offspring_rejection: Of,

    pub replacement: Replacement,
    pub termination: Te,

    pub adaption: Ad,

    pub logger: Lo,
}

// impl<
//     Ov: ObjectiveValue,
//     Ct: Context,
//     Ge: Genotype<Ct>,
//     St: Stats<Ov, Ct, Ge>,

//     Se: SelectionStrategies,
//     Cx: CrossoverStrategies,
//     Mu: MutationStrategies,
//     Of: OffspringRejectionStrategies,
//     Te: TerminationStrategies<Ov>,
//     Ad: Adaption,
//     Lo: Logger,
// > GeneticAlgorithmParameters<Ov, Ct, Ge, St>
//     for Parameters<Ov, Ct, Ge, St, Se, Cx, Mu, Of, Te, Ad, Lo>
// {

impl<
        Ov: ObjectiveValue,
        Ct: Context,
        Ge: Genotype<Ct>,
        St: Stats<Ov, Ct, Ge>,
        Se: SelectionStrategies,
        Cx: CrossoverStrategies,
        Mu: MutationStrategies,
        Of: OffspringRejectionStrategies,
        Te: TerminationStrategies<Ov>,
        Ad: Adaption,
        Lo: Logger,
    > Parameters<Ov, Ct, Ge, St, Se, Cx, Mu, Of, Te, Ad, Lo>
{
    /// Function to calculate the amount of individuals that must be selected
    /// from the current population, to yield just enough offspring, based
    /// on the chosen replacement strategy.
    fn selection_size(&self) -> usize {
        self.population_size - self.elite_size()
    }

    /// Calculates the size of the elite group, based on the chosen replacement
    /// strategy.
    fn elite_size(&self) -> usize {
        todo!()
    }

    /// Automatically adapts the parameters according to the strategy specified
    /// in the `adaption` field.
    fn adapt(&mut self, stats: &St) {
        todo!()
    }

    fn log(&self, stats: &St) {
        self.logger.log();
    }
}

////////////////////////////////////////////////////////////////////////////////
