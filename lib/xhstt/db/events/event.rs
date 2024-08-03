// Imports /////////////////////////////////////////////////////////////////////
use crate::{
    db::{
        resources::{
            resource::ResourceId, resource_group::ResourceGroupId,
            resource_type::ResourceTypeId,
        },
        times::time::TimeId,
    },
    parser::instances::events::Event as IEvent,
};

use super::{course::CourseId, event_group::EventGroupId};

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EventId(pub String);
impl From<String> for EventId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for EventId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Event {
    pub id: EventId,
    pub color: Option<String>,
    pub name: String,
    pub duration: u32,
    pub workload: Option<u32>,

    pub course: Option<CourseId>,
    pub time: Option<TimeId>,
    pub allocated_resources: Vec<AllocatedResource>,
    pub unallocated_resources: Vec<UnallocatedResource>,
    pub resource_groups: Vec<ResourceGroupId>,
    pub event_groups: Vec<EventGroupId>,
}

impl From<IEvent> for Event {
    fn from(value: IEvent) -> Self {
        let id = EventId(value.id);
        let color = value.color;
        let name = value.name;
        let duration = value.duration;
        let workload = value.workload;

        let course = value.course.map(|x| CourseId(x.reference));
        let time = value.time.map(|x| TimeId(x.reference));

        let mut allocated_resources = vec![];
        let mut unallocated_resources = vec![];
        if let Some(resources) = value.resources {
            for resource in resources.list {
                if let Some(reference) = resource.reference {
                    // Resource is preassigned
                    allocated_resources.push(AllocatedResource {
                        id: ResourceId(reference),
                        role: resource.role.map(|x| x.value),
                        resource_type: resource
                            .resource_type
                            .map(|x| ResourceTypeId(x.reference)),
                    });
                } else {
                    // Resource is NOT preassigned
                    assert!(resource.role.is_some());
                    assert!(resource.resource_type.is_some());
                    unallocated_resources.push(UnallocatedResource {
                        role: resource.role.unwrap().value,
                        resource_type: ResourceTypeId(
                            resource.resource_type.unwrap().reference,
                        ),
                    });
                }
            }
        }

        let resource_groups = value
            .resource_groups
            .map(|references| {
                references
                    .list
                    .into_iter()
                    .map(|x| ResourceGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        let event_groups = value
            .event_groups
            .map(|references| {
                references
                    .list
                    .into_iter()
                    .map(|x| EventGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        Self {
            id,
            color,
            name,
            duration,
            workload,
            course,
            time,
            allocated_resources,
            unallocated_resources,
            resource_groups,
            event_groups,
        }
    }
}

// Helper Structs //////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct AllocatedResource {
    pub id: ResourceId,
    pub role: Option<String>,
    pub resource_type: Option<ResourceTypeId>,
}

#[derive(Clone, Debug)]
pub struct UnallocatedResource {
    pub role: String,
    pub resource_type: ResourceTypeId,
}

////////////////////////////////////////////////////////////////////////////////
