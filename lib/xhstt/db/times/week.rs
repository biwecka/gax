// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::times::Week as IWeek;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Week {
    pub id: WeekId,
    pub name: String,
}

impl From<IWeek> for Week {
    fn from(value: IWeek) -> Self {
        let id = WeekId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

////////////////////////////////////////////////////////////////////////////////
