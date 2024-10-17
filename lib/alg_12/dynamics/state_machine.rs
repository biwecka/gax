// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    encoding::{Chromosome, Context, Cost, State},
    operators::{Crossover, Mutation}
};
use ga::{
    process::{
        rejection::Reject, replacement::Replace, selection::Select,
        termination::Terminate
    },
    runtime_data::RuntimeData, tools::rerun_logger::RerunLogger
};

// Functions ///////////////////////////////////////////////////////////////////
#[rustfmt::skip]
pub fn setup(
    _rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,
) {
    context.state_machine.state = State::Broad;

    parameters.selection = Select::LinearRank(1.4);
    parameters.crossover = Crossover::Pmx;
    parameters.mutation = Mutation::UniformSwap;
    parameters.mutation_rate = 0.015;

    parameters.replacement = Replace::Full;
}

#[rustfmt::skip]
pub fn exec(
    rtd: &RuntimeData<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>,>,
    parameters: &mut ga::parameters::Parameters<Cost,Context,Chromosome,Crossover,Mutation,usize,Select,Reject,Replace,Terminate<Cost>>,
    context: &mut Context,
    #[cfg(feature = "ga_log_dynamics")] rerun_logger: &RerunLogger,
) {
    context.state_machine.last_state_change += 1;

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
                parameters.crossover = Crossover::Ordered;
                parameters.mutation = Mutation::UniformSwap;
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
                parameters.crossover = Crossover::Ordered;
                parameters.mutation = Mutation::GaussSwap;
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

        State::Finish => {
            // After 3_000 generations, switch to focus state
            if context.state_machine.last_state_change > 3_000 {
                context.state_machine.last_state_change = 0;

                context.state_machine.state = State::Broad;

                parameters.selection = Select::LinearRank(1.4);
                parameters.crossover = Crossover::Pmx;
                parameters.mutation = Mutation::UniformSwap;
                parameters.mutation_rate = 0.015;

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
