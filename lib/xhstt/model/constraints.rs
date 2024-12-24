// Imports /////////////////////////////////////////////////////////////////////
use super::{EventGroupId, EventId, ResourceGroupId, ResourceId};
use crate::parser::instances::constraints::Constraint as ParserConstraint;

// Constraints /////////////////////////////////////////////////////////////////
structstruck::strike!(
    #[strikethrough[derive(Clone, Debug)]]
    #[strikethrough[deprecated]]
    pub enum Constraint {
        AssignResourceConstraint(pub struct {}),

        AssignTimeConstraint(pub struct {
            pub id: String,
            pub name: String,
            pub required: bool,
            pub weight: u32,
            pub cost_function: CostFunction,
            pub applies_to: AppliesToEventsAndGroups,
        }),

        SplitEventsConstraint(pub struct {}),
        DistributeSplitEventsConstraint(pub struct {}),
        PreferResourcesConstraint(pub struct {}),
        PreferTimesConstraint(pub struct {}),
        AvoidSplitAssignmentsConstraint(pub struct {}),
        SpreadEventsConstraint(pub struct {}),
        LinkEventsConstraint(pub struct {}),
        OrderEventsConstraint(pub struct {}),

        AvoidClashesConstraint(pub struct {
            pub id: String,
            pub name: String,
            pub required: bool,
            pub weight: u32, // [0, 1000]
            pub cost_function: CostFunction,
            pub applies_to: AppliesToResourcesAndGroups,
        }),

        AvoidUnavailableTimesConstraint(pub struct {}),
        LimitIdleTimesConstraint(pub struct {}),
        ClusterBusyTimesConstraint(pub struct {}),
        LimitBusyTimesConstraint(pub struct {}),
        LimitWorkloadConstraint(pub struct {}),
    }
);

impl From<crate::parser::instances::constraints::Constraint> for Constraint {
    fn from(value: ParserConstraint) -> Self {
        match value {
            ParserConstraint::AssignResourceConstraint(_) => {
                Self::AssignResourceConstraint(AssignResourceConstraint {})
            }

            ParserConstraint::AssignTimeConstraint(data) => {
                Self::AssignTimeConstraint(AssignTimeConstraint {
                    id: data.id,
                    name: data.name,
                    required: data.required,
                    weight: data.weight,
                    cost_function: data.cost_function.into(),
                    applies_to: data.applies_to.into(),
                })
            }

            ParserConstraint::SplitEventsConstraint(_) => {
                Self::SplitEventsConstraint(SplitEventsConstraint {})
            }
            ParserConstraint::DistributeSplitEventsConstraint(_) => {
                Self::DistributeSplitEventsConstraint(
                    DistributeSplitEventsConstraint {},
                )
            }
            ParserConstraint::PreferResourcesConstraint(_) => {
                Self::PreferResourcesConstraint(PreferResourcesConstraint {})
            }
            ParserConstraint::PreferTimesConstraint(_) => {
                Self::PreferTimesConstraint(PreferTimesConstraint {})
            }
            ParserConstraint::AvoidSplitAssignmentsConstraint(_) => {
                Self::AvoidSplitAssignmentsConstraint(
                    AvoidSplitAssignmentsConstraint {},
                )
            }
            ParserConstraint::SpreadEventsConstraint(_) => {
                Self::SpreadEventsConstraint(SpreadEventsConstraint {})
            }
            ParserConstraint::LinkEventsConstraint(_) => {
                Self::LinkEventsConstraint(LinkEventsConstraint {})
            }
            ParserConstraint::OrderEventsConstraint(_) => {
                Self::OrderEventsConstraint(OrderEventsConstraint {})
            }

            ParserConstraint::AvoidClashesConstraint(data) => {
                Self::AvoidClashesConstraint(AvoidClashesConstraint {
                    id: data.id,
                    name: data.name,
                    required: data.required,
                    weight: data.weight,
                    cost_function: data.cost_function.into(),
                    applies_to: data.applies_to.into(),
                })
            }

            ParserConstraint::AvoidUnavailableTimesConstraint(_) => {
                Self::AvoidUnavailableTimesConstraint(
                    AvoidUnavailableTimesConstraint {},
                )
            }
            ParserConstraint::LimitIdleTimesConstraint(_) => {
                Self::LimitIdleTimesConstraint(LimitIdleTimesConstraint {})
            }
            ParserConstraint::ClusterBusyTimesConstraint(_) => {
                Self::ClusterBusyTimesConstraint(ClusterBusyTimesConstraint {})
            }
            ParserConstraint::LimitBusyTimesConstraint(_) => {
                Self::LimitBusyTimesConstraint(LimitBusyTimesConstraint {})
            }
            ParserConstraint::LimitWorkloadConstraint(_) => {
                Self::LimitWorkloadConstraint(LimitWorkloadConstraint {})
            }
        }
    }
}

// Applies To ... //////////////////////////////////////////////////////////////

// ... Events and Event Groups
#[derive(Clone, Debug)]
#[deprecated]
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
#[deprecated]
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
    #[strikethrough[deprecated]]
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
#[deprecated]
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
#[deprecated]
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
