// Helper Functions ////////////////////////////////////////////////////////////
pub fn only_time_allocation_needed(db: &xhstt::db::Database) -> bool {
    for event in db.events() {
        if !event.unallocated_resources.is_empty() {
            return false;
        }
    }

    true
}

pub fn only_duration_of_1(db: &xhstt::db::Database) -> bool {
    for event in db.events() {
        if event.duration != 1 {
            return false;
        }
    }

    true
}

// pub fn allocation_and_chromosome_same_length(db: &xhstt::db::Database) -> bool {
//     if db.events().len() == db.events_with_no_time().len() {
//         true
//     } else {
//         false
//     }
// }

pub fn only_hard_constraints(db: &xhstt::db::Database) -> bool {
    db.contraints()
        .iter()
        .filter(|x| !x.is_required())
        .collect::<Vec<_>>()
        .is_empty()
}

////////////////////////////////////////////////////////////////////////////////
