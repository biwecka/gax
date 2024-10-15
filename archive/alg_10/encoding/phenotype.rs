// Imports /////////////////////////////////////////////////////////////////////
use super::{
    constraints, context::Context, genotype::Chromosome, objective_value::Cost,
    EventGene,
};
use bitvec::prelude::*;
use xhstt::{
    db::constraints::Constraint,
    parser::solution_groups::solution::events::{Event, TimeRef},
};

// Phenotype ///////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Phenotype {
    // `times` contains the exact inner value of a chromosome
    pub times: Vec<EventGene>,

    // The vector contains a bitvector for each resource. The inner bitvectors
    // contain information about which events use a given resource.
    // Example: Phenotype.resource [ resource_0 ] = 0 1 1 0 1 0
    // -> event_0 does NOT use resource_0
    // -> event_1 does     use resource_0
    // -> event_2 does     use resource_0
    pub resources: Vec<BitVec<usize, Lsb0>>,
}

impl Phenotype {
    pub fn blueprint(db: &xhstt::db::Database, ctx: &Context) -> Self {
        // Create times matrix (this is empty in the blueprint)
        let times = vec![];

        // Create resource vector of bitvectors
        let mut resources =
            vec![bitvec![usize, Lsb0; 0; ctx.num_events]; ctx.num_resources];

        // Fill resource 2D vector (matrix)
        for (event_idx, event) in db.events().iter().enumerate() {
            for resource in &event.allocated_resources {
                let resource_idx = db.resource_id_to_idx(&resource.id);
                resources[resource_idx].set(event_idx, true);
            }
        }

        Self { times, resources }
    }

    /// The derivation process ensures, that the event-time-allocation only
    /// includes time slots, which are of coherent, of correct length (duration)
    /// and don't overflow the time slot index. Therefore these details don't
    /// need to be checked in this function.
    pub fn to_solution_events(
        &self,
        db: &xhstt::db::Database,
        _ctx: &Context,
    ) -> Vec<Event> {
        let mut events = vec![];

        for (event_idx, event_gene) in self.times.iter().enumerate() {
            // Get event by index from database
            let event = db.event_by_idx(event_idx);

            for (d, sub_events) in &event_gene.sub_event_start_times {
                let start_times = sub_events.iter_ones().collect::<Vec<_>>();

                for start_time in start_times {
                    // Get time by index from database
                    let time = db.time_by_idx(start_time);

                    // Create solution event
                    events.push(Event {
                        reference: event.id.0.clone(),
                        duration: Some(*d as u32),
                        resources: None,
                        time: Some(TimeRef { reference: time.id.0.clone() }),
                    });
                }
            }
        }

        events
    }
}

impl ga::encoding::Phenotype<Cost, Context, Chromosome> for Phenotype {
    fn derive(&self, chromosome: &Chromosome, _ctx: &Context) -> Self {
        // Clone the blueprint phenotype
        let mut new = self.clone();

        // Copy inner value of chromosome to phenotype
        new.times = chromosome.0.clone();

        // Return the derived phenotypes
        new
    }

    fn evaluate(&self, ctx: &Context) -> Cost {
        let mut total_cost = 0;

        for (constraint, indices) in &ctx.constraints {
            #[allow(clippy::single_match)]
            match constraint {
                // The assign time constraint can be ignore, because the correct
                // time assignmend is ensured through the encoding.

                // Constraint::AssignTimeConstraint(params) => {
                //     let cost =
                //         constraints::assign_time(self, ctx, params, indices);
                //     println!("assign time const = {cost}");
                //     total_cost += cost;
                // }
                Constraint::AvoidClashesConstraint(params) => {
                    let cost =
                        constraints::avoid_clashes(self, ctx, params, indices);

                    total_cost += cost;
                }

                _ => {}
            }
        }

        // Return
        total_cost.into()
    }
}

////////////////////////////////////////////////////////////////////////////////
