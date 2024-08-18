// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Variable single-point crossover takes one argument:
    /// 1) f32      representing the crossover rate
    VariableSinglePoint(f32),

    /// Variable n-point crossover takes two arguments:
    /// 1) f32      representing the crossover rate
    /// 2) usize    representing the amount of crossover points
    VariableNPoint(f32, usize),
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        _context: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::VariableSinglePoint(rate) => {
                let (a, b) = ga::utils::crossover::variable_single_point(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    *rate,
                );

                (a.into(), b.into())
            }

            Crossover::VariableNPoint(rate, num_points) => {
                let (a, b) = ga::utils::crossover::variable_multi_point(
                    *num_points,
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    *rate,
                );

                (a.into(), b.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
