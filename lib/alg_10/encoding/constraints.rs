// Imports /////////////////////////////////////////////////////////////////////
use super::{Context, Phenotype};
use itertools::Itertools;
use xhstt::db::constraints::{
    assign_time_constraint::AssignTimeConstraint,
    avoid_clashes_constraint::AvoidClashesConstraint, Constraint,
};

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
pub fn assign_time(
    phenotype: &Phenotype,
    ctx: &Context,
    params: &AssignTimeConstraint,
    event_idxs: &[usize], // E_spec
) -> usize {
    let deviation: usize = event_idxs
        .iter()
        .map(|event_idx| {
            // Get event gene
            let eg = &phenotype.times[*event_idx];

            // Count allocations
            let sum = eg.times.count_ones();

            // Get event duration
            let duration = ctx.durations[*event_idx] as usize;

            duration - sum
        })
        .sum();

    // Return
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

    // Iterate over all resources
    for res_idx in resource_idxs {
        // Get the indices of events, that are need this resource
        let event_idxs: Vec<usize> =
            phenotype.resources[*res_idx].iter_ones().collect();

        for x in event_idxs.iter().combinations(2) {
            let e1 = *x[0];
            let e2 = *x[1];

            // Get time allocations of the two events
            let y_e1 = phenotype.times[e1].times.clone();
            let y_e2 = phenotype.times[e2].times.clone();

            // Combine time allocations with `and`
            let result = y_e1 & y_e2;

            // Ones in the result vector mark collisions. Count the ones and
            // add it to the deviation
            deviation += result.count_ones();
        }
    }

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

////////////////////////////////////////////////////////////////////////////////
