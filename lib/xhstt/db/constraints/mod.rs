// Modules /////////////////////////////////////////////////////////////////////
pub mod assign_time_constraint;
pub mod avoid_clashes_constraint;

// Imports /////////////////////////////////////////////////////////////////////
use super::{
    events::{event::EventId, event_group::EventGroupId},
    resources::{resource::ResourceId, resource_group::ResourceGroupId},
};
use crate::parser::instances::constraints::{
    AppliesToEventGroups as IAppliesToEventGroups,
    AppliesToEventPairs as IAppliesToEventPairs,
    AppliesToEventsAndGroups as IAppliesToEventsAndGroups,
    AppliesToResourcesAndGroups as IAppliesToResourcesAndGroups,
    Constraint as IConstraint, CostFunction as ICostFunction,
    EventPair as IEventPair,
};
use assign_time_constraint::AssignTimeConstraint;
use avoid_clashes_constraint::AvoidClashesConstraint;

// Constraints /////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub enum Constraint {
    // AssignResourceConstraint,
    AssignTimeConstraint(AssignTimeConstraint),
    // SplitEventsConstraint,
    // DistributeSplitEventsConstraint,
    // PreferResourcesConstraint,
    // PreferTimesConstraint,
    // AvoidSplitAssignmentsConstraint,
    // SpreadEventsConstraint,
    // LinkEventsConstraint,
    // OrderEventsConstraint,
    AvoidClashesConstraint(AvoidClashesConstraint),
    // AvoidUnavailableTimesConstraint,
    // LimitIdleTimesConstraint,
    // ClusterBusyTimesConstraint,
    // LimitBusyTimesConstraint,
    // LimitWorkloadConstraint,
}

impl From<IConstraint> for Constraint {
    fn from(value: crate::parser::instances::constraints::Constraint) -> Self {
        match value {
            IConstraint::AssignTimeConstraint(data) => {
                Self::AssignTimeConstraint(data.into())
            }

            IConstraint::AvoidClashesConstraint(data) => {
                Self::AvoidClashesConstraint(data.into())
            }
        }
    }
}

// Helper Structs //////////////////////////////////////////////////////////////

// Applies To: Events and Event Groups
#[derive(Clone, Debug)]
pub struct AppliesToEventsAndGroups {
    pub event_groups: Vec<EventGroupId>,
    pub events: Vec<EventId>,
}

impl From<IAppliesToEventsAndGroups> for AppliesToEventsAndGroups {
    fn from(value: IAppliesToEventsAndGroups) -> Self {
        let event_groups = value
            .event_groups
            .map(|refs| {
                refs.list
                    .into_iter()
                    .map(|x| EventGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        let events = value
            .events
            .map(|refs| {
                refs.list.into_iter().map(|x| EventId(x.reference)).collect()
            })
            .unwrap_or_default();

        Self { event_groups, events }
    }
}

// Applies To: Event Groups
#[derive(Clone, Debug)]
pub struct AppliesToEventGroups {
    pub event_groups: Vec<EventGroupId>,
}

impl From<IAppliesToEventGroups> for AppliesToEventGroups {
    fn from(value: IAppliesToEventGroups) -> Self {
        let event_groups = value
            .event_groups
            .list
            .into_iter()
            .map(|x| EventGroupId(x.reference))
            .collect();

        Self { event_groups }
    }
}

// Applies To: Event Pairs
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug)]]
    pub struct AppliesToEventPairs {
        pub event_pairs: Vec<pub struct EventPair {
            pub first_event: EventId,
            pub second_event: EventId,

            pub min_separation: Option<u32>,
            pub max_separation: Option<u32>,
        }>
    }
);

impl From<IAppliesToEventPairs> for AppliesToEventPairs {
    fn from(value: IAppliesToEventPairs) -> Self {
        let event_pairs =
            value.event_pairs.list.into_iter().map(|x| x.into()).collect();

        Self { event_pairs }
    }
}

impl From<IEventPair> for EventPair {
    fn from(value: IEventPair) -> Self {
        let first_event = EventId(value.first_event.reference);
        let second_event = EventId(value.second_event.reference);
        let min_separation = value.min_separation;
        let max_separation = value.max_separation;

        Self { first_event, second_event, min_separation, max_separation }
    }
}

// Applies To: Resources and Groups
#[derive(Clone, Debug)]
pub struct AppliesToResourcesAndGroups {
    pub resource_groups: Vec<ResourceGroupId>,
    pub resources: Vec<ResourceId>,
}

impl From<IAppliesToResourcesAndGroups> for AppliesToResourcesAndGroups {
    fn from(value: IAppliesToResourcesAndGroups) -> Self {
        let resource_groups = value
            .resource_groups
            .map(|refs| {
                refs.list
                    .into_iter()
                    .map(|x| ResourceGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        let resources = value
            .resources
            .map(|refs| {
                refs.list.into_iter().map(|x| ResourceId(x.reference)).collect()
            })
            .unwrap_or_default();

        Self { resource_groups, resources }
    }
}

// Cost Function ///////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub enum CostFunction {
    Linear,
    Quadratic,
    Step,
}

impl CostFunction {
    pub fn calc(&self, value: usize) -> usize {
        match self {
            CostFunction::Linear => value,
            CostFunction::Quadratic => value * value,
            CostFunction::Step => {
                if value != 0 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl From<ICostFunction> for CostFunction {
    fn from(value: ICostFunction) -> Self {
        match value {
            ICostFunction::Linear => Self::Linear,
            ICostFunction::Quadratic => Self::Quadratic,
            ICostFunction::Step => Self::Step,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
