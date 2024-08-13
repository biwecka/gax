// Imports /////////////////////////////////////////////////////////////////////
use super::{Context, Genotype, ObjectiveValue};

// Trait ///////////////////////////////////////////////////////////////////////
/// The phenotype is the actual, decoded representation of the genotype. It is
/// the candidate solution in usable form, which can be evaluated to determine
/// how well the genotype solves the problem.
pub trait State {}

#[derive(Clone)]
pub struct Base;
impl State for Base {}

pub struct Derived;
impl State for Derived {}

pub trait PhenotypeBase<Ov, Ct, Gt>: Clone
where
    Ov: ObjectiveValue,
    Ct: Context,
    Gt: Genotype<Ct>,
{
    fn derive(&self, chromosome: &Gt) -> impl PhenotypeDerived<Ov>;
}

pub trait PhenotypeDerived<Ov>
where
    Ov: ObjectiveValue,
{
    fn evaluate(&self) -> Ov;
}

////////////////////////////////////////////////////////////////////////////////
