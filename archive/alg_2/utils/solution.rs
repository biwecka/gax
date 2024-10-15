// Imports /////////////////////////////////////////////////////////////////////
use xhstt::{
    db::Database,
    parser::solution_groups::solution::events::{
        Event as SolutionEvent, TimeRef,
    },
};

use crate::encoding::chromosome::Chromosome;

// Functions ///////////////////////////////////////////////////////////////////
pub fn create_from(
    chromosome: &Chromosome,
    db: &Database,
) -> Vec<SolutionEvent> {
    let mut events = vec![];

    for (event_idx, time_idx) in chromosome.0.iter().enumerate() {
        // Get event and time
        let event = db.event_by_idx(event_idx);
        let time = db.time_by_idx(*time_idx as usize);

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
