// Imports /////////////////////////////////////////////////////////////////////
use crate::parser::instances::resources::ResourceType as IResourceType;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ResourceTypeId(pub String);
impl From<String> for ResourceTypeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for ResourceTypeId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct ResourceType {
    pub id: ResourceTypeId,
    pub name: String,
}

impl From<IResourceType> for ResourceType {
    fn from(value: IResourceType) -> Self {
        let id = ResourceTypeId(value.id);
        let name = value.name;

        Self { id, name }
    }
}

////////////////////////////////////////////////////////////////////////////////
