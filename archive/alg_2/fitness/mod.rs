// Imports /////////////////////////////////////////////////////////////////////
use crate::encoding::allocation::Allocation;
use rayon::prelude::*;
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
            Constraint::AssignTimeConstraint(_) => {}
            Constraint::AvoidClashesConstraint(params) => {
                total_cost +=
                    avoid_clashes_constraint(allocation, params, indices);
            }
            _ => {}
        }
    }

    total_cost
}

// Helper Functions ////////////////////////////////////////////////////////////
/// Calculate the cost of the assign time constraint.
/// This constraint checks if every event has a time assigned to it.
/// The allocation/encoding ensures that this is the case. No calculation
/// needed.
#[allow(unused)]
fn assign_time_constraint(
    allocation: &Allocation,
    params: &AssignTimeConstraint,
    event_idxs: &[usize],
) -> usize {
    0
}

fn avoid_clashes_constraint(
    allocation: &Allocation,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize],
) -> usize {
    // Deviation
    let deviation: usize = resource_idxs
        .par_iter()
        .map(|resource_idx| {
            // Get events by resource
            let event_idxs = allocation.events_by_resource(*resource_idx);

            if event_idxs.len() < 2 {
                return 0;
            }

            // Get all times allocated to the events
            let times = allocation.times_by_events(&event_idxs);

            // If the times list is shorter than the event list, this means that
            // some events have the same time assigned.
            if times.len() < event_idxs.len() {
                event_idxs.len() - times.len()
            } else {
                0
            }
        })
        .sum();

    // for resource_idx in resource_idxs {
    //     // Get events by resource
    //     let event_idxs = allocation.events_by_resource(*resource_idx);

    //     if event_idxs.len() < 2 {
    //         continue;
    //     }

    //     // Get all times allocated to the events
    //     let times = allocation.times_by_events(&event_idxs);

    //     // If the times list is shorter than the event list, this means that
    //     // some events have the same time assigned.
    //     if times.len() < event_idxs.len() {
    //         deviation += event_idxs.len() - times.len();
    //     }
    // }

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

////////////////////////////////////////////////////////////////////////////////
