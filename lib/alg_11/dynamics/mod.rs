// Imports /////////////////////////////////////////////////////////////////////
use control_circuits::PT2;
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
    encoding::{Chromosome, Context, Cost, State},
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

    /// Normal distributed random event when using `ctx.gauss_rand_event` number
    /// generator. Parameters:
    /// 1) f32      target success rate
    GaussRandomEvent(f32),

    /// Variable mutation rate which is controlled by the current mean objective
    /// value and how it compares to a given target mean objective value.
    /// The target mean objective value is given as a deviation (in percent)
    /// from the current best objective value.
    ///
    /// Parameters:
    /// 1) Deviation from current best (e.g. 1.10 for +10%)
    /// 2) Gain of the PT2 block (= amplification)
    TargetMeanByVariableMutationRate(f64, f64),

    /// Increasing linear rank selection pressure
    IncreasingLinearRankSelectionPressure,

    /// RotatingMutationMethods
    RotatingMutationMethods,

    /// State Machine
    StateMachine,
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
        context: &mut Context,
    ) {
        match self {
            Self::MutationRateCos(_, _, _) => {}
            Self::GaussRandomTime(_) => {}
            Self::GaussRandomEvent(_) => {}
            Self::TargetMeanByVariableMutationRate(_, gain) => {
                context.pt2 = PT2::new(1., 1., 0.4, *gain, 1.);
            }
            Self::IncreasingLinearRankSelectionPressure => {}
            Self::RotatingMutationMethods => {}
            Self::StateMachine => {}
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
            }

            Self::GaussRandomTime(tsr) => {
                gauss_random_time(*tsr, rtd, parameters, context, rerun_logger);
            }

            Self::GaussRandomEvent(tsr) => {
                gauss_random_event(
                    *tsr,
                    rtd,
                    parameters,
                    context,
                    rerun_logger,
                );
            }

            Self::TargetMeanByVariableMutationRate(target_mean, _) => {
                target_mean_by_variable_mutation_rate(
                    *target_mean,
                    rtd,
                    parameters,
                    context,
                    rerun_logger,
                );
            }

            Self::IncreasingLinearRankSelectionPressure => {
                increasing_linear_rank_selection_pressure(
                    rtd,
                    parameters,
                    context,
                    rerun_logger,
                );
            }

            Self::RotatingMutationMethods => {
                rotating_mutation_methods(
                    rtd,
                    parameters,
                    context,
                    rerun_logger,
                );
            }

            Self::StateMachine => {
                state_machine(rtd, parameters, context, rerun_logger);
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
    if rtd.success_rate_pt1 < target_success_rate {
        // Calc diff (this is always positive because of the `if` condition)
        let diff = target_success_rate - rtd.success_rate_pt1;

        // Multiply factor to get the summand
        let summand = 2. * diff; // * (0.1 * diff);

        // Add the summand to the mutation's standard deviation
        context.gauss_rand_time_sd += summand;

        // Apply the standard deviation to the random number generator in the
        // context.
        context.gauss_rand_time =
            Normal::<f32>::new(0., context.gauss_rand_time_sd).unwrap();

        // Reset the standard deviation if it passes a certain threshold.
        if context.gauss_rand_time_sd > context.num_times as f32 * 1.4 {
            context.gauss_rand_time_sd = 1.;
            context.gauss_rand_time =
                Normal::<f32>::new(0., context.gauss_rand_time_sd).unwrap();
        }
    }

    // Reset the standard deviation every time the overall best solution was
    // improved.
    if rtd.success {
        context.gauss_rand_time_sd = 1.;
        Normal::<f32>::new(0., context.gauss_rand_time_sd).unwrap();
    }

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger.log_mutation_std_deviation(
            rtd.generation,
            context.gauss_rand_time_sd,
        );
    };
}

fn gauss_random_event(
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
    if rtd.success_rate_pt1 < target_success_rate {
        // Calc diff (this is always positive because of the `if` condition)
        let diff = target_success_rate - rtd.success_rate_pt1;

        // Multiply factor to get the summand
        let summand = 2. * diff; // * (0.1 * diff);

        // Add the summand to the mutation's standard deviation
        context.gauss_rand_event_sd += summand;

        // Apply the standard deviation to the random number generator in the
        // context.
        context.gauss_rand_event =
            Normal::<f32>::new(0., context.gauss_rand_event_sd).unwrap();

        // Reset the standard deviation if it passes a certain threshold.
        if context.gauss_rand_event_sd > context.num_events as f32 * 1.4 {
            context.gauss_rand_event_sd = 1.;
            context.gauss_rand_event =
                Normal::<f32>::new(0., context.gauss_rand_event_sd).unwrap();
        }
    }

    // Reset the standard deviation every time the overall best solution was
    // improved.
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

fn target_mean_by_variable_mutation_rate(
    _target_mean_deviation: f64,

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
    let min_deviation = 1.08;
    let max_deviation = 4.00;

    let cos = (0.000_100 * rtd.generation as f64).cos().powi(2);

    let target_mean_deviation =
        (max_deviation - min_deviation) * cos + min_deviation;

    // Calculate the target mean objective value
    let current_best: usize = rtd.current_best.clone().into();
    let target = current_best as f64 * target_mean_deviation;

    // Calculate control error: target - current mean
    let error = target - rtd.current_mean as f64;

    // Update PT2 with error
    context.pt2.update(error);

    // Update mutation rate
    parameters.mutation_rate += context.pt2.get_output() as f32;

    parameters.mutation_rate = parameters.mutation_rate.clamp(0.000_565, 0.05); //0.017);

    #[cfg(feature = "ga_log_dynamics")]
    {
        rerun_logger
            .log_mutation_rate(rtd.generation, parameters.mutation_rate);
    };
}

fn increasing_linear_rank_selection_pressure(
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

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    // This dynamic obviously only works if the "LinearRank" selection method
    // is used. Therefore, the selection method is checked and the selection
    // pressure is extracted as mutable reference.
    if let Select::LinearRank(selection_pressure) = &mut parameters.selection {
        let no_improvement = rtd.generation - rtd.last_success;

        if no_improvement != 0 && no_improvement % 100 == 0 {
            if *selection_pressure <= 2.0 - 0.01 {
                *selection_pressure += 0.01;
            }
        }

        if *selection_pressure >= 1.999 && no_improvement > 5_000 {
            *selection_pressure = 1.0;
        }

        #[cfg(feature = "ga_log_dynamics")]
        {
            rerun_logger.log_mutation_rate(rtd.generation, *selection_pressure);
        };
    }
}

fn rotating_mutation_methods(
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

    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    if rtd.generation % 1_000 != 0 {
        return;
    }

    // match parameters.mutation {
    //     Mutation::Trade => {
    //         parameters.mutation = Mutation::MoveSubEvent;
    //     },
    //     Mutation::MoveSubEvent => {
    //         parameters.mutation = Mutation::MoveSingleTimeAlloc;
    //     },
    //     Mutation::MoveSingleTimeAlloc => {
    //         parameters.mutation = Mutation::Trade;
    //     },
    //     _ => {}
    // }

    match parameters.mutation {
        Mutation::GaussMoveSingleTimeAlloc => {
            parameters.mutation = Mutation::GaussTrade;
        }

        Mutation::GaussTrade => {
            parameters.mutation = Mutation::GaussMoveSingleTimeAlloc;
        }

        _ => {}
    }
}

fn state_machine(
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
    // Update counter in state
    // if rtd.success {
    // context.state_machine.last_state_change = 0;

    // } else {
    context.state_machine.last_state_change += 1;
    // }

    // State changes
    match context.state_machine.state {
        State::Broad => {
            if context.state_machine.last_state_change > 50 {
                parameters.replacement = Replace::EliteAbsolute(1);
            }

            // After 500 generations, switch to focus state
            if context.state_machine.last_state_change > 200 {
                context.state_machine.last_state_change = 0;

                context.state_machine.state = State::Focus;

                parameters.selection = Select::LinearRank(1.6);
                parameters.crossover = Crossover::Trade(1);
                parameters.mutation = Mutation::Trade;
                parameters.mutation_rate = 0.01;

                #[cfg(feature = "ga_log_dynamics")]
                {
                    rerun_logger.log_text(
                        rtd.generation,
                        &format!("{:?}", context.state_machine),
                    );
                };
            }
        }

        State::Focus => {
            // After 5_000 generations, switch to focus state
            if context.state_machine.last_state_change > 1_800 {
                context.state_machine.last_state_change = 0;

                context.state_machine.state = State::Finish;

                parameters.selection = Select::LinearRank(2.5);
                parameters.crossover = Crossover::Trade(1);
                parameters.mutation = Mutation::Trade;
                parameters.mutation_rate = 0.0056;

                #[cfg(feature = "ga_log_dynamics")]
                {
                    rerun_logger.log_text(
                        rtd.generation,
                        &format!("{:?}", context.state_machine),
                    );
                };
            }
        }

        State::Finish => {
            // After 3_000 generations, switch to focus state
            if context.state_machine.last_state_change > 3_000 {
                context.state_machine.last_state_change = 0;

                context.state_machine.state = State::Broad;

                parameters.selection = Select::LinearRank(1.4);
                parameters.crossover = Crossover::Trade(1);
                parameters.mutation = Mutation::Trade;
                parameters.mutation_rate = 0.015;

                // if let Replace::EliteAbsolute(elite) = parameters.replacement {
                //     parameters.replacement = Replace::EliteAbsolute(elite + 1);
                // }

                if context.state_machine.focus_without_success >= 4 {
                    parameters.replacement = Replace::Full;
                    context.state_machine.focus_without_success = 0;
                } else {
                    context.state_machine.focus_without_success += 1;
                }

                #[cfg(feature = "ga_log_dynamics")]
                {
                    rerun_logger.log_text(
                        rtd.generation,
                        &format!("{:?}", context.state_machine),
                    );
                };
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
