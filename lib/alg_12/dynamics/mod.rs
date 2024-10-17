// Modules /////////////////////////////////////////////////////////////////////
mod mut_rate_cos;
mod gauss_rand_event;
mod var_mut_rate_target_mean_sin;

// Imports /////////////////////////////////////////////////////////////////////
use crate::{encoding::{Chromosome, Context, Cost}, operators::{Crossover, Mutation}};
use ga::{process::{rejection::Reject, replacement::Replace, selection::Select, termination::Terminate}, rerun::external::arrow2::array::DaysMsArray};

// Dynamic Enum ////////////////////////////////////////////////////////////////
pub enum Dynamic {
    /// Variable mutation rate in form of a cosine function.  
    ///
    /// Parameters:
    /// 1) default mutation rate (y-axis shift)
    /// 2) amplitude (maximum change of the mutation rate in both directions)
    /// 3) wavelength (in generations)
    /// 4) optional min & max values (for a hard cutoff)
    MutRateCos(f32, f32, usize, Option<(f32, f32)>),

    /// Variable standard distribution (std_dev) for the normal distribution in
    /// the `Context` struct (-> `ctx.gauss_rand_time`).
    /// This can be used in combination with `Mutation::GaussSwap`.  
    ///
    /// Parameters:
    /// 1) target success rate
    GaussRandEvent(f32),

    /// Variable mutation rate which is chosen dynamically to reach the given
    /// variable target mean objective value in the population. The target mean
    /// is variable in form of a cosine function.
    ///
    /// Parameters
    /// 1) deviation from the current best (e.g. 1.10 for +10%)
    /// 2) gain of the PT2 block (= amplification)
    VarMutRateTargetMeanSin(f64, f64),
}

#[rustfmt::skip]
impl ga::dynamics::Dynamic<Cost, Context, Chromosome, Crossover, Mutation, usize, Select, Reject, Replace, Terminate<Cost>> for Dynamic {
    fn identifier(&self) -> String {
        match self {
            Self::MutRateCos(a, b, c, d) =>
                format!("mut-rate-cos-{a}-{b}-{c}-{d:?}"),

            Self::GaussRandEvent(x) =>
                format!("gauss-rnd-evnt-{x}"),

            Self::VarMutRateTargetMeanSin(a, b) =>
                format!("var-mut-rate-target-mean-sin-{a}-{b}"),
        }
    }

    fn setup(
        &self,
        rtd: &mut ga::runtime_data::RuntimeData<Cost, Context, Chromosome, Crossover, Mutation, usize, Select, Reject, Replace, Terminate<Cost>>,
        parameters: &mut ga::parameters::Parameters<Cost, Context, Chromosome, Crossover, Mutation, usize, Select, Reject, Replace, Terminate<Cost>>,
        context: &mut Context,
    ) {
        match self {
            Self::MutRateCos(dmr, _, _, _) =>
                mut_rate_cos::setup(rtd, parameters, context, *dmr),

            Self::GaussRandEvent(_) =>
                gauss_rand_event::setup(rtd, parameters, context),

            Self::VarMutRateTargetMeanSin(_, _) =>
                var_mut_rate_target_mean_sin::setup(rtd, parameters, context),
        }
    }

    fn exec(
        &self,
        rtd: &ga::runtime_data::RuntimeData<Cost, Context, Chromosome, Crossover, Mutation, usize, Select, Reject, Replace, Terminate<Cost>>,
        parameters: &mut ga::parameters::Parameters<Cost, Context, Chromosome, Crossover, Mutation, usize, Select, Reject, Replace, Terminate<Cost>>,
        context: &mut Context,
        #[cfg(feature = "ga_log_dynamics")] rerun_logger: &ga::tools::rerun_logger::RerunLogger,
    ) {
        match self {
            Self::MutRateCos(dmr, a, w, mm) => mut_rate_cos::exec(
                rtd, parameters, context, rerun_logger, *dmr, *a, *w, *mm
            ),

            Self::GaussRandEvent(tsr) => gauss_rand_event::exec(
                rtd, parameters, context, rerun_logger, *tsr
            ),

            Self::VarMutRateTargetMeanSin(_, _) =>
                var_mut_rate_target_mean_sin::exec(
                    rtd, parameters, context, rerun_logger
                ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
