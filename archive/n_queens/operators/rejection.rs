// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Cost, Genotype};

// Enum ////////////////////////////////////////////////////////////////////////
#[allow(unused)]
pub enum Rejection {
    None,
    // Bias should be configured in the rejection method
    BetterThanWorseParent,
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
            Self::None => (child0, child1),
            Self::BetterThanWorseParent => {
                better_than_worse_parent(parent0, parent1, child0, child1)
            }
        }
    }
}

// Implementations /////////////////////////////////////////////////////////////
fn better_than_worse_parent(
    parent0: (Genotype, Cost),
    parent1: (Genotype, Cost),
    child0: (Genotype, Cost),
    child1: (Genotype, Cost),
) -> ((Genotype, Cost), (Genotype, Cost)) {
    let better;
    let worse;
    if parent0.1 < parent1.1 {
        better = parent0;
        worse = parent1;
    } else {
        better = parent1;
        worse = parent0;
    }

    let offspring0 = if child0.1 < worse.1 { child0 } else { better.clone() };

    let offspring1 = if child1.1 < worse.1 { child1 } else { better };

    (offspring0, offspring1)
}

////////////////////////////////////////////////////////////////////////////////
