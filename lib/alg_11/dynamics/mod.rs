// Imports /////////////////////////////////////////////////////////////////////
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
    runtime_data::RuntimeData,
};
// use rand_distr::Normal;
// use simple_moving_average::SMA;

use crate::{
    encoding::{Chromosome, Context, Cost},
    operators::{Crossover, Mutation},
};

#[cfg(feature = "ga_log_dynamics")]
use ga::tools::rerun_logger::RerunLogger;

// Dynamic Enum ////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Dynamic {
    /// Variable mutation rate in form of `cos`.
    /// Parameters:
    /// 1) f32      default mutation rate
    /// 2) f32      amplitude factor    : a in `a * sin(k*x)`
    /// 3) f32      wavelength factor   : k in `a * sin(k*x)`
    MutationRateCos(f32, f32, f32),
}

impl
    ga::dynamics::Dynamic<
        Cost,
        Context,
        Chromosome,
        Crossover,
        Mutation,
        usize,
        Select,
        Reject,
        Replace,
        Terminate<Cost>,
    > for Dynamic
{
    fn setup(
        &self,
        // Output
        _rtd: &mut RuntimeData<
            Cost,
            Context,
            Chromosome,
            Crossover,
            Mutation,
            usize,
            Select,
            Reject,
            Replace,
            Terminate<Cost>,
        >,
        _parameters: &mut ga::parameters::Parameters<
            Cost,
            Context,
            Chromosome,
            Crossover,
            Mutation,
            usize,
            Select,
            Reject,
            Replace,
            Terminate<Cost>,
        >,
        _context: &mut Context,
    ) {
        match self {
            Dynamic::MutationRateCos(_, _, _) => {}
        }
    }

    fn exec(
        &self,
        // Input
        rtd: &RuntimeData<
            Cost,
            Context,
            Chromosome,
            Crossover,
            Mutation,
            usize,
            Select,
            Reject,
            Replace,
            Terminate<Cost>,
        >,

        // "Output"
        parameters: &mut ga::parameters::Parameters<
            Cost,
            Context,
            Chromosome,
            Crossover,
            Mutation,
            usize,
            Select,
            Reject,
            Replace,
            Terminate<Cost>,
        >,
        context: &mut Context,

        #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
    ) {
        match self {
            Dynamic::MutationRateCos(
                target_success_rate,
                k,
                def_std_deviation,
            ) => {
                mutation_rate_cos(
                    rtd,
                    parameters,
                    context,
                    *target_success_rate,
                    *k,
                    *def_std_deviation,
                    #[cfg(feature = "ga_log_dynamics")]
                    rerun_logger,
                );
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
fn mutation_rate_cos(
    rtd: &RuntimeData<
        Cost,
        Context,
        Chromosome,
        Crossover,
        Mutation,
        usize,
        Select,
        Reject,
        Replace,
        Terminate<Cost>,
    >,

    parameters: &mut ga::parameters::Parameters<
        Cost,
        Context,
        Chromosome,
        Crossover,
        Mutation,
        usize,
        Select,
        Reject,
        Replace,
        Terminate<Cost>,
    >,

    _context: &mut Context,

    reference: f32,
    a: f32,
    k: f32,

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    // Get generation number
    let x = rtd.generation as f32;

    // Calculate mutation rate
    // let mutation_rate = (reference + (a * (k * x).cos())).clamp(0.001, 0.999);
    let mutation_rate = (reference + (a * (k * x).sin())).clamp(0.001, 0.1);

    // Set mutation rate
    parameters.mutation_rate = mutation_rate;

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_rate(rtd.generation, mutation_rate);
    };
}

////////////////////////////////////////////////////////////////////////////////
