// Imports /////////////////////////////////////////////////////////////////////
use super::{EventGroupId, EventId, ResourceGroupId, ResourceId};

// Constraints /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug)]]
    pub enum Constraint {
        // AssignResourceConstraint,

        AssignTimeConstraint(pub struct {
            pub id: String,
            pub name: String,
            pub required: bool,
            pub weight: u32,
            pub cost_function: CostFunction,
            pub applies_to: AppliesToEventsAndGroups,
        }),

        // SplitEventsConstraint,
        // DistributeSplitEventsConstraint,
        // PreferResourcesConstraint,
        // PreferTimesConstraint,
        // AvoidSplitAssignmentsConstraint,
        // SpreadEventsConstraint,
        // LinkEventsConstraint,
        // OrderEventsConstraint,

        AvoidClashesConstraint(pub struct {
            pub id: String,
            pub name: String,
            pub required: bool,
            pub weight: u32, // [0, 1000]
            pub cost_function: CostFunction,
            pub applies_to: AppliesToResourcesAndGroups,
        }),

        // AvoidUnavailableTimesConstraint,
        // LimitIdleTimesConstraint,
        // ClusterBusyTimesConstraint,
        // LimitBusyTimesConstraint,
        // LimitWorkloadConstraint,
    }
);

impl From<crate::parser::instances::constraints::Constraint> for Constraint {
    fn from(value: crate::parser::instances::constraints::Constraint) -> Self {
        match value {
            crate::parser::instances::constraints::Constraint::AssignTimeConstraint(data) => {
                Self::AssignTimeConstraint(AssignTimeConstraint {
                    id: data.id,
                    name: data.name,
                    required: data.required,
                    weight: data.weight,
                    cost_function: data.cost_function.into(),
                    applies_to: data.applies_to.into(),
                })
            },

            crate::parser::instances::constraints::Constraint::AvoidClashesConstraint(data) => {
                Self::AvoidClashesConstraint(AvoidClashesConstraint {
                    id: data.id,
                    name: data.name,
                    required: data.required,
                    weight: data.weight,
                    cost_function: data.cost_function.into(),
                    applies_to: data.applies_to.into(),
                })
            }
        }
    }
}

// Applies To ... //////////////////////////////////////////////////////////////

// ... Events and Event Groups
#[derive(Clone, Debug)]
pub struct AppliesToEventsAndGroups {
    pub event_groups: Vec<EventGroupId>,
    pub events: Vec<EventId>,
}

impl From<crate::parser::instances::constraints::AppliesToEventsAndGroups>
    for AppliesToEventsAndGroups
{
    fn from(
        value: crate::parser::instances::constraints::AppliesToEventsAndGroups,
    ) -> Self {
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

// ... Event Groups
#[derive(Clone, Debug)]
pub struct AppliesToEventGroups {
    pub event_groups: Vec<EventGroupId>,
}

impl From<crate::parser::instances::constraints::AppliesToEventGroups>
    for AppliesToEventGroups
{
    fn from(
        value: crate::parser::instances::constraints::AppliesToEventGroups,
    ) -> Self {
        let event_groups = value
            .event_groups
            .list
            .into_iter()
            .map(|x| EventGroupId(x.reference))
            .collect();

        Self { event_groups }
    }
}

// ... Event Pairs
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

impl From<crate::parser::instances::constraints::AppliesToEventPairs>
    for AppliesToEventPairs
{
    fn from(
        value: crate::parser::instances::constraints::AppliesToEventPairs,
    ) -> Self {
        let event_pairs =
            value.event_pairs.list.into_iter().map(|x| x.into()).collect();

        Self { event_pairs }
    }
}

impl From<crate::parser::instances::constraints::EventPair> for EventPair {
    fn from(value: crate::parser::instances::constraints::EventPair) -> Self {
        let first_event = EventId(value.first_event.reference);
        let second_event = EventId(value.second_event.reference);
        let min_separation = value.min_separation;
        let max_separation = value.max_separation;

        Self { first_event, second_event, min_separation, max_separation }
    }
}

// ... Resources and Groups
#[derive(Clone, Debug)]
pub struct AppliesToResourcesAndGroups {
    pub resource_groups: Vec<ResourceGroupId>,
    pub resources: Vec<ResourceId>,
}

impl From<crate::parser::instances::constraints::AppliesToResourcesAndGroups>
    for AppliesToResourcesAndGroups
{
    fn from(
        value: crate::parser::instances::constraints::AppliesToResourcesAndGroups,
    ) -> Self {
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

impl From<crate::parser::instances::constraints::CostFunction>
    for CostFunction
{
    fn from(
        value: crate::parser::instances::constraints::CostFunction,
    ) -> Self {
        match value {
            crate::parser::instances::constraints::CostFunction::Linear => {
                Self::Linear
            }
            crate::parser::instances::constraints::CostFunction::Quadratic => {
                Self::Quadratic
            }
            crate::parser::instances::constraints::CostFunction::Step => {
                Self::Step
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
