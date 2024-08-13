// Imports /////////////////////////////////////////////////////////////////////
use super::encoding::{Genotype, ObjectiveValue};
use crate::encoding::Context;

// Traits //////////////////////////////////////////////////////////////////////
pub trait Stats<
    Ov: ObjectiveValue,
    Ct: Context,
    Ge: Genotype<Ct>,
    // St: Stats<Ov>,
    // Am: AdaptionMetrics<Ov, St>,
    // Pa: GeneticAlgorithmParameters<Ov, St, Am>, //<Ov, Ct, Ge>,
>
{
    /// Get current generation number
    fn generation(&self) -> usize;

    /// Increase internal generation counter.
    fn inc_gen(&mut self);

    /// Get current best objective value from the population.
    fn curr_best(&self) -> Ov;

    /// Update (re-calculate) the stats. This is mostly done in preparation
    /// for logging the stats.
    fn update(
        &mut self,
        population: &[(Ge, Ov)],
        // params: &Pa, //Parameters<Ov, Ct, Ge, Se, Cx, Mu, Of, Te, Ad>, // TODO
        distinct_selections: usize,
    );

    /// Log stats.
    fn log(&self);
}

////////////////////////////////////////////////////////////////////////////////
