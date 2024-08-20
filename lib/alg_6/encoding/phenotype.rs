// Imports /////////////////////////////////////////////////////////////////////
use super::{context::Context, genotype::Chromosome, objective_value::Cost};
use ndarray::{Array2, Axis};
use xhstt::{
    db::constraints::Constraint,
    parser::solution_groups::solution::events::{Event, TimeRef},
};

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    /// This 2D-matrix stores the scheduled times for each event, where
    /// each column represents an event (by id) and each row a timeslot (by
    /// index).
    /// If the cell `[2, 3]` contains the value `1` this means, that
    /// event `3` is scheduled at time `2` (the first index is for the row; like
    /// matrices in math).
    times: Array2<u8>,

    /// This 2D-matrix stores the allocated resources for each event, where
    /// each column represents and event (by id) and each row a resource (by
    /// index). If the cell `[2, 3]` contains the value `1` this means, that
    /// resource `2` is allocated to event `3`.
    ///
    /// Note: u8 is used as datatype, to allow mathematical operations on this
    /// matrix (like summing up rows and columns).
    resources: Array2<u8>,
}

impl Phenotype {
    pub fn blueprint(db: &xhstt::db::Database, ctx: &Context) -> Self {
        // Create times matrix (this is empty in the blueprint)
        let times = Array2::default((ctx.num_times, ctx.num_events));

        // Create resource matrix
        let mut resources =
            Array2::default((ctx.num_resources, ctx.num_events));
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                resources[[resource_idx, event_idx]] = 1;
            }
        }

        Self { times, resources }
    }

    /// This function calculates the clashes as defined in the
    /// `AvoidClashesConstraint`. The number of clashes returned by this
    /// function represents the deviation (as mentioned in the XHSTT docs).
    pub fn calc_clashes(&self, resource_idx: usize, ctx: &Context) -> usize {
        // Get the row with id `resource_idx` from the resource matrix. This
        // row contains information which event is allocated to the resource
        // with index `resource_idx`.
        let resource_row = self.resources.row(resource_idx);

        // The `resource_row` 1D-matrix/vector contains `0` and `1` values.
        // We now want to extract all columns from the `times` matrix, where
        // the `resource_row` vector has an `1` value. This creates a partial
        // "view" into the `times` matrix.
        let size = resource_row.sum() as usize;
        let mut partial_times = Array2::<u8>::default((ctx.num_times, size));

        let mut index = 0;
        for (i, val) in resource_row.iter().enumerate() {
            if *val == 1 {
                let mut col = partial_times.column_mut(index);
                col.assign(&self.times.column(i));
                index += 1;
            }
        }

        // Now that we have this partial view into the `times` matrix, all that
        // is left is to sum the values of the rows in this partial view.
        // If the sum is >1 this means that the currently looked at resource
        // is assigned to two or more events, which are scheduled at the same
        // time. This does not work and is considered a "clash".
        //
        // Because only values above 1 represent a clash, the sum values are
        // subtracted by 1.
        //
        let clash_vector = partial_times
            .fold_axis(Axis(1), 0, |acc, x| acc + *x)
            .map(|x| if *x > 0 { x - 1 } else { *x });

        // The total number of clashes is now the sum of the values in the
        // clash vector.
        clash_vector.sum() as usize
    }

    ///
    pub fn to_solution_events(
        &self,
        db: &xhstt::db::Database,
        ctx: &Context,
    ) -> Vec<Event> {
        let mut events = vec![];

        for (event_idx, column) in self.times.columns().into_iter().enumerate()
        {
            // Get start time
            let start_time_idx = column.iter().position(|x| *x == 1).unwrap();

            // Get time by index from database
            let time = db.time_by_idx(start_time_idx);

            // Get event by index from database
            let event = db.event_by_idx(event_idx);

            // get event duration
            let duration = ctx.durations[event_idx];

            // Create solution event
            events.push(Event {
                reference: event.id.0.clone(),
                duration: Some(duration as u32),
                resources: None,
                time: Some(TimeRef { reference: time.id.0.clone() }),
            });
        }

        events
    }
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromsome: &Chromosome, ctx: &Context) -> Self {
        let mut clone = self.clone();

        for (event_idx, start_time_idx) in chromsome.iter().enumerate() {
            // Get event duration
            let event_duration = ctx.durations[event_idx] as usize;

            // Apply times to the `times` matrix, taking into account that
            // events can have durations greater than 1.
            for time_idx in *start_time_idx..(*start_time_idx + event_duration)
            {
                clone.times[[time_idx, event_idx]] = 1;
            }
        }

        clone
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;
        for (constraint, indices) in &ctx.constraints {
            match constraint {
                Constraint::AssignTimeConstraint(_) => {}
                Constraint::AvoidClashesConstraint(x) => {
                    total_cost += super::constraints::avoid_clashes_constraint(
                        self, ctx, x, indices,
                    );
                }
            }
        }

        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
