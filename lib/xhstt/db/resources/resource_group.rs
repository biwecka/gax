// Imports /////////////////////////////////////////////////////////////////////
use super::resource_type::ResourceTypeId;
use crate::parser::instances::resources::ResourceGroup as IResourceGroup;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ResourceGroupId(pub String);
impl From<String> for ResourceGroupId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for ResourceGroupId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct ResourceGroup {
    pub id: ResourceGroupId,
    pub name: String,
    pub resource_type: ResourceTypeId,
}

impl From<IResourceGroup> for ResourceGroup {
    fn from(value: IResourceGroup) -> Self {
        let id = ResourceGroupId(value.id);
        let name = value.name;
        let resource_type = ResourceTypeId(value.resource_type.reference);

        Self { id, name, resource_type }
    }
}

////////////////////////////////////////////////////////////////////////////////
