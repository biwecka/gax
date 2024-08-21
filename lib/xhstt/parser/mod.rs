// Modules /////////////////////////////////////////////////////////////////////
pub mod instances;
pub mod metadata;
pub mod solution_groups;

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct XhsttArchive {
    // Attributes
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // Children
    #[serde(rename = "MetaData", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<metadata::MetaData>,

    #[serde(rename = "Instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<instances::Instances>,

    #[serde(
        rename = "SolutionGroups",
        skip_serializing_if = "Option::is_none"
    )]
    pub solution_groups: Option<solution_groups::SolutionGroups>,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub instance_id: String,
    pub instance_name: String,

    pub times: usize,

    pub time_groups_total: usize,
    pub weeks: usize,
    pub days: usize,
    pub generic_time_groups: usize,

    pub resource_types: usize,
    pub resource_groups: usize,

    pub events: usize,
    pub predefined_event_resources: usize,
    pub not_predefined_event_resources: usize,

    pub predefined_event_times: usize,
}

impl XhsttArchive {
    pub fn instance_stats(&self) -> Option<Stats> {
        let s = self.clone();
        let i = s.instances.and_then(|v| v.list.first().cloned());
        let instance = i?;

        let instance_id = instance.id;
        let instance_name = instance.metadata.name;

        let times = instance.times.times.len();

        let weeks =
            instance.times.time_groups.as_ref().map_or(0, |v| v.weeks.len());

        let days =
            instance.times.time_groups.as_ref().map_or(0, |v| v.days.len());

        let generic_time_groups = instance
            .times
            .time_groups
            .as_ref()
            .map_or(0, |v| v.time_groups.len());

        let time_groups_total = weeks + days + generic_time_groups;

        let resource_types = instance
            .resources
            .resource_types
            .as_ref()
            .map_or(0, |x| x.list.len());

        let resource_groups = instance
            .resources
            .resource_groups
            .as_ref()
            .map_or(0, |x| x.list.len());

        let events = instance.events.events.len();

        let mut predefined_event_resources = 0;
        let mut not_predefined_event_resources = 0;

        for e in instance.events.events.clone() {
            if let Some(resources) = e.resources {
                for r in resources.list {
                    if r.is_preassigned() {
                        predefined_event_resources += 1;
                    } else {
                        not_predefined_event_resources += 1;
                    }
                }
            }
        }

        let mut predefined_event_times = 0;
        for e in instance.events.events {
            if e.time.is_some() {
                predefined_event_times += 1;
            }
        }

        Some(Stats {
            instance_id,
            instance_name,

            times,

            time_groups_total,
            weeks,
            days,
            generic_time_groups,

            resource_types,
            resource_groups,

            events,
            predefined_event_resources,
            not_predefined_event_resources,
            predefined_event_times,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
