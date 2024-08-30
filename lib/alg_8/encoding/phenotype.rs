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
    times: Array2<u8>,
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
            // Check if the time allocation is coherent
            // TODO: is this correct?
            let chromosome: Chromosome = self.times.clone().into();
            let (_, coherent) = chromosome.get_event_time_allocation(event_idx);
            if !coherent {
                // println!("skipped an event (not coherent)");
                continue;
            }

            // Get start time
            let start_time_idx =
                if let Some(x) = column.iter().position(|x| *x == 1) {
                    x
                } else {
                    // println!("skipped an event (no time)");
                    continue;
                };

            // Get time by index from database
            let time = db.time_by_idx(start_time_idx);

            // Get event by index from database
            let event = db.event_by_idx(event_idx);

            // get event duration
            let duration = ctx.durations[event_idx];

            // Check if time allocation + duration overflows
            // the max. time slot index
            if start_time_idx + duration as usize - 1 >= ctx.num_times {
                continue;
            }


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
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromsome: &Chromosome, ctx: &Context) -> Self {
        // Clone the blueprint phenotype
        let mut new = self.clone();

        // new.times = chromsome.0.clone();

        // Assign the events matrix to the cloned phenotype
        new.times = Array2::default(chromsome.0.dim()); //chromsome.0.clone();

        for event_idx in 0..ctx.num_events {
            //
            let (sum, coherent) = chromsome.get_event_time_allocation(event_idx);

            let correct_duration = sum == ctx.durations[event_idx];

            let overflow: bool = if let Some(i) = self.times
                .column(event_idx)
                .iter()
                .position(|x| x == &1)
            {
                let duration = ctx.durations[event_idx];

                (i + duration as usize - 1) >= ctx.num_times

            } else { false };

            if correct_duration && coherent && !overflow {
                new.times.column_mut(event_idx).assign(&chromsome.0.column(event_idx));
            }
        }

        // Return the derived phenotypes
        new
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;

        for (constraint, indices) in &ctx.constraints {
            match constraint {
                Constraint::AssignTimeConstraint(params) => {
                    let deviation: usize = indices
                        .iter()
                        .map(|event_idx| {
                            // // Get event time allocation
                            // let chromosome: Chromosome = self.times
                            //     .clone()
                            //     .into();

                            // let (sum, coherent) = chromosome
                            //     .get_event_time_allocation(*event_idx);

                            // // Get event duration
                            // let duration = ctx.durations[*event_idx];

                            // if !coherent { // || duration > sum {
                            //     return duration as usize; // - sum as usize;
                            // }

                            // // Check if time allocation + duration overflows
                            // // the max. time slot index
                            // if let Some(i) = self.times
                            //     .column(*event_idx)
                            //     .iter()
                            //     .position(|x| x == &1)
                            // {
                            //     if (i + duration as usize - 1) >= ctx.num_times
                            //     {
                            //         return duration as usize; // - sum as usize;
                            //     }
                            // }
                            // duration as usize - sum as usize

                            let sum = self.times.column(*event_idx).sum();
                            let duration = ctx.durations[*event_idx] as usize;
                            duration - (sum as usize)
                        })
                        .sum();

                    let cost = (params.weight as usize)
                        * params.cost_function.calc(deviation);

                    total_cost += cost;
                }

                Constraint::AvoidClashesConstraint(params) => {
                    let deviation: usize = indices
                        .iter()
                        .map(|resource_idx| {
                            self.calc_clashes(*resource_idx, ctx)
                        })
                        .sum();

                    let cost = (params.weight as usize)
                        * params.cost_function.calc(deviation);

                    total_cost += cost;
                }
            }
        }

        // Return
        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
