use std::time::Instant;

// Imports /////////////////////////////////////////////////////////////////////
use crate::{population::Chromosome, stats::Stats};
use xhstt::model::{
    constraints::{AssignTimeConstraint, AvoidClashesConstraint, Constraint},
    events::EventId,
    Constraints, Data,
};

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Fitness(usize);
impl From<usize> for Fitness {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

// Functions ///////////////////////////////////////////////////////////////////

/// Evaluates the fitness of a chromosome.
pub fn eval(
    chromosome: &Chromosome,
    data: &Data,
    cstr: &Constraints,
    stats: &Stats,
) -> Fitness {
    // Clone instance (TODO: this might be not very smart)
    let mut data: Data = data.clone();
    // Apply chromosome to instance
    for (locus, gene) in chromosome.0.iter().enumerate() {
        // Translate locus and gene
        let event_index = locus;
        let time_index = gene.0;

        // Get event_id by gene_index
        let event_id = stats.events.get(event_index).unwrap();

        // Get time reference by gene
        let time = data.get_time_by_idx(time_index).clone();

        // Apply time to event
        let event = data.get_event_by_id_mut(event_id);
        event.time = Some(time.id.clone());
    }

    let mut total_fitness: usize = 0;

    for constraint in cstr.all() {
        match constraint {
            Constraint::AssignTimeConstraint(params) => {
                total_fitness += eval_assign_time_constraint(params, &data);
            }

            Constraint::AvoidClashesConstraint(params) => {
                total_fitness += eval_avoid_clashes_constraint(params, &data);
            }
        }
    }

    // Return
    total_fitness.into()
}

// Helpers /////////////////////////////////////////////////////////////////////
fn eval_assign_time_constraint(
    params: AssignTimeConstraint,
    data: &Data,
) -> usize {
    // Event IDs this constraint applies to
    let mut event_ids: Vec<EventId> = params.applies_to.events;

    // Add event_ids from event groups
    for event_group in &params.applies_to.event_groups {
        let ids = data.get_events_by_event_group(&event_group);
        event_ids.extend_from_slice(ids);
    }

    // Check constraint for events
    let mut deviation: usize = 0;
    for event_id in event_ids {
        // Get event
        let event = data.get_event_by_id(&event_id);

        if event.time.is_none() {
            deviation += event.duration as usize;
        }
    }

    // Calculate cost
    let cost = (params.weight as usize) * params.cost_function.calc(deviation);

    // Return
    cost
}

fn eval_avoid_clashes_constraint(
    params: AvoidClashesConstraint,
    data: &Data,
) -> usize {
    0
}

////////////////////////////////////////////////////////////////////////////////
