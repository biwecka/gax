// Imports /////////////////////////////////////////////////////////////////////
use xhstt::db::constraints::{
    assign_time_constraint::AssignTimeConstraint, Constraint,
};

use super::Context;

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

// pub fn assign_time_constraint(
//     phenotype: &Phenotype,
//     ctx: &Context,
//     params: &AssignTimeConstraint,
//     event_idxs: &[usize],
// ) -> usize {
//     let deviation: usize = event_idxs
//         .iter()
//         .map(|event_idx| {
//             // Get column
//             let col_sum = phenotype.times.column(*event_idx).clamp(0, 1).sum();

//             // Get duration
//             let duration = ctx.durations[*event_idx] as usize;

//             duration - (col_sum as usize)
//         })
//         .sum();

//     // Return
//     // Calc cost and return
//     (params.weight as usize) * params.cost_function.calc(deviation)
// }

////////////////////////////////////////////////////////////////////////////////
