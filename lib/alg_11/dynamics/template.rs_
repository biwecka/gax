// Imports /////////////////////////////////////////////////////////////////////
use crate::{encoding::{Chromosome, Context, Cost}, operators::{Crossover, Mutation}};
use ga::{process::{rejection::Reject, replacement::Replace, selection::Select, termination::Terminate}, rerun::external::arrow2::array::DaysMsArray, runtime_data::RuntimeData, tools::rerun_logger::RerunLogger};

// Functions ///////////////////////////////////////////////////////////////////
#[rustfmt::skip]
pub fn setup(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,
) {}

#[rustfmt::skip]
pub fn exec(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>,>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,
    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {

}

////////////////////////////////////////////////////////////////////////////////
