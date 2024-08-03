// Imports /////////////////////////////////////////////////////////////////////
use crate::{population::Chromosome, stats::Stats};
use hashbrown::HashSet;
use xhstt::model::{
    constraints::{AssignTimeConstraint, AvoidClashesConstraint, Constraint},
    events::EventId,
    resources::ResourceId,
    times::TimeId,
    Constraints, Data,
};

// Structs /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fitness(pub usize);
impl From<usize> for Fitness {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cost(pub usize);
impl From<usize> for Cost {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

// Functions ///////////////////////////////////////////////////////////////////

/// Evaluates the fitness/cost of a chromosome.
pub fn eval(
    chromosome: &Chromosome,
    data: &Data,
    cstr: &Constraints,
    stats: &Stats,
) -> Cost {
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

    let mut total_cost: usize = 0;

    for constraint in cstr.all() {
        match constraint {
            Constraint::AssignTimeConstraint(params) => {
                total_cost += eval_assign_time_constraint(params, &data);
            }

            Constraint::AvoidClashesConstraint(params) => {
                total_cost += eval_avoid_clashes_constraint(params, &data);
            }
        }
    }

    // Return
    total_cost.into()
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
    // Collect resources this constraint applies to
    let mut resource_ids = params.applies_to.resources;

    // Add resource_ids from resource groups
    for resource_group in &params.applies_to.resource_groups {
        let ids = data.get_resources_by_resource_group(resource_group);
        resource_ids.extend_from_slice(ids);
    }

    // Ensure uniqueness
    let set: HashSet<ResourceId> = HashSet::from_iter(resource_ids);
    resource_ids = set.into_iter().collect();

    // dbg!(&resource_ids.len());

    // Get events for every resource and check if constraint is violated
    let mut deviation: usize = 0;

    // let mut first = true;
    for resource_id in resource_ids {
        // Get events by resource_id
        let events = data.get_events_by_resource(&resource_id);

        if events.len() < 2 {
            continue;
        }

        // if first { dbg!(&events); }

        let times: HashSet<TimeId> =
            HashSet::from_iter(events.iter().map(|e| e.time.clone().unwrap()));

        // if first { dbg!(times.len()); }

        // TODO: this only works if all events have duration = 1 !!!
        if times.len() < events.len() {
            // deviation += events.len() - 1;
            deviation += events.len() - times.len();
        }

        // first = false;
    }

    // Calculate cost
    let cost = (params.weight as usize) * params.cost_function.calc(deviation);

    // Return
    cost
}

////////////////////////////////////////////////////////////////////////////////
