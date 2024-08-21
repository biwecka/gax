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
