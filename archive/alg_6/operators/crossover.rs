use rand::rngs::ThreadRng;

// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Chromosome, Context};

// Crossover ///////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Crossover {
    /// Variable single-point crossover takes one argument:
    VariableSinglePoint,

    /// Variable n-point crossover takes two arguments:
    /// 1) usize    representing the amount of crossover points
    VariableNPoint(usize),

    /// Uniform crossover. Arguments:
    Uniform,
    // PMX or Ordered Crossover cannot be used with this encoding, because
    // due to the duration of an event moving a time assignment from one
    // event to another could make the chromosome invalid, because the changed
    // time assignment and the potentially longer duration of the new event
    // it got assigned to, will lead to an "array out of bounds" error.
}

impl ga::operators::Crossover<Context, Chromosome> for Crossover {
    fn identifier(&self) -> String {
        match self {
            Self::VariableSinglePoint => "var-s-pt".into(),
            Self::VariableNPoint(n) => format!("var-{n}-pt"),
            Self::Uniform => "uni".into(),
        }
    }

    fn exec(
        &self,
        parent_0: &Chromosome,
        parent_1: &Chromosome,
        rate: Option<f32>,
        rng: &mut ThreadRng,
        _context: &Context,
    ) -> (Chromosome, Chromosome) {
        match self {
            Crossover::VariableSinglePoint => {
                let (a, b) = ga::operators::crossover::single_point(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    rate,
                    rng,
                );

                (a.into(), b.into())
            }

            Crossover::VariableNPoint(num_points) => {
                let (a, b) = ga::operators::crossover::multi_point(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    rate,
                    *num_points,
                    rng,
                );

                (a.into(), b.into())
            }

            Crossover::Uniform => {
                let (a, b) = ga::operators::crossover::uniform(
                    parent_0.as_slice(),
                    parent_1.as_slice(),
                    rate,
                    rng,
                );
                (a.into(), b.into())
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
