// Imports /////////////////////////////////////////////////////////////////////
use bits::{Bits32, Bits64};
use xhstt::{
    db::constraints::Constraint,
    parser::solution_groups::solution::events::{Event, TimeRef},
};

use super::{constraints, Chromosome, Context, Cost};

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    // Time allocation (inner values of the chromosome)
    pub times: Vec<Bits32>,

    // Resources matrix:
    // - the outer vector represents the resources
    // - the inner "bits" represent which event the resource is allocated to
    pub resources: Vec<Bits64>,
}

impl Phenotype {
    pub fn blueprint(db: &xhstt::db::Database, ctx: &Context) -> Self {
        // Create times matrix (this is empty in the blueprint)
        let times: Vec<Bits32> = Vec::with_capacity(ctx.num_events);

        // Create resource vector of bitvectors
        let mut resources =
            vec![Bits64::new(ctx.num_events as u64, 0); ctx.num_resources];

        // Fill resource 2D vector (matrix)
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);

                resources[resource_idx].set(event_idx as u64);
            }
        }

        Self { times, resources }
    }

    pub fn to_solution_events(
        &self,
        db: &xhstt::db::Database,
        _ctx: &Context,
    ) -> Vec<Event> {
        // Variable for storing the solution events.
        let mut events = vec![];

        for (event_idx, times) in self.times.iter().enumerate() {
            // Get event by index from database
            let event = db.event_by_idx(event_idx);

            // Copy the times data.
            let bits = *times;

            // The sub_events variable contains a list of all sub events as
            // tuples.
            // - 1st tuple value = sub_event duration
            // - 2nd tuple value = starting time
            let sub_events = bits
                .blocks()
                .iter()
                .enumerate()
                .filter_map(|(length, bits)| {
                    if bits.is_zero() {
                        return None;
                    }

                    Some(
                        bits.ones()
                            .map(|x| (length, x))
                            .collect::<Vec<(_, _)>>(),
                    )
                })
                .flatten()
                .collect::<Vec<(_, _)>>();

            for (duration, start_time) in sub_events {
                // Get time by index from database
                let time = db.time_by_idx(start_time as usize);

                events.push(Event {
                    reference: event.id.0.clone(),
                    duration: Some(duration as u32),
                    resources: None,
                    time: Some(TimeRef { reference: time.id.0.clone() }),
                })
            }
        }

        // Return
        events
    }
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromosome: &Chromosome, _ctx: &Context) -> Self {
        // Clone the blueprint
        let mut new = self.clone();

        // Copy the inner value of the chromosome to the phenotype
        new.times = chromosome.0.clone();

        // Return the derived phenotype
        new
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;

        for (constraint, indices) in &ctx.constraints {
            #[allow(clippy::single_match)]
            match constraint {
                Constraint::AvoidClashesConstraint(params) => {
                    let cost =
                        constraints::avoid_clashes(self, ctx, params, indices);

                    total_cost += cost;
                },

                // Constraint::AssignTimeConstraint(params) => {
                //     let cost =
                //         constraints::assign_time_constraint(self, ctx, params, indices);

                //     total_cost += cost;
                // },

                _ => {}
            }
        }

        // Return
        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
