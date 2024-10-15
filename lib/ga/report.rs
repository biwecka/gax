// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};
use std::marker::PhantomData;

// Report //////////////////////////////////////////////////////////////////////
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

    /// Total runtime of the algorithm in SECONDS.
    pub runtime: usize,

    pub parameter_identifier: String,
    pub dynamics_identifier: Option<String>,

    ctx: PhantomData<Ctx>,
}

impl<Ov, Ctx, Ge> Report<Ov, Ctx, Ge>
where
    Ov: ObjectiveValue,
    Ctx: Context,
    Ge: Genotype<Ctx>,
{
    pub fn new_with_log_capacity(log_capacity: usize) -> Self {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////
