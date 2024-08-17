// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::{Context, Genotype, ObjectiveValue};

// Trait ///////////////////////////////////////////////////////////////////////

/// This trait is usually implemented by enums, which represent a set of
/// rejection methods. The rejection methods are executed after crossover and
/// mutation are finished, to ensure certain features in the offspring
/// chromosomes (e.g. better than either of the parents).
///
pub trait Rejection<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>> {
    fn exec<'a>(
        &self,
        parent_0: &'a(Ge, Ov),
        parent_1: &'a(Ge, Ov),
        child_0: &'a(Ge, Ov),
        child_1: &'a(Ge, Ov),
        context: &Ctx
    ) -> (&'a(Ge, Ov), &'a(Ge, Ov));
}

// Implementation //////////////////////////////////////////////////////////////
pub enum Reject {
    None,
    BetterThanWorstParent,
}

impl<Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>> Rejection<Ov, Ctx, Ge> for Reject {
    fn exec<'a>(
        &self,
        parent_0: &'a (Ge, Ov),
        parent_1: &'a (Ge, Ov),
        child_0: &'a (Ge, Ov),
        child_1: &'a (Ge, Ov),
        _context: &Ctx
    ) -> (&'a (Ge, Ov), &'a (Ge, Ov)) {
        match self {
            Self::None => (child_0, child_1),
            Self::BetterThanWorstParent => better_than_worst_parent(parent_0, parent_1, child_0, child_1),
        }
    }
}


// Functions ///////////////////////////////////////////////////////////////////

fn better_than_worst_parent<'a, Ov: ObjectiveValue, Ctx: Context, Ge: Genotype<Ctx>>(
    parent0: &'a (Ge, Ov),
    parent1: &'a (Ge, Ov),
    child0: &'a (Ge, Ov),
    child1: &'a (Ge, Ov),
) -> (&'a (Ge, Ov),&'a (Ge, Ov)) {
    let better;
    let worse;
    if parent0.1 < parent1.1 {
        better = parent0;
        worse = parent1;
    } else {
        better = parent1;
        worse = parent0;
    }

    let offspring0 = if child0.1 < worse.1 { child0 } else { better };

    let offspring1 = if child1.1 < worse.1 { child1 } else { better };

    (offspring0, offspring1)
}

////////////////////////////////////////////////////////////////////////////////
