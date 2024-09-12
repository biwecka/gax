// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::allocation::Allocation;
// use rayon::prelude::*;
use xhstt::db::constraints::{
    assign_time_constraint::AssignTimeConstraint,
    avoid_clashes_constraint::AvoidClashesConstraint, Constraint,
};

// Functions ///////////////////////////////////////////////////////////////////
/// Pre-calculate target indices of constraints. This removes the need of
/// resolving what a constraint applies to on every iteration.
pub fn pre_calc_constraints(
    constraints: &[Constraint],
    db: &xhstt::db::Database,
) -> Vec<(Constraint, Vec<usize>)> {
    let mut result = vec![];

    for c in constraints {
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

/// This function calculates the cost of an allocation under the given
/// constraints.
pub fn calculate_cost(
    allocation: &Allocation,
    constraints: &[(Constraint, Vec<usize>)],
) -> usize {
    let mut total_cost = 0;

    for (constraint, indices) in constraints {
        match constraint {
            Constraint::AssignTimeConstraint(params) => {
                total_cost +=
                    assign_time_constraint(allocation, params, indices);
            }
            Constraint::AvoidClashesConstraint(_params) => {}
            _ => {}
        }
    }

    total_cost
}

// Helper Functions ////////////////////////////////////////////////////////////
/// Calculate the cost of the assign time constraint.
#[allow(unused)]
fn assign_time_constraint(
    allocation: &Allocation,
    params: &AssignTimeConstraint,
    event_idxs: &[usize],
) -> usize {
    let mut deviation = event_idxs
        .iter()
        // .par_iter()
        .map(|event_idx| {
            // Get time allocation for event
            let times = allocation.times_by_event(*event_idx);

            let allocated_times = times
                .into_iter()
                // .into_par_iter()
                .filter(|x| *x > 0)
                .sum::<i8>();

            // If no time was allocated, a cost of "event.duration" is added to
            // the deviation.
            if allocated_times == 0 {
                allocation.event_duration(*event_idx) as usize
            } else {
                0
            }
        })
        .sum();

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

/// Calculates the cost of the AvoidClashesConstraint.
/// This constraint checks if the allocation contains clashes of resources.
/// The allocation/encoding ensures that this is never the case. Therefore,
/// no calculation is needed.
#[allow(unused)]
fn avoid_clashes_constraint(
    allocation: &Allocation,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize],
) -> usize {
    0
}

////////////////////////////////////////////////////////////////////////////////
