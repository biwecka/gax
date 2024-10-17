// Imports /////////////////////////////////////////////////////////////////////
use crate::{encoding::{Chromosome, Context, Cost}, operators::{Crossover, Mutation}};
use ga::{
    process::{rejection::Reject, replacement::Replace, selection::Select, termination::Terminate},
    runtime_data::RuntimeData,
};
use rand_distr::Normal;

#[cfg(feature = "ga_log_dynamics")]
use ga::{
    tools::rerun_logger::RerunLogger,
    rerun::external::arrow2::array::DaysMsArray,
};

// Functions ///////////////////////////////////////////////////////////////////
#[rustfmt::skip]
pub fn setup(
    _rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    _parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    _context: &mut Context,
) {}

#[rustfmt::skip]
pub fn exec(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>,>,
    _parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,
    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,

    target_success_rate: f32,
) {
    if rtd.success_rate_pt1 < target_success_rate {
        // Calculate difference
        let diff = target_success_rate - rtd.success_rate_pt1;

        // Add the difference to the mutation's standard deviation
        context.gauss_rand_event_sd += diff;

        // Apply the std_dev to the random number generator in the context.
        context.gauss_rand_event =
            Normal::<f32>::new(0., context.gauss_rand_event_sd).unwrap();

        // Reset the standard deviation if it passes a certain threshold.
        if context.gauss_rand_event_sd > context.num_events as f32 * 1.5 {
            context.gauss_rand_event_sd = 1.;
            context.gauss_rand_event =
                Normal::<f32>::new(0., context.gauss_rand_event_sd).unwrap();
        }
    }

    // Reset the std_dev every time the overall best solution was improved.
    if rtd.success {
        context.gauss_rand_event_sd = 1.;
        Normal::<f32>::new(0., context.gauss_rand_event_sd).unwrap();
    }

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_std_deviation(
            rtd.generation,
            context.gauss_rand_event_sd,
        );
    };
}

////////////////////////////////////////////////////////////////////////////////
