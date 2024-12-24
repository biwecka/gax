// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};
use std::marker::PhantomData;

// Report //////////////////////////////////////////////////////////////////////
/// This struct represents the return value of the algorithm and therefore
/// contains not only all individuals of the final generation (including their
/// objective values), but also valuable metrics and a detailed log of all
/// generations, which can be used in post-analyzation of each algorithm
/// execution.
#[derive(Clone)]
pub struct Report<Ov, Ctx, Ge>
where
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
{
    /// All chromosomes and their objective value of the last generation sorted
    /// ascendingly, so that the best solution is the FIRST chromosome in the
    /// list.
    pub population: Vec<(Ge, Ov)>,

    pub generation: usize,

    /// Total runtime of the algorithm in SECONDS.
    pub runtime: usize,

    pub parameter_identifier: String,
    pub dynamics_identifier: Option<String>,

    pub log: Vec<ReportLog>,

    pub ctx: PhantomData<Ctx>,
}

#[derive(Clone)]
pub struct ReportLog {
    pub generation: usize,
    pub best: usize,
    pub worst: usize,

    pub mean: f64,
    pub median: f64,
    pub variance: f64,
    pub std_dev: f64,

    pub diversity: f64,
}

////////////////////////////////////////////////////////////////////////////////
