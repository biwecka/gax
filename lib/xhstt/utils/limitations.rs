//! This module contains functions which perform certain check on a given
//! database. These checks can be used to check/assert pre-conditions of a
//! XHSTT-solving algorithm.
//!
//! For example: If an algorithm can only handle events with a duration of 1,
//! the [`only_duration_of_1`] function can be used to check if this condition
//! is met by the problem instance that is passed into the algorithm.

// Helper Functions ////////////////////////////////////////////////////////////
pub fn only_time_allocation_needed(db: &crate::db::Database) -> bool {
    for event in db.events() {
        if !event.unallocated_resources.is_empty() {
            return false;
        }
    }

    true
}

pub fn only_duration_of_1(db: &crate::db::Database) -> bool {
    for event in db.events() {
        if event.duration != 1 {
            return false;
        }
    }

    true
}

pub fn no_event_has_preassigned_time(db: &crate::db::Database) -> bool {
    db.events().len() == db.events_with_no_time().len()
}

pub fn only_hard_constraints(db: &crate::db::Database) -> bool {
    db.contraints()
        .iter()
        .filter(|x| !x.is_required())
        .collect::<Vec<_>>()
        .is_empty()
}

////////////////////////////////////////////////////////////////////////////////
