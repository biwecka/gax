// Modules /////////////////////////////////////////////////////////////////////
#[rustfmt::skip] mod builder;
pub use builder::*;

// Imports /////////////////////////////////////////////////////////////////////
use std::{fmt::Debug, hash::Hash};

// Objective Value /////////////////////////////////////////////////////////////

/// This trait must be implemented by a struct which represents the objective
/// of the genetic search. This is also known as *fitness* or *cost*.
///
/// The genetic algorithm runtime uses Rust's `.sort()` function (and its
/// derivatives), which by default sorts ascendingly (e.g. numbers).
/// If an objective value represents a measure which should be maximized by
/// the genetic search, make sure to implement the [`PartialOrd`] and [`Ord`]
/// traits for the struct accordingly.
///
pub trait ObjectiveValue:
    Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Send + Sync + Hash
{
    fn calc_average(values: &[Self]) -> f32;

    /// This function calculates the objective value distribution for the passed
    /// objective values.
    /// The passed objective values must therefore be sorted into buckets of
    /// equal value and then counted.
    ///
    /// In the resulting array the INDEX represents the OBJECTIVE VALUE,
    /// and the value at an index represents the amount of times this objecive
    /// value occured.
    fn calc_distribution(values: &[Self]) -> Vec<usize>;

    fn to_usize(&self) -> usize;
}

// Context /////////////////////////////////////////////////////////////////////

/// The context trait helps injecting arbitrary data into certain parts of the
/// genetic algorithm runtime. An example use case for a context is to define
/// different distributions of random values (with different ranges etc.) which
/// are needed throughout the whole genetic algorithm.
/// Such random number distributions, and other useful data, which is not
/// strictly part of the genotype or phenotype, can be stored inside of the
/// context.
/// The genetic algorithm runtime will make the context accessible at certain
/// function calls.
///
pub trait Context: Send + Sync {}

// Genotype ////////////////////////////////////////////////////////////////////

/// TODO: docs
pub trait Genotype<Ctx: Context>:
    Clone + Debug + PartialEq + Eq + Send + Sync + Hash
{
    /// The generate method of the genotype is used to create the initial
    /// population for the genetic algorithm.
    /// The `amount` parameter specifies how many individuals/chromosomes
    /// should be created.
    /// The `ctx` parameter makes the custom context available to this function,
    /// which may contain pre-defined random value generators/distributions
    /// that help with generating lots of chromosomes.
    fn generate(amount: usize, ctx: &Ctx) -> Vec<Self>;

    /// This function calculates the diversity data of the population.
    /// Therefore equal chromosomes are grouped and counted.
    /// Afterwards they are sorted by their fitness and returned.
    /// Therefore `result[i]` should contain the amount of times, the fittest
    /// chromosome occured.
    fn calc_diversity<Ov: ObjectiveValue>(
        population: &[(Self, Ov)],
    ) -> Vec<usize>;
}

// Phenotype ///////////////////////////////////////////////////////////////////

/// TODO: docs
/// TODO: type-state pattern
pub trait Phenotype<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>>:
    Clone + Debug + Send + Sync
{
    fn derive(&self, chromsome: &Ge, ctx: &Ctx) -> Self;

    fn evaluate(&self, ctx: &Ctx) -> Ov;
}

////////////////////////////////////////////////////////////////////////////////
