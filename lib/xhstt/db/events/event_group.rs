// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::events::EventGroup as IEventGroup;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EventGroupId(pub String);
impl From<String> for EventGroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for EventGroupId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct EventGroup {
    pub id: EventGroupId,
    pub name: String,
}

impl From<IEventGroup> for EventGroup {
    fn from(value: IEventGroup) -> Self {
        let id = EventGroupId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

////////////////////////////////////////////////////////////////////////////////
