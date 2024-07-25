// Imports /////////////////////////////////////////////////////////////////////
use crate::{population::Chromosome, stats::Stats};
use xhstt::model::instances::{
    constraints::{AssignTimeConstraint, AvoidClashesConstraint, Constraint},
    events::TimeRef,
    Instance,
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
    instancex: &Instance,
    stats: &Stats,
) -> Fitness {
    // Clone instance (TODO: this might be not very smart)
    let mut instance = instancex.clone();

    // Apply chromosome to instance
    for (gene_index, gene) in chromosome.0.iter().enumerate() {
        // Get event_id by gene_index
        let event_id = stats.event_ids.get(gene_index).unwrap();

        // Get time reference by gene
        let time = instance.times.times.get(gene.0).unwrap();

        // Apply time to event
        let event = instance
            .events
            .events
            .iter_mut()
            .find(|e| e.id.eq(event_id))
            .unwrap();
        event.time = Some(TimeRef { reference: time.id.to_owned() });
    }

    let mut total_fitness: usize = 0;

    for constraint in &instance.constraints.list {
        match constraint {
            Constraint::AssignTimeConstraint(params) => {
                total_fitness += eval_assign_time_constraint(params, &instance);
            }

            Constraint::AvoidClashesConstraint(params) => {
                total_fitness +=
                    eval_avoid_clashes_constraint(params, &instance);
            }
        }
    }

    // Return
    total_fitness.into()
}

// Helpers /////////////////////////////////////////////////////////////////////
fn eval_assign_time_constraint(
    params: &AssignTimeConstraint,
    instance: &Instance,
) -> usize {
    // Event IDs this constraint applies to
    let mut event_ids: Vec<String> = vec![];

    // Collect event IDs
    if let Some(constraint_applies_to_events) = &params.applies_to.events {
        event_ids.append(
            &mut constraint_applies_to_events.list
                .iter()
                .map(|e| e.reference.clone())
                .collect()
            );
    }

    // Collect event IDs from event groups
    if let Some(constraint_applies_to_event_groups) = &params.applies_to.event_groups {
        for event_group in &constraint_applies_to_event_groups.list {
            // Get events of this event group
            let mut events = instance.events.events
                .iter()
                .filter(|e| match e.event_groups {
                    Some(ref event_groups) => event_groups.list
                        .iter()
                        .map(|eg| eg.reference.clone())
                        .collect::<Vec<String>>()
                        .contains(&event_group.reference),
                    None => false,
                })
                .map(|event| event.id.clone())
                .collect::<Vec<String>>();

            event_ids.append(&mut events);
        }
    }

    // Check constraint for events
    let mut deviation: usize = 0;
    for event_id in event_ids {
        // Get event
        let event = instance.events.events.iter().find(|e| e.id.eq(&event_id)).unwrap();
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
    params: &AvoidClashesConstraint,
    instance: &Instance,
) -> usize {
    0
}

////////////////////////////////////////////////////////////////////////////////
