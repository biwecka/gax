// Imports /////////////////////////////////////////////////////////////////////
use rand::distributions::Uniform;
use rand_distr::Normal;
use xhstt::db::constraints::Constraint;

// Context /////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Context {
    /// Number of "times" in this XHSTT instance.
    pub num_times: usize,

    /// Number of "events" in this XHSTT instance.
    pub num_events: usize,

    /// Number of "resources" in this XHSTT instance.
    pub num_resources: usize,

    /// Random event generator
    pub std_deviation: f32,
    pub rand_event: rand_distr::Normal<f32>,
    pub rand_event_uniform: rand::distributions::Uniform<usize>,

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

        let std_deviation = 1.;
        let rand_event = Normal::<f32>::new(0., std_deviation).unwrap();
        let rand_event_uniform = Uniform::<usize>::new(0, num_events);

        let constraints = super::constraints::pre_calc(db);

        // Get durations ( f(event_idx) -> duration )
        let durations: Vec<_> =
            db.events().iter().map(|e| e.duration as u8).collect();

        Self {
            num_times,
            num_events,
            num_resources,
            std_deviation,
            rand_event,
            rand_event_uniform,
            constraints,
            durations,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
