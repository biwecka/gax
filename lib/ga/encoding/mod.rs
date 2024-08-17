// Modules /////////////////////////////////////////////////////////////////////
#[rustfmt::skip] mod builder;
pub use builder::*;

// Imports /////////////////////////////////////////////////////////////////////
use std::fmt::Debug;

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
    Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Send + Sync
{
    fn calc_average(values: &[Self]) -> f32;
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
    Clone + Debug + PartialEq + Eq + Send + Sync
{
    /// The generate method of the genotype is used to create the initial
    /// population for the genetic algorithm.
    /// The `amount` parameter specifies how many individuals/chromosomes
    /// should be created.
    /// The `ctx` parameter makes the custom context available to this function,
    /// which may contain pre-defined random value generators/distributions
    /// that help with generating lots of chromosomes.
    fn generate(amount: usize, ctx: &Ctx) -> Vec<Self>;
}

// Phenotype ///////////////////////////////////////////////////////////////////

/// TODO: docs
/// TODO: type-state pattern
pub trait Phenotype<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>>:
    Clone + Debug + Send + Sync
{
    fn derive(&self, chromsome: &Ge) -> Self;

    fn evaluate(&self) -> Ov;
}

////////////////////////////////////////////////////////////////////////////////
