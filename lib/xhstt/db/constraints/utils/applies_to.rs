use hashbrown::HashSet;

// Imports /////////////////////////////////////////////////////////////////////
use crate::db::{
    events::{event::EventId, event_group::EventGroupId},
    resources::{resource::ResourceId, resource_group::ResourceGroupId},
};
use crate::parser::instances::constraints::{
    AppliesToEventGroups as IAppliesToEventGroups,
    AppliesToEventPairs as IAppliesToEventPairs,
    AppliesToEventsAndGroups as IAppliesToEventsAndGroups,
    AppliesToResourcesAndGroups as IAppliesToResourcesAndGroups,
    EventPair as IEventPair,
};

////////////////////////////////////////////////////////////////////////////////
// Applies To: Events and Event Groups
#[derive(Clone, Debug)]
pub struct AppliesToEventsAndGroups {
    pub event_groups: Vec<EventGroupId>,
    pub events: Vec<EventId>,
}

impl AppliesToEventsAndGroups {
    /// Resolves the event_groups and events to event indices.
    pub fn resolve_idxs(&self, db: &crate::db::Database) -> Vec<usize> {
        let mut event_idxs = vec![];

        // Resolve event groups
        for event_group in &self.event_groups {
            let mut event_indices = db.event_group_event_idxs(event_group);
            event_idxs.append(&mut event_indices);
        }

        // Resolve events
        for event in &self.events {
            event_idxs.push(db.event_id_to_idx(event));
        }

        // Return
        event_idxs
    }
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

////////////////////////////////////////////////////////////////////////////////
// Applies To: Event Groups
#[derive(Clone, Debug)]
pub struct AppliesToEventGroups {
    pub event_groups: Vec<EventGroupId>,
}

impl AppliesToEventGroups {
    /// Resolves the event_groups to event indices.
    pub fn resolve_idxs(&self, db: &crate::db::Database) -> Vec<usize> {
        let mut event_idxs = vec![];

        // Resolve event groups
        for event_group in &self.event_groups {
            let mut event_indices = db.event_group_event_idxs(event_group);
            event_idxs.append(&mut event_indices);
        }

        // Return
        event_idxs
    }
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

////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////
// Applies To: Resources and Groups
#[derive(Clone, Debug)]
pub struct AppliesToResourcesAndGroups {
    pub resource_groups: Vec<ResourceGroupId>,
    pub resources: Vec<ResourceId>,
}

impl AppliesToResourcesAndGroups {
    /// Resolves the resource_groups and resources to resource indices.
    pub fn resolve_idxs(&self, db: &crate::db::Database) -> Vec<usize> {
        let mut resource_idxs = vec![];

        // Resolve event groups
        for resource_group in &self.resource_groups {
            let mut event_indices =
                db.resource_group_resource_idxs(resource_group);
            resource_idxs.append(&mut event_indices);
        }

        // Resolve events
        for resource in &self.resources {
            resource_idxs.push(db.resource_id_to_idx(resource));
        }

        // Return
        let set = HashSet::<usize>::from_iter(resource_idxs);
        set.into_iter().collect()
    }
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

////////////////////////////////////////////////////////////////////////////////
