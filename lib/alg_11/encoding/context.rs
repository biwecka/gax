// Imports /////////////////////////////////////////////////////////////////////
use control_circuits::PT2;
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

    /// This vector contains the duration of every event.
    /// `durations[i]` yields the duration of the event an ID of `i`.
    pub durations: Vec<u8>,

    /// Constraints (with pre-calculated affected IDs)
    pub constraints: Vec<(Constraint, Vec<usize>)>,

    /// Random number generator for time indices
    pub rand_time: Uniform<u32>,
    pub gauss_rand_time: Normal<f32>,
    pub gauss_rand_time_sd: f32,

    /// Random number generator for event indices
    pub rand_event: Uniform<usize>,
    pub gauss_rand_event: Normal<f32>,
    pub gauss_rand_event_sd: f32,

    /// PT2 control circuit
    pub pt2: PT2,

    /// State machine (for the `StateMachine` dynamic)
    pub state_machine: StateMachine,
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

        let rand_event = Uniform::<usize>::new(0, num_events);
        let rand_time = Uniform::<u32>::new(0, num_times as u32);

        let gauss_rand_time_sd = 1.;
        let gauss_rand_time =
            Normal::<f32>::new(0., gauss_rand_time_sd).unwrap();

        let gauss_rand_event_sd = 1.;
        let gauss_rand_event =
            Normal::<f32>::new(0., gauss_rand_event_sd).unwrap();

        let pt2 = PT2::new(1., 1., 0.4, 1., 1.);

        let state_machine = StateMachine::default();

        Self {
            num_times,
            num_events,
            num_resources,
            constraints,
            durations,
            rand_event,
            rand_time,
            gauss_rand_time,
            gauss_rand_time_sd,
            gauss_rand_event,
            gauss_rand_event_sd,
            pt2,
            state_machine,
        }
    }
}

// State Machine ///////////////////////////////////////////////////////////////
#[derive(Clone, Default, Debug)]
pub struct StateMachine {
    pub last_state_change: usize,
    pub focus_without_success: usize,
    pub state: State
}

#[derive(Clone, Default, Debug)]
pub enum State {
    #[default]
    Broad,
    Focus,
    Finish,
}

////////////////////////////////////////////////////////////////////////////////

