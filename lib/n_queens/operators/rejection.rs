// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Cost, Genotype};

// Enum ////////////////////////////////////////////////////////////////////////
pub enum Rejection {
    None,

    // Bias should be configured in the rejection method
}

impl Rejection {
    pub fn exec<'a>(
        &self,
        parent0: (Genotype, Cost),
        parent1: (Genotype, Cost),
        child0: (Genotype, Cost),
        child1: (Genotype, Cost),
        //context: &Context,
    ) -> ((Genotype, Cost), (Genotype, Cost)) {
        match self {
            Self::None => (child0, child1)
        }
    }
}

// Implementations /////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////
