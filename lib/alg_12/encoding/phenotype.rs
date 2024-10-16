// Imports /////////////////////////////////////////////////////////////////////
use bits::matrix::BitsMatrix32x128;
use itertools::Itertools;
use xhstt::{
    db::constraints::Constraint,
    parser::solution_groups::solution::events::{Event, TimeRef},
};

use super::{Chromosome, Context, Cost};

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    /// The `times` matrix represents the allocation of times to events.
    /// Each column represents an event, and each row represents a timeslot.
    /// -> hdtt4 has 30 times and 59 events, therefore a 32x64 matrix is chosen
    pub times: BitsMatrix32x128,

    /// The `blocked` matrix has the same dimensions as the `times` matrix,
    /// but stores which timeslots are blocked due to time allocations of events
    /// and their resource-relations to other events.
    pub blocked: BitsMatrix32x128,
}

impl Phenotype {
    pub fn blueprint(_db: &xhstt::db::Database, ctx: &Context) -> Self {
        // Create times and blocked matrix (both are empty in the blueprint)
        let times =
            BitsMatrix32x128::new(ctx.num_times as u32, ctx.num_events as u128);

        let blocked =
            BitsMatrix32x128::new(ctx.num_times as u32, ctx.num_events as u128);

        Self { times, blocked }
    }

    pub fn to_solution_events(
        &self,
        db: &xhstt::db::Database,
        ctx: &Context,
    ) -> Vec<Event> {
        // Variable for storing the solution events.
        let mut events = vec![];

        for event_idx in 0..ctx.num_events {
            // Get event times
            let times = self.times.col(event_idx as u128);

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
    fn derive(&self, chromosome: &Chromosome, ctx: &Context) -> Self {
        // Clone the blueprint
        let mut new = self.clone();

        // Iterate over the. Its values are event indices. Schedule the events
        // in the order they appear in the chromosome.
        for event_idx in chromosome.0.iter() {
            // Get the pre-calculated resource_relations of this event
            let resource_relation = ctx.resource_relations[*event_idx as usize];

            // Get the duration of the event
            let duration = ctx.durations[*event_idx as usize];

            // Calculate and shufle the indices of free timeslots
            let free_timeslots = new.blocked.col(*event_idx as u128).zeros();
            // .collect::<Vec<u32>>();

            // free_timeslots.shuffle(&mut rand::thread_rng());

            // Create combinations of free timeslots in the target duration.
            let time_groups = free_timeslots
                .into_iter()
                .combinations(duration as usize)
                .take(32);

            // Evaluate each time group
            let mut evaluation: Vec<(Vec<u32>, u8)> = vec![];

            'l: for time_group in time_groups {
                // Initialize time group efficiency score
                let mut efficiency: u8 = 0;

                // Evaluate the time group one time slot after the other.
                for time_idx in &time_group {
                    // Get allocation of this time
                    let time_allocation = new.times.row(*time_idx);
                    let blocked = new.blocked.row(*time_idx);

                    // Check if the time_allocation and the resource_relation
                    // have any time slots in common. If so, the time group
                    // can't be used!
                    let overlap = *time_allocation & resource_relation;

                    if !overlap.is_zero() {
                        continue 'l;
                    }

                    // Now we know, that the time allocation does not collide
                    // with any resource relations.
                    // The next step is to evaluate the efficiency.
                    let overlap = *blocked & resource_relation;

                    efficiency += overlap.ones().count() as u8;
                }

                // If all times of the time group don't collide with other time
                // allocations, then add the time group to the evaluated list.
                evaluation.push((time_group, efficiency));
            }

            // Sort the evaluated time groups by their efficiency score
            evaluation.sort_by_key(|(_, efficiency)| *efficiency);

            // Allocate the time group with the best efficiency to the
            // times (and blocked) matrix
            // let best = &evaluation.last().unwrap().0;
            let best = match evaluation.last() {
                Some(x) => &x.0,
                None => continue,
            };

            // Times matrix
            for time_idx in best {
                new.times.set(*time_idx, *event_idx as u128);
            }

            // Blocked matrix
            for time_idx in best {
                // Get current row
                let mut tmp = *new.blocked.row(*time_idx);

                // "Or" it with the resource_relation vector. This is possible,
                // because the resource_relation vectors don't contain the
                // event they originate from/belong to.
                tmp |= resource_relation;

                // Update the row in the matrix
                new.blocked.set_row(*time_idx, tmp);
            }
        }

        // Return the derived phenotype
        new
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;

        for (constraint, indices) in &ctx.constraints {
            #[allow(clippy::single_match)]
            match constraint {
                Constraint::AssignTimeConstraint(params) => {
                    total_cost += super::constraints::assign_time_constraint(
                        self, ctx, params, indices,
                    );
                }

                Constraint::AvoidClashesConstraint(params) => {
                    total_cost += super::constraints::avoid_clashes(
                        self, ctx, params, indices,
                    );
                }

                _ => {}
            }
        }

        // Return
        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
