// Imports /////////////////////////////////////////////////////////////////////
use rand::distributions::Uniform;
use xhstt::db::constraints::Constraint;

// Context /////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Context {
    /// Number of "times" in this XHSTT instance.
    #[allow(unused)]
    pub num_times: usize,

    /// Number of "events" in this XHSTT instance.
    pub num_events: usize,

    /// Number of "resources" in this XHSTT instance.
    pub num_resources: usize,

    /// Random time generator. This vector contains a random number generator
    /// for each gene. Index `0` holds the random number generator for the gene
    /// of index `0`.
    pub rand_time: Vec<rand::distributions::Uniform<usize>>,

    /// Random event generator
    #[allow(unused)]
    pub rand_event: rand::distributions::Uniform<usize>,

    /// Constraints (with pre-calculated affected IDs)
    pub constraints: Vec<(Constraint, Vec<usize>)>,

    /// This vector contains the duration of every event.
    /// `durations[i]` yields the duration of the event an ID of `i`.
    pub durations: Vec<u8>,
}

impl ga::encoding::Context for Context {}

impl Context {
    pub fn init(db: &xhstt::db::Database) -> Self {
        let num_times = db.times().len();
        let num_events = db.events().len();
        let num_resources = db.resources().len();

        let rand_event = Uniform::<usize>::new(0, num_events);

        let constraints = super::constraints::pre_calc(db);

        // Get durations ( f(event_idx) -> duration )
        let durations: Vec<_> =
            db.events().iter().map(|e| e.duration as u8).collect();

        // Due to different durations, each event has a different set of valid
        // times.
        let mut rand_time = Vec::with_capacity(num_events);
        #[allow(clippy::needless_range_loop)]
        for event_idx in 0..num_events {
            // Get duration of this event
            let duration = durations[event_idx] as usize;

            // Create random number generator for this gene
            rand_time.push(Uniform::<usize>::new(0, num_times - duration + 1));
        }

        Self {
            num_times,
            num_events,
            num_resources,
            rand_time,
            rand_event,
            constraints,
            durations,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
