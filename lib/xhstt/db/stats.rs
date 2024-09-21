use super::resources::resource_type::ResourceTypeId;

#[derive(Debug)]
pub struct Stats {
    // General
    pub instance_id: String,
    pub instance_name: String,

    // Times
    pub times: usize,

    // Events
    pub events: usize,
    pub max_event_duration: usize,
    pub events_with_all_resources_preassigned: usize,
    pub events_with_class_preassigned: usize,
    pub events_with_class_unassigned: usize,
    pub events_with_time_preassigned: usize,

    // Resources
    pub resource_types: usize,
    pub resource_groups: usize,
    pub class_resource_type: bool,
}

impl Stats {
    pub fn new(db: &super::Database) -> Self {
        // Instance name
        let instance_id = db.instance_id.clone();
        let instance_name = db.instance_name.clone();

        // Times
        let times = db.times.len();

        // Events
        let events = db.events.len();

        let max_event_duration =
            db.events.iter().map(|x| x.duration).max().unwrap() as usize;

        let events_with_all_resources_preassigned = db
            .events
            .iter()
            .filter(|e| e.unallocated_resources.is_empty())
            .count();

        let events_with_class_preassigned = db
            .events
            .iter()
            .filter(|e| {
                e.allocated_resources.iter().any(|r| {
                    let resource = db.resource_by_id(&r.id);

                    resource.resource_type == ResourceTypeId("Class".into())
                        || resource.resource_type
                            == ResourceTypeId("class".into())
                })
            })
            .count();

        let events_with_class_unassigned = db
            .events
            .iter()
            .filter(|e| {
                e.unallocated_resources.iter().any(|r| {
                    r.resource_type == ResourceTypeId("Class".into())
                        || r.resource_type == ResourceTypeId("class".into())
                })
            })
            .count();

        let events_with_time_preassigned =
            db.events.iter().filter(|e| e.time.is_some()).count();

        // let events_with_course_preassigned = db.events.iter().filter(|e| {
        //     e.course.is_some()
        // }).count();

        // Resources
        let resource_types = db.resource_types.len();
        let resource_groups = db.resource_groups.len();
        let class_resource_type = db.resource_types.iter().any(|x| {
            x.id == ResourceTypeId("Class".into())
                || x.id == ResourceTypeId("class".into())
        });

        // Return
        Self {
            // General
            instance_id,
            instance_name,

            // Times
            times,

            // Events
            events,
            max_event_duration,
            events_with_all_resources_preassigned,
            events_with_class_preassigned,
            events_with_class_unassigned,
            events_with_time_preassigned,

            // Resources
            resource_types,
            resource_groups,
            class_resource_type,
        }
    }

    pub fn needs_resource_assignment(&self) -> bool {
        self.events_with_all_resources_preassigned != self.events
    }

    pub fn needs_class_assignment(&self) -> bool {
        self.events_with_class_unassigned != 0
    }
}
