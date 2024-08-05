// Imports /////////////////////////////////////////////////////////////////////
use xhstt::{
    db::Database,
    parser::solution_groups::solution::events::{
        Event as SolutionEvent, TimeRef,
    },
};

use crate::encoding::{allocation::Allocation, chromosome::Chromosome};

// Functions ///////////////////////////////////////////////////////////////////
pub fn create_from(
    chromosome: &Chromosome,
    allocation: &Allocation,
    db: &Database,
) -> Vec<SolutionEvent> {
    let allocation = allocation.derive(&chromosome);
    let mut events = vec![];

    for (i, event) in db.events().iter().enumerate() {
        let time_idx_opt = allocation
            .times_by_event(i)
            .iter()
            .enumerate()
            .filter_map(|(i, val)| if *val == 1 { Some(i) } else { None })
            .collect::<Vec<usize>>()
            .first()
            .cloned();

        let time_idx = match time_idx_opt {
            Some(x) => x,
            None => continue,
        };

        let time = db.time_by_idx(time_idx);

        // Create event
        events.push(SolutionEvent {
            reference: event.id.0.clone(),
            duration: None,
            resources: None,
            time: Some(TimeRef { reference: time.id.0.clone() }),
        });
    }

    // Return
    events
}

////////////////////////////////////////////////////////////////////////////////
