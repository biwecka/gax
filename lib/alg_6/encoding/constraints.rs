// Imports /////////////////////////////////////////////////////////////////////
use xhstt::db::constraints::{
    assign_time_constraint::AssignTimeConstraint,
    avoid_clashes_constraint::AvoidClashesConstraint, Constraint,
};

use super::Phenotype;

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
        };

        result.push((c.clone(), indices));
    }

    result
}

// Constraint Cost Functions ///////////////////////////////////////////////////

/// Calculate the cost of the assign time constraint.
/// This constraint checks if every event has a time assigned to it.
/// The allocation/encoding ensures that this is the case. No calculation
/// needed.
pub fn assign_time_constraint(
    phenotype: &Phenotype,
    params: &AssignTimeConstraint,
    event_idxs: &[usize],
) -> usize {
    0
}

pub fn avoid_clashes_constraint(
    phenotype: &Phenotype,
    params: &AvoidClashesConstraint,
    resource_idxs: &[usize],
) -> usize {
    // Deviation
    let deviation: usize = resource_idxs
        .iter()
        .map(|resource_idx| {
            // Get events by resource
            let event_idxs = phenotype.events_by_resource(*resource_idx);

            if event_idxs.len() < 2 {
                return 0;
            }

            // Get all times allocated to the events
            let times = phenotype.times_by_events(&event_idxs);

            // If the times list is shorter than the event list, this means that
            // some events have the same time assigned.
            if times.len() < event_idxs.len() {
                event_idxs.len() - times.len()
            } else {
                0
            }
        })
        .sum();

    // Calc cost and return
    (params.weight as usize) * params.cost_function.calc(deviation)
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
