// Imports /////////////////////////////////////////////////////////////////////
use super::{day::DayId, time_group::TimeGroupId, week::WeekId};
use crate::parser::instances::times::Time as ITime;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TimeId(pub String);
impl From<String> for TimeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for TimeId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Time {
    pub id: TimeId,
    pub name: String,

    pub week: Option<WeekId>,
    pub day: Option<DayId>,
    pub time_groups: Vec<TimeGroupId>,
}

impl From<ITime> for Time {
    fn from(value: ITime) -> Self {
        let id = TimeId(value.id);
        let name = value.name;

        let week = value.week.map(|x| WeekId(x.reference));
        let day = value.day.map(|x| DayId(x.reference));
        let time_groups = value
            .time_groups
            .map(|references| {
                references
                    .list
                    .into_iter()
                    .map(|x| TimeGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        Self { id, name, week, day, time_groups }
    }
}

////////////////////////////////////////////////////////////////////////////////
