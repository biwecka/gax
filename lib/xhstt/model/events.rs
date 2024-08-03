// Imports /////////////////////////////////////////////////////////////////////
use super::{ResourceGroupId, ResourceId, ResourceTypeId, TimeId};

// Course //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CourseId(pub String);

impl From<String> for CourseId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for CourseId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
pub struct Course {
    pub id: CourseId,
    pub name: String,
}

impl From<crate::parser::instances::events::Course> for Course {
    fn from(value: crate::parser::instances::events::Course) -> Self {
        let id = CourseId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

// Course //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EventGroupId(pub String);

impl From<String> for EventGroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for EventGroupId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
pub struct EventGroup {
    pub id: EventGroupId,
    pub name: String,
}

impl From<crate::parser::instances::events::EventGroup> for EventGroup {
    fn from(value: crate::parser::instances::events::EventGroup) -> Self {
        let id = EventGroupId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

// Event ///////////////////////////////////////////////////////////////////////
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

#[derive(Clone, Debug)]
pub struct Event {
    pub id: EventId,
    pub color: Option<String>,
    pub name: String,
    pub duration: u32,
    pub workload: Option<u32>,

    pub course: Option<CourseId>,
    pub time: Option<TimeId>,

    pub absent_resources: Vec<AbsentResource>,
    pub assigned_resources: Vec<AssignedResource>,
    pub resource_groups: Vec<super::resources::ResourceGroupRef>,

    pub event_groups: Vec<EventGroupId>,
}

impl From<crate::parser::instances::events::Event> for Event {
    fn from(value: crate::parser::instances::events::Event) -> Self {
        let id = EventId(value.id);
        let color = value.color;
        let name = value.name;
        let duration = value.duration;
        let workload = value.workload;

        let course = value.course.map(|r| CourseId(r.reference));
        let time = value.time.map(|r| TimeId(r.reference));

        let mut absent_resources = vec![];
        let mut assigned_resources = vec![];

        if let Some(resources) = value.resources {
            for resource in resources.list {
                if let Some(reference) = resource.reference {
                    // Resource is preassigned
                    assigned_resources.push(AssignedResource {
                        id: ResourceId(reference),
                        role: resource.role.map(|r| r.value),
                        resource_type: resource
                            .resource_type
                            .map(|r| ResourceTypeId(r.reference)),
                    });
                } else {
                    // Resource is NOT preassigned
                    absent_resources.push(AbsentResource {
                        role: resource.role.expect("role must be set").value,
                        resource_type: ResourceTypeId(
                            resource
                                .resource_type
                                .expect("resource type must be set")
                                .reference,
                        ),
                    })
                }
            }
        }

        let resource_groups = value
            .resource_groups
            .map(|refs| {
                refs.list
                    .into_iter()
                    .map(|x| ResourceGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        let event_groups = value
            .event_groups
            .map(|refs| {
                refs.list
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
            absent_resources,
            assigned_resources,
            resource_groups,
            event_groups,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AssignedResource {
    pub id: ResourceId,
    pub role: Option<String>,
    pub resource_type: Option<ResourceTypeId>,
}

#[derive(Clone, Debug)]
pub struct AbsentResource {
    pub role: String,
    pub resource_type: ResourceTypeId,
}

////////////////////////////////////////////////////////////////////////////////
