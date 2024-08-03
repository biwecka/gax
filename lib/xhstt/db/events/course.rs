// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::events::Course as ICourse;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CourseId(pub String);
impl From<String> for CourseId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for CourseId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Course {
    pub id: CourseId,
    pub name: String,
}

impl From<ICourse> for Course {
    fn from(value: ICourse) -> Self {
        let id = CourseId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

////////////////////////////////////////////////////////////////////////////////
