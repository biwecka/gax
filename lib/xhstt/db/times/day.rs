// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::times::Day as IDay;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Day {
    pub id: DayId,
    pub name: String,
}

impl From<IDay> for Day {
    fn from(value: IDay) -> Self {
        let id = DayId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

////////////////////////////////////////////////////////////////////////////////
