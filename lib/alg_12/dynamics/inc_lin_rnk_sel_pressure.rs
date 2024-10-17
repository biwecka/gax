// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Chromosome, Context, Cost},
    operators::{Crossover, Mutation},
};
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
    runtime_data::RuntimeData,
};

#[cfg(feature = "ga_log_dynamics")]
use ga::tools::rerun_logger::RerunLogger;

// Functions ///////////////////////////////////////////////////////////////////
#[rustfmt::skip]
pub fn setup(
    _rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    _context: &mut Context,
) {
    parameters.selection = Select::LinearRank(1.0);
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub fn exec(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>,>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    _context: &mut Context,

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,

    step_gen: usize,
    step_val: f32,
    max_sp: f32,
    reset: usize,
) {
    if let Select::LinearRank(sp) = &mut parameters.selection {
        // Calculate for how many generations there was no improvement
        let no_improv = rtd.generation - rtd.last_success;

        if  no_improv != 0 &&
            no_improv % step_gen == 0 &&
            *sp <= max_sp - 0.001
        {
            *sp += step_val;
        }

        if  *sp >= (max_sp - 0.001) &&
            no_improv > reset
        {
            *sp = 1.0;
        }

        #[cfg(feature = "ga_log_dynamics")]
        {
            rerun_logger.log_mutation_rate(rtd.generation, *sp);
        };
    }
}

////////////////////////////////////////////////////////////////////////////////
