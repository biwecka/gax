use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HighSchoolTimetableArchive {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Instances")]
    pub instances: Instances,
    #[serde(rename = "SolutionGroups")]
    pub solution_groups: SolutionGroups,
}

#[derive(Serialize, Deserialize)]
pub struct Instances {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Instance")]
    pub instance: Instance,
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MetaData")]
    pub meta_data: InstanceMetaData,
    #[serde(rename = "Times")]
    pub times: Times,
    #[serde(rename = "Resources")]
    pub resources: HighSchoolTimetableArchiveInstancesInstanceResources,
    #[serde(rename = "Events")]
    pub events: InstanceEvents,
    #[serde(rename = "Constraints")]
    pub constraints: Constraints,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceMetaData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Contributor")]
    pub contributor: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Remarks")]
    pub remarks: Remarks,
}

#[derive(Serialize, Deserialize)]
pub struct Remarks {
}

#[derive(Serialize, Deserialize)]
pub struct Times {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TimeGroups")]
    pub time_groups: TimesTimeGroups,
    #[serde(rename = "Time")]
    pub time: Vec<TimesTime>,
}

#[derive(Serialize, Deserialize)]
pub struct TimesTimeGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Day")]
    pub day: Vec<TimeGroupsDay>,
    #[serde(rename = "TimeGroup")]
    pub time_group: TimesTimeGroupsTimeGroup,
}

#[derive(Serialize, Deserialize)]
pub struct TimeGroupsDay {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimesTimeGroupsTimeGroup {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimesTime {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Day")]
    pub day: TimeDay,
    #[serde(rename = "TimeGroups")]
    pub time_groups: TimeTimeGroups,
}

#[derive(Serialize, Deserialize)]
pub struct TimeDay {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimeTimeGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TimeGroup")]
    pub time_group: TimeTimeGroupsTimeGroup,
}

#[derive(Serialize, Deserialize)]
pub struct TimeTimeGroupsTimeGroup {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct HighSchoolTimetableArchiveInstancesInstanceResources {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ResourceTypes")]
    pub resource_types: ResourceTypes,
    #[serde(rename = "ResourceGroups")]
    pub resource_groups: ResourcesResourceGroups,
    #[serde(rename = "Resource")]
    pub resource: Vec<InstanceResourcesResource>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceTypes {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Vec<ResourceTypesResourceType>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceTypesResourceType {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResourcesResourceGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ResourceGroup")]
    pub resource_group: Vec<ResourcesResourceGroupsResourceGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourcesResourceGroupsResourceGroup {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceGroupResourceType,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceGroupResourceType {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceResourcesResource {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceResourceType,
    #[serde(rename = "ResourceGroups")]
    pub resource_groups: ResourceResourceGroups,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceResourceType {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceResourceGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ResourceGroup")]
    pub resource_group: ResourceResourceGroupsResourceGroup,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceResourceGroupsResourceGroup {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceEvents {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "EventGroups")]
    pub event_groups: EventsEventGroups,
    #[serde(rename = "Event")]
    pub event: Vec<InstanceEventsEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct EventsEventGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Course")]
    pub course: Vec<EventGroupsCourse>,
    #[serde(rename = "EventGroup")]
    pub event_group: EventsEventGroupsEventGroup,
}

#[derive(Serialize, Deserialize)]
pub struct EventGroupsCourse {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct EventsEventGroupsEventGroup {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceEventsEvent {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Duration")]
    pub duration: String,
    #[serde(rename = "Course")]
    pub course: EventCourse,
    #[serde(rename = "Resources")]
    pub resources: InstanceEventsEventResources,
    #[serde(rename = "EventGroups")]
    pub event_groups: EventEventGroups,
}

#[derive(Serialize, Deserialize)]
pub struct EventCourse {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceEventsEventResources {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Resource")]
    pub resource: Vec<EventResourcesResource>,
}

#[derive(Serialize, Deserialize)]
pub struct EventResourcesResource {
    #[serde(rename = "@Reference")]
    pub reference: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Role")]
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct EventEventGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "EventGroup")]
    pub event_group: EventEventGroupsEventGroup,
}

#[derive(Serialize, Deserialize)]
pub struct EventEventGroupsEventGroup {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct Constraints {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AssignTimeConstraint")]
    pub assign_time_constraint: AssignTimeConstraint,
    #[serde(rename = "AvoidClashesConstraint")]
    pub avoid_clashes_constraint: AvoidClashesConstraint,
}

#[derive(Serialize, Deserialize)]
pub struct AssignTimeConstraint {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Required")]
    pub required: String,
    #[serde(rename = "Weight")]
    pub weight: String,
    #[serde(rename = "CostFunction")]
    pub cost_function: String,
    #[serde(rename = "AppliesTo")]
    pub applies_to: AssignTimeConstraintAppliesTo,
}

#[derive(Serialize, Deserialize)]
pub struct AssignTimeConstraintAppliesTo {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "EventGroups")]
    pub event_groups: AppliesToEventGroups,
}

#[derive(Serialize, Deserialize)]
pub struct AppliesToEventGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "EventGroup")]
    pub event_group: AppliesToEventGroupsEventGroup,
}

#[derive(Serialize, Deserialize)]
pub struct AppliesToEventGroupsEventGroup {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct AvoidClashesConstraint {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Required")]
    pub required: String,
    #[serde(rename = "Weight")]
    pub weight: String,
    #[serde(rename = "CostFunction")]
    pub cost_function: String,
    #[serde(rename = "AppliesTo")]
    pub applies_to: AvoidClashesConstraintAppliesTo,
}

#[derive(Serialize, Deserialize)]
pub struct AvoidClashesConstraintAppliesTo {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ResourceGroups")]
    pub resource_groups: AppliesToResourceGroups,
}

#[derive(Serialize, Deserialize)]
pub struct AppliesToResourceGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ResourceGroup")]
    pub resource_group: Vec<AppliesToResourceGroupsResourceGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct AppliesToResourceGroupsResourceGroup {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionGroups {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SolutionGroup")]
    pub solution_group: Vec<SolutionGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionGroup {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MetaData")]
    pub meta_data: SolutionGroupMetaData,
    #[serde(rename = "Solution")]
    pub solution: Solution,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionGroupMetaData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Contributor")]
    pub contributor: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Solution {
    #[serde(rename = "@Reference")]
    pub reference: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Events")]
    pub events: SolutionEvents,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionEvents {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Event")]
    pub event: Vec<SolutionEventsEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionEventsEvent {
    #[serde(rename = "@Reference")]
    pub reference: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: Option<String>,
    #[serde(rename = "Time")]
    pub time: EventTime,
    #[serde(rename = "Resources")]
    pub resources: SolutionEventsEventResources,
}

#[derive(Serialize, Deserialize)]
pub struct EventTime {
    #[serde(rename = "@Reference")]
    pub reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct SolutionEventsEventResources {
}