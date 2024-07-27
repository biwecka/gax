// Modules /////////////////////////////////////////////////////////////////////
pub mod constraints;
pub mod events;
pub mod resources;
pub mod times;

// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashMap;
use indexmap::IndexMap;

use constraints::*;
use events::*;
use resources::*;
use times::*;

// Structs /////////////////////////////////////////////////////////////////////

// Data ////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Default)]
pub struct Data {
    // Times
    weeks: HashMap<WeekId, Week>,
    days: HashMap<DayId, Day>,
    time_groups: HashMap<TimeGroupId, TimeGroup>,
    times: IndexMap<TimeId, Time>,
    // times: Vec<Time>,

    // Resources
    resource_types: HashMap<ResourceTypeId, ResourceType>,
    resource_groups: HashMap<ResourceGroupId, ResourceGroup>,
    resources: HashMap<ResourceId, Resource>,

    // Events
    courses: HashMap<CourseId, Course>,
    event_groups: HashMap<EventGroupId, EventGroup>,
    events: HashMap<EventId, Event>,

    // Indices
    indices: Indices,
}

#[derive(Clone, Debug, Default)]
pub struct Indices {
    // Times
    week_2_times: HashMap<WeekId, Vec<TimeId>>,
    day_2_times: HashMap<DayId, Vec<TimeId>>,
    time_group_2_times: HashMap<TimeGroupId, Vec<TimeId>>,

    // Resources
    resource_group_2_resources: HashMap<ResourceGroupId, Vec<ResourceId>>,
    resource_type_2_resource_groups:
        HashMap<ResourceTypeId, Vec<ResourceGroupId>>,
    resource_type_2_resources: HashMap<ResourceTypeId, Vec<ResourceId>>,

    // Events
    course_2_events: HashMap<CourseId, Vec<EventId>>,
    event_group_2_events: HashMap<EventGroupId, Vec<EventId>>,
}

// Methods for adding time information into the Data.
impl Data {
    pub fn add_week(&mut self, week: crate::parser::instances::times::Week) {
        // Convert
        let w: Week = week.into();

        // Insert
        self.weeks.insert(w.id.clone(), w);
    }

    pub fn add_day(&mut self, day: crate::parser::instances::times::Day) {
        // Convert
        let d: Day = day.into();

        // Insert
        self.days.insert(d.id.clone(), d);
    }

    pub fn add_time_group(
        &mut self,
        time_group: crate::parser::instances::times::TimeGroup,
    ) {
        // Convert
        let tg: TimeGroup = time_group.into();

        // Insert
        self.time_groups.insert(tg.id.clone(), tg);
    }

    pub fn add_time(&mut self, time: crate::parser::instances::times::Time) {
        // Convert
        let t: Time = time.into();

        // Insert
        self.times.insert(t.id.clone(), t.clone());
        // self.times.push(t.clone());

        // Update indices
        if let Some(week) = t.week {
            self.indices
                .week_2_times
                .entry(week) // get entry from hash map
                .or_default() // if it doesn't exist, set to default
                .push(t.id.clone()); // push an item to its vector value
        }

        if let Some(day) = t.day {
            self.indices
                .day_2_times
                .entry(day) // get entry from hash map
                .or_default() // if it doesn't exist, set to default
                .push(t.id.clone()); // push an item to its vector value
        }

        for time_group in t.time_groups {
            self.indices
                .time_group_2_times
                .entry(time_group) // get entry from hash map
                .or_default() // if it doesn't exist, set to default
                .push(t.id.clone()); // push an item to its vector value
        }
    }
}

// Methods for adding resource information into the Data.
impl Data {
    pub fn add_resource_type(
        &mut self,
        resource_type: crate::parser::instances::resources::ResourceType,
    ) {
        // Conversion
        let rt: ResourceType = resource_type.into();

        // Insert
        self.resource_types.insert(rt.id.clone(), rt);
    }

    pub fn add_resource_group(
        &mut self,
        resource_group: crate::parser::instances::resources::ResourceGroup,
    ) {
        // Conversion
        let rg: ResourceGroup = resource_group.into();

        // Insert
        self.resource_groups.insert(rg.id.clone(), rg.clone());

        // Update indices
        self.indices
            .resource_type_2_resource_groups
            .entry(rg.resource_type)
            .or_default()
            .push(rg.id);
    }

    pub fn add_resource(
        &mut self,
        resource: crate::parser::instances::resources::Resource,
    ) {
        // Conversion
        let r: Resource = resource.into();

        // Insert
        self.resources.insert(r.id.clone(), r.clone());

        // Update indices
        for resource_group in r.resource_groups {
            self.indices
                .resource_group_2_resources
                .entry(resource_group)
                .or_default()
                .push(r.id.clone());
        }

        self.indices
            .resource_type_2_resources
            .entry(r.resource_type)
            .or_default()
            .push(r.id);
    }
}

// Methods for adding event information into the Data.
impl Data {
    pub fn add_course(
        &mut self,
        course: crate::parser::instances::events::Course,
    ) {
        // Convert
        let c: Course = course.into();

        // Insert
        self.courses.insert(c.id.clone(), c);
    }

    pub fn add_event_group(
        &mut self,
        event_group: crate::parser::instances::events::EventGroup,
    ) {
        // Convert
        let eg: EventGroup = event_group.into();

        // Insert
        self.event_groups.insert(eg.id.clone(), eg);
    }

    pub fn add_event(
        &mut self,
        event: crate::parser::instances::events::Event,
    ) {
        // Convert
        let e: Event = event.into();

        // Insert
        self.events.insert(e.id.clone(), e.clone());

        // Update indices
        if let Some(course) = e.course {
            self.indices
                .course_2_events
                .entry(course)
                .or_default()
                .push(e.id.clone());
        }

        for event_group in e.event_groups {
            self.indices
                .event_group_2_events
                .entry(event_group)
                .or_default()
                .push(e.id.clone());
        }
    }
}

// Methods for querying the data structure
impl Data {
    pub fn get_times(&self) -> Vec<&Time> {
        self.times.values().collect()
        // &self.times
    }

    pub fn get_time_by_idx(&self, index: usize) -> &Time {
        self.times.get_index(index).unwrap().1
        // self.times.get(index).unwrap()
    }

    pub fn get_events(&self) -> Vec<&Event> {
        self.events.values().collect()
    }

    pub fn get_event_by_id(&self, id: &EventId) -> &Event {
        self.events.get(id).unwrap()
    }

    pub fn get_event_by_id_mut(&mut self, id: &EventId) -> &mut Event {
        self.events.get_mut(id).unwrap()
    }

    pub fn get_events_by_event_group(
        &self,
        id: &EventGroupId,
    ) -> &Vec<EventId> {
        self.indices.event_group_2_events.get(id).unwrap()
    }
}

// General methods
impl Data {
    pub fn init(xhstt_instance: &crate::parser::instances::Instance) -> Self {
        // Create empty Data
        let mut db = Self::default();

        // Time information
        if let Some(time_groups) = &xhstt_instance.times.time_groups {
            time_groups.weeks.iter().for_each(|week| db.add_week(week.clone()));
            time_groups.days.iter().for_each(|day| db.add_day(day.clone()));
            time_groups
                .time_groups
                .iter()
                .for_each(|tg| db.add_time_group(tg.clone()));
        }

        for time in &xhstt_instance.times.times {
            db.add_time(time.clone());
        }

        // Resource information
        if let Some(resource_types) = &xhstt_instance.resources.resource_types {
            for rt in &resource_types.list {
                db.add_resource_type(rt.clone());
            }
        }

        if let Some(resource_groups) = &xhstt_instance.resources.resource_groups
        {
            for rg in &resource_groups.list {
                db.add_resource_group(rg.clone());
            }
        }

        for resource in &xhstt_instance.resources.resources {
            db.add_resource(resource.clone());
        }

        // Event information
        if let Some(event_groups) = &xhstt_instance.events.event_groups {
            for course in &event_groups.courses {
                db.add_course(course.clone());
            }

            for eg in &event_groups.event_groups {
                db.add_event_group(eg.clone());
            }
        }

        for event in &xhstt_instance.events.events {
            db.add_event(event.clone());
        }

        // Return
        db
    }
}

// Constraints /////////////////////////////////////////////////////////////////
#[derive(Debug, Default)]
pub struct Constraints {
    constraints: Vec<Constraint>,
}

impl Constraints {
    pub fn init(xhstt_instance: &crate::parser::instances::Instance) -> Self {
        let constraints = xhstt_instance
            .constraints
            .list
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect();

        Self { constraints }
    }

    pub fn all(&self) -> Vec<Constraint> {
        self.constraints.clone()
    }
}

////////////////////////////////////////////////////////////////////////////////
