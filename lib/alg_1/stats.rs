// Imports /////////////////////////////////////////////////////////////////////
use xhstt::model::{events::EventId, Data};

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Stats {
    // Amount of timeslots available for scheduling
    pub times: usize,

    // IDs of the events which need to be scheduled (the ones that don't have
    // pre-assigned times).
    pub events: Vec<EventId>,
}

// Functions ///////////////////////////////////////////////////////////////////
pub fn calc(data: &Data) -> Stats {
    let times = data.get_times().len();

    let events = data
        .get_events()
        .iter()
        .filter_map(|event| match event.time {
            Some(_) => None,
            None => Some(event.id.clone()),
        })
        .collect::<Vec<EventId>>();

    Stats { times, events }
}

////////////////////////////////////////////////////////////////////////////////
