// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::constraints::CostFunction as ICostFunction;

// Cost Function ///////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub enum CostFunction {
    Linear,
    Quadratic,
    Step,
}

impl CostFunction {
    pub fn calc(&self, value: usize) -> usize {
        match self {
            CostFunction::Linear => value,
            CostFunction::Quadratic => value * value,
            CostFunction::Step => {
                if value != 0 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl From<ICostFunction> for CostFunction {
    fn from(value: ICostFunction) -> Self {
        match value {
            ICostFunction::Linear => Self::Linear,
            ICostFunction::Quadratic => Self::Quadratic,
            ICostFunction::Step => Self::Step,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
