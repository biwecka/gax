// Week ////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct WeekId(String);
pub type WeekRef = WeekId;

#[derive(Clone, Debug)]
pub struct Week {
    pub id: WeekId,
    pub name: String,
}

impl From<crate::parser::instances::times::Week> for Week {
    fn from(value: crate::parser::instances::times::Week) -> Self {
        let id = WeekId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

// Day /////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DayId(String);
pub type DayRef = DayId;

#[derive(Clone, Debug)]
pub struct Day {
    pub id: DayId,
    pub name: String,
}

impl From<crate::parser::instances::times::Day> for Day {
    fn from(value: crate::parser::instances::times::Day) -> Self {
        let id = DayId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

// Time Group //////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TimeGroupId(String);
pub type TimeGroupRef = TimeGroupId;

#[derive(Clone, Debug)]
pub struct TimeGroup {
    pub id: TimeGroupId,
    pub name: String,
}

impl From<crate::parser::instances::times::TimeGroup> for TimeGroup {
    fn from(value: crate::parser::instances::times::TimeGroup) -> Self {
        let id = TimeGroupId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

// Time ////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TimeId(pub String);
pub type TimeRef = TimeId;

#[derive(Clone, Debug)]
pub struct Time {
    pub id: TimeId,
    pub name: String,

    pub week: Option<WeekRef>,
    pub day: Option<DayRef>,
    pub time_groups: Vec<TimeGroupRef>,
}

impl From<crate::parser::instances::times::Time> for Time {
    fn from(value: crate::parser::instances::times::Time) -> Self {
        let id = TimeId(value.id);
        let name = value.name;

        let week = value.week.map(|r| WeekId(r.reference));
        let day = value.day.map(|r| DayId(r.reference));
        let time_groups = value
            .time_groups
            .map(|refs| {
                refs.list
                    .into_iter()
                    .map(|x| TimeGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        Self { id, name, week, day, time_groups }
    }
}

////////////////////////////////////////////////////////////////////////////////
