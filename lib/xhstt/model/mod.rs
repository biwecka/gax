//! This module contains a **deprecated** data structure, which represents the
//! timetabling information contained in a XHSTT instance. The root of this
//! data structure is the [`Data`] struct, which contains multiple hash maps
//! that contain all sorts of timetabling data and relations of that data.
//!
//! ## Where was it used?
//! This data structure was used in the first couple of algorithms developed
//! for my master's thesis. These algorithms have been deprecated and moved
//! to the `archive` directory in the root of the repository.
//!
//! ## Why is it deprecated?
//! Because the algorithms (e.g. `alg_1` and `alg_2`) have been deprecated
//! themselves, this data structure is not actively needed in any of the
//! current algorithms anymore. It has been **replaced by** the
//! [`crate::db::Database`].
//!

#![allow(deprecated)]

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

// Data ////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Default)]
#[deprecated]
pub struct Data {
    // Times
    weeks: HashMap<WeekId, Week>,
    days: HashMap<DayId, Day>,
    time_groups: HashMap<TimeGroupId, TimeGroup>,
    times: IndexMap<TimeId, Time>,

    // Resources
    resource_types: HashMap<ResourceTypeId, ResourceType>,
    resource_groups: HashMap<ResourceGroupId, ResourceGroup>,
    resources: HashMap<ResourceId, Resource>,

    // Events
    courses: HashMap<CourseId, Course>,
    event_groups: HashMap<EventGroupId, EventGroup>,
    events: HashMap<EventId, Event>,

    // Indices
    pub indices: Indices,
}

#[derive(Clone, Debug, Default)]
#[deprecated]
pub struct Indices {
    // Times
    pub week_2_times: HashMap<WeekId, Vec<TimeId>>,
    pub day_2_times: HashMap<DayId, Vec<TimeId>>,
    pub time_group_2_times: HashMap<TimeGroupId, Vec<TimeId>>,

    // Resources
    pub resource_group_2_resources: HashMap<ResourceGroupId, Vec<ResourceId>>,
    pub resource_type_2_resource_groups:
        HashMap<ResourceTypeId, Vec<ResourceGroupId>>,
    pub resource_type_2_resources: HashMap<ResourceTypeId, Vec<ResourceId>>,
    pub resource_2_events: HashMap<ResourceId, Vec<EventId>>,

    // Events
    pub course_2_events: HashMap<CourseId, Vec<EventId>>,
    pub event_group_2_events: HashMap<EventGroupId, Vec<EventId>>,
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

        for resource in e.assigned_resources {
            self.indices
                .resource_2_events
                .entry(resource.id)
                .or_default()
                .push(e.id.clone());
        }

        // TODO: what's with the resources assigned through resource groups?
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

    pub fn get_resources_by_resource_group(
        &self,
        id: &ResourceGroupId,
    ) -> &Vec<ResourceId> {
        self.indices.resource_group_2_resources.get(id).unwrap()
    }

    pub fn get_events_by_resource(&self, id: &ResourceId) -> Vec<&Event> {
        self.indices
            .resource_2_events
            .get(id)
            .unwrap()
            .iter()
            .map(|event_id| self.events.get(event_id).unwrap())
            .collect()
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

// Database ////////////////////////////////////////////////////////////////////
pub struct Database {
    pub times: TimeDb,
    pub resources: ResourceDb,
    pub events: EventDb,
}

pub struct TimeDb {
    weeks: Vec<Week>,
    days: Vec<Day>,
    groups: Vec<TimeGroup>,
    times: Vec<Time>,
}

impl TimeDb {
    // Weeks ///////////////////////////////////////////////////////////////////
    pub fn weeks(&self) -> &[Week] {
        &self.weeks
    }

    pub fn week_ids(&self) -> Vec<String> {
        self.weeks.iter().map(|w| w.id.0.clone()).collect()
    }

    pub fn times_of_week(&self, week_id: &WeekId) -> Vec<Time> {
        self.times
            .clone()
            .into_iter()
            .filter_map(|t| {
                if let Some(wid) = t.week.clone() {
                    if wid.eq(week_id) {
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn time_() {}

    // Days ////////////////////////////////////////////////////////////////////
    pub fn days(&self) -> &[Day] {
        &self.days
    }

    pub fn day_ids(&self) -> Vec<String> {
        self.days.iter().map(|d| d.id.0.clone()).collect()
    }

    pub fn times_of_day(&self, day_id: &DayId) -> Vec<Time> {
        self.times
            .clone()
            .into_iter()
            .filter_map(|t| {
                if let Some(did) = t.day.clone() {
                    if did.eq(day_id) {
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    // Groups //////////////////////////////////////////////////////////////////
    pub fn groups(&self) -> &[TimeGroup] {
        &self.groups
    }

    pub fn group_ids(&self) -> Vec<String> {
        self.groups.iter().map(|g| g.id.0.clone()).collect()
    }

    pub fn times_of_group(&self, time_group_id: &TimeGroupId) -> Vec<Time> {
        self.times
            .clone()
            .into_iter()
            .filter_map(|t| {
                if t.time_groups.contains(time_group_id) {
                    Some(t)
                } else {
                    None
                }
            })
            .collect()
    }

    // Times ///////////////////////////////////////////////////////////////////
    pub fn get(&self) -> &[Time] {
        &self.times
    }

    pub fn ids(&self) -> Vec<String> {
        self.times.iter().map(|t| t.id.0.clone()).collect()
    }
}

pub struct ResourceDb {
    types: Vec<ResourceType>,
    groups: Vec<ResourceGroup>,
    resources: Vec<Resource>,
}

impl ResourceDb {
    pub fn types(&self) -> &[ResourceType] {
        &self.types
    }

    pub fn type_ids(&self) -> Vec<String> {
        self.types.iter().map(|t| t.id.0.clone()).collect()
    }

    pub fn groups(&self) -> &[ResourceGroup] {
        &self.groups
    }

    pub fn group_ids(&self) -> Vec<String> {
        self.groups.iter().map(|g| g.id.0.clone()).collect()
    }

    pub fn get(&self) -> &[Resource] {
        &self.resources
    }

    pub fn ids(&self) -> Vec<String> {
        self.resources.iter().map(|r| r.id.0.clone()).collect()
    }

    pub fn resources_of_group(
        &self,
        resource_group_id: &ResourceGroupId,
    ) -> Vec<Resource> {
        self.resources
            .clone()
            .into_iter()
            .filter_map(|r| {
                if r.resource_groups.contains(resource_group_id) {
                    Some(r)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn groups_of_type(
        &self,
        resource_type_id: &ResourceTypeId,
    ) -> Vec<ResourceGroup> {
        self.groups
            .clone()
            .into_iter()
            .filter_map(|g| {
                if g.resource_type.eq(resource_type_id) {
                    Some(g)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn resources_of_type(
        &self,
        resource_type_id: &ResourceTypeId,
    ) -> Vec<Resource> {
        self.resources
            .clone()
            .into_iter()
            .filter_map(|r| {
                if r.resource_type.eq(resource_type_id) {
                    Some(r)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn classes(&self) -> Vec<Resource> {
        self.resources
            .clone()
            .into_iter()
            .filter_map(|r| {
                if r.resource_type.eq(&"Class".into()) {
                    Some(r)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn teachers(&self) -> Vec<Resource> {
        self.resources
            .clone()
            .into_iter()
            .filter_map(|r| {
                if r.resource_type.eq(&"Teacher".into()) {
                    Some(r)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn rooms(&self) -> Vec<Resource> {
        self.resources
            .clone()
            .into_iter()
            .filter_map(|r| {
                if r.resource_type.eq(&"Room".into()) {
                    Some(r)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct EventDb {
    courses: Vec<Course>,
    groups: Vec<EventGroup>,
    events: Vec<Event>,
}

impl EventDb {
    pub fn courses(&self) -> &[Course] {
        &self.courses
    }

    pub fn course_ids(&self) -> Vec<String> {
        self.courses.iter().map(|c| c.id.0.clone()).collect()
    }

    pub fn groups(&self) -> &[EventGroup] {
        &self.groups
    }

    pub fn group_ids(&self) -> Vec<String> {
        self.groups.iter().map(|g| g.id.0.clone()).collect()
    }

    pub fn get(&self) -> &[Event] {
        &self.events
    }

    pub fn ids(&self) -> Vec<String> {
        self.events.iter().map(|e| e.id.0.clone()).collect()
    }

    pub fn events_of_course(&self, course_id: &CourseId) -> Vec<Event> {
        self.events
            .clone()
            .into_iter()
            .filter_map(|e| {
                if let Some(course) = e.course.clone() {
                    if course.eq(course_id) {
                        Some(e)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn events_of_group(&self, event_group_id: &EventGroupId) -> Vec<Event> {
        self.events
            .clone()
            .into_iter()
            .filter_map(|e| {
                if e.event_groups.contains(event_group_id) {
                    Some(e)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Database {
    pub fn init(instance: &crate::parser::instances::Instance) -> Self {
        // Time information
        let mut weeks: Vec<Week> = vec![];
        let mut days: Vec<Day> = vec![];
        let mut t_groups: Vec<TimeGroup> = vec![];
        let mut times: Vec<Time> = vec![];

        if let Some(time_groups) = &instance.times.time_groups {
            time_groups
                .weeks
                .clone()
                .into_iter()
                .for_each(|week| weeks.push(week.into()));

            time_groups
                .days
                .clone()
                .into_iter()
                .for_each(|day| days.push(day.into()));

            time_groups
                .time_groups
                .clone()
                .into_iter()
                .for_each(|tg| t_groups.push(tg.into()));
        }

        instance
            .times
            .times
            .clone()
            .into_iter()
            .for_each(|t| times.push(t.into()));

        // Resource information
        let mut r_types: Vec<ResourceType> = vec![];
        let mut r_groups: Vec<ResourceGroup> = vec![];
        let mut resources: Vec<Resource> = vec![];

        if let Some(resource_types) = &instance.resources.resource_types {
            resource_types
                .list
                .clone()
                .into_iter()
                .for_each(|rt| r_types.push(rt.into()));
        }

        if let Some(resource_groups) = &instance.resources.resource_groups {
            resource_groups
                .list
                .clone()
                .into_iter()
                .for_each(|rg| r_groups.push(rg.into()));
        }

        instance
            .resources
            .resources
            .clone()
            .into_iter()
            .for_each(|r| resources.push(r.into()));

        // Event information
        let mut courses: Vec<Course> = vec![];
        let mut e_groups: Vec<EventGroup> = vec![];
        let mut events: Vec<Event> = vec![];

        if let Some(event_groups) = &instance.events.event_groups {
            event_groups
                .courses
                .clone()
                .into_iter()
                .for_each(|c| courses.push(c.into()));

            event_groups
                .event_groups
                .clone()
                .into_iter()
                .for_each(|eg| e_groups.push(eg.into()));
        }

        instance
            .events
            .events
            .clone()
            .into_iter()
            .for_each(|e| events.push(e.into()));

        // Return
        Self {
            times: TimeDb { weeks, days, groups: t_groups, times },
            resources: ResourceDb {
                types: r_types,
                groups: r_groups,
                resources,
            },
            events: EventDb { courses, groups: e_groups, events },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
