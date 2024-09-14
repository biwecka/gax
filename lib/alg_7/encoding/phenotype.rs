// Imports /////////////////////////////////////////////////////////////////////
use super::{context::Context, genotype::Chromosome, objective_value::Cost};
use itertools::Itertools;
use ndarray::{Array2, Axis};
use rand::seq::SliceRandom;
use xhstt::{
    db::constraints::Constraint,
    parser::solution_groups::solution::events::{Event, TimeRef},
};

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    /// This 2D-matrix stores the scheduled times for each event, where
    /// each column represents an event (by id) and each row a timeslot (
    /// by index).
    /// If the cell `[2, 3]` contains the value `1` this means, that time
    /// with index `2` is assigned to the event with index `3`.
    /// The cell value `0` denotes, that this event-time combination is open
    /// to be scheduled, whereas `-1` means, that this event-time combination
    /// cannot be scheduled, because this would introduce a resource conflict.
    pub times: Array2<i8>,

    /// This 2D-matrix stores the allocated resources for each event, where
    /// each column represents an event (by index) and each row a resource (
    /// by index). If the cell `[2, 3]` contains the value `1` this means, that
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

    /// TODO: docs
    pub fn to_solution_events(
        &self,
        db: &xhstt::db::Database,
        ctx: &Context,
    ) -> Vec<Event> {
        let mut events = vec![];

        for (event_idx, column) in self.times.columns().into_iter().enumerate()
        {
            // Collect 1-sequences as sub-events
            let mut tmp = vec![];
            let mut sub_events = vec![];

            for (i, val) in column.iter().enumerate() {
                if *val == 1 {
                    tmp.push(i);

                } else {
                    if !tmp.is_empty() {
                        sub_events.push(tmp);
                        tmp = vec![];
                    }
                }
            }

            if !tmp.is_empty() {
                sub_events.push(tmp);
            }

            for sub_event in sub_events {
                // Get start time index
                let start_time_idx = sub_event[0];

                // Get time by index from database
                let time = db.time_by_idx(start_time_idx);

                // Get event from database
                let event = db.event_by_idx(event_idx);

                // Duration
                let duration = sub_event.len();

                // Create solution event
                events.push(Event {
                    reference: event.id.0.clone(),
                    duration: Some(duration as u32),
                    resources: None,
                    time: Some(TimeRef { reference: time.id.0.clone() }),
                });
            }


            // // Get start time
            // let start_time_idx =
            //     if let Some(x) = column.iter().position(|x| *x == 1) {
            //         x
            //     } else {
            //         continue;
            //     };

            // // Get time by index from database
            // let time = db.time_by_idx(start_time_idx);

            // // Get event by index from database
            // let event = db.event_by_idx(event_idx);

            // // get event duration
            // let duration = ctx.durations[event_idx];

            // // Create solution event
            // events.push(Event {
            //     reference: event.id.0.clone(),
            //     duration: Some(duration as u32),
            //     resources: None,
            //     time: Some(TimeRef { reference: time.id.0.clone() }),
            // });
        }

        events
    }
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromosome: &Chromosome, ctx: &Context) -> Self {
        let mut new = self.clone();

        let mut rng = rand::thread_rng();

        // Iterate over the chromosome. Its values are event indices.
        // Schedule the events in the order they appear in the chromosome.
        for event_idx in chromosome.iter() {
            // Get the indices of the allocated resources of this event
            let resource_idxs = new
                .resources
                .column(*event_idx)
                .into_iter()
                .enumerate()
                .filter_map(
                    |(i, value)| if *value == 1 { Some(i) } else { None },
                )
                .collect::<Vec<usize>>();

            // Create a resource matrix containing only the events related to
            // this event.
            let mut event_resources =
                Array2::<u8>::default((resource_idxs.len(), ctx.num_events));
            for (i, resource_idx) in resource_idxs.iter().enumerate() {
                let mut row = event_resources.row_mut(i);
                row.assign(&new.resources.row(*resource_idx));
            }

            // This event-specific resource matrix can now be reduced, by
            // summing up the values in each column. As each column represents
            // one event, calculating the sum of each column will tell us
            // which events are "connected" through resource dependencies with
            // the current event (values > 0), and which events are not
            // connected to the current event (values = 0);
            // Using `clamp(0, 1)` will reduce all values > 1 to exactly 1.
            let relations = event_resources
                .fold_axis(Axis(0), 0, |acc, x| acc + *x)
                .clamp(0, 1);

            // Get the duration of the event
            let duration = ctx.durations[*event_idx];

            // Now we have to calculate the time groups, which the event can
            // be scheduled at. Why time GROUPS? Because events have a certain
            // duration (e.g. duration = 2), so the event is assigned two
            // time slots which we call "time group" here.
            // The time groups should contain the time indices of the times.
            // let time_groups: Vec<Vec<usize>> = new
            //     .times
            //     .column(*event_idx)
            //     .iter()
            //     .enumerate()
            //     .map(|(i, value)| (i, *value))
            //     .collect::<Vec<(usize, i8)>>()
            //     // .windows(duration as usize)
            //     .into_iter()
            //     .combinations(duration as usize)
            //     .collect::<Vec<Vec<(usize, i8)>>>()
            //     .into_iter()
            //     // Window content:  time_idx, value (-1, 0, 1)
            //     .filter_map(|window: Vec<(usize, i8)>| {
            //         let (indices, values): (Vec<usize>, Vec<i8>) =
            //             window.iter().copied().unzip();

            //         if values.contains(&-1) || values.contains(&1) {
            //             None
            //         } else {
            //             Some(indices)
            //         }
            //     })
            //     .collect();

            // Calculate and shuffle the indices of free timeslots
            let mut free_timeslots: Vec<usize> = new
                .times
                .column(*event_idx)
                .iter()
                .enumerate()
                .filter_map(|(index, value)| {
                    if *value == 0 {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect();

            free_timeslots.shuffle(&mut rng);



            let time_groups: Vec<Vec<usize>> = free_timeslots
                .into_iter()
                .combinations(duration as usize)
                .take(100)
                .collect();


            // If no time groups can be scheduled, continue with the main loop
            if time_groups.is_empty() {
                continue;
            }

            // The timegroups now have to be evaluated: At first it has to be
            // checked, if the event can even be scheduled in a time group
            // due to its resource relations to other events.
            // Secondly each time group is evaluated on how good of a "snug
            // fit" it is in the time matrix (having "-1"'s in the same spot
            // is counted as efficiency).
            let mut evaluated_time_groups: Vec<(Vec<usize>, usize)> = vec![];
            'l: for time_group in time_groups {
                // Efficiency score for this time group
                let mut efficiency = 0;

                // Iterate times in this time group
                for time_idx in &time_group {
                    // The `time_allocation` variable represents the allocation
                    // of events to the current time of index `time_idx`.
                    let time_allocation = new.times.row(*time_idx);

                    // We now take this time_allocation vector, ignore the `-1`
                    // values, because we're only interested in checking, if
                    // this time slot can be assigned to the current event
                    // (this is done by `.clamp(0, 1)`).
                    //
                    // The time allocation vector with `-1` clamped to 0 can now
                    // be added to the collision vector. The result will contain
                    // `1` for events, that are present in the time allocation
                    // OR the collision vector, and the value `2` denotes, that
                    // the event occurs in both vectors.
                    // Obviously, an event can only be allocated to the current
                    // time, if the resulting vector does not contain any `2`
                    // values, because in that case assigning the current time
                    // to the current event will result in a resource conflict
                    // with the event represented by the index with the value 2.
                    let mut result =
                        time_allocation.mapv(|x| x.clamp(0, 1) as u8);
                    result.scaled_add(1, &relations);

                    if result.iter().any(|x| *x > 1) {
                        continue 'l;
                    }
                    drop(result);

                    // Now that we know that the event can be assigned to this
                    // time slot, we need to calculate "how good" it fits.
                    // Therefore we again take the time allocation vector, but
                    // this time we're interested in the `-1` values.
                    // Therefore we map all the other values to `0` and then
                    // convert the `-1`'s to `1` (to be able to perform
                    // calculations).
                    // This gives us a vector, where `1` marks "blocked"
                    // spots.
                    let mut result =
                        time_allocation.mapv(|x| (-x).clamp(0, 1) as u8);
                    result.scaled_add(1, &relations);

                    // For each field in the resulting vector that contains
                    // the number `2`, one efficiency point is given.
                    // The fields with value `2` mean, that the collision vector
                    // and the modified time allocation vector contained a `1`
                    // at this index.
                    let efficiency_score: usize = result
                        .into_iter()
                        .map(|x| if x >= 2 { 1 } else { 0 })
                        .sum();

                    efficiency += efficiency_score;
                }

                // Add this time group to the evaluation list
                evaluated_time_groups.push((time_group, efficiency));
            }

            // Sort the time groups by their efficiency
            evaluated_time_groups.sort_by_key(|(_, efficiency)| *efficiency);

            // Allocate the event to the time group with the best efficiency
            let best = evaluated_time_groups.last().unwrap();
            // let relations_inverted = relations.mapv(|x| -(x as i8));

            for time_idx in &best.0 {
                for (i, collision) in relations.iter().enumerate() {
                    // The relations/collision vector is `1` for all events
                    // that collide with the current event, and most importantly
                    // it also INCLUDES the current event.
                    if *collision == 0 {
                        continue;
                    }

                    if i == (*event_idx) {
                        // Add `1` for the current event
                        new.times[[*time_idx, i]] = 1;
                    } else {
                        new.times[[*time_idx, i]] = -1;
                    }
                }
            }
        }

        // Return the new instance
        new
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;

        for (constraint, indices) in &ctx.constraints {
            match constraint {
                Constraint::AssignTimeConstraint(params) => {
                    total_cost += super::constraints::assign_time_constraint(
                        self, ctx, params, indices,
                    );
                }

                // There cannot be any resource clashes with this encoding.
                Constraint::AvoidClashesConstraint(_) => {}

                _ => {}
            }
        }

        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
