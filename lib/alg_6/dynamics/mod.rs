// Imports /////////////////////////////////////////////////////////////////////
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
    runtime_data::RuntimeData,
};
use simple_moving_average::SMA;

use crate::{
    encoding::{Chromosome, Context, Cost},
    operators::{Crossover, Mutation},
};

// #[cfg(feature = "ga_log_dynamics")]
use ga::tools::rerun_logger::RerunLogger;

// Dynamic Enum ////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Dynamic {
    /// Parameters:
    /// 1) f32  target success rate
    /// 2) f32  factor `k` in `default_std_deviation + k * (success difference)`
    /// 3) f32  default standard deviation
    SuccessDrivenBetaDistrStdDeviation(f32, f32, f32),

    /// Parameters:
    /// 1) f32  target success rate
    /// 2) f32  factor `k` in `default_std_deviation + k * (success difference)`
    /// 3) f32  default standard deviation
    SuccessDrivenNormalDistrStdDeviation(f32, f32, f32),

    /// Variable mutation rate in form of `cos`.
    /// Parameters:
    /// 1) f32      default mutation rate
    /// 2) f32      amplitude factor    : a in `a * sin(k*x)`
    /// 3) f32      wavelength factor   : k in `a * sin(k*x)`
    VariableMutationRateCos(f32, f32, f32),

    /// Variable mutation rate in form of `cos`.
    /// Parameters:
    /// 1) f32      default population size
    /// 2) f32      amplitude factor    : a in `a * sin(k*x)`
    /// 3) f32      wavelength factor   : k in `a * sin(k*x)`
    VariablePopulationSizeCos(usize, f32, f32),
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
            Dynamic::SuccessDrivenBetaDistrStdDeviation(
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
                context.rand_time_std_deviation = *def_std_deviation;
                for distr in &mut context.rand_time {
                    distr.set_std_deviation(*def_std_deviation)
                }
            }

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
                context.rand_time_std_deviation = *def_std_deviation;
                for distr in &mut context.rand_time {
                    distr.set_std_deviation(*def_std_deviation);
                }
            }

            Dynamic::VariableMutationRateCos(_, _, _) => {}
            Dynamic::VariablePopulationSizeCos(_, _, _) => {}
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
            Dynamic::SuccessDrivenBetaDistrStdDeviation(
                target_success_rate,
                k,
                _def_std_deviation,
            ) => {
                success_driven_beta_distr_std_deviation(
                    rtd,
                    context,
                    *target_success_rate,
                    *k,
                    #[cfg(feature = "ga_log_dynamics")]
                    rerun_logger,
                );
            }

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

            Dynamic::VariableMutationRateCos(reference, a, k) => {
                variable_mutation_rate_cos(
                    rtd,
                    parameters,
                    context,
                    *reference,
                    *a,
                    *k,
                    #[cfg(feature = "ga_log_dynamics")]
                    rerun_logger,
                );
            }

            Dynamic::VariablePopulationSizeCos(reference, a, k) => {
                variable_population_size_cos(
                    rtd,
                    parameters,
                    context,
                    *reference,
                    *a,
                    *k,
                    #[cfg(feature = "ga_log_dynamics")]
                    rerun_logger,
                )
            }
        }
    }
}

// Helper Functions ////////////////////////////////////////////////////////////
fn success_driven_beta_distr_std_deviation(
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

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    // Calculate the difference from the targeted success rate
    let success_rate_diff = rtd.success_rate_pt1 - target_success_rate;

    // Calculate new standard deviation
    let std_dev = (context.rand_time_std_deviation
        + (k * -1. * success_rate_diff))
        .clamp(0.001, 0.25);

    // Apply the standard deviation to the random number generators in the
    // context
    for beta_distr in &mut context.rand_time {
        beta_distr.set_std_deviation(std_dev);
    }

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_std_deviation(rtd.generation, std_dev);
    };
}

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
    def_std_deviation: f32,

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    if rtd.success_rate_pt1 < target_success_rate {
        // Calc diff (this is always positive because of the if condition)
        let diff = target_success_rate - rtd.success_rate_pt1;

        // Multiply factor to get the summand
        let summand = k * diff;

        // Add the summand to the mutation's standard deviation
        context.rand_time_std_deviation += summand;

        // Apply the standard deviation to the random number generators in the
        // context
        for d in &mut context.rand_time {
            d.set_std_deviation(context.rand_time_std_deviation);
        }

        // Reset the standard deviation if it passes a certain threshold
        if context.rand_time_std_deviation > context.num_times as f32 * 2. {
            context.rand_time_std_deviation = def_std_deviation;
            for d in &mut context.rand_time {
                d.set_std_deviation(context.rand_time_std_deviation);
            }
        }
    }

    if rtd.success {
        context.rand_time_std_deviation = 1.;
        for d in &mut context.rand_time {
            d.set_std_deviation(context.rand_time_std_deviation);
        }

    }

    // // Calculate the difference from the targeted success rate
    // let success_rate_diff = rtd.success_rate_pt1 - target_success_rate;

    // // Calculate new standard deviation
    // let std_dev = (context.rand_time_std_deviation
    //     + (k * -1. * success_rate_diff))
    //     .clamp(1., context.num_events as f32);

    // // Apply the standard deviation to the random number generators in the
    // // context
    // for dist in &mut context.rand_time {
    //     dist.set_std_deviation(std_dev);
    // }

    // // Update std deviation on context
    // context.rand_time_std_deviation = std_dev;

    let std_dev = context.rand_time_std_deviation;

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_std_deviation(rtd.generation, std_dev);
    };
}

fn variable_mutation_rate_cos(
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
    let mutation_rate =
        (reference + a + (-a * (k * x).cos())).clamp(0.001, 0.999);

    // Set mutation rate
    parameters.mutation_rate = mutation_rate;

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_rate(rtd.generation, mutation_rate);
    };
}

fn variable_population_size_cos(
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

    reference: usize,
    a: f32,
    k: f32,

    #[cfg(feature = "ga_log_dynamics")] _rerun_logger: &RerunLogger,
) {
    // Get generation number
    let x = rtd.generation as f32;

    // Calculate mutation rate
    let population_size = reference as f32 + a + (-a * (k * x).cos());

    // Set mutation rate
    parameters.population_size = population_size.round() as usize;
}

////////////////////////////////////////////////////////////////////////////////
