// Imports /////////////////////////////////////////////////////////////////////
use bits::{
    matrix::{BitsMatrix32x128, BitsMatrix64x128},
    Bits128,
};
use rand_distr::{Normal, Uniform};
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

    /// Resources matrix.
    pub resources: BitsMatrix32x128,

    /// The `resource_relation` vector represents the relation between events
    /// and resources. Instead of mapping this relationship directly in a
    /// matrix, this list contains the pre-computed collision vectors for
    /// each event.
    /// A collision vector for an event `e` contains a set bit (=1) at
    /// index `i`, if `e` and `e_i` have one (or more) resources in common.
    /// This means, that `e` and `e_i` cannot be scheduled at same timeslots,
    /// becaues this would lead to a resource collision.
    pub resource_relations: Vec<Bits128>,

    // /// Random number generator for time indices
    // pub rand_time: Uniform<u32>,
    // pub gauss_rand_time: Normal<f32>,
    // pub gauss_rand_time_sd: f32,
    /// Random number generator for event indices
    pub rand_event: Uniform<u8>,
    pub gauss_rand_event: Normal<f32>,
    pub gauss_rand_event_sd: f32,
    // /// PT2 control circuit
    // pub pt2: PT2,

    // /// State machine (for the `StateMachine` dynamic)
    // pub state_machine: StateMachine,
}

// #[derive(Clone, Default, Debug)]
// pub struct StateMachine {
//     pub last_state_change: usize,
//     pub focus_without_success: usize,
//     pub state: State
// }

// #[derive(Clone, Default, Debug)]
// pub enum State {
//     #[default]
//     Broad,
//     Focus,
//     Finish,
// }

impl ga::encoding::Context for Context {}

impl Context {
    pub fn init(db: &xhstt::db::Database) -> Self {
        // Get crucial numbers/stats of the problem instance.
        let num_times = db.times().len();
        let num_events = db.events().len();
        let num_resources = db.resources().len();

        // Extract the durations of each event
        let durations: Vec<_> =
            db.events().iter().map(|e| e.duration as u8).collect();

        // Pre-calculate the constraints
        let constraints = super::constraints::pre_calc(db);

        // >>> Pre-calculate the resource_relations <<<

        // Create the resource matrix (temporarily), for pre-calculating
        // the resource relations for each event.
        // -> hdtt4 has 45 resources and 59 events, therefore a 64x64 matrix is
        //    chosen
        let mut resources =
            BitsMatrix32x128::new(num_resources as u32, num_events as u128);

        // Fill resource 2D vector (matrix)
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);

                resources.set(resource_idx as u32, event_idx as u128);
            }
        }

        // Pre-calculate the resource relation vectors
        let resource_relations: Vec<Bits128> = (0..num_events)
            .into_iter()
            .map(|event_idx| {
                // Get resources related to the current event
                let resource_indices = resources.col(event_idx as u128).ones();

                // Initialize resource relation bitvec
                let mut rr = Bits128::new(num_events as u128, 0);

                // Iterate resource indices and "OR" them onto the `rr` vector.
                for i in resource_indices {
                    rr |= *resources.row(i as u32);
                }

                // Unset the bit of the current event
                rr.unset(event_idx as u128);

                // Return
                rr
            })
            .collect();

        // >>> Randomness <<<
        let rand_event = Uniform::<u8>::new(0, num_events as u8);
        // let rand_time = Uniform::<u32>::new(0, num_times as u32);

        // let gauss_rand_time_sd = 1.;
        // let gauss_rand_time =
        // Normal::<f32>::new(0., gauss_rand_time_sd).unwrap();

        let gauss_rand_event_sd = 1.;
        let gauss_rand_event =
            Normal::<f32>::new(0., gauss_rand_event_sd).unwrap();

        // let pt2 = PT2::new(1., 1., 0.4, 1., 1.);

        // let state_machine = StateMachine::default();

        // Return
        Self {
            num_times,
            num_events,
            num_resources,
            durations,
            constraints,
            resources,
            resource_relations,

            rand_event,
            // rand_time,
            // gauss_rand_time,
            // gauss_rand_time_sd,
            gauss_rand_event,
            gauss_rand_event_sd,
            // pt2,
            // state_machine,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
