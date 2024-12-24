// Week ////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[deprecated]
pub struct WeekId(pub String);
impl From<String> for WeekId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for WeekId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
#[deprecated]
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
#[deprecated]
pub struct DayId(pub String);
impl From<String> for DayId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for DayId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
#[deprecated]
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
#[deprecated]
pub struct TimeGroupId(pub String);
impl From<String> for TimeGroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for TimeGroupId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Clone, Debug)]
#[deprecated]
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
#[deprecated]
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

#[derive(Clone, Debug)]
#[deprecated]
pub struct Time {
    pub id: TimeId,
    pub name: String,

    pub week: Option<WeekId>,
    pub day: Option<DayId>,
    pub time_groups: Vec<TimeGroupId>,
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
