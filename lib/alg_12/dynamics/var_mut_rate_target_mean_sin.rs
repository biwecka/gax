// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Chromosome, Context, Cost},
    operators::{Crossover, Mutation},
};
use control_circuits::PT2;
use ga::{
    encoding::ObjectiveValue,
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
    _parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,

    gain: f64,          // gain of the pt2 block (=amplification)
) {
    context.pt2 = PT2::new(1., 1., 0.4, gain, 1.);
}

#[rustfmt::skip]
pub fn exec(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>,>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,
    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,

    avg_deviation: f64, // average deviation from current mean
    a: f32,             // amplitude of the sine function
    w: usize,           // wavelength of the sine function (in generations)
) {
    // Calculate the currently targeted deviation (based on the sine function).
    let wavelength = 2. * std::f32::consts::PI / (w as f32);
    let cos_val = a * (wavelength * rtd.generation as f32).sin();

    // Add this value to the average deviation and ensure that the minimum
    // deviation (minimum target mean) is 1% from the current best.
    let deviation = (avg_deviation + cos_val as f64).min(1.01);


    // Get current best and calculate target mean
    let current_best: usize = rtd.best.to_usize();
    let target = current_best as f64 * deviation;

    // Calculate control error
    let error = target - rtd.mean;

    // Update PT2 with error
    context.pt2.update(error);

    // Update mutation rate
    parameters.mutation_rate += context.pt2.get_output() as f32;
    parameters.mutation_rate = parameters.mutation_rate.clamp(0.000_1, 0.05);

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger
            .log_mutation_rate(rtd.generation, parameters.mutation_rate);
    };
}

////////////////////////////////////////////////////////////////////////////////
