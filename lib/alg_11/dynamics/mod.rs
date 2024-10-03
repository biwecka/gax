// Imports /////////////////////////////////////////////////////////////////////
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
    runtime_data::RuntimeData,
};
use rand_distr::Normal;
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

    /// Normal distributed random time when using `ctx.gauss_rand_time` number
    /// generator. Parameters:
    /// 1) f32      target success rate
    GaussRandomTime(f32),
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
            Self::MutationRateCos(_, _, _) => {},
            Self::GaussRandomTime(_) => {},
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
            Self::MutationRateCos(
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
            },

            Self::GaussRandomTime(tsr) => {
                gauss_random_time(*tsr, rtd, parameters, context, rerun_logger);
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

fn gauss_random_time(
    target_success_rate: f32,

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

    context: &mut Context,

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    if rtd.success_rate_pt1 < target_success_rate {
        // Calc diff (this is always positive because of the `if` condition)
        let diff = target_success_rate - rtd.success_rate_pt1;

        // Multiply factor to get the summand
        let summand = 2. * diff; // * (0.1 * diff);

        // Add the summand to the mutation's standard deviation
        context.gauss_rand_time_sd += summand;

        // Apply the standard deviation to the random number generator in the
        // context.
        context.gauss_rand_time = Normal::<f32>::new(0., context.gauss_rand_time_sd)
            .unwrap();

        // Reset the standard deviation if it passes a certain threshold.
        if context.gauss_rand_time_sd > context.num_times as f32 * 1.4 {
            context.gauss_rand_time_sd = 1.;
            context.gauss_rand_time = Normal::<f32>::new(0., context.gauss_rand_time_sd)
            .unwrap();
        }
    }

    // Reset the standard deviation every time the overall best solution was
    // improved.
    if rtd.success {
        context.gauss_rand_time_sd = 1.;
        Normal::<f32>::new(0., context.gauss_rand_time_sd)
            .unwrap();
    }

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_std_deviation(rtd.generation, context.gauss_rand_time_sd);
    };
}

////////////////////////////////////////////////////////////////////////////////
