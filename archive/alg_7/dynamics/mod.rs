// Imports /////////////////////////////////////////////////////////////////////
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
    runtime_data::RuntimeData,
};
use rand_distr::Normal;
use simple_moving_average::SMA;

use crate::{
    encoding::{Chromosome, Context, Cost},
    operators::{Crossover, Mutation},
};

#[cfg(feature = "ga_log_dynamics")]
use ga::tools::rerun_logger::RerunLogger;

// Dynamic Enum ////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Dynamic {
    /// Parameters:
    /// 1) f32  target success rate
    /// 2) f32  factor `k` in `default_std_deviation + k * (success difference)`
    /// 3) f32  default standard deviation
    SuccessDrivenNormalDistrStdDeviation(f32, f32, f32),
    // / Variable mutation rate in form of `cos`.
    // / Parameters:
    // / 1) f32      default mutation rate
    // / 2) f32      amplitude factor    : a in `a * sin(k*x)`
    // / 3) f32      wavelength factor   : k in `a * sin(k*x)`
    // VariableMutationRateCos(f32, f32, f32),

    // / Variable mutation rate in form of `cos`.
    // / Parameters:
    // / 1) f32      default population size
    // / 2) f32      amplitude factor    : a in `a * sin(k*x)`
    // / 3) f32      wavelength factor   : k in `a * sin(k*x)`
    // VariablePopulationSizeCos(usize, f32, f32),
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
    fn identifier(&self) -> String {
        todo!()
    }

    fn setup(
        &self,
        // Output
        rtd: &mut RuntimeData<
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
        context: &mut Context,
    ) {
        match self {
            Dynamic::SuccessDrivenNormalDistrStdDeviation(
                target_success_rate,
                _k,
                def_std_deviation,
            ) => {
                // Initialize success rate (pt1)
                rtd.success_rate_pt1 = *target_success_rate;

                // Initialize success rate (sma)
                for _ in 0..1_000 {
                    rtd.success_rate_sma.add_sample(*target_success_rate);
                }

                // Apply default standard deviation
                context.std_deviation = *def_std_deviation;
                context.rand_event =
                    Normal::<f32>::new(0., *def_std_deviation).unwrap();
            }
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
        context: &mut Context,

        #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
    ) {
        match self {
            Dynamic::SuccessDrivenNormalDistrStdDeviation(
                target_success_rate,
                k,
                def_std_deviation,
            ) => {
                success_driven_normal_distr_std_deviation(
                    rtd,
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

fn success_driven_normal_distr_std_deviation(
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

    context: &mut Context,

    target_success_rate: f32,
    k: f32,
    default_sd: f32,

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    if rtd.success_rate_pt1 < target_success_rate {
        // Calc diff (this is always positive because of the if condition)
        let diff = target_success_rate - rtd.success_rate_pt1;

        // Multiply factor to get the summand
        let summand = k * diff;

        // Add the summand to the mutation's standard deviation
        context.std_deviation += summand;

        // Apply the standard deviation to the random number generators in the
        // context
        context.rand_event =
            Normal::<f32>::new(0., context.std_deviation).unwrap();

        // Reset the standard deviation if it passes a certain threshold
        if context.std_deviation > context.num_events as f32 * 2. {
            context.std_deviation = default_sd;
        }
    }

    // Reset the standard deviation every time the overall best solution
    // was improved.
    if rtd.success {
        context.std_deviation = default_sd;
    }

    // // Calc success rate diff
    // let success_rate_diff = rtd.success_rate_pt1 - target_success_rate;

    // // Multiply factor
    // let addition =

    // // Calculate new standard deviation
    // let mut std_dev = context.std_deviation + (k * -1. * success_rate_diff);
    // std_dev = std_dev.clamp(1., context.num_events as f32 * 4.);

    // // Apply the standard deviation to the random number generators in the
    // // context
    // context.rand_event = Normal::<f32>::new(0., std_dev).unwrap();

    // // Update std deviation on context
    // // context.std_deviation = std_dev;

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger
            .log_mutation_std_deviation(rtd.generation, context.std_deviation);
    };
}

////////////////////////////////////////////////////////////////////////////////
