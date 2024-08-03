// Imports /////////////////////////////////////////////////////////////////////
use super::{resource_group::ResourceGroupId, resource_type::ResourceTypeId};
use crate::parser::instances::resources::Resource as IResource;

// ID //////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ResourceId(pub String);
impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

// Struct //////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Resource {
    pub id: ResourceId,
    pub name: String,
    pub resource_type: ResourceTypeId,
    pub resource_groups: Vec<ResourceGroupId>,
}

impl From<IResource> for Resource {
    fn from(value: IResource) -> Self {
        let id = ResourceId(value.id);
        let name = value.name;

        let resource_type = ResourceTypeId(value.resource_type.reference);
        let resource_groups = value
            .resource_groups
            .map(|references| {
                references
                    .list
                    .into_iter()
                    .map(|x| ResourceGroupId(x.reference))
                    .collect()
            })
            .unwrap_or_default();

        Self { id, name, resource_type, resource_groups }
    }
}

////////////////////////////////////////////////////////////////////////////////
