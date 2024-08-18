// Imports /////////////////////////////////////////////////////////////////////
use rand::distributions::Uniform;
use xhstt::db::constraints::Constraint;

// Context /////////////////////////////////////////////////////////////////////
pub struct Context {
    /// Number of "times" in this XHSTT instance.
    pub num_times: usize,

    /// Number of "events" in this XHSTT instance.
    pub num_events: usize,

    /// Number of "resources" in this XHSTT instance.
    pub num_resources: usize,

    /// Random time generator
    pub rand_time: rand::distributions::Uniform<usize>,

    /// Random event generator
    pub rand_event: rand::distributions::Uniform<usize>,

    /// Constraints (with pre-calculated affected IDs)
    pub constraints: Vec<(Constraint, Vec<usize>)>,
}

impl ga::encoding::Context for Context {}

impl Context {
    pub fn init(db: &xhstt::db::Database) -> Self {
        let num_times = db.times().len();
        let num_events = db.events().len();
        let num_resources = db.resources().len();
        let rand_time = Uniform::<usize>::new(0, num_times);
        let rand_event = Uniform::<usize>::new(0, num_events);

        let constraints = super::constraints::pre_calc(&db);

        Self {
            num_times,
            num_events,
            num_resources,
            rand_time,
            rand_event,
            constraints,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
