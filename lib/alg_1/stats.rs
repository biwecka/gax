// Imports /////////////////////////////////////////////////////////////////////
use xhstt::model::instances::Instance;

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Stats {
    // Amount of timeslots available for scheduling
    pub times: usize,

    // IDs of the events which don't have times pre-assigned
    pub event_ids: Vec<String>,

    // Amount of events to be assigned to a timeslot
    pub event_count: usize,
}

// Functions ///////////////////////////////////////////////////////////////////
pub fn calc(instance: &Instance) -> Stats {
    let times = instance.times.times.len();
    let event_ids = instance
        .events
        .events
        .iter()
        .filter(|event| event.time.is_none())
        .map(|event| event.id.clone())
        .collect::<Vec<String>>();
    let event_count = event_ids.len();

    Stats { times, event_ids, event_count }
}

////////////////////////////////////////////////////////////////////////////////
