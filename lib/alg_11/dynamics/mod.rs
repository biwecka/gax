// Modules /////////////////////////////////////////////////////////////////////
mod gauss_rand_event;
mod gauss_rand_time;
mod inc_lin_rnk_sel_pressure;
mod mut_rate_cos;
mod state_machine;
mod var_mut_rate_target_mean_sin;

// Imports /////////////////////////////////////////////////////////////////////
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate,
    },
    runtime_data::RuntimeData,
};

use crate::{
    encoding::{Chromosome, Context, Cost},
    operators::{Crossover, Mutation},
};

#[cfg(feature = "ga_log_dynamics")]
use ga::tools::rerun_logger::RerunLogger;

// Dynamic Enum ////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Dynamic {
    /// Variable mutation rate in form of a cosine function.  
    ///   
    /// Parameters:  
    /// 1) default mutation rate (y-axis shift)
    /// 2) amplitude (maximum change of the mutation rate in both directions)
    /// 3) wavelength (in generations)
    /// 4) optional min & max values (for a hard cutoff)
    MutRateCos(f32, f32, usize, Option<(f32, f32)>),

    /// Variable standard deviation (std_dev) for the normal distribution in
    /// the `Context` struct (-> `ctx.gauss_rand_time`).
    /// This can be used in combination with `Mutation::MoveSingleTimeAlloc`.
    ///   
    /// Params:  
    /// 1) target success rate
    GaussRandTime(f32),

    /// Variable standard deviation (std_dev) for the normal distribution in
    /// the `Context` struct (-> `ctx.gauss_rand_event`).
    /// This can be used in combination with `Mutation::GaussTrade`.
    ///   
    /// Params:  
    /// 1) target success rate
    GaussRandEvent(f32),

    /// Variable mutation rate which is chosen dynamically to reach the given
    /// variable target mean objective value in the population. The target mean
    /// is variable in form of a cosine function.  
    ///   
    /// Parameters:  
    /// 1) average deviation from the current best (e.g. 1.10 for +10%)
    /// 2) gain of the PT2 block (= amplification)
    /// 3) amplitude of the deviation change (sine-function)
    /// 4) wavelength (in generations)
    VarMutRateTargetMeanSin(f64, f64, f32, usize),

    /// Increase selection pressure by modifying the linear rank parameter,
    /// if for a certain amount of generations the best solution didn't change.
    /// This dynamic automatically sets the linear rank selection as selection
    /// method!  
    ///   
    /// Parameters:  
    /// 1) amount of generations to pass before another STEP is made
    /// 2) step in selection pressure value
    /// 3) max selection pressure
    /// 4) maximum amount of generations at max selection pressure
    IncLinearRankSelectionPressure(usize, f32, f32, usize),

    /// State Machine
    StateMachine,
}

#[rustfmt::skip]
impl ga::dynamics::Dynamic<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>> for Dynamic {
    fn identifier(&self) -> String {
        match self {
            Self::MutRateCos(a, b, c, d) =>
                format!("mut-rate-cos-{a}-{b}-{c}-{d:?}"),

            Self::GaussRandTime(x) =>
                format!("gauss-rnd-time-{x}"),

            Self::GaussRandEvent(x) =>
                format!("gauss-rnd-evnt-{x}"),

            Self::VarMutRateTargetMeanSin(a, b, c, d) =>
                format!("var-mut-rate-target-mean-sin-{a}-{b}-{c}-{d}"),

            Self::IncLinearRankSelectionPressure(a, b, c, d) =>
                format!("inc-lin-rnk-sel-pressure-{a}-{b}-{c}-{d}"),

            Self::StateMachine => "state-machine".into(),
        }
    }

    fn setup(
        &self,
        rtd: &mut RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
        parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
        context: &mut Context,
    ) {
        match self {
            Self::MutRateCos(dmr, _, _, _) =>
                mut_rate_cos::setup(rtd, parameters, context, *dmr),

            Self::GaussRandTime(_) =>
                gauss_rand_time::setup(rtd, parameters, context),

            Self::GaussRandEvent(_) =>
                gauss_rand_event::setup(rtd, parameters, context),

            Self::VarMutRateTargetMeanSin(_, g, _, _) =>
                var_mut_rate_target_mean_sin::setup(
                    rtd, parameters, context, *g
                ),

            Self::IncLinearRankSelectionPressure(_, _, _, _) =>
                inc_lin_rnk_sel_pressure::setup(rtd, parameters, context),

            Self::StateMachine =>
                state_machine::setup(rtd, parameters, context),
        }
    }

    fn exec(
        &self,
        rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
        parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
        context: &mut Context,
        #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
    ) {
        match self {
            Self::MutRateCos(dmr, a, w, mm) => mut_rate_cos::exec(
                rtd, parameters, context,
                #[cfg(feature = "ga_log_dynamics")] rerun_logger,
                *dmr, *a, *w, *mm
            ),

            Self::GaussRandTime(tsr) => gauss_rand_time::exec(
                rtd, parameters, context,
                #[cfg(feature = "ga_log_dynamics")] rerun_logger,
                *tsr
            ),

            Self::GaussRandEvent(tsr) => gauss_rand_event::exec(
                rtd, parameters, context,
                #[cfg(feature = "ga_log_dynamics")] rerun_logger,
                *tsr
            ),

            Self::VarMutRateTargetMeanSin(avg_dv, _, a, w) =>
                var_mut_rate_target_mean_sin::exec(
                    rtd, parameters, context,
                    #[cfg(feature = "ga_log_dynamics")] rerun_logger,
                    *avg_dv, *a, *w
                ),

            Self::IncLinearRankSelectionPressure(
                    step_gen, step_val, max_sp, reset
            ) =>
                inc_lin_rnk_sel_pressure::exec(
                    rtd, parameters, context,
                    #[cfg(feature = "ga_log_dynamics")] rerun_logger,
                    *step_gen, *step_val, *max_sp, *reset
                ),

            Self::StateMachine => state_machine::exec(
                rtd, parameters, context,
                #[cfg(feature = "ga_log_dynamics")] rerun_logger,
            ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
