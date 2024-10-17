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

    default_mutation_rate: f32,
) {
    // Set default mutation rate
    parameters.mutation_rate = default_mutation_rate;
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub fn exec(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>,>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    _context: &mut Context,
    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,

    default_mutation_rate: f32,
    amplitude: f32,
    wavelength: usize,
    min_max: Option<(f32, f32)>,
) {
    // Calculate the wavelength factor for the desired wavelength
    let wavelength = 2. * std::f32::consts::PI / (wavelength as f32);

    // Calculate the cosine function value
    let cos_val = amplitude * (wavelength * rtd.generation as f32).cos();

    // Apply offset
    let mut result = cos_val + default_mutation_rate;

    // Apply min/max
    if let Some((min, max)) = min_max {
        result = result.clamp(min, max);
    }

    // Apply the result as mutation rate
    parameters.mutation_rate = result;

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_rate(rtd.generation, result);
    };
}

////////////////////////////////////////////////////////////////////////////////
