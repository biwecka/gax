use itertools::Itertools;
// Imports /////////////////////////////////////////////////////////////////////
// use itertools::Itertools;
use xhstt::db::constraints::{
    assign_time_constraint::AssignTimeConstraint,
    avoid_clashes_constraint::AvoidClashesConstraint, Constraint,
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

#[allow(unused)]
pub fn assign_time_constraint(
    phenotype: &Phenotype,
    ctx: &Context,
    params: &AssignTimeConstraint,
    event_idxs: &[usize], // E_spec
) -> usize {
    let deviation: usize = event_idxs
        .iter()
        .map(|event_idx| {
            // Get time allocation
            let bits = phenotype.times[*event_idx];
            let allocations = bits.ones().count();

            // Get desired duration
            let duration = ctx.durations[*event_idx] as usize;

            duration - allocations
        })
        .sum();

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

pub fn avoid_clashes(
    phenotype: &Phenotype,
    _ctx: &Context,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize], // E_spec
) -> usize {
    let mut deviation: usize = 0;

    for res_idx in resource_idxs {
        // Get the indices of events, that need this resource,
        // and iterate all combinations of 2 of them.
        for x in phenotype.resources[*res_idx].ones().combinations(2) {
            let e0 = x[0] as usize;
            let e1 = x[1] as usize;

            let res = phenotype.times[e0] & phenotype.times[e1];

            deviation += res.ones().count();
        }

        // TODO: try alternative with aggregate mask of seen values.
    }

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

////////////////////////////////////////////////////////////////////////////////
