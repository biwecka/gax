// Modules /////////////////////////////////////////////////////////////////////
pub mod constraints;
pub mod events;
pub mod resources;
pub mod times;

// Imports /////////////////////////////////////////////////////////////////////
use constraints::Constraint;
use events::{
    course::{Course, CourseId},
    event::{Event, EventId},
    event_group::{EventGroup, EventGroupId},
};
use resources::{
    resource::{Resource, ResourceId},
    resource_group::{ResourceGroup, ResourceGroupId},
    resource_type::{ResourceType, ResourceTypeId},
};
use times::{
    day::{Day, DayId},
    time::{Time, TimeId},
    time_group::{TimeGroup, TimeGroupId},
    week::{Week, WeekId},
};

// Database ////////////////////////////////////////////////////////////////////
/// The database struct is initialized from a XHSTT problem instance and
/// represents all the information from this instance in one central struct,
/// which provides methods to query each part of that data.
///
/// Querying the data of a XHSTT problem instance is crucial for calculating
/// the dimensions of lists or matrices, when initializing the encoding for
/// a genetic algorithm, or ensuring the problem instance complies with certain
/// pre-conditions, the algorithm relies on.
pub struct Database {
    // Times Data //////////////////////////////////////////////////////////////
    weeks: Vec<Week>,
    days: Vec<Day>,
    time_groups: Vec<TimeGroup>,
    times: Vec<Time>,

    // Resources Data //////////////////////////////////////////////////////////
    resource_types: Vec<ResourceType>,
    resource_groups: Vec<ResourceGroup>,
    resources: Vec<Resource>,

    // Events Data /////////////////////////////////////////////////////////////
    courses: Vec<Course>,
    event_groups: Vec<EventGroup>,
    events: Vec<Event>,

    // Constraints Data ////////////////////////////////////////////////////////
    constraints: Vec<Constraint>,

    // Misc ////////////////////////////////////////////////////////////////////
    pub instance_id: String,
    pub instance_name: String,
    ////////////////////////////////////////////////////////////////////////////
}

impl Database {
    /// Initialize the [`Database`] from the given XHSTT problem instance.
    pub fn init(
        instance: &crate::parser::instances::Instance,
    ) -> Result<Self, Vec<String>> {
        // Time information
        let mut weeks: Vec<Week> = vec![];
        let mut days: Vec<Day> = vec![];
        let mut time_groups: Vec<TimeGroup> = vec![];
        let mut times: Vec<Time> = vec![];

        if let Some(x) = &instance.times.time_groups {
            x.weeks
                .clone()
                .into_iter()
                .for_each(|week| weeks.push(week.into()));

            x.days.clone().into_iter().for_each(|day| days.push(day.into()));

            x.time_groups
                .clone()
                .into_iter()
                .for_each(|tg| time_groups.push(tg.into()));
        }

        instance
            .times
            .times
            .clone()
            .into_iter()
            .for_each(|t| times.push(t.into()));

        // Resource information
        let mut resource_types: Vec<ResourceType> = vec![];
        let mut resource_groups: Vec<ResourceGroup> = vec![];
        let mut resources: Vec<Resource> = vec![];

        if let Some(x) = &instance.resources.resource_types {
            x.list
                .clone()
                .into_iter()
                .for_each(|rt| resource_types.push(rt.into()));
        }

        if let Some(x) = &instance.resources.resource_groups {
            x.list
                .clone()
                .into_iter()
                .for_each(|rg| resource_groups.push(rg.into()));
        }

        instance
            .resources
            .resources
            .clone()
            .into_iter()
            .for_each(|r| resources.push(r.into()));

        // Event information
        let mut courses: Vec<Course> = vec![];
        let mut event_groups: Vec<EventGroup> = vec![];
        let mut events: Vec<Event> = vec![];

        if let Some(x) = &instance.events.event_groups {
            x.courses.clone().into_iter().for_each(|c| courses.push(c.into()));

            x.event_groups
                .clone()
                .into_iter()
                .for_each(|eg| event_groups.push(eg.into()));
        }

        instance
            .events
            .events
            .clone()
            .into_iter()
            .for_each(|e| events.push(e.into()));

        // Constraints
        let constraints = instance
            .constraints
            .list
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect();

        // Get instance name
        let instance_id = instance.id.clone();
        let instance_name = instance.metadata.name.clone();

        // Create database instance
        let db = Self {
            weeks,
            days,
            time_groups,
            times,
            resource_types,
            resource_groups,
            resources,
            courses,
            event_groups,
            events,
            constraints,
            instance_id,
            instance_name,
        };

        // Check all references and return
        db.check_references()?;
        Ok(db)
    }

    /// Perform a check on all references contained in the XHSTT problem
    /// description.
    /// References are usually ID values, which refer to other entities of
    /// the problem instance description (e.g., times referring to time groups
    /// they belong to).
    ///
    /// This function returns `Ok(())`, if all references are valid.
    /// Otherwise, the function will return `Err(Vec<String>)` which contains
    /// a list of errors which will indicate which data's references have not
    /// been found in the problem instance.
    fn check_references(&self) -> Result<(), Vec<String>> {
        // Collect used references
        let mut week_ids = vec![];
        let mut day_ids = vec![];
        let mut time_group_ids = vec![];

        for time in self.times() {
            if let Some(x) = &time.week {
                week_ids.push(x);
            }
            if let Some(x) = &time.day {
                day_ids.push(x);
            }
            time.time_groups.iter().for_each(|x| {
                time_group_ids.push(x);
            });
        }

        let mut resource_type_ids = vec![];
        let mut resource_group_ids = vec![];

        for resource_goup in self.resource_groups() {
            resource_type_ids.push(resource_goup.resource_type.clone());
        }
        for resource in self.resources() {
            resource_type_ids.push(resource.resource_type.clone());
            resource.resource_groups.iter().for_each(|x| {
                resource_group_ids.push(x);
            });
        }

        let mut course_ids = vec![];
        let mut time_ids = vec![];
        let mut resource_ids = vec![];
        let mut event_group_ids = vec![];

        for event in self.events() {
            if let Some(course_id) = &event.course {
                course_ids.push(course_id);
            }
            if let Some(time_id) = &event.time {
                time_ids.push(time_id);
            }

            for res in &event.allocated_resources {
                resource_ids.push(res.id.clone());
                if let Some(rt) = &res.resource_type {
                    resource_type_ids.push(rt.clone());
                }
            }

            for res in &event.unallocated_resources {
                resource_type_ids.push(res.resource_type.clone());
            }

            for rg in &event.resource_groups {
                resource_group_ids.push(rg);
            }

            for eg in &event.event_groups {
                event_group_ids.push(eg);
            }
        }

        // Check references
        let mut report: Vec<String> = vec![];
        for week_id in week_ids {
            if !self.weeks().iter().any(|x| x.id.eq(week_id)) {
                report.push(format!("Week ID \"{}\" not found.", week_id.0));
            }
        }
        for day_id in day_ids {
            if !self.days().iter().any(|x| x.id.eq(day_id)) {
                report.push(format!("Day ID \"{}\" not found.", day_id.0));
            }
        }
        for tg_id in time_group_ids {
            if !self.time_groups().iter().any(|x| x.id.eq(tg_id)) {
                report.push(format!("TimeGroup ID \"{}\" not found.", tg_id.0));
            }
        }

        for rt_id in resource_type_ids {
            if !self.resource_types().iter().any(|x| x.id.eq(&rt_id)) {
                report.push(format!(
                    "ResourceType ID \"{}\" not found.",
                    rt_id.0
                ));
            }
        }
        for rg_id in resource_group_ids {
            if !self.resource_groups().iter().any(|x| x.id.eq(rg_id)) {
                report.push(format!(
                    "ResourceGroup ID \"{}\" not found.",
                    rg_id.0
                ));
            }
        }

        for c_id in course_ids {
            if !self.courses().iter().any(|x| x.id.eq(c_id)) {
                report.push(format!("Course ID \"{}\" not found.", c_id.0));
            }
        }
        for t_id in time_ids {
            if !self.times().iter().any(|x| x.id.eq(t_id)) {
                report.push(format!("Time ID \"{}\" not found.", t_id.0));
            }
        }
        for r_id in resource_ids {
            if !self.resources().iter().any(|x| x.id.eq(&r_id)) {
                report.push(format!("Resource ID \"{}\" not found.", r_id.0));
            }
        }
        for eg_id in event_group_ids {
            if !self.event_groups().iter().any(|x| x.id.eq(eg_id)) {
                // Also search courses for the given resource id, because the
                // GR-PA-08 instance uses course ids as event group references.
                if !self.courses().iter().any(|x| x.id.0.eq(&eg_id.0)) {
                    report.push(format!(
                        "EventGroup ID \"{}\" not found.",
                        eg_id.0
                    ));
                }
            }
        }

        // Return
        if report.is_empty() {
            Ok(())
        } else {
            Err(report)
        }
    }
}

// Time Data Methods ///////////////////////////////////////////////////////////

// Week-Methods
impl Database {
    /// Get a list of all weeks
    pub fn weeks(&self) -> &[Week] {
        &self.weeks
    }

    /// Get a week by id.
    pub fn week_by_id(&self, id: &WeekId) -> &Week {
        self.weeks.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a week by index.
    pub fn week_by_idx(&self, idx: usize) -> &Week {
        assert!(idx < self.weeks.len());
        &self.weeks[idx]
    }

    /// Get the list of time indices, which belong to the week.
    pub fn week_time_ids(&self, id: &WeekId) -> Vec<usize> {
        let mut time_idxs = vec![];
        for (i, time) in self.times.iter().enumerate() {
            if let Some(wid) = &time.week {
                if wid.eq(id) {
                    time_idxs.push(i);
                }
            }
        }

        time_idxs
    }
}

// Day-Methods
impl Database {
    /// Get a list of all days
    pub fn days(&self) -> &[Day] {
        &self.days
    }

    /// Get a day by id.
    pub fn day_by_id(&self, id: &DayId) -> &Day {
        self.days.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a day by index.
    pub fn day_by_idx(&self, idx: usize) -> &Day {
        assert!(idx < self.days.len());
        &self.days[idx]
    }

    /// Get the list of time indices, which belong to the day.
    pub fn day_time_ids(&self, id: &DayId) -> Vec<usize> {
        let mut time_idxs = vec![];
        for (i, time) in self.times.iter().enumerate() {
            if let Some(did) = &time.day {
                if did.eq(id) {
                    time_idxs.push(i);
                }
            }
        }

        time_idxs
    }
}

// TimeGroup-Methods
impl Database {
    /// Get a list of all time-groups
    pub fn time_groups(&self) -> &[TimeGroup] {
        &self.time_groups
    }

    /// Get a time-group by id.
    pub fn time_group_by_id(&self, id: &TimeGroupId) -> &TimeGroup {
        self.time_groups.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a time_group by index.
    pub fn time_group_by_idx(&self, idx: usize) -> &TimeGroup {
        assert!(idx < self.time_groups.len());
        &self.time_groups[idx]
    }

    /// Get the list of time indices, which belong to the time group.
    pub fn time_group_time_ids(&self, id: &TimeGroupId) -> Vec<usize> {
        let mut time_idxs = vec![];
        for (i, time) in self.times.iter().enumerate() {
            if time.time_groups.contains(id) {
                time_idxs.push(i);
            }
        }

        time_idxs
    }
}

// Time-Methods
impl Database {
    /// Get a list of all times
    pub fn times(&self) -> &[Time] {
        &self.times
    }

    /// Get a time by id.
    pub fn time_by_id(&self, id: &TimeId) -> &Time {
        self.times.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a time by index.
    pub fn time_by_idx(&self, idx: usize) -> &Time {
        assert!(idx < self.times.len());
        &self.times[idx]
    }

    /// Resolve id to index.
    pub fn time_id_to_idx(&self, id: &TimeId) -> usize {
        self.times.iter().position(|x| x.id.eq(id)).unwrap()
    }
}

// Resource Data Methods ///////////////////////////////////////////////////////

// RecourceType-Methods
impl Database {
    /// Get a list of all resource types
    pub fn resource_types(&self) -> &[ResourceType] {
        &self.resource_types
    }

    /// Get a resource type by id.
    pub fn resource_type_by_id(&self, id: &ResourceTypeId) -> &ResourceType {
        self.resource_types.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a resource type by index.
    pub fn resource_type_by_idx(&self, idx: usize) -> &ResourceType {
        assert!(idx < self.resource_types.len());
        &self.resource_types[idx]
    }

    /// Get list of resource group indices, which have this resource type.
    pub fn resource_type_resource_group_idxs(
        &self,
        id: ResourceTypeId,
    ) -> Vec<usize> {
        let mut resource_group_idxs = vec![];
        for (i, resource_group) in self.resource_groups.iter().enumerate() {
            if resource_group.resource_type.eq(&id) {
                resource_group_idxs.push(i);
            }
        }

        resource_group_idxs
    }
}

// ResourceGroup-Methods
impl Database {
    /// Get a list of all resource groups
    pub fn resource_groups(&self) -> &[ResourceGroup] {
        &self.resource_groups
    }

    /// Get a resource-group by id.
    pub fn resource_group_by_id(&self, id: &ResourceGroupId) -> &ResourceGroup {
        self.resource_groups.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a resource-group by index.
    pub fn resouce_group_by_idx(&self, idx: usize) -> &ResourceGroup {
        assert!(idx < self.resource_groups.len());
        &self.resource_groups[idx]
    }

    /// Get list of resource indices, which belong to this resource group.
    pub fn resource_group_resource_idxs(
        &self,
        id: &ResourceGroupId,
    ) -> Vec<usize> {
        let mut resource_idxs = vec![];
        for (i, resource) in self.resources.iter().enumerate() {
            if resource.resource_groups.contains(id) {
                resource_idxs.push(i);
            }
        }

        resource_idxs
    }
}

// Resource-Methods
impl Database {
    /// Get a list of all resources
    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    /// Get a resource by id.
    pub fn resource_by_id(&self, id: &ResourceId) -> &Resource {
        self.resources.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a resource by index.
    pub fn resource_by_idx(&self, idx: usize) -> &Resource {
        assert!(idx < self.resources.len());
        &self.resources[idx]
    }

    /// Resolve id to index.
    pub fn resource_id_to_idx(&self, id: &ResourceId) -> usize {
        self.resources.iter().position(|x| x.id.eq(id)).unwrap()
    }
}

// Event Data Methods //////////////////////////////////////////////////////////
// Courses-Methods
impl Database {
    /// Get a list of all courses
    pub fn courses(&self) -> &[Course] {
        &self.courses
    }

    /// Get a course by id.
    pub fn course_by_id(&self, id: &CourseId) -> &Course {
        self.courses.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a course by index.
    pub fn course_by_idx(&self, idx: usize) -> &Course {
        assert!(idx < self.courses.len());
        &self.courses[idx]
    }

    /// Get the list of event indices, which belong to the course.
    pub fn course_event_idxs(&self, id: &CourseId) -> Vec<usize> {
        let mut event_idxs = vec![];
        for (i, event) in self.events.iter().enumerate() {
            if let Some(cid) = &event.course {
                if cid.eq(id) {
                    event_idxs.push(i);
                }
            }
        }

        event_idxs
    }
}

// EventGroup-Methods
impl Database {
    /// Get a list of all event groups
    pub fn event_groups(&self) -> &[EventGroup] {
        &self.event_groups
    }

    /// Get a event-group by id.
    pub fn event_group_by_id(&self, id: &EventGroupId) -> &EventGroup {
        let res = self.event_groups.iter().find(|x| x.id.eq(id));
        match res {
            Some(x) => x,
            None => {
                // Also search courses for the given resource id, because the
                // GR-PA-08 instance uses course ids as event group references.
                let c = self.courses.iter().find(|x| x.id.0.eq(&id.0)).unwrap();

                let _e = EventGroup {
                    id: EventGroupId(c.id.0.clone()),
                    name: c.name.clone(),
                };

                todo!("greek case (event group referencing a course")
            }
        }
    }

    /// Get a event-group by index.
    pub fn event_group_by_idx(&self, idx: usize) -> &EventGroup {
        assert!(idx < self.event_groups.len());
        &self.event_groups[idx]
    }

    /// Get the list of event indices, which belong to the event group.
    pub fn event_group_event_idxs(&self, id: &EventGroupId) -> Vec<usize> {
        let mut event_idxs = vec![];
        for (i, event) in self.events.iter().enumerate() {
            if event.event_groups.contains(id) {
                event_idxs.push(i);
            }
        }

        event_idxs
    }

    /// Check which events don't have a time allocated, and return a list of
    /// their indices.
    pub fn events_with_no_time(&self) -> Vec<usize> {
        self.events
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x.time {
                Some(_) => None,
                None => Some(i),
            })
            .collect()
    }
}

// Event-Methods
impl Database {
    /// Get a list of all events
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Get a event by id.
    pub fn event_by_id(&self, id: &EventId) -> &Event {
        self.events.iter().find(|x| x.id.eq(id)).unwrap()
    }

    /// Get a event by index.
    pub fn event_by_idx(&self, idx: usize) -> &Event {
        assert!(idx < self.events.len());
        &self.events[idx]
    }

    /// Resolve id to index.
    pub fn event_id_to_idx(&self, id: &EventId) -> usize {
        self.events.iter().position(|x| x.id.eq(id)).unwrap()
    }

    /// Get the maximal duration present in the events.
    pub fn events_max_duration(&self) -> usize {
        self.events.iter().map(|x| x.duration).max().unwrap_or_default()
            as usize
    }
}

// Constraint Data Methods /////////////////////////////////////////////////////

// Constraint Methods
impl Database {
    pub fn contraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

////////////////////////////////////////////////////////////////////////////////
