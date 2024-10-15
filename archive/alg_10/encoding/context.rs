// Imports /////////////////////////////////////////////////////////////////////
use rand::distributions::Uniform;
// use rand_distr::Normal;
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

    /// This vector contains the duration of every event.
    /// `durations[i]` yields the duration of the event an ID of `i`.
    pub durations: Vec<u8>,

    /// Constraints (with pre-calculated affected IDs)
    pub constraints: Vec<(Constraint, Vec<usize>)>,

    // /// Random number generator for event indices
    // pub rand_event: Uniform<usize>,

    // pub gauss_sd: f32, // standard deviation
    // pub gauss_rand_event: Normal<f32>,
    /// Random number generator for time indices
    pub rand_time: Uniform<usize>,
}

impl ga::encoding::Context for Context {}

impl Context {
    pub fn init(db: &xhstt::db::Database) -> Self {
        let num_times = db.times().len();
        let num_events = db.events().len();
        let num_resources = db.resources().len();

        let durations: Vec<_> =
            db.events().iter().map(|e| e.duration as u8).collect();

        let constraints = super::constraints::pre_calc(db);

        // let rand_event = Uniform::<usize>::new(0, num_events);
        let rand_time = Uniform::<usize>::new(0, num_times);

        // let gauss_sd: f32 = 1.;
        // let gauss_rand_event = Normal::new(0., gauss_sd).unwrap();

        Self {
            num_times,
            num_events,
            num_resources,
            constraints,
            durations,
            // rand_event,
            rand_time,
            // gauss_sd,
            // gauss_rand_event,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
