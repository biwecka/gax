// Imports /////////////////////////////////////////////////////////////////////
use xhstt::db::constraints::{
    // assign_time_constraint::AssignTimeConstraint,
    avoid_clashes_constraint::AvoidClashesConstraint,
    Constraint,
};

use super::{Context, Phenotype};

// Functions ///////////////////////////////////////////////////////////////////

/// Pre-calculate target indices of constraints. This removes the need of
/// resolving what a constraint applies to on every iteration.
pub fn pre_calc(db: &xhstt::db::Database) -> Vec<(Constraint, Vec<usize>)> {
    let mut result = vec![];

    for c in db.contraints() {
        let indices = match c {
            Constraint::AssignTimeConstraint(x) => {
                x.applies_to.resolve_idxs(db)
            }
            Constraint::AvoidClashesConstraint(x) => {
                x.applies_to.resolve_idxs(db)
            }
            _ => continue,
        };

        result.push((c.clone(), indices));
    }

    result
}

// Constraint Cost Functions ///////////////////////////////////////////////////

pub fn avoid_clashes_constraint(
    phenotype: &Phenotype,
    ctx: &Context,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize],
) -> usize {
    // Deviation
    let deviation: usize = resource_idxs
        .iter()
        .map(|resource_idx| phenotype.calc_clashes(*resource_idx, ctx))
        .sum();

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

////////////////////////////////////////////////////////////////////////////////
