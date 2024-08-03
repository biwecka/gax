// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::times::TimeGroup as ITimeGroup;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct TimeGroup {
    pub id: TimeGroupId,
    pub name: String,
}

impl From<ITimeGroup> for TimeGroup {
    fn from(value: ITimeGroup) -> Self {
        let id = TimeGroupId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

////////////////////////////////////////////////////////////////////////////////
